import jsonpatch from "fast-json-patch";
import fixture from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixtures/🪆️derived-read/🔣️.json";
const row=fixture.checkpointRead;
const first=jsonpatch.applyPatch({live:row.checkpoint,selected:null} as {live:{n:number};selected:null|{n:number}},[{op:"copy",from:"/live",path:"/selected"},{op:"replace",path:"/live/n",value:row.live.n}],true,false).newDocument;
if(JSON.stringify(first.selected)!==JSON.stringify(row.checkpoint)||JSON.stringify(first.live)!==JSON.stringify(row.live)||first.selected===first.live)throw Error("checkpoint selection substituted live root");
const results:Record<string,boolean>={};
for(const name of ["genuine","foreign","stale"] as const){const original={owner:name==="foreign"?"foreign":"actual",current:name!=="stale",read:null};try{jsonpatch.applyPatch(original,[{op:"test",path:"/owner",value:"actual"},{op:"test",path:"/current",value:true},{op:"copy",from:"/owner",path:"/read"}],true,false);results[name]=true;}catch(error){if(!(error instanceof Error)||error.name!=="TEST_OPERATION_FAILED")throw error;results[name]=false;}if(results[name]!==row.admission[name])throw Error(name);}
const next=jsonpatch.applyPatch(first,[{op:"replace",path:"/live/n",value:row.next.n}],true,false).newDocument;
if(JSON.stringify(next.selected)!==JSON.stringify(row.checkpoint)||JSON.stringify(next.live)!==JSON.stringify(row.next))throw Error("selected checkpoint changed with live frontier");
console.log(JSON.stringify({selected:first.selected,live:first.live,next:next.live,admission:results,grantAxes:row.underfundedAxes}));
