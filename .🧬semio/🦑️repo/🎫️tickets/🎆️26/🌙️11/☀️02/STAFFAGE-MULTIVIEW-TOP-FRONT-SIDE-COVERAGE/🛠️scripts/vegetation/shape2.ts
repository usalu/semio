const a = JSON.parse(await Bun.file(process.argv[2]).text()).asset;
console.log(JSON.stringify(a.dimensions), JSON.stringify(a.rig.bones.slice(0,6)));
const r=a.representations["100"];
console.log(JSON.stringify(r.validRange),JSON.stringify(r.pens));
for(const v of ["elevation","front","plan"]){ const g=r.geometry[v]; console.log(v, Array.isArray(g)?g.length:Object.keys(g)); console.log(JSON.stringify(g).slice(0,1500)); }
console.log(JSON.stringify(r.decisions).slice(0,1500));
console.log(JSON.stringify(a.symbols[0]).slice(0,2500));
console.log(JSON.stringify(a.clips[0]).slice(0,2500));
console.log(JSON.stringify(a.clips[0].tracks).slice(0,1500));
console.log(JSON.stringify(a.thermal.regions[0]).slice(0,800), JSON.stringify(a.thermal.emitters));
