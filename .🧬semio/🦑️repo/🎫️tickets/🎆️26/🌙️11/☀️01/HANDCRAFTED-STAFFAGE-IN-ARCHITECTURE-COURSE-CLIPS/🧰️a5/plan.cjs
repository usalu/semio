const fs=require('fs');
const d=JSON.parse(fs.readFileSync(process.argv[2],'utf8')).asset;
const k=process.argv[3]||'100';
const r=d.representations[k];
console.log(JSON.stringify(r.pens), JSON.stringify(r.decisions,null,1));
console.log(r.geometry.plan.map(s=>JSON.stringify(s)).join('\n'));
