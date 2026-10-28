from pathlib import Path
import re,json,shutil
ticket=Path(__file__).parent
generated=ticket/"🗑️generated"
for original,name in [("ifc4x3-and-details-tests.txt","tests145-initial-failed.txt"),("ifc4x3-and-details-selection.json","selection145-initial-failed.json")]:
 assert not(generated/name).exists()
 shutil.copyfile(generated/original,generated/name)
text=(generated/"tests145-initial-failed.txt").read_text(encoding="utf8")
lines=text.splitlines()
indices=[i for i,line in enumerate(lines)if line.startswith("(fail)")]
parts=[]
for i in indices:
 start=max(0,i-22)
 parts.append("\n".join(lines[start:i+2]))
(generated/"checkpoint145-initial-failure-context.txt").write_text("\n\n".join(parts),encoding="utf8")
print("[DEBUG] failed145 archived unchanged")
print("\n\n".join(parts))

