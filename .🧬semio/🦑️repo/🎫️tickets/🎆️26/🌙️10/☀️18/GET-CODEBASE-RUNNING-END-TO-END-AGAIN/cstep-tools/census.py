"""cstep census: Rust files that name the interactive-close API surface. usage: census.py roots..."""
import os,re,sys
SKIP=('node_modules','🗑️generated','target','.cache','dist','bazel-out','temp','.git','.venv','.nx')
pat=re.compile(r'InteractiveJob\b|InteractiveJobCloseStep|WorkerJobCloseStep|ArtifactCommandWork|FixedOperationOwner|fn close_step')
def walk(roots):
    for root in roots:
        for dp,dn,fn in os.walk(root):
            dn[:]=[d for d in dn if d not in SKIP and not d.startswith('.')]
            for f in fn:
                if f.endswith('.rs'): yield os.path.join(dp,f)
if __name__=='__main__':
    for p in walk(sys.argv[1:]):
        try:s=open(p,encoding='utf-8').read()
        except Exception: continue
        if pat.search(s): print(p)
