import re, subprocess
R = "/Users/ueli/Documents/semio"
OLD = "semio-s-artifact-stdio-dwg-snapshot-objects"
DWG = R + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/Cargo.lock"
def blocks(t):
    head, *rest = re.split(r"(?m)^(?=\[\[package\]\]\n)", t)
    return head, rest
def name(b): return re.search(r'name = "([^"]+)"', b).group(1)
def deps(b):
    m = re.search(r"dependencies = \[\n(.*?)\]\n", b, re.S)
    return re.findall(r'"([^"]+)"', m.group(1)) if m else []
def setdeps(b, ds):
    body = "dependencies = [\n" + "".join(f' "{d}",\n' for d in ds) + "]\n"
    if re.search(r"dependencies = \[\n.*?\]\n", b, re.S): return re.sub(r"dependencies = \[\n.*?\]\n", lambda m: body, b, count=1, flags=re.S)
    return b + body
_, dwgb = blocks(open(DWG).read())
dm = {name(b): b for b in dwgb}
newn = sorted(n for n in dm if n.startswith(OLD + "-") or n == OLD)
cons = {n: [d for d in deps(b) if d.startswith(OLD)] for n, b in dm.items() if any(d.startswith(OLD) for d in deps(b))}
files = subprocess.run(["git","grep","-lI",'name = "'+OLD+'"',"--","*.lock"],cwd=R,capture_output=True,text=True).stdout.split("\n")
for f in [x for x in files if x]:
    p = R + "/" + f
    if p == DWG: continue
    t = open(p).read()
    head, bs = blocks(t)
    out = []
    for b in bs:
        n = name(b)
        if n == OLD:
            out += [dm[x] for x in newn]
            continue
        ds = deps(b)
        if OLD in ds:
            assert n in cons, (f, n)
            ds = sorted(set(d for d in ds if d != OLD) | set(cons[n]))
            b = setdeps(b, ds)
        out.append(b)
    import os
    tmp=os.path.join(os.path.dirname(p),".f24.tmp"); open(tmp,"w").write(head + "".join(out)); os.replace(tmp,p)
    print("fixed", f[:60])
