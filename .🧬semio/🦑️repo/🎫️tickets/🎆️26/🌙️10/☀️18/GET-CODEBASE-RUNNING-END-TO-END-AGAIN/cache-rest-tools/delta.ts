const a = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const b = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census-after.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
const am = new Map(a.targets.filter((t:any)=>own.has(t.owner)).map((t:any)=>[t.file+"::"+t.name,t]));
const c:any={falseToTrue:0,absentToTrue:0,trueToFalse:0,absentToFalse:0,outputsAdded:0,inputsAdded:0}; const byFile:any={};
for (const t of b.targets) { if(!own.has(t.owner))continue; const o:any=am.get(t.file+"::"+t.name); if(!o)continue;
  const k=(x:string,y:string)=>o.cache===x&&t.cache===y;
  let touched=false;
  if(k("false","true")){c.falseToTrue++;touched=true} if(k("absent","true")){c.absentToTrue++;touched=true} if(k("true","false")){c.trueToFalse++;touched=true} if(k("absent","false")){c.absentToFalse++;touched=true}
  if(o.outputs.kind==="absent"&&t.outputs.kind!=="absent"){c.outputsAdded++;touched=true} if(o.inputs.declaredKind==="absent"&&t.inputs.declaredKind!=="absent"){c.inputsAdded++;touched=true}
  if(touched) byFile[t.file]=(byFile[t.file]??0)+1; }
console.log(JSON.stringify(c)); console.log(Object.keys(byFile).length+" manifests with changed targets");
