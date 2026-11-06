const fs=require('fs');
const file='🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs', readme='🧰️framework/🔨️modules/🧵️job/⏱️context/README.md';
const before=fs.readFileSync(file,'utf8'),oldDocs=fs.readFileSync(readme,'utf8');let docs='\n## Additional Context APIs\n\n';
const after=before.split('\n').map(line=>{const m=line.match(/^(\s*)\/\/\/ (\S+) (.{50,})$/u);if(!m||m[3].startsWith('See `README.md`'))return line;docs+=m[2]+' '+m[3]+'\n\n';return m[1]+'/// '+m[2]+' See `README.md`.'}).join('\n');
if(before===after)throw Error('no prose found');const tmp=file+'.guest-prose-room';fs.writeFileSync(tmp,after,{flag:'wx'});if(fs.readFileSync(file,'utf8')!==before||fs.readFileSync(readme,'utf8')!==oldDocs){fs.unlinkSync(tmp);throw Error('concurrent change')}fs.writeFileSync(readme,oldDocs+docs);fs.renameSync(tmp,file);
