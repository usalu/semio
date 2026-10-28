import sys
p=sys.argv[1]+"/files/farm-close.ts"
s=open(p,encoding="utf-8").read()
s=s.replace("write:line=>lines.push(line)","write:(line:string)=>lines.push(line)")
s=s.replace("farm.onReady(total=>{if(total>0&&children.length===2)done(total);})","farm.onReady((total:number)=>{if(total>0&&children.length===2)done(total);})")
open(p,"w",encoding="utf-8").write(s)
