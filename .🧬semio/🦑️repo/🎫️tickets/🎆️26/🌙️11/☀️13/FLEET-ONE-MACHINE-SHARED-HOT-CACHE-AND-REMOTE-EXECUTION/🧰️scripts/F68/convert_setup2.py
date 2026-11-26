import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f68lib import *
t=LIB+"/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🧪️tests/🟦️.ts"
got,held=take({"caching-bootstrap"},8)
try:
    if not held:
        s=open(t,encoding="utf8").read()
        s=s.replace("type FixtureHostV1=SetupHostV1;\n","").replace("host:FixtureHostV1","host:SetupHostV1").replace("row.kind as SetupKindV1","row.kind").replace("row.host as SetupHostV1","row.host")
        write(t,s);print("ok")
finally:release(got)
