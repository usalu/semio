import sys,os
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
def edit(path,pairs,out=None):
    t=open(path).read()
    for old,new in pairs:
        if t.count(old)!=1: raise SystemExit("anchor count %d: %s"%(t.count(old),old[:80]))
        t=t.replace(old,new)
    open(out or path,"w").write(t)

TRACE=("const sourceTraceKeyV1=Symbol.for('semio.sourceTrace.v1'),sourceTraceSeenV1=new Set(),sourceTraceBufferV1=[],sourceTraceSinkV1={file:undefined},sourceTraceRelativeV1=name=>{const relative=path.relative(value.root,name);return relative&&!relative.startsWith('..')&&!path.isAbsolute(relative)?relative.split(path.sep).join('/'):name;},sourceTraceAppendV1=fs.appendFileSync;\n"
"globalThis[sourceTraceKeyV1]=Object.freeze({record:(kind,name)=>{const text=kind+' '+(kind==='a'?name:sourceTraceRelativeV1(name));if(sourceTraceSeenV1.has(text))return;sourceTraceSeenV1.add(text);if(sourceTraceSinkV1.file===undefined){if(sourceTraceBufferV1.length<65536)sourceTraceBufferV1.push(text);return;}if(sourceTraceSinkV1.file!==null)try{sourceTraceAppendV1(sourceTraceSinkV1.file,text+'\\n');}catch{}},enable:directory=>{if(sourceTraceSinkV1.file!==undefined)return;try{fs.mkdirSync(directory,{recursive:true});const file=path.join(directory,process.pid+'-'+Date.now()+'.trace');fs.writeFileSync(file,'s '+JSON.stringify({argv:process.argv.slice(2),entry:value.entry,files:value.files.length,sha256:expected})+'\\n'+sourceTraceBufferV1.map(line=>line+'\\n').join(''));sourceTraceBufferV1.length=0;sourceTraceSinkV1.file=file;}catch{sourceTraceSinkV1.file=null;}}});\n"
"const sourceTraceV1=globalThis[sourceTraceKeyV1];\n")

def loader(out=None):
    edit(L+"/📥️source/🔏️closure/🍞️bun/🟦️.ts",[
     ("const originalRead=fs.readFileSync,codeDirectories=",TRACE+"const originalRead=fs.readFileSync,codeDirectories="),
     ("if(!bytes)throw Error('Uncaptured implementation module refused: '+name);","if(!bytes)throw Error('Uncaptured implementation module refused: '+name);sourceTraceV1.record('m',name);"),
     ("currentImage(file,bytes);const encoding=","currentImage(file,bytes);sourceTraceV1.record('r',file);const encoding="),
     ("return originalRead(name,options);};","if(file)sourceTraceV1.record('x',file);return originalRead(name,options);};"),
     ("return promises.readFile(name,options);}}","if(file)sourceTraceV1.record('x',file);return promises.readFile(name,options);}}"),
    ],out)

def revision():
    edit(L+"/📥️inputs/🔏️revision/🟨️.mjs",[
     ("function capture(path,maximumBytes,retainText){return retryOnConcurrentEditSyncV1(","function capture(path,maximumBytes,retainText){globalThis[Symbol.for(\"semio.sourceTrace.v1\")]?.record(\"p\",path);return retryOnConcurrentEditSyncV1("),
    ])

EXEC='''/** 🔭️ Opens the opt-in source trace of this process when the caller named a directory: it records which captured modules and Sources an action class really touches, the corpus that proves a narrow implementation scope. */
function traceBazelImplementationV1(environment:NodeJS.ProcessEnv,task:BazelTaskV1,command:string):void{
 const directory=environment.SEMIO_BUILD_SOURCE_TRACE,trace=(globalThis as {[key:symbol]:{enable(directory:string):void;record(kind:string,text:string):void}|undefined})[Symbol.for("semio.sourceTrace.v1")];
 if(!directory||!trace)return;
 const semio=(task.definition.metadata as {semio?:{execution?:{kind?:string}}}|undefined)?.semio;
 trace.enable(directory);trace.record("a",JSON.stringify({command,id:task.id,workerKind:workerActionKindV1(workerPolicy,task.target,task.definition.continuous===true),executionKind:task.definition.execution.kind,semioKind:semio?.execution?.kind??null}));
}

'''
def executor():
    edit(L+"/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts",[
     ("/** 🧊️ Binds the captured proof of this process","%s/** 🧊️ Binds the captured proof of this process"%EXEC),
     ("const environment=(await readBazelRuntimeV1(manifestPath,implementation,initial)).environment,expected=","const environment=(await readBazelRuntimeV1(manifestPath,implementation,initial)).environment,expected="),
     ("const birth={kind,task:manifest.task.id,","traceBazelImplementationV1(environment,manifest.task,kind);const birth={kind,task:manifest.task.id,"),
     ("runtime=await readBazelRuntimeV1(manifestPath,implementation,checkBirth),environment=runtime.environment;","runtime=await readBazelRuntimeV1(manifestPath,implementation,checkBirth),environment=runtime.environment;traceBazelImplementationV1(environment,manifest.task,command);"),
    ])

if __name__=="__main__":
    what=sys.argv[1]
    if what=="loader-copy": loader(L+"/📥️source/🔏️closure/🍞️bun/🟦️.ckey2.ts")
    elif what=="loader": loader()
    elif what=="revision": revision()
    elif what=="executor": executor()
