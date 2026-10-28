"""🧭 Edits (mirror or temp root) of batch `jdir`: the change journal directory no longer depends on TMPDIR. usage: edit.py <root>"""
import sys
root = sys.argv[1]
L = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(rel, pairs):
    path = L + rel
    s = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert s.count(old) == 1, (rel, old[:70])
        s = s.replace(old, new)
    open(path, "w", encoding="utf8").write(s)
S = "📥️caller/🛰️service/🔑️session/🟨️"
edit(S + ".mjs", [
 ('import {tmpdir,userInfo} from "node:os";', 'import {homedir,tmpdir,userInfo} from "node:os";'),
 ('/** 🔐 Reads the service key:',
  '''/** 📌 The private state directory of one worktree and user that no per-process variable moves: below `XDG_RUNTIME_DIR` when that names one, else below the user cache directory of the account database (never `TMPDIR`, which a nested build action sets to its own scratch), so that every process of the worktree reaches the same change journal. */
export function stateDirectoryV1(root,environment=process.env,platform=process.platform,user=undefined,home=undefined){
 const id=worktreeIdV1(root),owner=user??(()=>{try{return String(userInfo().uid);}catch{return"0";}})(),account=home??(()=>{try{return userInfo().homedir;}catch{return homedir();}})();
 if(platform==="win32")return join(environment.LOCALAPPDATA||join(account,"AppData","Local"),"semio","repo",id);
 if(environment.XDG_RUNTIME_DIR)return join(environment.XDG_RUNTIME_DIR,"semio-repo-"+owner,id);
 return join(account,platform==="darwin"?"Library/Caches":".cache","semio","repo",id);
}

/** 🔐 Reads the service key:'''),
])
edit(S + ".d.mts", [
 ('export function readServiceKeyV1(', 'export function stateDirectoryV1(root:string,environment?:Readonly<Record<string,string|undefined>>,platform?:string,user?:string,home?:string):string;\nexport function readServiceKeyV1('),
])
E = "🏡️resident/📨️protocol/📍️endpoint/🟨️"
edit(E + ".mjs", [('export {canonicalRootV1,endpointsV1,readServiceKeyV1,worktreeIdV1}', 'export {canonicalRootV1,endpointsV1,readServiceKeyV1,stateDirectoryV1,worktreeIdV1}')])
edit(E + ".d.mts", [('export {canonicalRootV1,endpointsV1,readServiceKeyV1,worktreeIdV1}', 'export {canonicalRootV1,endpointsV1,readServiceKeyV1,stateDirectoryV1,worktreeIdV1}')])
edit("🏡️resident/📜️journal/🟦️.ts", [
 ('import {canonicalRootV1,endpointsV1} from "../📨️protocol/📍️endpoint/🟨️.mjs";', 'import {canonicalRootV1,stateDirectoryV1} from "../📨️protocol/📍️endpoint/🟨️.mjs";'),
 ('/** 📍️ The journal directory of a workspace: below the private runtime directory of the resident service endpoints (one per worktree and user). */\nexport function changeJournalDirectoryV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{\n return join(endpointsV1(canonicalRootV1(root),environment as NodeJS.ProcessEnv).directory,"journal");',
  '/** 📍️ The journal directory of a workspace: below the private state directory of the worktree and user, which no `TMPDIR` moves, so a nested build action reaches the journal of its parent. */\nexport function changeJournalDirectoryV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{\n return join(stateDirectoryV1(canonicalRootV1(root),environment),"journal");'),
])
print("edited", root[-30:])
