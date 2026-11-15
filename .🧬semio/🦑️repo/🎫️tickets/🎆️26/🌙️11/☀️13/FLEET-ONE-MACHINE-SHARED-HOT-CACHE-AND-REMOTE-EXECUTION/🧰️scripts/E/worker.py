import glob,sys
R=glob.glob("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution")[0]
root=sys.argv[1] if len(sys.argv)>1 else R
def edit(rel,pairs):
    p=root+"/"+rel
    s=open(p).read()
    for a,b in pairs:
        assert s.count(a)==1,(rel,a[:60],s.count(a))
        s=s.replace(a,b)
    open(p,"w").write(s)
edit("🔒️sandbox/🟦️.ts",[
 ("root:string;network:boolean}>;","root:string;network:boolean;readable?:readonly string[]}>;"),
 ("export function remoteExecutionDarwinProfileV1(root:string,network:boolean):string{","export function remoteExecutionDarwinProfileV1(root:string,network:boolean,readable:readonly string[]=[]):string{"),
 ("  `(allow file-read-data (subpath ${quote(root)}))`,\n","  `(allow file-read-data (subpath ${quote(root)}))`,\n  ...readable.map(path=>`(allow file-read-data (subpath ${quote(path)}))`),\n"),
 ("remoteExecutionDarwinProfileV1(realpathSync(spec.root),spec.network)","remoteExecutionDarwinProfileV1(realpathSync(spec.root),spec.network,(spec.readable??[]).map(path=>realpathSync(path)))"),
])
edit("👷️worker/🟦️.ts",[
 ('network?:"deny"|"allow";confinement','network?:"deny"|"allow";tools?:RemoteExecutionToolsV1;confinement'),
 ("blobs?:RemoteExecutionBlobCacheV1;network?:boolean;","blobs?:RemoteExecutionBlobCacheV1;network?:boolean;tools?:RemoteExecutionToolsV1;"),
 ("/** 🎬️ What executing one action needs besides the store. */","/** 🛠️ The tools a worker owns by name: an action that declares `SEMIO_TOOL_<NAME>=@tool` in its environment gets the path of the tool of that name, and its confined process may read the listed directories. The toolchain itself is never shipped, a worker that lacks the tool refuses the action with a precondition failure. */\nexport type RemoteExecutionToolsV1=Readonly<Record<string,Readonly<{path:string;readable:readonly string[]}>>>;\n\n/** 🎬️ What executing one action needs besides the store. */"),
 ("  if(env.TMPDIR===undefined&&process.platform!==\"win32\")env.TMPDIR=scratch;\n","  if(env.TMPDIR===undefined&&process.platform!==\"win32\")env.TMPDIR=scratch;\n  const readable:string[]=[];\n  for(const [name,value] of Object.entries(env)){\n   if(value!==\"@tool\"||!name.startsWith(\"SEMIO_TOOL_\"))continue;\n   const tool=run.tools?.[name.slice(\"SEMIO_TOOL_\".length).toLowerCase()];\n   if(!tool)return fail(9,`This worker does not offer the tool ${name.slice(\"SEMIO_TOOL_\".length).toLowerCase()}`);\n   env[name]=tool.path;readable.push(...tool.readable);\n  }\n"),
 ("network:run.network===true})","network:run.network===true,...(readable.length>0?{readable}:{})})"),
 ("signal:controller.signal,...(options.keepWork?{keepWork:true}:{}),blobs:store}","signal:controller.signal,...(options.keepWork?{keepWork:true}:{}),...(options.tools?{tools:options.tools}:{}),blobs:store}"),
])
print("ok")
