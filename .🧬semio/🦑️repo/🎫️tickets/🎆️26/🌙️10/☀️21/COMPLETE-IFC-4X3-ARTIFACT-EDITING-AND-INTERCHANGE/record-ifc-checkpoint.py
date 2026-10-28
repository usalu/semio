"""📊️ Archive an actual selected checkpoint and retain explicit prior-obligation supersessions."""
from pathlib import Path
from collections import Counter
import json,re,shutil,sys

ticket=Path(__file__).parent
generated=ticket/"🗑️generated"
expected=int(sys.argv[1])
selection=json.loads((generated/"ifc4x3-and-details-selection.json").read_text(encoding="utf8"))
assert selection["suites"]==expected
log=(generated/"ifc4x3-and-details-tests.txt").read_text(encoding="utf8")
totals={key:int(re.findall(r"^\s*(\d+) "+word+r"$",log,re.M)[-1])for key,word in [("pass","pass"),("skip","skip"),("fail","fail")]}
totals["assertions"]=int(re.findall(r"^\s*(\d+) expect\(\) calls$",log,re.M)[-1])
footer=re.findall(r"Ran (\d+) tests across (\d+) files\. \[(.*?)\]",log)[-1]
totals.update(tests=int(footer[0]),files=int(footer[1]),elapsed=footer[2],code=selection["code"],changedTests=len(selection["changedTests"]))
assert totals["files"]==expected
suffix=str(expected)if totals["code"]==0 else str(expected)+"-failed"
for original,archived in [("ifc4x3-and-details-tests.txt",f"tests{suffix}.txt"),("ifc4x3-and-details-selection.json",f"selection{suffix}.json")]:
 assert not(generated/archived).exists(),archived
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
missing=previous-current
superseded=[]
old_name="SELECT refuses oversized or mixed unsupported domains without truncating members"
new_name="SELECT recursively pages actual large domains without inventing required values"
for key,count in list(missing.items()):
 if key[0].endswith("/🔎️details/type_select_input.test.ts")and key[1:]==("pass",old_name):
  replacement=(key[0],"pass",new_name)
  assert count==1 and current[replacement]==1
  superseded.append({"file":key[0],"old":old_name,"new":new_name,"count":count,"authority":"host-type-value-select-supersession.md"})
  del missing[key]
for row in json.loads((ticket/"bim-occurrence-traversal-supersessions.json").read_text(encoding="utf8")):
 key=(row["file"],"pass",row["old"])
 if missing[key]:
  assert missing[key]==row["count"]and current[(row["file"],"pass",row["new"])]==row["count"]
  superseded.append(row)
  del missing[key]
def rows(counter):return [{"file":key[0],"outcome":key[1],"name":key[2],"count":count}for key,count in counter.items()]
comparison={"previousObservations":sum(previous.values()),"currentObservations":sum(current.values()),"unexplainedMissing":rows(missing),"explicitSupersessions":superseded,"added":rows(current-previous)}
(generated/f"checkpoint{suffix}-test-comparison.json").write_text(json.dumps(comparison,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
paths=[path.replace("\\","/")for path in selection["paths"]]
scope={"ifc":sum("/🏅️standards/🔖️4x3/"in path for path in paths),"details":sum("/🔎️details/"in path for path in paths)}
scope["shared"]=expected-scope["ifc"]-scope["details"]
(generated/f"checkpoint{suffix}-totals.json").write_text(json.dumps({"totals":totals,"scope":scope},indent=2)+"\n",encoding="utf8")
status=f"Actual selected{expected} terminal{totals['code']}: {totals['pass']}pass/{totals['skip']}skip/{totals['fail']}fail/{totals['assertions']} Bun expect calls/{totals['tests']}tests/{totals['files']}files/{totals['elapsed']}, {totals['changedTests']} selected test-source hash changes. Scope{scope['ifc']}IFC4x3/{scope['details']}Details/{scope['shared']}shared. Comparison with135 retains {sum(previous.values())-sum(row['count']for row in superseded)-sum(missing.values())} unchanged prior name/outcome observations, explicitly supersedes {sum(row['count']for row in superseded)} specifically documented obligations, has {sum(missing.values())} unexplained missing observations and {sum(row['count']for row in comparison['added'])} added observations (including the replacement)."
report=f"# Actual Selected{expected} Checkpoint\n\n"+status+f"\n\nOriginal output/selection are tests{suffix}.txt and selection{suffix}.json under generated, alongside exact totals and real-observation comparison. Repeated footer skip summaries are excluded. Node assertions remain additional and are not inferred from Bun expect counts. Explicit supersessions retain bounded recursive SELECT paging and declared BIM/FEM fanout semantics, as reviewed in host-type-value-select-supersession.md and bim-occurrence-traversal-supersession.md.\n\nThe failed145 originals remain tests145-initial-failed.txt and selection145-initial-failed.json. All eight actual failures were corrected and independently closed in the axis/TFS21 audit, without rewriting their archived baseline. Current external qualifications retain SDK aggregate normalization/cardinality gaps, placement helper discrepancies and source-mutating arithmetic behavior, rather than silently matching them.\n\nThis is a selected TypeScript/source-law checkpoint. Native type/runtime/funding/receipts and genuine host renderer proof remain unavailable. Complete IFC4x3/all-MVD, finite Boolean IFC dispatch, all geometry/interchange domains and native end-to-end input/publication are still incomplete. Goal/ticket remain active.\n"
(ticket/f"checkpoint{suffix}.md").write_text(report,encoding="utf8")
for name in ["current-state.md","registered-test-scope.md","exchange-profile-current-state.md","post135-current-state.md"]:
 path=ticket/name
 title,separator,body=path.read_text(encoding="utf8").partition("\n\n")
 path.write_text(title+separator+status+" Goal/ticket remain active/incomplete; native and renderer proof unavailable. See checkpoint"+suffix+".md.\n\n"+body,encoding="utf8")
print("[DEBUG]",json.dumps({"totals":totals,"scope":scope,"unexplainedMissing":sum(missing.values()),"superseded":len(superseded),"added":sum(row['count']for row in comparison['added'])}))
assert totals["code"]==0 and totals["fail"]==0 and totals["changedTests"]==0
assert not missing and len(superseded)==3
assert sum(previous.values())==3494 and sum(current.values())==totals["tests"]
