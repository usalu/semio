from pathlib import Path
ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
new='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/bim/properties/properties.test.ts",'
if new not in source:
 anchor='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/bim.test.ts",'
 assert anchor in source
 path.write_text(source.replace(anchor,anchor+"\n"+new,1),encoding="utf8")
print("[DEBUG] registered independently closed BIM96; expanded registry141, no141 full result yet")

