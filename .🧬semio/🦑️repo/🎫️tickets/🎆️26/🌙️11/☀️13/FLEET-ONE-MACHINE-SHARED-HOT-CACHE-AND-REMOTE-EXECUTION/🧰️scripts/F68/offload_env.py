import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f68lib import *
t=LIB+"/⚡️caching/🦀️cargo/🗃️unit-cache/🏗️buildscript/☁️offload/🧪️tests/🟦️.ts"
got,held=take({"caching-cargo"},8)
print("held",held)
try:
    if not held:
        print(edit(t,'const cargoEnvironment=(manifest:string,outDir:string,extra:Record<string,string>={})=>({','const cargoEnvironment=(manifest:string,outDir:string,extra:Record<string,string>={}):Record<string,string>=>({'))
finally:release(got)
