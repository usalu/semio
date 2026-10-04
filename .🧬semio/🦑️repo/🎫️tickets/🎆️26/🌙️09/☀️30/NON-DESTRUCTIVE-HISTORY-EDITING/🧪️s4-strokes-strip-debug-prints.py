"""🧹️ Removes temporary `println!`/`eprintln!` statements whose literal starts with `[DEBUG] ` (balanced over their arguments,
with the trailing `;` and the line when nothing else remains), plus a JS `console.error('[DEBUG] …');` inside a test script literal.
Usage: python3 🧪️s4-strokes-strip-debug-prints.py <file>…  (prints the count per file; refuses a file changed during the run)."""
import pathlib
import re
import sys

MACRO = re.compile(r'(?:e?println)!\("\[DEBUG\] ')
SCRIPT = re.compile(r"console\.error\('\[DEBUG\] [^']*'\);")


def statement_end(text: str, start: int) -> int:
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
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                end = index + 1
                return end + 1 if text[end:end + 1] == ";" else end
        index += 1
    raise ValueError("unterminated macro call")


def strip(text: str) -> tuple[str, int]:
    out, cursor, count = [], 0, 0
    for match in MACRO.finditer(text):
        if match.start() < cursor:
            continue
        end = statement_end(text, match.start())
        line_start = text.rfind("\n", 0, match.start()) + 1
        line_end = text.find("\n", end)
        line_end = len(text) if line_end < 0 else line_end
        if text[line_start:match.start()].strip() == "" and text[end:line_end].strip() == "":
            out.append(text[cursor:line_start])
            cursor = line_end + 1
        else:
            out.append(text[cursor:match.start()])
            cursor = end
        count += 1
    out.append(text[cursor:])
    result, scripts = SCRIPT.subn("", "".join(out))
    return result, count + scripts


for name in sys.argv[1:]:
    path = pathlib.Path(name)
    before = path.read_text()
    after, count = strip(before)
    if count:
        if path.read_text() != before:
            sys.exit(f"file changed during the run: {path}")
        path.write_text(after)
    print(f"{count:3} {path}")
