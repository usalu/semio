import sys,json
sys.path.insert(0,'.')
import mkcomp
for s in sys.argv[1:]:
    f=f'specs/{s}.json'
    spec=dict(json.load(open(f)),post=True,retransform=True)
    import fixrt
    idx={}
    for l in open(fixrt.ORIG+"/index.tsv"):
        h,sha,p=l.rstrip("\n").split("\t"); idx.setdefault(p,h)
    # plan on restored text without writing: read originals in memory by temporarily patching open? simpler: run retransform and print
    p=mkcomp.retransform(json.load(open(f)))
