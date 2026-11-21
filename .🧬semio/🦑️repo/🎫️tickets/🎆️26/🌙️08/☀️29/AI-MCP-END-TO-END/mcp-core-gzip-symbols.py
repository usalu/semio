import argparse
import gzip


class Stream:
    def __init__(self, source):
        self.source = source
        self.position = 0

    def read(self, count):
        data = self.source.read(count)
        if len(data) != count:
            raise ValueError("truncated component")
        self.position += count
        return data

    def skip_to(self, end):
        if end < self.position:
            raise ValueError("Wasm section exceeded its container")
        while self.position < end:
            self.read(min(64 * 1024, end - self.position))

    def unsigned(self):
        value = 0
        for shift in range(0, 35, 7):
            byte = self.read(1)[0]
            value |= (byte & 127) << shift
            if byte < 128 and value <= 0xFFFFFFFF:
                return value
        raise ValueError("invalid Wasm u32")

    def sections(self, end):
        while self.position < end:
            kind = self.read(1)[0]
            size = self.unsigned()
            stop = self.position + size
            if stop > end:
                raise ValueError("Wasm section exceeds its container")
            yield kind, stop
            self.skip_to(stop)


def find_names(stream, end, wanted):
    for kind, end in stream.sections(end):
        if kind != 0:
            continue
        size = stream.unsigned()
        if size != 4 or stream.read(size) != b"name":
            continue
        for kind, end in stream.sections(end):
            if kind != 1:
                continue
            for _ in range(stream.unsigned()):
                index = stream.unsigned()
                size = stream.unsigned()
                stop = stream.position + size
                if stop > end:
                    raise ValueError("Wasm function name exceeds its subsection")
                if index in wanted:
                    if size > 1024 * 1024:
                        raise ValueError("selected Wasm symbol exceeds 1 MiB")
                    yield index, stream.read(size).decode("utf8")
                stream.skip_to(stop)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("component_gzip")
    parser.add_argument("uncompressed_size", type=int)
    parser.add_argument("indices", type=int, nargs="+")
    args = parser.parse_args()
    wanted = set(args.indices)
    with gzip.open(args.component_gzip, "rb") as compressed:
        stream = Stream(compressed)
        if stream.read(8) != b"\x00asm\x0d\x00\x01\x00":
            raise ValueError("expected a Wasm component")
        module = 0
        for kind, end in stream.sections(args.uncompressed_size):
            if kind != 1:
                continue
            if stream.read(8) != b"\x00asm\x01\x00\x00\x00":
                raise ValueError("expected an embedded Core module")
            for index, name in find_names(stream, end, wanted):
                print(f"{module}:{index} {name}")
            module += 1


if __name__ == "__main__":
    main()
