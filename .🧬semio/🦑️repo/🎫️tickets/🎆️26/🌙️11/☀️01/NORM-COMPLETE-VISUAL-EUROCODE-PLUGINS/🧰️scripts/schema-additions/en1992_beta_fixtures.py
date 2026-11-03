"""en1992 `punching.betaMethod|momentC1|momentC2`: every committed snapshot document with a `punching` object gets the explicit simplified method and zero moments (the behaviour it always had), layout preserved. Idempotent, path-agnostic."""
import pathlib, re
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
PATTERN = re.compile(r'^(?P<i> *)"asw": (?P<v>[0-9.eE+-]+)\n', re.M)
count = 0
for path in ROOT.rglob("*.json"):
    text = path.read_text(encoding="utf8")
    if '"columnPosition"' not in text or '"betaMethod"' in text:
        continue
    found = len(PATTERN.findall(text))
    assert found == text.count('"columnPosition"'), (path, found)
    path.write_text(PATTERN.sub(lambda m: f'{m["i"]}"asw": {m["v"]},\n{m["i"]}"betaMethod": "Simplified",\n{m["i"]}"momentC1": 0.0,\n{m["i"]}"momentC2": 0.0\n', text), encoding="utf8")
    count += 1
print("fixtures", count)
