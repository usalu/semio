import{readFileSync,writeFileSync}from'node:fs';import{createHash}from'node:crypto';
const path='🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-api/🔓️viz-api.tex',before=readFileSync(path,'utf8');
if(createHash('sha256').update(before).digest('hex')!=='ade4f49ed1febe6a4e769e400d45449e3eb0621462950642d607d5d16cf4ecdb')throw Error('API guard');
const visual=JSON.parse(readFileSync('🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json','utf8')).printed.visual,known=new Set<string>(visual.titles.map((x:any)=>x[0]));
let after=before;
const slash=String.fromCharCode(92),group=(text:string,start:number)=>{let depth=1,index=start+1;for(;index<text.length;index++){if(text[index]===slash){index++;continue;}if(text[index]==='{')depth++;if(text[index]==='}'&&!--depth)break;}return{body:text.slice(start+1,index),end:index+1};},changes:any[]=[];
for(const match of [...before.matchAll(/\\SemioTableLong(?:\[[^\]]*\])?\s*\{/g)].reverse()){
const start=match.index!+match[0].length-1,first=group(before,start),id=/^\s*\\Key\{(api-[a-z0-9-]+)\}/.exec(first.body)?.[1];
if(!id)continue;if(!known.has(id))throw Error('unknown caption '+id);
const old=before.slice(start,first.end),next='{'+slash+'ApiTitle{'+id+'}}';after=after.slice(0,start)+next+after.slice(first.end);changes.push({id,old,next});
}
for(const id of known){const old=slash+'subsection{'+slash+'Key{'+id+'}}',next=slash+'subsection{'+slash+'ApiTitle{'+id+'}}';if(after.includes(old)){after=after.replaceAll(old,next);changes.push({id,old,next});}}
const marker='% 🙂 One emoji',apparatus=String.raw`% 🌍 Localized visible titles retain their source-owned scope IDs.
\ExplSyntaxOn
\prop_new:N \g_api_titles_prop
\NewDocumentCommand \ApiTitleDefine { m m m }
  { \prop_gput:Nnn \g_api_titles_prop {#1} { \ApiText{#2}{#3} } }
\NewExpandableDocumentCommand \ApiTitle { m }
  { \prop_item:Nn \g_api_titles_prop {#1} }
\tl_new:N \g_api_visible_title_tl
\cs_new_eq:NN \api_toc_title_plain:n \semio_window_toc_title_plain:n
\cs_set_protected:Npn \semio_window_toc_title_plain:n #1 {
  \group_begin:
  \cs_set:Npn \Key ##1 {##1}
  \cs_set:Npn \Cs ##1 { \char_generate:nn {92}{12} ##1 }
  \cs_set:Npn \meta ##1 { <##1> }
  \cs_set:Npn \{ { \char_generate:nn {123}{12} }
  \cs_set:Npn \} { \char_generate:nn {125}{12} }
  \tl_gset:Nx \g_api_visible_title_tl {#1}
  \group_end:
  \exp_args:NV \api_toc_title_plain:n \g_api_visible_title_tl
}
\ExplSyntaxOff
`+visual.titles.map((row:string[])=>slash+'ApiTitleDefine{'+row.join('}{')+'}').join('\n')+'\n\n';
if(!after.includes(marker))throw Error('apparatus marker');after=after.replace(marker,apparatus+marker);
after=after.replace(slash+'hyperref[api-keys:#1]{'+slash+'Key{#1}}',slash+'hyperref[api-keys:#1]{'+slash+'ApiTitle{#1}}');
for(const[code,value]of[['u00E4','ä'],['u00FC','ü'],['u00DF','ß']])after=after.replaceAll(slash+code!,value!);
writeFileSync(import.meta.dir+'/api-candidate.tex',after);writeFileSync(import.meta.dir+'/caption-delta.json',JSON.stringify(changes,null,2));
console.log('[DEBUG] '+changes.length+' caption/heading replacements; prepared API-local title apparatus');
