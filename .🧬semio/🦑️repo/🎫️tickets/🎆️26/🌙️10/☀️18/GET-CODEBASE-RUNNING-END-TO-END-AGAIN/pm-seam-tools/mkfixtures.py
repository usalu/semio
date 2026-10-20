#!/usr/bin/env python3
"""🧫️ Cuts the committed lock fixtures: excerpts of real tool output from this repository (provenance recorded per entry) plus rows composed in the exact tool format where no real lock holds the case."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import paths
R = paths.ROOT
def read(rel): return open(os.path.join(R, rel), encoding="utf-8").read()
tools = read(paths.LIBRARY + "/⚡️caching/🚀️bootstrap/🛠️tools/bun.lock").split("\n")
def row(name): return next(l for l in tools if l.startswith('    "%s": [' % name))
bun_header = '''{
  "lockfileVersion": 1,
  "configVersion": 1,
  "workspaces": {
    "": {
      "name": "@semio-tech/nx-bootstrap-tools",
      "dependencies": {
        "typescript": "5.9.3",
      },
    },
    "packages/member": {
      "name": "@scope/member",
      "version": "0.0.0",
    },
  },
  "packages": {
    "@scope/member": ["@scope/member@workspace:packages/member"],

'''
bun = bun_header + "\n\n".join(row(n) for n in ["@babel/code-frame", "@babel/compat-data", "@iarna/toml", "@babel/core/semver", "typescript"]) + "\n  }\n}\n"
cargo_blocks = read("🌎️hub/🧩️compositions/💠️lowpoly/📦️packages/🦀️rust/Cargo.lock").split("\n[[package]]\n")
cargo = cargo_blocks[0] + "".join("\n[[package]]\n" + b for b in cargo_blocks[1:5])
cargo += '\n[[package]]\nname = "member"\nversion = "0.1.0"\ndependencies = [\n "anyhow",\n]\n'
go_real = open(os.path.join(R, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️20/INVESTIGATE-SLOW-INTEGRATION-TESTS-IN-REPO-CLI-GO/scratch/gitignore-check/go.sum"), encoding="utf-8").read()
fixtures = {
 "bun": {"tools-excerpt": {"provenance": "excerpt of the tool-generated tools recipe lock in the build library, plus one workspace member in the same format", "text": bun}},
 "npm": {"workspaces": {"provenance": "composed in package-lock v3 format (replaced by real npm output when generated)", "text": json.dumps({"name": "root", "version": "1.0.0", "lockfileVersion": 3, "requires": True, "packages": {
    "": {"name": "root", "version": "1.0.0", "workspaces": ["packages/*"], "dependencies": {"left-pad": "1.3.0"}},
    "node_modules/left-pad": {"version": "1.3.0", "resolved": "https://registry.npmjs.org/left-pad/-/left-pad-1.3.0.tgz", "integrity": "sha512-XI5MPzVNApjAyhQzphX8BkmKsKUxD4LdyK24iZeQGinBN9yTQT3bFlCBy/aVx2HrNcqQGsdot8ghrjyrvMCoEA==", "deprecated": "use String.prototype.padStart()"},
    "node_modules/member": {"resolved": "packages/member", "link": True},
    "packages/member": {"name": "member", "version": "0.0.0"}}}, indent=2) + "\n"}},
 "cargo": {"lowpoly-excerpt": {"provenance": "first rows of the tool-generated lowpoly composition lock plus one workspace crate", "text": cargo}, "repo-test-host": {"provenance": "tool-generated lock of the repo test host crate (workspace crate only)", "text": read(paths.LIBRARY + "/🧪️test/📦️packages/🦀️rust/Cargo.lock") if False else read("🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.lock")}},
 "go": {"real-doublestar": {"provenance": "tool-generated go.sum of a ticket scratch module", "text": go_real},
        "composed-escape": {"provenance": "composed rows in go.sum format: an upper-case module path, a go.mod-only row and a zip row", "text": "github.com/BurntSushi/toml v1.3.2 h1:o7IhLm0Msx3BaB+n3Ag7L8EVlByGnpq14C4YWiu/gL8=\ngithub.com/BurntSushi/toml v1.3.2/go.mod h1:CxXYINrC8qIiEnFrOxCa7Jy5BFHlXnUU2pbicEuybxQ=\ngolang.org/x/mod v0.17.0/go.mod h1:hTbmBsO62+eylJbnUtE2MGJUyE7QWk4xUqPFrRgJ+7c=\n"}},
 "uv": {"styling": {"provenance": "tool-generated lock of the styling python package", "text": read("🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/uv.lock")}},
 "dotnet": {"empty-real": {"provenance": "tool-generated restore lock of the dotnet test package (no dependencies)", "text": read("🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/packages.lock.json")},
            "composed-frameworks": {"provenance": "composed in restore lock v1 format: two target frameworks, direct, transitive and project rows", "text": json.dumps({"version": 1, "dependencies": {
              "net8.0": {"Newtonsoft.Json": {"type": "Direct", "requested": "[13.0.3, )", "resolved": "13.0.3", "contentHash": "HrC5BXdl00IP9zeV+0Z848QWPAoCr9P3bDEZguI+gkLcBKAOxix/tLEAAHC+UvDNPv4a2d18lOReHMOagPa+zQ=="},
                          "Lib": {"type": "Project"}, "System.Memory": {"type": "Transitive", "resolved": "4.5.5", "contentHash": "XIWiDvKPXaTveaB7HVhmF+lJi3p2yGg2IFHHuoCBXgeDiFpzzcDw7XlCHRRqzC+J/Y5Sc1tM6dRDaTBC4lbNBg=="}},
              "net9.0": {"Newtonsoft.Json": {"type": "Direct", "requested": "[13.0.3, )", "resolved": "13.0.3", "contentHash": "HrC5BXdl00IP9zeV+0Z848QWPAoCr9P3bDEZguI+gkLcBKAOxix/tLEAAHC+UvDNPv4a2d18lOReHMOagPa+zQ=="}}}}, indent=2) + "\n"}},
}
GEN = "/private/tmp/pm-seam/gen/out"
REAL = {"bun": ("bun.lock", "bun.version"), "npm": ("package-lock.json", "npm.version"), "cargo": ("Cargo.lock", "cargo.version"), "go": ("go.sum", "go.version"), "uv": ("uv.lock", "uv.version"), "dotnet": ("packages.lock.json", "dotnet.version")}
SAVED = os.path.join(paths.HERE, "real-locks.json")
if os.path.isdir(GEN):
    saved = {}
    for manager, (lock, version) in REAL.items():
        if os.path.exists(os.path.join(GEN, lock)):
            saved[manager] = {"provenance": "real %s output (%s) generated offline from a two-member local project by gen-fixtures.sh" % (manager, open(os.path.join(GEN, version)).read().strip()), "text": open(os.path.join(GEN, lock), encoding="utf-8").read()}
    json.dump(saved, open(SAVED, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
for manager, row in json.load(open(SAVED, encoding="utf-8")).items():
    fixtures[manager]["real-members"] = row
for rows in fixtures.values():
    for row in rows.values():
        if "composed" in row["provenance"] or "replaced by real" in row["provenance"]:
            row["provenance"] += "; hash values are test data"
path = os.path.join(paths.NEW, paths.MANAGERS, "🧪️tests", "🧫️fixtures", "🔣️.json")
os.makedirs(os.path.dirname(path), exist_ok=True)
with open(path, "w", encoding="utf-8") as handle:
    json.dump(fixtures, handle, ensure_ascii=False, indent=2); handle.write("\n")
print(os.path.getsize(path))
