const fs=require('fs');
for (const p of process.argv.slice(2)) {
  const d=JSON.parse(fs.readFileSync(p,'utf8')).asset;
  console.log(p, d.dimensions, 'viewpoints', d.presentation.viewpoints);
  for (const b of d.rig.bones) console.log(' bone', b.id, b.parent, b.x,b.y,b.angleDeg,b.lengthM,b.joint.kind||JSON.stringify(b.joint));
  console.log(' poses', d.poses.map(x=>x.id+(x.key?'*':'')).join(','), ' clips', d.clips.map(c=>c.id).join(','));
  for (const k of Object.keys(d.representations)) { const r=d.representations[k]; console.log(' rep',k,r.status, r.status==='authored'?Object.keys(r.geometry):'', r.status==='authored'&&r.geometry.elevation.length); }
}
