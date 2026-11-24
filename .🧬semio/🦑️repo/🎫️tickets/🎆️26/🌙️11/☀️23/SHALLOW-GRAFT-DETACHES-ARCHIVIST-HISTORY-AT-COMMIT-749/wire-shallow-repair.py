"""Exact, fail-closed replacements that wire `repairGitShallowV1` into the replica daemon's git health round and document it."""
import sys,pathlib
D=pathlib.Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree")
EDITS=[
 ("🔁️replica/🟦️.ts",'import {sweepGitGarbageV1,repairGitConfigV1,applyIndexV1,','import {sweepGitGarbageV1,repairGitConfigV1,repairGitShallowV1,applyIndexV1,'),
 ("🔁️replica/🟦️.ts",'stageHeal:settings.now()+60_000,broken:null as GitBrokenV1|null};','stageHeal:settings.now()+60_000,broken:null as GitBrokenV1|null,shallowAt:0,shallow:false};'),
 ("🔁️replica/🟦️.ts",'  sound.broken=reason;\n','  sound.broken=reason;\n  if(now>=sound.shallowAt){\n   const shallow=await healthMetadata(()=>lstat(join(settings.worktree,".git","shallow")).then(()=>true,()=>false))&&!await repairGitShallowV1(settings.worktree,now,signal);\n   signal.throwIfAborted();\n   if(shallow){sound.shallowAt=now+(options.gitHealMilliseconds??600_000);noteOnce(replica,"git-broken-history-shallow",{event:"blocked",path:".git/shallow",reason:"git-broken-history-shallow"});}\n   else if(sound.shallow){resolved(replica,"git-broken-history-shallow");replica.say({event:"repaired",stage:"git",reason:"git-shallow"});}\n   sound.shallow=shallow;\n  }\n'),
 ("README.md",'replaced files stay in `lost/` as for every apply.','replaced files stay in `lost/` as for every apply.\n\n**The shallow boundary (2026-11-23).** `.git/shallow` states which commits the own object store lacks the parents of, so it is machine state and never travels. It was on the allow-list until a worker onboarded from a shallow clone published its boundary: every machine then read commit 749 of the archive as a root although it held all 767 commits. Every git round the daemon looks for a `.git/shallow` (`repairGitShallowV1`): when every commit, tree and blob behind the parents of its commits is present, the file is kept in `lost/<ms>-git-shallow/shallow` and removed (`repaired` with reason `git-shallow` once a boundary that was reported is gone); while that history is really absent the origin notes the failure `git-broken-history-shallow` (`fleet` warns `git-broken`) and looks again every 10 minutes (`gitHealMilliseconds`), by which time the shared objects have usually arrived. `repo micro-commit` and `repo commit` refuse to commit on a shallow repository.'),
]
texts={}
for name,old,new in EDITS:
 text=texts.setdefault(name,(D/name).read_text(encoding="utf-8"))
 if text.count(old)!=1: sys.exit(f"{name}: expected exactly one match, found {text.count(old)}: {old[:70]}")
 texts[name]=text.replace(old,new)
for name,text in texts.items(): (D/name).write_text(text,encoding="utf-8")
print("edited",len(texts),"files")
