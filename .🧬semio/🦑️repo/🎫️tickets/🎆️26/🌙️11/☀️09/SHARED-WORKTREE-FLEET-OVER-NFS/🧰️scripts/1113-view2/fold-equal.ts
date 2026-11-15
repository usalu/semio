const [moduleRoot,mirror]=process.argv.slice(2) as [string,string];
const {foldNameV1}=await import(`${moduleRoot}/🚧️boundary/🟦️.ts`);
const table=(await import(`${moduleRoot}/🚧️boundary/🔤️fold/🔣️.json`,{with:{type:"json"}})).default.map as [number,string][];
const FOLD=new Map<number,string>(table);
const reference=(name:string):string=>{
 if(/^[\u0000-\u007f]*$/u.test(name))return name.toLowerCase();
 let folded="";
 for(const char of name.normalize("NFC")){const mapped=FOLD.get(char.codePointAt(0) as number);folded+=mapped??(char>="A"&&char<="Z"?char.toLowerCase():char);}
 return folded.normalize("NFC");
};
let bad=0,count=0;
const check=(text:string)=>{count++;if(foldNameV1(text)!==reference(text)){bad++;if(bad<5)console.log("MISMATCH",JSON.stringify(text));}};
for(const [point] of table){check(String.fromCodePoint(point));check(`Ab${String.fromCodePoint(point)}Cd/é`);}
for(let point=0;point<0x30000;point+=1){if(point>=0xd800&&point<0xe000)continue;check(String.fromCodePoint(point));}
check("\ud800x");check("A\udc00B");check("ÄÖÜ ǅ ŉ ß ẞ SS ς Σ İ ı ſ");
const {Database}=await import("bun:sqlite").catch(()=>({Database:undefined}));
const {openReplicaV1,closeReplicaV1}=await import(`${moduleRoot}/🔁️replica/🟦️.ts`);
const machine=JSON.parse(await Bun.file(`${mirror}/.cache/f/machine.json`).text());
const replica=await openReplicaV1({worktree:mirror,storeRoot:machine.storeRoot,originId:machine.originId,groupId:machine.groupId,lock:false});
for(const id of replica.files.ids())check(replica.book.name(id));
await closeReplicaV1(replica);
console.log("checked",count,"mismatches",bad);
