"""🩹️ S4-PACKFIX second pass — repairs `PackError::from(<source>)` sites whose source has no `From` into `PackError`.

Usage: `python3 🧪️s4-packfix-from-fix.py <workspace-dir> <cargo-short-output>...`. Reads `the trait bound PackError: From<T>`
diagnostics and wraps the source at that line: standard UTF-8 decoder errors and codec errors that own a typed
`From<T> for ValueError` become `ValueError::from(<source>)`; plain `String` messages become
`ValueError::new(InvalidValue, <source>)`. Any other source type is listed as MANUAL. Idempotent.
"""
import pathlib
import re
import sys

TYPED = {"Utf8Error", "FromUtf8Error", "JpgError", "DwgExportError", "Part21Error", "PdfEngineError", "ZipError", "OpcError", "DocxError", "PptxError", "XlsxError"}
MESSAGE = {"String", "std::string::String", "&str"}
DIAGNOSTIC = re.compile(r"^(.*?🦀️\.rs):(\d+):\d+: error\[E0277\]: the trait bound `[A-Za-z_:]*PackError: From<([^>]+)>` is not satisfied")


def closing(line, start):
    depth = 0
    for index in range(start, len(line)):
        if line[index] == "(":
            depth += 1
        elif line[index] == ")":
            depth -= 1
            if depth == 0:
                return index
    raise ValueError("unbalanced")


def wrap(line, kind):
    out, at, changed = [], 0, 0
    while True:
        found = line.find("PackError::from(", at)
        if found < 0:
            out.append(line[at:])
            return "".join(out), changed
        start = found + len("PackError::from(")
        end = closing(line, start - 1)
        argument = line[start:end]
        out.append(line[at:start])
        if argument.startswith("semio_framework_value::ValueError::"):
            out.append(argument)
        elif kind == "typed":
            out.append(f"semio_framework_value::ValueError::from({argument})")
            changed += 1
        else:
            out.append(f"semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, {argument})")
            changed += 1
        out.append(")")
        at = end + 1


def main():
    workspace = pathlib.Path(sys.argv[1])
    sites = {}
    for log in sys.argv[2:]:
        for line in pathlib.Path(log).read_text(encoding="utf-8").splitlines():
            found = DIAGNOSTIC.match(line)
            if found:
                source = found.group(3).split("::")[-1] if found.group(3) not in MESSAGE else found.group(3)
                sites.setdefault((found.group(1), int(found.group(2))), set()).add(source)
    for (path, number), sources in sorted(sites.items()):
        kinds = {"typed" if source in TYPED else "message" if source in MESSAGE else "manual" for source in sources}
        if len(kinds) != 1 or "manual" in kinds:
            print(f"MANUAL {path}:{number} {sorted(sources)}")
            continue
        file = workspace / path
        lines = file.read_text(encoding="utf-8").split("\n")
        lines[number - 1], changed = wrap(lines[number - 1], kinds.pop())
        file.write_text("\n".join(lines), encoding="utf-8")
        print(f"fixed {changed} {path}:{number} {sorted(sources)}")


main()
