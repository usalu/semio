# 7. one queued exclusive build lease over the whole rebuild span (the fleet mutex, as a repo-library primitive).
SNIPPETS = ".tmp-ticket/wp-w2/item4"
leases = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🟦️.ts"
replace(leases, 'import { lstatSync, mkdirSync, realpathSync } from "node:fs";', 'import { lstatSync, mkdirSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";')
replace(leases, "/** 🪢️ Orders a resource set consistently and releases it in reverse order on every exit. */", open(f"{SNIPPETS}/queued-lease.snippet.ts", encoding="utf-8").read() + "\n/** 🪢️ Orders a resource set consistently and releases it in reverse order on every exit. */")
lease_law = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts"
final_line = '    console.log("[DEBUG] Resource lease deadlines, cancellation while held, progress failure, ordered multi-resource access, consumer failure, identity/journal and path guards PASS");\n'
replace(lease_law, final_line, open(f"{SNIPPETS}/lease-law.snippet.ts", encoding="utf-8").read() + final_line)

caching = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts"
replace(caching, 'import { stageArtifacts } from "./📦️artifacts/🟦️.ts";', 'import { stageArtifacts } from "./📦️artifacts/🟦️.ts";\nimport { acquireQueuedResourceLease } from "./🔒️leases/🟦️.ts";\nimport { repoCacheDirectory } from "./🟦️.ts";')
replace(caching, "const router = new ScriptRouter(SCRIPT_ROOT)\n  .register(\"test\", TestScript)", open(f"{SNIPPETS}/lease-command.snippet.ts", encoding="utf-8").read() + "const router = new ScriptRouter(SCRIPT_ROOT)\n  .register(\"test\", TestScript)\n  .register(\"lease\", LeaseScript)")

# 8. the chain: one Nx invocation builds each component once for describe AND materialize, then registry, s, verify, catalog.
chain = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🔣️.json"
replace(chain, '    { "id": "describe", "stage": "descriptors", "command": ["nx", "run-many", "-t", "describe", "--exclude", "@semio-tech/os-plugin-describe-rs", "--parallel=1"] },', '    { "id": "components", "stage": "descriptors", "command": ["nx", "run-many", "-t", "describe", "materialize-dev", "--exclude", "@semio-tech/os-plugin-describe-rs", "--parallel=2"] },')
replace(chain, '    { "id": "activate-s", "stage": "guests", "command": ["nx", "run", "@semio-tech/framework-os-dev:activate-s-react-dev"] },', '    { "id": "activate-s", "stage": "guests", "command": ["nx", "run", "@semio-tech/framework-os-dev:activate-s-react-dev"] },\n    { "id": "verify-s", "stage": "guests", "command": ["nx", "run", "@semio-tech/plugin-registry:verify-staged", "--variant", "s"] },')

REBUILD_SHA = "6a5d2e5c449a699ac0c8da0698eb239b6162433dc6ea1d332b6e60e55e771787"
rebuild = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts"
import hashlib
if hashlib.sha256(open(rebuild, "rb").read()).hexdigest() != REBUILD_SHA:
    raise SystemExit(f"CHANGED {rebuild}: re-derive rebuild.new.ts from the current file")
edits[rebuild] = open(f"{SNIPPETS}/rebuild.new.ts", encoding="utf-8").read()
rebuild_law = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🔁️rebuild/🟦️.ts"
replace(rebuild_law, 'expect(chain.map((step) => step.id)).toEqual(["describe", "generate", "check", "activate-s", "publish-catalog"]);', 'expect(chain.map((step) => step.id)).toEqual(["describe", "generate", "check", "activate-s", "publish-catalog"]);')
edits[rebuild_law] = open(f"{SNIPPETS}/rebuild-law.new.ts", encoding="utf-8").read()

registry_script = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts"
replace(registry_script, 'import { RebuildAllScript } from "./🔁️rebuild/🟦️.ts";', 'import { RebuildAllScript, VerifyStagedScript } from "./🔁️rebuild/🟦️.ts";')
replace(registry_script, '.register("rebuild-all", RebuildAllScript)', '.register("rebuild-all", RebuildAllScript).register("verify-staged", VerifyStagedScript)')
registry_project = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json"
replace(registry_project, '''    "rebuild-all": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry",
        "command": "bun ./📜️script.ts rebuild-all",
        "forwardAllArgs": true
      },
      "outputs": []
    },''', '''    "rebuild-all": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry",
        "command": "bun ./📜️script.ts rebuild-all",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "verify-staged": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry",
        "command": "bun ./📜️script.ts verify-staged",
        "forwardAllArgs": true
      },
      "outputs": []
    },''')
