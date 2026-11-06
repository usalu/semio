const fs=require("node:fs"),{spawnSync}=require("node:child_process"),path=require("node:path");
const result=JSON.parse(fs.readFileSync(path.join(__dirname,"🗑️generated/raster-bindings.json"),"utf8")),files=result.paths.filter(file=>file.endsWith(".rs"));
const run=spawnSync("repo",["tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustfmt","--emit","stdout","--edition","2021","--config","skip_children=true",...files],{stdio:["ignore","ignore","pipe"],encoding:"utf8",maxBuffer:1048576});
if(run.status!==0)throw Error(run.error?.message??run.stderr);console.log(JSON.stringify({files:files.length,status:run.status}));
