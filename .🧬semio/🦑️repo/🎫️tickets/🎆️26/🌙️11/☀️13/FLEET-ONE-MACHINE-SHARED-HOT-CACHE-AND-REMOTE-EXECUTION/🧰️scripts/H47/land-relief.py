import os,sys,re
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts"
s=open(p,encoding="utf8").read()
def rep(old,new):
    global s
    if s.count(old)!=1: sys.exit("anchor %d: %s"%(s.count(old),old[:80]))
    s=s.replace(old,new)
rep('run(stage:ReadyStageV1,options:Readonly<{signal:AbortSignal;jobs:number;onLine(line:string):void;onStage?(stage:string,adopted:boolean):void;onActivity?():void;mirror?:ReadyLogMirrorV1}>)',
    'run(stage:ReadyStageV1,options:Readonly<{signal:AbortSignal;jobs:number;onLine(line:string):void;onStage?(stage:string,adopted:boolean):void;onActivity?():void;mirror?:ReadyLogMirrorV1;collectImage?:(image:ReadyImageShortV1,signal:AbortSignal)=>Promise<void>}>)')
rep('result=await ports.run(row,{signal,jobs,onStage,onActivity,onLine:line=>{status.update({tail:[line.slice(0,P.tailCharacters)]});},',
    'result=await ports.run(row,{signal,jobs,onStage,onActivity,onLine:line=>{status.update({tail:[line.slice(0,P.tailCharacters)]});},...ports.recover===undefined?{}:{collectImage:async(image,collectSignal)=>{await ports.recover!({stage:row.name,short:image,relief:true,signal:collectSignal,report:()=>{}});}},')
i=s.index("collectImage:async(image,signal)=>{try{")
j=s.index("catch{}},",i)+len("catch{}},")
s=s[:i]+s[j:]
rep('writers=image&&request.short.limitedByHost!==true?readyImageWritersV1(','writers=image&&(request.short.limitedByHost!==true||request.relief===true)?readyImageWritersV1(')
rep('if(request.relief===true)return{fits:true,consumers:[]};','if(request.relief===true&&readyImageStoreV1(request.short.path)===request.short.path)return{fits:true,consumers:[]};')
t=p+".h47tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,p);print("relief landed")
