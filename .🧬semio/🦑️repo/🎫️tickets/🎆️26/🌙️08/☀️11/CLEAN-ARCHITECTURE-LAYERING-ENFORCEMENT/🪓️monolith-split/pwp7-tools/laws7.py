import sys, os, re, json
sys.path.insert(0,'.')
from model7 import PLUG, ROOT, GEN
SNAP=GEN+'/pwp7-snap'
FAMILY_DIRS=['🧵️instance','🪜️maintenance-ladder','⏪️time-travel/🧵️instance','⏯️tool-run/🧵️instance']
def files_of(root_dirs, snap):
    out={}
    for d in root_dirs:
        for dp,dn,fn in os.walk(PLUG+d):
            dn[:]=[x for x in dn if x not in ('📦️packages','🧪️tests','node_modules')]
            for f in fn:
                if f.endswith('.rs'): out[os.path.join(dp,f)[len(PLUG):]]=os.path.join(dp,f)
    if snap:
        # pre-WP7 tree: snapshot files replace live ones; files created by WP7 disappear
        touched=json.load(open(GEN+'/pwp7-touched.json'))
        created=set(); 
        for st in touched: created|=set(st['created'])
        out={k:v for k,v in out.items() if k not in created}
        for dp,dn,fn in os.walk(SNAP):
            for f in fn:
                if f.endswith('.rs'):
                    rel=os.path.join(dp,f)[len(SNAP)+1:]
                    if any(rel.startswith(d+'/') for d in FAMILY_DIRS): out[rel]=os.path.join(dp,f)
    return out
def text_of(files): return '\n'.join(open(p,encoding='utf-8').read() for k,p in sorted(files.items()))
def strip(t): return re.sub(r'\s+','',t)
def literals(t):
    out=[]
    for m in re.finditer(r'"((?:[^"\\\n]|\\.)*)"',t):
        s=m.group(1)
        if len(s)>=14: out.append(s)
    return out
def unesc(s):
    return s.replace('\\"','"').replace('\\\\','\\').replace('\\n','\n')
if __name__=='__main__':
    pre=strip(text_of(files_of(FAMILY_DIRS,True))); post=strip(text_of(files_of(FAMILY_DIRS,False)))
    tests=sys.argv[1:]
    for tf in tests:
        t=open(ROOT+tf,encoding='utf-8').read()
        bad=[]
        for s in set(literals(t)):
            u=strip(unesc(s))
            if len(u)<10: continue
            if u in pre and u not in post: bad.append(s)
        print(tf.split('/')[-2],len(bad))
        for s in bad: print('   ',s[:230])
