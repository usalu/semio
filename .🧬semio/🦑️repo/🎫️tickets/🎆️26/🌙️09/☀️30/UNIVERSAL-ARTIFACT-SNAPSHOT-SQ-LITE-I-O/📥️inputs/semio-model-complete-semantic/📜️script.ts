import {Database} from 'bun:sqlite';
import assert from 'node:assert/strict';
import Ajv2020 from 'ajv/dist/2020';
import {parseBinary64} from '../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts';
const root='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/';
const sql=await Bun.file(root+'🗄️.sql').text(),metadata=await Bun.file('🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🗄️.sql').text();
const word=(one=false)=>({bits:one?'3ff0000000000000':'0000000000000000'});
const placement=()=>({translation:{x:word(),y:word(),z:word()},rotation:{x:word(),y:word(),z:word(),w:word(true)},scale:{x:word(true),y:word(true),z:word(true)}});
const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,1.2345678901234567);const numberWord=view.getBigUint64(0).toString(16).padStart(16,'0');
const spatialKinds=['site','building','storey','space'],classes=['wall','slab','column','beam','door','window','roof','stair','furniture','other'],relations=['aggregates','containedIn','connectsTo','fillsVoid','voidsElement','other'];
const full={schema:'stdio.semio.model',spatial:spatialKinds.map((kind,index)=>({id:`spatial-${index}`,kind,name:`Spatial ${index}`,parentId:index===0?null:`spatial-${index-1}`,placement:placement()})),elements:classes.map((kind,index)=>({id:`element-${index}`,class:kind==='other'?{kind,name:'IfcCustom世界'}:{kind},placement:placement(),geometry:index%3===0?{kind:'none'}:index%3===1?{kind:'brep',brepId:'external-geometry'}:{kind:'mesh',meshId:'external-geometry'},spatialId:index%2===0?'spatial-3':null,psets:[{name:'Pset typed',properties:[{key:'same',value:{kind:'text',value:'Grüße\u0000世界'}},{key:'same',value:{kind:'number',value:{bits:numberWord}}},{key:'boolean',value:{kind:'boolean',value:index%2===0}}]}]})),relations:relations.map((kind,index)=>({id:`relation-${index}`,kind:kind==='other'?{kind,label:'custom relation'}:{kind},from:'spatial-0',to:`element-${index}`}))};
const empty={schema:full.schema,spatial:[],elements:[],relations:[]};
function census(source:typeof full|typeof empty,encoding?:string){
 const db=new Database(':memory:');db.exec('PRAGMA foreign_keys=ON');db.exec(sql);if(encoding)db.exec(metadata);
 const insert=(table:string,values:unknown[])=>db.query('INSERT INTO '+table+' VALUES ('+values.map(()=>'?').join(',')+')').run(...values as any[]);
 const place=(id:number)=>insert('semio_model_placement',[id,0,0,0,0,0,0,1,1,1,1,...[false,false,false,false,false,false,true,true,true,true].flatMap(one=>[one?0x3ff0000000000000n:0n,'finite'])]);
 insert('semio_model_document',[1,source.schema]);
 if(source.spatial.length){
  source.spatial.forEach((node,index)=>{const id=index+1;insert('semio_model_entity',[id,1,'spatial',index,node.id]);insert('semio_model_spatial',[id,node.kind,node.name,index===0?null:index]);place(id);});
  source.elements.forEach((element,index)=>{const id=index+5;insert('semio_model_entity',[id,1,'element',index,element.id]);insert('semio_model_element',[id,element.class.kind,element.class.kind==='other'?'IfcCustom世界':null,index%2===0?4:null]);place(id);insert('semio_model_geometry_reference',[id,index%3===0?'none':index%3===1?'brep':'mesh',index%3===0?null:'external-geometry']);insert('semio_model_property_set',[index+1,id,0,'Pset typed']);insert('semio_model_property',[index*3+1,index+1,0,'same','text','Grüße\u0000世界',null,null,null,null]);insert('semio_model_property',[index*3+2,index+1,1,'same','number',null,1.2345678901234567,null,BigInt('0x'+numberWord),'finite']);insert('semio_model_property',[index*3+3,index+1,2,'boolean','boolean',null,null,index%2===0?1:0,null,null]);});
  const sqlKinds=['aggregates','contained_in','connects_to','fills_void','voids_element','other'];source.relations.forEach((relation,index)=>insert('semio_model_relation',[index+1,1,index,relation.id,sqlKinds[index],index===5?'custom relation':null,1,index+5]));
  assert.deepEqual(db.query('SELECT source.native_id source,target.native_id target,semio_model_relation.kind kind FROM semio_model_relation JOIN semio_model_entity source ON source.id=source_entity_id JOIN semio_model_entity target ON target.id=target_entity_id ORDER BY semio_model_relation.ordinal').all(),sqlKinds.map((kind,index)=>({source:'spatial-0',target:`element-${index}`,kind})));
  assert.equal((db.query('SELECT hex(text_value) bytes FROM semio_model_property WHERE kind=\'text\' LIMIT 1').get() as any).bytes,Buffer.from('Grüße\u0000世界').toString('hex').toUpperCase());
 }
 if(encoding)insert('semio_snapshot',[1,'s.stdio.semio','v1','model',1,encoding]);
 assert.deepEqual(db.query('PRAGMA integrity_check').get(),{integrity_check:'ok'});assert.deepEqual(db.query('PRAGMA foreign_key_check').all(),[]);
 let rows=0,valueBytes=0,schemaBytes=0;const tableWidths:Record<string,number>={};
 for(const row of db.query("SELECT name,sql FROM sqlite_master WHERE type='table' ORDER BY name").all() as {name:string,sql:string}[]){schemaBytes+=Buffer.byteLength(row.name)+Buffer.byteLength(row.sql);tableWidths[row.name]=db.query('PRAGMA table_info('+row.name+')').all().length;const cells=db.query('SELECT * FROM '+row.name).values();rows+=cells.length;for(const values of cells)for(const value of values)valueBytes+=value===null?0:typeof value==='string'?Buffer.byteLength(value):typeof value==='number'||typeof value==='bigint'?8:(value as Uint8Array).length;}
 db.close();return {rows,valueBytes,schemaBytes,tableWidths};
}
const f=census(full),e=census(empty),fb=census(full,'binary'),ft=census(full,'text'),eb=census(empty,'binary'),et=census(empty,'text');assert.equal(Object.keys(f.tableWidths).length,9);assert.equal(Math.max(...Object.values(f.tableWidths)),31);assert.equal(fb.schemaBytes-f.schemaBytes,293);assert.equal(fb.valueBytes-f.valueBytes,42);assert.equal(ft.valueBytes-f.valueBytes,40);

