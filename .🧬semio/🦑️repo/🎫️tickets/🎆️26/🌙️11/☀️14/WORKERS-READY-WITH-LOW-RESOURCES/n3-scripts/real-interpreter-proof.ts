import {spawnSync} from "node:child_process";
import {mkdirSync, mkdtempSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {pythonInterpreterHomeV1, toolchainHostV1} from "/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import {prepareInterpreterV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🟦️.ts";

const series = process.argv[2] ?? "3.12", uv = process.argv[3] ?? "uv", host = toolchainHostV1(), root = mkdtempSync(join(tmpdir(), "n3-real-")), started = Date.now();
const run = (command: string, args: readonly string[], cwd: string) => { const result = spawnSync(command, [...args], {cwd, encoding: "utf8"}); if (result.status !== 0) throw new Error(command + " " + result.stderr); return result.stdout; };
const port = {platform: process.platform, offline: false, environment: {SEMIO_LANGUAGE: "en"}, fetch: (url: string, signal: AbortSignal) => fetch(url, {signal}), run: async (c: string, a: readonly string[], d: string) => { run(c, a, d); }, capture: async (c: string, a: readonly string[], d: string) => run(c, a, d), log: (text: string) => console.log(text)};
try {
  const binary = await prepareInterpreterV1(series, host, root, port, AbortSignal.timeout(300000));
  console.log("binary", binary, run(binary, ["--version"], root).trim(), "ms", Date.now() - started);
  const project = join(root, "project");
  mkdirSync(project);
  writeFileSync(join(project, "pyproject.toml"), `[project]\nname = "x"\nversion = "0"\nrequires-python = "==${series}.*"\ndependencies = []\n`);
  const env = {PATH: "/usr/bin:/bin", HOME: join(root, "home"), UV_NO_CONFIG: "1", UV_PYTHON_PREFERENCE: "only-managed", UV_PYTHON_DOWNLOADS: "never", UV_PYTHON_INSTALL_DIR: join(pythonInterpreterHomeV1(series, host, root), ".."), UV_PYTHON: series, UV_CACHE_DIR: join(root, "cache"), UV_PROJECT_ENVIRONMENT: join(root, "venv")};
  const found = spawnSync(uv, ["python", "find", "--no-python-downloads", series], {env, encoding: "utf8"});
  console.log("uv python find:", found.status, found.stdout.trim(), found.stderr.trim());
  const sync = spawnSync(uv, ["sync", "--project", project], {env, encoding: "utf8"});
  console.log("uv sync:", sync.status, sync.stderr.trim().split("\n").slice(-3).join(" | "));
  console.log("venv python", run(join(root, "venv", "bin", "python"), ["--version"], root).trim());
  if (found.status !== 0 || sync.status !== 0) process.exitCode = 1;
} finally {
  rmSync(root, {recursive: true, force: true});
}
