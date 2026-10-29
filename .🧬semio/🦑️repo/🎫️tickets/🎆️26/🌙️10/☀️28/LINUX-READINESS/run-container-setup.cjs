const fs = require("node:fs"), path = require("node:path"), cp = require("node:child_process");
const config = JSON.parse(fs.readFileSync(".devcontainer/devcontainer.json", "utf8"));
const label = process.argv[2] ?? "container-setup";
if(!/^[a-z0-9-]+$/u.test(label))throw Error("Finite setup log label required");
const log = path.join(__dirname, "🗑️generated", label+".log");
const output = fs.openSync(log, "w");
const env = Object.entries(config.containerEnv).flatMap(([name, value]) => ["--env", name + "=" + value.replaceAll("${containerWorkspaceFolder}", config.workspaceFolder)]);
const run = command => {
  const result = cp.spawnSync("/home/deck/.local/bin/repo", ["tools", "exec", "docker", "--", "exec", ...env, "semio-linux-readiness-source", ...command], { env: { ...process.env, SEMIO_TOOL_DOCKER: "/usr/bin/podman", SEMIO_LANGUAGE: "en" }, stdio: ["ignore", output, output] });
  if (result.error) throw result.error;
  if (result.status !== 0) process.exit(result.status ?? 1);
};
run(["/home/vscode/.local/bin/repo","tools","exec","--light","node","--",".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/LINUX-READINESS/verify-container-source.cjs",".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/LINUX-READINESS/🗑️generated/container-current-source.json"]);
run(config.onCreateCommand);
run(config.postCreateCommand);
fs.closeSync(output);
console.log("[DEBUG] Configured container creation completed");
