"""📊️ Preserve the actual terminal145 selection and compare real test observations to135."""
from pathlib import Path
from collections import Counter
import json,re,shutil
ticket=Path(__file__).parent
generated=ticket/"🗑️generated"
selection=json.loads((generated/"ifc4x3-and-details-selection.json").read_text(encoding="utf8"))
assert selection["suites"]==145
log=(generated/"ifc4x3-and-details-tests.txt").read_text(encoding="utf8")
totals={key:int(re.findall(r"^\s*(\d+) "+word+r"$",log,re.M)[-1])for key,word in [("pass","pass"),("skip","skip"),("fail","fail")]}
totals["assertions"]=int(re.findall(r"^\s*(\d+) expect\(\) calls$",log,re.M)[-1])
footer=re.findall(r"Ran (\d+) tests across (\d+) files\. \[(.*?)\]",log)[-1]
totals.update(tests=int(footer[0]),files=int(footer[1]),elapsed=footer[2],code=selection["code"],changedTests=len(selection["changedTests"]))
assert totals["files"]==145
for original,archived in [("ifc4x3-and-details-tests.txt","tests145.txt"),("ifc4x3-and-details-selection.json","selection145.json")]:
 assert not(generated/archived).exists()
 shutil.copyfile(generated/original,generated/archived)
def observations(text):
 result=Counter();current=None
 for line in text.splitlines():
  if re.match(r"^\d+ tests skipped:",line):break
  if line.startswith("✏️s")and line.endswith(":"):current=line[:-1].replace("\\","/")
  match=re.match(r"^\((pass|skip|fail)\) (.*?)(?: \[[0-9.]+(?:ms|s)\])?$",line)
  if match:
   assert current is not None,line
   result[(current,match[1],match[2])]+=1
 return result
previous=observations((generated/"tests135.txt").read_text(encoding="utf8"))
current=observations(log)
def rows(counter):return [{"file":key[0],"outcome":key[1],"name":key[2],"count":count}for key,count in counter.items()]
comparison={"previousObservations":sum(previous.values()),"currentObservations":sum(current.values()),"missing":rows(previous-current),"added":rows(current-previous)}
(generated/"checkpoint145-test-comparison.json").write_text(json.dumps(comparison,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
paths=[path.replace("\\","/")for path in selection["paths"]]
scope={"ifc":sum("/🏅️standards/🔖️4x3/"in path for path in paths),"details":sum("/🔎️details/"in path for path in paths)}
scope["shared"]=145-scope["ifc"]-scope["details"]
(generated/"checkpoint145-totals.json").write_text(json.dumps({"totals":totals,"scope":scope},indent=2)+"\n",encoding="utf8")
status=f"Actual selected145 terminal{totals['code']}: {totals['pass']}pass/{totals['skip']}skip/{totals['fail']}fail/{totals['assertions']} Bun expect calls/{totals['tests']}tests/{totals['files']}files/{totals['elapsed']}, {totals['changedTests']} selected test-source hash changes. Scope{scope['ifc']}IFC4x3/{scope['details']}Details/{scope['shared']}shared. Real test-name/outcome comparison versus135: {len(comparison['missing'])} missing and {sum(row['count']for row in comparison['added'])} added. Full IFC4x3/all-MVD goal/ticket remain active/incomplete. Node assert observations are additional, not inferred Bun expect counts. Native runtime/type/physical funding/receipt/host renderer proof remains unavailable."
report="# Expanded145 Registered Checkpoint\n\n"+status+"\n\nArchived actual output/selection/totals/comparison are under generated: tests145.txt, selection145.json, checkpoint145-totals.json and checkpoint145-test-comparison.json. Previous135 evidence remains untouched. Footer repeated skipped-name summaries are excluded from real observation comparison; no tests are invented from those repeated prints.\n\nAdded suites since135 are issued-owner coverage52, numeric120, Measure29, CSG132, consumed-domain21, BIM96 and Boolean control65/cooperative154/receiver25/policy379. Independent source closures and external oracle qualifications are retained in their own ticket reports. In particular, raw aggregate INTEGER-versus-REAL SDK normalization, fixed ARRAY SDK cardinality gaps, native unexecuted laws and existing gated skips remain explicit. New SelfWeight32, Value109 red and vector49 are outside this running selection and are not counted here. Official originals remain distinct from authored fixtures.\n\nNo full repository/build/native/MVD certification follows from this selected checkpoint. The actual IFC finite Boolean adapter remains test-first work; new public31 red is preserved with existing dispatcher unchanged. Resource-session/workspace and exact winding ownership remain required before implementation. Ticket and goal are not closed.\n"
(ticket/"checkpoint145.md").write_text(report,encoding="utf8")
for name in ["current-state.md","registered-test-scope.md","exchange-profile-current-state.md","post135-current-state.md"]:
 path=ticket/name
 title,separator,body=path.read_text(encoding="utf8").partition("\n\n")
 path.write_text(title+separator+status+"\n\n"+body,encoding="utf8")
print("[DEBUG]",json.dumps({"totals":totals,"scope":scope,"missing":len(comparison["missing"]),"added":sum(row["count"]for row in comparison["added"])}))
assert totals["code"]==0 and totals["fail"]==0 and totals["changedTests"]==0
assert not comparison["missing"]
assert sum(previous.values())==3494 and sum(current.values())==totals["tests"]

