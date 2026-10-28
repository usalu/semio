"""🛂️ Malformed graph writer refusal precedes any physical STEP output claim."""
from pathlib import Path

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/boolean-expression.test.ts"
source=path.read_text(encoding="utf8")
old="const fixture=ifcBooleanExpressionFixture(row),before=structuredClone(fixture.snapshot),step=await writeIfc4x3Step(fixture.snapshot),result=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096});"
new="const fixture=ifcBooleanExpressionFixture(row),before=structuredClone(fixture.snapshot);if(row.physical==='malformed'){await expect(writeIfc4x3Step(fixture.snapshot)).rejects.toThrow('step.graph');const rejected=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096});expect(rejected.artifact).toBeUndefined();expect(rejected.diagnostics.some(item=>item.code==='exchange.graph'&&item.severity==='error')).toBe(true);expect(fixture.snapshot).toEqual(before);return;}const step=await writeIfc4x3Step(fixture.snapshot),result=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096});"
assert source.count(old)==1
path.write_text(source.replace(old,new),encoding="utf8")
print("[DEBUG] dangling source law now expects actual central writer refusal")
