import {readFile} from "node:fs/promises";
import jsonpatch from "fast-json-patch";
import fixture from "../../../../../../../✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️transform-tool-transactions/🔣️.json";
const start={document:[0,0],preview:[0,0],historyRows:0,sourceActive:false};
const tick=jsonpatch.applyPatch(start,[{op:"replace",path:"/preview",value:fixture.tick},{op:"replace",path:"/sourceActive",value:true}],true,false).newDocument;
const cancelled=jsonpatch.applyPatch(tick,[{op:"copy",from:"/document",path:"/preview"},{op:"replace",path:"/sourceActive",value:false}],true,false).newDocument;
const actual={preview:tick.preview,committed:cancelled.document,cancelled:cancelled.preview,historyRows:cancelled.historyRows,sourceClosed:!cancelled.sourceActive};
const expected={preview:fixture.preview,committed:fixture.committed,cancelled:fixture.cancelled,historyRows:fixture.historyRows,sourceClosed:fixture.sourceClosed};
if(JSON.stringify(actual)!==JSON.stringify(expected))throw Error("original follower lifecycle oracle mismatch");
if(process.argv[2]){const log=await readFile(process.argv[2],"utf8");const line=log.split("\n").find(line=>line.startsWith("original-follower-lifecycle="));if(!line)throw Error("actual native lifecycle record missing");const native=JSON.parse(line.slice("original-follower-lifecycle=".length));for(const key of Object.keys(actual))if(JSON.stringify(native[key])!==JSON.stringify(actual[key]))throw Error("native lifecycle differs: "+key);}
console.log(JSON.stringify(actual));

const settlementRecords=[];
for(const cancel of [false,true]){
 const initial={input:fixture.settlement.members,staged:[] as string[],committed:false,closed:false};
 const partial=jsonpatch.applyPatch(initial,[{op:"copy",from:"/input/1",path:"/staged/0"},{op:"remove",path:"/input/1"}],true,false).newDocument;
 const final=jsonpatch.applyPatch(partial,cancel?[{op:"remove",path:"/staged/0"},{op:"replace",path:"/closed",value:true}]:[{op:"copy",from:"/staged/0",path:"/input/1"},{op:"remove",path:"/staged/0"},{op:"replace",path:"/committed",value:true},{op:"replace",path:"/closed",value:true}],true,false).newDocument;
 const record={cancelled:cancel,members:final.input,restored:final.committed,ownerClosed:final.closed};
 if(JSON.stringify(record.members)!==JSON.stringify(cancel?fixture.settlement.cancelRemaining:fixture.settlement.committed))throw Error("staged settlement differs");settlementRecords.push(record);
}
if(process.argv[2]){const log=await readFile(process.argv[2],"utf8");const native=log.split("\n").filter(line=>line.startsWith("original-press-settlement=")).map(line=>JSON.parse(line.slice("original-press-settlement=".length)));if(native.length!==2)throw Error("actual settlement case count must be two");for(const expected of settlementRecords){const found=native.find(row=>row.cancelled===expected.cancelled);for(const key of Object.keys(expected))if(JSON.stringify(found?.[key])!==JSON.stringify(expected[key]))throw Error("native accepted settlement mismatch: "+key);}}
console.log(JSON.stringify({settlement:2,records:settlementRecords}));

const tags=fixture.tagCases.map(row=>{
 const input={source:row,tag:{window:"",tool:row.app+"#"+row.action,value:"",kind:""}};
 const ops=[{op:"copy",from:"/source/value",path:"/tag/value"},{op:"copy",from:"/source/kind",path:"/tag/kind"},...(row.window===null?[]:[{op:"copy",from:"/source/window",path:"/tag/window"}])];
 return jsonpatch.applyPatch(input,ops,true,false).newDocument.tag;
});
if(process.argv[2]){const log=await readFile(process.argv[2],"utf8");const native=log.split("\n").filter(line=>line.startsWith("original-tool-tag=")).map(line=>JSON.parse(line.slice("original-tool-tag=".length)));if(native.length!==tags.length)throw Error("actual original tag count differs");for(const expected of tags){const found=native.find(row=>row.kind===expected.kind);for(const key of Object.keys(expected))if(found?.[key]!==expected[key])throw Error("native original tag metadata differs: "+key);}}
console.log(JSON.stringify({originalTags:tags.length,utf8:true}));

