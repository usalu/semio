import sys,os
sys.path.insert(0,os.path.dirname(__file__))
import crates,fixrt
apply="--apply" in sys.argv
shorts=[a for a in sys.argv[1:] if not a.startswith("--")]
md=crates.manifest_dirs()
for s in shorts:
    tot=0; files=crates.test_files(s)
    for f in files:
        t=open(f).read()
        n,c,man=fixrt.rewrite_text(t,f,md[s])
        tot+=c
        if man: print("MANUAL",s,f.split("subsets/")[-1],[t[x[1]:x[1]+160].replace("\n"," ") for x in man])
        if apply and n!=t:
            fixrt.save_orig(f); open(f,"w").write(n)
    print(("applied" if apply else "dry"),s,tot,"sites",len(files),"files")
