"""R36 part 2c: an ordinary kind is clamped to the grant of one project (so it is never admitted alone only after the wait bound nor refused), a root kind to the root ceiling."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
M=B+"🧠️memory/🟨️.mjs"
add(M,'.map(bytes=>Math.min(bytes,policy.peakCeilingMegabytes*MEGABYTE))]));','.map(bytes=>Math.min(bytes,ceilingOf(kind)))]));')
add(M,'Math.min(owner.taken,policy.peakCeilingMegabytes*MEGABYTE)','Math.min(owner.taken,ceilingOf(owner.kind))')
add(M,' const queue=[],owners=new Set(),stats=',' const ceilingOf=kind=>(String(kind).startsWith("root:")?policy.peakCeilingMegabytes:policy.grantMegabytes)*MEGABYTE;\n const queue=[],owners=new Set(),stats=')
TF=B+"🧠️memory/🦶️footprint/🧪️tests/🟦️.ts"
add(TF,'const ceiling=4608*MB,written','const ceiling=2048*MB,written')
add(TF,'toBeLessThanOrEqual(4608);','toBeLessThanOrEqual(2048);')
add(TF,'expect(time).toBeLessThan(6000);','expect(time).toBeLessThan(6000);\n expect(governor.stats().ownerCostMegabytes).toBeLessThanOrEqual(2048);')
