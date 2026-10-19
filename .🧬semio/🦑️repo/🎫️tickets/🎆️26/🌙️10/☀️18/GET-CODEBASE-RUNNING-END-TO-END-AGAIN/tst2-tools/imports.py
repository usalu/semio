import os,re,sys,json,collections
ROOT="/Users/ueli/Documents/semio"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
files=[]
for k in ["bun","vitest","both"]:
    files+=[l.strip() for l in open(f"{S}/scope-{k}.txt") if l.strip()]
BUILTIN=set("assert buffer child_process cluster console constants crypto dgram dns domain events fs http http2 https module net os path perf_hooks process punycode querystring readline repl stream string_decoder sys timers tls tty url util v8 vm worker_threads zlib async_hooks inspector".split())
imp=re.compile(r'(?:from\s*|import\s*\(\s*|require\s*\(\s*|import\s+)(["\'])([^"\'\n]+)\1')
cache={}
def resolvable(d,pkg):
    cur=d
    while True:
        key=(cur,pkg)
        if key in cache: r=cache[key]
        else:
            r=os.path.exists(os.path.join(cur,"node_modules",pkg)); cache[key]=r
        if r: return True
        if cur==ROOT or cur=="/": return False
        cur=os.path.dirname(cur)
missing=collections.defaultdict(set)
for f in files:
    p=os.path.join(ROOT,f)
    try: t=open(p,encoding="utf-8").read()
    except Exception: continue
    for m in imp.finditer(t):
        s=m.group(2)
        if s.startswith((".","/","node:","bun:","#")) : continue
        parts=s.split("/")
        pkg="/".join(parts[:2]) if s.startswith("@") else parts[0]
        if pkg in BUILTIN or pkg in ("bun","vitest/config") : continue
        if not resolvable(os.path.dirname(p),pkg): missing[pkg].add(f)
json.dump({k:sorted(v) for k,v in missing.items()},open(sys.argv[1],"w"),ensure_ascii=False)
for k,v in sorted(missing.items(),key=lambda x:-len(x[1])): print(len(v),k)
