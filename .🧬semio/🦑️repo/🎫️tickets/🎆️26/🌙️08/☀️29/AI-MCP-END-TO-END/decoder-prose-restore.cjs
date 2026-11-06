const fs=require('fs');
const file='🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🦀️.rs';
const readme='🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/README.md';
const before=fs.readFileSync(file,'utf8'), docs=fs.readFileSync(readme,'utf8').split('\n').slice(2).filter(Boolean);let index=0;
const after=before.split('\n').map(line=>{const m=line.match(/^(\s*)\/\/\/ (\S+) See `README.md`\.$/u);if(!m)return line;const text=docs[index++];if(!text?.startsWith(m[2]+' '))throw Error('prose mismatch');return m[1]+'/// '+text}).join('\n');
if(index!==docs.length)throw Error('unrestored prose');const tmp=file+'.guest-doc-restore';fs.writeFileSync(tmp,after,{flag:'wx'});if(fs.readFileSync(file,'utf8')!==before){fs.unlinkSync(tmp);throw Error('concurrent source change')}fs.renameSync(tmp,file);fs.unlinkSync(readme);
