import os,re,sys,shutil
LIVE="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/AC/scratch/"
def place(src,rel,rewrite=True):
    dst=SC+rel
    os.makedirs(os.path.dirname(dst),exist_ok=True)
    t=open(src).read()
    if rewrite:
        def fix(m):
            spec=m.group(1)
            if not spec.startswith("../"): return m.group(0)
            if os.path.exists(os.path.normpath(os.path.join(os.path.dirname(dst),spec))): return m.group(0)
            return '"'+os.path.normpath(os.path.join(os.path.dirname(LIVE+rel),spec))+'"'
        t=re.sub(r'"(\.\./[^"]*)"',fix,t)
    open(dst,'w').write(t)
if __name__=="__main__":
    place(sys.argv[1],sys.argv[2],len(sys.argv)<4)
