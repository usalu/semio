from pathlib import Path
import re
ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
for anchor,new in [
('  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️numeric/🧪️.test.ts",','  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️vector/🧪️.test.ts",\n  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️placement/🧪️.test.ts",'),
('  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/fem.test.ts",','  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/self-weight/self-weight.test.ts",')]:
 if new not in source:
  assert anchor in source,anchor
  source=source.replace(anchor,anchor+"\n"+new,1)
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle)/[^"\r\n]+)"',source)
assert len(selected)==148,len(selected)
report=ticket/"checkpoint145-integration-correction.md"
text=report.read_text(encoding="utf8").replace("max149-or-less path units (exact census emitted)","maximum134 UTF16 path units")
report.write_text(text,encoding="utf8")
head="Latest full selected145 run failed8 with4547pass/12existing gated skips/198668 Bun expect calls/4567tests/455.85s and0 selected test-source hash changes. Failed archives remain tests145-initial-failed.txt and selection145-initial-failed.json. Last full green remains135. Root corrected placement diagnostic ownership and explicitly repaired the obsolete raw-integer TFS expectation:258/55079 focused and strict7consumers pass, independent21 source audit pending. Registry now148 after independently closed SelfWeight32/vector49/placement46, no full148 result. See checkpoint145-integration-correction.md. Goal/ticket remain active/incomplete."
for name in ["current-state.md","registered-test-scope.md","exchange-profile-current-state.md","post135-current-state.md"]:
 path=ticket/name
 title,separator,body=path.read_text(encoding="utf8").partition("\n\n")
 path.write_text(title+separator+head+"\n\n"+body,encoding="utf8")
print("[DEBUG] closed SelfWeight/vector/placement suites registered; actual selected148, no148 full result")

