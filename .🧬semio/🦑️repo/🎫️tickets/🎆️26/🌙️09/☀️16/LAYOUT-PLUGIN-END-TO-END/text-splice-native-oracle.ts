import fc from "fast-check";
import Ajv2020 from "ajv/dist/2020.js";
import nativeFixture from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🧪️tests/🔣️.json";
import nativeSchema from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🧪️tests/🧬️schema.json";
const validate=new Ajv2020({strict:true}).compile(nativeSchema);if(!validate(nativeFixture))throw Error(JSON.stringify(validate.errors));
import jp from "fast-json-patch";
import {readFile} from "node:fs/promises";
import {composeTextSplicesV1,textSpliceFromEditV1} from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🟦️.ts";
import fixture from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/✂️text-splice/🔣️.json";
type S={start:number,deleted:string,insert:string,before:string,after:string};
const chars=(s:string)=>Array.from(s);
function bounded(a:S,b:S,c=32){
 const p=[a.before,a.deleted,a.insert,a.after,b.before,b.deleted,b.insert,b.after].map(chars),[B,D,I,A,NB,ND,NI,NA]=p,w=[...B,...I,...A],origin=a.start-B.length,at=b.start-origin,end=at+ND.length;
 if(at<0||end>w.length)return{kind:"disjoint"};
 const seenB=Math.min(NB.length,at),seenA=Math.min(NA.length,w.length-end);
 const eq=(x:string[],y:string[])=>x.join("")===y.join("");
 if(!eq(w.slice(at,end),ND)||!eq(w.slice(at-seenB,at),NB.slice(NB.length-seenB))||!eq(w.slice(end,end+seenA),NA.slice(0,seenA)))return{kind:"disjoint"};
 const rotated=(s:string[],delta:number,i:number)=>s[((delta+i)%s.length+s.length)%s.length];
 const limits=(pos:number,inside:string[],outside:string[])=>{
  let l=pos,r=pos;if(!inside.length&&!outside.length)return[l,r];
  while(l>0){const delta=l-pos,x=w[l-1];if(inside.length&&rotated(inside,delta,inside.length-1)!==x||outside.length&&rotated(outside,delta,outside.length-1)!==x)break;l--}
  while(r+inside.length<w.length){const delta=r-pos,x=w[r+inside.length];if(inside.length&&rotated(inside,delta,0)!==x||outside.length&&rotated(outside,delta,0)!==x)break;r++}
  return[l,r];
 };
 const[rl,rr]=limits(B.length,I,D),[nl,nr]=limits(at,ND,NI),low=Math.max(rl,nl-I.length),high=Math.min(rr,nr+ND.length);
 if(low>high)return{kind:"disjoint"};
 const run=Math.max(low,Math.min(high,B.length)),next=Math.max(Math.max(nl,run-ND.length),Math.min(Math.min(nr,run+I.length),at));
 const orig=(k:number)=>k<run?w[k]:k<run+D.length?rotated(D,run-B.length,k-run):w[k-D.length+I.length];
 const typed=(k:number)=>k<next?w[k]:k<next+NI.length?rotated(NI,next-at,k-next):w[k-NI.length+ND.length];
 const olen=w.length-I.length+D.length,tlen=w.length-ND.length+NI.length;
 if(olen===tlen&&Array.from({length:olen},(_,i)=>orig(i)).every((x,i)=>x===typed(i)))return{kind:"cancelled"};
 const lo=Math.min(run,next),hi=Math.max(run+I.length,next+ND.length),oh=hi-I.length+D.length,th=hi-ND.length+NI.length;
 const leading=[...NB.slice(0,NB.length-seenB),...w.slice(0,lo)],trailing=[...Array.from({length:olen-oh},(_,i)=>orig(oh+i)),...NA.slice(seenA)];
 return{kind:"composed",splice:{start:origin+lo,deleted:Array.from({length:oh-lo},(_,i)=>orig(lo+i)).join(""),insert:Array.from({length:th-lo},(_,i)=>typed(lo+i)).join(""),before:leading.slice(Math.max(0,leading.length-c)).join(""),after:trailing.slice(0,c).join("")}};
}
const check=(a:S,b:S,c=32)=>{const actual=bounded(a,b,c),expected=composeTextSplicesV1(a,b,c);jp.applyPatch({actual},[{op:"test",path:"/actual",value:expected}],true,false);return actual};
let cases=0;
for(const row of fixture.compositions){const actual=check(row.net,row.next);jp.applyPatch({actual},[{op:"test",path:"/actual",value:row.composition}],true,false);cases++}
for(const row of fixture.typingRuns){let net:S|null=null;for(let i=1;i<row.texts.length;i++){const edit=textSpliceFromEditV1(row.texts[i-1],row.texts[i]);if(!edit)continue;if(net){check(net,edit);const result=bounded(net,edit);net=result.kind==="composed"?(result as any).splice:result.kind==="cancelled"?null:edit;}else net=edit;cases++}}
const text=fc.array(fc.constantFrom("a","b","😀","é","́"),{maxLength:24}).map(x=>x.join(""));
const vectors:{previous:S,next:S,context:number,expected:any}[]=[];
fc.assert(fc.property(text,text,text,fc.integer({min:0,max:40}),(a,b,c,context)=>{const p=textSpliceFromEditV1(a,b,context),n=textSpliceFromEditV1(b,c,context);if(!p||!n)return;const expected=check(p,n,context);if(vectors.length<96)vectors.push({previous:p,next:n,context,expected});cases++}),{seed:941177,numRuns:12000});
const short=fc.array(fc.constantFrom("a","b","😀"),{maxLength:8}).map(x=>x.join(""));
const splice=fc.record({start:fc.integer({min:0,max:20}),before:short,deleted:short,insert:short,after:short});
fc.assert(fc.property(splice,splice,fc.integer({min:0,max:40}),(a,b,c)=>{check(a,b,c);cases++}),{seed:941178,numRuns:24000});
if(process.argv[2]){
 const log=await readFile(process.argv[2],"utf8"),prefix="[DEBUG] [text-splice-native] ";
 const records=log.split("\n").filter(x=>x.startsWith(prefix)).map(x=>JSON.parse(x.slice(prefix.length)));
 const expectedRecords:any[]=fixture.compositions.map((row,index)=>({source:"fixture",index,actual:row.composition}));
 for(const row of fixture.typingRuns){let net:S|null=null;for(let i=1;i<row.texts.length;i++){const next=textSpliceFromEditV1(row.texts[i-1],row.texts[i]);if(!next)continue;if(net){const actual=composeTextSplicesV1(net,next);expectedRecords.push({source:"typing",previous:net,next,context:32,actual});net=actual.kind==="composed"?actual.splice:actual.kind==="cancelled"?null:next;}else net=next;}}
 let state=BigInt(nativeFixture.generative.seed);const random=()=>{state=(state*6364136223846793005n+1442695040888963407n)&((1n<<64n)-1n);return state};const text=()=>{let value="";const n=Number(random()%25n);for(let i=0;i<n;i++)value+=nativeFixture.generative.alphabet[Number(random()%5n)];return value};
 let count=0;while(count<nativeFixture.generative.cases){const old=text(),middle=text(),final=text(),context=Number(random()%41n),previous=textSpliceFromEditV1(old,middle,context),next=textSpliceFromEditV1(middle,final,context);if(!previous||!next)continue;expectedRecords.push({source:"generated",previous,next,context,actual:composeTextSplicesV1(previous,next,context)});count++;}
 jp.applyPatch({actual:records},[{op:"test",path:"/actual",value:expectedRecords}],true,false);
 const cancelPrefix="[DEBUG] [text-splice-cancel] ",cancel=log.split("\n").filter(x=>x.startsWith(cancelPrefix)).map(x=>JSON.parse(x.slice(cancelPrefix.length)));
 for(const row of cancel)if(typeof row.readyAtCancel!=="boolean")throw Error("actual cancellation readiness missing");
 if(cancel[0]?.readyAtCancel!==false)throw Error("initial cancellation cut was vacuous");
 jp.applyPatch({actual:cancel.map(({cut,closed,unchanged})=>({cut,closed,unchanged}))},[{op:"test",path:"/actual",value:nativeFixture.cancelCuts.map(cut=>({cut,closed:true,unchanged:true}))}],true,false);
 const expiryPrefix="[DEBUG] [text-splice-expiry] ",expiry=log.split("\n").filter(x=>x.startsWith(expiryPrefix)).map(x=>JSON.parse(x.slice(expiryPrefix.length)));
 jp.applyPatch({actual:expiry},[{op:"test",path:"/actual",value:[{closed:true,renewed:false,unchanged:true}]}],true,false);
}
console.log(JSON.stringify({cases,vectors:vectors.length,fastCheck:true,jsonPatch:true,nativeCompared:!!process.argv[2]}));
