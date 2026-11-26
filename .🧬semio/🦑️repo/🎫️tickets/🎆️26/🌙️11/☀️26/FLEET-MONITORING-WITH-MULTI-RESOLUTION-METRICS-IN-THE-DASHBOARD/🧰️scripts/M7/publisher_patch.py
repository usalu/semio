import os
root="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/"
def edit(path,pairs):
    p=root+path;t=open(p,encoding="utf8").read()
    for a,b in pairs:
        assert t.count(a)==1,a[:70]
        t=t.replace(a,b,1)
    tmp=p+".m7.tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,p)
edit("🔌️host/🟦️.ts",[('''  self:()=>{const usage=process.cpuUsage();''','''  cpuMicroseconds:()=>{const usage=process.cpuUsage();return usage.user+usage.system;},
  self:()=>{const usage=process.cpuUsage();''')])
edit("📤️publication/🟦️.ts",[
('import {closeSync,copyFileSync,mkdirSync,','import {closeSync,mkdirSync,'),
('''const TWO32=4294967296,HEAD=40,STAMP=R.stampBytes,IDENTITY=[[0,40],[48,56]] as const,CHUNK_ROWS=64;''','''const TWO32=4294967296,HEAD=40,STAMP=R.stampBytes,IDENTITY=[[0,40],[48,56]] as const,CHUNK_ROWS=64,COPY_BYTES=262144,APPLE_DOUBLE=P.appleDoublePrefix;'''),
('''const sameLayout=''','''const copyPlain=(source:string,target:string):void=>{
 const from=openSync(source,"r");
 try{
  const to=openSync(target,"w");
  try{const buffer=Buffer.allocUnsafe(COPY_BYTES);for(let at=0;;){const read=readSync(from,buffer,0,COPY_BYTES,at);if(read===0)break;writeSync(to,buffer,0,read);at+=read;}}
  finally{closeSync(to);}
 }finally{closeSync(from);}
};
const sameLayout='''),
('''   for(const name of readdirSync(input.to)){
    const found=''','''   for(const name of readdirSync(input.to)){
    if(name.startsWith(APPLE_DOUBLE)){try{rmSync(join(input.to,name),{force:true});removed.push(name);}catch{}continue;}
    const found='''),
('''   copyFileSync(source,temporary);
   const size''','''   copyPlain(source,temporary);
   const size'''),
('''   renameSync(temporary,target);
   return size;''','''   renameSync(temporary,target);
   try{rmSync(join(input.to,APPLE_DOUBLE+name),{force:true});}catch{}
   return size;'''),
])
