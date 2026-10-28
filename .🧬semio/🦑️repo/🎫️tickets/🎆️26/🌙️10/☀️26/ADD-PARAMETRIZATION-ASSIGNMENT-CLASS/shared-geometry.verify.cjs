const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const fixture = JSON.parse(fs.readFileSync(path.join(__dirname, "shared-geometry.case.json"), "utf8"));
const source = fs.readFileSync("🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex", "utf8");
for (const shape of fixture.shapes) assert.ok(source.includes("\\" + shape.macro + "{" + shape.L + "}{" + shape.B + "}{" + shape.H + "}"), shape.macro);
for (const [index, parameter] of ["L", "B", "H"].entries()) {
  assert.ok(source.includes("\\AssignmentParameterMatrixRow{#" + (index + 1) + "}{" + fixture.shapes.map(shape => shape[parameter]).join(",") + "}"), parameter);
  assert.equal(new Set(fixture.shapes.map(shape => shape[parameter])).size, 2, parameter + " has two equal values and one odd value");
}
assert.deepEqual(["L", "B", "H"].map(key => fixture.shapes[0][key]), ["L", "B", "H"].map(key => fixture.shapes[1][key]));
assert.equal(new Set(fixture.shapes.map(shape => shape.macro)).size, 3);
assert.ok(source.includes("type-columns=3,parameter-rows=4,header-height=20mm"));
assert.ok(source.includes("\\AssignmentParameterMatrixRow{#4}{#5,#6,#7}"));
assert.ok(source.includes("\\newcommand{\\AssignmentWorkedExampleTable}[7]"));
assert.ok(source.includes("\\newcommand{\\AssignmentWorkedExampleGraph}[6]"));
assert.ok(source.includes("atan(.45*#2/#1)"));
assert.ok(source.includes("{#3+#2*.175+#2*.175*sin(\\x)}"));
for (const [a, b, argument] of [[1,2,1],[2,1,2],[2,3,3],[3,2,4],[1,3,5],[3,1,6]]) assert.ok(source.includes("\\AssignmentWorkedExampleGraphEdge{" + a + "}{" + b + "}{#" + argument + "}"));
assert.ok(source.includes("\\AssignmentGraphRadiusY=53mm"));
console.log("Shared geometry contracts passed.");
