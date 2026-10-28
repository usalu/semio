import sys,os,re
src,dst=sys.argv[1:3]
subs=[tuple((x.split('@@@',1) if '@@@' in x else x.split('=>',1))) for x in sys.argv[3:]]
d=os.path.dirname(os.path.abspath(src))
s=open(src).read()
def fix(m):
    p=m.group(2)
    return m.group(1)+os.path.normpath(os.path.join(d,p))+m.group(3)
s=re.sub(r'(from\s+")(\.\.?/[^"]*)(")',fix,s)
s=re.sub(r'(import\()(?:)',r'\1',s)
for a,b in subs:
    assert a in s,a
    s=s.replace(a,b)
os.makedirs(os.path.dirname(dst),exist_ok=True)
open(dst,'w').write(s)
