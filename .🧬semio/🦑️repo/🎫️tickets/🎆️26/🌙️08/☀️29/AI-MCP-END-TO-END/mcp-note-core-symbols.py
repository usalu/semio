import argparse
import mmap


def unsigned(data, start, end):
    value = 0
    for shift in range(0, 35, 7):
        if start >= end:
            break
        byte = data[start]
        start += 1
        value |= (byte & 127) << shift
        if byte < 128 and value <= 0xFFFFFFFF:
            return value, start
    raise ValueError("invalid Wasm u32")


def sections(data, start, end):
    while start < end:
        section = data[start]
        length, start = unsigned(data, start + 1, end)
        stop = start + length
        if stop > end:
            raise ValueError("Wasm section exceeds its container")
        yield section, start, stop
        start = stop


def function_names(data, start, end):
    for section, payload, stop in sections(data, start, end):
        if section != 0:
            continue
        length, name = unsigned(data, payload, stop)
        if length != 4 or data[name : name + length] != b"name":
            continue
        for subsection, payload, stop in sections(data, name + length, stop):
            if subsection != 1:
                continue
            count, position = unsigned(data, payload, stop)
            for _ in range(count):
                index, position = unsigned(data, position, stop)
                length, position = unsigned(data, position, stop)
                after_name = position + length
                if after_name > stop:
                    raise ValueError("Wasm function name exceeds its subsection")
                yield index, position, after_name
                position = after_name
            if position != stop:
                raise ValueError("trailing bytes in Wasm function names")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("component")
    parser.add_argument("query", nargs="+", help="Core function indices or name substrings")
    args = parser.parse_args()
    indices = {int(query) for query in args.query if query.isdecimal()}
    names = [query.encode() for query in args.query if not query.isdecimal()]
    with open(args.component, "rb") as source, mmap.mmap(source.fileno(), 0, access=mmap.ACCESS_READ) as data:
        if data[:8] != b"\x00asm\x0d\x00\x01\x00":
            raise ValueError("expected a Wasm component")
        module = 0
        for kind, start, end in sections(data, 8, len(data)):
            if kind != 1:
                continue
            if data[start : start + 8] != b"\x00asm\x01\x00\x00\x00":
                raise ValueError("expected an embedded Core module")
            for index, name, stop in function_names(data, start + 8, end):
                if index in indices or any(data.find(query, name, stop) >= 0 for query in names):
                    print(f"{module}:{index} {data[name:stop].decode('utf-8')}")
            module += 1


if __name__ == "__main__":
    main()
