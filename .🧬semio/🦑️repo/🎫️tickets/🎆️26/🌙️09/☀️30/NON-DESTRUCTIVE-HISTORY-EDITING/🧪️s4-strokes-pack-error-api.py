"""🎒️ Moves the plugin codecs onto the `semio-framework-pack-error` API (S4-PACKFIX): `PackError::Schema(<text>)` becomes the
semantic refusal `PackError::from(ValueError::new(InvalidValue, <text>))`, `PackError::ValueRefusal` as a function becomes
`PackError::from`, and a `PackError::TextRefusal(e)` pattern becomes `PackError::Refusal(PackRefusal::TextRefusal(e))`.
Usage: python3 🧪️s4-strokes-pack-error-api.py <dir>…  (balanced over the argument; refuses a file changed during the run)."""
import pathlib
import re
import sys

SCHEMA = re.compile(r"((?:[A-Za-z_][A-Za-z0-9_]*::)*)PackError::Schema\(")


def close(text: str, open_index: int) -> int:
    depth, index, quote = 0, open_index, False
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
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    raise ValueError("unbalanced PackError::Schema call")


def convert(text: str) -> tuple[str, int]:
    out, cursor, count = [], 0, 0
    while True:
        match = SCHEMA.search(text, cursor)
        if not match:
            break
        open_index = match.end() - 1
        end = close(text, open_index)
        argument = text[open_index + 1:end]
        out.append(text[cursor:match.start()])
        out.append(f"{match.group(1)}PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, {argument}))")
        cursor = end + 1
        count += 1
    out.append(text[cursor:])
    text = "".join(out)
    text, functions = re.subn(r"((?:[A-Za-z_][A-Za-z0-9_]*::)*)PackError::ValueRefusal\b(?!\()", r"\1PackError::from", text)
    text, patterns = re.subn(r"((?:[A-Za-z_][A-Za-z0-9_]*::)*)PackError::TextRefusal\(([a-z_]+)\)", r"\1PackError::Refusal(\1PackRefusal::TextRefusal(\2))", text)
    return text, count + functions + patterns


total = 0
for root in sys.argv[1:]:
    for path in sorted(pathlib.Path(root).rglob("*.rs")):
        before = path.read_text()
        if "PackError::" not in before:
            continue
        after, count = convert(before)
        if count == 0:
            continue
        if path.read_text() != before:
            sys.exit(f"file changed during the run: {path}")
        path.write_text(after)
        total += count
        print(f"{count:3} {path}")
print(f"total={total}")
