import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
let root = import.meta.dir;
while (!existsSync(join(root, "bun.lock")) || !existsSync(join(root, "📋️project.json"))) { const parent = dirname(root); if (parent === root) throw Error("Repository root missing"); root = parent; }
const command = process.argv[2], rest = process.argv.slice(3);
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const kernel = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust";
const repositoryEnv = { ...process.env, NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, REPO_ROOT: root };
if (command === "kernel-corpus") {
  const failed: string[] = [];
  for (const law of ["paged-history-stack-check", "wal-recovery-check", "wal-capacity-check", "wal-committed-transactions-check", "wal-writer-authority-check", "database-history-completion-check", "database-catalog-read-ownership-check", "database-capability-completion-check", "wal-committed-compaction-check", "database-shutdown-check", "document-mount-single-flight-check", "durable-owned-group-decision-check", "durable-group-journal-check", "retained-clone-check", "directory-event-page-bootstrap-check"]) {
    console.log("[DEBUG] kernel behavior oracle " + law);
    const child = Bun.spawn([process.execPath, join(root, kernel, "📜️script.ts"), law], { cwd: root, env: repositoryEnv, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
    const stop = () => child.kill("SIGTERM");
    process.once("SIGINT", stop); process.once("SIGTERM", stop);
    const code = await child.exited;
    process.off("SIGINT", stop); process.off("SIGTERM", stop);
    if (code) failed.push(law);
  }
  console.log("[DEBUG] kernel behavior oracle failures=" + JSON.stringify(failed));
  process.exit(failed.length ? 1 : 0);
}
const args = command === "test-owner-command" ? [join(root, rest[0]!), ...rest.slice(1)] : command === "owner-command" ? [join(root, library, "⚡️caching/🦀️cargo/📜️script.ts"), "native", "owner-command", "--manifest", join(dirname(rest[0]!), "Cargo.toml"), "--cwd", dirname(rest[0]!), "--", process.execPath, join(root, rest[0]!), ...rest.slice(1)] : command === "structural-catalog-test" ? ["test", join(root, library, "🧪️tests/🔬️workspace-contract/🟦️.ts"), "-t", "schema scope catalog"] : command === "kernel-test" ? [join(root, kernel, "📜️script.ts"), "test", "os_store::tests::", "--", "--nocapture"] : command === "kernel-command" ? [join(root, kernel, "📜️script.ts"), ...rest] : command === "schema" ? [join(root, "📜️script.ts"), "schema", ...rest] : command === "verify" ? [join(root, "📜️script.ts"), "verify", ...rest] : command === "tests" ? ["test", ...rest.map(path => join(root, path))] : command === "parity" ? [join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts"), "parity", "quick", "--owner", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test", "--case", "🖥️host-protocol-parity"] : null;
if (!args) throw Error("Unknown verification command");
console.log("[DEBUG] isolated Nx verification " + command);
const child = Bun.spawn([process.execPath, ...args], { cwd: root, env: repositoryEnv, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
const interrupt = () => child.kill("SIGINT"), terminate = () => child.kill("SIGTERM");
process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
process.exitCode = await child.exited;
