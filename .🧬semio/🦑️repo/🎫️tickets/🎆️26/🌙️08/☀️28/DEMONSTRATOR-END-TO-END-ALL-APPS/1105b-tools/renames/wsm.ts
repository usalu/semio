import {readFileSync,existsSync} from "node:fs";
import {dirname,join} from "node:path";
const R="/Users/ueli/Documents/semio/";
const iss=JSON.parse(readFileSync("census.json","utf8")).filter((i:string[])=>i[0]==="WS-MISSING-KEY");
const names=new Map(readFileSync("names.tsv","utf8").split("\n").map(l=>l.split("\t") as [string,string]));
const g=new Map<string,string[]>();
for(const i of iss){let d=dirname(i[1]);let root="?";while(d.length>R.length-1){const c=join(d,"Cargo.toml");if(c!==i[1]&&existsSync(c)&&/^\[workspace\]/m.test(readFileSync(c,"utf8"))){root=c.slice(R.length);break;}d=dirname(d);}const k=root+" :: "+i[2]+(names.has(i[2])?" (pkg exists at "+names.get(i[2])!.slice(0,60)+")":" (NO PKG)");g.set(k,[...(g.get(k)??[]),i[1].slice(R.length)]);}
for(const[k,v]of [...g].sort())console.log(v.length,k.slice(0,260),"\n    e.g.",v[0].slice(0,150));
