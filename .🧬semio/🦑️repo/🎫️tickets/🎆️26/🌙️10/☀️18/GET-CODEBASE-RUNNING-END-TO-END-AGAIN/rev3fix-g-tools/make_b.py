from make_lib import *

# ---- B: planner skip: a directory the walk does not enter and git does not ignore taints the witness
script = B + "/🔧️tools/🏗️bazel/📜️script.ts"
edit(script, "/** 🔭️ Makes the git universe observable before it is asked for:",
"""/** 🙈️ The directories among `directories` (workspace relative, existing) that git does not ignore: the walk of the witness never enters a directory with an excluded name, so a file that appears below one of them is part of the git universe without changing any identity the witness observes; a directory that git ignores keeps everything below it out of the universe (a tracked file below it is covered by the digest of the tracked listing). Git's own directory and the ticket folder are never in the universe. Where git cannot answer, every directory counts as not ignored. */
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

/** 🔭️ Makes the git universe observable before it is asked for:""")
edit(script, " const seen=new Set<string>();\n const lane=async()=>{", " const seen=new Set<string>(),excluded:string[]=[];\n const lane=async()=>{")
edit(script, "   if(directory.split(\"/\").some(part=>BAZEL_SOURCE_EXCLUDED_V1.has(part)))continue;\n   let entries;try{entries=await readdir(join(root,directory),{withFileTypes:true});}catch{continue;}",
 "   if(directory.split(\"/\").some(part=>BAZEL_SOURCE_EXCLUDED_V1.has(part))){excluded.push(directory);continue;}\n   let entries;try{entries=await readdir(join(root,directory),{withFileTypes:true});}catch{continue;}")
edit(script, "    if(entry.isDirectory()&&!BAZEL_SOURCE_EXCLUDED_V1.has(entry.name))pending.push(slash(join(directory,entry.name)));\n",
 "    if(entry.isDirectory()){const path=slash(join(directory,entry.name));if(BAZEL_SOURCE_EXCLUDED_V1.has(entry.name))excluded.push(path);else pending.push(path);}\n")
edit(script, " await Promise.all(Array.from({length:16},lane));\n witness.observe(\"g\",",
 " await Promise.all(Array.from({length:16},lane));\n const exposed=excluded.length?await witness.allow(\"git\",()=>bazelNonIgnoredDirectoriesV1(root,excluded,invocation.control.signal)):[];\n if(exposed.length)witness.taint(\"the source universe holds a directory the walk does not enter and git does not ignore: \"+exposed[0]);\n witness.observe(\"g\",")
newfile(B + "/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🙈️excluded-directory/🟦️.ts", "excluded-directory.ts")

if False:
    write("rev3fix")
    print(len(spec["edits"]), "edits")
