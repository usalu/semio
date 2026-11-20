import os,re,sys,json
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R211"
R="/Users/ueli/Documents/semio"
def stage(orig,out,src=None):
    os.makedirs(out,exist_ok=True)
    for f in (os.listdir(orig) if src is None else [os.path.basename(src)]):
        p=os.path.join(orig,f) if src is None else src
        if os.path.isfile(p) and f.endswith((".ts",".mjs",".json",".mts")):
            t=open(p,encoding="utf8").read()
            def fix(m):
                spec=m.group(2)
                if not spec.startswith("."): return m.group(0)
                return m.group(1)+os.path.normpath(os.path.join(orig,spec))+m.group(3)
            t=re.sub(r'(from\s*")([^"]+)(")',fix,t)
            t=re.sub(r'(import\(\s*")([^"]+)(")',fix,t)
            t=re.sub(r'(new URL\(\s*")([^"]+)(",\s*import\.meta\.url)',lambda m:m.group(1)+os.path.normpath(os.path.join(orig,m.group(2)))+m.group(3) if m.group(2).startswith(".") else m.group(0),t)
            t=re.sub(r'(import\s*")([^"]+)(")',fix,t)
            open(os.path.join(out,f if src is None else '🟦️.ts'),"w",encoding="utf8").write(t)
if __name__=="__main__":
    stage(sys.argv[1],sys.argv[2],sys.argv[3] if len(sys.argv)>3 else None)
