"""📚️ Reconcile two specifically evidenced traversal obligations without rewriting runtime archives."""
from pathlib import Path
import json,shutil,hashlib

ticket=Path(__file__).parent
generated=ticket/"🗑️generated"
path=generated/"checkpoint154-test-comparison.json"
before=json.loads(path.read_text(encoding="utf8"))
assert len(before["unexplainedMissing"])==2
archive=generated/"checkpoint154-initial-retention.json"
assert not archive.exists()
shutil.copyfile(path,archive)
mapping=json.loads((ticket/"bim-occurrence-traversal-supersessions.json").read_text(encoding="utf8"))
for row in mapping:
 old={"file":row["file"],"outcome":"pass","name":row["old"],"count":row["count"]}
 new={"file":row["file"],"outcome":"pass","name":row["new"],"count":row["count"]}
 assert old in before["unexplainedMissing"]and new in before["added"]
 before["unexplainedMissing"].remove(old)
 before["explicitSupersessions"].append(row)
assert not before["unexplainedMissing"]and len(before["explicitSupersessions"])==3
source=ticket.parents[6]/mapping[0]["file"]
assert hashlib.sha256(source.read_bytes()).hexdigest()=="4816f70065cd622243d0d865c282e4b5d6044b4a45e7766217b2ced336d5010b"
before["retentionAmendmentAuthority"]="bim-occurrence-traversal-supersession.md"
before["initialComparisonArchive"]="checkpoint154-initial-retention.json"
path.write_text(json.dumps(before,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
status="Actual selected154 GREEN terminal0:4871pass/12existing gated skips/0fail/372425 Bun expect calls/4883tests/154files/534.80s,0 selected test-source hash changes. Scope108IFC4x3/32Details/14shared. Compared with135:3491unchanged prior name/outcome observations,3explicit documented supersessions (oneSELECT,twoBIM/FEMfanout),0unexplainedmissing,1392added observations including3replacements (=1389new)."
report="# Actual Selected154 Green Checkpoint\n\n"+status+"\n\nOriginal tests154.txt and selection154.json, exact totals and final comparison remain under generated. The initial retention comparison correctly refused two then-unexplained names; checkpoint154-initial-retention.json preserves it. The reviewed authority bim-occurrence-traversal-supersession.md explains exact original/current hashes and assertions. BIM now requires all257generic properties plus unchanged spatial core; FEM semantics/phase/threshold remain unchanged under a shared renamed title. host-type-value-select-supersession.md explains recursive31-choice paging replacing obsolete >32 refusal. No runtime archive or test was renamed/weakened to satisfy retention.\n\nLast older full green135 and failed145 originals remain untouched. All eight145failures are independently closed by the axis/TFS21 correction. This154run includes the registered completeValue37, weights32, SelfWeight32, vector49, placement46, arithmetic/sourceisolation50, cumulativeBoolean32 and logicalworkspace36 additions. It excludes new unregistered projection44, surface/curve66, primitive8 and the explicit red kernel61/finite31/nested14 producer inputs.\n\nThis is a selected TypeScript/source-law checkpoint. Node assertions remain additional. NativeValue94 has separate source-only independent closure; authentic Cargo still101 before rustc, so no native type/runtime/funding/receipt or hostrenderer proof. Full IFC4x3/all-MVD goal/ticket remain active/incomplete. Actual shared-kernel workspace enforcement, owned mesh transfer, finite IFC Boolean dispatch, rounded profile, whole view authority and other remaining geometry/interchange domains still require work.\n"
(ticket/"checkpoint154.md").write_text(report,encoding="utf8")
for name in ["current-state.md","registered-test-scope.md","exchange-profile-current-state.md","post135-current-state.md"]:
 path=ticket/name
 title,separator,body=path.read_text(encoding="utf8").partition("\n\n")
 path.write_text(title+separator+status+" Goal/ticket remain active/incomplete; see checkpoint154.md for proof boundaries.\n\n"+body,encoding="utf8")
print("[DEBUG] green154 retention reconciled:3491unchanged+3explicit+1389new;0unexplained")
