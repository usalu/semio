"""📏️ Lists every Rust source a crate entry reaches through #[path] mods and include_str!-free `mod` files, with byte totals."""
import os,re,sys
seen={}
def walk(path):
    path=os.path.normpath(path)
    if path in seen or not os.path.exists(path):return
    data=open(path,'rb').read();seen[path]=len(data);text=data.decode('utf8','replace')
    for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+',text):
        walk(os.path.join(os.path.dirname(path),m.group(1)))
walk(sys.argv[1])
for p,n in sorted(seen.items(),key=lambda x:-x[1])[:int(sys.argv[2]) if len(sys.argv)>2 else 15]:print(n,p)
print("files",len(seen),"total",sum(seen.values()))
