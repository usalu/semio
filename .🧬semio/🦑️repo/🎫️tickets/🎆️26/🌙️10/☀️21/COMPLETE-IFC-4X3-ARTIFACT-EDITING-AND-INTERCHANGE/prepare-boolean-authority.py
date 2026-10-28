from pathlib import Path
import json, hashlib
ticket=Path(__file__).parent
root=ticket.parents[6]
artifact=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
boolean=artifact/"📐️geometry/🧊️boolean"
tests=boolean/"🧪️tests"
rows=[{"operation":operation,"observer":observer,"phase":phase,"outcome":outcome} for operation in ["difference","union","intersection","halfspace"] for observer,outcomes in [("tick",["error","undefined"]),("progress",["error","undefined","false"])] for phase in ["initial","interior","completion"] for outcome in outcomes]
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":60,"maxItems":60,"items":{"type":"object","additionalProperties":False,"required":["operation","observer","phase","outcome"],"properties":{"operation":{"enum":["difference","union","intersection","halfspace"]},"observer":{"enum":["tick","progress"]},"phase":{"enum":["initial","interior","completion"]},"outcome":{"enum":["error","undefined","false"]}},"allOf":[{"if":{"properties":{"observer":{"const":"tick"}}},"then":{"properties":{"outcome":{"enum":["error","undefined"]}}}}]}}}}
for name,data in [("control-authority.json",{"version":1,"cases":rows}),("control-authority.schema.json",schema)]:
 (tests/name).write_text(json.dumps(data,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
registration=artifact/"📦️packages/🟦️typescript/📜️script.ts"
text=registration.read_text(encoding="utf8")
for anchor,new in [
 ('  "🔎️details/type_set_select_input.test.ts",','  "🔎️details/type_measure_select_input.test.ts",'),
 ('  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/csg-block.test.ts",','  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/csg-round.test.ts",\n  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/geometry-domain.test.ts",')]:
 if new not in text:
  assert anchor in text,anchor
  text=text.replace(anchor,anchor+"\n"+new,1)
registration.write_text(text,encoding="utf8")
source=boolean/"🟦️.ts"
plan="# Cooperative Boolean Control Plan\n\nThe actual shared TypeScript Boolean core is synchronous and repeatedly reads caller-mutable callbacks. Introduce closed language-neutral60 callback authority obligations before implementation: four real public operations, original tick/progress observers, initial/interior/completion boundaries and exact Error/undefined/false behavior. Every law mutates the original options object during its initial callback; later replacements must not redirect original cancellation/fault authority. Getters must be captured once before work. Inputs and results remain private on failure. These laws use existing public operations, so red evidence does not depend on an absent proposed API.\n\nThen expose one cooperative generator evaluation behind the existing synchronous drain and genuine async consumers. All source-sized BSP, clipping, welding, output, sorting, copying, normalization and validation loops must yield; explicit traversal frames avoid JS recursion limits. Halfspace construction and evaluation spend one original cumulative captured budget. Preserve existing Manifold/Three corroboration, geometric semantics and no external runtime dependencies. Add direct step/async cancellation and host-yield laws after the foundation. Native runtime remains unproved.\n\nOriginal Boolean source SHA256: "+hashlib.sha256(source.read_bytes()).hexdigest()+". Original source and existing tests were read after terminal rootdomain60 audit release; no held-source edit occurred. Repo CLI/MCP ingress remains unavailable; direct bounded runtime fallback is retained without modifying build infrastructure.\n"
(ticket/"boolean-cooperation-plan.md").write_text(plan,encoding="utf8")
print("[DEBUG] created60 neutral authority rows and registered three independently closed suites")

