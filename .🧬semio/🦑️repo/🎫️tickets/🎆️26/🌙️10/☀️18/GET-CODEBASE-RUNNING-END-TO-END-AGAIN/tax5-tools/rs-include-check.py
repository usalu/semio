# 🔬 Static check: every `include_str!("…")` / `include_bytes!("…")` literal of the given Rust files resolves to an existing file relative to the file. usage: rs-include-check.py <file.rs>…
import os,re,sys
miss=[];total=0
for path in sys.argv[1:]:
    text=open(path,encoding="utf8").read()
    for match in re.finditer(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)',text):
        total+=1
        if not os.path.isfile(os.path.normpath(os.path.join(os.path.dirname(path),match.group(1)))): miss.append((path[-60:],match.group(1)[-90:]))
print("checked",total,"include literals;",len(miss),"missing")
for row in miss[:15]: print("  ",row)
