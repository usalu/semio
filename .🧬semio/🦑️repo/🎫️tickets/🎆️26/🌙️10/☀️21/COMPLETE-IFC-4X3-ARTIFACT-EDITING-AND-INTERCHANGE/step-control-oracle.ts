/** 🔬️ Retain bounded-codec stress models for independent IfcOpenShell validation. */
import {mkdir} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import assert from "node:assert/strict";
const ticket=import.meta.dir,root=resolve(ticket,"../../../../../../.."),base=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any"),output=resolve(ticket,"🗑️generated/stepctl");
const {parseIfc4x3Step,writeIfc4x3Step,writeIfc4x3StepChunks}=await import(pathToFileURL(resolve(base,"🚪️io/step.ts")).href);
const source=(data:string)=>`ISO-10303-21;HEADER;FILE_DESCRIPTION(('ViewDefinition [ReferenceView]'),'2;1');FILE_NAME('stepctl.ifc','2026-10-21T12:00:00',('Author'),('Organisation'),'Writer','Application','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;${data}ENDSEC;END-ISO-10303-21;`;
const models=[
 {name:"unicode",data:"#1=IFCPROPERTYSINGLEVALUE('Unicode',$,IFCTEXT('\\X2\\03B100E4\\X0\\\\X4\\0001F600\\X0\\''\\\\'),$);"},
 {name:"long-unicode",data:`/*${" ".repeat(16384)}*/#1=IFCPROPERTYSINGLEVALUE('LongUnicode',$,IFCTEXT('\\X2\\${"00E4".repeat(16384)}\\X0\\'),$);`},
 {name:"long-binary",data:`#1=IFCPROPERTYSINGLEVALUE('LongBinary',$,IFCBINARY("0${"AB".repeat(16384)}"),$);`},
 {name:"long-list",data:`#1=IFCPROPERTYLISTVALUE('LongList',$,(${Array.from({length:16384},(_,index)=>`IFCINTEGER(${index})`).join(",")}),$);`},
 {name:"exact-integer",data:"#1=IFCPROPERTYSINGLEVALUE('ExactInteger',$,IFCINTEGER(9007199254740993),$);"},
];
const neutral=await Bun.file(resolve(base,"🚪️io/🧪️tests/control-cases.json")).json();
for(const [index,row] of neutral.lexical.entries()){const carrier=index===0?"IFCPROPERTYSINGLE\r\nVALUE":"IFCPROPERTYSINGLEVALUE",domain=row.binary?"IFCBINARY":"IFCTEXT",data=row.data.replace(/IFC(?:\r\n)?LABEL\(/,`${carrier}('Lexical${index}',$,${domain}(`).replace(");","),$);");models.push({name:`lexical-${index}`,data});}
await mkdir(output,{recursive:true});let chunks=0;
for(const model of models){const text=source(model.data);await Bun.write(resolve(output,model.name+".source.ifc"),text);const graph=await parseIfc4x3Step(text,{batchSize:4096}),encoded=await writeIfc4x3Step(graph,{batchSize:4096});let streamed="";for await(const chunk of writeIfc4x3StepChunks(graph,{batchSize:4096})){assert.ok(chunk.length<=4096);streamed+=chunk;chunks++;}assert.equal(streamed,encoded);await Bun.write(resolve(output,model.name+".owned.ifc"),encoded);assert.deepEqual(await parseIfc4x3Step(encoded,{batchSize:4096}),graph);}
await Bun.write(resolve(output,"cases.json"),JSON.stringify(models.map(({name})=>({name})),null,2));console.log(JSON.stringify({models:models.length,chunks}));
