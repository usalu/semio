import{readFileSync}from"node:fs";import{resolve}from"node:path";import Parser from"web-tree-sitter";
const root=resolve(import.meta.dir,"../../../../../../..");
await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const p=new Parser();p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));
for(const f of process.argv.slice(2)){const s=readFileSync(resolve(root,f),"utf8"),t=p.parse(s);const errs:string[]=[];const w=(n:Parser.SyntaxNode)=>{if(n.type==="ERROR"||n.isMissing())errs.push(`${n.startPosition.row+1}:${n.text.slice(0,120)}`);for(const c of n.children)w(c)};w(t.rootNode);console.log(f.slice(-50),t.rootNode.hasError(),errs.slice(0,5))}
