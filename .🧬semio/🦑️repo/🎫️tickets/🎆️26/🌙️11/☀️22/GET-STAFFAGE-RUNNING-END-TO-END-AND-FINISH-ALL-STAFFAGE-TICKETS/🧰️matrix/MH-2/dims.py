"""📐️ Sets declared dimensions of a catalog asset to the measured extents stated in dims/<id>.json: dims.py <catalog 🔣️.json> <dims.json> rewrites only the numbers of the keys given inside the `dimensions` block of the file, text in place (indentation and every other byte kept); a second run changes nothing. The file is read again right before it is written and left alone when it changed meanwhile."""
import json, re, sys
path, want = sys.argv[1], json.load(open(sys.argv[2]))
text = open(path, encoding="utf-8").read()
block = re.search(r'"dimensions"\s*:\s*\{[^}]*\}', text)
new = block.group(0)
for key, value in want.items():
    new = re.sub(r'("%s"\s*:\s*)[0-9.]+' % key, lambda m: m.group(1) + json.dumps(value), new)
if new == block.group(0):
    print("dimensions unchanged")
elif open(path, encoding="utf-8").read() != text:
    print("REFUSED: the file changed on disk"); sys.exit(1)
else:
    open(path, "w", encoding="utf-8").write(text[: block.start()] + new + text[block.end():]); print("dimensions written:", json.dumps(want))
