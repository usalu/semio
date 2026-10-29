const fs=require('fs');const [file,ids]=[process.argv[2],process.argv[3].split(',')];
const m=JSON.parse(fs.readFileSync(file,'utf8'));
for(const a of m.assets){ if(!ids.includes(a.id)) continue;
 const s=a.structure; console.log('##',a.id,'dyn',s.dynamics,'supports',JSON.stringify(s.supports));
 console.log('  checks', JSON.stringify(s.checks||s.governing||Object.keys(s)).slice(0,500));
 console.log('  assumptions', (s.assumptions||[]).map(x=>x.en).join(' || ').slice(0,700));
}
