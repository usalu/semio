"""🔁 [backlog] rebases xplat3-g's staged witness test onto the live file (witness2 + racy fixes landed): applies xplat3's one change (the sandbox-escape fixture writes below its own scratch directory instead of /tmp, Windows-safe) to the LIVE text, replaces the staged copy (original kept as *.pre-backlog) and records the live digest as the new base in bases.tsv. Idempotent."""
import hashlib, os, shutil, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = os.path.abspath(os.path.join(C, *[".."] * 7))
rel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🧪️tests/🟦️.ts"
staged = f"{C}/xplat3-g-tools/staged/files/{rel}"; tsv = f"{C}/xplat3-g-tools/bases.tsv"
for f in (staged, tsv):
    if not os.path.exists(f + ".pre-backlog"): shutil.copy(f, f + ".pre-backlog")
text = open(f"{R}/{rel}", encoding="utf8").read()
old1 = '   writeFileSync("/tmp/semio-witness-outside-"+process.pid,"x");new Database("/tmp/semio-witness-db-"+process.pid);\n'
new1 = '   writeFileSync(${JSON.stringify(outside)},"x");new Database(${JSON.stringify(database)});\n'
old2 = ' const{base,root}=world();\n try{\n  const sneaky=`'
new2 = ' const{base,root}=world();\n try{\n  const outside=join(base,"outside.txt"),database=join(base,"outside.db");\n  const sneaky=`'
old3 = '{recursive:true,force:true});rmSync("/tmp/semio-witness-outside-"+process.pid,{force:true});}'
new3 = '{recursive:true,force:true});}'
if old1 in text:
    assert text.count(old1) == 1 and text.count(old2) == 1 and text.count(old3) == 1
    text = text.replace(old1, new1).replace(old2, new2).replace(old3, new3)
open(staged, "w", encoding="utf8").write(text)
live_sha = hashlib.sha256(open(f"{R}/{rel}", "rb").read()).hexdigest()
rows = [l.rstrip("\n").split("\t", 1) for l in open(tsv + ".pre-backlog", encoding="utf8") if l.strip()]
open(tsv, "w", encoding="utf8").write("".join(f"{live_sha if r == rel else b}\t{r}\n" for b, r in rows))
print("xplat3 witness test rebased; base", live_sha[:12])
