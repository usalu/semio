# adds a path dev-dependency: python3 ctc-adddev.py CONSUMER_PKG DEP_PKG [features]
import sys,os,subprocess
T=os.path.dirname(os.path.abspath(__file__))
R='/Users/ueli/Documents/semio/'
def mf(n):
    for l in open(T+'/../🗑️generated/ctc-manifests.tsv'):
        a,b=l.rstrip('\n').split('\t')
        if a==n: return R+b
    raise SystemExit('unknown '+n)
c,d=sys.argv[1],sys.argv[2]
feats=sys.argv[3].split(',') if len(sys.argv)>3 else []
cm,dm=mf(c),mf(d)
rel=os.path.relpath(os.path.dirname(dm),os.path.dirname(cm))
line=f'{d} = {{ path = "{rel}"'+(', features = ['+', '.join(f'"{x}"' for x in feats)+']' if feats else '')+' }'
s=open(cm).read()
if f'\n{d} =' in s.split('[dev-dependencies]')[-1] and '[dev-dependencies]' in s:
    print('present'); sys.exit(0)
if '[dev-dependencies]' in s:
    s=s.replace('[dev-dependencies]\n','[dev-dependencies]\n'+line+'\n',1)
else:
    s=s.rstrip('\n')+'\n\n[dev-dependencies]\n'+line+'\n'
open(cm,'w').write(s)
print('added',line)
