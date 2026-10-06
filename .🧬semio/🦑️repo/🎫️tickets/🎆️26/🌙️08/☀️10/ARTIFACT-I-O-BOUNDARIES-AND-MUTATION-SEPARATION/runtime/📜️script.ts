import { readFileSync, writeFileSync, mkdirSync, existsSync, renameSync, readdirSync, statSync } from 'node:fs';
import { resolve, dirname, join, relative } from 'node:path';
import { execFileSync } from 'node:child_process';

const repo = process.cwd();
const ticket = resolve(import.meta.dir, '..');
const files = execFileSync('rg', ['--files', '-g', '*.rs', '✏️s/🔌️plugins'], { cwd: repo, encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 }).trim().split('\n').map(f => resolve(repo, f));

function mask(source: string): string {
  const out = source.split('');
  const hide = (a: number, b: number) => { for (let j = a; j < b; j++) if (out[j] !== '\n') out[j] = ' '; };
  for (let i = 0; i < source.length;) {
    if (source.startsWith('//', i)) { const end = source.indexOf('\n', i); const stop = end < 0 ? source.length : end; hide(i, stop); i = stop; continue; }
    if (source.startsWith('/*', i)) { let end = i + 2, level = 1; while (end < source.length && level) { if (source.startsWith('/*', end)) { level++; end += 2; } else if (source.startsWith('*/', end)) { level--; end += 2; } else end++; } hide(i, end); i = end; continue; }
    const raw = source[i] === 'r' ? source.slice(i, i + 16).match(/^r(#{0,12})"/) : null;
    if (raw) { const end = source.indexOf('"' + raw[1], i + raw[0].length); const stop = end < 0 ? source.length : end + raw[1].length + 1; hide(i, stop); i = stop; continue; }
    if (source[i] === '"') { let end = i + 1; while (end < source.length) { if (source[end] === '\\') end += 2; else if (source[end++] === '"') break; } hide(i, end); i = end; continue; }
    const char = source[i] === "'" ? source.slice(i, i + 24).match(/^'(?:\\(?:u\{[a-fA-F0-9]+\}|.|x[a-fA-F0-9]{2})|[^'\n])'/u) : null;
    if (char) { hide(i, i + char[0].length); i += char[0].length; continue; }
    i++;
  }
  return out.join('');
}

type Item = { start: number, end: number, code: string, head: string, name: string, kind: string };
function items(source: string): Item[] {
  const clean = mask(source), depth: number[] = []; let level = 0;
  for (let i = 0; i < clean.length; i++) { depth[i] = level; if (clean[i] === '{') level++; else if (clean[i] === '}') level--; }
  const result: Item[] = [];
  const re = /^(?:(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?)?(fn|const|struct|enum|impl|use)\b[^\n]*/gm;
  for (const found of clean.matchAll(re)) {
    if (depth[found.index!] !== 0) continue;
    let start = found.index!, pos = start;
    while (pos > 0) { const end = pos - 1, begin = clean.lastIndexOf('\n', end - 1) + 1, line = source.slice(begin, end).trim(); if (line.startsWith('///') || line.startsWith('#[') || line.startsWith('// 🚫️async')) { start = begin; pos = begin; } else break; }
    let end = found.index!; const kind = /^pub const fn|^const fn/.test(source.slice(found.index!, source.indexOf('\n',found.index!))) ? 'fn' : found[1];
    if (kind === 'use' || kind === 'const') { end = clean.indexOf(';', end) + 1; }
    else { let open = clean.indexOf('{', end); if (open < 0) continue; let cursor = open + 1, n = 1; while (cursor < clean.length && n) { if (clean[cursor] === '{') n++; else if (clean[cursor] === '}') n--; cursor++; } end = cursor; }
    const head = source.slice(found.index!, source.indexOf('\n', found.index!));
    const name = (kind==='fn'?head.match(/\bfn\s+([\w]+)/):head.match(/\b(?:const|struct|enum)\s+([\w]+)/))?.[1] ?? '';
    result.push({ start, end, code: source.slice(start, end), head, name, kind });
  }
  return result;
}

function wireFunction(item:Item,file:string):boolean {
if(item.kind!=='fn')return false;
if(['read_region','write_region','parse_f64'].includes(item.name))return false;
if(/\bserde_json::Value\b/.test(item.head))return true;
if(/(?:as (?:store::)?Artifact(?:Dsl|Pack)>|Artifact(?:Dsl|Pack)::|semio_framework_pack_json::(?:to_json_string|to_string(?:_pretty)?|parse|from_json_str(?:ing)?)|serde_json::(?:to_string(?:_pretty)?|from_str)|(?:\.|::)(?:parse_dsl|print_dsl|encode_pack|decode_pack)\()/.test(mask(item.code)))return true;
if(/(?:^hex_(?:encode|decode)$|^(?:enc|dec)_|_report_json$|^(?:encode|decode)_snapshot(?:_binary)?$)/.test(item.name))return true;
if(/^read_/.test(item.name))return /source_(?:text|binary)$/.test(item.name)||/(?:ByteReader|\[u8\])/.test(item.head)&&item.name!=='read_region';
return /^(?:encode|decode|parse|print|write|serialize|deserialize)_/.test(item.name)&&(file.includes('/📸️snapshot/')||/(?:json|dsl|binary|pack|text)/.test(item.name)) || /^assert_(?:json|native)_round_trip$/.test(item.name);
}

if(process.argv[2]==='cad-frontiers'){
 const file=files.find(file=>file.endsWith('/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🚦️frontiers/🦀️.rs'))!,destination=file.replace('/🧬️schema/📎️references/','/🚪️io/🪶️sqlite/📸️snapshot/'),scope='standards::v1::subsets::any';let source=readFileSync(file,'utf8');
 const chosen=items(source).filter(item=>['keyed','by_identity','dense','references'].includes(item.name));
 for(const item of chosen.sort((a,b)=>b.start-a.start))source=source.slice(0,item.start)+source.slice(item.end);
 source=source.replace('use store::sqlite_snapshot::SqliteRow;\n','').replace('use super::{invalid, close};','use super::close;');writeFileSync(file,source);
 mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,'//! 🚦️ Borrowed SQLite row admission and relational ordering.\nuse semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};\nuse store::sqlite_snapshot::SqliteRow;\nuse crate::'+scope+'::schema::references::{invalid,frontiers::order};\n'+chosen.sort((a,b)=>a.start-b.start).map(item=>item.code).join('\n')+'\n');
 const owner=join(dirname(dirname(destination)),'🦀️.rs');let ownerSource=readFileSync(owner,'utf8');ownerSource+='\n#[path="🚦️frontiers/🦀️.rs"]\npub(crate) mod frontiers;\n';writeFileSync(owner,ownerSource);
 for(const consumer of [owner,join(dirname(owner),'🚦️owner/🦀️.rs')]){let code=readFileSync(consumer,'utf8');code=code.replace('schema::references::{close, invalid, frontiers};','schema::references::{close, invalid};\nuse crate::'+scope+'::schema::references::frontiers as semantic_frontiers;'+(consumer===owner?'':'\nuse crate::'+scope+'::io::sqlite::snapshot::frontiers;'));code=code.replaceAll('frontiers::collect','semantic_frontiers::collect');writeFileSync(consumer,code);}
 writeFileSync(join(ticket,'🗑️generated/runtime-cad-frontiers.json'),JSON.stringify({file,destination,owner,names:chosen.map(item=>item.name)},null,2));console.log('[DEBUG] CAD SQLite frontier adapters moved '+chosen.length);
}
if(process.argv[2]==='host-closure'){
 let traitImpls=0,aliases=0;
 for(const file of files.filter(file=>file.includes('/🀄️wfc/')&&file.includes('/🏠️host/')&&file.endsWith('/🦀️.rs'))){let source=readFileSync(file,'utf8');const all=items(source);for(const item of all.filter(item=>item.kind==='impl'&&/\bfor\b/.test(item.head)).sort((a,b)=>b.start-a.start)){const code=item.code.replace(/\bpub\(crate\) fn /g,'fn ');if(code!==item.code){source=source.slice(0,item.start)+code+source.slice(item.end);traitImpls++;}}writeFileSync(file,source);}
 for(const file of files.filter(file=>file.includes('/🀄️wfc/')&&file.includes('/🚪️io/'))){let source=readFileSync(file,'utf8'),before=source;source=source.replaceAll('crate::io::','crate::standards::v1::subsets::any::io::');source=source.replace(/(?:\/\/\/[^\n]*\n)*#\[allow\(clippy::too_many_arguments\)\]\s*(?=\})/g,'');if(source!==before){writeFileSync(file,source);aliases++;}}
 for(const file of files.filter(file=>file.includes('/🀄️wfc/')&&file.endsWith('/🦀️.rs'))){let source=readFileSync(file,'utf8'),before=source;source=source.replace(/(pub use host::inferences::\*;\s*){2,}/g,'pub use host::inferences::*;\n');if(source!==before)writeFileSync(file,source);}
 console.log('[DEBUG] WFC trait impl visibility repairs '+traitImpls+', canonical IO owners '+aliases);
}
if(process.argv[2]==='bitmap-host-bytes'){
 const host=files.find(file=>file.includes('/🀄️wfc/🗿️artifacts/🖼️bitmap/')&&file.endsWith('/🏠️host/💡️inferences/🦀️.rs'))!,io=host.slice(0,host.indexOf('/🔨️modules/'))+'/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/💡️inferences/🦀️.rs';
 let source=readFileSync(host,'utf8').replaceAll('io::binary::snapshot::{encode_base64}','io::text::snapshot::{encode_base64}').replace('snapshot.input.pixels.as_bytes()','snapshot.input.pixels.as_slice()');writeFileSync(host,source);
 source=readFileSync(io,'utf8').replaceAll('io::binary::snapshot::{encode_base64}','io::text::snapshot::{encode_base64}').replace('semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|error.to_string())','crate::standards::v1::subsets::any::io::text::bitmap_json_decode(text).map_err(|error|error.to_string())').replace('semio_framework_pack_json::to_json_string(value)','crate::standards::v1::subsets::any::io::text::bitmap_json_encode(value)');writeFileSync(io,source);
 writeFileSync(join(ticket,'🗑️generated/runtime-bitmap-host-bytes.json'),JSON.stringify({host,io},null,2));console.log('[DEBUG] Bitmap retained output and request codecs use the intrinsic pixel binder');
}
if(process.argv[2]==='dwg-sqlite-types'){
 let count=0;for(const root of files.filter(file=>file.includes('/🖊️dwg/')&&file.endsWith('/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs'))){const namespace=readFileSync(root,'utf8').match(/use (crate::[\w:]+::schema::snapshot)::\*;/)?.[1];if(!namespace)throw new Error('Missing DWG schema import '+root);for(const file of files.filter(file=>file.startsWith(dirname(root)+'/'))){let source=readFileSync(file,'utf8'),before=source;source=source.replace(/use super::super::\*;/g,'use '+namespace+'::*;');if(source!==before){writeFileSync(file,source);count++;}}}console.log('[DEBUG] DWG physical SQL leaf semantic type imports '+count);
}
if(process.argv[2]==='payload-imports'){
 let changed=0;for(const file of files.filter(file=>file.includes('/🚪️io/')&&file.includes('/🧬️mutations/'))){let source=readFileSync(file,'utf8'),before=source;const subset=file.slice(0,file.indexOf('/🚪️io/')),semanticFile=join(subset,'🧬️schema/🧬️mutations/🦀️.rs');if(!existsSync(semanticFile))continue;const semantic=readFileSync(semanticFile,'utf8');
 source=source.replace(/::io::(?:binary|text)::mutations::(\w+)/g,(whole,name)=>new RegExp('\\b'+name+'\\b').test(semantic)?'::schema::mutations::'+name:whole);
 if(file.endsWith('/🧬️mutations/🦀️.rs')){const moduleNames=[...mask(source).matchAll(/\bmod (\w+)\s*;/g)].map(found=>found[1]);for(const name of moduleNames){const pattern=new RegExp('(use crate::[^;]+schema::mutations::\\{)([^}]+)(\\};)','g');let aliased=false;source=source.replace(pattern,(whole,prefix,members,suffix)=>{const parts=members.split(',').map((part:string)=>part.trim());if(!parts.includes(name))return whole;aliased=true;return prefix+parts.map((part:string)=>part===name?name+' as semantic_'+name:part).join(', ')+suffix;});if(aliased)source=source.replace(new RegExp('(?<![\\w:])'+name+'::','g'),'semantic_'+name+'::');}}
 if(source!==before){writeFileSync(file,source);changed++;}}
 console.log('[DEBUG] Canonical semantic payload imports and representation mount collisions repaired '+changed);
}
if(process.argv[2]==='deflate-native'){
 const root=files.find(file=>file.endsWith('/🗜️deflate/🦀️.rs'))!,schema=join(dirname(root),'🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs'),scope='standards::v_rfc1950::subsets::any',names=['compress_zlib','decompress_zlib'];let source=readFileSync(schema,'utf8').replace(/\/\/\/ 🗜️ Declares[^\n]*\npub use sqlite_native::\{[^}]+\};/,'');writeFileSync(schema,source);
 for(const file of files){let code=readFileSync(file,'utf8'),before=code;if(file.startsWith(dirname(root)))code=code.replaceAll('use sqlite_native::{compress_zlib,decompress_zlib};','use crate::'+scope+'::io::sqlite::snapshot::native::{compress_zlib,decompress_zlib};');for(const name of names)code=code.replaceAll(scope+'::schema::snapshot::'+name,scope+'::io::binary::snapshot::'+name).replaceAll('semio_s_artifact_stdio_deflate::schema::snapshot::'+name,'semio_s_artifact_stdio_deflate::'+scope+'::io::binary::snapshot::'+name);if(code!==before)writeFileSync(file,code);}
 const binary=schema.replace('/🧬️schema/','/🚪️io/💾️binary/');writeFileSync(binary,readFileSync(binary,'utf8')+'\npub use crate::'+scope+'::io::sqlite::snapshot::native::{compress_zlib,decompress_zlib};\n');console.log('[DEBUG] Deflate bounded codec helper exports owned binary IO');
}
if(process.argv[2]==='txt-registry'){
 for(const file of files.filter(file=>file.includes('/🔤️txt/')&&/\/🚪️io\/(?:💾️binary|📝️text)\/🧬️mutations\/🦀️\.rs$/.test(file))){let source=readFileSync(file,'utf8'),names=['insert_line','remove_line','set_line','set_line_ending','set_snapshot','set_trailing_newline'];source=source.replace(/use crate::schema::mutations::\{[^}]+\};/,'use crate::schema::mutations::TxtMutation;');for(const name of names)for(const rep of ['binary','text'])source=source.replaceAll(name+'::'+rep+'::','crate::standards::v_utf_8::subsets::any::io::'+rep+'::mutations::'+name+'::').replaceAll('semantic_'+name+'::'+rep+'::','crate::standards::v_utf_8::subsets::any::io::'+rep+'::mutations::'+name+'::');writeFileSync(file,source);}console.log('[DEBUG] TXT codec registries use their own representation members');
}
if(process.argv[2]==='las-closure'){
 const root=files.find(file=>file.endsWith('/☁️las/🦀️.rs'))!,physical=join(dirname(root),'🏅️standards/🔖️1.0/🪆️subsets/🎩️header'),semantic='standards::v1_0::subsets::any::schema::mutations',binary=join(physical,'🚪️io/💾️binary/🧬️mutations/🦀️.rs'),semanticFile=join(physical,'🧬️schema/🧬️mutations/🦀️.rs');let source=readFileSync(binary,'utf8'),pieces=[];
 for(const name of ['point','vlr']){const clean=mask(source),match=clean.match(new RegExp('^pub\\(crate\\) fn '+name+'\\b','m'));if(!match)continue;let start=match.index!,pos=start;while(pos>0){const end=pos-1,begin=source.lastIndexOf('\n',end-1)+1,line=source.slice(begin,end).trim();if(line.startsWith('///')||line.startsWith('#[')){start=begin;pos=begin;}else break;}let end=clean.indexOf('{',match.index!)+1,n=1;while(end<clean.length&&n){if(clean[end]==='{')n++;else if(clean[end]==='}')n--;end++;}pieces.push(source.slice(start,end));source=source.slice(0,start)+source.slice(end);}
 writeFileSync(binary,source);writeFileSync(semanticFile,readFileSync(semanticFile,'utf8')+'\n'+pieces.join('\n')+'\n');
 for(const file of files.filter(file=>file.startsWith(physical)&&file.includes('/🚪️io/')&&!file.includes('/🔺️diff/'))){let code=readFileSync(file,'utf8'),before=code;code=code.replace(/use super::(?:super::)+\{(LasHeader,LasPoint,LasSnapshot,LasVlr)\};/g,'use crate::schema::snapshot::{$1};').replace('use super::super::LasSnapshot;','use crate::LasSnapshot;').replace(/use crate::[\w:]+::io::binary::mutations::\{(point|vlr)\};/g,'#[cfg(test)]\nuse crate::'+semantic+'::{$1};');if(code!==before)writeFileSync(file,code);}
 console.log('[DEBUG] LAS pure fixture helpers restored and native imports canonical '+pieces.length);
}
function repairWireDiffRoutes(){
 let count=0;for(const file of files.filter(file=>file.includes('/🚪️io/')&&!file.includes('/🔺️diff/')&&!file.includes('/🧪️tests/'))){let source=readFileSync(file,'utf8'),before=source;if(!source.includes('diff::'))continue;const subset=file.slice(0,file.indexOf('/🚪️io/')),semanticFile=join(subset,'🧬️schema/🔺️diff/🦀️.rs'),entry=sourceModules.get(semanticFile)?.[0];if(!entry)continue;const scope=entry.module.replace(/::component(?=::|$)/g,'').split('::schema')[0],rep=file.includes('/💾️binary/')?'binary':'text',other=rep==='binary'?'text':'binary';
 for(const representation of [rep,other]){const io=join(subset,'🚪️io',representation==='binary'?'💾️binary':'📝️text','🔺️diff/🦀️.rs');if(!existsSync(io))continue;const names=[...mask(readFileSync(io,'utf8')).matchAll(/\bpub(?:\(crate\))? fn (\w+)/g)].map(found=>found[1]);for(const name of names)source=source.replace(new RegExp('(?<![\\w:])diff::'+name+'\\b','g'),'crate::'+scope+'::io::'+representation+'::diff::'+name);}
 if(source!==before){writeFileSync(file,source);count++;}}
 console.log('[DEBUG] Codec bodies use physical diff helper owners '+count);
}
function extractGltfFacades(){
 const file=files.find(file=>file.includes('/🧊️gltf/')&&file.endsWith('/🧬️mutations/🌳️node/🏷️rename/🦀️.rs'))!,owner=sourceModules.get(file)![0],semantic=owner.module.replace(/::component$/,''),scope=semantic.split('::schema')[0],subset=file.slice(0,file.indexOf('/🧬️schema/')),io=scope+'::io';
 let source=readFileSync(file,'utf8');const begin=source.indexOf('//#region 🔗️Facade'),end=source.indexOf('//#endregion 🔗️Facade',begin)+'//#endregion 🔗️Facade'.length,facade=source.slice(begin,end),binaryBegin=facade.indexOf("struct FacadeProtobufReader"),textBody=facade.slice(0,binaryBegin),binaryBody=facade.slice(binaryBegin).replace('//#endregion 🔗️Facade','');
 source=source.slice(0,begin)+source.slice(end);writeFileSync(file,source.replace('use semio_framework_value::DslValue;\n',''));
 const textFile=join(subset,'🚪️io/📝️text/🧬️mutations/🦀️.rs'),binaryFile=join(subset,'🚪️io/💾️binary/🧬️mutations/🦀️.rs');
 const text=textBody.replace('type FacadeResult','pub(crate) type FacadeResult').replace('fn facade_error','pub(crate) fn facade_error');
 writeFileSync(textFile,readFileSync(textFile,'utf8')+'\nmod node_name_facades {\nuse crate::'+semantic+'::*;\nuse semio_framework_value::DslValue;\n'+text+'\n}\npub use node_name_facades::{GltfChangeNodeNameFacadeError,decode_gltf_change_node_name_graphql,decode_gltf_change_node_name_proto};\npub(crate) use node_name_facades::{FacadeResult,facade_error};\n');
 writeFileSync(binaryFile,readFileSync(binaryFile,'utf8')+'\nmod node_name_protobuf {\nuse crate::'+semantic+'::*;\nuse crate::'+io+'::text::mutations::{FacadeResult,facade_error};\n'+binaryBody+'\n}\npub use node_name_protobuf::decode_gltf_change_node_name_protobuf;\n');
 const moved={root:owner.root,semantic,io:io+'::text::mutations',names:['GltfChangeNodeNameFacadeError','decode_gltf_change_node_name_graphql','decode_gltf_change_node_name_proto']},binaryMoved={root:owner.root,semantic,io:io+'::binary::mutations',names:['decode_gltf_change_node_name_protobuf']};
 const previous=JSON.parse(readFileSync(join(ticket,'🗑️generated/runtime-moved.json'),'utf8'));writeFileSync(join(ticket,'🗑️generated/runtime-moved.json'),JSON.stringify([...previous,moved,binaryMoved],null,2));writeFileSync(join(ticket,'🗑️generated/runtime-gltf-facades.json'),JSON.stringify({file,textFile,binaryFile,moved,binaryMoved},null,2));console.log('[DEBUG] glTF GraphQL/proto/protobuf adapters own IO');
}
function extractSerdeOwners(){
 const gltf=files.find(file=>file.includes('/🧊️gltf/')&&file.endsWith('/🧬️schema/📸️snapshot/🦀️.rs'))!,io=gltf.replace('/🧬️schema/','/🚪️io/📝️text/');let source=readFileSync(gltf,'utf8'),clean=mask(source);
 if(source.includes('impl Serialize for GltfJson')){
 const begin=source.indexOf('impl Serialize for GltfJson'),end=source.indexOf('/// 🌉️ `ToValue`/`FromValue`',begin);let serdeBody=source.slice(begin,end);source=source.slice(0,begin)+source.slice(end);
 const moduleBegin=source.indexOf('mod ordered_attr_map {'),moduleOpen=moduleBegin+'mod ordered_attr_map {'.length;clean=mask(source);let cursor=moduleOpen,n=1;while(cursor<clean.length&&n){if(clean[cursor]==='{')n++;else if(clean[cursor]==='}')n--;cursor++;}let ordered=source.slice(moduleBegin,cursor).replace('mod ordered_attr_map {','pub(crate) mod ordered_attr_map {');source=source.slice(0,moduleBegin)+source.slice(cursor);
 source=source.replace(/#\[derive\(([^\n]+)\)\]/g,(whole,args)=>{const traits=args.split(',').map((part:string)=>part.trim()),serde=traits.filter((part:string)=>['Serialize','Deserialize'].includes(part));return serde.length?'#[derive('+traits.filter((part:string)=>!serde.includes(part)).join(', ')+')]\n#[cfg_attr(test, derive('+serde.join(', ')+'))]':whole;});
 source=source.replace(/#\[serde\(([^\n]*)\)\]/g,'#[cfg_attr(test, serde($1))]');source=source.replaceAll('ordered_attr_map::','crate::standards::v2_0::subsets::any::io::text::snapshot::ordered_attr_map::');
 source=source.replace(/^use serde::(?:de|ser)::[^\n]+\n/gm,'').replace('use serde::{Deserialize, Deserializer, Serialize, Serializer};','#[cfg(test)]\nuse serde::{Deserialize, Serialize};').replace('use std::fmt;\n','');
 writeFileSync(gltf,source);writeFileSync(io,readFileSync(io,'utf8')+'\n#[cfg(test)]\nmod serde_oracle {\nuse crate::standards::v2_0::subsets::any::schema::snapshot::*;\nuse serde::de::{MapAccess,SeqAccess,Visitor};\nuse serde::ser::{SerializeMap,SerializeSeq};\nuse serde::{Deserialize,Deserializer,Serialize,Serializer};\nuse std::fmt;\n'+serdeBody+'\n'+ordered+'\n}\n#[cfg(test)]\npub(crate) use serde_oracle::ordered_attr_map;\n');
 }
 const flattened=files.find(file=>file.includes('/🧿️semio/')&&file.endsWith('/🎛️flattened-scene/🦀️.rs'))!,entry=sourceModules.get(flattened)![0],sem=entry.module.replace(/::component$/,''),scope=sem.split('::schema')[0],destination=flattened.slice(0,flattened.indexOf('/🧬️schema/'))+'/🚪️io/📝️text/💡️inferences/🦀️.rs';source=readFileSync(flattened,'utf8');const chosen=items(source).filter(item=>item.kind==='impl'&&/serde::(?:Serialize|Deserialize)/.test(item.head));for(const item of chosen.sort((a,b)=>b.start-a.start))source=source.slice(0,item.start)+source.slice(item.end);writeFileSync(flattened,source);mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,(existsSync(destination)?readFileSync(destination,'utf8'):'//! 📝️ Drawing inference physical representations.\n')+'\nmod flattened_node_cache {\nuse crate::'+sem+'::FlattenedNode;\n'+chosen.sort((a,b)=>a.start-b.start).map(item=>item.code).join('\n')+'\n}\n');
 const repOwner=join(dirname(dirname(destination)),'🦀️.rs');if(!existsSync(repOwner))writeFileSync(repOwner,'//! 📝️ Drawing text representations.\n');let repSource=readFileSync(repOwner,'utf8');if(!/mod inferences;/.test(repSource))repSource+='\n#[path="💡️inferences/🦀️.rs"]\npub mod inferences;\n';writeFileSync(repOwner,repSource);
 writeFileSync(join(ticket,'🗑️generated/runtime-serde-owners.json'),JSON.stringify({gltf,io,flattened,destination},null,2));console.log('[DEBUG] glTF retained serde test oracle and drawing bytecache serializers relocated');
}

const sourceModules = new Map<string, { root: string, module: string }[]>();
const contexts: { file: string, root: string, module: string, base: string, end: number }[] = [];
function modules(file: string, root: string, module: string, seen: Set<string>): void {
  const key = file + ':' + module; if (seen.has(key) || !existsSync(file)) return; seen.add(key);
  const previous = sourceModules.get(file) ?? []; previous.push({ root, module }); sourceModules.set(file, previous);
  const source = readFileSync(file, 'utf8'), clean = mask(source);
  function walk(from: number, to: number, current: string, base: string): void {
    contexts.push({ file, root, module: current, base, end: to });
    const re = /\b(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*([;{])/g; re.lastIndex = from;
    while (true) {
      const found = re.exec(clean); if (!found || found.index >= to) break;
      const prefix = source.slice(Math.max(from, source.lastIndexOf('\n\n', found.index) + 2), found.index);
      const paths = [...prefix.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)], path = paths.at(-1)?.[1];
      const nextModule = [current, found[1]].filter(Boolean).join('::');
      if (found[2] === ';') {
        const target = path ? resolve(base, path) : resolve(base, found[1] + '.rs');
        if (target.startsWith(dirname(root)) && !/\/(?:🧪️tests|⚙️engine|🚪️io)\//.test(target)) modules(target, root, nextModule, seen);
      } else {
        const open = re.lastIndex - 1; let end = open + 1, level = 1;
        while (end < to && level) { if (clean[end] === '{') level++; else if (clean[end] === '}') level--; end++; }
        walk(open + 1, end - 1, nextModule, path ? resolve(base, path) : resolve(base, found[1])); re.lastIndex = end;
      }
    }
  }
  walk(0, source.length, module, dirname(file));
}
if (!['pack-mounts', 'codec-links', 'split-native'].includes(process.argv[2])) for (const file of files.filter(f => /\/🗿️artifacts\/[^/]+\/🦀️\.rs$/.test(f) || /\/🧩️extensions\/[^/]+\/🦀️\.rs$/.test(f))) modules(file, file, '', new Set());

const roots = files.filter(f => f.includes('/🧬️schema/') && !f.includes('/🧪️tests/') && sourceModules.has(f) && !(process.argv[2]==='extract'&&(f.includes('/🀄️wfc/🗿️artifacts/🖼️bitmap/')||f.includes('/🔌️jack/')&&f.includes('/executor/'))));
if(process.argv[2]==='gltf-facades')extractGltfFacades();
if(process.argv[2]==='serde-owners')extractSerdeOwners();
if(process.argv[2]==='wire-diff-routes')repairWireDiffRoutes();
if(process.argv[2]==='cache-codecs'){
 const changed=[];
 for(const file of roots){let source=readFileSync(file,'utf8');const all=items(source),chosen=all.filter(item=>item.kind==='impl'&&/\b(?:serde::)?(?:Serialize|Deserialize)\b/.test(item.head)&&!/#\[cfg\(test\)\]/.test(item.code));if(!chosen.length)continue;const owner=sourceModules.get(file)![0],semantic=owner.module.replace(/::component$/,''),facet=file.includes('/💡️inferences/')?'inferences':file.includes('/🧬️mutations/')?'mutations':'snapshot',glyph=facet==='inferences'?'💡️inferences':facet==='mutations'?'🧬️mutations':'📸️snapshot',subset=file.slice(0,file.indexOf('/🧬️schema/')),destination=join(subset,'🚪️io/📝️text',glyph,'🦀️.rs');
 const imports=all.filter(item=>item.kind==='use').map(item=>item.code.replace(/^pub use/,'use').replace(/\bsuper::/g,'crate::'+semantic.split('::').slice(0,-1).join('::')+'::')).join('\n');
 for(const item of chosen.sort((a,b)=>b.start-a.start))source=source.slice(0,item.start)+source.slice(item.end);writeFileSync(file,source);mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,(existsSync(destination)?readFileSync(destination,'utf8'):'//! 📝️ Artifact physical text representations.\n')+'\nmod semantic_cache_codec {\nuse crate::'+semantic+'::*;\n'+imports+'\n'+chosen.sort((a,b)=>a.start-b.start).map(item=>item.code).join('\n')+'\n}\n');
 const rep=join(subset,'🚪️io/📝️text/🦀️.rs');let repSource=existsSync(rep)?readFileSync(rep,'utf8'):'//! 📝️ Artifact text representations.\n';if(!new RegExp('mod '+facet+';').test(repSource))repSource+='\n#[path="'+glyph+'/🦀️.rs"]\npub mod '+facet+';\n';mkdirSync(dirname(rep),{recursive:true});writeFileSync(rep,repSource);changed.push({file,destination,names:chosen.map(item=>item.head)});
 }
 writeFileSync(join(ticket,'🗑️generated/runtime-cache-codecs.json'),JSON.stringify(changed,null,2));console.log('[DEBUG] Explicit physical cache codec owners '+changed.length);
}
if(process.argv[2]==='puzzle-checkpoint'){
 const file=files.find(file=>file.includes('/🧩️puzzle/🗿️artifacts/🧊️3d/')&&file.endsWith('/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs'))!,entry=sourceModules.get(file)![0],semantic=entry.module.replace(/::component$/,''),scope=semantic.split('::schema')[0],destination=file.replace('/🧬️schema/','/🚪️io/💾️binary/💡️inferences/');let source=readFileSync(file,'utf8');const start=source.indexOf('impl FillRunCheckpoint {');if(start<0)throw new Error('Missing checkpoint owner');const clean=mask(source);let end=clean.indexOf('{',start)+1,n=1;while(end<clean.length&&n){if(clean[end]==='{')n++;else if(clean[end]==='}')n--;end++;}const body=source.slice(start,end);source=source.slice(0,start)+source.slice(end);writeFileSync(file,source);mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,(existsSync(destination)?readFileSync(destination,'utf8'):'//! 💾️ Fill inference checkpoint representation.\n')+'\nuse crate::'+semantic+'::FillRunCheckpoint;\n'+body+'\n');const rep=join(dirname(dirname(destination)),'🦀️.rs');let repSource=readFileSync(rep,'utf8');if(!/mod inferences;/.test(repSource))repSource+='\n#[path="💡️inferences/🦀️.rs"]\npub mod inferences;\n';writeFileSync(rep,repSource);writeFileSync(join(ticket,'🗑️generated/runtime-puzzle-checkpoint.json'),JSON.stringify({file,destination,rep},null,2));console.log('[DEBUG] Fill checkpoint byte methods owned binary inference IO');
}
if(process.argv[2]==='wfc-host'){
 const changed=[];
 for(const file of roots.filter(f=>f.includes('/🀄️wfc/')&&f.includes('/💡️inferences/'))){
  const source=readFileSync(file,'utf8'),clean=mask(source),all=items(source);
  writeFileSync(join(ticket,'🗑️generated/wfc-source-'+file.match(/artifacts\/([^/]+)/)![1]+'.txt'),source);
  const artifact=file.slice(0,file.indexOf('/🏅️standards/')),artifactRoot=join(artifact,'🦀️.rs');
  const job=all.find(i=>i.kind==='struct'&&/InferenceJob$/.test(i.name));if(!job)continue;
  const prefix=job.name.replace('InferenceJob','');
  const selected=all.filter(i=>i.name&&(/Inference(?:Request|Stage|Job|JobFactory)$/.test(i.name)||/INFERENCE_(?:JOB_KIND|TOOL_ID|PAYLOAD_SCHEMA|REQUEST_SCHEMA|CONTRACT|COMMIT_ACTION)$/.test(i.name)||/^(?:retire_payload|retained_payload|solve_with_job|solve_with_clock|register_.*_inference_factory|.*_inference_metadata|.*_artifact_inference_descriptor)$/.test(i.name))||i.kind==='enum'&&i.name==='EncodePhase'||i.kind==='impl'&&(/Inference(?:Request|Job|JobFactory)\b/.test(i.head)||/InferredField</.test(i.head)));
  // Unit marker structs are not parsed as braced items; retain them beside their domain result types.
  const safe=selected.filter(i=>!/^pub struct \w+;$/.test(i.head));
  let host=safe.map(i=>i.code).join('\n\n'),remaining=source;
  for(const item of [...safe].sort((a,b)=>b.start-a.start))remaining=remaining.slice(0,item.start)+remaining.slice(item.end);
  remaining=remaining.replace(/^const (MAX_\w+)\b/gm,'pub(crate) const $1').replace(/^fn (\w+)\(/gm,'pub(crate) fn $1(');
  const methodSource=mask(host),extractMethods: {start:number,end:number,code:string}[]=[];
  for(const match of methodSource.matchAll(/^    fn (next_encode_chunk|encode_chunk)\b[^\n]*/gm)){
   const open=methodSource.indexOf('{',match.index!),stackEnd=(()=>{let p=open+1,n=1;while(n){if(methodSource[p]==='{')n++;else if(methodSource[p]==='}')n--;p++;}return p;})();
   let begin=match.index!;while(begin>0){const end=begin-1,start=host.lastIndexOf('\n',end-1)+1;if(host.slice(start,end).trim().startsWith('///'))begin=start;else break;}
   extractMethods.push({start:begin,end:stackEnd,code:host.slice(begin,stackEnd).replace('    fn ','    pub(crate) fn ')});
  }
  for(const item of [...extractMethods].sort((a,b)=>b.start-a.start))host=host.slice(0,item.start)+host.slice(item.end);
  host=host.replace(/^    fn (\w+)\(/gm,'    pub(crate) fn $1(');
  host=host.replace(/pub struct (\w+InferenceJob) \{([\s\S]*?)\n\}/g,(whole,name,body)=>`pub struct ${name} {${body.replace(/^    (\w+):/gm,'    pub(crate) $1:')}\n}`);
  host=host.replace(/^enum EncodePhase/gm,'pub(crate) enum EncodePhase');
  const ioNamespace='crate::standards::v1::subsets::any::io::text::inferences';
  host=host.replace(/semio_framework_pack_json::from_json_str(?:::<([^>]+)>)?\((\w+), semio_framework_pack_json::JsonMemberPolicy::Reject\)/g,(_,type,arg)=>`${ioNamespace}::decode_inference_value${type?'::<'+type+'>':''}(${arg})`);
  host=host.replaceAll('semio_framework_pack_json::to_json_string(',ioNamespace+'::encode_inference_value(');
  const imports=all.filter(i=>i.kind==='use').map(i=>i.code.replace(/^pub(?:\([^)]*\))? use /,'use ')).join('\n');
  const owner=join(artifact,'🔨️modules/🏠️host/💡️inferences/🦀️.rs');mkdirSync(dirname(owner),{recursive:true});
  const hostBody='//! 🏠️ Inference admission, publication and retained job lifecycle.\n'+imports+'\nuse crate::standards::v1::subsets::any::schema::inferences::*;\n'+host;
  writeFileSync(owner,hostBody+'\n#[cfg(test)]\n#[path="🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n');
  const testDir=join(dirname(file),'🧪️tests');if(existsSync(testDir))renameSync(testDir,join(dirname(owner),'🧪️tests'));
  remaining=remaining.replace(/#\[cfg\(test\)\]\s*#\[path = "🧪️tests\/🔬️unit\/🦀️\.rs"\]\s*mod tests;/g,'');
  remaining=remaining.replace(/use crate::standards::v1::subsets::any::io::binary::snapshot::\{encode_base64\};\n/,'');
  writeFileSync(file,remaining);
  const mount=join(artifact,'🔨️modules/🏠️host/🦀️.rs');mkdirSync(dirname(mount),{recursive:true});
  let mounting=existsSync(mount)?readFileSync(mount,'utf8'):'//! 🏠️ Host services for the artifact.\n';mounting+='\n#[path="💡️inferences/🦀️.rs"]\npub mod inferences;\n';writeFileSync(mount,mounting);
  let root=readFileSync(artifactRoot,'utf8');if(!root.includes('pub mod host;'))root+='\n#[path="🔨️modules/🏠️host/🦀️.rs"]\npub mod host;\n';root+='\npub use host::inferences::*;\n';writeFileSync(artifactRoot,root);
  const ioFile=join(file.slice(0,file.indexOf('/🧬️schema/')),'🚪️io/📝️text/💡️inferences/🦀️.rs');
  mkdirSync(dirname(ioFile),{recursive:true});
  const repOwner=join(dirname(dirname(ioFile)),'🦀️.rs');let repSource=readFileSync(repOwner,'utf8');if(!repSource.includes('pub mod inferences;')){repSource+='\n#[path="💡️inferences/🦀️.rs"]\npub mod inferences;\n';writeFileSync(repOwner,repSource);}
  const io=(existsSync(ioFile)?readFileSync(ioFile,'utf8'):'//! 📝️ Inference request and result codecs.\n')+'\n#[allow(unused_imports)]\nmod inference_runtime_codec {\n'+imports+'\nuse crate::host::inferences::*;\nuse crate::standards::v1::subsets::any::schema::inferences::*;\n'+(extractMethods.length?`impl ${job.name} {\n${extractMethods.map(i=>i.code).join('\n')}\n}\n`:'')+'}\n'+`pub(crate) fn decode_inference_value<T:semio_framework_value::FromValue>(text:&str)->Result<T,String> {semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|error.to_string())}\npub(crate) fn encode_inference_value<T:semio_framework_value::ToValue>(value:&T)->String {semio_framework_pack_json::to_json_string(value)}\n`;
  writeFileSync(ioFile,io);
  const names=safe.filter(i=>i.name).map(i=>i.name);changed.push({semantic:relative(repo,file),host:relative(repo,owner),io:relative(repo,ioFile),names});
  for(const current of files.filter(f=>f.startsWith(artifact)&&existsSync(f))){let before=readFileSync(current,'utf8'),after=before;
   after=after.replace(/use (crate::(?:standards::v1::subsets::any::)?schema::inferences)::\{([^}]*)\};/g,(whole,path,list)=>{const parts=list.split(',').map(s=>s.trim()).filter(Boolean),moved=parts.filter(s=>names.includes(s.split(' as ')[0])),stay=parts.filter(s=>!names.includes(s.split(' as ')[0]));return(stay.length?`use ${path}::{${stay.join(', ')}};\n`:'')+(moved.length?`use crate::host::inferences::{${moved.join(', ')}};`:'');});
   for(const name of names)after=after.replaceAll('crate::schema::inferences::'+name,'crate::host::inferences::'+name).replaceAll('crate::standards::v1::subsets::any::schema::inferences::'+name,'crate::host::inferences::'+name);
   if(after!==before)writeFileSync(current,after);
  }
 }
 writeFileSync(join(ticket,'🗑️generated/runtime-wfc-host.json'),JSON.stringify(changed,null,2));
 console.log('[DEBUG] WFC host/codec split '+changed.length+' inference owners');
}
if(process.argv[2]==='wfc-host-closure'){
 const changed=[];
 for(const semantic of roots.filter(f=>f.includes('/🀄️wfc/')&&f.includes('/💡️inferences/'))){
  const artifact=semantic.slice(0,semantic.indexOf('/🏅️standards/')),host=join(artifact,'🔨️modules/🏠️host/💡️inferences/🦀️.rs');if(!existsSync(host))continue;
  let source=readFileSync(semantic,'utf8'),owner=readFileSync(host,'utf8');
  for(const item of items(source).filter(i=>i.kind==='fn'&&/_inference_metadata$/.test(i.name))){owner+='\n'+item.code+'\n';source=source.slice(0,item.start)+source.slice(item.end);}
  source=source.replace(/^const (\w+)\b/gm,'pub(crate) const $1');
  writeFileSync(semantic,source);writeFileSync(host,owner);
  const names=items(owner).filter(i=>i.name&&/^pub /.test(i.head)).map(i=>i.name);
  const currentFiles=execFileSync('rg',['--files','-g','*.rs',artifact],{encoding:'utf8'}).trim().split('\n').map(f=>resolve(repo,f));
  for(const current of currentFiles){let before=readFileSync(current,'utf8'),after=before;
   after=after.replace(/use (crate::(?:standards::v1::subsets::any::)?schema::inferences)::\{([^}]*)\};/g,(whole,path,list)=>{const parts=list.split(',').map(s=>s.trim()).filter(Boolean),moved=parts.filter(s=>names.includes(s.split(' as ')[0])),stay=parts.filter(s=>!names.includes(s.split(' as ')[0]));return(stay.length?`use ${path}::{${stay.join(', ')}};\n`:'')+(moved.length?`use crate::host::inferences::{${moved.join(', ')}};`:'');});
   for(const name of names)after=after.replaceAll('crate::schema::inferences::'+name,'crate::host::inferences::'+name).replaceAll('crate::standards::v1::subsets::any::schema::inferences::'+name,'crate::host::inferences::'+name);
   if(current===host||current.includes('/🏠️host/💡️inferences/🧪️tests/'))after=after.replace(/include_(str|bytes)!\("([^"]+)"\)/g,(whole,kind,path)=>{if(path.includes('🏅️standards')||current!==host&&existsSync(resolve(dirname(current),path)))return whole;const oldFile=current.replace(join(artifact,'🔨️modules/🏠️host/💡️inferences'),dirname(semantic));const target=resolve(dirname(oldFile),path);return`include_${kind}!("${relative(dirname(current),target)}")`;});
   if(after!==before)writeFileSync(current,after);
  }
  const textRep=join(semantic.slice(0,semantic.indexOf('/🧬️schema/')),'🚪️io/📝️text/🦀️.rs');let rep=readFileSync(textRep,'utf8');if(!rep.includes('pub mod inferences;')){rep+='\n#[path="💡️inferences/🦀️.rs"]\npub mod inferences;\n';writeFileSync(textRep,rep);}
  const rootFile=join(artifact,'🦀️.rs');let root=readFileSync(rootFile,'utf8');root=root.replace(/pub mod inferences \{\n(\s+)pub use crate::standards::v1::subsets::any::schema::inferences::\*;/,`pub mod inferences {\n$1pub use crate::standards::v1::subsets::any::schema::inferences::*;\n$1pub use crate::host::inferences::*;`);writeFileSync(rootFile,root);
  changed.push({semantic:relative(repo,semantic),host:relative(repo,host),names});
 }
 writeFileSync(join(ticket,'🗑️generated/runtime-wfc-host-closure.json'),JSON.stringify(changed,null,2));console.log('[DEBUG] WFC host imports, facets and fixture anchors closed '+changed.length+' owners');
}
if(process.argv[2]==='syntax'){
 const selected=files.filter(f=>existsSync(f)&&(/\/(?:🀄️wfc|🖍️draw)\//.test(f)||f.includes('/🧊️generation3d/')));
 const added=execFileSync('rg',['--files','-g','*.rs','✏️s/🔌️plugins/🀄️wfc','✏️s/🔌️plugins/🖍️draw'],{encoding:'utf8'}).trim().split('\n').map(f=>resolve(repo,f));
 let count=0;for(const file of new Set([...selected,...added])){try{execFileSync('rustfmt',['--emit','stdout','--config','skip_children=true','--edition','2021',file],{encoding:'utf8',maxBuffer:32*1024*1024,stdio:['ignore','pipe','pipe']});count++;}catch(error){console.error('[DEBUG] Rust syntax failure '+relative(repo,file));console.error(String((error as any).stderr));throw error;}}
 console.log('[DEBUG] Rust parser checked '+count+' Drawing, WFC and Generation3d source owners');
}
if(process.argv[2]==='doc-order'){
 let count=0;const currentFiles=execFileSync('rg',['--files','-g','*.rs','✏️s/🔌️plugins'],{encoding:'utf8',maxBuffer:32*1024*1024}).trim().split('\n');
 for(const file of currentFiles){const before=readFileSync(file,'utf8'),after=before.replace(/^((?:use [^\n]+\n)+)((?:\/\/!.*\n)+)/,'$2\n$1');if(before!==after){writeFileSync(file,after);count++;}}
 console.log('[DEBUG] Restored module doc header order '+count+' consumers');
}
if(process.argv[2]==='energy-inference-adapter'){
 const file=roots.find(f=>f.includes('/🔋️energy/')&&f.endsWith('/💡️inferences/🦀️.rs'))!,io=file.replace('/🧬️schema/','/🚪️io/📝️text/');
 let source=readFileSync(file,'utf8');const selected=items(source).filter(i=>i.kind==='impl'&&(/protocol::Inference</.test(i.head)||/^impl Default/.test(i.head)));
 for(const item of [...selected].sort((a,b)=>b.start-a.start))source=source.slice(0,item.start)+source.slice(item.end);
 source=source.replace('use super::entries::compute_energy_model_entries;','');writeFileSync(file,source);
 writeFileSync(io,readFileSync(io,'utf8')+'\nmod energy_inference_adapter {\nuse super::*;\nuse crate::EnergyModelSnapshot;\nuse crate::standards::v1::subsets::any::schema::inferences::EnergyModelInference;\n'+selected.map(i=>i.code).join('\n')+'\n}\n');
 console.log('[DEBUG] Energy wire census adapters moved '+selected.length+' impls');
}
if(process.argv[2]==='file-report'){
 const paths=new Set<string>();
 const record=(value:any)=>{if(typeof value==='string'&&(value.startsWith(repo+'/✏️s/')||value.startsWith('✏️s/'))){const path=resolve(repo,value);if(/\.(?:rs|ts|json|proto|graphql|semio)$/.test(path))paths.add(path);else if(existsSync(path)&&statSync(path).isDirectory())for(const file of execFileSync('rg',['--files',path],{encoding:'utf8',maxBuffer:16*1024*1024}).trim().split('\n'))if(file)paths.add(file);}else if(Array.isArray(value))value.forEach(record);else if(value&&typeof value==='object')Object.values(value).forEach(record);};
 for(const name of readdirSync(join(ticket,'🗑️generated')).filter(name=>/^runtime.*\.json$/.test(name))){try{record(JSON.parse(readFileSync(join(ticket,'🗑️generated',name),'utf8')));}catch{}}
 for(const name of ['🖍️draw','🀄️wfc'])for(const file of execFileSync('rg',['--files','✏️s/🔌️plugins/'+name],{encoding:'utf8'}).trim().split('\n'))if(/\/🏠️host\/|\/🚪️io\/📝️text\/🔺️diff\/|\/🧫️fixtures\/.*\/🔺️diff\/🔣️\.json$/.test(file))paths.add(resolve(repo,file));
 const lines=[...paths].sort().map(path=>'- '+(existsSync(path)?'Present':'Removed')+': ['+relative(repo,path)+']('+path+')');
 writeFileSync(join(ticket,'runtime-files.md'),'# Recorded Runtime Extraction Files\n\nThis ownership-scope list consolidates extraction/control/native/host records before generated logs are cleaned. Artifact root files are shared with other agents; their presence here does not attribute every edit solely to this task.\n\n'+lines.join('\n')+'\n');
 console.log('[DEBUG] Runtime source paths recorded '+paths.size);
}
const inventory = ['repair-mounts', 'consumers'].includes(process.argv[2]) ? JSON.parse(readFileSync(join(ticket, '🗑️generated/runtime-inventory.json'), 'utf8')) : roots.map(file => {
  const source = readFileSync(file, 'utf8'), all = items(source);
  const codecs = all.filter(i => /\bimpl\s+(?:(?:protocol|store)::)?(?:Op(?:Text|Binary)|Artifact(?:Dsl|Pack))\b/.test(i.head) || wireFunction(i,file) || /^COMPONENT_(?:GRAMMAR|PROTOCOL)/.test(i.name));
  return { file: relative(repo, file), modules: sourceModules.get(file), items: codecs.map(i => ({ name: i.name, head: i.head, bytes: i.code.length })) };
}).filter(row => row.items.length);
mkdirSync(join(ticket, '🗑️generated'), { recursive: true });
writeFileSync(join(ticket, '🗑️generated/runtime-inventory.json'), JSON.stringify(inventory, null, 2));
console.log(JSON.stringify({ roots: inventory.length, items: inventory.reduce((n, row) => n + row.items.length, 0), unmapped: inventory.filter(row => !row.modules).map(row => row.file) }));

if (process.argv[2] === 'pack-mounts') {
  const moved = JSON.parse(readFileSync(join(ticket, '🗑️generated/runtime-pack-modules.json'), 'utf8'));
  for (const row of moved) for (const file of files.filter(file => file.startsWith(dirname(row.root)) && /\/🚪️io\/💾️binary\/📸️snapshot\/📦️pack\/🦀️\.rs$/.test(file))) {
    const mount = join(dirname(dirname(file)), '🦀️.rs'), source = readFileSync(mount, 'utf8');
    if (!new RegExp('mod ' + row.name + '\\b').test(source)) writeFileSync(mount, source + '\n#[path = "📦️pack/🦀️.rs"]\npub(crate) mod ' + row.name + ';\n');
  }
  console.log(JSON.stringify({ repaired: moved.length }));
}

if (['extract', 'repair-mounts'].includes(process.argv[2])) {
  const additions = new Map<string, string[]>(), moved: { root: string, semantic: string, io: string, names: string[] }[] = [];
  const moduleAdditions = new Map<string, { end: number, text: string }[]>();
  const scopes = new Map<string, { module: string, root: string, io: string }>();
  for (const row of inventory) {
    const file = resolve(repo, row.file), source = readFileSync(file, 'utf8'), all = items(source);
    const owner = row.modules![0], semantic = owner.module.replace(/::component$/, '');
    const kind = file.match(/\/🧬️schema\/(🧬️mutations|📸️snapshot|🔺️diff|💡️inferences)\//)?.[1] ?? (file.includes('/⚙️operations/') ? '🧬️mutations' : '📸️snapshot');
    const kindName = ({ '🧬️mutations': 'mutations', '📸️snapshot': 'snapshot', '🔺️diff': 'diff', '💡️inferences': 'inferences' } as Record<string, string>)[kind];
    const physicalScope = file.slice(0, file.lastIndexOf('/🧬️schema/'));
    const scopeModule = semantic === kindName ? '' : semantic.includes('::schema') ? semantic.slice(0, semantic.lastIndexOf('::schema')) : semantic.endsWith('::' + kindName) ? semantic.slice(0, semantic.lastIndexOf('::' + kindName)) : semantic;
    const ioBase = join(physicalScope, '🚪️io'), ioModule = [scopeModule, 'io'].filter(Boolean).join('::');
    scopes.set(physicalScope, { module: scopeModule, root: owner.root, io: ioBase });
    const chosen = new Map<Item, string>();
    for (const item of all) {
      if (/\bimpl\s+(?:(?:protocol|store)::)?(?:OpText|ArtifactDsl)\b/.test(item.head)) chosen.set(item, 'text/' + kindName);
      if (/\bimpl\s+(?:(?:protocol|store)::)?(?:OpBinary|ArtifactPack)\b/.test(item.head)) chosen.set(item, 'binary/' + kindName);
      if (wireFunction(item,file)) chosen.set(item, (/(?:\[u8\]|Vec<u8>|ByteReader)/.test(item.head)&&!/^hex_/.test(item.name)?'binary/':'text/') + (/_snapshot|_(?:[a-z0-9]+_)?dsl$/.test(item.name) && !/_mutation/.test(item.name) ? 'snapshot' : kindName));
    }
    for (const rep of ['text', 'binary']) {
      const representation = join(ioBase, rep === 'text' ? '📝️text' : '💾️binary', kind, '🦀️.rs');
      if (!existsSync(representation)) continue; const current = mask(readFileSync(representation, 'utf8'));
      for (const item of all) if (item.name && !/^pub\s/.test(item.head) && ['fn', 'struct', 'enum'].includes(item.kind) && new RegExp('\\b' + item.name + '\\b').test(current)) chosen.set(item, rep + '/' + kindName);
    }
    let changed = true;
    while (changed) {
      changed = false;
      for (const [entry, target] of [...chosen]) for (const item of all) {
        if (chosen.has(item) || !item.name || !new RegExp('\\b' + item.name + '\\b').test(mask(entry.code))) continue;
        if (item.kind === 'fn' && !/^(apply|inverse|agg_|diff_|restore_|fixture|base_snapshot|demo_|sweep_|node_at|edge_at|param_value_at)/.test(item.name) || ['struct', 'enum'].includes(item.kind) && (!/^pub\s/.test(item.head) || /(?:Dsl|JsonDsl|PackRecord)$/.test(item.name))) { chosen.set(item, target); changed = true; }
      }
      for (const [entry, target] of [...chosen]) if (['struct', 'enum'].includes(entry.kind)) for (const item of all) if (!chosen.has(item) && item.kind === 'impl' && new RegExp('^impl(?:<[^>]+>)?\\s+' + entry.name + '\\s*[{<]').test(item.head)) { chosen.set(item, target); changed = true; }
    }
    const semanticParent = owner.module.split('::').slice(0, -1).join('::');
    const imports = all.filter(item => item.kind === 'use').map(item => item.code.replace(/\bpub(?:\([^)]*\))?\s+use/g, 'use').replace(/\bsuper::/g, 'crate::' + semanticParent + '::')).join('\n');
    const groups = new Map<string, Item[]>();
    for (const [entry, target] of chosen) { const group = groups.get(target) ?? []; group.push(entry); groups.set(target, group); }
    for (const [target, entries] of groups) {
      const [rep, facet] = target.split('/'), repEmoji = rep === 'text' ? '📝️text' : '💾️binary';
      const facetEmoji = ({ mutations: '🧬️mutations', snapshot: '📸️snapshot', diff: '🔺️diff', inferences: '💡️inferences' } as Record<string, string>)[facet];
      const destination = join(ioBase, repEmoji, facetEmoji, '🦀️.rs');
      const codecPath = ioModule + '::' + rep + '::' + facet;
      const body = entries.sort((a, b) => a.start - b.start).map(entry => {
        let code = entry.code.replace(/\bsuper::/g, 'crate::' + semanticParent + '::');
        if (entry.kind === 'fn' && !/^pub/.test(entry.head)) code = code.replace(/^fn /m, 'pub(crate) fn ');
        if (['struct', 'enum'].includes(entry.kind) && !/^pub/.test(entry.head)) code = code.replace(new RegExp('^' + entry.kind + ' ', 'm'), 'pub(crate) ' + entry.kind + ' ');
        if (entry.kind === 'impl' && !/\bfor\b/.test(entry.head)) code = code.replace(/^(\s+)fn /gm, '$1pub(crate) fn ');
        code = code.replace(/include_str!\("(?:💾️binary\/)?📡️\.protocol\.semio"\)/g, 'crate::' + ioModule + '::binary::' + kindName + '::COMPONENT_PROTOCOL_SEMIO');
        return code;
      }).join('\n\n');
      const siblings = [...groups.entries()].filter(([key]) => key !== target).flatMap(([key, others]) => {
        const names = others.filter(item => item.name && new RegExp('\\b' + item.name + '\\b').test(mask(body))).map(item => item.name);
        return names.length ? ['use crate::' + ioModule + '::' + key.replace('/', '::') + '::{' + names.join(', ') + '};'] : [];
      }).join('\n');
      const existing = (existsSync(destination) ? readFileSync(destination, 'utf8') : '')+(additions.get(destination)??[]).join('\n');
      let codecName=kindName+'_codec',iteration=0;
      while(existing.includes('mod '+codecName)){iteration++;codecName=kindName+'_wire'+(iteration>1?iteration:'')+'_codec';}
      const block = `\n#[allow(unused_imports)]\nmod ${codecName} {\nuse crate::${semantic}::*;\n${imports}\n${siblings}\n${body}\n}\npub use ${codecName}::*;\n`;
      const old = additions.get(destination) ?? []; old.push(block); additions.set(destination, old);
      moved.push({ root: owner.root, semantic, io: codecPath, names: entries.filter(entry => entry.kind === 'fn' && /^pub/.test(entry.head)).map(entry => entry.name) });
    }
    const ranges = [...chosen.keys()].sort((a, b) => b.start - a.start);
    let output = source;
    for (const item of ranges) output = output.slice(0, item.start) + output.slice(item.end);
    if (output !== source) writeFileSync(file, output);
  }
  for (const [destination, blocks] of additions) { mkdirSync(dirname(destination), { recursive: true }); const old = existsSync(destination) ? readFileSync(destination, 'utf8') : '//! 🚪️ Artifact representation codecs.\n'; writeFileSync(destination, old + blocks.join('\n')); }
  for (const [physicalScope, scope] of scopes) {
    const rootFile = join(scope.io, '🦀️.rs');
    if (!existsSync(rootFile)) {
      mkdirSync(scope.io, { recursive: true }); writeFileSync(rootFile, '//! 🚪️ Artifact representation boundary.\n#[path = "💾️binary/🦀️.rs"]\npub mod binary;\n#[path = "📝️text/🦀️.rs"]\npub mod text;\n');
    }
    if (!contexts.some(entry => entry.root === scope.root && entry.module === [scope.module, 'io'].filter(Boolean).join('::')) && ![...sourceModules.values()].flat().some(entry => entry.root === scope.root && entry.module === [scope.module, 'io'].filter(Boolean).join('::'))) {
      const context = contexts.find(entry => entry.root === scope.root && entry.module === scope.module);
      if (!context) throw new Error('Missing mount ' + physicalScope + ': ' + scope.module);
      const list = moduleAdditions.get(context.file) ?? [];
      list.push({ end: context.end, text: '\n#[path = ' + JSON.stringify(relative(context.base, rootFile)) + ']\npub mod io;\n' }); moduleAdditions.set(context.file, list);
    }
    for (const [rep, emoji] of [['binary', '💾️binary'], ['text', '📝️text']]) {
      const file = join(scope.io, emoji, '🦀️.rs'); mkdirSync(dirname(file), { recursive: true });
      let source = existsSync(file) ? readFileSync(file, 'utf8') : '//! 🚪️ Artifact ' + rep + ' representations.\n';
      for (const [facet, name] of [['🧬️mutations', 'mutations'], ['📸️snapshot', 'snapshot'], ['🔺️diff', 'diff'], ['💡️inferences', 'inferences']]) if (existsSync(join(dirname(file), facet, '🦀️.rs')) && !new RegExp('pub mod ' + name + '\\b').test(source)) source += '#[path = "' + facet + '/🦀️.rs"]\npub mod ' + name + ';\n';
      writeFileSync(file, source);
    }
  }
  for (const [file, injections] of moduleAdditions) { let source = readFileSync(file, 'utf8'); for (const injection of injections.sort((a, b) => b.end - a.end)) source = source.slice(0, injection.end) + injection.text + source.slice(injection.end); writeFileSync(file, source); }
  writeFileSync(join(ticket, '🗑️generated/runtime-moved.json'), JSON.stringify(moved, null, 2));
  console.log(JSON.stringify({ destinations: additions.size, mounts: moduleAdditions.size }));
}

if (process.argv[2] === 'host') {
  const artifact = resolve(repo, '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d');
  const ioBase = join(artifact, '🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io');
  const binary = join(ioBase, '💾️binary/🧬️mutations/🦀️.rs'), text = join(ioBase, '📝️text/🧬️mutations/🦀️.rs');
  const host = join(artifact, '🔨️modules/🏠️host/🦀️.rs');
  const original = readFileSync(binary, 'utf8');
  const begin = original.indexOf('//#region 🔖️RetainedMountedIngress');
  const terminal = 'mod retained_authority_laws;';
  const end = original.indexOf(terminal, begin) + terminal.length;
  if (begin < 0 || end < terminal.length) throw new Error('Missing host authority bounds');
  const imports = items(original).filter(item => item.kind === 'use').map(item => item.code.replace('::io::text::mutations::Generation3dMutation', '::schema::mutations::Generation3dMutation')).join('\n');
  mkdirSync(dirname(host), { recursive: true });
  const hostBody = original.slice(begin, end).replace(/include_str!\("🧫️fixtures\/🧬️semantic-wire\/🔣️\.json"\)/g, 'include_str!(' + JSON.stringify(relative(dirname(host), join(dirname(binary), '🧫️fixtures/🧬️semantic-wire/🔣️.json'))) + ')');
  writeFileSync(host, '//! 🏠️ Generation3d publication, retained replay and store ownership.\n#![allow(unused_imports)]\n' + imports + '\n' + hostBody + '\n');
  const laws = join(dirname(binary), '🧪️tests/🔬️retained-authority-laws');
  const destination = join(dirname(host), '🧪️tests/🔬️retained-authority-laws');
  mkdirSync(dirname(destination), { recursive: true }); renameSync(laws, destination);
  let codec = original.slice(0, begin) + original.slice(end);
  const lifted = items(codec).filter(item => item.name === 'Generation3dOperationDsl' || /^generation3d_operation_(to|from)_dsl$/.test(item.name) || /^impl protocol::OpText for Generation3d/.test(item.head));
  const textBody = lifted.map(item => item.code.replace(/^enum /m, 'pub(crate) enum ').replace(/^fn /m, 'pub(crate) fn ')).join('\n\n');
  for (const item of lifted.sort((a, b) => b.start - a.start)) codec = codec.slice(0, item.start) + codec.slice(item.end);
  codec = codec.replace('::io::text::mutations::Generation3dMutation', '::schema::mutations::Generation3dMutation');
  codec += '\nuse crate::standards::v1::subsets::any::io::text::mutations::{Generation3dOperationDsl, generation3d_operation_to_dsl, generation3d_operation_from_dsl};\n';
  writeFileSync(binary, codec);
  writeFileSync(text, readFileSync(text, 'utf8') + '\n#[allow(unused_imports)]\nmod operation_codec {\n' + imports + '\n' + textBody + '\n}\npub use operation_codec::*;\n');
  const root = join(artifact, '🦀️.rs');
  writeFileSync(root, readFileSync(root, 'utf8') + '\n#[path = "🔨️modules/🏠️host/🦀️.rs"]\npub mod host;\n');
  const names = items(hostBody).filter(item => /^pub/.test(item.head) && item.name).map(item => item.name);
  for (const file of files) {
    let source = readFileSync(file, 'utf8'); const before = source;
    for (const name of names) source = source.replace(new RegExp('((?:crate|semio_s_artifact_procedural_generation3d)::)(?:standards::v1::subsets::any::)?io::binary::mutations::' + name, 'g'), '$1host::' + name);
    source = source.replace(/(use\s+)((?:crate|semio_s_artifact_procedural_generation3d)::)(?:standards::v1::subsets::any::)?io::binary::mutations::\{([^}]+)\};/g, (whole, use, prefix, members) => {
      const parts = members.split(',').map((part: string) => part.trim()).filter(Boolean), hosts = parts.filter((part: string) => names.includes(part.split(' ')[0])), kept = parts.filter((part: string) => !names.includes(part.split(' ')[0]));
      return (kept.length ? use + prefix + 'standards::v1::subsets::any::io::binary::mutations::{' + kept.join(', ') + '};\n' : '') + (hosts.length ? use + prefix + 'host::{' + hosts.join(', ') + '};' : '');
    });
    if (source !== before) writeFileSync(file, source);
  }
  console.log(JSON.stringify({ host, movedHostBytes: hostBody.length, textCodecBytes: textBody.length, publicHostNames: names }));
}

if (process.argv[2] === 'consumers') {
  const helpers: { name: string, root: string, source: string, semantic:string, io: string, private?: boolean }[] = [];
  const crateNames=new Map<string,string>();
  const crateName=(root:string)=>{let name=crateNames.get(root);if(name)return name;const manifest=join(dirname(root),'📦️packages/🦀️rust/Cargo.toml');name=existsSync(manifest)?readFileSync(manifest,'utf8').match(/^name\s*=\s*"([^"]+)"/m)?.[1].replaceAll('-','_'):undefined;if(!name)throw new Error('No crate identity '+root);crateNames.set(root,name);return name;};
  for (const file of files.filter(file => /\/🚪️io\/(?:📝️text|💾️binary)\/(?:🧬️mutations|📸️snapshot|💡️inferences)\/🦀️\.rs$/.test(file))) {
    const source = readFileSync(file, 'utf8'), clean = mask(source);
    for (const block of clean.matchAll(/mod (?:native_)?(mutations|snapshot|diff|inferences)(?:_wire\d*)?_codec\s*\{/g)) {
      const begin = block.index! + block[0].length; let end = begin, level = 1;
      while (end < clean.length && level) { if (clean[end] === '{') level++; else if (clean[end] === '}') level--; end++; }
      const body = source.slice(begin, end - 1), semantic = body.match(/use crate::([\w:]+)::\*;/)?.[1];
      if (!semantic) continue;
      const kind = ({ mutations: '🧬️mutations', snapshot: '📸️snapshot', diff: '🔺️diff', inferences: '💡️inferences' } as Record<string, string>)[block[1]];
      const physical = file.slice(0, file.lastIndexOf('/🚪️io/'));
      const semanticFile = [...sourceModules.entries()].find(([candidate,owners]) => candidate.startsWith(physical+'/🧬️schema/')&&owners.some(owner=>owner.module.replace(/::component$/,'')===semantic))?.[0];
      if (!semanticFile) continue;
      const owner = sourceModules.get(semanticFile)?.[0]; if (!owner) continue;
      const scope = semantic === block[1] ? '' : semantic.includes('::schema') ? semantic.slice(0, semantic.lastIndexOf('::schema')) : semantic.endsWith('::'+block[1]) ? semantic.slice(0, semantic.lastIndexOf('::' + block[1])) : semantic;
      const parts = file.match(/\/(📝️text|💾️binary)\/([^/]+)\/🦀️\.rs$/)!, name = ({ '🧬️mutations': 'mutations', '📸️snapshot': 'snapshot', '🔺️diff': 'diff', '💡️inferences': 'inferences' } as Record<string, string>)[parts[2]];
      for (const entry of items(body)) if (entry.name && /\bpub(?:\([^)]*\))? fn/.test(entry.head)) helpers.push({ name: entry.name, root: owner.root, source: semanticFile, semantic, io: [scope, 'io', parts[1]==='📝️text'?'text':'binary', name].filter(Boolean).join('::'), private: entry.head.startsWith('pub(crate)') });
    }
  }
  const allFiles = execFileSync('rg', ['--files', '-g', '*.rs'], { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 }).trim().split('\n');
  let changed = 0;
  for (const relativeFile of allFiles) {
    const file = resolve(repo, relativeFile); if (!existsSync(file)||file.includes('/🔺️diff/')) continue; let source = readFileSync(file, 'utf8'), before = source;
    for (const helper of helpers) {
      if (!source.includes(helper.name) || helper.private && !file.startsWith(dirname(helper.root))) continue;
      source = source.replace(/(^[ \t]*)(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);/gm, (whole, indent, visibility, body) => {
        if (!new RegExp('\\b' + helper.name + '\\b').test(body)) return whole;
        const externalOwner=body.match(/^(semio_[a-z0-9_]+)::/)?.[1];
        if(externalOwner?externalOwner!==crateName(helper.root):!file.startsWith(dirname(helper.root)))return whole;
        const open = body.indexOf('::{');
        if (open >= 0 && body.endsWith('}') && !body.slice(open + 3, -1).includes('{')) {
          const path = body.slice(0, open), members = body.slice(open + 3, -1).split(',').map((part: string) => part.trim()).filter(Boolean);
          const moved = members.filter((part: string) => part.split(' ')[0] === helper.name), kept = members.filter((part: string) => part.split(' ')[0] !== helper.name);
          if (!moved.length) return whole;
          const external = path.match(/^(semio_[a-z0-9_]+)::/)?.[1];
          const destination = external ? external + '::' + helper.io : 'crate::' + helper.io;
          return (kept.length ? indent + (visibility ?? '') + 'use ' + path + '::{' + kept.join(', ') + '};\n' : '') + indent + (visibility ?? '') + 'use ' + destination + '::{' + moved.join(', ') + '};';
        }
        if (body.endsWith('::' + helper.name)) { const external = body.match(/^(semio_[a-z0-9_]+)::/)?.[1]; return indent + (visibility ?? '') + 'use ' + (external ?? 'crate') + '::' + helper.io + '::' + helper.name + ';'; }
        return whole;
      });
      source = source.replace(new RegExp('\\b((?:crate|semio_[a-z0-9_]+)::)(?:[\\w:]*::(?:mutations|snapshot|schema)::)?' + helper.name + '\\b', 'g'),(whole,prefix)=>{const external=prefix.slice(0,-2);if(external==='crate'?!file.startsWith(dirname(helper.root)):external!==crateName(helper.root))return whole;return prefix+helper.io+'::'+helper.name;});
      if (file.startsWith(dirname(helper.source) + '/🧪️tests/') && new RegExp('\\b' + helper.name + '\\b').test(mask(source)) && !new RegExp('use[^;]*\\b' + helper.name + '\\b').test(source)) source = 'use crate::' + helper.io + '::' + helper.name + ';\n' + source;
    }
    if (file.includes('/📖️playbook/🧩️extensions/🌀️procedural/🚪️io/')) source = source.replace(/crate::mutation::io/g, 'crate::io');
    if (source !== before) { writeFileSync(file, source); changed++; }
  }
  console.log(JSON.stringify({ helpers: helpers.length, consumers: changed }));
}

if (process.argv[2] === 'pack') {
  const movedModules: { root: string, old: string, next: string, sourceParent: string, name: string }[] = [];
  for (const file of files.filter(file => /\/🧬️schema\/📸️snapshot\/📦️pack\/🦀️\.rs$/.test(file))) {
    const owner = sourceModules.get(file)?.[0]; if (!owner) throw new Error('Unmapped pack ' + file);
    const name = owner.module.split('::').at(-1)!;
    const parent = owner.module.split('::').slice(0, -1).join('::').replace(/::component$/, '');
    const scope = parent.slice(0, parent.lastIndexOf('::schema::'));
    const physical = file.slice(0, file.lastIndexOf('/🧬️schema/'));
    const binary = join(physical, '🚪️io/💾️binary/📸️snapshot/🦀️.rs'), text = join(physical, '🚪️io/📝️text/📸️snapshot/🦀️.rs');
    const destination = join(dirname(binary), '📦️pack');
    let source = readFileSync(file, 'utf8');
    const textImpls = items(source).filter(item => /^impl store::ArtifactDsl\b/.test(item.head));
    const originalImports = items(source).filter(item => item.kind === 'use').map(item => item.code.replace(/\bsuper::/g, 'crate::' + parent + '::')).join('\n');
    const textCode = textImpls.map(item => item.code.replace(/\bsuper::/g, 'crate::' + parent + '::')).join('\n');
    for (const item of textImpls.sort((a, b) => b.start - a.start)) source = source.slice(0, item.start) + source.slice(item.end);
    source = source.replace(/\bsuper::/g, 'crate::' + parent + '::').replace(/pub\(super\)/g, 'pub(crate)');
    const impls = items(source);
    for (const item of impls.sort((a, b) => b.start - a.start)) {
      let replacement = item.code;
      if (['struct', 'enum'].includes(item.kind) && !/^pub/.test(item.head)) replacement = replacement.replace(new RegExp('^' + item.kind + '\\s*', 'm'), 'pub(crate) ' + item.kind + ' ');
      if (item.kind === 'impl' && !/\bfor\b/.test(item.head)) replacement = replacement.replace(/^(\s*)fn\s*/gm, '$1pub(crate) fn ');
      source = source.slice(0, item.start) + replacement + source.slice(item.end);
    }
    writeFileSync(file, source);
    mkdirSync(dirname(destination), { recursive: true }); renameSync(dirname(file), destination);
    writeFileSync(binary, readFileSync(binary, 'utf8') + '\n#[path = "📦️pack/🦀️.rs"]\npub(crate) mod ' + name + ';\n');
    if (textCode) writeFileSync(text, readFileSync(text, 'utf8') + '\n#[allow(unused_imports)]\nmod pack_codec {\n' + originalImports + '\nuse crate::' + scope + '::io::binary::snapshot::' + name + '::*;\n' + textCode + '\n}\n');
    movedModules.push({ root: owner.root, old: owner.module, next: scope + '::io::binary::snapshot::' + name, sourceParent: parent, name });
  }
  const currentFiles = execFileSync('rg', ['--files', '-g', '*.rs', '✏️s/🔌️plugins'], { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 }).trim().split('\n');
  let consumers = 0;
  for (const relativeFile of currentFiles) {
    const file = resolve(repo, relativeFile); if (!existsSync(file)) continue; let source = readFileSync(file, 'utf8'), before = source;
    for (const moved of movedModules) {
      if (!file.startsWith(dirname(moved.root))) continue;
      if (!file.includes('/🚪️io/')) source = source.replace(new RegExp('#\\[path\\s*=\\s*"[^"]*📦️pack/🦀️\\.rs"\\]\\s*(?:pub(?:\\([^)]*\\))?\\s*)?mod\\s+' + moved.name + '\\s*;', 'g'), '');
      source = source.replaceAll(moved.old + '::', moved.next + '::');
      if (file.includes('/🧬️schema/📸️snapshot/')) source = source.replace(new RegExp('(?<![\\w:])' + moved.name + '::', 'g'), 'crate::' + moved.next + '::');
    }
    if (source !== before) { writeFileSync(file, source); consumers++; }
  }
  writeFileSync(join(ticket, '🗑️generated/runtime-pack-modules.json'), JSON.stringify(movedModules, null, 2));
  console.log(JSON.stringify({ packOwners: movedModules.length, consumers }));
}

if (process.argv[2] === 'trinity') {
  const artifact = resolve(repo, '✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack');
  const schema = join(artifact, '🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime');
  const io = join(artifact, '🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io');
  const binary = join(io, '💾️binary/🧬️mutations/🦀️.rs'), text = join(io, '📝️text/🧬️mutations/🦀️.rs');
  const host = join(artifact, '🔨️modules/🏠️host');
  const original = readFileSync(join(schema, '🦀️.rs'), 'utf8'), split = original.indexOf('//#region 🔖️OwnedSprCatalog');
  const prefix = original.slice(0, split), hostBody = original.slice(split);
  const imports = items(prefix).filter(item => item.kind === 'use').map(item => item.code.replace('::io::text::mutations::TrinityGraphMutation', '::schema::mutations::TrinityGraphMutation')).join('\n');
  const textItems = items(prefix).filter(item => item.name === 'TrinityGraphOperationDsl' || /^trinity_graph_operation_(to|from)_dsl$/.test(item.name) || /^impl OpText for/.test(item.head));
  const textBody = textItems.map(item => item.code.replace(/^enum /m, 'pub(crate) enum ').replace(/^fn /m, 'pub(crate) fn ')).join('\n');
  let binaryBody = prefix;
  for (const item of textItems.sort((a, b) => b.start - a.start)) binaryBody = binaryBody.slice(0, item.start) + binaryBody.slice(item.end);
  binaryBody = binaryBody.replace('include_str!("../../🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio")', 'include_str!("📡️.protocol.semio")').replace('::io::text::mutations::TrinityGraphMutation', '::schema::mutations::TrinityGraphMutation');
  mkdirSync(dirname(host), { recursive: true }); renameSync(schema, host);
  writeFileSync(join(host, '🦀️.rs'), '//! 🏠️ Jack retained publication, replay and document host ownership.\n#![allow(unused_imports)]\n' + imports + '\n#[cfg(test)]\nuse crate::standards::v1::subsets::any::io::binary::mutations::{encode_op, decode_op};\n' + hostBody);
  writeFileSync(text, readFileSync(text, 'utf8') + '\n#[allow(unused_imports)]\nmod operation_codec {\n' + imports + '\n' + textBody + '\n}\npub use operation_codec::*;\n');
  writeFileSync(binary, readFileSync(binary, 'utf8').replace(/pub use crate::standards::v1::subsets::any::schema::wire_runtime::\*;\s*/, '') + '\n' + binaryBody + '\nuse crate::standards::v1::subsets::any::io::text::mutations::{TrinityGraphOperationDsl, trinity_graph_operation_to_dsl, trinity_graph_operation_from_dsl};\n');
  const root = join(artifact, '🦀️.rs');
  let rootSource = readFileSync(root, 'utf8').replace(/#\[path\s*=\s*"\."\]\s*pub mod wire_runtime\s*\{[^}]+\}/g, '');
  rootSource += '\n#[path = "🔨️modules/🏠️host/🦀️.rs"]\npub mod host;\n'; writeFileSync(root, rootSource);
  const currentFiles = execFileSync('rg', ['--files', '-g', '*.rs', relative(repo, artifact)], { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 }).trim().split('\n');
  for (const relativeFile of currentFiles) {
    const file = resolve(repo, relativeFile); if (!existsSync(file)) continue; const source = readFileSync(file, 'utf8'), output = source.replace(/(?:standards::v1::subsets::any::)?schema::wire_runtime::/g, 'host::'); if (output !== source) writeFileSync(file, output);
  }
  console.log(JSON.stringify({ hostBytes: hostBody.length, textBytes: textBody.length, binaryBytes: binaryBody.length }));
}

if (process.argv[2] === 'codec-links') {
  let count = 0;
  for (const file of files.filter(file => file.includes('/🚪️io/'))) {
    if (!existsSync(file)) continue; const before = readFileSync(file, 'utf8');
    const source = before.replace(/(mod (?:mutations|snapshot|inferences)(?:_wire\d*)?_codec\s*\{)\n(?!use super::\*;)/g, '$1\nuse super::*;\n');
    if (source !== before) { writeFileSync(file, source); count++; }
  }
  console.log(JSON.stringify({ codecLinks: count }));
}

if (process.argv[2] === 'shared-helpers') {
  let restored = 0;
  for (const file of files.filter(file => /\/🚪️io\/(?:📝️text|💾️binary)\/(?:🧬️mutations|📸️snapshot|🔺️diff|💡️inferences)\/🦀️\.rs$/.test(file))) {
    let source = readFileSync(file, 'utf8');
    const clean = mask(source), removals: { start: number, end: number }[] = [];
    for (const block of clean.matchAll(/mod (?:mutations|snapshot|diff|inferences)(?:_wire)?_codec\s*\{/g)) {
      const begin = block.index! + block[0].length; let end = begin, level = 1;
      while (end < clean.length && level) { if (clean[end] === '{') level++; else if (clean[end] === '}') level--; end++; }
      const body = source.slice(begin, end - 1), semantic = body.match(/use crate::([\w:]+)::\*;/)?.[1]; if (!semantic) continue;
      const semanticFile = [...sourceModules.entries()].find(([candidate, owners]) => candidate.includes('/🧬️schema/') && owners.some(owner => owner.module.replace(/::component$/, '') === semantic))?.[0];
      if (!semanticFile) continue;
      let semanticSource = readFileSync(semanticFile, 'utf8');
      const helpers = items(body).filter(item => item.kind === 'fn' && item.head.startsWith('pub(crate)'));
      const selected = new Set<Item>(); let changed = true;
      while (changed) {
        changed = false;
        const semanticCode = mask(semanticSource.replace(/^\s*(?:pub(?:\([^)]*\))?\s+)?use[^;]+;/gm, ''));
        for (const helper of helpers) if (!selected.has(helper) && new RegExp('\\b' + helper.name + '\\b').test(semanticCode) && !new RegExp('\\bfn ' + helper.name + '\\b').test(semanticCode)) {
          if (/semio_framework_(?:pack|dsl_record)::|store::pack_rt::/.test(mask(helper.code))) continue;
          semanticSource += '\n' + helper.code + '\n'; selected.add(helper); changed = true; restored++;
        }
      }
      if (selected.size) { writeFileSync(semanticFile, semanticSource); for (const helper of selected) removals.push({ start: begin + helper.start, end: begin + helper.end }); }
    }
    for (const range of removals.sort((a, b) => b.start - a.start)) source = source.slice(0, range.start) + source.slice(range.end);
    if (removals.length) writeFileSync(file, source);
  }
  console.log(JSON.stringify({ restoredSemanticHelpers: restored }));
}

if (process.argv[2] === 'geojson') {
  const subset = resolve(repo, '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson');
  const schema = join(subset, '🧬️schema/🦀️.rs'), sqlite = join(subset, '🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs');
  const semantic = join(subset, '🧬️schema/🛡️conformance/🦀️.rs');
  const original = readFileSync(sqlite, 'utf8');
  let algorithm = original.replace(/use semio_framework_os_kernel::sqlite_snapshot::\{[^}]+\};/, 'use semio_framework_value::{ValueError, ValueRefusalKind};');
  algorithm = algorithm.replace("struct Gate<'a,'b>{control:&'a mut SqliteSnapshotControl<'b>", "struct Gate<'a,C:GeoJsonConformanceControl>{control:&'a mut C").replace("impl Gate<'_,'_>", "impl<C:GeoJsonConformanceControl> Gate<'_,C>");
  algorithm = algorithm.replaceAll('checkpoint(SqliteSnapshotPhase::ProjectSnapshot,', 'checkpoint(').replaceAll('.limits().max_rows', '.maximum_rows()');
  algorithm = algorithm.replace("pub fn check_geojson_conformance_controlled(snapshot:&JsonSnapshot,control:&mut SqliteSnapshotControl<'_>)", 'pub fn check_geojson_conformance_with_control<C:GeoJsonConformanceControl>(snapshot:&JsonSnapshot,control:&mut C)');
  algorithm = algorithm.replace('crate::schema::snapshot::number::meaning(lexeme,gate.control,SqliteSnapshotPhase::ProjectSnapshot,gate.steps,0)?.valid', 'gate.control.number_valid(lexeme,gate.steps)?');
  const contract = `\n/// 🛡️ Neutral work and lexical-validation authority for semantic GeoJSON conformance.\npub trait GeoJsonConformanceControl {\nfn checkpoint(&mut self, visited:usize, total:usize)->std::result::Result<(),ValueError>;\nfn maximum_rows(&self)->usize;\nfn number_valid(&mut self, lexeme:&str, visited:usize)->std::result::Result<bool,ValueError>;\n}\n\npub(super) struct UnboundedGeoJsonConformance;\nimpl GeoJsonConformanceControl for UnboundedGeoJsonConformance {\nfn checkpoint(&mut self,_:usize,_:usize)->std::result::Result<(),ValueError>{Ok(())}\nfn maximum_rows(&self)->usize{usize::MAX}\nfn number_valid(&mut self,lexeme:&str,_:usize)->std::result::Result<bool,ValueError>{Ok(valid_number_lexeme(lexeme))}\n}\n\nfn valid_number_lexeme(lexeme:&str)->bool {\nlet bytes=lexeme.as_bytes(); let mut cursor=usize::from(bytes.first()==Some(&b'-'));\nmatch bytes.get(cursor){Some(b'0')=>cursor+=1,Some(b'1'..=b'9')=>{cursor+=1;while bytes.get(cursor).is_some_and(u8::is_ascii_digit){cursor+=1;}},_=>return false}\nif bytes.get(cursor)==Some(&b'.'){cursor+=1;let start=cursor;while bytes.get(cursor).is_some_and(u8::is_ascii_digit){cursor+=1;}if cursor==start{return false}}\nif matches!(bytes.get(cursor),Some(b'e'|b'E')){cursor+=1;if matches!(bytes.get(cursor),Some(b'+'|b'-')){cursor+=1;}let start=cursor;while bytes.get(cursor).is_some_and(u8::is_ascii_digit){cursor+=1;}if cursor==start{return false}}\ncursor==bytes.len()\n}\n`;
  mkdirSync(dirname(semantic), { recursive: true }); writeFileSync(semantic, algorithm + contract);
  const adapter = `//! 🪶️ SQLite work authority adapts to neutral GeoJSON semantic conformance.\nuse crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;\nuse crate::standards::v_rfc8259::subsets::geojson::schema::{GeoJsonConformanceControl,check_geojson_conformance_with_control};\nuse semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,ValueError};\nstruct SqliteGeoJsonConformance<'a,'b>{control:&'a mut SqliteSnapshotControl<'b>}\nimpl GeoJsonConformanceControl for SqliteGeoJsonConformance<'_,'_>{\nfn checkpoint(&mut self,visited:usize,total:usize)->Result<(),ValueError>{self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,visited,total)}\nfn maximum_rows(&self)->usize{self.control.limits().max_rows}\nfn number_valid(&mut self,lexeme:&str,visited:usize)->Result<bool,ValueError>{Ok(crate::schema::snapshot::number::meaning(lexeme,self.control,SqliteSnapshotPhase::ProjectSnapshot,visited,0)?.valid)}\n}\n/// 🛡️ Validates GeoJSON under the SQLite operation's cancellation and row budget.\npub fn check_geojson_conformance_controlled(snapshot:&JsonSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<semio_framework_diagnostic::Diagnostic>,ValueError>{check_geojson_conformance_with_control(snapshot,&mut SqliteGeoJsonConformance{control})}\n`;
  writeFileSync(sqlite, adapter);
  let source = readFileSync(schema, 'utf8');
  const wrapper = items(source).find(item => item.name === 'check_geojson_conformance')!;
  source = source.slice(0, wrapper.start) + '/// 🛡️ Applies the semantic RFC7946 constraints without a storage authority.\npub fn check_geojson_conformance(snapshot:&JsonSnapshot)->Vec<semio_framework_diagnostic::Diagnostic>{check_geojson_conformance_with_control(snapshot,&mut conformance::UnboundedGeoJsonConformance).expect("unbounded semantic GeoJSON conformance")}\n' + source.slice(wrapper.end);
  source = source.replace(/#\[path\s*=\s*"🪶️sqlite\/🦀️\.rs"\]\s*mod sqlite_conformance;\s*pub use sqlite_conformance::check_geojson_conformance_controlled;/, '#[path="🛡️conformance/🦀️.rs"]\nmod conformance;\npub use conformance::{GeoJsonConformanceControl,check_geojson_conformance_with_control};');
  const readText = items(source).find(item => item.name === 'read_geojson_text');
  if (readText) {
    const text = join(subset, '🚪️io/📝️text/📸️snapshot/🦀️.rs');
    writeFileSync(text, readFileSync(text, 'utf8') + '\n#[allow(unused_imports)]\nmod reader {\nuse crate::standards::v_rfc8259::subsets::geojson::schema::*;\nuse crate::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;\n' + readText.code + '\n}\npub use reader::*;\n');
    source = source.slice(0, readText.start) + source.slice(readText.end);
    source = source.replace(/^use crate::standards::v_rfc8259::subsets::base::io::text::snapshot::\{?parse_json_text\}?;\n/m, '');
  }
  writeFileSync(schema, source);
  const artifact = subset.slice(0, subset.indexOf('/🏅️standards/'));
  const current = execFileSync('rg', ['--files', '-g', '*.rs', relative(repo, artifact)], { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 }).trim().split('\n');
  for (const relativeFile of current) {
    const file = resolve(repo, relativeFile); if (!existsSync(file)) continue; const before = readFileSync(file, 'utf8');
    let output = before.replace(/geojson::schema::check_geojson_conformance_controlled/g, 'geojson::io::sqlite::snapshot::check_geojson_conformance_controlled').replace(/use sqlite_conformance::check_geojson_conformance_controlled;\n/g, '');
    if (output !== before) writeFileSync(file, output);
  }
  const io = join(subset, '🚪️io/🦀️.rs'), ioSource = readFileSync(io, 'utf8');
  if (!/pub mod sqlite\b/.test(ioSource)) writeFileSync(io, ioSource + '\n#[path="🪶️sqlite/📸️snapshot/🦀️.rs"]\npub mod sqlite_snapshot;\npub mod sqlite { pub use super::sqlite_snapshot as snapshot; }\n');
  console.log(JSON.stringify({ neutralConformance: semantic, sqliteAdapter: sqlite }));
}

if (process.argv[2] === 'split-native') {
  const namesMoved: { name:string, text:string, binary:string, private:boolean }[] = [];
  for (const file of files.filter(file => /\/🚪️io\/📝️text\/(?:📸️snapshot|🧬️mutations)\/🦀️\.rs$/.test(file))) {
    let source=readFileSync(file,'utf8'); const clean=mask(source), removes:{start:number,end:number}[]=[];
    for(const found of clean.matchAll(/mod (snapshot|mutations)(?:_wire)?_codec\s*\{/g)) {
      const begin=found.index!+found[0].length;let end=begin,level=1;while(end<clean.length&&level){if(clean[end]==='{')level++;else if(clean[end]==='}')level--;end++;}
      const body=source.slice(begin,end-1),all=items(body),chosen=new Set<Item>();
      for(const item of all)if(item.kind==='fn'&&/^(?:encode|decode|write)_/.test(item.name)&&/(?:\[u8\]|Vec<u8>|pack_rt::write_)/.test(item.head))chosen.add(item);
      if(!chosen.size)continue;
      let changed=true;while(changed){changed=false;for(const item of all)if(!chosen.has(item)&&item.name&&(item.kind==='fn'&&item.head.startsWith('pub(crate)')||['struct','enum'].includes(item.kind)&&item.head.startsWith('pub(crate)'))&&[...chosen].some(selected=>new RegExp('\\b'+item.name+'\\b').test(mask(selected.code)))){chosen.add(item);changed=true;}}
      const semantic=body.match(/use crate::([\w:]+)::\*;/)?.[1];if(!semantic)throw new Error('Unmapped native '+file);
      const scope=semantic.includes('::schema')?semantic.slice(0,semantic.lastIndexOf('::schema')):semantic.endsWith('::'+found[1])?semantic.slice(0,semantic.lastIndexOf('::'+found[1])):semantic===found[1]?'':semantic;
      const textPath=[scope,'io','text',found[1]].filter(Boolean).join('::'),binaryPath=[scope,'io','binary',found[1]].filter(Boolean).join('::');
      const destination=file.replace('/📝️text/','/💾️binary/'),imports=all.filter(item=>item.kind==='use').map(item=>item.code).join('\n');
      const bodyMoved=[...chosen].sort((a,b)=>a.start-b.start).map(item=>item.code).join('\n');
      const module='native_'+found[1]+'_codec';
      writeFileSync(destination,readFileSync(destination,'utf8')+'\n#[allow(unused_imports)]\nmod '+module+' {\nuse super::*;\n'+imports+'\nuse crate::'+textPath+'::*;\n'+bodyMoved+'\n}\npub use '+module+'::*;\n');
      for(const item of chosen){removes.push({start:begin+item.start,end:begin+item.end});if(item.name&&item.kind==='fn')namesMoved.push({name:item.name,text:textPath,binary:binaryPath,private:item.head.startsWith('pub(crate)')});}
    }
    for(const item of removes.sort((a,b)=>b.start-a.start))source=source.slice(0,item.start)+source.slice(item.end);
    if(removes.length)writeFileSync(file,source);
  }
  for(const file of files){if(!existsSync(file))continue;let source=readFileSync(file,'utf8'),before=source;for(const entry of namesMoved){if(!source.includes(entry.name))continue;source=source.replace(new RegExp(entry.text+'::'+entry.name+'\\b','g'),entry.binary+'::'+entry.name);source=source.replace(new RegExp('(use\\s+(?:crate|semio_[a-z0-9_]+)::)'+entry.text+'::\\{([^}]+)\\};','g'),(whole,prefix,members)=>{const parts=members.split(',').map((part:string)=>part.trim()).filter(Boolean),moved=parts.filter((part:string)=>part===entry.name),kept=parts.filter((part:string)=>part!==entry.name);if(!moved.length)return whole;return(kept.length?prefix+entry.text+'::{'+kept.join(', ')+'};\n':'')+prefix+entry.binary+'::{'+moved.join(', ')+'};';});}if(source!==before)writeFileSync(file,source);}
  writeFileSync(join(ticket,'🗑️generated/runtime-native-moves.json'),JSON.stringify(namesMoved,null,2));
  console.log(JSON.stringify({nativeFunctions:namesMoved.length}));
}

if (process.argv[2] === 'controls') {
 const moves:{old:string,next:string,module:string}[]=[];
 for(const file of files.filter(file=>/\/🧬️schema\/📸️snapshot\/🚦️native\/🦀️\.rs$/.test(file))){
  const entry=sourceModules.get(file)?.[0];if(!entry)throw new Error('Unmapped native '+file);
  const semantic=entry.module.replace(/::sqlite_native$/,''),scope=semantic.replace(/::schema::snapshot$/,''),io=scope+'::io::sqlite::snapshot';
  const destination=file.replace('/🧬️schema/📸️snapshot/🚦️native/','/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/');
  let source=readFileSync(file,'utf8').replace(/\bsuper::sqlite::/g,'crate::'+io+'::').replace(/\bsuper::/g,'crate::'+semantic+'::');
  source=source.replace(/\bpub\(super\)/g,'pub(crate)');
  source=source.replace(/include_(str|bytes)!\("([^"]+)"\)/g,(whole,kind,path)=>'include_'+kind+'!("'+relative(dirname(destination),resolve(dirname(file),path)).split('\\').join('/')+'")');
  mkdirSync(dirname(dirname(destination)),{recursive:true});renameSync(dirname(file),dirname(destination));writeFileSync(destination,source);
  const semanticFile=join(dirname(dirname(file)),'🦀️.rs'),before=readFileSync(semanticFile,'utf8');writeFileSync(semanticFile,before.replace(/#\[path\s*=\s*"🚦️native\/🦀️\.rs"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod sqlite_native;/g,''));
  const ioFile=join(dirname(dirname(destination)),'🦀️.rs'),ioSource=readFileSync(ioFile,'utf8');writeFileSync(ioFile,ioSource+'\n#[path = "🚦️native/🦀️.rs"]\npub(crate) mod native;\n');
  moves.push({old:semantic+'::sqlite_native',next:io+'::native',module:semantic});
 }
 for(const file of files.filter(file=>/\/🧬️schema\/📸️snapshot\/🚦️sqlite\/🪶️copy\/🦀️\.rs$/.test(file))){
  const scope=file.slice(0,file.indexOf('/🧬️schema/')),semanticFile=join(scope,'🧬️schema/📸️snapshot/🦀️.rs'),semantic=sourceModules.get(semanticFile)?.[0]?.module;if(!semantic)throw new Error('Unmapped copy '+file);
  const destination=file.replace('/🧬️schema/📸️snapshot/🚦️sqlite/🪶️copy/','/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🪶️copy/');
  let source=readFileSync(file,'utf8').replace(/\bsuper::/g,'crate::'+semantic+'::');source=source.replace(/include_(str|bytes)!\("([^"]+)"\)/g,(whole,kind,path)=>'include_'+kind+'!("'+relative(dirname(destination),resolve(dirname(file),path)).split('\\').join('/')+'")');
  mkdirSync(dirname(dirname(destination)),{recursive:true});renameSync(dirname(file),dirname(destination));writeFileSync(destination,source);
 }
 const live=execFileSync('rg',['--files','-g','*.rs','✏️s/🔌️plugins'],{encoding:'utf8',maxBuffer:32*1024*1024}).trim().split('\n');let changed=0;
 for(const file of live){let source=readFileSync(file,'utf8'),before=source;for(const move of moves)source=source.replaceAll(move.old,move.next);if(source!==before){writeFileSync(file,source);changed++;}}
 writeFileSync(join(ticket,'🗑️generated/runtime-controls-moves.json'),JSON.stringify(moves,null,2));console.log(JSON.stringify({nativeOwners:moves.length,consumers:changed}));
}
if(process.argv[2]==='controls-fix'){
 const moves=JSON.parse(readFileSync(join(ticket,'🗑️generated/runtime-controls-moves.json'),'utf8'));let changed=0;
 for(const file of files){if(!existsSync(file))continue;let source=readFileSync(file,'utf8'),before=source;
 for(const move of moves){const semantic=move.module.replace(/::component$/,''),scope=semantic.replace(/::schema::snapshot$/,''),io=scope+'::io::sqlite::snapshot';source=source.replaceAll(move.next,io+'::native').replaceAll(move.old,io+'::native').replaceAll(semantic+'::sqlite_native',io+'::native').replaceAll(move.module+'::io::sqlite::snapshot',io);if(file.includes('/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/'))source=source.replaceAll(move.module+'::',semantic+'::');}
 if(source!==before){writeFileSync(file,source);changed++;}}
 console.log(JSON.stringify({corrected:changed}));
}
if(process.argv[2]==='orchestrators'){
 const owners:{scope:string,root:string,names:string[]}[]=[];let changed=0;
 for(const file of roots.filter(file=>/\/🧬️schema\/🦀️\.rs$/.test(file))){
  let source=readFileSync(file,'utf8'),clean=mask(source);const chunks:{start:number,end:number,code:string}[]=[];
  for(const found of clean.matchAll(/pub mod derived_(?:construction|analysis)\s*\{/g)){
   const start=found.index!,open=start+found[0].length-1;let end=open+1,level=1;while(end<clean.length&&level){if(clean[end]==='{')level++;else if(clean[end]==='}')level--;end++;}
   const body=source.slice(start,end);if(!/(?:from_text|from_binary|AnalyzeSource|parse_dsl|decode_pack)/.test(body))continue;
   const name=found[0].match(/mod (\w+)/)![1],tail=source.slice(end).match(new RegExp('^\\s*pub use '+name+'::\\*;'));if(tail)end+=tail[0].length;
   chunks.push({start,end,code:source.slice(start,end)});
  }
  if(!chunks.length)continue;
  for(const found of clean.matchAll(/semio_framework_plugin::derive_artifact_facets!\s*\(/g)){const start=found.index!,open=start+found[0].length-1;let end=open+1,level=1;while(end<clean.length&&level){if(clean[end]==='(')level++;else if(clean[end]===')')level--;end++;}if(clean[end]===';')end++;chunks.push({start,end,code:source.slice(start,end)});}
  const owner=sourceModules.get(file)?.[0];if(!owner)throw new Error('Unmapped orchestration '+file);
  const semantic=owner.module.replace(/::component$/,''),scope=semantic.replace(/::schema$/,''),ioFile=file.replace('/🧬️schema/','/🚪️io/');
  if(!existsSync(ioFile))throw new Error('Missing IO orchestration owner '+ioFile);
  let moved=chunks.map(chunk=>chunk.code).join('\n\n').replace(/\bsuper::super::io::/g,'crate::'+scope+'::io::');
  for(const chunk of chunks.sort((a,b)=>b.start-a.start))source=source.slice(0,chunk.start)+source.slice(chunk.end);
  writeFileSync(file,source);writeFileSync(ioFile,readFileSync(ioFile,'utf8')+'\n'+moved+'\n');
  const names=[...moved.matchAll(/pub (?:struct|spec) (\w+)|(?:builder|analyzer|composer):\s*(\w+)/g)].map(found=>found[1]??found[2]);
  owners.push({scope,root:owner.root,names});changed++;
 }
 for(const owner of owners){const rootSource=readFileSync(owner.root,'utf8');writeFileSync(owner.root,rootSource+'\npub use crate::'+owner.scope+'::io::{'+owner.names.join(', ')+'};\n');}
 for(const file of files){if(!existsSync(file))continue;let source=readFileSync(file,'utf8'),before=source;for(const owner of owners)for(const name of owner.names)source=source.replaceAll(owner.scope+'::schema::'+name,owner.scope+'::io::'+name);if(source!==before)writeFileSync(file,source);}
 writeFileSync(join(ticket,'🗑️generated/runtime-orchestrators.json'),JSON.stringify(owners,null,2));console.log(JSON.stringify({schemaOwners:changed,exportedSymbols:owners.reduce((n,owner)=>n+owner.names.length,0)}));
}
if(process.argv[2]==='io-qualify'){
 let changed=0;for(const file of files.filter(file=>/\/🚪️io\/🦀️\.rs$/.test(file))){let source=readFileSync(file,'utf8');if(!/(?<![\w:])io::(?:text|binary)::/.test(mask(source)))continue;const schemaFile=file.replace('/🚪️io/','/🧬️schema/'),owner=sourceModules.get(schemaFile)?.[0],scope=owner?.module.replace(/::component$/,'').replace(/::schema$/,'');if(!scope)throw new Error('Unmapped IO '+file);source=source.replace(/(?<![\w:])io::(text|binary)::/g,'crate::'+scope+'::io::$1::');writeFileSync(file,source);changed++;}console.log(JSON.stringify({qualifiedIo:changed}));
}
if(process.argv[2]==='generation-examples'){
 const root=files.find(file=>file.endsWith('/🧊️generation3d/🦀️.rs'))!,scope='standards::v1::subsets::any',schemaFile=join(dirname(root),'🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs'),ioFile=schemaFile.replace('/🧬️schema/','/🚪️io/📝️text/📸️snapshot/');
 let source=readFileSync(schemaFile,'utf8');const all=items(source),names=['default_snapshot','example_snapshot','example_document_json'],chosen=all.filter(item=>names.includes(item.name));
 const imports=source.match(/use crate::standards::v1::subsets::any::io::text::snapshot::\{[\s\S]*?\};/)?.[0]??'';
 for(const item of chosen.sort((a,b)=>b.start-a.start))source=source.slice(0,item.start)+source.slice(item.end);
 source=source.replace(imports,'').replace(/^use store::ArtifactDsl;\n/m,'');writeFileSync(schemaFile,source);
 writeFileSync(ioFile,readFileSync(ioFile,'utf8')+'\nmod source_examples {\nuse super::*;\nuse crate::'+scope+'::schema::*;\nuse crate::Generation3dSnapshot;\nuse store::ArtifactDsl;\n'+chosen.sort((a,b)=>a.start-b.start).map(item=>item.code).join('\n')+'\n}\npub use source_examples::*;\n');
 for(const file of files.filter(file=>file.startsWith(dirname(root)))){let source=readFileSync(file,'utf8'),before=source;for(const name of names)source=source.replaceAll(scope+'::schema::'+name,scope+'::io::text::snapshot::'+name);source=source.replace(new RegExp('(use crate::'+scope+'::schema::)\\{([^}]+)\\};','g'),(whole,prefix,members)=>{const parts=members.split(',').map((part:string)=>part.trim()).filter(Boolean),moved=parts.filter((part:string)=>names.includes(part)),kept=parts.filter((part:string)=>!names.includes(part));if(!moved.length)return whole;return(kept.length?prefix+'{'+kept.join(', ')+'};\n':'')+'use crate::'+scope+'::io::text::snapshot::{'+moved.join(', ')+'};';});if(source!==before)writeFileSync(file,source);}
 console.log(JSON.stringify({sourceHelpers:chosen.length}));
}
if(process.argv[2]==='restore-pure'){
 const restores=[{artifact:'🖼️bitmap',rep:'💾️binary',names:['write_region']},{artifact:'🏭️vdi3805',rep:'📝️text',names:['attributes_from_records','parse_f64']}];let restored=0;
 for(const row of restores){const root=files.find(file=>file.endsWith('/'+row.artifact+'/🦀️.rs'))!,file=files.find(file=>file.startsWith(dirname(root))&&file.endsWith('/🚪️io/'+row.rep+'/📸️snapshot/🦀️.rs'))!,semanticFile=file.replace('/🚪️io/'+row.rep+'/','/🧬️schema/'),module=sourceModules.get(semanticFile)![0].module.replace(/::component$/,''),scope=module.replace(/::schema::snapshot$/,'');let source=readFileSync(file,'utf8'),pieces:string[]=[];
 for(const name of row.names){const clean=mask(source),found=[...clean.matchAll(new RegExp('^pub(?:\\(crate\\))? fn '+name+'\\b','gm'))][0];if(!found)throw new Error('Missing pure '+name);const start=found.index!,open=clean.indexOf('{',start);let end=open+1,level=1;while(end<clean.length&&level){if(clean[end]==='{')level++;else if(clean[end]==='}')level--;end++;}pieces.push(source.slice(start,end));source=source.slice(0,start)+source.slice(end);restored++;}
 writeFileSync(file,source);writeFileSync(semanticFile,readFileSync(semanticFile,'utf8')+'\n'+pieces.join('\n')+'\n');
 for(const consumer of files.filter(file=>file.startsWith(dirname(root)))){if(!existsSync(consumer))continue;let code=readFileSync(consumer,'utf8'),before=code;for(const name of row.names)code=code.replaceAll(scope+'::io::'+(row.rep==='💾️binary'?'binary':'text')+'::snapshot::'+name,module+'::'+name);code=code.replace(new RegExp('(use crate::'+scope+'::io::'+(row.rep==='💾️binary'?'binary':'text')+'::snapshot::)\\{([^}]+)\\};','g'),(whole,prefix,members)=>{const parts=members.split(',').map((part:string)=>part.trim()).filter(Boolean),moved=parts.filter((part:string)=>row.names.includes(part)),kept=parts.filter((part:string)=>!row.names.includes(part));return moved.length?(kept.length?prefix+'{'+kept.join(', ')+'};\n':'')+'use crate::'+module+'::{'+moved.join(', ')+'};':whole;});if(code!==before)writeFileSync(consumer,code);}
 }console.log(JSON.stringify({restoredPureHelpers:restored}));
}
if(process.argv[2]==='unused-schema-io'){
 let changed=0;for(const file of files.filter(file=>file.includes('/🧬️schema/')&&!file.includes('/🧪️tests/'))){if(!existsSync(file))continue;let source=readFileSync(file,'utf8'),clean=mask(source),all=items(source),imports=all.filter(item=>item.kind==='use'&&/::io::(?:binary|text)::/.test(item.code));if(!imports.length)continue;let body=clean;for(const item of all.filter(item=>item.kind==='use').sort((a,b)=>b.start-a.start))body=body.slice(0,item.start)+body.slice(item.end);const edits:{start:number,end:number,text:string}[]=[];for(const item of imports){const match=item.code.match(/^(use [^{]+)::\{([^}]+)\};$/),single=item.code.match(/^use (.*)::(\w+);$/);if(match){const kept=match[2].split(',').map(part=>part.trim()).filter(part=>new RegExp('\\b'+(part.match(/ as (\w+)$/)?.[1]??part)+'\\b').test(body));if(kept.length!==match[2].split(',').filter(part=>part.trim()).length)edits.push({start:item.start,end:item.end,text:kept.length?match[1]+'::{'+kept.join(', ')+'};':''});}else if(single&&!new RegExp('\\b'+single[2]+'\\b').test(body))edits.push({start:item.start,end:item.end,text:''});}for(const edit of edits.sort((a,b)=>b.start-a.start))source=source.slice(0,edit.start)+edit.text+source.slice(edit.end);if(edits.length){writeFileSync(file,source);changed++;}}
 console.log(JSON.stringify({cleanedSemanticImports:changed}));
}
if(process.argv[2]==='dedupe-mounts'){
 let changed=0;for(const file of files){if(!existsSync(file))continue;let source=readFileSync(file,'utf8'),before=source;source=source.replace(/(#\[path\s*=\s*"🚪️io\/🦀️\.rs"\]\s*pub mod io;)\s*#\[path\s*=\s*"🚪️io\/🦀️\.rs"\]\s*pub mod io;/g,'$1');if(source!==before){writeFileSync(file,source);changed++;}}
 console.log(JSON.stringify({deduplicatedOwners:changed}));
}
if(process.argv[2]==='semantic-aliases'){
 const aliases:{root:string,io:string,semantic:string,names:string[]}[]=[];let owners=0;
 for(const file of files.filter(file=>/\/🚪️io\/(?:💾️binary|📝️text)\/(?:📸️snapshot|🧬️mutations|🔺️diff|💡️inferences)(?:\/[^/]+)?\/🦀️\.rs$/.test(file))){
  let source=readFileSync(file,'utf8'),before=source;const semanticFile=file.replace(/\/🚪️io\/(?:💾️binary|📝️text)\//,'/🧬️schema/'),owner=sourceModules.get(semanticFile)?.[0];if(!owner)continue;
  const semantic=owner.module.replace(/::component$/,''),facet=file.match(/\/(📸️snapshot|🧬️mutations|🔺️diff|💡️inferences)\//)![1],rep=file.includes('/💾️binary/')?'binary':'text',facetName=({'📸️snapshot':'snapshot','🧬️mutations':'mutations','🔺️diff':'diff','💡️inferences':'inferences'} as Record<string,string>)[facet],scope=semantic.split('::schema')[0],io=scope+'::io::'+rep+'::'+facetName+semantic.slice(semantic.indexOf('::schema')+('::schema::'+facetName).length);
  for(const item of items(source).filter(item=>item.kind==='use'&&/^pub use crate::.*schema(?:::|;)/.test(item.code))){
 const names=item.code.match(/::\{([^}]+)\};/)?.[1].split(',').map(part=>part.trim()).filter(Boolean)??(item.code.endsWith('::*;')?items(readFileSync(semanticFile,'utf8')).filter(item=>item.name&&/^pub/.test(item.head)).map(item=>item.name):[item.code.match(/::(\w+);$/)?.[1]].filter(Boolean) as string[]);
   aliases.push({root:owner.root,io,semantic,names});source=source.replace(item.code,item.code.replace(/^pub use/,'use'));
  }
  if(source!==before){writeFileSync(file,source);owners++;}
 }
 let consumers=0;
 for(const file of files){if(!existsSync(file))continue;let source=readFileSync(file,'utf8'),before=source;
 for(const alias of aliases){const own=file.startsWith(dirname(alias.root)),manifest=join(dirname(alias.root),'📦️packages/🦀️rust/Cargo.toml'),crate=existsSync(manifest)?readFileSync(manifest,'utf8').match(/^name\s*=\s*"([^"]+)"/m)?.[1].replaceAll('-','_'):undefined;for(const prefix of [...(own?['crate::']:[]),...(crate?[crate+'::']:[])]){
  if(!source.includes(prefix+alias.io)&&!source.includes(prefix+'io::'))continue;
  for(const path of [alias.io,'io::'+alias.io.split('::io::')[1]])for(const name of alias.names){const native=name.split(/\s+as\s+/)[0],visible=name.split(/\s+as\s+/)[1]??native;source=source.replaceAll(prefix+path+'::'+visible,prefix+alias.semantic+'::'+native);source=source.replace(new RegExp('(use '+prefix+path+'::)\\{([^}]+)\\};','g'),(whole,use,members)=>{const parts=members.split(',').map((part:string)=>part.trim()).filter(Boolean),moved=parts.filter((part:string)=>part.split(' ')[0]===visible),kept=parts.filter((part:string)=>part.split(' ')[0]!==visible);return moved.length?(kept.length?use+'{'+kept.join(', ')+'};\n':'')+'use '+prefix+alias.semantic+'::{'+moved.map((part:string)=>native+part.slice(visible.length)).join(', ')+'};':whole;});}
 }}if(source!==before){writeFileSync(file,source);consumers++;}}
 writeFileSync(join(ticket,'🗑️generated/runtime-semantic-aliases.json'),JSON.stringify(aliases,null,2));console.log(JSON.stringify({representationOwners:owners,canonicalConsumers:consumers}));
}
if(process.argv[2]==='native-adapters'){
 const moves:{root:string,old:string,next:string,scope:string,name:string}[]=[];
 for(const file of files.filter(file=>/\/🧬️schema\/📸️snapshot\/🛬️native\/🦀️\.rs$|\/🧬️schema\/📸️snapshot\/🛫️native\/🦀️\.rs$/.test(file))){
 const entry=sourceModules.get(file)?.[0];if(!entry)throw new Error('Unmapped native adapter '+file);const old=entry.module.replace(/::component(?=::|$)/g,''),name=old.split('::').at(-1)!,semantic=old.slice(0,old.lastIndexOf('::')),scope=semantic.replace(/::schema::snapshot$/,''),next=scope+'::io::sqlite::snapshot::'+name,destination=file.replace('/🧬️schema/📸️snapshot/','/🚪️io/🪶️sqlite/📸️snapshot/'),folder=file.match(/\/(🛬️native|🛫️native)\//)![1];
 let source=readFileSync(file,'utf8').replace(/\bsuper::sqlite::/g,'crate::'+scope+'::io::sqlite::snapshot::').replace(/\bsuper::/g,'crate::'+semantic+'::').replace(/\bpub\(super\)/g,'pub(crate)');source=source.replace(/include_(str|bytes)!\("([^"]+)"\)/g,(whole,kind,path)=>'include_'+kind+'!("'+relative(dirname(destination),resolve(dirname(file),path)).split('\\').join('/')+'")');
 mkdirSync(dirname(dirname(destination)),{recursive:true});renameSync(dirname(file),dirname(destination));writeFileSync(destination,source);
 const schemaFile=join(dirname(dirname(file)),'🦀️.rs'),schemaSource=readFileSync(schemaFile,'utf8');writeFileSync(schemaFile,schemaSource.replace(new RegExp('#\\[path\\s*=\\s*"'+folder+'/🦀️\\.rs"\\]\\s*(?:pub(?:\\([^)]*\\))?\\s+)?mod '+name+';','g'),''));const ioFile=join(dirname(dirname(destination)),'🦀️.rs');writeFileSync(ioFile,readFileSync(ioFile,'utf8')+'\n#[path = "'+folder+'/🦀️.rs"]\npub(crate) mod '+name+';\n');moves.push({root:entry.root,old,next,scope,name});
 }
 let changed=0;const live=execFileSync('rg',['--files','-g','*.rs','✏️s/🔌️plugins'],{encoding:'utf8',maxBuffer:32*1024*1024}).trim().split('\n');for(const file of live){let source=readFileSync(file,'utf8'),before=source;for(const move of moves){source=source.replaceAll(move.old,move.next);if(resolve(file).startsWith(dirname(move.root)))source=source.replaceAll('crate::'+move.scope.split('::').at(-1)+'::schema::snapshot::'+move.name,'crate::'+move.next);}
 if(source!==before){writeFileSync(file,source);changed++;}}
 writeFileSync(join(ticket,'🗑️generated/runtime-native-adapters.json'),JSON.stringify(moves,null,2));console.log(JSON.stringify({nativeAdapterOwners:moves.length,consumers:changed}));
}
if(process.argv[2]==='remaining-authority'){
 const drawing=files.find(file=>file.endsWith('/🖍️drawing/🦀️.rs'))!,oldBase=join(dirname(drawing),'🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned'),newBase=join(dirname(drawing),'🔨️modules/🏠️host/🧰️owned');
 if(existsSync(oldBase)){mkdirSync(dirname(newBase),{recursive:true});renameSync(oldBase,newBase);let root=readFileSync(drawing,'utf8');root=root.replace(/#\[path = "🏅️standards\/🔖️1\/🪆️subsets\/✳️any\/🧬️schema\/🧰️owned\/🦀️\.rs"\]\s*pub mod owned;/,'');root=root.replace(/pub mod owned\s*\{\s*pub use crate::standards::v1::subsets::any::schema::owned::\*;\s*\}/,'');root+='\n#[path = "."]\npub mod host {\n#[path = "🔨️modules/🏠️host/🧰️owned/🦀️.rs"]\npub mod owned;\n}\n';writeFileSync(drawing,root);
 for(const file of files){if(!existsSync(file))continue;let source=readFileSync(file,'utf8'),before=source;source=source.replaceAll('standards::v1::subsets::any::schema::owned::','host::owned::');if(file.startsWith(dirname(drawing)))source=source.replaceAll('crate::owned::','crate::host::owned::');source=source.replaceAll('semio_s_artifact_draw_drawing::owned::','semio_s_artifact_draw_drawing::host::owned::');if(source!==before)writeFileSync(file,source);}}
 const processRoot=files.find(file=>file.endsWith('/🧊️process3d/🦀️.rs'))!,schemaFile=join(dirname(processRoot),'🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs'),destination=schemaFile.replace('/🧬️schema/','/🚪️io/💾️binary/');let source=readFileSync(schemaFile,'utf8'),begin=source.indexOf('//#region 🔖️MountedTypedSnapshotOwner'),terminal='//#endregion 🔖️MountedTypedSnapshotOwner',end=source.indexOf(terminal,begin)+terminal.length;
 if(begin>=0){const imports=items(source).filter(item=>item.kind==='use').map(item=>item.code).join('\n'),body=source.slice(begin,end),names=items(body).filter(item=>item.name&&/^pub/.test(item.head)).map(item=>item.name);writeFileSync(schemaFile,source.slice(0,begin)+source.slice(end));writeFileSync(destination,readFileSync(destination,'utf8')+'\nmod mounted_snapshot_codec {\nuse super::*;\nuse crate::standards::v1::subsets::any::schema::snapshot::*;\n'+imports+'\n'+body+'\n}\npub use mounted_snapshot_codec::*;\n');
 for(const file of files){if(!existsSync(file))continue;let source=readFileSync(file,'utf8'),before=source;for(const name of names)source=source.replaceAll('standards::v1::subsets::any::schema::snapshot::'+name,'standards::v1::subsets::any::io::binary::snapshot::'+name);if(source!==before)writeFileSync(file,source);}
 writeFileSync(processRoot,readFileSync(processRoot,'utf8')+'\npub use crate::standards::v1::subsets::any::io::binary::snapshot::{'+names.join(', ')+'};\n');console.log(JSON.stringify({drawingHost:newBase,processMountedCodecNames:names}));}
}
if(process.argv[2]==='native-imports'){
 let changed=0;for(const file of files.filter(file=>/\/🚪️io\/🪶️sqlite\/📸️snapshot\/(?:🛬️native|🛫️native|🚦️native)\/🦀️\.rs$/.test(file))){
 let source=readFileSync(file,'utf8'),before=source;const physical=file.slice(0,file.indexOf('/🚪️io/')),schemaFile=join(physical,'🧬️schema/📸️snapshot/🦀️.rs'),semantic=readFileSync(schemaFile,'utf8');
 source=source.replace(/use crate::([\w:]+::schema::snapshot)::\{([^}]+)\};/g,(whole,path,members)=>{const parts=members.split(',').map((part:string)=>part.trim()).filter(Boolean),kept:string[]=[],moved:{name:string,rep:string}[]=[];for(const part of parts){const name=part.split(' ')[0];if(new RegExp('\\b'+name+'\\b').test(semantic)){kept.push(part);continue;}const rep=['binary','text'].find(rep=>{const ioFile=join(physical,'🚪️io',rep==='binary'?'💾️binary':'📝️text','📸️snapshot/🦀️.rs');return existsSync(ioFile)&&new RegExp('\\b(?:struct|enum) '+name+'\\b').test(mask(readFileSync(ioFile,'utf8')));});if(rep)moved.push({name:part,rep});else kept.push(part);}if(!moved.length)return whole;const scope=path.replace(/::schema::snapshot$/,'');return(kept.length?'use crate::'+path+'::{'+kept.join(', ')+'};\n':'')+moved.map(item=>'use crate::'+scope+'::io::'+item.rep+'::snapshot::'+item.name+';').join('\n');});
 if(source!==before){writeFileSync(file,source);changed++;}}
 console.log(JSON.stringify({nativeMirrorImports:changed}));
}
if(process.argv[2]==='process-mounted-fix'){
 const root=files.find(file=>file.endsWith('/🧊️process3d/🦀️.rs'))!,scope='standards::v1::subsets::any',binary=join(dirname(root),'🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs'),text=binary.replace('/💾️binary/','/📝️text/');let source=readFileSync(text,'utf8'),clean=mask(source);const found=clean.match(/^pub fn process3d_mounted_pack_session\b/m);
 if(found){const start=found.index!,open=clean.indexOf('{',start);let end=open+1,level=1;while(end<clean.length&&level){if(clean[end]==='{')level++;else if(clean[end]==='}')level--;end++;}const code=source.slice(start,end);source=source.slice(0,start)+source.slice(end);writeFileSync(text,source);let binarySource=readFileSync(binary,'utf8'),masked=mask(binarySource),begin=masked.indexOf('mod mounted_snapshot_codec {')+'mod mounted_snapshot_codec {'.length,cursor=begin,n=1;while(cursor<masked.length&&n){if(masked[cursor]==='{')n++;else if(masked[cursor]==='}')n--;cursor++;}binarySource=binarySource.slice(0,cursor-1)+'\n'+code+'\n'+binarySource.slice(cursor-1);writeFileSync(binary,binarySource);}
 const names=['Process3dMountedPackSession','Process3dMountedSnapshotOwner','process3d_mounted_pack_session'];for(const file of files.filter(file=>file.startsWith(dirname(root)))){if(!existsSync(file))continue;let source=readFileSync(file,'utf8'),before=source;for(const name of names){source=source.replaceAll('crate::schema::snapshot::'+name,'crate::'+scope+'::io::binary::snapshot::'+name).replaceAll(scope+'::io::text::snapshot::'+name,scope+'::io::binary::snapshot::'+name);if(file.includes('/schema/')&&file.includes('/🧪️tests/')){} }if(file.includes('/🧪️tests/🔬️retained-mounted-laws/'))source='use crate::'+scope+'::io::binary::snapshot::{'+names.join(', ')+'};\n'+source;if(source!==before)writeFileSync(file,source);}
 writeFileSync(root,readFileSync(root,'utf8')+'\npub use crate::'+scope+'::io::binary::snapshot::{Process3dMountedPackSession, process3d_mounted_pack_session};\n');
 console.log(JSON.stringify({processMountedNames:names}));
}
