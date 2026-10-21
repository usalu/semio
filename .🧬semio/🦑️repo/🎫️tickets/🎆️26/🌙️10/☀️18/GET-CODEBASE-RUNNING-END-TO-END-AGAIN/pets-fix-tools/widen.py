import re,sys,os,subprocess
G=sys.argv[1]
root='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🐾️pets/🔨️modules'
names=set()
for l in open(G+'/errs.txt'):
    m=re.search(r'error\[E0603\]: (?:function|constant|struct|enum|type alias|trait) `([^`]+)` is private',l)
    if m: names.add(m.group(1))
mods=['📝️draft','📏️spacing','🗓️schedule','👀️attention','🚶️locomotion']
files=[]
for m in mods:
    for dp,dn,fn in os.walk(os.path.join(root,m)):
        if '🧪️' in dp: continue
        for f in fn:
            if f.endswith('.rs'): files.append(os.path.join(dp,f))
for n in sorted(names):
    hit=False
    for f in files:
        s=open(f,encoding='utf8').read()
        pat=re.compile(r'^(pub\((?:crate|super)\) )?(fn|const|struct|enum|type) '+re.escape(n)+r'\b',re.M)
        if pat.search(s):
            s2=pat.sub(lambda m:'pub '+m.group(2)+' '+n,s,count=1)
            open(f,'w',encoding='utf8').write(s2); print('widened',n,f.split('/')[-2]); hit=True
    if not hit: print('NOTFOUND',n)
