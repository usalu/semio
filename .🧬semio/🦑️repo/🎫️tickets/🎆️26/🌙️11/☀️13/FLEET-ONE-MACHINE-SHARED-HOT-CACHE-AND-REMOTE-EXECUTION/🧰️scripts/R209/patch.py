import sys
S=sys.argv[1]
def sub(f,old,new):
    p=S+"/"+f;t=open(p,encoding="utf8").read()
    assert t.count(old)==1,(f,old[:50],t.count(old))
    open(p,"w",encoding="utf8").write(t.replace(old,new))
sub("closures.new.ts",'function namesAt(proof:Proof,',
'''/** 🌀️ A process-private row log spill (`rl/rl-<pid>-<time>-<serial>.rows` beside the inference data of a graph writer): never a closure publication, member or reference, so the proof neither reads nor retires it. */
function spill(name:string):boolean{return /^inference\\/(?:(?:selections|contexts)\\/[a-f0-9]{64}\\/)?rl\\/rl-[0-9]+-[0-9a-z]+-[0-9]+\\.rows$/u.test(name);}
function namesAt(proof:Proof,''')
sub("closures.new.ts",'if(path===root&&P.ignoredFiles.includes(name as never)){','if(spill(rel)){if(!row.isFile())throw Error("Owned row log spill file required");continue;}if(path===root&&P.ignoredFiles.includes(name as never)){')
sub("rowlog.new.mjs",'/** 🧹️ Removes the logs of dead',
'''/** 🏷️ The file name of the row log one process opens: `rl-<pid>-<time in base 36>-<serial>.rows`, the shape the closure proof of the graph store knows as a private spill. */
export const rowLogFileNameV1=(pid,time,serial)=>"rl-"+pid+"-"+time.toString(36)+"-"+serial+EXTENSION;
const alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error?.code!=="ESRCH";}};
/** 🧹️ Removes the logs of dead''')
sub("rowlog.new.mjs",'try{if(now-statSync(path).mtimeMs>P.staleSeconds*1000)rmSync(path,{force:true});}catch{}',
 'try{const owner=/^rl-([0-9]+)-/u.exec(name);if(now-statSync(path).mtimeMs>P.staleSeconds*1000||owner&&Number(owner[1])!==process.pid&&!alive(Number(owner[1])))rmSync(path,{force:true});}catch{}')
sub("rowlog.new.d.mts",'/** 🗃️ Creates the log',
 '/** 🏷️ The file name of the row log one process opens. */\nexport function rowLogFileNameV1(pid:number,time:number,serial:number):string;\n/** 🗃️ Creates the log')
sub("discovery.new.ts",'import {createRowLogV1,type RowLogListV1}','import {createRowLogV1,rowLogFileNameV1,type RowLogListV1}')
sub("discovery.new.ts",'"rl-"+process.pid+"-"+Date.now().toString(36)+"-"+(rowLogSerialV1++)+".rows"','rowLogFileNameV1(process.pid,Date.now(),rowLogSerialV1++)')
