import sys,os,difflib,re,shutil
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from roots import *
for k in sys.argv[1:]:
    f=os.path.join(find(roots[k]),'🦀️.rs')
    cur=open(f).read().split('\n'); org=open(os.path.dirname(os.path.abspath(__file__))+'/orig/%s.rs'%k).read().split('\n')
    for l in difflib.unified_diff(org,cur,lineterm='',n=0):
        if l[:1] in '+-' and not l.startswith(('+++','---')) and not re.match(r'(pub(\([^)]*\))?\s+)?use\s|#\[cfg|^$',l[1:].strip()):
            print(k,'FOREIGN CHANGE, abort',l[:100]); sys.exit(1)
    shutil.copy(os.path.dirname(os.path.abspath(__file__))+'/orig/%s.rs'%k,f)
