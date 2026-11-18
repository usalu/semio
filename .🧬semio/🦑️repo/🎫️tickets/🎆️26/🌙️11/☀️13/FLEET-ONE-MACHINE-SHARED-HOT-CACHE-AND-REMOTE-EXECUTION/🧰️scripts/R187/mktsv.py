S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R187"
F="/Users/ueli/Documents/semio/🧰️framework/🔨️modules"
O=F+"/📁️filesystem/🧾️observation"; P=F+"/🏃️process/🏗️build/🗣️language/📣️progress"
M=S+"/mirror/🧰️framework/🔨️modules"
import shutil,os
os.makedirs(S+"/land",exist_ok=True)
rows=[(O+"/📁️descriptor/🟦️.ts",S+"/stage/d.ts",S+"/base/d.ts"),
(O+"/📁️descriptor/🧬️schema/🔣️.json",S+"/stage/ds.json",S+"/base/ds.json"),
(P+"/🟦️.ts",S+"/stage/p.ts",S+"/base/p.ts")]
for n in ["🟦️.ts","🔣️.json","🐍️oracle.py"]:
    src=M+"/📁️filesystem/🧾️observation/🧪️tests/🪟️rename-settlement/"+n
    dst=S+"/land/rs-"+n.replace("🟦️","t").replace("🔣️","j").replace("🐍️","p")
    shutil.copy(src,dst);rows.append((O+"/🧪️tests/🪟️rename-settlement/"+n,dst,""))
dst=S+"/land/wr-t.ts";shutil.copy(M+"/🏃️process/🏗️build/🗣️language/📣️progress/🧪️tests/🪟️wrapped-refusal/🟦️.ts",dst)
rows.append((P+"/🧪️tests/🪟️wrapped-refusal/🟦️.ts",dst,""))
open(S+"/land.tsv","w").write("".join("\t".join(r)+"\n" for r in rows))
print(len(rows))
