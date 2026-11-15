from lib import *
P="🔁️replica/🟦️.ts"
def code(t):
    s=read(src(P))
    t=sub(t,'import {compileIgnoreV1,gitPrivateV1,hashFileV1,','import {compileIgnoreV1,foldNameV1,gitPrivateV1,hashFileV1,')
    t=sub(t,'type AttesterGitV1,type AttesterViewV1,type FleetReportV1}','type AttesterGitV1,type AttesterViewV1,type FleetGuardsV1,type FleetReportV1}')
    t=sub(t,'import {applyIndexV1,branchRefV1,linearizeRefV1,gitStateV1,gitViewV1,','import {applyIndexV1,branchRefV1,linearizeRefV1,gitGuardsV1,gitStateV1,gitViewV1,')
    t=sub(t,'const windows=process.platform==="win32",originPattern=','const originPattern=')
    t=sub(t,'this.folded[id]=hashText(path.toLowerCase());','this.folded[id]=hashText(foldNameV1(path));')
    t=sub(t,'const fold=path.toLowerCase(),hash=hashText(fold),mask=this.slots.length-1;','const fold=foldNameV1(path),hash=hashText(fold),mask=this.slots.length-1;')
    t=sub(t,'this.book.name(held-1).toLowerCase()===fold)return this.book.name(held-1);','foldNameV1(this.book.name(held-1))===fold)return this.book.name(held-1);')
    t=sub(t,'if((fold??=this.book.name(id).toLowerCase())===this.book.name(held-1).toLowerCase()){','if((fold??=foldNameV1(this.book.name(id)))===foldNameV1(this.book.name(held-1))){')
    t=sub(t,'persistMilliseconds:number;now:()=>number}>;','persistMilliseconds:number;modeless:boolean;now:()=>number}>;')
    t=sub(t,'persistMilliseconds:options.persistMilliseconds??5000,now:options.now??Date.now};','persistMilliseconds:options.persistMilliseconds??5000,modeless:options.modeless??process.platform==="win32",now:options.now??Date.now};')
    t=sub(t,'persistMilliseconds?:number;','persistMilliseconds?:number;modeless?:boolean;')
    t=sub(t,'gitView?:(worktree:string,signal?:AbortSignal)=>Promise<AttesterViewV1','gitGuards?:(worktree:string,signal?:AbortSignal)=>Promise<FleetGuardsV1|null>;guardsMilliseconds?:number;guardsMinimumMilliseconds?:number;gitView?:(worktree:string,signal?:AbortSignal)=>Promise<AttesterViewV1')
    t=sub(t,"/** 🎚️ The explicit inputs of every replica operation: both directories, the identity of this machine and the tunables that default to the topology's `replica` block.","/** 🎚️ The explicit inputs of every replica operation: both directories, the identity of this machine and the tunables that default to the topology's `replica` block. `modeless` (default: Windows) says that the volume cannot represent the executable bit: such a replica never reads the bit from disk, keeps the bit of the manifest entry when a file changes, takes it from the staged index entry for a new file and never writes it.")
    t=sub(t,'  const executable=windows?(current?.executable??false):entry.executable;','  const executable=replica.settings.modeless?(current?.operation==="put"?current.executable!:stagedExecutable(replica,entry.path)):entry.executable;')
    t=sub(t,'async function prepare(replica:ReplicaV1,job:Job,signal?:AbortSignal)',between(s,"/** 🔑️ The executable bit","async function prepare(")+'async function prepare(replica:ReplicaV1,job:Job,signal?:AbortSignal)')
    t=sub(t,'executable:!windows&&(info.mode&0o111)!==0});','executable:!replica.settings.modeless&&(info.mode&0o111)!==0});')
    t=sub(t,'&&(windows||files.executableAt(id)===entry.executable);','&&(replica.settings.modeless||files.executableAt(id)===entry.executable);')
    t=sub(t,'!racy(current)&&(windows||current.executable===entry.executable)','!racy(current)&&(replica.settings.modeless||current.executable===entry.executable)')
    t=sub(t,'if(entry.operation==="put"&&!windows&&((here!.mode&0o111)!==0)!==entry.executable)','if(entry.operation==="put"&&!replica.settings.modeless&&((here!.mode&0o111)!==0)!==entry.executable)')
    t=sub(t,'mode:!windows&&entry.executable?0o755:0o644,clone:true','mode:!replica.settings.modeless&&entry.executable?0o755:0o644,clone:true')
    t=sub(t,'(windows||files.executableAt(id)===((info.mode&0o111)!==0))','(replica.settings.modeless||files.executableAt(id)===((info.mode&0o111)!==0))')
    # publishStage
    t=sub(t,'''  try{await publishIndexV1(indexHost(replica),signal);}
  catch(error){if(signal?.aborted)throw error;noteOnce(replica,`error:git:${messageOf(error)}`,{event:"error",stage:"git",message:messageOf(error)});}
 }
 await persist(replica,false);''','''  await publishStage(replica,signal);
 }
 await persist(replica,false);''')
    fn=between(s,"/** 🎭️ Publishes the stage","/** 📤️ One publish round:")
    t=sub(t,'/** 📤️ One publish round:',fn+'/** 📤️ One publish round:')
    k=t.index("export async function publishOnceV1(");e=t.index("{\n",k)+2
    t=t[:e]+' if(replica.settings.modeless&&(paths===undefined||paths.includes(".git/index")))await publishStage(replica,signal);\n'+t[e:]
    t=sub(t,'viewMilliseconds:options.viewMilliseconds,viewDebounceMilliseconds:options.viewDebounceMilliseconds,','viewMilliseconds:options.viewMilliseconds,viewDebounceMilliseconds:options.viewDebounceMilliseconds,guards:async signal=>{if(options.gitGuards!==undefined)return options.gitGuards(settings.worktree,signal);const found=await gitGuardsV1(settings.worktree,{signal,modeless:settings.modeless});if(found===null)return null;const {samples:_,...guards}=found;return guards;},guardsMilliseconds:options.guardsMilliseconds,guardsMinimumMilliseconds:options.guardsMinimumMilliseconds,')
    return t
edit(P,code)
