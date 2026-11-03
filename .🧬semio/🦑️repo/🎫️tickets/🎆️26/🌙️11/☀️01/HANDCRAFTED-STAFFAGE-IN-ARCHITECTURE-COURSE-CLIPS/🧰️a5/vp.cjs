const fs=require('fs');
for (const c of fs.readdirSync('.')) { if(!fs.statSync(c).isDirectory()) continue;
 for (const a of fs.readdirSync(c)) { const p=c+'/'+a+'/🔣️.json'; if(!fs.existsSync(p)) continue;
  const d=JSON.parse(fs.readFileSync(p,'utf8')).asset;
  const rp=Object.entries(d.representations).filter(([k,r])=>r.status==='authored'&&r.geometry.plan&&r.geometry.plan.length).map(([k,r])=>k+':'+r.geometry.plan.length);
  const sp=d.symbols.filter(s=>s.geometry.plan&&s.geometry.plan.length).map(s=>s.id);
  if (d.presentation.viewpoints.includes('plan')||rp.length||sp.length) console.log(a, d.presentation.viewpoints.join('/'), 'reps', rp.join(','), 'syms', sp.join(','));
 }}
