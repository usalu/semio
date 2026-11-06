const fs=require('fs'),crypto=require('crypto');
const base='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/';
const paths=['🔌️owned-semio-boundary/🦀️.rs','🏛️instance-state/🦀️.rs','🏗️instance/🧪️tests/🦀️.rs','🧮️numeric-execution/🦀️.rs','🦀️.rs','⚙️instruction-execution/🦀️.rs','📸️checkpoint-codec/🦀️.rs'];
const documents=paths.map(path=>{const full=base+path,old=fs.readFileSync(full,'utf8'),lines=old.split('\n'),notes=lines.filter(line=>/^\s*\/\/(?:\/| )/.test(line)),next=lines.filter(line=>!/^\s*\/\/(?:\/| )/.test(line)).join('\n');return{full,old,next,path,notes}});
if(documents.some(row=>!row.notes.length))throw Error('unexpected comment roster');
const readme=base+'README.md';if(fs.existsSync(readme))throw Error('interpreter README already exists');
const text='# Interpreter Source Notes\n\n'+documents.map(row=>'## '+row.path+'\n\n'+row.notes.map(line=>line.replace(/^\s*\/\/\/?\s?/,'' )).join('\n')+'\n').join('\n');
fs.writeFileSync(readme,text,{flag:'wx'});
for(const{full,old,next}of documents){const temp=full+'.docs-'+crypto.randomBytes(4).toString('hex');fs.writeFileSync(temp,next,{flag:'wx'});if(fs.readFileSync(full,'utf8')!==old){fs.unlinkSync(temp);throw Error('concurrent source change: '+full)}fs.renameSync(temp,full);}
