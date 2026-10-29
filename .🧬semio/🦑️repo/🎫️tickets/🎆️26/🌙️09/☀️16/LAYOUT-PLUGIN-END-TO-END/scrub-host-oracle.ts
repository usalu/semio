import {readFile} from "node:fs/promises";
import jsonpatch from "fast-json-patch";
import fixture from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🫧️host/🧪️tests/🔣️.json";
const pointer=(value:string)=>value.replaceAll("~","~0").replaceAll("/","~1");
const expected=fixture.cases.map(row=>{
 let state:any={open:structuredClone(row.open),closed:{}};
 if(row.roster.length)for(const window of Object.keys(state.open))if(!row.roster.includes(window))state=jsonpatch.applyPatch(state,[{op:"remove",path:"/open/"+pointer(window)}],true,false).newDocument;
 const tag=row.tag;
 if(tag&&(tag.abort||row.frozen)){state=jsonpatch.applyPatch(state,[{op:"add",path:"/closed/"+pointer(tag.window),value:tag.gesture}],true,false).newDocument;if(state.open[tag.window]===tag.gesture)state=jsonpatch.applyPatch(state,[{op:"remove",path:"/open/"+pointer(tag.window)}],true,false).newDocument;}
 const actual={open:Object.keys(state.open).sort(),closed:state.closed,settled:!!tag&&(tag.abort||!!row.frozen),tagRetained:!!tag&&!tag.abort&&!row.frozen};
 jsonpatch.applyPatch({actual},[{op:"test",path:"/actual",value:row.expected}],true,false);
 return{name:row.name,actual};
});
const registry=fixture.registry;
const admitted=Array.from({length:registry.limit},(_,operation)=>({operation,gesture:"g"+operation}));
const first=admitted[0].operation===0;
const taken=admitted[registry.takeOperation].operation;
const remaining=jsonpatch.applyPatch({admitted},[{op:"remove",path:"/admitted/"+registry.takeOperation}],true,false).newDocument.admitted;
const registryExpected={admitted:registry.limit,fullPreservesOriginal:true,firstRetained:first,taken,secondTake:false,remaining:remaining.length,pendingOwned:true};
jsonpatch.applyPatch({actual:registryExpected},[{op:"test",path:"/actual",value:registry.expected}],true,false);
if(process.argv[2]){
 const text=await readFile(process.argv[2],"utf8");
 const records=(prefix:string)=>text.split("\n").filter(line=>line.startsWith(prefix+" ")).map(line=>JSON.parse(line.slice(prefix.length+1)));
 jsonpatch.applyPatch({actual:records("[SCRUB-HOST]")},[{op:"test",path:"/actual",value:expected}],true,false);
 jsonpatch.applyPatch({actual:records("[TAG-REGISTRY]")},[{op:"test",path:"/actual",value:[registryExpected]}],true,false);
 jsonpatch.applyPatch({actual:records("[SCRUB-HOST-CANCEL]")},[{op:"test",path:"/actual",value:fixture.cancelCuts.map(cut=>({cut,unchanged:true,sourceClosed:true}))}],true,false);
}
console.log(JSON.stringify({cases:expected.length,cancelCuts:fixture.cancelCuts.length,nativeCompared:!!process.argv[2]}));
