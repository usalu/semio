const unresolved:string[]=[];const nodeHits = new Map<string,string[]>();
const r = await Bun.build({ entrypoints:["/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🟦️.tsx"], target:"browser", define:{"import.meta.vitest":"undefined"}, write:false, metafile:true as any, throw:false,
 plugins:[{name:"nodeprobe",setup(b){
  b.onResolve({filter:/^(node:|fs$|path$|child_process$|os$|crypto$|url$|module$)/},a=>{const l=nodeHits.get(a.path)??[];l.push(a.importer);nodeHits.set(a.path,l);return {path:a.path,namespace:"stub"};});
  b.onLoad({filter:/.*/,namespace:"stub"},()=>({contents:"export default {};",loader:"js"}));
  b.onResolve({filter:/(\.(wasm|css)|\?.*|^virtual:.*|^@semio-tech\/(flow-core|[a-z0-9-]+-rs)(\/.*)?)$/},a=>({path:a.path,external:true}));
 }}]});
console.log("success",r.success);
for(const l of r.logs.slice(0,40))console.log(String(l.message??l).slice(0,300));
for(const [k,v] of nodeHits)console.log("NODE",k,[...new Set(v)].slice(0,8));
const meta:any=(r as any).metafile; if(meta) console.log("inputs",Object.keys(meta.inputs).length);
for(const k of Object.keys((r as any).metafile.inputs))console.log('IN',k);
