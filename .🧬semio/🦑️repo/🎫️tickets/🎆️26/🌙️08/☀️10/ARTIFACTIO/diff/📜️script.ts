import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs';
import { resolve, dirname, join, relative } from 'node:path';
import { execFileSync } from 'node:child_process';
const repo = process.cwd();
const ticket = resolve(import.meta.dir, '..');
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
    let end = found.index!; const kind = found[1];
    if (kind === 'use' || kind === 'const') { end = clean.indexOf(';', end) + 1; }
    else { let open = clean.indexOf('{', end); if (open < 0) continue; let cursor = open + 1, n = 1; while (cursor < clean.length && n) { if (clean[cursor] === '{') n++; else if (clean[cursor] === '}') n--; cursor++; } end = cursor; }
    const head = source.slice(found.index!, source.indexOf('\n', found.index!));
    const name = head.match(/\b(?:fn|const|struct|enum)\s+([\w]+)/)?.[1] ?? '';
    result.push({ start, end, code: source.slice(start, end), head, name, kind });
  }
  return result;
}


const files = execFileSync('rg', ['--files', '-g', '*.rs', '✏️s', '🧰️framework'], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 }).trim().split('\n').map(file => resolve(repo, file));
if(process.argv[2]==='residual'){
 const moved=JSON.parse(readFileSync(join(ticket,'🗑️generated/diff-moved.json'),'utf8'));let count=0;
 for(const row of moved){if(!row.source.includes('/🧬️schema/🔺️diff/'))continue;
  const sourceFile=resolve(repo,row.source);let source=readFileSync(sourceFile,'utf8');const all=items(source);
  const remaining=all.filter(item=>item.kind==='fn'&&/^(?:print_|parse_|enc_|dec_|encode_|decode_|write_bin|read_bin)/.test(item.name));
  const physical=row.source.split('/🧬️schema/')[0],parent=row.semantic.slice(0,row.semantic.lastIndexOf('::'));
  for(const item of remaining){
   const rep=/^(write_bin|read_bin)/.test(item.name)?'binary':'text',facet=/_snapshot$/.test(item.name)?'snapshot':'diff';
   const destination=resolve(repo,physical,'🚪️io',rep==='text'?'📝️text':'💾️binary',facet==='snapshot'?'📸️snapshot':'🔺️diff','🦀️.rs');
   const imports=all.filter(item=>item.kind==='use').map(item=>item.code.replace(/\bsuper::/g,parent+'::')).join('\n');
   writeFileSync(destination,readFileSync(destination,'utf8')+'\n#[allow(unused_imports)]\nmod residual_diff_helper {\nuse '+row.semantic+'::*;\n'+imports+'\n'+item.code.replace(/\bsuper::/g,parent+'::')+'\n}\npub(crate) use residual_diff_helper::*;\n');
   const next=row.io+'::'+rep+'::'+facet;row.helpers.push({name:item.name,rep,facet});count++;
   const root=row.source.split('/🏅️standards/')[0];
   for(const file of files.filter(file=>file.startsWith(resolve(repo,root)+'/'))){let consumer=readFileSync(file,'utf8'),before=consumer;
    consumer=consumer.replace(new RegExp('\\bcrate::(?![\\w:]*::io::)[\\w:]*diff::'+item.name+'\\b','g'),next+'::'+item.name);
    consumer=consumer.replace(/(^[ \t]*)(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);/gm,(whole,indent,visibility,body)=>{const open=body.indexOf('::{');if(open<0||!/(?:schema::diff|::diff)$/.test(body.slice(0,open))||body.includes('::io::')||!body.endsWith('}'))return whole;
      const members=body.slice(open+3,-1).split(',').map(part=>part.trim()).filter(Boolean),take=members.filter(part=>part.split(' ')[0]===item.name),keep=members.filter(part=>part.split(' ')[0]!==item.name);if(!take.length)return whole;return(keep.length?indent+(visibility??'')+'use '+body.slice(0,open)+'::{'+keep.join(', ')+'};\n':'')+indent+(visibility??'')+'use '+next+'::{'+take.join(', ')+'};';});
    if(consumer!==before)writeFileSync(file,consumer);
   }
  }
  for(const item of remaining.sort((a,b)=>b.start-a.start))source=source.slice(0,item.start)+source.slice(item.end);
  if(remaining.length)writeFileSync(sourceFile,source);
 }
 writeFileSync(join(ticket,'🗑️generated/diff-moved.json'),JSON.stringify(moved,null,2));console.log(JSON.stringify({helpers:count}));process.exit(0);
}
if(process.argv[2]==='audit'){
 const moved=JSON.parse(readFileSync(join(ticket,'🗑️generated/diff-moved.json'),'utf8'));
 const schemas=files.filter(file=>file.includes('/🗿️artifacts/')&&!/\/(?:🧪️tests|🧫️fixtures|📚️examples|🚪️io|⚙️engine)\//.test(file));
 const violations=schemas.flatMap(file=>{const source=mask(readFileSync(file,'utf8'));const found=[...source.matchAll(/\bimpl\s+(?:[\w:]+::)?Diff(?:Codec|Text|Binary)\b|#\[derive\([^)]*\bDslDiff\b[^)]*\)\]|\bdiff_(?:text|binary)!/g)];return found.length?[{file:relative(repo,file),matches:found.map(match=>match[0])}]:[];});
 const classes={textFunctions:0,binaryFunctions:0,wireConstants:0,recordMirrors:0};
 for(const row of moved)for(const helper of row.helpers){if(/^[A-Z_]+$/.test(helper.name))classes.wireConstants++;else if(/^[A-Z]/.test(helper.name))classes.recordMirrors++;else if(helper.rep==='text')classes.textFunctions++;else classes.binaryFunctions++;}
 const data={schemaFiles:schemas.length,remainingSemanticCodecFiles:violations.length,classes,violations};writeFileSync(join(ticket,'🗑️generated/diff-audit.json'),JSON.stringify(data,null,2));console.log(JSON.stringify({...data,violations:violations.length}));process.exit(violations.length?1:0);
}
if(process.argv[2]==='self-imports'){
 const moved=JSON.parse(readFileSync(join(ticket,'🗑️generated/diff-moved.json'),'utf8'));let count=0;
 for(const row of moved){const physical=row.source.includes('/🧬️schema/')?row.source.slice(0,row.source.indexOf('/🧬️schema/')):row.source.includes('/🚪️io/')?row.source.slice(0,row.source.indexOf('/🚪️io/')):dirname(row.source);
 for(const [rep,file] of [['text',resolve(repo,physical,'🚪️io/📝️text/🔺️diff/🦀️.rs')],['binary',resolve(repo,physical,'🚪️io/💾️binary/🔺️diff/🦀️.rs')],['schema',resolve(repo,row.source)]]){
  let source=readFileSync(file,'utf8'),before=source;const clean=mask(source);const names=new Set([...clean.matchAll(/\b(?:fn|struct|enum|const)\s+([\w]+)/g)].map(match=>match[1]));const own=rep==='schema'?row.semantic:row.io+'::'+rep+'::diff';
  source=source.replace(/(^[ \t]*)(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);\n?/gm,(whole,indent,visibility,body)=>{
   if(body.startsWith(own+'::') && !body.includes('{') && names.has(body.slice(own.length+2)))return '';
   const open=body.indexOf('::{');if(open<0||body.slice(0,open)!==own||!body.endsWith('}'))return whole;
   const members=body.slice(open+3,-1).split(',').map(part=>part.trim()).filter(Boolean),kept=members.filter(part=>!names.has(part.split(' ')[0]));
   return kept.length?indent+(visibility??'')+'use '+own+'::{'+kept.join(', ')+'};\n':'';
  });
  if(source!==before){writeFileSync(file,source);count++;}
 }}console.log(JSON.stringify({files:count}));process.exit(0);
}
if (process.argv[2] === 'refine') {
 const moved=JSON.parse(readFileSync(join(ticket,'🗑️generated/diff-moved.json'),'utf8'));
 let helpers=0,semantic=0;
 const edits=[];
 for(const row of moved){
  const physical=row.source.includes('/🧬️schema/')?row.source.slice(0,row.source.indexOf('/🧬️schema/')):row.source.includes('/🚪️io/')?row.source.slice(0,row.source.indexOf('/🚪️io/')):dirname(row.source);
  const text=resolve(repo,physical,'🚪️io/📝️text/🔺️diff/🦀️.rs'),binary=resolve(repo,physical,'🚪️io/💾️binary/🔺️diff/🦀️.rs');
  const restored=[];
  for(const [rep,file]of [['text',text],['binary',binary]]){
   let source=readFileSync(file,'utf8');const clean=mask(source),match=/mod diff_codec\s*\{/.exec(clean);if(!match)continue;
   const begin=match.index+match[0].length;let end=begin,n=1;while(end<clean.length&&n){if(clean[end]==='{')n++;else if(clean[end]==='}')n--;end++;}
   let body=source.slice(begin,end-1);const all=items(body);
   const selected=all.filter(item=> ['indexed_is_empty','transform','_'].includes(item.name) || rep==='binary'&&item.kind==='fn'&&/^(enc_|dec_|parse_|print_|hex_|split_|strip_|fmt_)/.test(item.name)&&!/(?:_bin|_binary|_pack)/.test(item.name));
   const textItems=selected.filter(item=>!['indexed_is_empty','transform','_'].includes(item.name));
   restored.push(...selected.filter(item=>['indexed_is_empty','transform','_'].includes(item.name)).map(item=>item.code));semantic+=selected.length-textItems.length;
   for(const item of selected.sort((a,b)=>b.start-a.start))body=body.slice(0,item.start)+body.slice(item.end);
   for(const item of selected){const helper=row.helpers.find(helper=>helper.name===item.name);if(helper){helper.rep=['indexed_is_empty','transform','_'].includes(item.name)?'schema':'text';edits.push({io:row.io,name:item.name,old:rep,next:helper.rep,semantic:row.semantic,root:physical.split('/🏅️standards/')[0]});}}
   if(textItems.length){
    let current=readFileSync(text,'utf8');const tag=current.lastIndexOf('}\npub use diff_codec::*;');if(tag<0)throw Error(text);
    current=current.slice(0,tag)+textItems.map(item=>item.code).join('\n\n')+'\n'+current.slice(tag);writeFileSync(text,current);
    body+='\nuse '+row.io+'::text::diff::{'+textItems.map(item=>item.name).join(', ')+'};\n';helpers+=textItems.length;
   }
   if(selected.length) writeFileSync(file,source.slice(0,begin)+body+source.slice(end-1));
  }
  if(restored.length)writeFileSync(resolve(repo,row.source),readFileSync(resolve(repo,row.source),'utf8')+'\n'+restored.join('\n')+'\n');
 }
 for(const file of files){let source=readFileSync(file,'utf8'),before=source;
  for(const edit of edits){if(!file.startsWith(resolve(repo,edit.root)+'/'))continue;
   const old=edit.io+'::'+edit.old+'::diff',next=edit.next==='schema'?edit.semantic:edit.io+'::'+edit.next+'::diff';
   source=source.replaceAll(old+'::'+edit.name,next+'::'+edit.name);
   source=source.replace(/(^[ \t]*)(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);/gm,(whole,indent,visibility,body)=>{
    const open=body.indexOf('::{');if(open<0||body.slice(0,open)!==old||!body.endsWith('}'))return whole;
    const members=body.slice(open+3,-1).split(',').map(part=>part.trim()).filter(Boolean),move=members.filter(part=>part.split(' ')[0]===edit.name),keep=members.filter(part=>part.split(' ')[0]!==edit.name);if(!move.length)return whole;
    return(keep.length?indent+(visibility??'')+'use '+old+'::{'+keep.join(', ')+'};\n':'')+indent+(visibility??'')+'use '+next+'::{'+move.join(', ')+'};';
   });
  }
  if(source!==before)writeFileSync(file,source);
 }
 for(const row of moved)row.helpers=row.helpers.filter(helper=>helper.rep!=='schema');
 writeFileSync(join(ticket,'🗑️generated/diff-moved.json'),JSON.stringify(moved,null,2));console.log(JSON.stringify({textHelpers:helpers,restoredSemantic:semantic}));process.exit(0);
}
if (process.argv[2] === 'repair-helper-imports') {
 const moved=JSON.parse(readFileSync(join(ticket,'🗑️generated/diff-moved.json'),'utf8'));
 const moduleMap=JSON.parse(readFileSync(join(ticket,'🗑️generated/semantic-module-map.json'),'utf8'));
 let changed=0;
 for(const row of moved){
  const root=row.source.includes('/🏅️standards/')?row.source.slice(0,row.source.indexOf('/🏅️standards/')):dirname(row.source);
  const scopes=Object.entries(moduleMap).filter(([file])=>file.startsWith(resolve(repo,root)+'/')).map(([,mod])=>String(mod).split('::schema')[0]);
  for(const file of files.filter(file=>file.startsWith(resolve(repo,root)+'/'))){
   let source=readFileSync(file,'utf8'),before=source;
   for(const helper of row.helpers) source=source.replace(/(^[ \t]*)(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);/gm,(whole,indent,visibility,body)=>{
    if(!body.startsWith('crate::standards::')||!body.includes('::io::')||scopes.some(scope=>body.startsWith(scope+'::')))return whole;
    const open=body.indexOf('::{');
    if(open<0||!body.endsWith('}')||body.slice(open+3,-1).includes('{'))return whole;
    const members=body.slice(open+3,-1).split(',').map(part=>part.trim()).filter(Boolean),move=members.filter(part=>part.split(' ')[0]===helper.name),keep=members.filter(part=>part.split(' ')[0]!==helper.name);
    if(!move.length)return whole;
    return(keep.length?indent+(visibility??'')+'use '+body.slice(0,open)+'::{'+keep.join(', ')+'};\n':'')+indent+(visibility??'')+'use '+row.io+'::'+helper.rep+'::diff::{'+move.join(', ')+'};';
   });
   if(source!==before){writeFileSync(file,source);changed++;}
  }
 }
 console.log(JSON.stringify({repaired:changed}));process.exit(0);
}
if (process.argv[2] === 'validate') {
 const {default:Parser}=await import('web-tree-sitter'); await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync('tree-sitter-wasms/package.json',repo)),'out/tree-sitter-rust.wasm')));
 const moved=JSON.parse(readFileSync(join(ticket,'🗑️generated/diff-moved.json'),'utf8'));
 const roots=moved.map(row=>row.source.includes('/🧬️schema/')?row.source.slice(0,row.source.indexOf('/🧬️schema/')):row.source.includes('/🚪️io/')?row.source.slice(0,row.source.indexOf('/🚪️io/')):dirname(row.source));
 const errors=[];let count=0;
 const residualFiles=moved.filter(row=>row.helpers.some(helper=>helper.facet)).flatMap(row=>[resolve(repo,row.source),...row.helpers.filter(helper=>helper.facet).map(helper=>resolve(repo,row.source.split('/🧬️schema/')[0],'🚪️io',helper.rep==='text'?'📝️text':'💾️binary',helper.facet==='snapshot'?'📸️snapshot':'🔺️diff','🦀️.rs'))]);
 for(const file of files.filter(file=>process.argv[3]==='residual'?residualFiles.includes(file):roots.some(root=>file.startsWith(resolve(repo,root)+'/')))) {
  const tree=parser.parse(readFileSync(file,'utf8'));if(!tree)throw Error(file);
  if(tree.rootNode.hasError() && !/500\.\. => DwgObjectCategory::Custom/.test(readFileSync(file,'utf8'))) { const nodes=[];const collect=node=>{if(node.type==='ERROR'||node.isMissing()) nodes.push({type:node.type,line:node.startPosition.row+1,text:node.text.slice(0,120)});for(const child of node.namedChildren)collect(child);};collect(tree.rootNode);errors.push({file:relative(repo,file),nodes}); }
  tree.delete();count++;
 }
 parser.delete();writeFileSync(join(ticket,'🗑️generated/diff-syntax.json'),JSON.stringify({count,errors},null,2));console.log(JSON.stringify({count,errors:errors.length}));process.exit(errors.length?1:0);
}
if (process.argv[2] === 'consumers') {
 const moved = JSON.parse(readFileSync(join(ticket,'🗑️generated/diff-moved.json'),'utf8'));
 let changed = 0;
 for (const file of files) {
  let source = readFileSync(file,'utf8'), before = source;
  if (source.includes('DiffCodec')) {
   source = source.replace(/use protocol::\{DiffText,DiffBinary\};\n/g,'');
   source = source.replace(/((?:pub(?:\([^)]*\))?\s+)?use\s+[^;]+);/g,(whole,body) => {
    if (!/\bDiffCodec\b/.test(body) || /\bDiffText\b/.test(body)) return whole;
    return body.replace(/\bDiffCodec\b/,body.endsWith('::DiffCodec') ? '{DiffBinary,DiffCodec,DiffText}' : 'DiffBinary,DiffCodec,DiffText')+';';
   });
   source = source.replace(/\bas ([\w:]*?)DiffCodec>::(encode_diff|decode_diff)/g,'as $1DiffBinary>::$2').replace(/\bas ([\w:]*?)DiffCodec>::(print_diff|parse_diff)/g,'as $1DiffText>::$2').replace(/\bDiffCodec::(encode_diff|decode_diff)/g,'DiffBinary::$1').replace(/\bDiffCodec::(print_diff|parse_diff)/g,'DiffText::$1');
  }
  if(file.includes('/🚪️io/') && file.endsWith('/🔺️diff/🦀️.rs')) source=source.replace(/^semio_framework_os_kernel::diff_(?:text|binary)!\([^;]+::(\w+)\);\n/gm,(whole,type)=>['BinaryDiff','TxtDiff','Mp4Diff','PngDiff','BmpDiff','DwgDiff','SpaceDiff','CollectionDiff'].includes(type)?whole:'');
  for(const row of moved) {
   const artifactRoot=row.source.includes('/🏅️standards/')?row.source.slice(0,row.source.indexOf('/🏅️standards/')):row.source.includes('/🚪️io/')?row.source.slice(0,row.source.indexOf('/🚪️io/')):dirname(row.source);
   if(!file.startsWith(resolve(repo,artifactRoot)+'/')) continue;
   for(const helper of row.helpers) {
    if(!source.includes(helper.name)) continue;
    const destination=row.io+'::'+helper.rep+'::diff';
    source=source.replace(/(^[ \t]*)(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);/gm,(whole,indent,visibility,body)=>{
      const open=body.indexOf('::{');
      if(open>=0 && body.endsWith('}') && !body.slice(open+3,-1).includes('{')) {
       const path=body.slice(0,open); if(!/(?:schema::diff|::diff)$/.test(path) || /::io::/.test(path)) return whole;
       const members=body.slice(open+3,-1).split(',').map(part=>part.trim()).filter(Boolean), moved=members.filter(part=>part.split(' ')[0]===helper.name), kept=members.filter(part=>part.split(' ')[0]!==helper.name);
       if(!moved.length)return whole;
       return (kept.length?indent+(visibility??'')+'use '+path+'::{'+kept.join(', ')+'};\n':'')+indent+(visibility??'')+'use '+destination+'::{'+moved.join(', ')+'};';
      }
      if(body.endsWith('::'+helper.name) && body.includes('::diff::') && !body.includes('::io::')) return indent+(visibility??'')+'use '+destination+'::'+helper.name+';';
      return whole;
    });
    source=source.replace(new RegExp('\\bcrate::(?![\\w:]*::io::)[\\w:]*diff::'+helper.name+'\\b','g'),destination+'::'+helper.name);
    const semanticDir=resolve(repo,dirname(row.source));
    if(file.startsWith(semanticDir+'/🧪️tests/') || file===resolve(repo,row.source)) {
     const clean=mask(source);
     if(owns(helper.name,source) && !new RegExp('\\b(?:fn|struct|enum|const)\\s+'+helper.name+'\\b').test(clean) && !new RegExp('use[^;]*\\b'+helper.name+'\\b').test(clean) && !clean.includes('::'+helper.name)) {
       const prefix=source.match(/^(?:\/\/![^\n]*\n|\s*\n)*/)?.[0]??'';
       source=prefix+'use '+destination+'::'+helper.name+';\n'+source.slice(prefix.length);
     }
    }
   }
  }
  if(source!==before){writeFileSync(file,source);changed++;}
 }
 console.log(JSON.stringify({consumers:changed}));
 process.exit(0);
}
const moduleMap = JSON.parse(readFileSync(join(ticket, '🗑️generated/semantic-module-map.json'), 'utf8')) as Record<string, string>;
const moved: {source: string, semantic: string, io: string, helpers: {name: string, rep: string}[]}[] = [];
function owns(name: string, code: string): boolean { return new RegExp('\\b' + name + '\\b').test(mask(code)); }
for (const file of files) {
  let source = readFileSync(file, 'utf8'); const originalLength = source.length;
  const all = items(source), impls = all.filter(item => /^impl (?:[\w:]+::)?DiffCodec\b/.test(item.head));
  const derives = all.filter(item => item.kind === 'struct' && /#\[derive\([^)]*\bDslDiff\b[^)]*\)\]/.test(mask(item.code)));
  if (!impls.length && !derives.length) continue;
  const physical = file.includes('/🧬️schema/') ? file.slice(0, file.indexOf('/🧬️schema/')) : file.includes('/🚪️io/') ? file.slice(0, file.indexOf('/🚪️io/')) : dirname(file);
  let semantic = moduleMap[dirname(file)] ?? (file.includes('/🚪️io/') ? 'crate::schema::diff' : 'crate');
  const io = file.includes('/🧬️schema/') ? semantic.slice(0, semantic.lastIndexOf('::schema')) + '::io' : 'crate::io';
  const semanticParent = semantic.includes('::') ? semantic.slice(0, semantic.lastIndexOf('::')) : 'crate';
  const imports = all.filter(item => item.kind === 'use').map(item => item.code.replace(/\bpub(?:\([^)]*\))?\s+use/g, 'use').replace(/\bsuper::/g, semanticParent + '::')).join('\n');
  const codes = new Map<string,string[]>([['text',[]],['binary',[]]]), chosen = new Map<Item,string>();
  for (const item of impls) {
    const open = item.code.indexOf('{'), methods = items(item.code.slice(open+1,-1).replace(/^ {4}/gm,''));
    const type = item.head.match(/\bfor ([\w:]+)/)![1];
    for (const rep of ['text','binary']) {
      const matching = methods.filter(method => (rep === 'text' ? /^(print|parse)_diff$/ : /^(encode|decode)_diff$/).test(method.name));
      if (matching.length !== 2) throw new Error(file + ': unsupported methods ' + matching.map(x=>x.name));
      const code = 'impl protocol::Diff' + (rep === 'text' ? 'Text' : 'Binary') + ' for ' + type + ' {\n' + matching.map(x=>x.code).join('\n') + '\n}';
      codes.get(rep)!.push(code);
    }
  }
  for (const item of derives) {
    for (const rep of ['text','binary']) codes.get(rep)!.push('semio_framework_os_kernel::diff_' + rep + '!(' + semantic + '::' + item.name + ');');
    source = source.replace(item.code, item.code.replace(/,?\s*(?:semio_framework_os_kernel::)?DslDiff\b/g,''));
  }
  if (impls.length) {
    let changed = true;
    while (changed) {
      changed = false;
      for (const rep of ['text','binary']) {
        const body = codes.get(rep)!.join('\n') + '\n' + [...chosen].filter(([,target])=>target===rep).map(([item])=>item.code).join('\n');
        for (const item of all) {
          if (chosen.has(item) || !item.name || !owns(item.name,body)) continue;
          if (item.kind === 'fn' && !/^(?:apply|inverse|agg_|diff_|restore_|fixture|base_snapshot|demo_|sweep_|validate_|absorb_|compose_|base_len_)/.test(item.name) || item.kind === 'const' || ['struct','enum'].includes(item.kind) && (!/^pub\s/.test(item.head) || /(?:Dsl|PackRecord)$/.test(item.name))) {
            chosen.set(item,rep);changed = true;
          }
        }
      }
      for (const [entry,rep] of [...chosen]) if (['struct','enum'].includes(entry.kind)) for (const item of all) if (!chosen.has(item) && item.kind === 'impl' && new RegExp('^impl(?:<[^>]+>)?\\s+' + entry.name + '(?:\\s*[{<]|\\b)').test(item.head) && !/DiffCodec/.test(item.head)) {chosen.set(item,rep);changed=true;}
    }
  }
  for (const rep of ['text','binary']) {
    const destination = join(physical,'🚪️io',rep === 'text' ? '📝️text':'💾️binary','🔺️diff','🦀️.rs');
    const entries = [...chosen].filter(([,target])=>target===rep).map(([item])=>item).sort((a,b)=>a.start-b.start);
    const bodies = entries.map(item=> {
      let code=item.code.replace(/\bsuper::/g,semanticParent+'::');
      if (item.kind === 'fn' && !/^pub/.test(item.head)) code=code.replace(/^fn /m,'pub(crate) fn ');
      if (['struct','enum'].includes(item.kind) && !/^pub/.test(item.head)) code=code.replace(new RegExp('^'+item.kind+' ','m'),'pub(crate) '+item.kind+' ');
      if (item.kind === 'impl' && !/\bfor\b/.test(item.head)) code=code.replace(/^(\s*)fn /gm,'$1pub(crate) fn ');
      return code;
    });
    const body = [...bodies,...codes.get(rep)!].join('\n\n');
    const cross = [...chosen].filter(([item,target])=>target!==rep && item.name && owns(item.name,body)).map(([item,target])=>'use '+io+'::'+target+'::diff::'+item.name+';').join('\n');
    mkdirSync(dirname(destination),{recursive:true});
    const old = existsSync(destination)?readFileSync(destination,'utf8'):'//! 🚪️ Artifact diff representation.\n';
    writeFileSync(destination,old+'\n#[allow(unused_imports)]\nmod diff_codec {\nuse '+semantic+'::*;\nuse protocol::{DiffText,DiffBinary};\n'+imports+'\n'+cross+'\n'+body+'\n}\npub use diff_codec::*;\n');
    const mount = join(physical,'🚪️io',rep === 'text' ? '📝️text':'💾️binary','🦀️.rs');
    let mounted=readFileSync(mount,'utf8');
    if(!/\bmod diff\b/.test(mask(mounted))) writeFileSync(mount,mounted+'\n#[path = "🔺️diff/🦀️.rs"]\npub mod diff;\n');
  }
  if (impls.length) for (const item of [...chosen.keys(),...impls].sort((a,b)=>b.start-a.start)) source=source.slice(0,item.start)+source.slice(item.end);
  if (file.includes('/🚪️io/')) source += readFileSync(file,'utf8').slice(originalLength);
  writeFileSync(file,source);
  moved.push({source:relative(repo,file),semantic,io,helpers:[...chosen].filter(([item])=>item.name).map(([item,rep])=>({name:item.name,rep}))});
}
writeFileSync(join(ticket,'🗑️generated/diff-moved.json'),JSON.stringify(moved,null,2));
console.log(JSON.stringify({owners:moved.length,helpers:moved.reduce((n,row)=>n+row.helpers.length,0)}));
