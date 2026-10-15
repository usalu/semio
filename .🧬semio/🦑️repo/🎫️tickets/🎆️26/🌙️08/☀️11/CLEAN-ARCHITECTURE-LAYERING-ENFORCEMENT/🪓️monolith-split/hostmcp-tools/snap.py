import shutil,os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from sbx import *
for name,rel in (("orig-host",OS+"🖥️host"),("orig-mcp",OS+"🔨️modules/🌉️mcp")):
    d=SCR+"snap/"+name
    shutil.rmtree(d,ignore_errors=True);os.makedirs(os.path.dirname(d),exist_ok=True)
    shutil.copytree(REPO+rel,d,symlinks=True,ignore=IGN)
    print(name,sum(len(f) for _,_,f in os.walk(d)))
