const m=require(process.argv[2]);
for (const a of m.assets){
 const bones=a.rig?.bones||[];
 const jt={};bones.forEach(b=>{const t=b.joint?.type||'none';jt[t]=(jt[t]||0)+1});
 console.log(`## ${a.id} | ${a.title?.en} | rigKind=${a.rig?.kind} bones=${bones.length} joints=${JSON.stringify(jt)}`);
 console.log(`S: ${a.summary?.en}`);
 console.log(`BONES: ${bones.map(b=>b.id).join(',')}`);
 console.log(`ANCHORS: ${(a.anchors||[]).map(x=>x.id+'('+x.kind+'/'+(x.support||'')+')').join(',')}`);
 console.log(`POSES: ${(a.poses||[]).map(p=>p.id+'='+p.title?.en).join(' | ')}`);
 console.log(`CLIPS: ${(a.clips||[]).map(c=>c.id+'='+c.title?.en+'('+c.durationS+'s'+(c.loop?',loop':'')+')').join(' | ')}`);
 console.log(`THERMAL: ${(a.thermal?.regions||[]).map(r=>r.id+':'+r.model).join(',')} | STRUCT: ${a.structure?.dynamics} | PRES: ${a.presentation?.defaultPose}`);
 console.log('');
}
