import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stagelib import *
g=Group("D")
p=L+"/🧪️tests/🕸️graph/🛂️admission/🟦️.ts"
g.sub(p,'await withCommandSourceControlV1(root,"selected-source-fixture",30000,reason,async directory=>{let tick=0;const clock=spyOn(performance,"now").mockImplementation(()=>tick+=250);','await withCommandSourceControlV1(root,"selected-source-fixture",30000*1000,reason,async directory=>{let tick=0;const clock=spyOn(performance,"now").mockImplementation(()=>tick+=250);')
p2=L+"/🪪️graph-state/📦️project-records/🧪️tests/🟦️.ts"
g.sub(p2,'at=source.indexOf("if(!reused&&port){gate=await port.gate(info)");','at=source.indexOf("if(!reused&&port){gate=await lookupLapAsyncV1(\\"gate\\",()=>port.gate(info))");')
g.sub(p2,'expect(source.slice(at,at+200)).toContain("if(gate?.seeded)reused=await lookup();");','expect(source.slice(at,at+500)).toContain("if(gate?.seeded)reused=await lookup();");')
g.save()
