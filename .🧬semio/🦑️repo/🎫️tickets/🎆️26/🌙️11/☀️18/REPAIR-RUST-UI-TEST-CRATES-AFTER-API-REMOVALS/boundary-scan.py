"""📐️ Approximates each crate's Rust source boundary use (lib plus #[path] mods, inline-module aware) and flags crates near or over budget."""
import os,re,sys
def walk(path,seen,inline=""):
    path=os.path.normpath(path)
    if path in seen or not os.path.exists(path):return
    seen[path]=os.path.getsize(path);text=open(path,encoding='utf8',errors='replace').read()
    for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+',text):
        walk(os.path.join(os.path.dirname(path),m.group(1)),seen)
for line in open(sys.argv[1]):
    name,manifest=line.rstrip("\n").split("\t")[:2]
    toml=open(manifest).read();b=re.search(r'maxSourceBytes\s*=\s*(\d+)',toml);lib=re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"',toml,re.S)
    entry=os.path.join(os.path.dirname(manifest),lib.group(1) if lib else "src/lib.rs");seen={};walk(entry,seen)
    total=sum(seen.values());limit=int(b.group(1)) if b else 0
    if limit and total>0.97*limit:print(f"{name}\t{total}\t{limit}\t{total-limit:+d}")
