"""🛠️ Anchored, idempotent edits of batch `planfp` round 3 on a copy of 📜️script.ts: the universe walk does not enter directories git ignores as a whole (a task's own generated store), so a build that writes there leaves the plan reusable.
usage: python3 -I edit_script3.py <script.ts>"""
import sys
p=sys.argv[1];t=open(p,encoding="utf8").read()
def rep(old,new):
    global t
    if t.count(new)==1 and new not in old: return
    assert t.count(old)==1,("ANCHOR",t.count(old),old[:120]); t=t.replace(old,new)
rep('''/** 🧾️ The digest of the tracked listing,''','''/** 🕳️ The directories below the walked roots that git ignores as a whole (`ls-files --others --ignored --directory`): git never lists an untracked file below them, so their listings are no input of a plan; no roots ask nothing, and an answer git cannot give is the empty set, which only observes more. */
async function bazelIgnoredDirectoriesV1(root:string,roots:readonly string[],signal:AbortSignal):Promise<ReadonlySet<string>>{
 if(!roots.length)return new Set();
 const answer=await bazelGitAnswerV1(root,["ls-files","--others","--ignored","--exclude-standard","--directory","-z",...(roots.includes("")?[]:["--",...roots])],signal).catch(()=>({code:1,out:""}));signal.throwIfAborted();
 return new Set(answer.code===0?answer.out.split("\\0").filter(path=>path.endsWith("/")).map(path=>path.slice(0,-1)):[]);
}

/** 🧾️ The digest of the tracked listing,''')
rep('''without such names the universe is unbounded and the plan is not reused.
 * @see ./🌌️universe/🟦️.ts */''','''without such names the universe is unbounded and the plan is not reused. A directory git ignores as a whole (a project's generated store such as `.🧾️store`) is not entered: no untracked file below it can join the listing, the rules that ignore it are observed (`i` and every `.gitignore` on the way) and its appearing or vanishing changes its parent's listing, while tracked files below it stay in the tracked listing; so a build writing its own outputs there leaves the plan reusable.
 * @see ./🌌️universe/🟦️.ts */''')
rep(''' witness.observe("i","excludes",await witness.allow("git",()=>bazelIgnoreSourcesDigestV1(root,invocation.control.signal)));
 const pending:string[]=[];''',''' witness.observe("i","excludes",await witness.allow("git",()=>bazelIgnoreSourcesDigestV1(root,invocation.control.signal)));
 const ignored=await witness.allow("git",()=>bazelIgnoredDirectoriesV1(root,prefixes,invocation.control.signal)),pending:string[]=[];''')
rep('''   if(directory.split("/").some(part=>BAZEL_SOURCE_EXCLUDED_V1.has(part)))continue;
   let entries;try{entries=await readdir(join(root,directory),{withFileTypes:true});}catch{continue;}''','''   if(directory.split("/").some(part=>BAZEL_SOURCE_EXCLUDED_V1.has(part))||ignored.has(directory))continue;
   let entries;try{entries=await readdir(join(root,directory),{withFileTypes:true});}catch{continue;}''')
open(p,"w",encoding="utf8").write(t);print("ok")
