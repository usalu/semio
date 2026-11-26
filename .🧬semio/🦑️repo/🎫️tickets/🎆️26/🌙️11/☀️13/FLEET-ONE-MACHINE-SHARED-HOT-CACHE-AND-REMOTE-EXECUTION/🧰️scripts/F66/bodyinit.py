#!/usr/bin/env python3
"""F66 group: bytes handed to fetch/Response must be ArrayBuffer-backed (`Uint8Array<ArrayBuffer>`): protobuf encoder return type plus the `bytes:Uint8Array` parameters of the tests whose fetch bodies tsc refuses."""
import sys,collections;sys.path.insert(0,__import__("os").path.dirname(__file__))
from lib import *
tsc=sys.argv[1];apply=len(sys.argv)>2
txt=open(tsc,encoding="utf8").read().split("\n")
files=set()
for i,l in enumerate(txt):
    m=ERR.match(l)
    if m and "tests" in m.group(1) and any("'BodyInit" in x or "BufferSource" in x for x in txt[i+1:i+4] if not ERR.match(x)):
        files.add(os.path.normpath(os.path.join(TSCDIR,m.group(1))))
prod=LIB+"/🏗️build/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts"
edits=[(prod,[("view():Uint8Array{","view():Uint8Array<ArrayBuffer>{"),("export function encodeProtobufV1(name:string,value:ProtobufMessageV1):Uint8Array{","export function encodeProtobufV1(name:string,value:ProtobufMessageV1):Uint8Array<ArrayBuffer>{")])]
import re
for f in sorted(files):edits.append((f,None))
areas={area(f) for f,_ in edits}
got,held=locks(areas) if apply else ([],set())
try:
    for f,rep in edits:
        if area(f) in held or foreign(f):print("skip",f);continue
        s=open(f,encoding="utf8").read();o=s
        if rep:
            for a,b in rep:
                if a not in s:print("missing",a[:40]);continue
                s=s.replace(a,b)
        else:
            s=re.sub(r'(\b[A-Za-z_]\w*\??):Uint8Array(?=[,)=])',r'\1:Uint8Array<ArrayBuffer>',s)
        if s!=o:
            print("edit",f.split("/")[-3:-1],sum(1 for _ in re.finditer("Uint8Array<ArrayBuffer>",s))-sum(1 for _ in re.finditer("Uint8Array<ArrayBuffer>",o)))
            if apply:write(f,s)
finally:release(got)
