"""🗂️ Sets the excavator entry of the catalog manifest to the corrected overall dimensions (the asset file is the single source; validator `manifest-dimension-mismatch`). The manifest is re-read right before writing and only the two numbers of this entry change."""
import re

PATH = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"
text = open(PATH, encoding="utf8").read()
pattern = re.compile(r'("id": "excavator",\s*"category": "machinery",\s*"title": \{[^}]*\},\s*"heightM": )[0-9.]+(,\s*"widthM": )[0-9.]+')
assert len(pattern.findall(text)) == 1
new = pattern.sub(lambda m: m.group(1) + "3.44" + m.group(2) + "9.64", text)
if new != text:
    open(PATH, "w", encoding="utf8").write(new)
print("changed" if new != text else "unchanged")
