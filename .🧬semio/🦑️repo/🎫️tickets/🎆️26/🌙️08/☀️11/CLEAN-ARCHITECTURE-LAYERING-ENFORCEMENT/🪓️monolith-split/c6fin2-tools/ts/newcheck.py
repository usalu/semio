"""usage: newcheck.py <list-of-new-files.txt>  (absolute paths). Checks per new file: folder is <emoji>U+FE0F<kebab>, the docstring's first character after the opener is the folder emoji, the emoji is a taxonomy prefix, siblings' emojis are unique, the path is <= 240 bytes, no [DEBUG], no leading-edge problems."""
import sys, os, re, json, collections
T="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
tx=json.load(open(T,encoding="utf8"))
names=set()
def walk(o):
    if isinstance(o,dict):
        for k,v in o.items():
            if k=="memberNames" and isinstance(v,list): names.update(x for x in v if isinstance(x,str))
            else: walk(v)
    elif isinstance(o,list):
        for x in o: walk(x)
walk(tx)
prefixes=set()
for n in names:
    m=re.match(r"^(\S+?)️",n)
    if m: prefixes.add(m.group(1))
bad=0
files=[l.strip() for l in open(sys.argv[1],encoding="utf8") if l.strip()]
def emoji_of(folder):
    i=folder.index("️")
    return folder[:i+1], folder[:i]
sib=collections.defaultdict(list)
for p in files:
    folder=os.path.basename(os.path.dirname(p))
    if "️" not in folder or not re.match(r"^[a-z0-9-]+$",folder[folder.index("️")+1:]):
        print("BAD-FOLDER",folder); bad+=1; continue
    em,bare=emoji_of(folder)
    t=open(p,encoding="utf8").read()
    m=re.match(r"^/\*\*? "+re.escape(em)+r"(?!️)\s",t) or re.match(r"^/\*\*? "+re.escape(bare)+r"\s",t)
    if not m: print("BAD-DOC",p[-80:],repr(t[:20])); bad+=1
    if bare not in prefixes and em not in prefixes: print("NOT-IN-TAXONOMY",em); bad+=1
    if len(p.encode("utf8"))>240: print("LONG",len(p.encode())); bad+=1
    if "[DEBUG]" in t: print("DEBUG",p); bad+=1
    sib[os.path.dirname(os.path.dirname(p))].append((bare,folder))
for d,l in sib.items():
    c=collections.Counter(b for b,_ in l)
    ex=set(os.listdir(d))
    existing=collections.Counter(x[:x.index("️")] for x in ex if "️" in x)
    for b,n in c.items():
        if n>1: print("DUP-AMONG-NEW",d[-60:],b); bad+=1
        if existing[b]>n: print("CLASH-WITH-OLD-SIBLING",d[-60:],b,[x for x in ex if x.startswith(b+"️")]); bad+=1
print("files",len(files),"problems",bad, "max path bytes",max(len(p.encode()) for p in files))
