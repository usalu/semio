const fs=require('fs');
const file='🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🦀️.rs';
const readme='🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/README.md';
if(fs.existsSync(readme))throw Error('decoder README already exists');
const before=fs.readFileSync(file,'utf8');let docs='# Original Decoder\n\n';
const after=before.split('\n').map(line=>{const m=line.match(/^(\s*)\/\/\/ (\S+) (.{50,})$/u);if(!m)return line;docs+=m[2]+' '+m[3]+'\n\n';return m[1]+'/// '+m[2]+' See `README.md`.'}).join('\n');
if(before===after)throw Error('no prose found');
const temp=file+'.guest-doc-edit';fs.writeFileSync(temp,after,{flag:'wx'});
if(fs.readFileSync(file,'utf8')!==before||fs.existsSync(readme)){fs.unlinkSync(temp);throw Error('concurrent change')}
fs.writeFileSync(readme,docs,{flag:'wx'});fs.renameSync(temp,file);
