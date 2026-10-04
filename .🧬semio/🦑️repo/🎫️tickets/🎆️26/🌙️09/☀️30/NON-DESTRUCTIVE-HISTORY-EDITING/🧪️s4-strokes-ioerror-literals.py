"""🚪️ Rewrites `IoError { message: <expr>, diagnostics: Vec::new() }` literals (the field became `cause: ValueError`) into
`IoError::from_value_error(ValueError::new(InvalidValue, <expr>))` in the given trees; refuses any other literal shape.
Usage: python3 🧪️s4-strokes-ioerror-literals.py <dir>..."""
import pathlib
import sys

HEAD = "IoError { message: "
TAIL = ", diagnostics: Vec::new() }"


def expression_end(text: str, start: int) -> int:
    depth, index, quote = 0, start, False
    while index < len(text):
        char = text[index]
        if quote:
            if char == "\\":
                index += 2
                continue
            if char == '"':
                quote = False
        elif char == '"':
            quote = True
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            if depth == 0:
                return index
            depth -= 1
        elif char == "," and depth == 0:
            return index
        index += 1
    raise ValueError("unterminated literal")


def convert(path: pathlib.Path) -> int:
    before = path.read_text()
    out, cursor, count = [], 0, 0
    while True:
        start = before.find(HEAD, cursor)
        if start < 0:
            break
        value_start = start + len(HEAD)
        end = expression_end(before, value_start)
        if not before.startswith(TAIL, end):
            sys.exit(f"unexpected IoError literal shape in {path}: {before[start:end + 40]!r}")
        expression = before[value_start:end]
        out.append(before[cursor:start])
        out.append(f"IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, {expression}))")
        cursor = end + len(TAIL)
        count += 1
    out.append(before[cursor:])
    if count and path.read_text() == before:
        path.write_text("".join(out))
    elif count:
        sys.exit(f"file changed during conversion: {path}")
    return count


total = 0
for root in sys.argv[1:]:
    for path in sorted(pathlib.Path(root).rglob("*.rs")):
        if HEAD in path.read_text():
            count = convert(path)
            total += count
            print(f"{count:3} {path}")
print(f"total={total}")
