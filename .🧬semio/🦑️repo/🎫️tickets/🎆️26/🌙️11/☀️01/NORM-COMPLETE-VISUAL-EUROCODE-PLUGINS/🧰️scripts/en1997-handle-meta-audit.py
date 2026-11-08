import re,glob,sys
root=sys.argv[1]
def norm(p): return re.sub(r'\[id=[^\]]*\]','[]',p)
rules=set(re.findall(r'SetFieldRule \{ path: "([^"]+)"',open(glob.glob(root+'/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs')[0]).read()))
paths=set()
for f in glob.glob(root+'/🎬️scene/**/*.rs',recursive=True):
    if '🧪️tests' in f: continue
    for m in re.findall(r'"([a-zA-Z]+\[id=[^"]*\](?:\.[a-zA-Z\[\]=\{\}]+)+)"',open(f).read()):
        paths.add(norm(re.sub(r'\{[^}]*\}','',m)))
meta=''
for f in glob.glob(root+'/🧬️schema/💡️inferences/⚖️checks/**/*.rs',recursive=True): meta+=open(f).read()
print("handle paths:",len(paths))
for p in sorted(paths):
    ok=p in rules
    print(("ok  " if ok else "MISS"),p)
