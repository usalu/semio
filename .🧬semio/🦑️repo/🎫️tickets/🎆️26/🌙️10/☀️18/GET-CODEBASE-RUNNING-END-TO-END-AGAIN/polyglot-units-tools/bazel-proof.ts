// Runs the generated `repo_unit` rule once with real Bazel on a scratch fixture (one batch-mode build at a time, no server stays).
// usage: bun bazel-proof.ts <module-dir (overlay copy of L/🏗️build/🧩️units)> <bazel executable> <install base> <work dir> [python|go]
// The runner of every action is the standalone entry `⌨️command/📜️script.ts`; SEMIO_UNITS_TRACE collects the ids of the executed units.
import {spawnSync} from "node:child_process";
import {appendFileSync, chmodSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";

const [moduleDir, bazel, installBase, work, which = "python"] = process.argv.slice(2);
if (!moduleDir || !bazel || !installBase || !work) {console.error("usage: bun bazel-proof.ts <module-dir> <bazel> <install-base> <work> [python|go]"); process.exit(2);}
const units = await import(join(moduleDir, "🟦️.ts"));
const workspace = join(work, "ws-" + which), root = join(work, "output-root"), trace = join(work, `trace-${which}.txt`);
rmSync(workspace, {recursive: true, force: true}); rmSync(root, {recursive: true, force: true}); mkdirSync(workspace, {recursive: true});

const python: Record<string, string> = {
  "lab/__init__.py": "",
  "lab/a.py": "def big(x):\n    return x + 1\n\nCONSTANT = 3\n",
  "lab/b.py": "from lab.a import big, CONSTANT\n\ndef use():\n    return big(CONSTANT)\n",
  "lab/c.py": "from lab.b import use\n\nRESULT = use()\n",
};
const go: Record<string, string> = {
  "go.mod": "module example.com/lab\n\ngo 1.25\n",
  "a/a.go": "package a\n\nimport \"strings\"\n\nfunc Big(s string) string {\n\tvar out []string\n\tfor i := 0; i < 10; i++ {\n\t\tout = append(out, strings.ToUpper(s))\n\t}\n\treturn strings.Join(out, \",\")\n}\n",
  "a/a_test.go": "package a\n\nimport \"testing\"\n\nfunc TestBig(t *testing.T) { if Big(\"q\") == \"\" { t.Fatal(\"empty\") } }\n",
  "b/b.go": "package b\n\nimport \"example.com/lab/a\"\n\nfunc Use() string { return a.Big(\"q\") }\n",
  "c/c.go": "package main\n\nimport \"example.com/lab/b\"\n\nfunc main() { println(b.Use()) }\n",
};
const files = which === "python" ? python : go;
const write = (path: string, text: string): void => {mkdirSync(dirname(join(workspace, path)), {recursive: true}); writeFileSync(join(workspace, path), text);};
for (const [path, text] of Object.entries(files)) write(path, text);
write("WORKSPACE", "");
write("runner", `#!/bin/sh\nexec /Users/ueli/.bun/bin/bun ${join(moduleDir, "⌨️command", "📜️script.ts")} "$@"\n`);
chmodSync(join(workspace, "runner"), 0o755);

const ports = {
  write: (line: string) => console.log("  | " + line.split("\n")[0]),
  progress: async () => {},
  check: () => {},
  signal: new AbortController().signal,
  language: (requested: "en" | "de" | undefined) => requested ?? "en",
  run: async (argv: readonly string[], options: {cwd: string; env: Readonly<Record<string, string>>}) => {const r = spawnSync(argv[0]!, argv.slice(1), {cwd: options.cwd, env: {...process.env, ...options.env}, encoding: "utf8"}); return {code: r.status ?? 1, output: (r.stdout ?? "") + (r.stderr ?? "")};},
  files: async () => Object.keys(files),
};
await units.runUnitsCommandV1(workspace, ["units", "emit", "bazel", `--for=${which}`, "--out=.", "--lang=en"], ports);

const build = (label: string): {executed: string[]; code: number; tail: string} => {
  rmSync(trace, {force: true}); writeFileSync(trace, "");
  const started = performance.now();
  const run = spawnSync("nice", ["-n", "10", bazel, "--nohome_rc", "--nosystem_rc", "--noworkspace_rc", `--install_base=${installBase}`, `--output_user_root=${root}`, "--batch", "--host_jvm_args=-Xmx768m", "build", "//...", "--enable_workspace", "--noenable_bzlmod", "--noexperimental_convenience_symlinks" ,
    `--action_env=PATH=/Users/ueli/.local/go/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin`, `--action_env=SEMIO_UNITS_TRACE=${trace}`, "--action_env=HOME=" + (process.env.HOME ?? "/tmp"), `--sandbox_writable_path=${work}`, `--action_env=GOCACHE=${join(work, "gocache")}`, "--color=no", "--curses=no", "--show_progress_rate_limit=0", "--noshow_progress", "--jobs=2"], {cwd: workspace, encoding: "utf8", env: {...process.env, USE_BAZEL_VERSION: ""}, maxBuffer: 64 * 1024 * 1024});
  const executed = readFileSync(trace, "utf8").split("\n").filter(Boolean).sort();
  const output = (run.stdout ?? "") + (run.stderr ?? "");
  const processes = /(\d+) processes?:?\s*([^\n]*)/.exec(output)?.[0] ?? "";
  console.log(`[${label}] exit ${run.status} in ${Math.round(performance.now() - started)} ms; executed ${executed.length}: ${executed.join(", ") || "-"}; ${processes}`);
  return {executed, code: run.status ?? 1, tail: output.split("\n").slice(-25).join("\n")};
};
const results: Record<string, {executed: string[]; code: number}> = {};
const step = (label: string, edit?: () => void): void => {edit?.(); const r = build(label); results[label] = r; if (r.code !== 0) console.log(r.tail);};

if (which === "python") {
  step("cold");
  step("again");
  step("body edit of lab.a", () => write("lab/a.py", files["lab/a.py"]!.replace("x + 1", "x + 2")));
  step("signature edit of lab.a", () => write("lab/a.py", files["lab/a.py"]!.replace("x + 1", "x + 2").replace("def big(x):", "def big(x, y=0):")));
  step("removed name (b must fail)", () => write("lab/a.py", "CONSTANT = 3\n"));
  const expect = (label: string, ids: string[], code = 0): void => {const r = results[label]!; const ok = JSON.stringify(r.executed) === JSON.stringify(ids.sort()) && r.code === code; console.log(`${ok ? "PASS" : "FAIL"} ${label}`); if (!ok) process.exitCode = 1;};
  expect("cold", ["python:lab", "python:lab.a", "python:lab.b", "python:lab.c"]);
  expect("again", []);
  expect("body edit of lab.a", ["python:lab.a"]);
  expect("signature edit of lab.a", ["python:lab.a", "python:lab.b", "python:lab.c"]);
  const failed = results["removed name (b must fail)"]!;
  const okFail = failed.code !== 0 && failed.executed.includes("python:lab.b");
  console.log(`${okFail ? "PASS" : "FAIL"} removed name: build fails at lab.b`); if (!okFail) process.exitCode = 1;
} else {
  step("cold");
  step("again");
  step("body edit of a", () => write("a/a.go", files["a/a.go"]!.replace("ToUpper", "ToLower")));
  const expect = (label: string, ids: string[], code = 0): void => {const r = results[label]!; const ok = JSON.stringify(r.executed) === JSON.stringify(ids.sort()) && r.code === code; console.log(`${ok ? "PASS" : "FAIL"} ${label}`); if (!ok) process.exitCode = 1;};
  expect("cold", ["go:example.com/lab/a", "go:example.com/lab/a#test", "go:example.com/lab/b", "go:example.com/lab/c"]);
  expect("again", []);
  console.log("go body edit executes (sources mode, no build-tool cutoff):", results["body edit of a"]!.executed.join(", "));
}
void existsSync; void appendFileSync;
