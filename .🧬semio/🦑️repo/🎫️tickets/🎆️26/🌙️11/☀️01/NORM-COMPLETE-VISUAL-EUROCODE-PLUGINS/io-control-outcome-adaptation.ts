import fs from "node:fs/promises";
const base="🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/";
function endBrace(s,start){let depth=0,quote=false,escape=false;for(let i=start;i<s.length;i++){const c=s[i];if(quote){if(escape)escape=false;else if(c==="\\")escape=true;else if(c==='"')quote=false;continue}if(c==='"'){quote=true;continue}if(c==="{")depth++;if(c==="}"&&!--depth)return i;}throw Error("brace")}
for(const name of ["🦀️.rs","nested-io.rs"]){let s=await fs.readFile(base+name,"utf8");s=s.replace(/\.(receive|receive_nested|drive_cursor)::<([^>]*(?:>[^>(]*)?)>/g,(all,method,args)=>all);s=s.replaceAll("::<String,String>","::<String,String,_>").replaceAll("::<Vec<String>,String>","::<Vec<String>,String,_>").replaceAll("::<(),()>","::<(),(),_>").replaceAll("::<String,()>","::<String,(),_>").replaceAll("::<Vec<u8>,Vec<u8>>","::<Vec<u8>,Vec<u8>,_>");
const patches=[];const re=/\.(receive|receive_nested|drive_cursor)(?:::<[^\n]*?>)?\([^\n]*?\|[^|\n]*\|\{/g;let m;while((m=re.exec(s))){const start=re.lastIndex-1,end=endBrace(s,start);patches.push({start,end,kind:m[1]});re.lastIndex=start+1;}
for(const p of patches.sort((a,b)=>b.start-a.start)){const wrap=p.kind==="drive_cursor"?"fixture_cursor_turn":"super::native_snapshot_turn";s=s.slice(0,p.end+1)+")"+s.slice(p.end+1);s=s.slice(0,p.start)+wrap+"(|work|"+s.slice(p.start);}
await fs.writeFile(base+name,s);
}
