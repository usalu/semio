#!/usr/bin/env python3
"""F66 group: typed `world(count,names)` harness of the selected-exchange tests returns the machines as a tuple of the names it was given, so the 19 call-site casts to a hand-written tuple type go away."""
import re,sys;sys.path.insert(0,__import__("os").path.dirname(__file__))
from lib import *
f=LIB+"/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🟦️.ts"
got,held=wait_locks({area(f)},int(sys.argv[1]) if len(sys.argv)>1 else 0)
try:
    if area(f) in held or foreign(f):print("held");sys.exit(0)
    s=open(f,encoding="utf8").read()
    old='const world=(count:number,names:string[])=>{const records=makeRecords(),fleet=makeFleet(),machines=names.map(name=>makeMachine({name,count,records}));return{records,fleet,machines};};'
    new='const world=<const Names extends readonly string[]>(count:number,names:Names)=>{const records=makeRecords(),fleet=makeFleet(),machines=names.map(name=>makeMachine({name,count,records})) as unknown as {-readonly [Index in keyof Names]:MachineV1};return{records,fleet,machines};};'
    if old not in s:print("world missing");sys.exit(1)
    s=s.replace(old,new)
    s,n=re.subn(r'\)\s*as\s*\{fleet:FleetV1;machines:\[(?:MachineV1,?)+\](?:;records:RecordsV1)?\}',')',s)
    print("casts removed",n)
    write(f,s)
finally:release(got)
