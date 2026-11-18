import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stagelib import *
g=Group("A")
T=N+"/🧪️tests/"
# governed-dispatch
p=T+"🚦️governed-dispatch/🟦️.ts"
g.sub(p,'''named=all.filter(call=>call.callee==="phases?.running"&&call.text.includes('"inference"'));
 expect(named.length).toBe(1);''','''named=all.filter(call=>call.callee==="phases?.running"&&call.text.startsWith('phases?.running("inference",list'));
 expect(named.length).toBe(1);''')
g.sub(p,'''running=all.filter(call=>call.callee==="phases?.running"&&call.text.includes('"inference"'));
 expect(running.length).toBe(1);''','''running=all.filter(call=>call.callee==="phases?.running"&&call.text.startsWith('phases?.running("inference",list'));
 expect(running.length,"the warm wave names its own pseudo owner beside this call, it never publishes the running list").toBe(1);''')
# parent-budget
p=T+"🧹️parent-budget/🟦️.ts"
g.sub(p,'''expect(source).not.toContain("memoryUsage().rss");expect(source).not.toContain("collectAbove");''','''const all=calls();for(const call of all.filter(row=>["createParentCollectorV1","createGraphParentMemoryGovernorV1","collectResident"].includes(row.callee)))expect(call.text).not.toMatch(/memoryUsage|\\.rss|maxRSS/u);
 expect(source).toContain("const collectResident=()=>{parentCollector.afterProject();};");expect(source).not.toContain("collectAbove");''')
# inferred
p=T+"📈️inferred/🟦️.ts"
g.sub(p,'''expect(text).toContain("()=>{missed.add(name);return inferProjectBodyV1(projectEnvironment,");''','''expect(text).toContain('()=>{missed.add(name);if(farm?.exists?.())throw farmJobRefusedV1(name,root,"the owner (the farm gave no record)");return inferProjectBodyV1(projectEnvironment,');''')
# too-heavy
p=T+"🛑️too-heavy/🟦️.ts"
g.sub(p,'''const refusal=source.indexOf('refuseBuildV1("unserved","graph.owner-too-heavy"');''','''const refusal=source.indexOf('refuseBuildV1("unserved","graph.owner-too-heavy"',source.indexOf("if(tooHeavy.size>0)"));
 expect(source.indexOf("if(tooHeavy.size>0)")).toBeGreaterThan(source.indexOf('phases?.end("inference"'));''')
g.sub(p,'a 16 GB machine may give one project 1756 managed MB','a 16 GB machine may give one project 2048 managed MB')
g.sub(p,'expect(sixteen.limitBytes).toBe(1756*MB)','expect(sixteen.limitBytes,"(8192 admitted - 3072 parent floor) / factor 2 / margin 1.25").toBe(Math.floor((8192-3072)/2/1.25)*MB)')
g.sub(p,'available=1000*MB;expect(sixteen.soloLimitBytes!()).toBe(1756*MB);','available=1000*MB;expect(sixteen.soloLimitBytes!(),"never below the limit of an owner beside others").toBe(Math.floor((8192-3072)/2/1.25)*MB);')
g.sub(p,'Math.floor((16384-2048-3800)/2/1.25)*MB','Math.floor((16384-2048-3072)/2/1.25)*MB')
g.sub(p,'expect(small.soloLimitBytes!()).toBe(1024*MB);','expect(small.soloLimitBytes!(),"an 8 GB host: 8192 less the system reserve 1536 less the parent floor 3072, by factor and margin").toBe(Math.floor((8192-1536-3072)/2/1.25)*MB);')
g.save()
