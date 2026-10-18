import sys,os,re,subprocess,json
R="/Users/ueli/Documents/semio/"
LIT=re.compile(r'(["\'`])((?:(?!\1)[^\\\n]|\\.)*?🧫️fixtures(?:(?!\1)[^\\\n]|\\.)*?)\1')
def tracked_candidates(src):
    # candidate files: tracked files in the ancestors up to the module/product anchor that mention fixtures
    out=set()
    d=os.path.dirname(src)
    for up in range(0,9):
        base=os.path.normpath(os.path.join(d,*[".."]*up))
        if len(base)<len(R+"🧰️framework/")+2: break
        r=subprocess.run(["git","-c","core.fsmonitor=false","-c","core.quotepath=false","grep","-l","-F","🧫️fixtures","--",os.path.relpath(base,R)],cwd=R,capture_output=True,text=True)
        out.update(x for x in r.stdout.split("\n") if x)
    return sorted(out)
def plan(moves,write):
    changes=[]
    srcs={os.path.normpath(R+s):os.path.normpath(R+d) for s,d in moves}
    cands=set()
    for s in srcs: cands.update(tracked_candidates(s))
    # extra: any ui/mesh entries
    for rel in sorted(cands):
        p=R+rel
        if not rel.endswith((".rs",".ts",".tsx",".json",".md")): continue
        try: text=open(p,encoding="utf8").read()
        except Exception: continue
        new=text; n=0
        for m in list(LIT.finditer(text)):
            lit=m.group(2)
            if "\n" in lit: continue
            base=os.path.dirname(p)
            hit=None
            for src,dst in srcs.items():
                # relative to file dir
                cand=os.path.normpath(os.path.join(base,lit))
                if cand==src: hit=("rel",src,dst);break
                # module-root-relative (test_inputs)
                parts=base.split("/")
                for k in range(len(parts)-1,0,-1):
                    root="/".join(parts[:k])
                    if os.path.normpath(os.path.join(root,lit))==src and not lit.startswith("."): hit=("root",src,dst,root);break
                if hit: break
            if not hit: continue
            if hit[0]=="rel": repl=os.path.relpath(hit[2],base)
            else: repl=os.path.relpath(hit[2],hit[3])
            if hit[0]=="rel" and lit.startswith(".") and not repl.startswith("."): repl="./"+repl
            if hit[0]=="rel" and not lit.startswith(".") : # bare relative like "🧫️fixtures/x"
                pass
            new=new.replace(m.group(0),m.group(1)+repl+m.group(1),1); n+=1
            changes.append((rel,lit,repl))
        if write and n: open(p,"w",encoding="utf8").write(new)
    return changes
def main():
    write="--write" in sys.argv
    spec=json.load(open(sys.argv[1],encoding="utf8"))
    moves=spec["moves"]
    ch=plan(moves,write)
    for c in ch: print(c[0][-90:],"|",c[1][-60:],"->",c[2][-60:])
    if write:
        for s,d in moves:
            os.makedirs(os.path.dirname(R+d),exist_ok=True); os.rename(R+s,R+d)
            print("moved",s[-70:])
main()
