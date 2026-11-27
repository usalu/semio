import{readFileSync}from"node:fs";import{resolve}from"node:path";import{TOML}from"bun";
const root="/Users/ueli/Documents/semio";const U=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/");
const locks:string[]=JSON.parse(readFileSync(resolve(U,"📋️locks.json"),"utf8"));const rec=JSON.parse(readFileSync(resolve(U,"📋️feature-consumers.json"),"utf8"));
const bad:Record<string,string[]>={};
for(const p of locks){const l:any=TOML.parse(readFileSync(resolve(root,p),"utf8"));const pk=(n:string)=>l.package.filter((x:any)=>x.name===n);const why:string[]=[];
if(pk("semio-plugin-registration").length!==1)why.push("registration "+pk("semio-plugin-registration").length);
const c=pk("semio-plugin-testing-laws")[0];if(c&&JSON.stringify(c.dependencies)!==JSON.stringify(rec.dependencies.filter((n:string)=>p==="Cargo.lock"||n!=="semio-framework-value-derive")))why.push("testing-laws deps differ");
if(why.length)bad[p]=why;}
const g:Record<string,string[]>={};for(const[p,w]of Object.entries(bad))(g[w.join("; ")]??=[]).push(p);
for(const[k,v]of Object.entries(g))console.log(k,v.length,v.slice(0,6));require("fs").writeFileSync(process.argv[2]??"/dev/null",Object.keys(bad).join("\n")+"\n");console.log("locks",locks.length,"bad",Object.keys(bad).length);
