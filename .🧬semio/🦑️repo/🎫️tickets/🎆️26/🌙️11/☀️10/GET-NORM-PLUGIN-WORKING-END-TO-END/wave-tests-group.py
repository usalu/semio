import re,sys,collections
import subprocess
from pathlib import Path
T = Path(__file__).resolve().parent
R = Path(subprocess.check_output(['git', '-C', str(T), 'rev-parse', '--show-toplevel'], text=True).strip())
log=open(sys.argv[1],errors='replace').read().split('\n')
pre=str(R)+'/✏️s/🔌️plugins/📕️norm/🗿️artifacts/'
c=collections.defaultdict(collections.Counter)
for i,l in enumerate(log):
    if re.match(r'^error(\[E\d+\])?:',l) and 'could not compile' not in l and i+1<len(log):
        m=re.search(r'--> (\S+?):\d+',log[i+1])
        if not m: continue
        p=m.group(1)
        k=p.split('/📦️packages/🦀️rust')[0].replace(pre,'') if '/📦️packages/🦀️rust' in p else p.replace(pre,'')[:150]
        msg=re.sub(r'`[^`]*`','`_`',l)[:90] if False else l[:110]
        c[k][msg]+=1
for k,v in sorted(c.items()):
    print(k.split('/')[-1] if False else k[-110:])
    for m,n in v.most_common(6): print('   ',n,m)
