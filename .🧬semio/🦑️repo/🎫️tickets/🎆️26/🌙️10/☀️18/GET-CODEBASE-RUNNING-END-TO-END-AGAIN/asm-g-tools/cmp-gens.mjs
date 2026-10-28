// [asm-g] compares the current generations of two scratch stores owner by owner (fragment content, closure tokens normalised): usage bun cmp-gens.mjs <dirA> <dirB>
import{readFileSync}from"node:fs";import{join}from"node:path";import{brotliDecompressSync}from"node:zlib";
const load=d=>{const p=JSON.parse(readFileSync(join(d,"current.json"),"utf8")),m=JSON.parse(readFileSync(join(d,"m",p.manifest+".json"),"utf8"));return{d,p,m};};
const [A,B]=[load(process.argv[2]),load(process.argv[3])];
const frag=(g,row)=>JSON.parse(brotliDecompressSync(readFileSync(join(g.d,"o",row.fragment+".json.br"))).toString("utf8"));
const norm=v=>JSON.stringify(v).replace(/SEMIO_SOURCE_CLOSURE_[a-f0-9]{64}/g,"SEMIO_SOURCE_CLOSURE_X").replace(/SEMIO_CARGO_LOCK_[a-f0-9]{64}/g,"SEMIO_CARGO_LOCK_X");
console.log("generations",A.m.generation,B.m.generation,"owners",Object.keys(A.m.owners).length,Object.keys(B.m.owners).length);
const names=new Set([...Object.keys(A.m.owners),...Object.keys(B.m.owners)]);let only=0,same=0,rawDiff=0,diff=[];
for(const n of names){const a=A.m.owners[n],b=B.m.owners[n];if(!a||!b){only++;diff.push("only-in-"+(a?"A":"B")+" "+n);continue;}
 if(a.tag!==b.tag)diff.push("tag "+n+" "+a.tag+" "+b.tag);
 if(a.fragment===b.fragment){same++;continue;}rawDiff++;
 if(norm(frag(A,a))!==norm(frag(B,b)))diff.push("content "+n+" ("+(a.tag??"primary")+")");}
console.log("identical fragments",same,"differ only by closure tokens",rawDiff-diff.filter(x=>x.startsWith("content")).length,"only in one",only);
console.log("real differences",diff.length);for(const x of diff.slice(0,15))console.log("  ",x.slice(0,160));
