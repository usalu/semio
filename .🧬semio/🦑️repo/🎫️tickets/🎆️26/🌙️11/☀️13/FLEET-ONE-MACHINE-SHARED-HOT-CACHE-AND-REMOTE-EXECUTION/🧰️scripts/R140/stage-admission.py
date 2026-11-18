import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
import importlib.util
spec=importlib.util.spec_from_file_location("pa",os.path.dirname(os.path.abspath(__file__))+"/patch-admission.py");pa=importlib.util.module_from_spec(spec);spec.loader.exec_module(pa)
out=sys.argv[1]
src=absolutize(pa.patch(read(pa.PATH)),os.path.dirname(pa.PATH))
write(out+"/admission.ts",src)
tdir=os.path.dirname(os.path.abspath(__file__))+"/new/source-owners"; vdir=os.path.dirname(pa.PATH)+"/🧪️tests/🧭️source-owners"
tt=read(tdir+"/🟦️.ts").replace('"../../🟦️.ts"','"'+out+'/admission.ts"')
tt=absolutize(tt,vdir)
write(out+"/source-owners.test.ts",tt)
write(out+"/🐍️.py",read(tdir+"/🐍️.py"))
