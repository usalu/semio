/** 🔎 Scans tracked project.json targets and classifies their cache correctness (read-only). */
import { readFileSync, existsSync } from "node:fs";
import { dirname, posix } from "node:path";

const WS = "/Users/ueli/Documents/semio";
const C = process.argv[2];
const list = readFileSync(`${C}/cache-audit-tools/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const policy = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`, "utf8"));
const matchesCommand = (name: string, cmds: string[]) => cmds.some((c) => name === c || name.startsWith(`${c}-`));
const cacheableFamily = (name: string) =>
  /^(test(?:-|$)|check(?:-|$)|lint(?:-|$)|typecheck(?:-|$)|format-check(?:-|$)|generate(?:-|$)|schema(?:-|$)|verify(?:-|$)|stdio(?:-|$)|build(?:-|$)|wasm$|native-build$|package$|extension-package$|cpp-(?:configure|build|test)$|graph-check$|policy-check$|artifact-check$|artifact-package-contract$|toolchain$)/.test(name) ||
  /(?:^|-)contract(?:-|$)/.test(name) || (/^generator(?:-|$)/.test(name) && !name.includes("generator-inputs"));
const mutatingName = (n: string) => /(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)/.test(n);
const liveName = (n: string) => /(?:^|-)(?:e2e|live)(?:-|$)/.test(n);
const verifyCommand = (cmd: string) => /(?:^|[\s"'])verify(?:[\s"']|$)/.test(cmd);
const PRODUCES = /(?:^|-)(build|compile|package|bundle|wasm|generate|schema|toolchain|export|emit|pack|transpile|codegen|fonts|render|native-build|extension-package|artifact-package-contract|pdf|docs?|assets?|prepare|bindings?|dist)(?:-|$)/;
const NETWORK = /\b(curl|wget|git clone|git fetch|npm install|bun install|bun add|bunx|npx|cargo fetch|cargo install|pip install|uv sync|uv pip|apt-get|brew install|go get|go mod download)(?![-\w])/;
const NONDET = /\b(date|mktemp|uuidgen|Date\.now|new Date|Math\.random|crypto\.randomUUID|\$RANDOM|hostname|whoami)\b/;
const MUTATING_CMD = /(--fix(?![\w-])|--write|\s-w\b|\bclean\b|\binstall\b|\bpublish\b|\bdeploy\b|\bgit\b|\bformat\b(?! ?-?check)|\bseed\b|\bmigrate\b|\bupdate\b|\bsetup\b)/;
const ABS = /(^|[\s"'=:])(\/Users\/|\/home\/|\/tmp\/|\/private\/|[A-Za-z]:\\)/;
const toolEnv = new Set<string>(["NODE_ENV", "BUN_ENV", "CI", "SEMIO_BUILD_MODE", "SEMIO_RENDERER", "SEMIO_PLUGIN", "SEMIO_TEST_LEVEL", "SEMIO_TEST_BUDGET_MS", "SEMIO_BUILD_BUDGET_MS", "RUSTUP_TOOLCHAIN", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_TARGET", "SDKROOT", "MACOSX_DEPLOYMENT_TARGET", "SEMIO_WASM_OPT", "SEMIO_DISABLE_WASM_OPT", "SEMIO_PLUGIN_PROFILE", "CC", "CXX", "GOOS", "GOARCH", "CGO_ENABLED", "GOFLAGS", "UV_PYTHON", "PYTHONHASHSEED"]);
const sharedEnv = new Set<string>(["SEMIO_BUILD_MODE"]);

type Finding = { file: string; target: string; rule: string; detail: string; cache: string; severity: "high" | "medium" | "low" };
const findings: Finding[] = [];
const stats: Record<string, number> = {};
const bump = (k: string, n = 1) => (stats[k] = (stats[k] ?? 0) + n);
const parseFails: string[] = [];
const noTargets: string[] = [];
const defaultOn: Record<string, number> = {};
const effective: Record<string, number> = {};

function tokenResolve(p: string, root: string): { abs: string | null; dynamic: boolean } {
  if (p.includes("{options.")) return { abs: null, dynamic: true };
  if (p.startsWith("!")) p = p.slice(1);
  if (p.startsWith("{projectRoot}")) return { abs: posix.normalize(posix.join(root || ".", p.slice("{projectRoot}".length).replace(/^\//, ""))), dynamic: false };
  if (p.startsWith("{workspaceRoot}")) return { abs: posix.normalize(p.slice("{workspaceRoot}".length).replace(/^\//, "") || "."), dynamic: false };
  if (p.startsWith("/")) return { abs: p, dynamic: false };
  if (p.startsWith("{")) return { abs: null, dynamic: true };
  return { abs: posix.normalize(p), dynamic: false };
}
const inside = (abs: string, root: string) => {
  const r = root === "" || root === "." ? "" : root.replace(/\/$/, "");
  return r === "" ? !abs.startsWith("..") && !abs.startsWith("/") : abs === r || abs.startsWith(r + "/");
};

for (const file of list) {
  const root = dirname(file) === "." ? "" : dirname(file);
  let json: any;
  try { json = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { parseFails.push(file); continue; }
  const targets = json.targets ?? {};
  const names = Object.keys(targets);
  if (names.length === 0) { noTargets.push(file); continue; }
  bump("projects");
  const isAppRoot = json.projectType === "application" && root === "";
  const scriptPath = `${WS}/${root ? root + "/" : ""}📜️script.ts`;
  const ownsScript = existsSync(scriptPath);
  for (const name of names) {
    const t = targets[name] ?? {};
    bump("targets");
    const cmd = String(t.command ?? t.options?.command ?? (Array.isArray(t.options?.commands) ? t.options.commands.join(" && ") : "") ?? "");
    const inputs: any[] = Array.isArray(t.inputs) ? t.inputs : [];
    const outputs: string[] = Array.isArray(t.outputs) ? t.outputs : [];
    const hasExplicitInputs = Array.isArray(t.inputs);
    const envIn = new Set(inputs.filter((i) => i && typeof i === "object" && typeof i.env === "string").map((i) => i.env));
    const optEnv = Object.keys(t.options?.env ?? {});
    const exec = t.metadata?.semio?.execution?.kind ?? (t.executor ?? "default");
    // policy classification (mirrors target-policy.targetPolicy)
    let eff: string; let why: string;
    if (t.cache === false) { eff = "off-explicit"; why = "explicit"; }
    else if (matchesCommand(name, policy.continuous)) { eff = "off-continuous"; why = "continuous"; }
    else if (policy.uncachedExact.includes(name) || matchesCommand(name, policy.uncached)) { eff = "off-policy"; why = "uncached"; }
    else if (policy.cachedExact.includes(name)) { eff = "on-cachedExact"; why = "cachedExact"; }
    else if (mutatingName(name) || liveName(name)) { eff = "off-mutating"; why = "mutating/live"; }
    else if (cacheableFamily(name) || verifyCommand(cmd)) { eff = "on-family"; why = "family"; }
    else { eff = "on-default"; why = "fallthrough"; }
    const cacheOn = eff.startsWith("on");
    effective[eff] = (effective[eff] ?? 0) + 1;
    if (eff === "on-default") defaultOn[file] = (defaultOn[file] ?? 0) + 1;
    const rec = (rule: string, detail: string, severity: Finding["severity"]) => findings.push({ file, target: name, rule, detail: detail.slice(0, 300), cache: eff, severity });

    if (!cacheOn) {
      // candidates for caching among explicitly uncached targets
      if (t.cache === false && cacheableFamily(name) && !MUTATING_CMD.test(cmd) && outputs.length > 0 && !t.continuous) rec("uncached-but-cacheable-candidate", `cache:false on ${name} with declared outputs and non-mutating command`, "low");
      bump("uncached");
      continue;
    }
    bump("cacheable");
    if (outputs.length === 0 && PRODUCES.test(name) && !/^(test|check|lint|typecheck|format-check|verify)/.test(name)) rec("cacheable-produces-no-outputs", `name suggests artifact (${name}) but outputs empty; replay restores nothing; cmd=${cmd.slice(0, 160)}`, "high");
    if (outputs.length === 0 && cmd && PRODUCES.test(cmd.split(/\s+/).slice(0, 4).join(" ")) && /(build|compile|package|bundle|--out|--outdir|-o\s)/.test(cmd)) rec("cacheable-command-writes-without-outputs", `cmd produces files but outputs empty: ${cmd.slice(0, 160)}`, "high");
    if (!hasExplicitInputs) { bump("cacheable-implicit-default-inputs"); if (eff === "on-default") rec("fallthrough-cached-without-inputs", `target not classified; cached by fallthrough with implicit default inputs; cmd=${cmd.slice(0, 160)}`, "medium"); }
    if (hasExplicitInputs && inputs.length === 0) rec("empty-inputs-array", "inputs: [] on cacheable target", "high");
    for (const o of outputs) {
      const r = tokenResolve(o, root);
      if (r.dynamic) { if (o.includes("{options.env.")) { const m = o.match(/\{options\.env\.([A-Z0-9_]+)\}/); if (m && !envIn.has(m[1]) && !sharedEnv.has(m[1])) rec("output-depends-on-env-not-in-inputs", `output ${o} uses ${m[1]} not in inputs {env}`, "high"); } bump("outputs-dynamic"); continue; }
      if (r.abs === null) continue;
      if (r.abs.startsWith("/")) rec("output-absolute", o, "high");
      else if (!inside(r.abs, root)) rec("output-outside-project", `${o} -> ${r.abs} (project ${root || "."})`, "high");
      else if (!o.startsWith("{projectRoot}") && !o.startsWith("{workspaceRoot}") && root) rec("output-relative-to-workspace", `${o} is workspace-relative (Nx), project ${root}`, "medium");
    }
    for (const i of inputs) {
      const s = typeof i === "string" ? i : null;
      if (s && ABS.test(s)) rec("input-absolute-path", s, "high");
      if (s && s.startsWith("/")) rec("input-absolute-path", s, "high");
      if (s && s.startsWith("..") ) rec("input-outside-workspace", s, "medium");
      if (s && s.includes("{workspaceRoot}/..")) rec("input-outside-workspace", s, "medium");
    }
    if (ABS.test(cmd)) rec("command-absolute-path", cmd.slice(0, 200), "high");
    if (NETWORK.test(cmd)) rec("cacheable-network-command", cmd.slice(0, 200), "high");
    if (NONDET.test(cmd)) rec("cacheable-nondeterministic-command", cmd.slice(0, 200), "medium");
    if (MUTATING_CMD.test(cmd) && !/--check|--dry-run|verify/.test(cmd)) rec("cacheable-mutating-command", cmd.slice(0, 200), "high");
    for (const e of optEnv) if (!envIn.has(e) && !sharedEnv.has(e)) rec("option-env-not-in-inputs", `options.env.${e} not in inputs {env}`, toolEnv.has(e) ? "high" : "medium");
    const dash = [] as string[];
    // toolchain env vars that a tool command implies but not declared
    if (/\b(bun|cargo|node|rustc|go|uv|python|cmake)\b/.test(cmd) && hasExplicitInputs) {
      const missing = [...toolEnv].filter((e) => !envIn.has(e) && !sharedEnv.has(e) && ["SEMIO_TEST_LEVEL", "SEMIO_RENDERER", "SEMIO_PLUGIN", "SEMIO_PLUGIN_PROFILE", "SEMIO_WASM_OPT", "SEMIO_DISABLE_WASM_OPT", "RUSTFLAGS", "RUSTUP_TOOLCHAIN", "CI", "NODE_ENV", "BUN_ENV"].includes(e));
      if (missing.length) rec("explicit-inputs-omit-toolchain-env", `explicit inputs omit env ${missing.join(",")} used by tool command`, "medium");
    }
    if (ownsScript && hasExplicitInputs && !inputs.some((i) => typeof i === "string" && /default|\{projectRoot\}/.test(i))) rec("explicit-inputs-exclude-project-sources", `explicit inputs without default/projectRoot: ${JSON.stringify(inputs).slice(0, 160)}`, "medium");
  }
}

const counts: Record<string, number> = {};
for (const f of findings) counts[f.rule] = (counts[f.rule] ?? 0) + 1;
const out = { stats, effective, parseFails, noTargets: noTargets.length, projectsWithFallthroughCacheOn: Object.keys(defaultOn).length, findingCounts: counts, findings };
const path = `${C}/🗑️generated/cache-audit-scan.json`;
await Bun.write(path, JSON.stringify(out, null, 1));
console.log(JSON.stringify({ stats, effective, parseFails: parseFails.length, noTargets: noTargets.length, findingCounts: counts }, null, 1));
