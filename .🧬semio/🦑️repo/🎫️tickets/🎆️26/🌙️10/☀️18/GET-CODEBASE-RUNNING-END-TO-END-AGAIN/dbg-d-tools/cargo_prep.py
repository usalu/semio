import sys;sys.path.insert(0,sys.argv[1]+"/dbg-d-tools")
from edit import edit,find
edit("🗂️workspaces/🦀️cargo/🟦️.ts",[('export function logCargoPreparationDiagnosticV1(phase: CargoPreparationDiagnosticPhaseV1, owner: string, elapsedMs: number, items: number): void {\n}','export function logCargoPreparationDiagnosticV1(phase: CargoPreparationDiagnosticPhaseV1, owner: string, elapsedMs: number, items: number): void {\n  console.error("[cargo-preparation-timing] " + JSON.stringify({ phase, owner, elapsedMs, items }));\n}')])
p=find("🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts")
t=open(p,encoding="utf8").read();n=t.count("[DEBUG] cargo-preparation ");print(n)
open(p,"w",encoding="utf8").write(t.replace("[DEBUG] cargo-preparation ","[cargo-preparation-timing] "))
