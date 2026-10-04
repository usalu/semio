"""🔤️ Moves `dsl::json::*` calls (removed from the `dsl` facade by the peer value/pack extraction) onto `semio_framework_pack_json` in the
given trees: `from_json_str(x)` gains the explicit `JsonMemberPolicy::Reject`, `parse(x)` likewise, the rest keep their names.
Usage: python3 🧪️s4-strokes-dsl-json-to-pack-json.py <dir>... (prints per-file counts; refuses a file changed during the run)."""
import pathlib
import re
import sys

REJECT = "semio_framework_pack_json::JsonMemberPolicy::Reject"


def close_paren(text: str, start: int) -> int:
    depth, index, quote = 0, start, None
    while index < len(text):
        char = text[index]
        if quote:
            if char == "\\":
                index += 2
                continue
            if char == quote:
                quote = None
        elif text.startswith('r#"', index):
            end = text.index('"#', index + 3)
            index = end + 2
            continue
        elif char == '"':
            quote = char
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    raise ValueError("unbalanced call")


def with_policy(text: str, name: str) -> tuple[str, int]:
    pattern = re.compile(r"dsl::json::" + name + r"(::<[^>]*>)?\(")
    out, cursor, count = [], 0, 0
    for match in pattern.finditer(text):
        if match.start() < cursor:
            continue
        open_index = match.end() - 1
        end = close_paren(text, open_index)
        argument = text[open_index + 1:end]
        out.append(text[cursor:match.start()])
        out.append(f"semio_framework_pack_json::{name}{match.group(1) or ''}({argument}, {REJECT})")
        cursor = end + 1
        count += 1
    out.append(text[cursor:])
    return "".join(out), count


def convert(path: pathlib.Path) -> int:
    before = path.read_text()
    text, total = before, 0
    for name in ("from_json_str", "parse"):
        text, count = with_policy(text, name)
        total += count
    text, renamed = re.subn(r"\bdsl::json::", "semio_framework_pack_json::", text)
    text, imports = re.subn(r"^use dsl::json;\n", "use semio_framework_pack_json as json;\n", text, flags=re.M)
    total += renamed + imports
    if total == 0:
        return 0
    if path.read_text() != before:
        sys.exit(f"file changed during conversion: {path}")
    path.write_text(text)
    return total


grand = 0
for root in sys.argv[1:]:
    for path in sorted(pathlib.Path(root).rglob("*.rs")):
        if "dsl::json" in path.read_text():
            count = convert(path)
            grand += count
            print(f"{count:4} {path}")
print(f"total={grand}")
