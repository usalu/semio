/** 🧯 Law: under hold's width gate one action needs two slots (its birth socket and its child's execution join), so births that fill the grant's jobs block their own children. Usage: bun audit10-tools/width-deadlock.ts <staged bazel host-capacity 🟦️.ts> */
const text=await Bun.file(process.argv[2]!).text(),start=text.indexOf("export function createBazelActionWidthV1"),end=text.indexOf("\n}\n",start)+3;
const source=new Bun.Transpiler({loader:"ts"}).transformSync(text.slice(start,end)).replace("export function","function");
const create=new Function(source+"\nreturn createBazelActionWidthV1;")() as <K>(jobs:()=>number|undefined,poll:number)=>{enter:(k:K)=>Promise<void>;leave:(k:K)=>void;running:()=>number;waiting:()=>number};
const settle=(p:Promise<void>,ms:number)=>Promise.race([p.then(()=>"started"),new Promise(r=>setTimeout(()=>r("blocked"),ms))]);
let red=0;
for(const jobs of [1,3,8]){
 const width=create<string>(()=>jobs,50),births=[...Array(jobs)].map((_,i)=>width.enter(`birth${i}`));
 await Promise.all(births);
 const children=[...Array(jobs)].map((_,i)=>settle(width.enter(`exec${i}`),400));
 const outcome=await Promise.all(children),blocked=outcome.filter(o=>o==="blocked").length;
 console.log(`[DEBUG] jobs=${jobs}: ${jobs} births running, their ${jobs} execution joins: ${blocked} blocked (running=${width.running()} waiting=${width.waiting()})`);
 if(blocked===jobs)red++;
 for(let i=0;i<jobs;i++){width.leave(`exec${i}`);width.leave(`birth${i}`);}
}
console.log(red===3?"DEADLOCK REPRODUCED (every action's child waits behind its own birth)":"no deadlock");
process.exit(0);