if(process.argv.includes('late-owned-reconstruction')){
 const schemaText={character:'x',count:131073};const late=census({...full,schema:schemaText.character.repeat(schemaText.count)});
 const contract={sourceCase:'full',schemaText,schemaUtf8Bytes:131073,rows:late.rows,valueBytes:late.valueBytes,schemaBytes:late.schemaBytes,tableWidths:late.tableWidths,maxTables:9,maxColumns:31,cancellation:{phase:'ReconstructSnapshot',completed:65536,total:131073},ownership:{success:'all concrete requests admitted',exact:'same concrete allowance',shortfall:1,refusalCleanup:'all requested backing released including diagnostic and retirement scratch'}};
 assert.equal(late.rows,f.rows);assert.equal(late.valueBytes,f.valueBytes-Buffer.byteLength(full.schema)+schemaText.count);
 const contractSchema={$schema:'https://json-schema.org/draft/2020-12/schema',type:'object',properties:Object.fromEntries(Object.entries(contract).map(([key,value])=>[key,{const:value}])),required:Object.keys(contract),additionalProperties:false};
 assert.equal(new Ajv2020({strict:true}).compile(contractSchema)(contract),true);
 await Bun.write(new URL('handcrafted-late-owned-reconstruction-contract.json',import.meta.url),JSON.stringify(contract,null,2)+'\n');
 await Bun.write(new URL('handcrafted-late-owned-reconstruction-contract-schema.json',import.meta.url),JSON.stringify(contractSchema,null,2)+'\n');
 console.log('[DEBUG] independent Model long-schema nonempty SQLite census '+JSON.stringify(late));process.exit(0);
}

const hydrate='const hydrate=(value:unknown):unknown=>Array.isArray(value)?value.map(hydrate):value!==null&&typeof value==="object"?("bits"in value?{bits:String((value as {bits:string}).bits).length===8?Number.parseInt(String((value as {bits:string}).bits),16):BigInt("0x"+String((value as {bits:string}).bits))}:Object.fromEntries(Object.entries(value).map(([key,item])=>[key,hydrate(item)]))):value;Object.assign(source,hydrate(source))';
const spec=[{name:'model',cap:'Model',tables:9,maxColumns:31,contract:{schemaBytes:f.schemaBytes,tableWidths:f.tableWidths,cases:[{id:'full',sourceSnapshot:full,rows:f.rows,valueBytes:f.valueBytes},{id:'metadataRetainingEmpty',sourceSnapshot:empty,rows:e.rows,valueBytes:e.valueBytes}],publicIo:{schemaBytes:fb.schemaBytes,tableWidths:fb.tableWidths,cases:[{id:'full',rows:fb.rows,valueBytes:{binary:fb.valueBytes,text:ft.valueBytes}},{id:'metadataRetainingEmpty',rows:eb.rows,valueBytes:{binary:eb.valueBytes,text:et.valueBytes}}]}},rustBody:'let mut source=fixture();if case["id"]=="metadataRetainingEmpty"{source.spatial.clear();source.elements.clear();source.relations.clear();}source',hydrate}];
let words=0;const admit=(value:unknown):void=>{if(Array.isArray(value))value.forEach(admit);else if(value&&typeof value==='object'){if('bits'in value){parseBinary64(value);words++;}else Object.values(value).forEach(admit);}};
const transpiler=new Bun.Transpiler({loader:'ts'});for(const item of spec[0].contract.cases){const source=structuredClone(item.sourceSnapshot);new Function('source',transpiler.transformSync(hydrate))(source);admit(source);assert.equal(source.schema,'stdio.semio.model');}
const schema={$schema:'https://json-schema.org/draft/2020-12/schema',type:'object',properties:Object.fromEntries(Object.entries(spec[0].contract).map(([key,value])=>[key,{const:value}])),required:Object.keys(spec[0].contract),additionalProperties:false};assert.equal(new Ajv2020({strict:true}).compile(schema)(spec[0].contract),true);
await Bun.write(new URL('handcrafted-native-matched-demand-specifications.json',import.meta.url),JSON.stringify(spec,null,2)+'\n');await Bun.write(new URL('handcrafted-native-matched-contract-schema.json',import.meta.url),JSON.stringify(schema,null,2)+'\n');console.log(JSON.stringify({f,e,publicIo:spec[0].contract.publicIo,numberWord,actualBinary64ParserWords:words}));
