import jsonpatch from "fast-json-patch";
const law=await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🎟️capture/🧪️tests/🔣️.json").json();
const path=process.argv[2];
if(!path)throw new Error("actual native owned-emission log is required");
const text=await Bun.file(path).text();
const packets=text.split(/\r?\n/).filter(line=>line.includes("[OWNED-EMISSION] ")).map(line=>JSON.parse(line.slice(line.indexOf("[OWNED-EMISSION] ")+17)));
if(!packets.length)throw new Error("no actual native owned-emission records");
let records=0;
for(const packet of packets){
 const initial=packet.records[0];
 if(!initial)continue;
 const stable={operations:packet.originalOperations,metadata:packet.originalMetadata,capacity:packet.originalCapacity,values:law.ownedProducer.mutationValues};
 if(!Number.isSafeInteger(stable.operations)||stable.operations<=0||!Number.isSafeInteger(stable.metadata)||stable.metadata<=0)throw new Error("missing actual native source allocations");
 let model={...stable,producer:true,source:false};
 for(const record of packet.records){
  const expected=jsonpatch.applyPatch(structuredClone(model),[{op:"replace",path:"/producer",value:record.source?false:true},{op:"replace",path:"/source",value:record.source}],true).newDocument;
  const observed={operations:record.operations,metadata:record.metadata,capacity:record.capacity,values:record.values,producer:record.producer,source:record.source};
  if(JSON.stringify(observed)!==JSON.stringify(expected))throw new Error("accepted native owner/backing or custody differs from independent JSONPatch relation");
  if(model.source&&!record.source)throw new Error("source custody returned before physical retirement");
  if(record.producer===record.source)throw new Error("actual accepted owner duplicated or lost");
  model=expected;records++;
 }
 if(!packet.cancelled&&!model.source)throw new Error("normal native sequence never reached source");
}
if(!records)throw new Error("native packet contains no actual identity transitions");
console.log(JSON.stringify({packets:packets.length,records,nativeCompared:true}));
