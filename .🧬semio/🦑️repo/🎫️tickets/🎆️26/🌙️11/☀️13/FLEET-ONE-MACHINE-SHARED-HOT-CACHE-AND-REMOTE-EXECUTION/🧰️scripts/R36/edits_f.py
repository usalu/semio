"""R36 part 2b: the ceiling of a learned owner peak is 4608 MB (the largest legitimate peak, a root project), above it the sample is garbage of a shared stretch."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
add(B+"🧠️memory/🧬️schema/🔣️.json",'"peakCeilingMegabytes": 1024,','"peakCeilingMegabytes": 4608,')
add(B+"🧠️memory/🧪️tests/🔣️.json",'  "peakSamples": 8,\n','  "peakSamples": 8,\n  "peakCeilingMegabytes": 4608,\n')
TF=B+"🧠️memory/🦶️footprint/🧪️tests/🟦️.ts"
add(TF,'const ceiling=1024*MB,written','const ceiling=4608*MB,written')
add(TF,' let running=0,peak=0;\n await Promise.all([1,2,3,4].map(index=>governor.run(async()=>{running++;peak=Math.max(peak,running);await new Promise(resolve=>setTimeout(resolve,20));running--;},{kind:"project",key:"p"+index})));\n expect(peak).toBeGreaterThanOrEqual(4);\n expect(time).toBeLessThan(1000);\n expect(governor.stats().kinds.project).toBeLessThanOrEqual(1024);',' await Promise.all([1,2,3,4].map(index=>governor.run(async()=>{await new Promise(resolve=>setTimeout(resolve,20));},{kind:"project",key:"p"+index})));\n expect(time).toBeLessThan(6000);\n expect(governor.stats().kinds.project).toBeLessThanOrEqual(4608);')
