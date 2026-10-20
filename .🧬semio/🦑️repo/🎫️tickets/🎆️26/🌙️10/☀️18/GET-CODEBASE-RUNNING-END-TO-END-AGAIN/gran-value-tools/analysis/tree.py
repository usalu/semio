"""🌳 Module-tree resolver: follows `#[path]` mounts and `include*!` from a crate lib root (the repo mounts every module by `#[path]`)."""
import os,re
ROOT=os.environ.get('GRAN_ROOT','/Users/ueli/Documents/semio')
PATH_RE=re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INC_RE=re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
def module_tree(libfile):
    seen=[];st=[libfile]
    S=set()
    while st:
        f=st.pop()
        f=os.path.normpath(f)
        if f in S or not os.path.isfile(f): continue
        S.add(f);seen.append(f)
        try: t=open(f,encoding='utf-8',errors='ignore').read()
        except Exception: continue
        d=os.path.dirname(f)
        for m in PATH_RE.finditer(t):
            st.append(os.path.join(d,m.group(1)))
        for m in INC_RE.finditer(t):
            st.append(os.path.join(d,m.group(1)))
    return seen
def libfile(crate_dir, c):
    lp=c['libpath'] or 'src/lib.rs'
    return os.path.normpath(os.path.join(ROOT,crate_dir,lp))
def text_of(files):
    return {f:open(f,encoding='utf-8',errors='ignore').read() for f in files}
