import json,sys
# usage: undo.py <undo.json> : restores the files recorded by a --write run (only my own tool runs create these)
d=json.load(open(sys.argv[1]))
for f,t in d.items(): open(f,'w',encoding='utf8').write(t)
print('restored',len(d))
