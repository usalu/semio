const fs=require('node:fs');
const ticket='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/';
const audit=JSON.parse(fs.readFileSync(ticket+'rust-extern-namespace-audit.json','utf8'));
const rows=audit.findings.filter(row=>row.status==='matching_direct_dependency'&&(row.file.includes('🧠️interpreter')||row.file.includes('🌱️ui-arena')));
for(const file of new Set(rows.map(row=>row.file))){
 const before=fs.readFileSync(file,'utf8');let after=before;
 for(const row of rows.filter(row=>row.file===file))after=after.replace(new RegExp(`\\b${row.old}(?=::)`,'g'),row.candidate);
 if(after===before)continue;
 const temp=ticket+'🗑️generated/extern-'+rows.findIndex(row=>row.file===file)+'.tmp';
 fs.mkdirSync(require('node:path').dirname(temp),{recursive:true});fs.writeFileSync(temp,after);
 if(fs.readFileSync(file,'utf8')!==before)throw Error(`concurrent edit in ${file}`);
 fs.renameSync(temp,file);console.log(file);
}
