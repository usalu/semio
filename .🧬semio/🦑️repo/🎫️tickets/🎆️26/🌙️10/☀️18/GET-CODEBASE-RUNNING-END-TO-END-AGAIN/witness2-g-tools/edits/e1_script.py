#!/usr/bin/env python3
"""🧭️ e1: script.ts - the git ignore sources are observed through git's own answers (observation kind `i`), checked by the `ignore` port."""
import sys
root = sys.argv[1]
F = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts"
text = open(F, encoding="utf8").read()
def once(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:80])
    text = text.replace(old, new)
once('''/** 🧾️ The digest of the tracked listing, or `!failed`''', '''const BAZEL_GIT_IGNORE_KEYS_V1:ReadonlySet<string>=new Set(["core.excludesfile","core.ignorecase","core.precomposeunicode","core.sparsecheckout","core.sparsecheckoutcone","core.worktree","core.bare"]);

/** 🙈️ Asks git one read-only question in a checkout and returns its exit code and output; git is the only authority on what it ignores. */
async function bazelGitAnswerV1(root:string,args:readonly string[],signal:AbortSignal):Promise<{code:number;out:string}>{
 const git=startToolV1("git",["-c","core.fsmonitor=false",...args],{cwd:root,stdio:["ignore","pipe","pipe"],windowsHide:true}),chunks:Buffer[]=[];let bytes=0;
 const cancel=()=>git.kill();signal.addEventListener("abort",cancel,{once:true});
 try{
  const code=await new Promise<number>((accept,reject)=>{git.stdout.on("data",chunk=>{bytes+=chunk.length;if(bytes>1024*1024){git.kill();reject(Error("git answer exceeds 1 MiB"));}else chunks.push(chunk);});git.stderr.resume();git.once("error",reject);git.once("close",value=>accept(value??1));});
  signal.throwIfAborted();return {code,out:Buffer.concat(chunks).toString()};
 }finally{signal.removeEventListener("abort",cancel);}
}

/** 🧭️ The digest of everything outside the tree that decides what git ignores, read from git itself: the effective values of the ignore-relevant config keys across every scope, include and environment override, the file `core.excludesFile` names (default `$XDG_CONFIG_HOME/git/ignore`, else `~/.config/git/ignore`) and `.git/info/exclude` wherever git keeps it, both by git's own content hash, so a rewrite with the same content or an unrelated config key changes nothing; `!failed` where git cannot answer. */
export async function bazelIgnoreSourcesDigestV1(root:string,signal:AbortSignal):Promise<string>{
 try{
  const[list,named,exclude]=await Promise.all([bazelGitAnswerV1(root,["config","--list","-z"],signal),bazelGitAnswerV1(root,["config","--path","--get","core.excludesFile"],signal),bazelGitAnswerV1(root,["rev-parse","--git-path","info/exclude"],signal)]);
  if(list.code!==0||exclude.code!==0)return "!failed";
  const rows=list.out.split("\\0").filter(row=>BAZEL_GIT_IGNORE_KEYS_V1.has(row.split("\\n",1)[0]!.toLowerCase()));
  const configured=named.code===0&&named.out.trim()!=="",home=process.env.HOME||homedir(),base=process.env.XDG_CONFIG_HOME||join(home,".config");
  const sources=[configured?resolve(root,named.out.replace(/\\r?\\n$/u,"")):join(base,"git","ignore"),resolve(root,exclude.out.replace(/\\r?\\n$/u,""))];
  const contents=await Promise.all(sources.map(async path=>{const answer=await bazelGitAnswerV1(root,["hash-object","--no-filters","--",path],signal).catch(()=>({code:1,out:""}));return answer.code===0?answer.out.trim():"-";}));
  return hash(JSON.stringify([rows,sources,contents]));
 }catch(error){signal.throwIfAborted();return "!failed";}
}

/** 🧾️ The digest of the tracked listing, or `!failed`''')
once('''const home=process.env.HOME??homedir(),config=process.env.XDG_CONFIG_HOME||join(home,".config"),single=[join(root,".git/info/exclude"),join(root,".git/config"),join(home,".gitconfig"),join(config,"git/ignore"),join(config,"git/config"),"/etc/gitconfig"];
 await Promise.all(single.map(path=>lstat(path).catch(()=>undefined)));
''', '''witness.observe("i","excludes",await witness.allow("git",()=>bazelIgnoreSourcesDigestV1(root,invocation.control.signal)));
''')
once('''  runtime:async(subject:string,expected:string)=>{''', '''  ignore:async(_subject:string,expected:string)=>(await bazelIgnoreSourcesDigestV1(root,phaseControl.signal))===expected,
  runtime:async(subject:string,expected:string)=>{''')
open(F, "w", encoding="utf8").write(text)
