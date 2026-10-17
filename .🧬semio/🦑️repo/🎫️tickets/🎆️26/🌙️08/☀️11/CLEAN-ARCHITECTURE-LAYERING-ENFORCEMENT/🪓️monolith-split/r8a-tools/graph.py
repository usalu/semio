import json,sys,re
n=json.load(open(sys.argv[1]))
skip=sys.argv[2] if len(sys.argv)>2 else 'commands'
for k,v in n.items():
    if skip and skip in k: continue
    outs=sorted({t.split('::',2)[-1][-30:] for t,w in v['out'] if '[test]' not in w})
    print(f"{v['lines']:5} {k[-40:]:40} -> {', '.join(outs)[:300]}")
