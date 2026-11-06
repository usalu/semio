const fs=require('fs'),f='🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs';const before=fs.readFileSync(f,'utf8');let s=before;
for(const[a,b]of[
 ['/// 🌉️ Runs one non-suspending async step inside one original native receiving scope and settles that scope as its own semantic receipt; a suspended step is refused instead of being dropped silently.','/// 🌉️ Runs one receiving step; see `🧪️testing/README.md`.'],
 ['/// 🚪️ Quotes one normal-work demand against the caller\'s remaining retained grant on every axis, independent of CPU fuel; a denied quote allocates and posts nothing.','/// 🚪️ Quotes normal work; see `🧪️testing/README.md`.']]){if(s.split(a).length!==2)throw Error(a);s=s.replace(a,b)}
if(fs.readFileSync(f,'utf8')!==before)throw Error('changed');let t=f+'.guest-prose2.tmp';fs.writeFileSync(t,s);if(fs.readFileSync(f,'utf8')!==before){fs.unlinkSync(t);throw Error('changed before rename')}fs.renameSync(t,f);
