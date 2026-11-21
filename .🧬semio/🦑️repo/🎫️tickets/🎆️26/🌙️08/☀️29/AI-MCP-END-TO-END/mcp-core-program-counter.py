import argparse
import mmap
from runpy import run_path
from pathlib import Path


parser = run_path(str(Path(__file__).with_name("mcp-note-core-symbols.py")))
unsigned = parser["unsigned"]
sections = parser["sections"]


def imported_functions(data, position, end):
    count, position = unsigned(data, position, end)
    functions = 0
    for _ in range(count):
        for _ in range(2):
            length, position = unsigned(data, position, end)
            position += length
        kind = data[position]
        position += 1
        if kind == 0:
            _, position = unsigned(data, position, end)
            functions += 1
        elif kind == 1 or kind == 2:
            if kind == 1:
                position += 1
            flags, position = unsigned(data, position, end)
            if flags & ~3:
                raise ValueError("unexpected imported linear memory limits")
            _, position = unsigned(data, position, end)
            if flags & 1:
                _, position = unsigned(data, position, end)
        elif kind == 3:
            position += 2
        elif kind == 4:
            position += 1
            _, position = unsigned(data, position, end)
        else:
            raise ValueError(f"unexpected Wasm import kind {kind}")
    return functions


def locate(data, query):
    modules = (record for record in sections(data, 8, len(data)) if record[0] == 1)
    _, module, module_end = next(modules)
    first_defined = 0
    for section, start, end in sections(data, module + 8, module_end):
        if section == 2:
            first_defined = imported_functions(data, start, end)
        elif section == 10:
            count, position = unsigned(data, start, end)
            for index in range(first_defined, first_defined + count):
                body_size, body = unsigned(data, position, end)
                position = body + body_size
                if index not in query:
                    continue
                groups, code = unsigned(data, body, position)
                for _ in range(groups):
                    _, code = unsigned(data, code, position)
                    code += 1
                for pc in range(code, position):
                    if pc - code in query[index]:
                        context = " ".join(f"{byte:02x}" for byte in data[pc:min(position, pc + 12)])
                        print(f"{index}:{pc-code}: {context}")
            return


if __name__ == "__main__":
    arguments = argparse.ArgumentParser()
    arguments.add_argument("component")
    arguments.add_argument("program_counters", nargs="+")
    options = arguments.parse_args()
    query = {}
    for counter in options.program_counters:
        function, offset = map(int, counter.split(":"))
        query.setdefault(function, set()).add(offset)
    with open(options.component, "rb") as source, mmap.mmap(source.fileno(), 0, access=mmap.ACCESS_READ) as component:
        locate(component, query)
