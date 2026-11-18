import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stagelib import *
g=Group("J")
p=L+"/🌡️host-capacity/🧪️tests/🔼️upgrade/🟦️.ts"
g.sub(p,' },{write:(line:string)=>lines.push(line)});\n expect(lines.some(line=>/^\\[capacity\\] run: forecast quiet because',' },{write:(line:string)=>lines.push(line),observe:async()=>Math.floor(buildPolicy.hostCapacity.estimates.quietReferenceMegabytes*1.2)});\n expect(lines.some(line=>/^\\[capacity\\] run: forecast quiet because')
p=L+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🌳️tree/🟦️.ts"
g.sub(p,'expect(calm.detail).toBe("[repo] still working");','expect(calm.ok).toBe(false);expect(calm.detail,"a caller that stops waiting detaches from the read within its grant, it never stops it as an overrun").toBe("detached");')
g.save()
