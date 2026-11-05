import{readFileSync,readdirSync,statSync,writeFileSync}from"node:fs";
import{join}from"node:path";
const root=".cache/b/r/m/nx/artifacts/terminalOutputs";
const rows=readdirSync(root).map(name=>({file:join(root,name),stat:statSync(join(root,name))})).filter(row=>row.stat.isFile()&&row.stat.size<=8*1024*1024).sort((a,b)=>b.stat.mtimeMs-a.stat.mtimeMs);
const found=[];
for(const row of rows){const text=readFileSync(row.file,"utf8").replace(/\u001b\[[0-9;]*m/g,"");if(!/raster|Raster/.test(text))continue;const lines=text.split("\n"),selected=new Set<number>();for(let i=0;i<lines.length;i++)if(/error:|Error:|fail\)|failed|✗|SIGKILL|Cannot find|ENOENT|EACCES|EPERM/.test(lines[i]!))for(let j=Math.max(0,i-3);j<Math.min(lines.length,i+6);j++)selected.add(j);found.push({file:row.file,modified:new Date(row.stat.mtimeMs).toISOString(),bytes:row.stat.size,lines:[...selected].slice(0,120).map(index=>({line:index+1,text:lines[index]}))});}
writeFileSync(join(import.meta.dir,"📉️javascript-failure.md"),"# Raster JavaScript Failure Audit\n\nRead-only inspection of bounded Nx terminal output artifacts. Cache files are neither changed nor deleted. A cached failure is supporting diagnostic evidence, not a new test execution.\n\n"+found.map(row=>"## "+row.file+"\n\nModified: "+row.modified+"; bytes: "+row.bytes+".\n\n```text\n"+row.lines.map(line=>line.line+": "+line.text).join("\n")+"\n```").join("\n\n")+"\n");
console.log(JSON.stringify(found));
