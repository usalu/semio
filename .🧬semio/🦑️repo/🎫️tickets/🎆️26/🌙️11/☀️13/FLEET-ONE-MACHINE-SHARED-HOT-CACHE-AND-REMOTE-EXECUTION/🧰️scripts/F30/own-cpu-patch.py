import hashlib, sys, pathlib
B = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes")
TAKE = B / "📥️take" / "🟨️.mjs"
DTS = B / "📥️take" / "🟨️.d.mts"
NODES = B / "🟨️.mjs"
TEST = B / "📥️take" / "🧪️tests" / "🚦️no-progress" / "🟦️.ts"
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()[:12]

def edit(path, old, new, count=1):
    text = path.read_text(encoding="utf8")
    if new in text and old not in text:
        print("already applied", path.name); return
    if text.count(old) != count:
        sys.exit(f"anchor count {text.count(old)} != {count} in {path}: {old[:60]!r}")
    path.write_text(text.replace(old, new), encoding="utf8")

before = {p: sha(p) for p in (TAKE, DTS, NODES, TEST)}

edit(TAKE, "/** 🔥️ A sign of CPU progress for the watch:", '''/** 🥁️ The CPU milliseconds this process itself has used so far (user and system time): an owner inferred in this process (the farm declined it, so one thread walks its closure) shows no row for as long as that walk runs, and what that thread burns is progress as much as what a child burns.
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export function takeOwnCpuV1(usage = process.cpuUsage()) { return (usage.user + usage.system) / 1000; }

/** 🪵️ The CPU milliseconds of the work of a read: its child processes and this process itself; undefined where the children cannot be read.
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export async function takeWorkCpuV1(root = process.pid, run = execFile, usage = undefined) {
  const children = await takeChildrenCpuV1(root, run);
  return children === undefined ? undefined : children + takeOwnCpuV1(usage ?? process.cpuUsage());
}

/** 🔥️ A sign of CPU progress for the watch:''')
edit(TAKE, "and the processor time of the children while owners run.", "and the processor time of this process and its children while owners run.")
edit(DTS, "export function takeChildrenCpuV1(root?: number, run?: (...input: any[]) => unknown): Promise<number | undefined>;\n", "export function takeChildrenCpuV1(root?: number, run?: (...input: any[]) => unknown): Promise<number | undefined>;\nexport function takeOwnCpuV1(usage?: Readonly<{ user: number; system: number }>): number;\nexport function takeWorkCpuV1(root?: number, run?: (...input: any[]) => unknown, usage?: Readonly<{ user: number; system: number }>): Promise<number | undefined>;\n")
text = NODES.read_text(encoding="utf8")
if "sample:()=>takeChildrenCpuV1()" in text:
    if text.count("takeChildrenCpuV1") != 2: sys.exit("takeChildrenCpuV1 used elsewhere in project-nodes: " + str(text.count("takeChildrenCpuV1")))
    NODES.write_text(text.replace("takeChildrenCpuV1", "takeWorkCpuV1"), encoding="utf8")
else: print("already applied", NODES.name)
edit(TEST, "takeChildrenCpuV1,takeCpuMillisecondsV1,", "takeChildrenCpuV1,takeOwnCpuV1,takeWorkCpuV1,takeCpuMillisecondsV1,")
test = TEST.read_text(encoding="utf8")
if "takeWorkCpuV1()" not in test:
    test += '''
test("the CPU of this process is user plus system time in milliseconds and the work of a read adds it to its children",async()=>{
 expect(takeOwnCpuV1({user:1500000,system:500000})).toBe(2000);
 const table=" 1 0 0:01.00\\n 2 1 0:02.50\\n";
 const run=(_command:string,_args:string[],_options:unknown,done:(error:Error|null,out:string)=>void)=>done(null,table);
 const work=await takeWorkCpuV1(1,run,{user:3000000,system:1000000});
 expect(work).toBe(2500+4000);
 const failing=(_command:string,_args:string[],_options:unknown,done:(error:Error|null,out:string)=>void)=>done(new Error("no ps"),"");
 expect(await takeWorkCpuV1(1,failing,{user:1,system:1})).toBeUndefined();
});

test("the CPU of a thread of this process that burns is counted, one that sleeps adds none, and the operating system process table agrees",async()=>{
 const start=takeOwnCpuV1(),deadline=Date.now()+20000;
 while(takeOwnCpuV1()-start<300&&Date.now()<deadline);
 const burned=takeOwnCpuV1()-start;
 expect(burned).toBeGreaterThanOrEqual(300);
 const idle=takeOwnCpuV1();
 await new Promise(resolve=>setTimeout(resolve,400));
 expect(takeOwnCpuV1()-idle).toBeLessThan(200);
 if(process.platform!=="win32"){
  const{execFileSync}=await import("node:child_process"),table=takeCpuMillisecondsV1(execFileSync("ps",["-o","time=","-p",String(process.pid)],{encoding:"utf8"}).trim());
  expect(Math.abs(table-takeOwnCpuV1())).toBeLessThan(250);
 }
},30000);

test("an owner inferred in a busy parent never stalls the watch while a parent that burns nothing does",async()=>{
 let at=0,used=0;
 const watch=createProgressWatchV1({now:()=>at,windowMilliseconds:300000}),cpu=createCpuProgressV1({sample:()=>used,now:()=>at});
 for(let step=1;step<=90;step++){at=step*10000;used+=7000;expect(watch.check(["rows0",await cpu.tick()].join("|"))).toBeUndefined();}
 let hit;
 for(let step=1;step<=31&&hit===undefined;step++){at+=10000;used+=100;hit=watch.check(["rows0",await cpu.tick()].join("|"));}
 expect(hit).toEqual({sinceMilliseconds:300000});
});
'''
    TEST.write_text(test, encoding="utf8")

old_test = TEST.read_text(encoding="utf8")
a = old_test.find('test("a real child that burns CPU raises the sample')
b = old_test.find('test("the default window is the authored window')
if a >= 0 and b > a:
    fixed = '''test("a real child that burns CPU raises the sample of the children and one that sleeps does not",async()=>{
 const{spawn}=await import("node:child_process"),busy=spawn(process.execPath,["-e","while(process.cpuUsage().user+process.cpuUsage().system<800000);setTimeout(()=>{},30000);"],{stdio:"ignore"}),idle=spawn(process.execPath,["-e","setTimeout(()=>{},30000);"],{stdio:"ignore"});
 try{
  const first=await takeChildrenCpuV1(),deadline=Date.now()+25000;
  let second=first;
  while(Date.now()<deadline&&second!-first!<500){await new Promise(resolve=>setTimeout(resolve,250));second=await takeChildrenCpuV1();}
  expect(first).toBeDefined();expect(second!-first!).toBeGreaterThanOrEqual(500);
 }finally{busy.kill();idle.kill();}
},40000);

'''
    TEST.write_text(old_test[:a] + fixed + old_test[b:], encoding="utf8")
for p, b in before.items(): print(p.name, b, "->", sha(p))
