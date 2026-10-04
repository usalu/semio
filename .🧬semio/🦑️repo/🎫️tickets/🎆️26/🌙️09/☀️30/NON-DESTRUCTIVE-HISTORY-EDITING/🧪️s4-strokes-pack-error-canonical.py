"""🎒️ Aligns the `🧪️s4-strokes-pack-error-api.py` output with the canonical S4-PACKFIX mapping (`📓️s4-packfix-report.md` § Schema Decisions):
a stringified refusal `PackError::from(ValueError::new(InvalidValue, e.to_string()))` keeps its real cause instead — envelope build /
unwrap (`SemioError`) → `e.into_value_error()`, UTF-8 → `ValueError::from(e)`, a typed JSON / value decode (`ValueError`) → `e` itself.
Envelope-identity mismatches and other literal texts stay `InvalidValue`. Usage: python3 🧪️s4-strokes-pack-error-canonical.py <dir>…"""
import pathlib
import re
import sys

STRINGIFIED = re.compile(r"PackError::from\(semio_framework_value::ValueError::new\(semio_framework_value::ValueRefusalKind::InvalidValue, ([a-z_]+)\.to_string\(\)\)\)")
CAUSES = (
    (re.compile(r"(?:unwrap_binary|from_envelope_id)\([^;]*?\)\.map_err\(\|([a-z_]+)\|[^|]*$"), lambda name: f"PackError::from({name}.into_value_error())"),
    (re.compile(r"from_utf8\([^;]*?\)\.map_err\(\|([a-z_]+)\|[^|]*$"), lambda name: f"PackError::from(semio_framework_value::ValueError::from({name}))"),
    (re.compile(r"(?:from_json_str|from_value|FromValue>::from_value)\([^;]*?\)\.map_err\(\|([a-z_]+)\|[^|]*$"), lambda name: f"PackError::from({name})"),
)


def align(text: str) -> tuple[str, int, list[str]]:
    out, cursor, count, unclassified = [], 0, 0, []
    for match in STRINGIFIED.finditer(text):
        line_start = text.rfind("\n", 0, match.start()) + 1
        prefix = text[line_start:match.start()]
        replacement = None
        for pattern, build in CAUSES:
            found = pattern.search(prefix)
            if found and found.group(1) == match.group(1):
                replacement = build(match.group(1))
                break
        if replacement is None:
            unclassified.append(prefix.strip()[-120:])
            continue
        out.append(text[cursor:match.start()])
        out.append(replacement)
        cursor = match.end()
        count += 1
    out.append(text[cursor:])
    return "".join(out), count, unclassified


total = 0
for root in sys.argv[1:]:
    for path in sorted(pathlib.Path(root).rglob("*.rs")):
        before = path.read_text()
        if "ValueRefusalKind::InvalidValue, " not in before:
            continue
        after, count, unclassified = align(before)
        for prefix in unclassified:
            print(f"  kept InvalidValue: {path.name} … {prefix}")
        if count:
            if path.read_text() != before:
                sys.exit(f"file changed during the run: {path}")
            path.write_text(after)
            total += count
            print(f"{count:3} {path}")
print(f"total={total}")
