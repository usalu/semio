import re,os,sys,collections
sys.path.insert(0,os.environ.get("RSCK_DIR","/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🗑️generated/compile-review-a"))
from rsck import clean
T=sys.argv[1]
fields=set(open(T+'/defined-fields.txt').read().split())|set(open(T+'/defined-fns.txt').read().split())
res=collections.defaultdict(list)
for a in sys.argv[2:]:
    for dp,dn,fn in os.walk(a):
        dn[:]=[x for x in dn if x not in('node_modules','target','.git')]
        for f in fn:
            if not f.endswith('.rs'): continue
            p=os.path.join(dp,f)
            try: s=clean(open(p,encoding='utf8').read())
            except: continue
            for m in re.finditer(r'[\w\)\]]\s*\.\s*([a-z_][a-z_0-9]*)\b(?!\s*(?:\(|::))',s):
                n=m.group(1)
                if n not in fields: res[n].append('%s:%d'%(p.replace('/home/deck/semio/✏️s/🔌️plugins/📕️norm/',''),s.count('\n',0,m.start())+1))
for n,l in sorted(res.items()): print(n,len(l),l[0][-120:])
