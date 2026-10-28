const fs=require('fs'),cp=require('child_process'),path=require('path');
const ticket='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-OPTIMIZATION-VIDEOS', out=ticket+'/🗑️generated/encoding', base='.🧬semio/🎓️teaching/⚡️cache/videos/de/speaker';
fs.mkdirSync(out,{recursive:true});
const topic=process.argv[2], file=base+'/rendered/Optimization_'+topic+'_speaker_de_1080p30.mp4', cert=file.replace('.mp4','.render.json');
if(!fs.existsSync(cert)){console.log('pending '+topic);process.exit(0);}
const result=cp.spawnSync('repo',['tools','exec','--light','ffmpeg','--','-hide_banner','-i',file,'-f','null','-'],{shell:true,encoding:'utf8',env:{...process.env,SEMIO_TOOL_FFMPEG:path.resolve('.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe')},maxBuffer:10*1024*1024});
fs.writeFileSync(out+'/'+topic+'.log',result.stdout+result.stderr);
const text=result.stdout+result.stderr,d=text.match(/Duration: (\d+):(\d+):(\d+\.\d+)/), seconds=d?+d[1]*3600+ +d[2]*60+ +d[3]:null;
const value={topic,file,bytes:fs.statSync(file).size,seconds,certificate:fs.statSync(cert).size,exit:result.status,video:text.match(/Stream.*Video:[^\r\n]+/)?.[0],audio:text.match(/Stream.*Audio:[^\r\n]+/)?.[0]};
fs.writeFileSync(out+'/'+topic+'.json',JSON.stringify(value,null,2)); console.log(value);