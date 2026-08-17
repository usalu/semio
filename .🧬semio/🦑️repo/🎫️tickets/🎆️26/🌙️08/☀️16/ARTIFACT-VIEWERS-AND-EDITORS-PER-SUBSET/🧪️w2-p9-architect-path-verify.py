import re, os
glue_path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs"
text = open(glue_path, encoding="utf-8").read()
total = 0
missing = 0
for p in re.findall(r'#\[path\s*=\s*"([^"]+)"\]', text):
    total += 1
    if p != "." and not os.path.isfile(os.path.normpath(os.path.join(os.path.dirname(glue_path), p))):
        missing += 1
        print("MISSING", p)
print(f"total paths checked: {total}, missing: {missing}")
