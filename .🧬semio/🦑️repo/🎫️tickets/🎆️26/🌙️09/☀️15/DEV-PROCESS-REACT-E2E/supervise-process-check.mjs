import { spawn } from "node:child_process";
import { closeSync, mkdirSync, openSync, statSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const script = fileURLToPath(import.meta.url);
const ticket = dirname(script);
const output = resolve(ticket, "🗑️generated");
const workspace = resolve(ticket, "../../../../../../..");
const artifact = resolve(workspace, "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d");
const cargo = ["cargo", "--store=shared", "--configuration=prototype", "--parallelism=1", "--", "test"];
const checks = {
  dev: [workspace, "run", "@semio-tech/framework-os-dev:dev-process3d-react-prototype"],
  wgpu: [workspace, "run", "@semio-tech/framework-os-dev:dev-process3d-wgpu-prototype"],
  "native-smoke": [workspace, "run", "@semio-tech/framework-os-dev:smoke-process3d-native-prototype"],
  "native-scope": [workspace, ...cargo, "-p", "semio-renderer-wgpu-shell-plugins", "--lib", "--", "native_scope_delivers_only_admitted_workpiece_contributions"],
  editor: [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--no-fail-fast", "--lib",
    "--test", "process3d_bearbeiten_pane_smoke", "--test", "process3d_workpiece_window",
    "--test", "process3d_panels_inspection", "--test", "process3d_panels_workshop",
    "--test", "process3d_panels_artifact", "--test", "process3d_panels_catalogue",
    "--test", "process3d_interaction_behavior", "--test", "original_host_configuration"],
  replay: [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-inferences", "--lib"],
  browser: [workspace, "tools", "exec", "--light", "bun", "--", resolve(ticket, "wait-and-verify-browser.ts")],
  "wgpu-browser": [workspace, "tools", "exec", "--light", "bun", "--", resolve(ticket, "wait-and-verify-browser.ts"), "http://127.0.0.1:6122/", "wgpu"],
};
const [mode, check] = process.argv.slice(2);
if (!Object.hasOwn(checks, check) || !["start", "run"].includes(mode)) throw Error("Choose start or run and a declared Process3d check");
mkdirSync(output, { recursive: true });
const log = resolve(output, `supervised-${check}.log`);
const state = resolve(output, `supervised-${check}.jsonl`);
const record = value => writeFileSync(state, JSON.stringify({ at: new Date().toISOString(), ...value }) + "\n", { flag: "a" });

if (mode === "start") {
  const fd = openSync(log, "a");
  try {
    const supervisor = spawn(process.execPath, [script, "run", check], {
      cwd: workspace,
      detached: true,
      stdio: ["ignore", fd, fd],
    });
    await new Promise((accept, reject) => {
      supervisor.once("spawn", accept);
      supervisor.once("error", reject);
    });
    supervisor.unref();
    console.log("[DEBUG] Process check supervisor", { check, pid: supervisor.pid, log, state });
  } finally {
    closeSync(fd);
  }
} else {
  const [cwd, ...args] = checks[check];
  const logOffset = statSync(log, { throwIfNoEntry: false })?.size ?? 0;
  const child = spawn("repo", args, { cwd, stdio: ["ignore", "inherit", "inherit"] });
  child.once("spawn", () => record({ check, supervisor: process.pid, pid: child.pid, logOffset, args }));
  child.once("error", error => {
    record({ check, error: error.message });
    process.exitCode = 1;
  });
  child.once("exit", (code, signal) => {
    record({ check, code, signal });
    process.exitCode = code ?? 1;
  });
}
