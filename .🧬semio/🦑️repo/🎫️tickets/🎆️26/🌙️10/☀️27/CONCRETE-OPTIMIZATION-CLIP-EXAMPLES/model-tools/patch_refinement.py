from pathlib import Path
clip = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
def swap(path, old, new):
    text = path.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    path.write_text(text.replace(old, new), encoding="utf-8")
swap(clip / "🧮️model/🧬️schema/🔣️.schema.json", '''          "required": [
            "member",
            "design"
          ],
          "additionalProperties": false,
          "properties": {
            "member": {
              "type": "string",
              "minLength": 1
            },
            "design": {
              "$ref": "#/$defs/designPartial"
            }
          }''', '''          "required": [
            "member"
          ],
          "additionalProperties": false,
          "properties": {
            "member": {
              "type": "string",
              "minLength": 1
            }
          }''')
swap(Path(__file__).parent / "build_schema.py", '"refinement": strict({"member": text, "design": reference("designPartial")})', '"refinement": strict({"member": text})')
swap(clip / "🧮️model/🐍️.py", '''    refine = setup["refinement"]
    refined = evaluate({**base, **refine["design"]})
    candidate = {"cost": refined["cost"], "electricity": refined["electricity"]}
    refinement = {"member": refine["member"], "design": {**base, **refine["design"]}, "cost": refined["cost"], "electricity": refined["electricity"],''', '''    optimum = surface()["minimum"]["design"]
    refined = evaluate(optimum)
    candidate = {"cost": refined["cost"], "electricity": refined["electricity"]}
    refinement = {"member": setup["refinement"]["member"], "design": optimum, "cost": refined["cost"], "electricity": refined["electricity"],''')
