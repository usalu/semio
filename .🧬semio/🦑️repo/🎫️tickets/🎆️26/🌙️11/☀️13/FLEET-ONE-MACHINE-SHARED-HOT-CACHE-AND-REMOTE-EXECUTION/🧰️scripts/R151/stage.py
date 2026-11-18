import sys,os
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
from lib import *
import patches as P
for path,fn in P.TARGETS.items():
    new=fn(read(path))
    out=SC+"/stage/"+path[len(ROOT)+1:]
    write(out,new)
    print("staged",path[-60:],len(read(path)),"->",len(new))
