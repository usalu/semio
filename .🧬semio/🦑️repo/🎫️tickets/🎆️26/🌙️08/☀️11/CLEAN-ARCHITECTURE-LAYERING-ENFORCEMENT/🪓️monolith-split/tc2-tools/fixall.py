"""tc2: run-time fixtures for every test-only embedded input of a crate (also inside inline cfg(test) modules of production files).
usage: fixall.py [--apply] <short>... ; reads the closure lists from a census dump dir (default: baseline)"""
import sys,os,glob
sys.path.insert(0,os.path.dirname(__file__))
import crates,fixrt,testonly as T
args=[a for a in sys.argv[1:] if not a.startswith("--")]
apply="--apply" in sys.argv
dump=[a.split("=",1)[1] for a in sys.argv if a.startswith("--dump=")]
md=crates.manifest_dirs()
for short in args:
    t=T.load(glob.glob(f"{T.base}/tc2-inputs-semio-s-artifact-{short}-lib-test-*.txt")[0])
    n={r[3] for r in T.load(f"{T.base}/tc2-inputs-{short}-normal.txt")}
    targets={os.path.normpath(crates.ROOT+r[3]) for r in t if r[0]=="embedded" and r[3] not in n}
    srcs=[crates.ROOT+r[3] for r in t if r[0]!="embedded"]
    tot=0; touched=0
    for f in srcs:
        txt=open(f).read()
        new,c,man=fixrt.rewrite_text(txt,f,md[short],only=targets)
        if man: print("MANUAL",short,f.split("subsets/")[-1],[txt[x[1]:x[1]+140].replace("\n"," ") for x in man])
        if c:
            tot+=c; touched+=1
            if apply and new!=txt:
                fixrt.save_orig(f); open(f,"w").write(new)
    print(("applied" if apply else "dry"),short,"sites",tot,"files",touched,"test-only embedded targets",len(targets))
