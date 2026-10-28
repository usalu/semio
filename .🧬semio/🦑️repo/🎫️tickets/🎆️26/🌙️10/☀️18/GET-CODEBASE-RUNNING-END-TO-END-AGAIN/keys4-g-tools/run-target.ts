// run-target: bun run-target.ts <tag> <target|ALL> <bun|cargo|all> [--cargo-log a.jsonl --cargo-log b.jsonl] [--only <leg-id-substring>] : runs every leg of a narrowed target under the read oracle (overlay CLI of the staged module: Seatbelt kernel enforcement, profile split for large keys, mirror only when a plan needs more than --max-profiles) with exactly the target's declared set, and writes <evidence>/<tag>-<target>.json (one row per leg, undeclared reads learned, backend, strength). Legs are copied by hand from the router classes of the two script.ts files; a cargo leg runs every test executable of its crates (all tests, a superset of the nextest filter the script passes), cwd = package directory as cargo does.
import {createReadStream, existsSync, mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {createInterface} from "node:readline";
import {spawnSync} from "node:child_process";
import {dirname, join, relative} from "node:path";

const W = "/Users/ueli/Documents/semio", C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const R_LIVE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const K = `${W}/${C}/keys4-g-tools`, G = process.env.KEYS4_G ?? `${W}/${C}/🗑️generated/keys4-g`, E = `${K}/evidence`;
const CLI = process.env.KEYS4_CLI ?? `${W}/${R_LIVE}/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts`;
const R = "🧰️framework/🛍️products/🦑️repo", F = "🧰️framework", M = `${F}/🔨️modules`, D = `${R}/🔨️modules/🎛️dashboard`;
const BUN = (await Bun.$`bun -e ${"console.log(process.execPath)"}`.text()).trim();
const TSC = "node_modules/typescript/bin/tsc", TSC_FLAGS = ["--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun"];

type Leg = {id: string; command?: string[]; cwd?: string; env?: Record<string, string>; crates?: string[]; features?: string; log?: "framework" | "windows" | "unit"};
const dotted = (file: string) => (file.startsWith("/") || file.startsWith("./") ? file : `./${file}`);
const bun = (id: string, ...files: string[]): Leg => ({id, command: [BUN, "test", ...files.map(dotted)]});
const tsc = (id: string, ...files: string[]): Leg => ({id, command: [BUN, TSC, ...TSC_FLAGS, ...files]});
const cargo = (id: string, ...crates: string[]): Leg => ({id, crates, log: "framework"});
const TOOL = `${M}/⏯️tool-run`;
const TARGETS: Record<string, Leg[]> = {
  "test-staged-payload-funding": [bun("bun-job", `${M}/🧵️job/🧪️tests/⛽️staged-projection/🟦️.ts`), cargo("cargo-job", "semio-framework-job"), bun("bun-tool-run", `${TOOL}/🎞️tick/🛫️encode/🧪️tests/🟦️.ts`, `${TOOL}/🎞️tick/⏭️successor/🧪️tests/🟦️.ts`, `${TOOL}/📨️payload/🌊️stream/🧪️tests/🟦️.ts`, `${TOOL}/📨️payload/🌊️stream/📡️wire/🧪️tests/🟦️.ts`, `${TOOL}/🎞️tick/🛬️receive/🧪️tests/🟦️.ts`), cargo("cargo-tool-run", "semio-framework-tool-run")],
  "test-retained-pack-encoding": [bun("bun-pack", `${M}/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🟦️.ts`), cargo("cargo-pack", "semio-framework-pack")],
  "test-window-utility-native": [cargo("cargo-framework", "semio-framework")],
  "test-source-projection": [bun("bun-source-projection", `${M}/📁️filesystem/📷️snapshot/🧪️tests/🟦️.ts`)],
  "test-directory-discovery": [bun("bun-directory-discovery", `${M}/📁️filesystem/🔎️discovery/🧪️tests/🟦️.ts`)],
  "test-action-choices": [cargo("cargo-app-surface", "semio-framework-manifest-app-surface")],
  "test-tool-run-actions": [bun("bun-tool-run-actions", `${TOOL}/🕹️actions/🧪️tests/🟦️.ts`), cargo("cargo-tool-run-actions", "semio-framework-tool-run-actions")],
  "test-history-edit-actions": [bun("bun-time-travel-actions", `${M}/⏪️time-travel/🕹️actions/🧪️tests/🟦️.ts`), cargo("cargo-time-travel-actions", "semio-framework-time-travel-actions")],
  "test-mutation-inputs": [bun("bun-mutation-inputs", `${M}/🛂️manifest/🧪️tests/🧪️mutation-inputs/🟦️.ts`), {id: "python-mutation-inputs", command: ["/usr/bin/python3", `${M}/🛂️manifest/🧪️tests/🧪️mutation-inputs/🐍️.py`]}, cargo("cargo-mutation-inputs", "semio-framework-manifest-mutation-inputs")],
  "test-host-effect-invocation": [bun("bun-host-effect", `${M}/🛂️manifest/🧪️tests/🔁️host-effect-invocation/🟦️.ts`)],
  "test-snapshot-sqlite": [bun("bun-sqlite-unit", `${M}/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🟦️.ts`), cargo("cargo-sqlite", "semio-framework-io-sqlite-snapshot"), bun("bun-sqlite-interop", `${M}/🚪️io/🪶️sqlite-snapshot/🧪️tests/🤝️interoperability/🟦️.ts`)],
  "test-snapshot-sqlite-native": [cargo("cargo-sqlite", "semio-framework-io-sqlite-snapshot")],
  "test-core-modules": [cargo("cargo-core", "semio-framework-hash", "semio-framework-pixels", "semio-framework-intrinsic-size", "semio-framework-mesh-engine")],
  "test-deflate-encoding": [cargo("cargo-deflate", "semio-framework-deflate")],
  "test-wire-retirement-source": [{id: "bun-wire-retirement", command: [BUN, "-e", `const {testWireRetirementFixture}=await import("${W}/${M}/🎯️action-bus/🧹️wire-retirement/🧪️tests/🔬️wire-retirement/🟦️.ts");testWireRetirementFixture();`]}],
  "test-wire-retirement-native": [cargo("cargo-action-bus", "semio-framework-action-bus")],
  "test-package-descriptor-value-codec": [cargo("cargo-manifest-plugin", "semio-framework-manifest-plugin")],
  "test-artifact-kind-source": [bun("bun-artifact-kind", `${M}/🚪️io/🧬️schema/🧪️tests/🪪️artifact-kind/🟦️.ts`)],
  "test-command-ingress-consumer": [tsc("tsc-command-ingress", `${M}/🎠️kernel/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts`), bun("bun-command-ingress", `${M}/🎠️kernel/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts`)],
  "test-history-progress": [cargo("cargo-kernel-history", "semio-framework-kernel-history")],
  "test-snapshot-receiving-source": [tsc("tsc-receiving", `${M}/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🟦️.ts`, `${M}/🚪️io/⏱️control/🧪️tests/nested-io/🟦️.ts`), bun("bun-receiving", `${M}/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🟦️.ts`, `${M}/🚪️io/⏱️control/🧪️tests/nested-io/🟦️.ts`)],
  "test-argument-retirement-native": [cargo("cargo-mutation-inputs", "semio-framework-manifest-mutation-inputs")],
  "test-original-factory-admission": [bun("bun-admission", `${M}/🎯️action-bus/🚪️admission/🧪️tests/🟦️.ts`), cargo("cargo-framework-action-bus", "semio-framework")],
  "test-original-factory-admission-source": [bun("bun-admission", `${M}/🎯️action-bus/🚪️admission/🧪️tests/🟦️.ts`)],
  "dash:test-windows": [bun("bun-windows", `${D}/🧪️tests/🪟️windows/🟦️.ts`, `${D}/🧪️tests/⌨️controls/🟦️.ts`, `${M}/🖱️ui/⌨️tui/🧪️tests/🖱️pointer-routing/🟦️.ts`), {id: "cargo-dash-windows-a", crates: ["semio-framework-repo-dashboard-terminal-laws", "semio-framework-repo-dashboard-source-view"], log: "windows"}, {id: "cargo-tui", crates: ["semio-framework-ui-tui-core", "semio-framework-ui-tui-vt", "semio-framework-ui-tui-terminal", "semio-framework-ui-tui-widgets", "semio-framework-ui-tui-frame", "semio-framework-ui-tui-engine", "semio-framework-ui-tui-host", "semio-framework-ui-tui-unit-laws", "semio-framework-ui-tui-vt-laws", "semio-framework-ui-tui-laws"], log: "windows"}],
  "dash:test-unit": [{id: "cargo-dash-unit", crates: ["semio-framework-repo-dashboard-core", "semio-framework-repo-dashboard-launch", "semio-framework-repo-dashboard-repo-domain", "semio-framework-repo-dashboard-registry", "semio-framework-repo-dashboard-inventory", "semio-framework-repo-dashboard-wire", "semio-framework-repo-dashboard-preferences", "semio-framework-repo-dashboard-labels", "semio-framework-repo-dashboard-feedback", "semio-framework-repo-dashboard-keymap-editor", "semio-framework-repo-dashboard-source-view", "semio-framework-repo-dashboard-launcher", "semio-framework-repo-dashboard-language-choice", "semio-framework-repo-dashboard-daemon", "semio-framework-repo-dashboard-terminal", "semio-framework-repo-dashboard-terminal-laws", "semio-framework-repo-dashboard-cli", "semio-framework-repo-dashboard"], log: "unit"}],
};

const args = process.argv.slice(2), tag = args[0]!, which = args[1]!, mode = args[2] ?? "all";
const flag = (name: string) => args.flatMap((arg, at) => (arg === `--${name}` ? [args[at + 1]!] : []));
const only = flag("only")[0], logs = flag("cargo-log");
const allowFile = (name: string) => `${G}/allow/${name.replace(/^dash:/u, "")}.allow`;
const names = which === "ALL" ? Object.keys(TARGETS) : which.split(",");
mkdirSync(E, {recursive: true}); mkdirSync(`${G}/reports`, {recursive: true});

const executables = new Map<string, {executable: string; manifest: string; kinds: string[]; test: string}[]>();
for (const log of logs) {
  const lines = createInterface({input: createReadStream(log), crlfDelay: Infinity});
  for await (const line of lines) {
    let row: any;
    try { row = JSON.parse(line); } catch { continue; }
    if (row?.reason !== "compiler-artifact" || !row.executable || !row.profile?.test) continue;
    const id: string = row.package_id, after = id.includes("#") ? id.slice(id.indexOf("#") + 1) : id;
    const name = after.includes("@") ? after.slice(0, after.indexOf("@")) : id.slice(0, id.indexOf("#")).split("/").at(-1)!;
    const list = executables.get(name) ?? [];
    list.push({executable: row.executable, manifest: row.manifest_path, kinds: row.target.kind, test: row.target.name});
    executables.set(name, list);
  }
}

const ambient = [`${C}/keys4-g-tools`, `${C}/keys2-g-tools`, `${R}/🔨️modules/🚀️launcher`, `${R}/tsconfig.json`, `${C}/🗑️generated/keys4-g`, `${C}/🗑️generated/keys4r`];
function oracle(legTag: string, allow: string, command: string[], cwd: string, env: Record<string, string>) {
  const out = `${G}/reports/${legTag}.json`;
  const flags = ["--language", "en", "--root", W, "--cwd", cwd, "--allow-list", allow, "--allow-name", "package.json", ...ambient.flatMap((tree) => ["--allow-tree", tree]), "--rounds", process.env.KEYS4_ROUNDS ?? "8", "--timeout", "1200", "--backend", "auto", "--compare", "exit", "--max-profiles", "12", "--out", out, ...Object.entries(env).flatMap(([key, value]) => ["--env", `${key}=${value}`])];
  const run = spawnSync("nice", ["-n", "10", BUN, CLI, ...flags, "--", ...command], {cwd: W, encoding: "utf8", maxBuffer: 64 * 1024 * 1024});
  writeFileSync(`${G}/reports/${legTag}.log`, (run.stdout ?? "") + (run.stderr ?? ""));
  if (!existsSync(out)) return {error: (run.stderr ?? "").slice(-600)};
  const report = JSON.parse(readFileSync(out, "utf8"));
  return {backend: report.backend, strength: report.strength, passed: report.passed, clean: report.clean, learned: report.learned as string[], rounds: report.rounds.length, seconds: Math.round(report.rounds.reduce((sum: number, round: any) => sum + round.seconds, 0)), tail: String(report.rounds.at(-1)?.tail ?? "").split("\n").slice(-4).join(" | ").slice(0, 300), notes: report.rounds.flatMap((round: any) => round.notes).slice(0, 4)};
}

for (const target of names) {
  const legs = TARGETS[target];
  if (!legs) { console.log(JSON.stringify({target, error: "unknown target"})); continue; }
  const rows: any[] = [];
  for (const leg of legs) {
    if (only && !leg.id.includes(only)) continue;
    if (mode === "bun" && leg.crates) continue;
    if (mode === "cargo" && !leg.crates) continue;
    const allow = allowFile(target.replace(/^dash:/u, ""));
    if (leg.command) {
      const rest = oracle(`${tag}-${target.replace(":", "_")}-${leg.id}`, allow, leg.command, leg.cwd ?? "", leg.env ?? {});
      rows.push({leg: leg.id, kind: leg.command.includes("test") ? "bun" : "tool", ...rest});
    } else {
      for (const crate of leg.crates!) {
        const found = (executables.get(crate) ?? []).filter((entry) => (leg.log === "framework" ? entry.kinds.includes("lib") : true));
        if (!found.length) { rows.push({leg: leg.id, crate, error: "no test executable in the cargo log"}); continue; }
        for (const entry of found) {
          const directory = dirname(entry.manifest);
          const rest = oracle(`${tag}-${target.replace(":", "_")}-${crate}-${entry.test}`, allow, [entry.executable, "--test-threads", "1"], relative(W, directory), {CARGO_MANIFEST_DIR: directory});
          rows.push({leg: leg.id, crate, test: `${entry.kinds.join("+")}:${entry.test}`, kind: "cargo", ...rest});
          console.log(JSON.stringify({target, crate, test: entry.test, clean: (rest as any).clean, passed: (rest as any).passed, learned: (rest as any).learned?.length, backend: (rest as any).backend, seconds: (rest as any).seconds, error: (rest as any).error}));
        }
      }
    }
    const last = rows.at(-1);
    console.log(JSON.stringify({target, leg: leg.id, clean: last.clean, passed: last.passed, learned: last.learned?.length, backend: last.backend, strength: last.strength, seconds: last.seconds, error: last.error}));
  }
  writeFileSync(`${E}/${tag}-${target.replace(":", "_")}.json`, JSON.stringify({target, tag, declared: readFileSync(allowFile(target), "utf8").split("\n").filter(Boolean).length, rows}, null, 1));
}
