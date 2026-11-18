import sys,os,json,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
FILES={"hm":L+"/🏠️home/🟦️.ts","ht":L+"/🏠️home/🧪️tests/🟦️.ts","lab":L+"/🟦️.ts"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
HM='''
/** 🏗️ The launcher of a lab machine built from the sources of the tree it reads (what the `native` stage does on a worker: it builds the launcher from the checkout, so a change of the Go sources reaches the machine): the member `📦️packages/🐹️go` files are copied into a private module, built offline (`GOWORK=off GOTOOLCHAIN=local GOPROXY=off CGO_ENABLED=0`) with the Go of the lab home, signed ad hoc on macOS, and written over the launcher binary of the home by removing it first (an in-place overwrite of a running binary is killed by macOS). Never touches the live account. `ok` false with a reason when no Go or no sources are there. */
export function sliceLabLauncherBuildV1(root:string,home:string,go?:string):Readonly<{ok:boolean;seconds:number;reason:string}>{
 const started=performance.now(),done=(ok:boolean,reason:string)=>({ok,seconds:round((performance.now()-started)/1000),reason});
 const module=join(root,"🧰️framework","🛍️products","🦑️repo","🔨️modules","🚀️launcher","📦️packages","🐹️go"),compiler=go??join(home,".local","go","bin","go"),target=join(home,".local","share","semio","launcher","bin","repo");
 if(!existsSync(compiler))return done(false,"no-go");
 if(!existsSync(join(module,"🐹️.go")))return done(false,"no-sources");
 const work=mkdtempSync(join(tmpdir(),"slw-lb-"));
 try{
  mkdirSync(join(work,"cmd"));writeFileSync(join(work,"go.mod"),"module github.com/usalu/semio/repo/launcher\\n\\ngo 1.25\\n");
  let index=0;
  for(const source of[join(module,"🐹️.go"),...readdirSync(module).sort().map(name=>join(module,name,"🐹️.go"))]){
   if(!existsSync(source))continue;
   if(source.includes("📦️main")){copyFileSync(source,join(work,"cmd","main.go"));continue;}
   index++;copyFileSync(source,join(work,`s${index}.go`));
  }
  const build=spawnSync(compiler,["build","-trimpath","-ldflags","-s -w","-o",join(work,"repo"),"./cmd"],{cwd:work,encoding:"utf8",env:{PATH:"/usr/bin:/bin",HOME:home,GOWORK:"off",GOTOOLCHAIN:"local",GOPROXY:"off",CGO_ENABLED:"0",GOCACHE:join(work,"gocache"),GOPATH:join(work,"gopath")}});
  if(build.status!==0)return done(false,`build-failed: ${String(build.stderr).trim().split("\\n")[0]}`);
  if(process.platform==="darwin")spawnSync("codesign",["-s","-","--identifier","tech.semio.repo","--force",join(work,"repo")]);
  mkdirSync(dirname(target),{recursive:true});rmSync(target,{force:true});copyFileSync(join(work,"repo"),target);chmodSync(target,0o755);
  return done(true,"built");
 }finally{rmSync(work,{recursive:true,force:true});}
}
'''
def hm(t):
    t=one(t,'import {chmodSync,existsSync,lstatSync,mkdirSync,readdirSync,statSync,symlinkSync,writeFileSync} from "node:fs";','import {chmodSync,copyFileSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";')
    t=one(t,'import {hostname} from "node:os";','import {hostname,tmpdir} from "node:os";')
    return one(t,'\n/** 🪜️ The plan of a worker, stage by stage',HM.rstrip("\n")+'\n\n/** 🪜️ The plan of a worker, stage by stage')
HT='''
 test("the launcher of a lab machine is built from the sources of the tree and replaces the seeded binary",()=>{
  const goBinary=spawnSync("/bin/sh",["-c","command -v go"],{encoding:"utf8"}).stdout.trim();
  if(goBinary===""){expect(sliceLabLauncherBuildV1("/nonexistent","/nonexistent")).toMatchObject({ok:false,reason:"no-go"});return;}
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-"))),root=join(directory,"root"),home=join(directory,"h"),module=join(root,"🧰️framework","🛍️products","🦑️repo","🔨️modules","🚀️launcher","📦️packages","🐹️go");
  try{
   mkdirSync(join(module,"📦️main"),{recursive:true});mkdirSync(join(module,"🧭️workspace-root"),{recursive:true});
   writeFileSync(join(module,"🐹️.go"),"package launcher\\n");writeFileSync(join(module,"🧭️workspace-root","🐹️.go"),'package launcher\\n\\nfunc Marker() string { return "built-from-sources" }\\n');
   writeFileSync(join(module,"📦️main","🐹️.go"),'package main\\n\\nimport (\\n\\t"fmt"\\n\\tlauncher "github.com/usalu/semio/repo/launcher"\\n)\\n\\nfunc main() { fmt.Println(launcher.Marker()) }\\n');
   mkdirSync(join(home,".local","share","semio","launcher","bin"),{recursive:true});writeFileSync(join(home,".local","share","semio","launcher","bin","repo"),"seeded");
   expect(sliceLabLauncherBuildV1(join(directory,"empty"),home,goBinary)).toMatchObject({ok:false,reason:"no-sources"});
   const row=sliceLabLauncherBuildV1(root,home,goBinary);
   expect(row).toMatchObject({ok:true,reason:"built"});
   const target=join(home,".local","share","semio","launcher","bin","repo");expect(spawnSync(target,[],{encoding:"utf8"}).stdout.trim()).toBe("built-from-sources");
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
'''
def ht(t):
    t=one(t,'sliceLabAllStagesV1,','sliceLabAllStagesV1,sliceLabLauncherBuildV1,')
    return one(t,'describe("slice lab home: registry, warm partitions, tool selection",()=>{','describe("slice lab home: registry, warm partitions, tool selection",()=>{'+HT.rstrip("\n"))
def lab(t):
    t=one(t,'sliceLabLinkHomeV1,','sliceLabLauncherBuildV1,sliceLabLinkHomeV1,')
    return one(t,' say(`home seeded: ',' if(options.stages==="all")for(const plan of plans){const built=sliceLabLauncherBuildV1(view,plan.home);say(`launcher ${built.reason} ${built.seconds}s / Launcher ${built.reason}`);seedRows.push({machine:plan.origin,id:"launcher-build",ok:built.ok,method:built.reason,seconds:built.seconds});}\n say(`home seeded: ')
PATCH={"hm":hm,"ht":ht,"lab":lab}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k,p in FILES.items(): new[k]=PATCH[k](read(p))
    for i,(k,p) in enumerate(FILES.items()):
        old=read(p);write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k,p in FILES.items(): write(p,new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,p in FILES.items():
        m=manifest[k];now=sha(read(p))
        if now==m["after"]: write(p,read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
