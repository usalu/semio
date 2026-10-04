/** 🧫️ Forms semantic schema is independently queryable and declared on its actual snapshot owner. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import fixture from"../../🧫️fixtures/🪶️sqlite/🔣️.json";
import * as snapshot from"../../🟦️.ts";
import{answerError}from"../../../✅️validation/🟦️.ts";
import{prepareResponse}from"../../../📨️response/🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import{binary64Value}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import{NativeDecodeControl}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
function portableValue(value:any):any{
 switch(value.kind){case"null":return{kind:"null"};case"boolean":case"text":return{kind:value.kind,value:value.value};case"unsigned":case"signed":return{kind:value.kind,value:BigInt(value.value)};case"float":return{kind:"float",value:{bits:BigInt("0x"+value.bits)}};case"bytes":return{kind:"bytes",value:new Uint8Array(value.octets)};case"array":return{kind:"array",items:value.items.map(portableValue)};case"object":return{kind:"object",members:value.members.map((member:any)=>({name:member.name,value:portableValue(member.value)}))};default:throw Error("fixture variant");}
}
function specimen():any{
 const q=fixture.question,values=fixture.values.map(portableValue),child=(value:typeof fixture.structure)=>({childId:value.childId,target:{artifactId:value.artifactId,dialect:{artifactKind:value.artifactKind,standard:value.standard,subset:value.subset}}});
 return{schema:fixture.schema,id:fixture.id,version:fixture.version,title:fixture.title,definition:{steps:[{...fixture.step,blocks:[{id:q.id,label:q.label,kind:q.kind,description:q.description,required:q.required,placeholder:q.placeholder,default:{kind:"array",items:values},min:binary64Value({bits:BigInt("0x"+q.minimumBits)}),max:binary64Value({bits:BigInt("0x"+q.maximumBits)}),step:binary64Value({bits:BigInt("0x"+q.incrementBits)}),unit:q.unit,text:q.text,options:q.options,fields:q.fields.map(f=>({key:f.key,...("label"in f?{label:f.label}:{}),value:binary64Value({bits:BigInt("0x"+f.valueBits)})})),schema:q.questionSchema,src:q.src,accept:q.accept,fixtureSlug:q.fixtureSlug,params:portableValue(fixture.values[8]),condition:{kind:"eq",left:{kind:"const",value:portableValue(fixture.values[2])},right:{kind:"or",items:[{kind:"var",name:"answer"},{kind:"truthy",expr:{kind:"and",items:[]}}]}}}]}]},responses:[{id:fixture.response.id,submittedAt:Number(fixture.response.submittedAt),definitionVersion:fixture.response.definitionVersion,answers:values.map((value,i)=>({questionId:"answer"+i,label:"Label"+i,kind:"free",value}))}],structure:child(fixture.structure),results:child(fixture.results)};
}
function codec(){const owner=snapshot as Record<string,any>;if(typeof owner.formsSnapshotToSqliteDatabase!=="function")throw Error("missing actual Forms source provider");return{to:owner.formsSnapshotToSqliteDatabase,from:owner.formsSnapshotFromSqliteDatabase,guard:owner.validateFormsSnapshotSqliteDialect};}
test("Forms complete neutral state survives third-party serialization and editable typed joins",async()=>{
 const {to,from,guard}=codec(),original=specimen(),database:SqliteDatabase=await to(original),bytes=await exportSqliteDatabase(database);const oracle=Database.deserialize(bytes);
 try{expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(oracle.query("SELECT high,low FROM forms_unsigned ORDER BY id LIMIT 1").get()).toEqual({high:4294967295,low:4294967295});expect(oracle.query("SELECT printf('%016llx',value_ieee754_bits) AS bits,value_numeric_class,value FROM forms_float ORDER BY id LIMIT 1").get()).toEqual({bits:"fff0000000000123",value_numeric_class:"nan",value:null});expect(await from(await importSqliteDatabase(oracle.serialize()))).toEqual(original);
  oracle.run("UPDATE forms_document SET title=?",[fixture.editedTitle]);const edited=await from(await importSqliteDatabase(oracle.serialize()));expect(edited.title).toBe(fixture.editedTitle);expect(edited.definition.steps[0].blocks[0].params.members.map((m:any)=>m.name)).toEqual(["same","same",""]);await expect(guard(original,{artifactKind:"s.forms.forms",standard:"1",subset:"*"},database)).resolves.toEqual([]);await expect(guard(original,{artifactKind:"s.forms.forms",standard:"2",subset:"*"},database)).rejects.toThrow();
 }finally{oracle.close();}
});
test("Forms accepts independently renumbered structural identities and retains absent versus present-empty options",async()=>{
 const{to,from,guard}=codec(),original=specimen();original.structure.childId="independent-child";original.results.target.artifactId="independent-target";
 const extra={id:"empty",label:"",kind:"free",options:[],fields:[],default:{kind:"null"}},absent={id:"absent",label:"",kind:"free"};original.definition.steps[0].blocks.push(extra,absent);
 const oracle=Database.deserialize(await exportSqliteDatabase(await to(original)));
 try{
  const names=(oracle.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY rowid").all()as{name:string}[]).map(r=>r.name);
  oracle.exec("BEGIN");
  for(const name of names){const columns=oracle.query('PRAGMA foreign_key_list("'+name+'")').all()as{from:string}[];oracle.exec('UPDATE "'+name+'" SET id=-id*11-7');oracle.exec('UPDATE "'+name+'" SET id=-id');for(const column of columns)if(column.from!=="id")oracle.exec('UPDATE "'+name+'" SET "'+column.from+'"="'+column.from+'"*11+7');}
  oracle.exec("COMMIT");expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const database=await importSqliteDatabase(oracle.serialize()),restored=await from(database);expect(restored).toEqual(original);expect(restored.definition.steps[0].blocks[1].options).toEqual([]);expect(restored.definition.steps[0].blocks[2].options).toBeUndefined();await expect(guard(original,{artifactKind:"s.forms.forms",standard:"1",subset:"*"},database)).resolves.toEqual([]);
  for(const coordinate of fixture.invalidDialects){const at=coordinate.indexOf("@"),slash=coordinate.lastIndexOf("/");await expect(guard(original,{artifactKind:coordinate.slice(0,at),standard:coordinate.slice(at+1,slash),subset:coordinate.slice(slash+1)},database)).rejects.toThrow();}
 }finally{oracle.close();}
});
test("Forms owns deep values and conditions and rejects unreachable topology before publication",async()=>{
 const{to,from}=codec(),original=specimen();let value:any={kind:"null"},condition:any={kind:"const",value:{kind:"boolean",value:true}};for(let i=0;i<fixture.deepLevels;i++){value={kind:"array",items:[value]};condition={kind:"truthy",expr:condition};}original.definition.steps[0].blocks[0].default=value;original.definition.steps[0].blocks[0].condition=condition;
 const database:SqliteDatabase=await to(original),restored=await from(await importSqliteDatabase(await exportSqliteDatabase(database)));let a=restored.definition.steps[0].blocks[0].default,b=restored.definition.steps[0].blocks[0].condition;for(let i=0;i<fixture.deepLevels;i++){a=a.items[0];b=b.expr;}expect(a.kind).toBe("null");expect(b.value).toEqual({kind:"boolean",value:true});const rows=database.tables.reduce((n,t)=>n+t.rows.length,0);await expect(to(original,{maxRows:rows-1})).rejects.toThrow();await expect(from(database,{maxValueBytes:16})).rejects.toThrow();
 const corrupt:SqliteDatabase={tables:database.tables.map(t=>t.name==="forms_condition_truthy"?{...t,rows:t.rows.map((r,i)=>i===0?{...r,values:[r.values[0],r.values[0]]}:r)}:t)};await expect(from(corrupt)).rejects.toThrow();
});
test("Intrinsic construction publishes exact Unicode work and admits ownership before copies",async()=>{
 const parse=(intrinsic as Record<string,any>).parseIntrinsicValueControlled;if(typeof parse!=="function")throw Error("missing controlled intrinsic owner");const value={kind:"text",value:fixture.unicodeText.repeat(fixture.unicodeRepeat)},events:any[]=[];const control=new NativeDecodeControl(1_000_000,event=>{events.push(event);return !(event.total===value.value.length&&event.completed>=256);});await expect(parse(value,control)).rejects.toThrow("canceled");expect(events.some(e=>e.total===value.value.length&&e.completed>=256&&e.completed<e.total)).toBe(true);const small=new NativeDecodeControl(1,()=>true);await expect(parse({kind:"bytes",value:new Uint8Array(65537)},small)).rejects.toThrow("limit");expect(small.ownedBytes).toBe(0);
});
import * as intrinsic from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import * as jsonTransport from "../../../🌱️value/🔣️json/🟦️.ts";

test("Forms JSON transport admits literal values separately from canonical snapshot admission",()=>{
 const transport=jsonTransport as Record<string,any>;expect(typeof transport.parseFormsJsonValue).toBe("function");expect(typeof transport.formsValueJson).toBe("function");
 const plain=fixture.jsonTransportValue,value=transport.parseFormsJsonValue(plain);expect(value.kind).toBe("object");expect(JSON.parse(transport.formsValueJson(value))).toEqual(plain);
 expect(transport.formsValueJson({kind:"unsigned",value:18446744073709551615n})).toBe("18446744073709551615");
 expect(transport.formsValueJson({kind:"bytes",value:new Uint8Array([0,255])})).toBe("[0,255]");
 expect(transport.formsValueJson({kind:"object",members:[{name:"x",value:{kind:"unsigned",value:1n}},{name:"x",value:{kind:"unsigned",value:2n}}]})).toBe('{"x":1,"x":2}');
 expect(transport.parseFormsJsonValue(Number.MAX_SAFE_INTEGER+1).kind).toBe("float");expect(()=>intrinsic.parseIntrinsicValue(true)).toThrow();
});
test("Forms JSON condition transport clones repeated acyclic occurrences while rejecting cycles",()=>{
 const shared={kind:"const",value:fixture.jsonTransportValue};const decoded=jsonTransport.parseFormsJsonCondition({kind:"eq",left:shared,right:shared});expect(decoded.kind).toBe("eq");if(decoded.kind!=="eq")throw Error("condition kind");expect(decoded.left).toEqual(decoded.right);expect(decoded.left).not.toBe(decoded.right);
 const literal={kind:"const"as const,value:{kind:"unsigned"as const,value:1n}};expect(jsonTransport.formsConditionJson({kind:"eq",left:literal,right:literal})).toEqual(JSON.parse('{"kind":"eq","left":{"kind":"const","value":1},"right":{"kind":"const","value":1}}'));
 const cyclic:{kind:"truthy";expr?:unknown}={kind:"truthy"};cyclic.expr=cyclic;expect(()=>jsonTransport.parseFormsJsonCondition(cyclic)).toThrow();
});
test("Forms admits wide question descendants before reading or allocating their identities",async()=>{
 const {to}=codec(),original=specimen();let reads=0;const options=Array.from({length:fixture.wideOptionsCount},(_,i)=>({get value(){reads++;return String(i)},label:""}));original.definition.steps[0].blocks[0].options=options;
 await expect(to(original,{maxRows:5})).rejects.toThrow();expect(reads).toBe(0);const events:any[]=[],controller=new AbortController();await expect(to(original,{signal:controller.signal,onProgress:(event:any)=>{events.push(event);if(event.total===fixture.wideOptionsCount+2&&event.completed>=256)controller.abort();}})).rejects.toThrow(/cancel/);expect(events.some(e=>e.total===fixture.wideOptionsCount+2&&e.completed>=256&&e.completed<e.total)).toBe(true);
});

test("Forms authored schema exposes full typed questions, ordered conditions and intrinsic values",async()=>{
 const sql=await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text();const database=new Database(":memory:");
 try{database.exec(sql);expect(database.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(database.query("SELECT COUNT(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:30});
  database.run("INSERT INTO forms_document VALUES(1,?,?,?,?)",[fixture.schema,fixture.id,fixture.version,fixture.title]);database.run("INSERT INTO forms_step VALUES(1,1,0,?,?,?)",[fixture.step.id,fixture.step.title,fixture.step.description]);
  database.exec("INSERT INTO forms_value VALUES(1,'unsigned'); INSERT INTO forms_unsigned VALUES(1,4294967295,4294967295)");
  database.run("INSERT INTO forms_response VALUES(1,1,0,?,2097151,4294967295,?)",[fixture.response.id,fixture.response.definitionVersion]);database.run("INSERT INTO forms_answer VALUES(1,1,0,?,?,?,1)",[fixture.question.id,fixture.question.label,fixture.question.kind]);
  expect(database.query("SELECT s.title FROM forms_step s JOIN forms_document d ON d.id=s.document_id").get()).toEqual({title:fixture.step.title});expect(database.query("SELECT u.high,u.low FROM forms_answer a JOIN forms_unsigned u ON u.id=a.value_id").get()).toEqual({high:4294967295,low:4294967295});expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);
 }finally{database.close();}
});

test("Forms actual snapshot module owns relational export and import",()=>{
 const owner=snapshot as Record<string,unknown>;expect(typeof owner.formsSnapshotToSqliteDatabase).toBe("function");expect(typeof owner.formsSnapshotFromSqliteDatabase).toBe("function");expect(typeof owner.validateFormsSnapshotSqliteDialect).toBe("function");
});

test("Forms actual consumers accept canonical tagged Boolean and full integer answers",()=>{
 expect(answerError({id:"boolean",label:"",kind:"boolean",required:true},{kind:"boolean",value:true})).toBe(null);expect(answerError({id:"number",label:"",kind:"number"},{kind:"unsigned",value:18446744073709551615n})).toBe(null);const definition={steps:[{id:"step",title:"",blocks:[{id:"answer",label:"",kind:"boolean",condition:{kind:"const"as const,value:{kind:"boolean"as const,value:true}}}]}]};const result=prepareResponse(definition,{answer:{kind:"boolean",value:true}},{id:"response",submittedAt:1,definitionVersion:"1"});expect(result.errors).toEqual([]);expect(result.response!.answers).toHaveLength(1);expect(result.response!.answers[0]!.value).toEqual({kind:"boolean",value:true});
});

test("Framework intrinsic owner preserves unsigned words, NaN identity and duplicate ordered members",()=>{
 const parser=(intrinsic as Record<string,unknown>).parseIntrinsicValue as ((value:unknown)=>unknown)|undefined;expect(typeof parser).toBe("function");
 if(!parser)throw Error("missing canonical intrinsic source owner");
 const value={kind:"object",members:[{name:"same",value:{kind:"unsigned",value:18446744073709551615n}},{name:"same",value:{kind:"float",value:{bits:0xfff0000000000123n}}},{name:"",value:{kind:"bytes",value:new Uint8Array([0,127,255])}}]};
 expect(parser(value)).toEqual(value);const oracle=new Database(":memory:");try{oracle.exec("CREATE TABLE word(high INTEGER,low INTEGER);INSERT INTO word VALUES(4294967295,4294967295)");expect(oracle.query("SELECT high,low FROM word").get()).toEqual({high:4294967295,low:4294967295});}finally{oracle.close();}
 let deep:unknown={kind:"null"};for(let i=0;i<fixture.deepLevels;i++)deep={kind:"array",items:[deep]};let restored=parser(deep) as {kind:string;items?:unknown[]};for(let i=0;i<fixture.deepLevels;i++)restored=restored.items![0] as typeof restored;expect(restored.kind).toBe("null");
 expect(()=>parser({kind:"signed",value:9223372036854775808n})).toThrow();const cycle:{kind:"array";items:unknown[]}={kind:"array",items:[]};cycle.items.push(cycle);expect(()=>parser(cycle)).toThrow();
});

import Ajv from "ajv";
import requestContract from "../../🧫️fixtures/🪶️sqlite/💰️backing/🔬️requests/🔣️.json";
import requestContractSchema from "../../🧫️fixtures/🪶️sqlite/💰️backing/🔬️requests/🧬️schema/🔣️.json";
test("Forms concrete ownership contract retains all literal IEEE words and independently queryable domains",async()=>{
 const validate=new Ajv({strict:true}).compile(requestContractSchema);expect(validate(requestContract),JSON.stringify(validate.errors)).toBe(true);
 expect(fixture.values.filter(value=>value.kind==="float").map(value=>value.bits)).toEqual(requestContract.ieeeWords);
 const{to,from}=codec(),owner=specimen(),database:SqliteDatabase=await to(owner);expect(Object.keys(owner)).toEqual(requestContract.fieldOrder);expect(database.tables).toHaveLength(requestContract.tableCount);
 for(const phase of requestContract.phases){await expect(phase==="projectSnapshot"?to(owner,{maxAllocationBytes:requestContract.zeroOwnershipBytes}):from(database,{maxAllocationBytes:requestContract.zeroOwnershipBytes})).rejects.toHaveProperty("kind",requestContract.refusalKind);}
 const oracle=Database.deserialize(await exportSqliteDatabase(database));try{
  const words=(oracle.query("SELECT printf('%016llx',value_ieee754_bits) AS bits FROM forms_float ORDER BY id").all()as{bits:string}[]).map(value=>value.bits);expect([...new Set(words)]).toEqual(requestContract.ieeeWords);
  expect((oracle.query("SELECT DISTINCT kind FROM forms_value ORDER BY kind").all()as{kind:string}[]).map(value=>value.kind)).toEqual([...requestContract.intrinsicKinds].sort());
  expect((oracle.query("SELECT DISTINCT kind FROM forms_condition ORDER BY kind").all()as{kind:string}[]).map(value=>value.kind)).toEqual([...requestContract.conditionKinds].sort());
  expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  for(const word of requestContract.ieeeWords){const buffer=Buffer.alloc(8);buffer.writeBigUInt64BE(BigInt("0x"+word));const view=new DataView(buffer.buffer,buffer.byteOffset,8);expect(view.getBigUint64(0,false)).toBe(BigInt("0x"+word));const value=view.getFloat64(0,false);if(!Number.isNaN(value)){const out=Buffer.alloc(8);out.writeDoubleBE(value);expect(out.toString("hex")).toBe(word);}}
  expect(await from(await importSqliteDatabase(oracle.serialize()))).toEqual(owner);
 }finally{oracle.close();}
});

import preflightContract from "../../🧫️fixtures/🪶️sqlite/📏️preflight/🔣️.json";
import whitespaceContract from "../../🧫️fixtures/🪶️sqlite/📏️preflight/🔤️whitespace/🔣️.json";
import whitespaceContractSchema from "../../🧫️fixtures/🪶️sqlite/📏️preflight/🔤️whitespace/🧬️schema/🔣️.json";
test("Forms borrowed whitespace kind completes bounded UTF-8 census before semantic refusal",()=>{
 const validate=new Ajv({strict:true}).compile(whitespaceContractSchema);expect(validate(whitespaceContract),JSON.stringify(validate.errors)).toBe(true);
 const kind=whitespaceContract.kindUnit.repeat(whitespaceContract.repeat),bytes=Buffer.byteLength(kind,"utf8");expect(bytes).toBe(whitespaceContract.utf8Bytes);expect(new TextEncoder().encode(kind).length).toBe(bytes);expect(Array.from(kind)).toHaveLength(whitespaceContract.unicodeScalars);expect(kind.trim()).toBe("");expect((kind+"x").trim()).toBe("x");
 const oracle=new Database(":memory:");try{oracle.exec("CREATE TABLE borrowed_kind(kind TEXT NOT NULL)");oracle.run("INSERT INTO borrowed_kind VALUES(?)",[kind]);expect(oracle.query("SELECT length(CAST(kind AS BLOB)) AS bytes,length(kind) AS scalars FROM borrowed_kind").get()).toEqual({bytes,scalars:whitespaceContract.unicodeScalars});expect(oracle.query("SELECT kind FROM borrowed_kind").get()).toEqual({kind});}finally{oracle.close();}
 expect(whitespaceContract.cancelAfterBytes).toBeGreaterThan(0);expect(whitespaceContract.cancelAfterBytes).toBeLessThan(bytes);expect(whitespaceContract.ownedScanBytes).toBe(0);expect(whitespaceContract.encodings).toEqual(["binary","text"]);
});
import preflightContractSchema from "../../🧫️fixtures/🪶️sqlite/📏️preflight/🧬️schema/🔣️.json";
test("Forms borrowed preflight distinguishes prospective wire ceilings from paid scratch ownership",async()=>{
 const validate=new Ajv({strict:true}).compile(preflightContractSchema);expect(validate(preflightContract),JSON.stringify(validate.errors)).toBe(true);
 const contract=preflightContract as {literalUtf8Bytes:number;copiedOwnerFields:boolean;forecastCeilings:string[];ownedAllowance:string;scratchOwnership:string;encodings:string[];tableCount:number;censusAuthority:string;literalGrowthRequestDelta:number};
 const literal=fixture.unicodeText.repeat(fixture.unicodeRepeat),bytes=Buffer.byteLength(literal,"utf8");expect(bytes).toBe(contract.literalUtf8Bytes);expect(new TextEncoder().encode(literal).byteLength).toBe(bytes);
 const oracle=new Database(":memory:");try{oracle.exec("CREATE TABLE borrowed_literal(title TEXT NOT NULL)");oracle.run("INSERT INTO borrowed_literal VALUES(?)",[literal]);expect(oracle.query("SELECT length(CAST(title AS BLOB)) AS bytes FROM borrowed_literal").get()).toEqual({bytes});expect(oracle.query("SELECT title FROM borrowed_literal").get()).toEqual({title:literal});}finally{oracle.close();}
 const owner=specimen();owner.title=literal;const database:SqliteDatabase=await codec().to(owner);expect(database.tables).toHaveLength(contract.tableCount);const physical=Database.deserialize(await exportSqliteDatabase(database));try{expect(physical.query("SELECT length(CAST(title AS BLOB)) AS bytes FROM forms_document").get()).toEqual({bytes});const union=database.tables.map(table=>'SELECT COUNT(*) AS n FROM "'+table.name+'"').join(" UNION ALL ");expect(physical.query("SELECT SUM(n) AS rows FROM ("+union+")").get()).toEqual({rows:database.tables.reduce((count,table)=>count+table.rows.length,0)});expect(physical.query("PRAGMA foreign_key_check").all()).toEqual([]);}finally{physical.close();}expect(contract.censusAuthority).toBe("allStoredRows");expect(contract.literalGrowthRequestDelta).toBe(0);
 expect(contract.copiedOwnerFields).toBe(false);expect(contract.forecastCeilings).not.toContain(contract.ownedAllowance);expect(contract.scratchOwnership).toBe("paidFrontiersOnly");expect(contract.encodings).toEqual(["binary","text"]);
});
