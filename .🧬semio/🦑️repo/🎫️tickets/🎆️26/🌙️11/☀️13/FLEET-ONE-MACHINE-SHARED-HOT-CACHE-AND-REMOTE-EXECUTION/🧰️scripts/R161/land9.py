import sys,os,json,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
FILES={"fz":L+"/🧊️frozen/🟦️.ts","ft":L+"/🧪️tests/🧊️frozen/🟦️.ts","lab":L+"/🟦️.ts"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
FZ='''
/** 🔁️ Gives a reusable frozen root back its links after a run whose `dependencies` stage was real: that stage links every `node_modules` of the root into the generations of the lab's own state, which the end of the run deletes; each such link (found among the ignored paths of the root, and the root `node_modules` itself) is made again as the live tree has it, and the root `node_modules` is rebuilt as the directory of entry links of a freeze. Returns how many links were made again. */
export function relinkFrozenRootV1(source:string,root:string,work:string):number{
 const below=work+sep,pointsIntoWork=(path:string):boolean=>{try{return lstatSync(path).isSymbolicLink()&&readlinkSync(path).startsWith(below);}catch{return false;}};
 let made=0;
 for(const name of names(git(root,["ls-files","-z","--others","--ignored","--exclude-standard","--directory"]))){
  const path=name.replace(/\\/$/u,""),at=join(root,path);
  if(path==="node_modules"||!pointsIntoWork(at))continue;
  const from=join(source,path);
  unlinkSync(at);made++;
  try{if(lstatSync(from).isSymbolicLink())symlinkSync(readlinkSync(from),at);}catch{}
 }
 if(pointsIntoWork(join(root,"node_modules"))){unlinkSync(join(root,"node_modules"));made++;}
 linkAmbientV1(source,root);
 return made;
}
'''
def fz(t):
    t=one(t,'readlinkSync,realpathSync,symlinkSync,utimesSync,writeFileSync}','readlinkSync,realpathSync,symlinkSync,unlinkSync,utimesSync,writeFileSync}')
    return one(t,'/** ✏️ Appends one line feed',FZ.lstrip("\n")+'\n/** ✏️ Appends one line feed')
FT='''
describe("frozen slice lab root after a real dependencies stage",()=>{
 test("the links the stage pointed into the lab's state are made again as the live tree has them, the root node_modules is a directory of entry links again",()=>{
  const live=fixture(),work=realpathSync(mkdtempSync(join(tmpdir(),"frz-rl-"))),frozen=join(work,"r"),gen=join(work,"s","j","g1");
  try{
   freezeSliceLabRootV1(live,frozen);
   mkdirSync(join(gen,"node_modules"),{recursive:true});
   const member=join(frozen,"🧰️framework","lib","node_modules");rmSync(member,{force:true});symlinkSync(join(gen,"node_modules"),member);
   rmSync(join(frozen,"node_modules"),{recursive:true,force:true});symlinkSync(join(gen,"node_modules"),join(frozen,"node_modules"));
   expect(relinkFrozenRootV1(live,frozen,work)).toBe(2);
   expect(readlinkSync(member)).toBe("/nonexistent-store/deps");
   expect(lstatSync(join(frozen,"node_modules")).isDirectory()).toBe(true);expect(readdirSync(join(frozen,"node_modules"))).toContain("pkg");
   expect(relinkFrozenRootV1(live,frozen,work)).toBe(0);
  }finally{rmSync(work,{recursive:true,force:true});}
 });
});
'''
def ft(t):
    t=one(t,'readFileSync,readlinkSync,realpathSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";','readdirSync,readFileSync,readlinkSync,realpathSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";')
    t=one(t,'import {editFrozenFileV1,freezeSliceLabRootV1,frozenRootV1,linkAmbientV1} from','import {editFrozenFileV1,freezeSliceLabRootV1,frozenRootV1,linkAmbientV1,relinkFrozenRootV1} from')
    return t.rstrip("\n")+"\n"+FT
def lab(t):
    t=one(t,'import {editFrozenFileV1,freezeSliceLabRootV1,frozenRootV1} from','import {editFrozenFileV1,freezeSliceLabRootV1,frozenRootV1,relinkFrozenRootV1} from')
    return one(t,' const guard=resolve(work);',' if(frozen!==undefined&&options.frozenDirectory!==undefined){try{const again=relinkFrozenRootV1(live,view,work);say(`frozen root relinked: ${again} links / eingefrorene Wurzel neu verlinkt: ${again} Links`);}catch(error){say(`frozen root relink failed: ${String(error)}`);}}\n const guard=resolve(work);')
PATCH={"fz":fz,"ft":ft,"lab":lab}
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
