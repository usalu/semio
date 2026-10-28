#!/usr/bin/env python3
"""🕳 Anchored edit of the Bazel planner script for batch `excluded` (lane audit3): the witness walk enters a directory with an excluded name exactly when git does not ignore it.
usage: python3 -I edit_excluded.py <in script.ts> <out script.ts>   (exit 1 = an anchor is missing or not unique)"""
import sys
src = open(sys.argv[1], encoding="utf8").read()
def once(old, new):
    global src
    if src.count(old) != 1: sys.exit("anchor %d times: %s" % (src.count(old), old[:90]))
    src = src.replace(old, new)
once('/** 🚫️ The directory names a source walk never enters; the filesystem fallback and the witness of the git universe agree on them. */',
     '/** 🚫️ The directory names of build outputs and bookkeeping: the filesystem fallback never enters one; the witness of the git universe enters one exactly when git does not ignore it ({@link bazelIgnoredAmongV1}), because git\'s listing, not the name, decides what the planner admits (a nested `.🧬semio` a tool left in a package, a `target` below `🧫️fixtures` that `.gitignore` un-ignores). */')
once('const BAZEL_UNADMITTED_PREFIXES_V1=[".🧬semio/",".git/"] as const,',
     '/** 🙈️ The directories among `paths` that git ignores as a whole (`check-ignore --no-index --stdin -z`): git lists no untracked file below one; an answer git cannot give ignores nothing, which only observes more.\n * @see https://git-scm.com/docs/git-check-ignore */\n'
     'async function bazelIgnoredAmongV1(root:string,paths:readonly string[],signal:AbortSignal):Promise<ReadonlySet<string>>{\n'
     ' const git=startToolV1("git",["-c","core.fsmonitor=false","check-ignore","--no-index","--stdin","-z"],{cwd:root,stdio:["pipe","pipe","pipe"],windowsHide:true}),chunks:Buffer[]=[];let bytes=0;\n'
     ' const cancel=()=>git.kill();signal.addEventListener("abort",cancel,{once:true});\n'
     ' try{\n'
     '  const code=await new Promise<number>((accept,reject)=>{git.stdout.on("data",chunk=>{bytes+=chunk.length;if(bytes>16*1024*1024){git.kill();reject(Error("git answer exceeds 16 MiB"));}else chunks.push(chunk);});git.stderr.resume();git.once("error",reject);git.once("close",value=>accept(value??1));git.stdin.on("error",()=>undefined);git.stdin.end(paths.map(path=>path+"\\0").join(""));});\n'
     '  signal.throwIfAborted();return new Set(code===0?Buffer.concat(chunks).toString().split("\\0").filter(Boolean):[]);\n'
     ' }catch(error){signal.throwIfAborted();return new Set();}\n'
     ' finally{signal.removeEventListener("abort",cancel);}\n'
     '}\n\n'
     'const BAZEL_UNADMITTED_PREFIXES_V1=[".🧬semio/",".git/"] as const,')
once('/** 🔭️ Makes the git universe observable before it is asked for: every directory below the traversal roots (their identities say whether a file appeared or vanished),',
     '/** 🔭️ Makes the git universe observable before it is asked for: every directory below the traversal roots in which git could list an untracked file (their identities say whether a file appeared or vanished; a directory with an excluded name is entered exactly when git does not ignore it, a `.git` directory and the unadmitted bookkeeping never),')
once(' const seen=new Set<string>();\n const walk=async(skip:(name:string)=>boolean)=>{const lane=async()=>{',
     ' const walk=async(skip:(name:string)=>boolean)=>{const lane=async()=>{')
once(' const ignored=await witness.allow("git",()=>bazelIgnoredDirectoriesV1(root,prefixes,invocation.control.signal)),pending:string[]=[];\n',
     ' const ignored=await witness.allow("git",()=>bazelIgnoredDirectoriesV1(root,prefixes,invocation.control.signal)),pending:string[]=[],seen=new Set<string>(),held=new Set<string>();\n const enter=(path:string,excluded:boolean)=>{if(!bazelAdmittedSourceV1(path+"/")||path.split("/").includes(".git"))return;if(excluded)held.add(path);else pending.push(path);};\n')
once('   if(directory.split("/").some(part=>BAZEL_SOURCE_EXCLUDED_V1.has(part))||ignored.has(directory))continue;\n',
     '   if(ignored.has(directory))continue;\n')
once('    if(entry.isDirectory()&&!BAZEL_SOURCE_EXCLUDED_V1.has(entry.name)&&!skip(entry.name))pending.push(slash(join(directory,entry.name)));\n',
     '    if(entry.isDirectory()&&!skip(entry.name))enter(slash(join(directory,entry.name)),BAZEL_SOURCE_EXCLUDED_V1.has(entry.name));\n')
once(' };await Promise.all(Array.from({length:16},lane));};\n await walk(()=>false);\n if(whole){await lstat(join(root,".gitignore")).catch(()=>undefined);pending.push("");await walk(name=>bazelUniversePrunesV1(whole,name));}\n',
     ' };await Promise.all(Array.from({length:16},lane));while(held.size){const asked=[...held].filter(path=>!seen.has(path)&&!ignored.has(path));held.clear();const shut=asked.length?await witness.allow("git",()=>bazelIgnoredAmongV1(root,asked,invocation.control.signal)):new Set<string>();for(const path of asked)if(!shut.has(path))pending.push(path);await Promise.all(Array.from({length:16},lane));}};\n await walk(()=>false);\n if(whole){await lstat(join(root,".gitignore")).catch(()=>undefined);pending.push("");await walk(name=>bazelUniversePrunesV1(whole,name));}\n')
once(' for(const prefix of prefixes){\n  for(let directory=dirname(prefix);;directory=dirname(directory)){await lstat(join(root,directory==="."?"":directory,".gitignore")).catch(()=>undefined);if(directory==="."||directory==="/")break;}\n  pending.push(prefix);\n }\n',
     ' for(const prefix of prefixes){\n  for(let directory=dirname(prefix);;directory=dirname(directory)){await lstat(join(root,directory==="."?"":directory,".gitignore")).catch(()=>undefined);if(directory==="."||directory==="/")break;}\n  enter(prefix,prefix.split("/").some(part=>BAZEL_SOURCE_EXCLUDED_V1.has(part)));\n }\n')
open(sys.argv[2], "w", encoding="utf8").write(src)
