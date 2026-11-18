"""🧯 R95: an interleaved or repeated phase report with a smaller count is dropped instead of ending the slice read (g1-m10: `Monotonic graph phase progress required` after `decisions`)."""
import os,sys,subprocess,datetime,time
ROOT="/Users/ueli/Documents/semio"
T=os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
L=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load"
SRC,TEST=L+"/🟨️.mjs",L+"/🧪️tests/🟦️.ts"
BUN=ROOT+"/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
OLD="const active=new Set();let chain=Promise.resolve(),reportedAt=-Infinity;"
NEW="const active=new Set(),seen=new Map();let chain=Promise.resolve(),reportedAt=-Infinity;"
OLD2="await clock.begin(phase,total);active.add(phase);}\n  await clock.advance(phase,done);"
NEW2="await clock.begin(phase,total);active.add(phase);seen.set(phase,-1);}\n  else if(done<seen.get(phase))return;\n  seen.set(phase,done);await clock.advance(phase,done);"
LAW='''
test("a phase report that falls behind an interleaved one is dropped and never ends the load",async()=>{
 const{createGenerationProgressV1}=await import("../🟨️.mjs"),{createGraphPhaseClockV1}=await import("../../../⏱️phases/🟨️.mjs");
 const published:string[]=[],progress=createGenerationProgressV1(createGraphPhaseClockV1({publish:resource=>{published.push(resource);},minimumMilliseconds:0}) as never);
 await progress({phase:"decisions",done:5,total:10});
 await progress({phase:"decisions",done:3,total:12});
 await progress({phase:"decisions",done:12,total:12});
 await progress({phase:"decisions",done:2,total:4});
 await progress({phase:"decisions",done:4,total:4});
 expect(published.some(resource=>resource.includes("decisions"))).toBe(true);
});
'''
def run():
    p=subprocess.run([BUN,"test",TEST,"-t","phase report that falls behind|pass through the generation progress"],cwd=ROOT,capture_output=True,text=True,timeout=240)
    return p.returncode,(p.stdout+p.stderr)[-1500:]
lock=T+"/landing.lock"
for _ in range(120):
    try:os.mkdir(lock);break
    except FileExistsError:time.sleep(5)
else:sys.exit("lock busy")
open(lock+"/owner","w").write("main-R95-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
src,test=open(SRC,encoding="utf-8").read(),open(TEST,encoding="utf-8").read()
try:
    assert src.count(OLD)==1 and src.count(OLD2)==1,"anchors"
    open(TEST,"w",encoding="utf-8").write(test.rstrip("\n")+"\n"+LAW)
    before=run();print("before rc",before[0])
    assert before[0]!=0,"law must be red before"
    open(SRC,"w",encoding="utf-8").write(src.replace(OLD,NEW).replace(OLD2,NEW2))
    after=run();print("after rc",after[0]);print(after[1])
    assert after[0]==0,"red after"
    open(T+"/landings.md","a",encoding="utf-8").write("\n## Package R95 landing (%s): a phase report behind an interleaved one is dropped\n- Cause: g1-m10 passed take and `decisions 17937 of 17937`, then the slice read ended with `Monotonic graph phase progress required` (`⏱️phases` advance, from `🕰️generations/🔌️load` progress handler): interleaved reports of one phase carried a smaller count.\n- Change: `createGenerationProgressV1` drops a report whose count is below the last one of the running phase; law added in `🕰️generations/🔌️load/🧪️tests/🟦️.ts` (red before, green after, run with bun -t). Standing preflight tests not run in this hold.\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
    print("LANDED")
except BaseException as e:
    open(SRC,"w",encoding="utf-8").write(src);open(TEST,"w",encoding="utf-8").write(test);print("RESTORED",repr(e))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
