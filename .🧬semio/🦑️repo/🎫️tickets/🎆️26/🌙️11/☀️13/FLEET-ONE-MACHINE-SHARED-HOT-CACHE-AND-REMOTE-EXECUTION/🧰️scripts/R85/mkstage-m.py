#!/usr/bin/env python3
"""R85 landing m: the dependency inputs are compared by CONTENT (not inode, mode or times) and a moved funding is re-run for a bounded TIME, not three times. Usage: mkstage-m.py M_DIR STAGE_DIR (M_DIR has paths.txt, base.X, new.X)"""
import os,shutil,sys
M=sys.argv[1].rstrip("/")+"/";D=sys.argv[2].rstrip("/")+"/";R="/Users/ueli/Documents/semio/"
shutil.rmtree(D,ignore_errors=True);os.makedirs(D+"base");os.makedirs(D+"new")
rows=[]
for line in open(M+"paths.txt",encoding="utf8").read().splitlines():
    k,rel=line.split(" ",1);i="m"+k
    shutil.copyfile(M+"new."+k,D+"new/"+i)
    new=not os.path.exists(R+rel)
    if not new:shutil.copyfile(M+"base."+k,D+"base/"+i)
    rows.append("%s %s %s"%(i,rel,"new" if new else "mod"))
open(D+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
C="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
S=C+"💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/";B=C+"🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/"
tests=[B+"🧾️content-recheck/🟦️.ts",S+"🧪️tests/🌊️moving-tree/🟦️.ts",B+"👓️reading/📥️recheck/🟦️.ts",B+"🚧️fence/🟦️.ts",S+"🧪️tests/⏳️busy-funding/🟦️.ts",S+"💰️funding/🧪️tests/🧬️reused-pid/🟦️.ts",S+"🧪️tests/🟦️.ts",C+"📖️texts/🧪️tests/🟦️.ts"]
open(D+"tests.txt","w",encoding="utf8").write("\n".join(tests)+"\n")
open(D+"before.txt","w",encoding="utf8").write("\n".join(tests[:2])+"\n")
open(D+"note.txt","w",encoding="utf8").write("""Files: dependency storage `recheckJavascriptDependencyInputsV1` (content only), funding `retryWhileDependencyFundingMovesV1` (time bound `maximumMilliseconds` 7200000 replaces `maximumReruns` 3; a `now` port), setup schema, caching texts (`dependency-store.funding-rerun` EN+DE), tests `moving-tree` (updated) and new `content-recheck`.
- g1-m05 `tools` ended its try with `Original Bun dependency input changed during installation ... attempt 3 of 3` in a tree with constant replica writes: the recheck compared dev, ino and mode of every input and ancestor directory, so a rewrite of identical bytes (new inode) was a change. Now only content counts: bytes of files, kind of paths (file, directory, absent), single link. A real change re-runs the funding from the progress already published for at most two hours (was three runs, then exit 75 and a back-off).
""")
print(len(rows),"files")