const expiry=fixture.metadataExpiry;
const metadataModel={metadata:{actor:{text:expiry.actor},instance:expiry.instance},source:null,semanticOpen:false,retirement:false};
jsonpatch.applyPatch(metadataModel,[{op:"move",from:"/metadata",path:"/source"}],true,true);
const actorIdentity=metadataModel.source.actor;
let semanticBlocked=false;try{jsonpatch.applyPatch(metadataModel,[{op:"test",path:"/semanticOpen",value:true},{op:"copy",from:"/source",path:"/metadata"}],true,true);}catch(error){if(!(error instanceof Error)||error.name!=="TEST_OPERATION_FAILED")throw error;semanticBlocked=true;}
jsonpatch.applyPatch(metadataModel,[{op:"replace",path:"/retirement",value:true},{op:"test",path:"/retirement",value:true},{op:"move",from:"/source",path:"/metadata"}],true,true);
const metadataRecord={semanticBlocked,restored:metadataModel.metadata!==undefined,sameActorPointer:metadataModel.metadata.actor===actorIdentity,closed:metadataModel.source===undefined};
for(const key of Object.keys(metadataRecord))if(metadataRecord[key]!==expiry[key])throw Error("expired metadata restore differs: "+key);
if(process.argv[2]){const log=await readFile(process.argv[2],"utf8");const line=log.split("\n").find(line=>line.startsWith("original-metadata-expiry="));if(!line)throw Error("actual metadata expiry record missing");const native=JSON.parse(line.slice("original-metadata-expiry=".length));for(const key of Object.keys(metadataRecord))if(native[key]!==metadataRecord[key])throw Error("native expired metadata differs: "+key);}
console.log(JSON.stringify({metadataExpiry:metadataRecord}));

const typingRecords=[false,true].map(cancelled=>{
 const input={metadata:{actor:{text:fixture.typingMetadata.actor}},owner:null,replacement:{actor:{text:fixture.typingMetadata.replacement}},ready:false,suppressed:false};
 const pointer=input.metadata.actor;
 jsonpatch.applyPatch(input,[{op:"move",from:"/metadata",path:"/owner"}],true,true);
 const externalAbsentWhilePending=input.metadata===undefined;
 let replacementRefused=false;try{jsonpatch.applyPatch(input,[{op:"test",path:"/replacement",value:null},{op:"move",from:"/owner",path:"/replacement"}],true,true);}catch(error){if(!(error instanceof Error)||error.name!=="TEST_OPERATION_FAILED")throw error;replacementRefused=true;}
 jsonpatch.applyPatch(input,cancelled?[{op:"remove",path:"/owner"}]:[{op:"move",from:"/owner",path:"/metadata"},{op:"replace",path:"/ready",value:true},{op:"replace",path:"/suppressed",value:true}],true,true);
 return {cancelled,externalAbsentWhilePending,replacementRefused,restored:input.metadata!==undefined,sameActorPointer:input.metadata?.actor===pointer,ready:input.ready,suppressed:input.suppressed,closed:input.owner===undefined};
});
if(process.argv[2]){const log=await readFile(process.argv[2],"utf8");const native=log.split("\n").filter(line=>line.startsWith("original-typing-metadata=")).map(line=>JSON.parse(line.slice("original-typing-metadata=".length)));if(native.length!==2)throw Error("actual typing metadata cases must be two");for(const expected of typingRecords){const found=native.find(row=>row.cancelled===expected.cancelled);for(const key of Object.keys(expected))if(found?.[key]!==expected[key])throw Error("native typing metadata custody differs: "+key);}}
console.log(JSON.stringify({typingMetadata:typingRecords}));

