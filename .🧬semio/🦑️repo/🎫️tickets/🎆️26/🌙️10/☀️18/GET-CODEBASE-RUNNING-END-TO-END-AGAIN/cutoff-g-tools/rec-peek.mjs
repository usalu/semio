import{readFileSync,readdirSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const D="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const name=process.argv[2]??readdirSync(D).find(n=>/^[0-9a-f]{24}\.json$/.test(n));
const text=brotliDecompressSync(readFileSync(D+name)).toString("utf8");const nl=text.indexOf("\n");const header=JSON.parse(text.slice(0,nl)),value=JSON.parse(text.slice(nl+1));
console.log(name,"header keys",Object.keys(header),"blocks",header.blocks?.length,"inline rows",header.observations.length,"valueBytes",text.length-nl);
console.log(JSON.stringify(header.blocks?.slice(0,2)));
console.log("value keys",Object.keys(value),"recipes",Object.keys(value.closureRecipes??{}).length,"recipeBytes",JSON.stringify(value.closureRecipes??{}).length,"inputsBytes",JSON.stringify(value.closureInputs??{}).length);
const k={};for(const r of header.observations)k[r.kind]=(k[r.kind]??0)+1;console.log("inline kinds",JSON.stringify(k));
console.log(JSON.stringify(header.observations.filter(r=>r.kind==="closure").slice(0,3)).slice(0,800));
