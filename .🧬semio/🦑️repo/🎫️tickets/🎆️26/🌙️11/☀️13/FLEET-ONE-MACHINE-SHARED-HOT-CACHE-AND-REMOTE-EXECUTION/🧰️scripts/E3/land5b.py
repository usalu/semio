import sys
U=sys.argv[1].rstrip('/')+'/'
def edit(path,pairs):
    s=open(path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:90],s.count(a))
        s=s.replace(a,b)
    open(path,'w').write(s)
edit(U+'🔏️key/🟦️.ts',[('/** 📥️ The selected environment the compilation can observe','''/** 🚫️ The command line without its incremental session (`-C incremental=DIR` in the three spellings): an incremental compilation names its object files with a name that differs on every run, so its rlib has other bytes every time and other bytes than a compilation without a session, while the key of the unit leaves the session out. A unit that is shared through the fleet is compiled without one. */
export function withoutIncrementalV1(args:readonly string[]):readonly string[]{
 const kept:string[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(argument==="-C"&&(args[index+1]??"").startsWith("incremental=")){index++;continue;}
  if(argument.startsWith("-Cincremental=")||argument.startsWith("-C=incremental="))continue;
  kept.push(argument);
 }
 return kept;
}

/** 📥️ The selected environment the compilation can observe''')])
edit(U+'📜️script.ts',[
('sha256V1,toolchainIdentityV1,','sha256V1,toolchainIdentityV1,withoutIncrementalV1,'),
('const compile=async(capture:boolean):Promise<Outcome>=>{','let shared=false;\n const compile=async(capture:boolean):Promise<Outcome>=>{'),
('let fleet=prepare(sensitive),rooted:readonly string[]|undefined;','let fleet=prepare(sensitive),rooted:readonly string[]|undefined;shared=fleet!==undefined;'),
('  try{return await run(settings.inner,[...argv,...linkRootArgumentsV1(','  try{return await run(settings.inner,[...(shared&&unit.incremental!==undefined&&!settings.focus.has(crate)?withoutIncrementalV1(argv):argv),...linkRootArgumentsV1('),
('   if(fleet&&entry&&gap.length>0){','   const incrementalBuild=unit.incremental!==undefined&&remote===undefined&&!(shared&&!settings.focus.has(crate));\n   if(fleet&&entry&&incrementalBuild){event("remote-skip",key,0,0,"incremental: the unit was compiled with an incremental session, whose bytes differ on every run");countSkipV1(workspaceOf,"incremental");await hold?.release();}\n   else if(fleet&&entry&&gap.length>0){')])
t=open(__import__('os').path.dirname(__import__('os').path.abspath(__file__))+'/incr-test.ts').read()
p=U+'🔏️key/🔗️sources/🧪️tests/🟦️.ts'
s=open(p).read()
i=s.index('afterAll(()=>{for')
s=s[:i]+t.lstrip('\n')+'\n'+s[i:]
s=s.replace('import {parseDepInfoV1} from "../../🟦️.ts";','import {parseDepInfoV1,withoutIncrementalV1} from "../../🟦️.ts";',1)
open(p,'w').write(s)
print("ok")
