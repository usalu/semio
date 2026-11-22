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
const nativeScope = ["-p", "semio-renderer-wgpu-shell-plugins", "--lib", "--", "native_scope_delivers_only_admitted_workpiece_contributions"];
const checks = {
  dev: [workspace, "run", "@semio-tech/framework-os-dev:dev-process3d-react-prototype"],
  wgpu: [workspace, "run", "@semio-tech/framework-os-dev:dev-process3d-wgpu-prototype"],
  "native-smoke": [workspace, "run", "@semio-tech/framework-os-dev:smoke-process3d-native-prototype"],
  "native-scope": [workspace, ...cargo, ...nativeScope],
  "native-timeline-marker": [workspace, "cargo", "--store=isolated", ...cargo.slice(2), "-p", "semio-renderer-wgpu-shell-tests-input-2", "--lib", "--", "retained_tree_reorder_tests::retained_tree_reorder_marker_follows_the_physical_pointer_and_clears", "--exact"],
  "native-scope-isolated": [workspace, "cargo", "--store=isolated", ...cargo.slice(2), ...nativeScope],
  editor: [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--no-fail-fast", "--lib",
    "--test", "process3d_bearbeiten_pane_smoke", "--test", "process3d_workpiece_window",
    "--test", "process3d_panels_inspection", "--test", "process3d_panels_workshop",
    "--test", "process3d_panels_artifact", "--test", "process3d_panels_catalogue",
    "--test", "process3d_interaction_behavior", "--test", "original_host_configuration"],
  "editor-types": [artifact, ...cargo.slice(0, -1), "check", "-p", "semio-s-artifact-process-process3d-host", "-p", "semio-s-artifact-process-process3d-config", "--lib"],
  "editor-envelope": [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--lib", "--", "unit_tests::production_envelope::"],
  "editor-pane": [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--test", "process3d_bearbeiten_pane_smoke"],
  "editor-publication": [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--test", "process3d_bearbeiten_pane_smoke", "--", "host_delivered_set_contributions_dispatches_through_the_command_registry"],
  "editor-composition": [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--test", "process3d_bearbeiten_pane_smoke", "--", "composition::"],
  "editor-forest": [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--test", "process3d_bearbeiten_pane_smoke", "--", "pane_smoke::bearbeiten_pane_", "--nocapture", "--test-threads=1"],
  "editor-workshop": [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--test", "process3d_panels_workshop"],
  "editor-workpiece": [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--test", "process3d_workpiece_window"],
  "editor-timeline": [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--lib", "--", "timeline_drop"],
  "editor-history": [artifact, ...cargo, "-p", "semio-s-artifact-process-process3d-editor", "--test", "process3d_panels_artifact", "--", "context::"],
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
