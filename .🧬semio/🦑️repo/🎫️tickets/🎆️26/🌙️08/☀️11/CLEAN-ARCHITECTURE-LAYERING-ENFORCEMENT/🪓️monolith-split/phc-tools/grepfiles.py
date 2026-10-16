import json, re, sys
d=json.load(open('../🗑️generated/phc-closure-before.json'))
H="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/"
files=[]
for k in ('test native',):
    for r in d[k]:
        if r['kind']=='source' and r['path'] not in files: files.append('/Users/ueli/Documents/semio/'+r['path'])
pat=re.compile(sys.argv[1])
for p in files:
    src=open(p,encoding='utf-8').read()
    n=len(pat.findall(src))
    if n: print(n, p.replace(H,'H/'))
