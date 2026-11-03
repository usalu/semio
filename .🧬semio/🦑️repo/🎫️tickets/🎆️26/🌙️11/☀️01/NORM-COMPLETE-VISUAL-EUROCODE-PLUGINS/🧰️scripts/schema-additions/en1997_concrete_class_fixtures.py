"""en1997 `piles[].concreteClass` in every mutation fixture (snapshots `before`/`after`, diffs, insert-pile payloads): inserts `"concreteClass": "notGiven"` (null in a diff patch) after each `"hammerEfficiency"` key of a pile row; layout independent (works on the flat `🔣️.<role>.json` layout and the old one). Idempotent. Run only after `[done] PATH-BUDGET`."""
import os, re, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any"
count = 0
for top in ("/🧪️tests/🧫️fixtures/🧬️mutations", "/🚪️io", "/🎬️scene", "/📚️examples"):
    for dirpath, dirs, files in os.walk(ROOT + top):
        dirs[:] = [d for d in dirs if d not in ("target", "node_modules")]
        for name in files:
            if not name.endswith(".json"):
                continue
            path = os.path.join(dirpath, name)
            text = open(path, encoding="utf8").read()
            if '"hammerEfficiency"' not in text or '"concreteClass"' in text:
                continue
            if "\n" in text.strip():
                new = re.sub(r'^(\s*)"hammerEfficiency": ([^\n,]+)(,?)\n', lambda m: f'{m.group(1)}"hammerEfficiency": {m.group(2)},\n{m.group(1)}"concreteClass": {"null" if m.group(2).strip() == "null" else chr(34) + "notGiven" + chr(34)}{m.group(3)}\n', text, flags=re.M)
            else:
                new = re.sub(r'("hammerEfficiency":\s*)([^,}\s]+)', lambda m: f'{m.group(1)}{m.group(2)},"concreteClass":{"null" if m.group(2) == "null" else chr(34) + "notGiven" + chr(34)}', text)
            if new != text:
                open(path, "w", encoding="utf8").write(new)
                count += 1
print("fixtures updated:", count)
