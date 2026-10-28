import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const R="/Users/ueli/Documents/semio/",G=R+".cache/b/g/m/generations/";
const man=JSON.parse(readFileSync(process.argv[2],"utf8"));
const name=process.argv[3]??Object.keys(man.owners).find(n=>n.startsWith("@semio-tech/repo-dashboard-rs"));
const row=man.owners[name];const f=JSON.parse(brotliDecompressSync(readFileSync(G+"o/"+row.fragment+".json.br")).toString("utf8"));
console.log(name,row.root,"bytes",row.bytes,"node keys",Object.keys(f.node),"edges",f.edges.length);
const ni=f.node.namedInputs;console.log("namedInputs",Object.keys(ni).map(k=>k+":"+(Array.isArray(ni[k])?ni[k].length:typeof ni[k])).join(" "));
const t=Object.entries(f.node.targets)[0];console.log(t[0],JSON.stringify(t[1]).slice(0,600));
