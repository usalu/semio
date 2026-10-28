const fs = require("fs");
const path = require("path");
const target = "🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex";
const source = fs.readFileSync(target, "utf8");
const before = String.raw`\newcommand{\AssignmentLedger}[2]{
  \par\noindent
  \begingroup
  \renewcommand{\AssignmentLedgerRows}{#1}
  \AssignmentLedgerColumnWidth=\dimexpr\linewidth/2\relax
  \begin{tikzpicture}[x=\AssignmentLedgerColumnWidth,y=1mm]`;
const after = String.raw`\newcommand{\AssignmentLedger}[2]{
  \par
  \begingroup
  \renewcommand{\AssignmentLedgerRows}{#1}
  \AssignmentLedgerColumnWidth=\dimexpr\linewidth/2\relax
  \noindent\begin{tikzpicture}[x=\AssignmentLedgerColumnWidth,y=1mm]`;
const beforeBaseline = String.raw`at ({.18+\AssignmentColumn},{-6.3*\AssignmentRow-1.5}) {#3};`;
const afterBaseline = String.raw`at ({.18+\AssignmentColumn},{-6.3*\AssignmentRow-.7}) {#3};`;
if (!source.includes(before)||!source.includes(beforeBaseline)) throw Error("Expected exact ledger layout missing");
const result = source.replace(before,after).replace(beforeBaseline,afterBaseline);
const staged = path.join(__dirname,"🗑️generated","ledger-geometry.new.tex");
fs.mkdirSync(path.dirname(staged),{recursive:true});
fs.writeFileSync(staged,result);
if(fs.readFileSync(target,"utf8")!==source) throw Error("Concurrent geometry edit");
fs.renameSync(staged,target);
const edit = path.join(__dirname,"ledger-geometry.edit.cjs");
const initial = fs.readFileSync(edit,"utf8");
fs.writeFileSync(edit,initial.replace(before,after).replace(beforeBaseline,afterBaseline));
fs.appendFileSync(path.join(__dirname,"ledger-geometry.md"),"\nNative DE and EN compilation exposed one normal space before the full width ledger (2.13867 pt overfull). Moving noindent after setup keeps row count assignment whitespace in vertical mode. Value baselines are lifted 0.8 mm above the original rules to clear typographic descenders; arrow identifiers, rule positions and dimensions remain unchanged.\n");
process.stdout.write("[DEBUG] Ledger whitespace and descender clearance refined atomically.\n");
