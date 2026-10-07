import {Database} from 'bun:sqlite';
import assert from 'node:assert/strict';
const root='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/';
const sql=await Bun.file(root+'🗄️.sql').text();
const metadata=await Bun.file('🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🗄️.sql').text();
const bits=(one=false)=>({bits:one?'3ff0000000000000':'0000000000000000'});
const reference=(id:string,subset:string)=>({artifactId:id,dialect:{artifactKind:'s.stdio.semio',standard:'v1',subset}});
const child=(id:string,subset:string)=>({childId:id,target:reference(id,subset)});
const full={schema:'stdio.semio.kit',types:[{id:'chair',name:'Chair 世界',category:'furniture'}],designs:[{id:'layout',name:'Design',pieces:['left','right'].map((id,index)=>({id,typeId:'chair',transform:{translation:{x:bits(index===1),y:bits(),z:bits()},rotation:{x:bits(),y:bits(),z:bits(),w:bits(true)},scale:{x:bits(true),y:bits(true),z:bits(true)}}})),connections:[{id:'joint',connectingPieceId:'left',connectingPort:'a',connectedPieceId:'right',connectedPort:'b'}]}],objects:[child('object-one','object'),child('object-two','object')],models:[child('model-one','model')],properties:child('value-one','value'),representations:[{target:reference('representation-0','mesh'),role:'chair',pin:{kind:'head'}},{target:reference('representation-1','mesh'),role:'chair',pin:{kind:'checkpoint',id:'checkpoint-世界'}},{target:reference('representation-2','mesh'),role:'chair',pin:{kind:'snapshot',blob:{hash:'content-address',size:'18446744073709551615',mediaType:'application/octet-stream'}}}]};
const empty={schema:full.schema,types:[],designs:[],objects:[],models:[],representations:[]};
const widths:Record<string,number>={};
function census(source:typeof full|typeof empty,encoding?:string){
 const db=new Database(':memory:');db.exec('PRAGMA foreign_keys=ON');db.exec(sql);if(encoding)db.exec(metadata);
 const insert=(table:string,values:unknown[])=>db.query('INSERT INTO '+table+' VALUES ('+values.map(()=>'?').join(',')+')').run(...values as any[]);
 insert('semio_kit_document',[1,source.schema]);
 if(source.types.length){
  insert('semio_kit_type',[1,1,0,'chair','Chair 世界','furniture']);insert('semio_kit_design',[1,1,0,'layout','Design']);
  source.designs[0].pieces.forEach((piece,index)=>{const words=Object.values(piece.transform).flatMap(Object.values) as {bits:string}[];const numeric=words.map(word=>word.bits==='3ff0000000000000'?1:0);const sidecars=words.flatMap(word=>[BigInt('0x'+word.bits),'finite']);insert('semio_kit_piece',[index+1,1,index,piece.id,1,...numeric,...sidecars]);});
  insert('semio_kit_connection',[1,1,0,'joint',1,'a',2,'b']);
  const refs=[...source.objects,...source.models,source.properties!].map(value=>value.target).concat(source.representations.map(value=>value.target));
  refs.forEach((r,index)=>insert('semio_kit_reference',[index+1,r.artifactId,r.dialect.artifactKind,r.dialect.standard,r.dialect.subset]));
  source.objects.forEach((v,index)=>insert('semio_kit_object_child',[index+1,1,index,v.childId,index+1]));insert('semio_kit_model_child',[1,1,0,'model-one',3]);insert('semio_kit_value_child',[1,1,'value-one',4]);
  insert('semio_kit_blob',[1,'content-address','18446744073709551615','application/octet-stream']);
  insert('semio_kit_representation',[1,1,0,1,5,'head',null,null]);insert('semio_kit_representation',[2,1,1,1,6,'checkpoint','checkpoint-世界',null]);insert('semio_kit_representation',[3,1,2,1,7,'snapshot',null,1]);
  assert.equal((db.query('SELECT size FROM semio_kit_blob').get() as any).size,'18446744073709551615');
  assert.deepEqual(db.query('SELECT a.native_id a,b.native_id b FROM semio_kit_connection JOIN semio_kit_piece a ON a.id=connecting_piece_id JOIN semio_kit_piece b ON b.id=connected_piece_id').get(),{a:'left',b:'right'});
 }
 if(encoding)insert('semio_snapshot',[1,'s.stdio.semio','v1','kit',1,encoding]);
 assert.deepEqual(db.query('PRAGMA integrity_check').get(),{integrity_check:'ok'});assert.deepEqual(db.query('PRAGMA foreign_key_check').all(),[]);
 let rows=0,valueBytes=0;const tableWidths:Record<string,number>={};
 for(const {name} of db.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").all() as {name:string}[]){tableWidths[name]=db.query('PRAGMA table_info('+name+')').all().length;const cells=db.query('SELECT * FROM '+name).values();rows+=cells.length;for(const row of cells)for(const value of row)valueBytes+=value===null?0:typeof value==='string'?Buffer.byteLength(value):typeof value==='number'||typeof value==='bigint'?8:(value as Uint8Array).length;}
 const schemaBytes=(db.query("SELECT name,sql FROM sqlite_master WHERE type='table'").all() as {name:string,sql:string}[]).reduce((total,row)=>total+Buffer.byteLength(row.name)+Buffer.byteLength(row.sql),0);db.close();return {rows,valueBytes,tableWidths,schemaBytes};
}
const cases=[{id:'full',sourceSnapshot:full,...census(full)},{id:'metadataRetainingEmpty',sourceSnapshot:empty,...census(empty)}];
Object.assign(widths,cases[0].tableWidths);for(const value of cases){delete (value as any).tableWidths;delete (value as any).schemaBytes;}
const pubFullB=census(full,'binary'),pubFullT=census(full,'text'),pubEmptyB=census(empty,'binary'),pubEmptyT=census(empty,'text');
assert.equal(Object.keys(widths).length,11);assert.equal(Math.max(...Object.values(widths)),35);assert.equal(pubFullB.schemaBytes-census(full).schemaBytes,293);
const hydrate='const hydrate=(value:unknown):unknown=>Array.isArray(value)?value.map(hydrate):value!==null&&typeof value==="object"?("bits"in value?{bits:String((value as {bits:string}).bits).length===8?Number.parseInt(String((value as {bits:string}).bits),16):BigInt("0x"+String((value as {bits:string}).bits))}:Object.fromEntries(Object.entries(value).map(([key,item])=>[key,hydrate(item)]))):value;Object.assign(source,hydrate(source));for(const link of source.representations){if(link.pin.kind==="snapshot")link.pin.blob.size=BigInt(link.pin.blob.size)}';
const spec=[{name:'kit',cap:'Kit',tables:11,maxColumns:35,contract:{schemaBytes:census(full).schemaBytes,tableWidths:widths,cases,publicIo:{schemaBytes:pubFullB.schemaBytes,tableWidths:pubFullB.tableWidths,cases:[{id:'full',rows:pubFullB.rows,valueBytes:{binary:pubFullB.valueBytes,text:pubFullT.valueBytes}},{id:'metadataRetainingEmpty',rows:pubEmptyB.rows,valueBytes:{binary:pubEmptyB.valueBytes,text:pubEmptyT.valueBytes}}]}},rustBody:'let mut source=fixture();if case["id"]=="metadataRetainingEmpty"{source.types.clear();source.designs.clear();source.objects.clear();source.models.clear();source.properties=None;source.representations.clear();}source',hydrate}];
await Bun.write(new URL('handcrafted-native-matched-demand-specifications.json',import.meta.url),JSON.stringify(spec,null,2)+'\n');console.log(JSON.stringify({schemaBytes:Buffer.byteLength(sql),cases:cases.map(({id,rows,valueBytes})=>({id,rows,valueBytes})),publicIo:spec[0].contract.publicIo}));
