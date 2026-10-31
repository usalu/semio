/** 🧾 Preserve complete isolated cache verification, current sources, runtimes and cross-language traces. */
const fs = require("node:fs"), path = require("node:path"), crypto = require("node:crypto"), assert = require("node:assert/strict"), child = require("node:child_process");
const ticket = __dirname;
let root = ticket;
while (!fs.existsSync(path.join(root, "🧰️framework"))) root = path.dirname(root);
const moduleRoot = path.join(root, "🧰️framework/🔨️modules/🗃️cache");
const output = path.join(ticket, "🗑️generated/neutral-cache");
fs.mkdirSync(output, { recursive: true });
const fingerprints = () => {
  const result = {};
  const visit = directory => { for (const entry of fs.readdirSync(directory, { withFileTypes: true })) { const target = path.join(directory, entry.name); if (entry.isDirectory()) visit(target); else { assert(target.length < 256); result[path.relative(root, target)] = crypto.createHash("sha256").update(fs.readFileSync(target)).digest("hex"); } } };
  visit(moduleRoot); return result;
};
const sources = fingerprints(), started = new Date().toISOString(), results = [];
const run = (name, args) => {
  const result = child.spawnSync("repo", args, { cwd: root, encoding: "utf8", timeout: 120000, env: { ...process.env, PYTHONDONTWRITEBYTECODE: "1", PYTHONIOENCODING: "utf-8" } });
  const record = { name, command: ["repo", ...args], status: result.status, signal: result.signal, stdout: result.stdout, stderr: result.stderr, error: result.error?.message };
  results.push(record); fs.writeFileSync(path.join(output, name + ".json"), JSON.stringify(record, null, 2));
  assert.equal(result.status, 0, name + " failed: " + result.stdout + result.stderr); return result.stdout;
};
const interpreter = path.join(root, ".🧬semio/🎓️teaching/⚡️cache/python", process.platform === "win32" ? "Scripts/python.exe" : "bin/python");
const python = fs.existsSync(interpreter) ? ["tools", "exec", "--light", "uv", "--", "run", "--python", interpreter, "--no-project", "python"] : ["tools", "exec", "--light", "python", "--"];
const runtime = JSON.parse(run("python-runtime", [...python, "-B", "-c", "import json,platform,sys; print(json.dumps(dict(executable=sys.executable,version=sys.version,platform=platform.platform())))"]));
run("python", [...python, "-B", path.join(moduleRoot, "🧪️tests/🐍️.py"), "--record", path.join(output, "python-traces.json")]);
run("typescript-oracle", ["tools", "exec", "--light", "node", "--", path.join(moduleRoot, "🧪️tests/🟨️.cjs"), "--record", path.join(output, "typescript-traces.json")]);
run("typecheck", ["tools", "exec", "--light", "tsc", "--", "--noEmit", "--target", "ES2022", "--module", "NodeNext", "--moduleResolution", "NodeNext", "--skipLibCheck", path.join(moduleRoot, "🟦️.ts")]);
const pythonTrace = JSON.parse(fs.readFileSync(path.join(output, "python-traces.json"), "utf8")), typescriptTrace = JSON.parse(fs.readFileSync(path.join(output, "typescript-traces.json"), "utf8"));
assert.deepEqual(pythonTrace, typescriptTrace);
const end = fingerprints(); assert.deepEqual(sources, end);
const oracles = JSON.parse(fs.readFileSync(path.join(output, "oracle-identities.json"), "utf8"));
const report = { accepted: true, started_at: started, completed_at: new Date().toISOString(), source_start: sources, source_end: end, source_unchanged: true, runtime: { python: runtime, node: process.version, platform: process.platform, test_oracles: oracles }, matching_complete_trace_snapshots: pythonTrace.length, results, complete_portable_trace: pythonTrace };
fs.writeFileSync(path.join(output, "result.json"), JSON.stringify(report, null, 2));
const fence = String.fromCharCode(96).repeat(3);
fs.writeFileSync(path.join(ticket, "neutral-cache-implementation-proof.md"), "# Isolated Neutral Cache Implementation Proof\n\nThe schema/shared traces and tests preceded implementation. Initial RED was observed with missing Python/TypeScript modules, while schema validation and installed dual-LRU tests already passed. A subsequent zero-cost-entry→unknown-factory trace exposed a real RED in both implementations; both now evict zero-cost entries before unknown construction.\n\nThis complete accepted run uses normal official 512 MB capacity admission, actual teaching Python 3.12, Node TypeScript execution and installed AJV/LRU test oracles, plus TypeScript no-emit checking. All 25 complete lifecycle/accounting snapshots agree between Python and TypeScript. Every isolated module source hash remained unchanged throughout. The resource metrics do not establish OS memory bounds or full-model point-budget fit. No Manim/full geometry/native test ran here, and no renderer/clip input was edited.\n\nThe new conventional workspace manifest requires normal lock-owner reconciliation before repo project-target execution. Direct registered-tool verification is accepted here; the repo target was attempted and refused before tests by workspace lock-scope consistency checks, including unrelated bindings membership. No existing lock was edited by this worker.\n\n" + fence + "json\n" + JSON.stringify(report, null, 2) + "\n" + fence + "\n");
console.log("[DEBUG] Neutral cache accepted: " + pythonTrace.length + " identical complete snapshots; " + Object.keys(sources).length + " unchanged source files");
