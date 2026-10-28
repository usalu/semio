"""🧪️ Run the selected genuine CSG consumer without changing repository command routing."""
from pathlib import Path
import json,subprocess,sys
ticket=Path(__file__).parent
root=ticket.parents[6]
tests=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests"
names=[name for name in sys.argv[2:] if not name.startswith('--match=')] or ["csg-round.test.ts"]
flags=[arg for name in sys.argv[2:] if name.startswith('--match=') for arg in ['--test-name-pattern',name[len('--match='):]]]
log=ticket/"🗑️generated"/(sys.argv[1]+".txt")
with log.open("w",encoding="utf-8") as output:
 result=subprocess.run(["bun","test",*[str(tests/name)for name in names],*flags],cwd=root,stdout=output,stderr=subprocess.STDOUT)
text=log.read_text(encoding="utf-8")
print("[DEBUG]",json.dumps({"exit":result.returncode,"log":str(log),"tail":text[-2000:]}))
sys.exit(result.returncode)
