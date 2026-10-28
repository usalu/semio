#!/usr/bin/env python3
"""[obs-g] adds the startup-law rows of the fleet status command (and the exception for the Bazel status leaf) to a workspace tree. usage: apply_startup.py <workspace dir> <fleet modules> <fleet bytes> <door modules> <door bytes> <leaf modules> <leaf bytes>"""
import sys
W=sys.argv[1].rstrip("/")+"/";fm,fb,dm,db,lm,lb=[int(x) for x in sys.argv[2:8]]
p=W+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/🚦️startup/🔣️.json"
t=open(p,encoding="utf8").read()
if '"fleet-entry"' in t: sys.exit("already applied")
old='''      "id": "tools",
      "match": [
        "/🏗️build/🔧️tools/"
      ]
    },'''
new='''      "id": "tools",
      "match": [
        "/🏗️build/🔧️tools/"
      ],
      "except": [
        "/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/📖️status/"
      ]
    },'''
assert t.count(old)==1;t=t.replace(old,new)
def row(id_,kind,key,value,forbid,mods,size):
    f=",\n".join('        "%s"'%x for x in forbid)
    return '    {\n      "id": "%s",\n      "kind": "%s",\n      "%s": %s,\n      "forbid": [\n%s\n      ],\n      "maximumModules": %d,\n      "maximumBytes": %d\n    },\n'%(id_,kind,key,value,f,mods,size)
heavy=["graph","native-build","test-harness","cargo","caller","invocation","selection","resident","bootstrap","store-receipt","tools","revision","routing","leases","host-capacity","sealed"]
fleet_forbid=["graph","cargo","caller","invocation","selection","resident","bootstrap","store-receipt","tools","revision"]
rows=row("fleet-entry","import","module",'"../../../🚢️fleet-status/⌨️command/🟦️.ts"',fleet_forbid,fm,fb)+row("fleet-door","door","args",'["repo", "fleet", "status"]',fleet_forbid,dm,db)+row("bazel-status-leaf","import","module",'"../../../🔧️tools/🏗️bazel/🧵️sessions/📖️status/🟦️.ts"',heavy,lm,lb)
anchor='    {\n      "id": "leases-entry",'
assert t.count(anchor)==1;t=t.replace(anchor,rows+anchor)
open(p,"w",encoding="utf8").write(t);print("rows added")
