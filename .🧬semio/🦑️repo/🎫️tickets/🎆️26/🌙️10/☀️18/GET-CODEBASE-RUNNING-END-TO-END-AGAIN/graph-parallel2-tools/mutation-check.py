#!/usr/bin/env python3
"""🧬️ Mutation check of the project farm laws: copies the farm directory and the inputs module to sibling mutants (never mutates a live file), applies one mutation at a time and runs the copied laws; every mutant must fail."""
import os, shutil, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
L = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
N = L + "/🏗️build/🕸️graph/📥️inference/📦️project-nodes"
INPUTS = L + "/🏗️build/📥️inputs/🟨️.mjs"
MUT_FARM = N + "/🏊️mutant-farm"
MUT_INPUTS = L + "/🏗️build/📥️inputs/🟨️mutant.mjs"
MUTANTS = [
    ("pool", "/🏊️pool/🟨️.mjs", "if (job.attempts >= policy.maximumAttempts) settle(job, { lost: why }); else queue.unshift(job);", "settle(job, { lost: why });"),
    ("pool", "/🏊️pool/🟨️.mjs", "const bounds = [[\"work\", Math.floor(projects / policy.projectsPerWorker)], [\"cpu\", cores - policy.cpuReserve]];", "const bounds = [[\"work\", Math.floor(projects / policy.projectsPerWorker)], [\"cpu\", cores]];"),
    ("pool", "/🏊️pool/🟨️.mjs", "if (message.rssMegabytes > megabytes) {", "if (false) {"),
    ("pool", "/🏊️pool/🟨️.mjs", "if (!accept(message.identity)) { disable(\"identity differs\"); return; }", "accept(message.identity);"),
    ("pool", "/🏊️pool/🟨️.mjs", "return named === undefined ? [...P.languages] : [named];", "return named === undefined ? [P.languages[0]] : [named];"),
    ("pool", "/🏊️pool/🟨️.mjs", "if (closed || disabled !== undefined) return Promise.resolve({ lost:", "if (false) return Promise.resolve({ lost:"),
    ("inputs", None, "if (standing !== undefined && standing.size === bytes.byteLength && readFileSync(target).equals(bytes)) {", "if (false) {"),
    ("inputs", None, "if (standing !== undefined && !block && !record) { plan.present++; continue; }", "if (false) { plan.present++; continue; }"),
    ("inputs", None, "last: record });", "last: false });"),
    ("inputs", None, "createHash(\"sha256\").update(inflated).digest(\"hex\") !== block[1]", "false"),
    ("inputs", None, "createHash(\"sha256\").update(bytes).digest(\"hex\") !== leaf.slice(0, -5)", "false"),
    ("inputs", None, "optional.sort((a, b) => Number(a.last) - Number(b.last));", ""),
    ("inputs", None, "part === \"..\"", "false"),
]
def run():
    out = subprocess.run(["nice", "-n", "10", "bun", "test", "./🟦️.ts"], cwd=MUT_FARM + "/🧪️tests", capture_output=True, text=True, env=dict(os.environ, SEMIO_TEST_ARTIFACT_DIR="/private/tmp/gpp2"))
    return out.returncode, (out.stdout + out.stderr)[-300:]
killed = 0
for index, (kind, rel, old, new) in enumerate(MUTANTS):
    shutil.rmtree(MUT_FARM, ignore_errors=True)
    shutil.copytree(N + "/🏊️project-farm", MUT_FARM)
    shutil.copy(INPUTS, MUT_INPUTS)
    for path in (MUT_FARM + "/🧪️tests/🟦️.ts", MUT_FARM + "/🧪️tests/🧫️fixtures/🛰️adopter/🟨️.mjs"):
        text = open(path, encoding="utf-8").read()
        text = text.replace("📥️inputs/🟨️.mjs", "📥️inputs/🟨️mutant.mjs")
        open(path, "w", encoding="utf-8").write(text)
    target = MUT_INPUTS if kind == "inputs" else MUT_FARM + rel
    text = open(target, encoding="utf-8").read()
    if text.count(old) != 1:
        print("MUTANT", index, "ANCHOR MISSING", old[:60]); continue
    open(target, "w", encoding="utf-8").write(text.replace(old, new))
    code, tail = run()
    print("MUTANT", index, "killed" if code else "SURVIVED", old[:70].replace("\n", " "))
    killed += 1 if code else 0
shutil.rmtree(MUT_FARM, ignore_errors=True)
try: os.remove(MUT_INPUTS)
except OSError: pass
print("killed", killed, "of", len(MUTANTS))
