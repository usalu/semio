"""🧷 builds staged/witnesshost/spec.json: the world witness names no runtime; the hosting runtime's own members come from a table in its adapter directory."""
import json, os
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
W = L + "/🏗️build/🔭️witness"
lines = open(W + "/🟨️.mjs", encoding="utf8").read().split("\n")
def block(first_marker, last_marker):
    start = next(i for i, l in enumerate(lines) if l.startswith(first_marker))
    end = next(i for i in range(start, len(lines)) if lines[i].startswith(last_marker))
    return "\n".join(lines[start:end + 1])
old_bun = block(" if (typeof Bun !== \"undefined\") {", " }")
assert old_bun.count("\n") == 6, old_bun.count("\n")
new_bun = '''\
 const hostBinding = config.host, hostRuntime = hostBinding === undefined ? undefined : globalThis[hostBinding.namespace];
 if (hostRuntime !== undefined) {
  const label = hostBinding.namespace;
  for (const name of hostBinding.spawn) patch(hostRuntime, name, spawned(label + "." + name));
  for (const name of hostBinding.unobserved) patch(hostRuntime, name, function_ => function (...args) { if (name === hostBinding.write) markWrite(typeof args[0] === "string" || args[0] instanceof URL ? args[0] : undefined); else taint("unobserved " + label + "." + name); return function_.apply(this, args); });
  patch(hostRuntime, hostBinding.fileReader, function_ => function (...args) { if (typeof args[0] === "string" || args[0] instanceof URL) observeBefore("f", args[0]); else if (args[0] !== 1 && args[0] !== 2) taint("unobserved " + label + "." + hostBinding.fileReader + " of " + (args[0] === null ? "null" : typeof args[0] === "object" ? (args[0].constructor && args[0].constructor.name) : typeof args[0] + " " + String(args[0]))); return function_.apply(this, args); });
  try { const glob = hostRuntime[hostBinding.glob]; if (glob !== undefined && glob.prototype !== undefined) for (const name of ["scan", "scanSync"]) patch(glob.prototype, name, function_ => function (...args) { taint("unobserved " + label + "." + hostBinding.glob + "." + name); return function_.apply(this, args); }); } catch { taint("cannot observe " + label + "." + hostBinding.glob); }
 }'''
# the original file reader body must be kept byte for byte apart from the names: assert the old text still says what the new text generalises
assert 'taint("unobserved Bun.file of "' in old_bun and 'taint("unobserved Bun.Glob."' in old_bun
files = {}
def edit(file, old, new, count=1): files.setdefault(file, []).append({"old": old, "new": new, "count": count})
f = W + "/🟨️.mjs"
edit(f, 'import schema from "./🧬️schema/🔣️.json" with {type: "json"};\n', 'import schema from "./🧬️schema/🔣️.json" with {type: "json"};\nimport {readFileSync} from "node:fs";\nimport {runtimeAdapterNameV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";\n')
edit(f, old_bun, new_bun)
edit(f, '  const sqlite = load("bun:sqlite");\n', '  const sqlite = hostBinding === undefined || hostBinding.sqlite === undefined ? undefined : load(hostBinding.sqlite);\n')
edit(f, '} catch { /* a runtime without bun:sqlite has nothing to observe */ }', '} catch { /* a runtime without a built-in database has nothing to observe */ }')
edit(f, ' return {[policy.config]: JSON.stringify(config)};\n}', ' config.host = worldWitnessHostBindingV1();\n return {[policy.config]: JSON.stringify(config)};\n}')
edit(f, '/** 🧾️ The environment a child needs to run under the witness;', '''/** 🍞️ The members of the hosting runtime that the witness must observe, read from the table in the adapter directory of that runtime next to this module (`🍞️bun/🔣️.json` for Bun), so this file names no runtime; undefined when the runtime has no adapter directory and nothing of its own to observe. */
export function worldWitnessHostBindingV1() {
 try { return JSON.parse(readFileSync(new URL(`./${runtimeAdapterNameV1()}/🔣️.json`, import.meta.url), "utf8")); } catch { return undefined; }
}

/** 🧾️ The environment a child needs to run under the witness;''')
f = W + "/🟨️.d.mts"
edit(f, 'export function worldWitnessEnvironmentV1(', 'export function worldWitnessHostBindingV1(): Readonly<{ namespace: string; spawn: readonly string[]; unobserved: readonly string[]; write: string; fileReader: string; glob: string; sqlite?: string }> | undefined;\nexport function worldWitnessEnvironmentV1(')
f = W + "/🧬️schema/🔣️.json"
edit(f, '  "definitions": {\n    "Policy": {', '''  "definitions": {
    "HostBinding": {
      "type": "object",
      "additionalProperties": false,
      "required": ["namespace", "spawn", "unobserved", "write", "fileReader", "glob"],
      "properties": {
        "namespace": {"type": "string", "minLength": 1},
        "spawn": {"type": "array", "items": {"type": "string", "minLength": 1}, "minItems": 1},
        "unobserved": {"type": "array", "items": {"type": "string", "minLength": 1}},
        "write": {"type": "string", "minLength": 1},
        "fileReader": {"type": "string", "minLength": 1},
        "glob": {"type": "string", "minLength": 1},
        "sqlite": {"type": "string", "minLength": 1}
      }
    },
    "Policy": {''')
f = W + "/🧪️tests/🟦️.ts"
files_new = {W + "/🍞️bun/🔣️.json": "host-binding.json"}
json.dump({"files": files, "new": files_new}, open(C + "/red-g-tools/staged/witnesshost/spec.json", "w"), ensure_ascii=False, indent=1)
open(C + "/red-g-tools/staged/witnesshost/files/host-binding.json", "w", encoding="utf8").write('''{
  "namespace": "Bun",
  "spawn": ["spawn", "spawnSync", "$"],
  "unobserved": ["which", "resolve", "resolveSync", "build", "write"],
  "write": "write",
  "fileReader": "file",
  "glob": "Glob",
  "sqlite": "bun:sqlite"
}
''')
