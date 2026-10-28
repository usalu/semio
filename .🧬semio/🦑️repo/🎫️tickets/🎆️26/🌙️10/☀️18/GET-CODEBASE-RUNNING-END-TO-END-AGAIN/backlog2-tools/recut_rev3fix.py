"""🔁 [backlog2] re-cuts the dropped universe-exposure part of rev3fix on top of the LIVE rustedit universe walk (`observeBazelSourceUniverseV1`): two anchored edits of `🏗️bazel/📜️script.ts` (the function + the walk block) and the law; usage: python3 -I recut_rev3fix.py [--root R] ; rewrites staged/rev3fix/edits.json + bases.json (originals kept as *.pre-backlog2)."""
import hashlib, json, os, shutil, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
root = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else LIVE
S = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts"
LAW = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🙈️excluded-directory/🟦️.ts"
D = C + "/rev3fix-g-tools/staged/rev3fix"
text = open(f"{root}/{S}", encoding="utf8").read()
START = " const seen=new Set<string>();\n const walk=async(skip:(name:string)=>boolean)=>{const lane=async()=>{"
END = " if(whole){await lstat(join(root,\".gitignore\")).catch(()=>undefined);pending.push(\"\");await walk(name=>bazelUniversePrunesV1(whole,name));}\n"
a = text.index(START); b = text.index(END) + len(END)
old_block = text[a:b]; assert text.count(old_block) == 1
NEW_BLOCK = ''' const seen=new Set<string>(),reached=new Set<string>(),excluded:string[]=[],closed=new Set(ignored);
 const walk=async(skip:(name:string)=>boolean)=>{const lane=async()=>{
  while(pending.length){
   checkScriptInvocation(invocation);const directory=pending.pop()!;if(seen.has(directory))continue;seen.add(directory);
   if(closed.has(directory))continue;
   if(!reached.has(directory)&&directory.split("/").some(part=>BAZEL_SOURCE_EXCLUDED_V1.has(part))){excluded.push(directory);continue;}
   let entries;try{entries=await readdir(join(root,directory),{withFileTypes:true});}catch{continue;}
   for(const entry of entries){
    if(entry.name===".gitignore")await lstat(join(root,directory,entry.name)).catch(()=>undefined);
    if(!entry.isDirectory())continue;const path=slash(join(directory,entry.name));
    if(BAZEL_SOURCE_EXCLUDED_V1.has(entry.name))excluded.push(path);else if(!skip(entry.name)){reached.add(path);pending.push(path);}
   }
   await invocation.control.yieldContinuation();
  }
 };
 await Promise.all(Array.from({length:16},lane));
 while(excluded.length){
  const open=await witness.allow("git",()=>bazelNonIgnoredDirectoriesV1(root,excluded.splice(0),invocation.control.signal));if(!open.length)break;
  for(const path of await witness.allow("git",()=>bazelIgnoredDirectoriesV1(root,open,invocation.control.signal)))closed.add(path);
  for(const path of open){seen.delete(path);reached.add(path);pending.push(path);}
  await Promise.all(Array.from({length:16},lane));
 }};
 await walk(()=>false);
''' + END
FN = '''/** 🙈️ The directories among `directories` (workspace relative, existing) that git does not ignore: the walk of the witness does not descend into a directory with an excluded name on its own, so a file that appears below one of them is part of the git universe without changing any identity the witness observes; the walk enters exactly these (a directory that git ignores keeps everything below it out of the universe, a tracked file below it is covered by the digest of the tracked listing). Git's own directory and the ticket folder are never in the universe. Where git cannot answer, every directory counts as not ignored. */
async function bazelNonIgnoredDirectoriesV1(root:string,directories:readonly string[],signal:AbortSignal):Promise<string[]>{
 const candidates=[...new Set(directories)].filter(path=>!path.split("/").includes(".git")&&path!==".🧬semio"&&!path.startsWith(".🧬semio/"));
 if(!candidates.length)return[];
 const git=startToolV1("git",["-c","core.fsmonitor=false","check-ignore","-z","--stdin"],{cwd:root,stdio:["pipe","pipe","pipe"],windowsHide:true}),chunks:Buffer[]=[];
 const cancel=()=>git.kill();signal.addEventListener("abort",cancel,{once:true});
 try{
  const code=await new Promise<number>((accept,reject)=>{git.stdout.on("data",chunk=>chunks.push(chunk));git.stderr.resume();git.stdin.on("error",()=>undefined);git.once("error",reject);git.once("close",code=>accept(code??-1));git.stdin.end(candidates.map(path=>path+"/\\0").join(""));});
  signal.throwIfAborted();if(code!==0&&code!==1)return candidates;
  const ignored=new Set(Buffer.concat(chunks).toString().split("\\0").filter(Boolean).map(path=>path.replace(/\\/$/u,"")));
  return candidates.filter(path=>!ignored.has(path));
 }catch(error){signal.throwIfAborted();return candidates;}
 finally{signal.removeEventListener("abort",cancel);}
}

'''
ANCHOR = "/** 🔭️ Makes the git universe observable before it is asked for:"
assert text.count(ANCHOR) == 1
spec = json.load(open(D + "/edits.json", encoding="utf8"))
if not os.path.exists(D + "/edits.json.pre-backlog2"): shutil.copyfile(D + "/edits.json", D + "/edits.json.pre-backlog2")
if not os.path.exists(D + "/bases.json.pre-backlog2"): shutil.copyfile(D + "/bases.json", D + "/bases.json.pre-backlog2")
spec["edits"] = [e for e in spec["edits"] if e["file"] != S] + [{"file": S, "old": ANCHOR, "new": FN + ANCHOR}, {"file": S, "old": old_block, "new": NEW_BLOCK}]
spec["newFiles"] = [n for n in spec["newFiles"] if n["path"] != LAW] + [{"path": LAW, "from": "files/excluded-directory.ts"}]
json.dump(spec, open(D + "/edits.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
bases = json.load(open(D + "/bases.json", encoding="utf8")); bases[S] = hashlib.sha256(open(f"{root}/{S}", "rb").read()).hexdigest()
json.dump(bases, open(D + "/bases.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("edits", len(spec["edits"]), "newFiles", len(spec["newFiles"]), "script base", bases[S][:10])
