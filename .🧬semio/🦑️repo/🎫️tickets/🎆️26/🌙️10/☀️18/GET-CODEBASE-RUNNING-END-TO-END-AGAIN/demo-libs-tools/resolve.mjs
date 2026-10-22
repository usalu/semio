import{readdirSync,readFileSync,statSync}from"node:fs";import{join,dirname,resolve}from"node:path";
const root=resolve(process.argv[2]);const pairs=[];
const pat=/(?:from\s+|import\s*\(\s*|import\s+|require\(\s*)(["'])([^./"'][^"']*)\1/g;
(function walk(d){for(const e of readdirSync(d)){if(["node_modules","dist","test-results",".cache"].includes(e))continue;const p=join(d,e);const s=statSync(p);if(s.isDirectory())walk(p);else if(/\.(ts|tsx|mjs|js|mts)$/.test(e)){const t=readFileSync(p,"utf8");for(const m of t.matchAll(pat))pairs.push([p,m[2]]);}}})(root);
const seen=new Set();let bad=0;
for(const[p,s]of pairs){if(s.startsWith("node:")||s.startsWith("bun:"))continue;const k=dirname(p)+"|"+s;if(seen.has(k))continue;seen.add(k);try{Bun.resolveSync(s,dirname(p));}catch(e){bad++;console.log("UNRESOLVED",p.replace(root,""),"->",s);}}
console.log("checked",seen.size,"unresolved",bad);