const intentLaw=fixture.directIntentQualification;
const intentRecords=intentLaw.cases.map(row=>{
 const model={catalog:intentLaw.catalog,suppressed:false,document:fixture.committed,closed:false};
 let qualified=true;try{jsonpatch.applyPatch(model,[{op:"test",path:"/catalog/"+row.action,value:true},{op:"replace",path:"/suppressed",value:true}],true,true);}catch(error){if(!(error instanceof Error)||!["TEST_OPERATION_FAILED","OPERATION_PATH_UNRESOLVABLE"].includes(error.name))throw error;qualified=false;}
 jsonpatch.applyPatch(model,[{op:"replace",path:"/closed",value:true}],true,true);
 if(qualified!==row.qualified)throw Error("intent qualification differs from neutral catalog");
 return {action:row.action,qualified,suppressed:model.suppressed,unchanged:JSON.stringify(model.document)===JSON.stringify(fixture.committed),closed:model.closed};
});
if(process.argv[2]){const log=await readFile(process.argv[2],"utf8");const native=log.split("\n").filter(line=>line.startsWith("direct-intent-qualification=")).map(line=>JSON.parse(line.slice("direct-intent-qualification=".length)));if(native.length!==intentRecords.length)throw Error("actual direct intent count differs");for(const expected of intentRecords){const found=native.find(row=>row.action===expected.action);for(const key of Object.keys(expected))if(found?.[key]!==expected[key])throw Error("native direct intent qualification differs: "+key);}}
console.log(JSON.stringify({directIntentQualification:intentRecords}));

const requested={text:intentLaw.cases[1].action};
const rewriteModel={requested,canonical:intentLaw.cases[1].action,suppressed:false,lease:true,document:fixture.committed};
jsonpatch.applyPatch(rewriteModel,[{op:"replace",path:"/requested/text",value:intentLaw.changedAction}],true,true);
let rewriteRefused=false;try{jsonpatch.applyPatch(rewriteModel,[{op:"test",path:"/requested/text",value:rewriteModel.canonical},{op:"replace",path:"/suppressed",value:true}],true,true);}catch(error){if(!(error instanceof Error)||error.name!=="TEST_OPERATION_FAILED")throw error;rewriteRefused=true;}
jsonpatch.applyPatch(rewriteModel,[{op:"remove",path:"/lease"}],true,true);
const rewriteRecord={samePointer:rewriteModel.requested===requested,sameLength:rewriteModel.requested.text.length===rewriteModel.canonical.length,refused:rewriteRefused,unchanged:JSON.stringify(rewriteModel.document)===JSON.stringify(fixture.committed),closed:rewriteModel.lease===undefined};
const cancelModel={metadata:intentLaw.typing,args:intentLaw.typingCommit,lease:true,document:fixture.committed};
jsonpatch.applyPatch(cancelModel,[{op:"remove",path:"/lease"}],true,true);
const cancelRecord={unchanged:JSON.stringify(cancelModel.document)===JSON.stringify(fixture.committed),metadataRetained:cancelModel.metadata===intentLaw.typing,argsRetained:cancelModel.args===intentLaw.typingCommit,closed:cancelModel.lease===undefined};
if(process.argv[2]){const log=await readFile(process.argv[2],"utf8");for(const [prefix,expected]of [["direct-intent-qualification-rewrite=",rewriteRecord],["direct-intent-qualification-cancel=",cancelRecord]]as const){const line=log.split("\n").find(line=>line.startsWith(prefix));if(!line)throw Error("actual direct intent law record missing: "+prefix);const native=JSON.parse(line.slice(prefix.length));for(const key of Object.keys(expected))if(native[key]!==expected[key])throw Error("native direct intent custody differs: "+key);}}
console.log(JSON.stringify({directIntentRewrite:rewriteRecord,directIntentCancel:cancelRecord}));
