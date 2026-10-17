import sys,re
sys.path.insert(0,__file__.rsplit("/",1)[0])
from edit_lib import *
p=sys.argv[1]; needle=sys.argv[2]; w=int(sys.argv[3]) if len(sys.argv)>3 else 300
t=read(p)
for m in re.finditer(re.escape(needle),t):
    ln=t.count("\n",0,m.start())+1
    print(f"--- line {ln} col {m.start()-t.rfind(chr(10),0,m.start())}")
    print(t[max(0,m.start()-w):m.end()+w])
