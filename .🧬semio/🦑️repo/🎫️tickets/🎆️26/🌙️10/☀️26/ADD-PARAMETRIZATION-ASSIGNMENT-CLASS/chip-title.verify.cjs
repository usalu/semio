const fs = require("fs");
const path = require("path");
const ticket = __dirname;
const contract = JSON.parse(fs.readFileSync(path.join(ticket, "chip-title.case.json"), "utf8"));
let root = ticket;
while (!fs.existsSync(path.join(root, "🧰️framework/🛍️products/📓️print"))) root = path.dirname(root);
const base = path.join(root, "🧰️framework/🛍️products/📓️print/🖋️latex");
const assignment = fs.readFileSync(path.join(base, "semio-assignment.sty"), "utf8");
const window = fs.readFileSync(path.join(base, "semio-window.sty"), "utf8");
const slash = String.fromCharCode(92);
for (const scenario of contract.cases) {
  if (scenario.context === "AssignmentTable") {
    if (!assignment.includes(slash + "bool_set_true:N " + slash + "l_semio_assignment_matrix_active_bool")) throw Error(scenario.name + ": explicit active context required");
    const branches = assignment.split(slash + "bool_if:NT " + slash + "l_semio_assignment_matrix_active_bool").length - 1;
    if (branches !== 2) throw Error(scenario.name + ": open and close must use active context");
  }
  if (!scenario.leftChip && scenario.context === "AssignmentTable") {
    if (!window.includes(slash + "if" + slash + "relax" + slash + "detokenize" + slash + "expandafter{" + slash + "semio@window@header@titleval}" + slash + "relax")) throw Error(scenario.name + ": empty title must omit title chip");
  }
}
console.log("PASS " + contract.cases.length + " title and context contracts");
