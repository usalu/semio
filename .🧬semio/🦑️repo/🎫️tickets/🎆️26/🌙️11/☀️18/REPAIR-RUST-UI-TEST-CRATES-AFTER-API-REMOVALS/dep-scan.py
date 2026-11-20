"""🔎️ Flags crate paths a UI crate's sources reference without declaring the crate as a (dev-)dependency."""
import os,re,sys
def walk(path,seen):
    path=os.path.normpath(path)
    if path in seen or not os.path.exists(path):return
    t=open(path,encoding='utf8',errors='replace').read();seen[path]=t
    for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+',t):walk(os.path.join(os.path.dirname(path),m.group(1)),seen)
for line in open(sys.argv[1]):
    name,manifest=line.rstrip("\n").split("\t")[:2];toml=open(manifest).read()
    lib=re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"',toml,re.S);seen={}
    walk(os.path.join(os.path.dirname(manifest),lib.group(1) if lib else "src/lib.rs"),seen)
    for m in re.finditer(r'\[\[test\]\][^\[]*?path\s*=\s*"([^"]+)"',toml,re.S):walk(os.path.join(os.path.dirname(manifest),m.group(1)),seen)
    declared={k.replace('-','_') for k in re.findall(r'^([A-Za-z0-9_-]+)\s*=\s*\{',toml,re.M)}|{k.replace('-','_') for k in re.findall(r'^([A-Za-z0-9_-]+)(?:\.workspace)?\s*=',toml,re.M)}|{name.replace('-','_')}
    used=set()
    for t in seen.values():
        t=re.sub(r'//[^\n]*','',t)
        used|=set(re.findall(r'(?<![\w:])(semio_[a-z0-9_]+)::',t))
    missing=sorted(u for u in used if u not in declared)
    if missing:print(name,missing)
