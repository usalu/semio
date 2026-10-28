/** 🧪 Actual GLB endpoint parity baseline, never a source copy. */
import {readFile,writeFile,access} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const base='file:///C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/';
const {readIfcGltfGlb}=await import(base+'gltf.ts');
const original=await readFile(new URL('🧪️tests/box.glb',base)),v=new DataView(original.buffer,original.byteOffset,original.byteLength),jl=v.getUint32(12,true),json=new TextDecoder().decode(original.subarray(20,20+jl)),bin=original.subarray(28+jl);
const cases=[];const add=(id,text,mutate)=>cases.push({id,text,mutate});
add('original',json);
for(const [id,extra]of [['duplicate','{"x":1,"y":2,"x":3}'],['escaped-duplicate','{"x":1,"\\u0078":2}'],['integer-keys','{"10":1,"2":2,"01":3,"0":4,"4294967294":5,"4294967295":6}'],['dangerous','{"__proto__":{"polluted":true},"constructor":1,"prototype":2,"toJSON":"data"}'],['overflow-replaced','{"x":1e400,"x":1}'],['finite-replaced-overflow','{"x":1,"x":1e400}'],['minuszero','-0'],['underflow','-1e-999'],['max','1.7976931348623157e308'],['subnormal','5e-324'],['lone-high','"\\ud800"'],['lone-low','"\\udfff"'],['pair','"\\ud83d\\ude00"'],['namespace','{"attributes":[],"targets":[],"bits":"ordinary","extras":{}}']])add('extras-'+id,json.replace('{','{"extras":'+extra+','));
for(const [id,value]of [['exponent','1e0'],['decimal','1.0'],['minuszero','-0'],['safe-max','9007199254740991'],['above-safe','9007199254740992'],['fraction','1.5'],['negative','-1'],['overflow','1e400']])add('scene-'+id,json.replace(/"scene"\s*:\s*\d+/, '"scene":'+value));
for(const [id,prefix,suffix]of [['bom','\ufeff',''],['two-bom','\ufeff\ufeff',''],['whitespace',' \t\r\n','\n\r\t '],['nul','','\0'],['trailing-token','','false']])add('text-'+id,prefix+json+suffix);
add('duplicate-asset-valid-last','{"asset":false,'+json.slice(1));add('unknown-root',json.replace('{','{"unknown":0,'));add('defaults','{"asset":{"version":"2.0"},"buffers":[{"byteLength":'+bin.length+'}]}');
add('utf8-invalid',json,bytes=>{bytes[20]=0xff;});add('utf8-truncated',json,bytes=>{const d=new DataView(bytes.buffer);bytes[20+d.getUint32(12,true)-1]=0xc3;});
const family=(id,change)=>{const doc=JSON.parse(json);change(doc);add('family-'+id,JSON.stringify(doc));};
family('material-defaults',d=>d.materials=[{}]);
family('material-full',d=>d.materials=[{name:'ä😀',pbrMetallicRoughness:{baseColorFactor:[.1,-0,1,1],baseColorTexture:{index:0,texCoord:1},metallicFactor:.2,roughnessFactor:.8,metallicRoughnessTexture:{index:0}},normalTexture:{index:0,scale:-.5},occlusionTexture:{index:0,strength:.6},emissiveTexture:{index:0},emissiveFactor:[.1,.2,.3],alphaMode:'MASK',alphaCutoff:.4,doubleSided:true,extensions:{X:{n:1}},extras:['x',null]}]);
for(const mode of ['OPAQUE','BLEND','BAD'])family('alpha-'+mode,d=>d.materials=[{alphaMode:mode}]);
family('texture-defaults',d=>{d.textures=[{}];d.samplers=[{}];});
family('texture-full',d=>{d.textures=[{sampler:0,source:0,name:'T',extras:false}];d.samplers=[{magFilter:9729,minFilter:9987,wrapS:33071,wrapT:33648,name:'S'}];});
family('images-uri',d=>d.images=[{uri:'image.png',name:'i',mimeType:'image/png'}]);
family('images-buffer',d=>d.images=[{bufferView:0,mimeType:'image/png',extensions:{X:[]}}]);
family('skin-defaults',d=>d.skins=[{}]);
family('skin-full',d=>d.skins=[{inverseBindMatrices:0,skeleton:0,joints:[0,1],name:'skin'}]);
family('animation-defaults',d=>d.animations=[{}]);
for(const path of ['translation','rotation','scale','weights','bad'])family('animation-'+path,d=>d.animations=[{channels:[{sampler:0,target:{node:0,path}}],samplers:[{input:0,output:1}]}]);
for(const interpolation of ['LINEAR','STEP','CUBICSPLINE','BAD'])family('interpolation-'+interpolation,d=>d.animations=[{channels:[],samplers:[{input:0,output:0,interpolation}]}]);
family('camera-perspective-default',d=>d.cameras=[{type:'perspective',perspective:{yfov:1,znear:.01}}]);
family('camera-perspective-full',d=>d.cameras=[{type:'perspective',name:'camera',perspective:{aspectRatio:1.5,yfov:.9,znear:.01,zfar:1000,extras:0}}]);
family('camera-ortho',d=>d.cameras=[{type:'orthographic',orthographic:{xmag:1,ymag:2,znear:-1,zfar:10}}]);
family('camera-invalid-tag',d=>d.cameras=[{type:'BAD',perspective:{yfov:1,znear:.01}}]);
family('sparse-defaults',d=>d.accessors[0].sparse={count:1,indices:{bufferView:0,componentType:5123},values:{bufferView:0}});
family('sparse-full',d=>d.accessors[0].sparse={count:1,indices:{bufferView:0,componentType:5121,byteOffset:4},values:{bufferView:0,byteOffset:8}});
family('node-full',d=>d.nodes=[{children:[1],mesh:0,camera:0,skin:0,matrix:[1,0,0,0,0,1,0,0,0,0,1,0,.1,.2,.3,1],translation:[1,2,3],rotation:[0,0,0,1],scale:[-1,2,3],weights:[.2,.3],name:'node'}]);
family('mesh-targets',d=>{d.meshes[0].weights=[.5];d.meshes[0].primitives[0].targets=[{POSITION:0,NORMAL:1}];d.meshes[0].primitives[0].mode=4;d.meshes[0].primitives[0].material=0;});
family('scene-asset',d=>{d.scenes=[{nodes:[0],name:'scene',extras:{x:true}}];d.asset={version:'2.0',generator:'audit',copyright:'c',minVersion:'2.0',extensions:{X:{}},extras:[]};d.extensionsUsed=['X'];d.extensionsRequired=['X'];});
family('buffer-view-full',d=>d.bufferViews[0]={...d.bufferViews[0],byteOffset:0,byteStride:24,target:34962,name:'view',extras:null});
family('accessor-defaults',d=>d.accessors=[{componentType:5126,count:0,type:'SCALAR'}]);
for(const type of ['SCALAR','VEC2','VEC3','VEC4','MAT2','MAT3','MAT4','BAD'])family('accessor-'+type,d=>d.accessors=[{componentType:5126,count:1,type,min:[-1],max:[1]}]);
family('explicit-null-rejected',d=>d.nodes[0].children=null);
family('unknown-nested-rejected',d=>d.materials=[{pbrMetallicRoughness:{unknown:1}}]);
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
function normalize(value){if(typeof value==='bigint')return {$bigint:String(value)};if(typeof value==='number'){const d=new DataView(new ArrayBuffer(8));d.setFloat64(0,value,false);return {$numberBits:d.getBigUint64(0,false).toString(16).padStart(16,'0')};}if(Array.isArray(value))return value.map(normalize);if(value&&typeof value==='object')return {$object:Object.keys(value).map(key=>[key,normalize(value[key])])};return value;}
function glb(text){const raw=new TextEncoder().encode(text),length=Math.ceil(raw.length/4)*4,b=new Uint8Array(28+length+bin.length),d=new DataView(b.buffer);d.setUint32(0,0x46546c67,true);d.setUint32(4,2,true);d.setUint32(8,b.length,true);d.setUint32(12,length,true);d.setUint32(16,0x4e4f534a,true);b.fill(32,20,20+length);b.set(raw,20);d.setUint32(20+length,bin.length,true);d.setUint32(24+length,0x004e4942,true);b.set(bin,28+length);return b;}
const rows=[];for(const c of cases){const bytes=glb(c.text);c.mutate?.(bytes);try{const snapshot=normalize(await readIfcGltfGlb(bytes,{batchSize:4096}));const serialized=JSON.stringify(snapshot);rows.push({id:c.id,inputHash:hash(bytes),accepted:true,snapshot,digest:hash(serialized)});}catch(e){rows.push({id:c.id,inputHash:hash(bytes),accepted:false,error:{name:e.name,code:e.code??null,at:e.at??null,message:e.message}});}}
const path=new URL('./🗑️generated/glb-reader-baseline-v3.json',import.meta.url);if(process.argv.includes('--compare')){const old=JSON.parse(await readFile(path,'utf8'));const mismatches=rows.filter((r,i)=>JSON.stringify(r)!==JSON.stringify(old.rows[i])).map(r=>r.id);await writeFile(new URL('./🗑️generated/glb-reader-comparison-v3.json',import.meta.url),JSON.stringify({cases:rows.length,mismatches,differences:rows.flatMap((r,i)=>JSON.stringify(r)!==JSON.stringify(old.rows[i])?[{id:r.id,before:old.rows[i].error,after:r.error,acceptedBefore:old.rows[i].accepted,acceptedAfter:r.accepted}]:[]),nativeExecution:false},null,2));console.log('[DEBUG] '+JSON.stringify({cases:rows.length,mismatches}));if(mismatches.length)process.exitCode=1;}else{try{await access(path);throw Error('Baseline exists');}catch(e){if(e.code!=='ENOENT')throw e;}await writeFile(path,JSON.stringify({producer:'actual pre-change readIfcGltfGlb',nativeExecution:false,rows},null,2));console.log('[DEBUG] '+JSON.stringify({cases:rows.length,accepted:rows.filter(r=>r.accepted).length}));}



