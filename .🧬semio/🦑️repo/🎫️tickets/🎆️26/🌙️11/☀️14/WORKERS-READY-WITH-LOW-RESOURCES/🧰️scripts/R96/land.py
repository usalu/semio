"""🛡️ R96: progress is advisory; no report (order, shape, clock or sink failure) can reject into a generation load. Lands only 22:00-22:05 or 22:30-22:35 UTC (waits up to 9 minutes for the window)."""
import os,sys,subprocess,datetime,time
ROOT="/Users/ueli/Documents/semio"
D=os.path.dirname(os.path.abspath(__file__))
T=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
LIB=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
L=LIB+"/🏗️build/🕸️graph/🕰️generations/🔌️load"
SRC,TEST=L+"/🟨️.mjs",L+"/🧪️tests/🟦️.ts"
BUN=ROOT+"/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
HARNESS=LIB+"/🧪️testing/🌱️genuine-birth/📜️script.ts"
STANDING=[LIB+"/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts",LIB+"/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts"]
A=" const handle=async({phase,done,total,running,decision})=>{"
B=" const apply=async({phase,done,total,running,decision})=>{"
C=" return event=>{const run=chain.then(()=>handle(event));chain=run.catch(()=>undefined);return run;};"
H=""" const handle=async event=>{
  const{phase,done,total,running}=event;
  if(typeof phase!=="string"||!/^[a-z]+$/u.test(phase))return;
  if(running===undefined&&(!Number.isSafeInteger(done)||done<0||!Number.isSafeInteger(total)||total<0))return;
  try{await apply(event);}catch{active.delete(phase);seen.delete(phase);try{await clock.end(phase);}catch{}}
 };
"""
NEWTEST=open(D+"/new-test.ts.txt",encoding="utf-8").read()
def bun(args):
    p=subprocess.run([BUN]+args,cwd=ROOT,capture_output=True,text=True,timeout=280)
    return p.returncode,(p.stdout+p.stderr)[-1200:]
def harness(f):
    return bun([HARNESS,"--file="+f,"--birth=setup","--root="+ROOT])
now=datetime.datetime.now(datetime.timezone.utc)
if os.environ.get("R96_FORCE")!="1":
    t0=time.time()
    while True:
        n=datetime.datetime.now(datetime.timezone.utc)
        if n.minute%30<5 and n.hour>=22 or n.day>16:break
        if time.time()-t0>540:sys.exit("window not reached "+n.isoformat())
        time.sleep(10)
lock=T+"/landing.lock"
for _ in range(30):
    try:os.mkdir(lock);break
    except FileExistsError:time.sleep(5)
else:sys.exit("lock busy")
if datetime.datetime.now(datetime.timezone.utc).minute%30>=5 and os.environ.get("R96_FORCE")!="1":
    os.rmdir(lock);sys.exit("window closed")
open(lock+"/owner","w").write("main-R96-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
src,test=open(SRC,encoding="utf-8").read(),open(TEST,encoding="utf-8").read()
try:
    assert src.count(A)==1 and src.count(C)==1 and "const apply=" not in src,"anchors"
    open(TEST,"w",encoding="utf-8").write(test.rstrip("\n")+"\n"+NEWTEST)
    before=bun(["test",TEST,"-t","no ordering, shape or failure"]);print("before rc",before[0]);print(before[1])
    assert before[0]!=0,"law must be red before"
    open(SRC,"w",encoding="utf-8").write(src.replace(A,B).replace(C,H+C))
    for f in [TEST]+STANDING:
        r=harness(f);print(os.path.basename(os.path.dirname(os.path.dirname(f))),"rc",r[0]);print(r[1][-300:])
        assert r[0]==0,"red after: "+f
    open(T+"/landings.md","a",encoding="utf-8").write("\n## Package R96 landing (%s UTC): progress can never reject into a generation load\n- Cause: the failure files of g1-m10 (`3a9fba88` 18:47Z `Closed graph phase and finite total required`, `79312331` 20:19Z `Monotonic graph phase progress required`) both predate R95 (20:35Z); their traces match the pre-R95 lines 274/275. R95 still left `NaN`, fractional, negative, oversized or foreign phase reports and failing clock or publish calls able to reject.\n- Change: `createGenerationProgressV1` validates each report and runs the clock behind a guard that resets the phase on any failure (`handle` over `apply`); law `no ordering, shape or failure of progress reports can reject a report and end the load` (300 random interleavings, red before, green after). Standing preflight tests green.\n"%datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d %H:%M"))
    print("LANDED")
except BaseException as e:
    open(SRC,"w",encoding="utf-8").write(src);open(TEST,"w",encoding="utf-8").write(test);print("RESTORED",repr(e))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
