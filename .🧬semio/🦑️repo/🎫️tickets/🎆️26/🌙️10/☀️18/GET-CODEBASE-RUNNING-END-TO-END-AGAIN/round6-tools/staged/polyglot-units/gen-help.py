#!/usr/bin/env python3
"""Builds /private/tmp/polyglot-units/ov5 (the live build library with the operations directory copied and wired), regenerates the help snapshot fixture of the operations suite into wiring/help-fixture.json (plus its recorded base digest) and runs the operations suite on the wired copy. One bun process at a time."""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import wiring
R = "/Users/ueli/Documents/semio"; F = "🧰️framework"; L = F + "/🛍️products/🦑️repo/🔨️modules/📚️library"
O = "/private/tmp/polyglot-units/ov5"
shutil.rmtree(O, ignore_errors=True)
os.makedirs(O + "/" + F + "/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build")
os.symlink(R + "/node_modules", O + "/node_modules")
os.symlink(R + "/" + F + "/🔨️modules", O + "/" + F + "/🔨️modules")
os.symlink(R + "/package.json", O + "/package.json")
for name in os.listdir(R + "/" + L):
    if name != "🏗️build": os.symlink(R + "/" + L + "/" + name, O + "/" + L + "/" + name)
for name in os.listdir(R + "/" + L + "/🏗️build"):
    if name != "⚙️operations": os.symlink(R + "/" + L + "/🏗️build/" + name, O + "/" + L + "/🏗️build/" + name)
shutil.copytree(R + "/" + L + "/🏗️build/⚙️operations", O + "/" + L + "/🏗️build/⚙️operations")
ops = O + "/" + L + "/🏗️build/⚙️operations"
def edit(path, fn):
    text = open(path, encoding="utf8").read(); open(path, "w", encoding="utf8").write(fn(text))
edit(ops + "/🔣️catalog.json", wiring.catalog_edit); edit(ops + "/🧬️schema/🔣️.json", wiring.schema_edit)
fixture = ops + "/🧪️tests/🧫️fixtures/📖️help/🔣️.json"
base = hashlib.sha256(open(fixture, "rb").read()).hexdigest()
generator = os.path.join(O, "gen.ts")
open(generator, "w", encoding="utf8").write('''import catalog from "./%s/🏗️build/⚙️operations/🔣️catalog.json" with {type: "json"};
import {readBuildHelpRequestV1, renderBuildHelpV1, type BuildHelpCatalogV1} from "./%s/🏗️build/⚙️operations/📖️help/🟦️.ts";
const help = catalog as BuildHelpCatalogV1, entry = (name: string) => name === "root" ? [] : name.split(" ");
const rows = ["root", ...help.operations.flatMap(row => [row.name, ...(row.subcommands ?? []).map(sub => row.name + " " + sub.name)])];
const snapshots: Record<string, Record<string, string[]>> = {};
for (const language of help.languages) { snapshots[language] = {}; for (const row of rows) snapshots[language]![row] = renderBuildHelpV1(help, readBuildHelpRequestV1(help, entry(row)), language).split("\\n"); }
process.stdout.write(JSON.stringify({version: 1, snapshots}));
''' % (L, L))
out = subprocess.run(["nice", "-n", "10", "bun", generator], cwd=O, capture_output=True, text=True)
if out.returncode: sys.exit(out.stderr)
new = json.loads(out.stdout)
old = json.load(open(fixture, encoding="utf8"))
for language in old["snapshots"]:
    for row, lines in old["snapshots"][language].items():
        if row in ("root",): continue
        assert new["snapshots"][language][row] == lines, "regeneration changed an existing snapshot: " + language + " " + row
text = json.dumps(new, ensure_ascii=False, indent=2) + "\n"
os.makedirs(HERE + "/wiring", exist_ok=True)
open(HERE + "/wiring/help-fixture.json", "w", encoding="utf8").write(text)
open(HERE + "/wiring/help-fixture.base", "w").write(base + "\n")
open(fixture, "w", encoding="utf8").write(text)
print("wrote wiring/help-fixture.json; base", base[:12], "rows", len(new["snapshots"]["en"]))
test = subprocess.run(["nice", "-n", "10", "bun", "test", "./" + L + "/🏗️build/⚙️operations/🧪️tests/🟦️.ts"], cwd=O, capture_output=True, text=True)
print(test.stdout[-400:] + test.stderr[-1500:])
sys.exit(test.returncode)
