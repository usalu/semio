import sys
from pathlib import Path
src = Path(sys.argv[1]).read_text(encoding="utf-8")
m = 'await page.clock.runFor(frame.transition_elapsed??1000);'
assert m in src
new = '''const __cdp=index===3||index===12?await page.context().newCDPSession(page):null;if(__cdp){await __cdp.send("Profiler.enable");await __cdp.send("Profiler.start")}await page.clock.runFor(frame.transition_elapsed??1000);if(__cdp){const {profile}=await __cdp.send("Profiler.stop");const self=new Map();const byId=new Map(profile.nodes.map(n=>[n.id,n]));const dt=profile.timeDeltas;profile.samples.forEach((id,i)=>{const n=byId.get(id);const k=n.callFrame.functionName+" "+n.callFrame.url.split("/").slice(-2).join("/")+":"+n.callFrame.lineNumber;self.set(k,(self.get(k)||0)+(dt[i]||0))});console.error("[CPU] f"+index+" "+JSON.stringify([...self.entries()].sort((a,b)=>b[1]-a[1]).slice(0,12).map(([k,v])=>[k,Math.round(v/1000)])));await __cdp.detach()}'''
Path(sys.argv[2]).write_text(src.replace(m, new, 1), encoding="utf-8")
