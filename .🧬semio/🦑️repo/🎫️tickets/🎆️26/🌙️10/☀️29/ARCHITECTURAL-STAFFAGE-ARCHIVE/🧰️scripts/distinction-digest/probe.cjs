const fs=require('fs');
const [file,ids,what]=[process.argv[2],process.argv[3].split(','),process.argv[4]];
const m=JSON.parse(fs.readFileSync(file,'utf8'));
for(const a of m.assets){ if(!ids.includes(a.id)) continue;
 console.log('##',a.id);
 if(what==='bones') for(const b of a.rig.bones) console.log(' B',b.id,'<-',b.parent,b.joint?.type,JSON.stringify(b.joint||{}).slice(0,160),'len',b.lengthM,b.mass?'m'+b.mass.kg:'');
 if(what==='anchors') for(const x of a.anchors) console.log(' A',x.id,x.bone,x.kind,x.support||'',x.title?.en||'');
 if(what==='poses') for(const p of a.poses) console.log(' P',p.id,'contacts=',JSON.stringify(p.contacts),'ground=',p.ground,'bones=',Object.keys(p.bones||{}).length,'| desc:',(p.description?.en||'').slice(0,200));
 if(what==='clips') for(const c of a.clips) console.log(' C',c.id,'|',(c.description?.en||'').slice(0,260));
 if(what==='struct') console.log(' S',JSON.stringify(a.structure).slice(0,1400));
 if(what==='thermal') console.log(' T',JSON.stringify(a.thermal).slice(0,1500));
}
