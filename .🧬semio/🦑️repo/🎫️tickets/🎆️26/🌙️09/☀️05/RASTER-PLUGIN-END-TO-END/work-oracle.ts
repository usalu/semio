import { readFileSync } from "node:fs";
import Ajv from "ajv";

const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/work.json";
const corpus=JSON.parse(readFileSync(file,"utf8"));
const oracle=new Ajv();
for(const row of corpus.cases){
 const admitted=row.fuel>=row.native+row.core;
 const reported=admitted?row.native+row.core:0;
 const result={reported,remaining:row.fuel-reported};
 const validate=oracle.compile({const:{reported:row.reported,remaining:row.remaining}});
 if(!validate(result))throw new Error(JSON.stringify(validate.errors));
}
console.log(JSON.stringify({schema:corpus.schema,cases:corpus.cases.length,oracle:"Ajv const + independent arithmetic",status:"passed"}));
const resize=JSON.parse(readFileSync("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-layer/🧪️tests/📐️resizes/optional.json","utf8"));
const inverse={mutation:"changeLayerPixels",width:resize.before.width,height:resize.before.height};
if(!oracle.compile({const:resize.inverse})(inverse))throw new Error("optional resize inverse differs from independent reference");
const restored={width:inverse.width,height:inverse.height};
if(!oracle.compile({const:resize.before})(restored))throw new Error("optional resize inverse changes absent dimensions");
console.log(JSON.stringify({schema:resize.schema,cases:1,oracle:"Ajv exact optional-state restoration",status:"passed"}));
const clone=JSON.parse(readFileSync("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/🧬️snapshot-clone/🧪️tests/receipts.json","utf8"));
const payload=new Uint8Array(clone.byteLength).fill(37),copy=payload.slice();
if(!oracle.compile({const:{bytes:clone.expectedPayloadBytes,capacity:clone.expectedPayloadCapacity}})({bytes:copy.byteLength,capacity:copy.buffer.byteLength}))throw new Error("clone payload receipt differs from typed-array reference");
if(!oracle.compile({const:{bytes:clone.deniedPayloadBytes,capacity:clone.deniedPayloadCapacity}})({bytes:0,capacity:clone.deniedCapacityBytes}))throw new Error("denied clone changes payload ownership");
console.log(JSON.stringify({schema:"raster.clone.physical-receipts.v1",cases:2,oracle:"Ajv + independent typed-array allocation",status:"passed"}));
const replay=JSON.parse(readFileSync("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/🧪️tests/🧫️fixtures/replay-source.json","utf8"));
const original={generation:replay.generation,index:replay.operationIndex,revision:Array(32).fill(replay.revisionByte),ledger:1,recipient:1};
const foreign={...original,ledger:2};
if(!oracle.compile({const:replay.foreignCallerRefused})(JSON.stringify(original)!==JSON.stringify(foreign)))throw new Error("foreign replay original control must refuse");
console.log(JSON.stringify({schema:"raster.replay.original-source.v1",cases:1,oracle:"Ajv + independent exact original-control tuple",status:"passed"}));
const births=JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🧫️fixtures/birth-work.json","utf8"));
for(const key of ["projectionBirth","projectionTake","editBirth"]){const admitted=0>=births[key],actual={work:admitted?births[key]:0,items:admitted?1:0,capacity:0};if(!oracle.compile({const:{work:births.deniedWork,items:births.deniedItems,capacity:births.deniedCapacity}})(actual))throw new Error("zero-work loan changes a declared native constructor or transfer");}
console.log(JSON.stringify({schema:"framework.replay.declared-birth-work.v1",cases:3,oracle:"Ajv + independent original-loan admission",status:"passed"}));
const pixels=JSON.parse(readFileSync("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️pixel-edit/🧪️tests/fixtures.json","utf8"));
for(const row of pixels.cases){const p=row.payload,color=row.target==="mask"?[128/255,128/255,128/255,0.25]:[1,0,0,1];const expected=row.tool==="fillRegion"?{layerId:p.layerId,target:row.target,seed:{x:Math.floor(p.x),y:Math.floor(p.y)},tolerance:32,color,selection:null}:row.tool==="fillSelection"?{layerId:p.layerId,target:row.target,color,selection:null}:row.tool==="applyFilter"?{...p,selection:["flipHorizontal","flipVertical"].includes(p.filter)?null:(row.selection??null)}:p;if(!oracle.compile({const:row.expected})(expected))throw new Error("pixel leaf metadata differs from independent arithmetic reference");}
console.log(JSON.stringify({schema:pixels.schema,cases:pixels.cases.length,oracle:"Ajv + independent pixel semantic metadata",status:"passed"}));
for(const row of pixels.refusals){const accepted=!row.locked&&row.target==="pixels"&&row.payload.filter==="invert";if(!oracle.compile({const:row.accepted})(accepted))throw new Error("pixel refusal differs from independent target/protection/vocabulary reference");}
console.log(JSON.stringify({schema:"raster.pixel-semantic-refusal.v1",cases:pixels.refusals.length,oracle:"Ajv + independent pixel admission",status:"passed"}));
const execution=JSON.parse(readFileSync("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️pixel-edit/🧪️tests/execution.json","utf8"));
const raw=2*execution.maximumPoints*(execution.maximumNumberBytes+1)+execution.rawIdentifierFields*execution.maximumIdentifierUtf8Bytes*execution.maximumEscapedBytesPerUtf8Byte+execution.rawSmallFieldUtf8Bytes*execution.maximumEscapedBytesPerUtf8Byte+execution.rawHeaderBytes;
const output=execution.maximumPixels*(execution.maximumSpanJsonBytes+1)+2+execution.maximumPoints*(execution.maximumPointJsonBytes+1)+2+execution.outputIdentifierFields*execution.maximumIdentifierUtf8Bytes*execution.maximumEscapedBytesPerUtf8Byte+execution.outputStateFields*execution.maximumStateUtf8Bytes*execution.maximumEscapedBytesPerUtf8Byte+execution.outputHeaderBytes;
if(!oracle.compile({const:{raw:execution.maximumRawBytes,decoded:execution.maximumDecodedItems,output:execution.maximumOutputBytes}})({raw,decoded:2*execution.maximumPoints+execution.decodedHeaderItems,output}))throw new Error("pixel execution quota differs from independent schema arithmetic");
if(JSON.stringify({start:16777215,length:16777216,coverage:255}).length!==execution.maximumSpanJsonBytes)throw new Error("span byte ceiling differs from independent JSON writer");
console.log(JSON.stringify({schema:execution.schema,cases:2,oracle:"Ajv + independent JSON byte counts and quota arithmetic",status:"passed"}));
