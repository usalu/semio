/** @emoji 🔁 LSP round-trip smoke test via jack language server (native). */
import { spawnSync } from "node:child_process";
import { join } from "node:path";

const root = process.cwd();
const result = spawnSync("cargo", ["test", "-p", "trinity_jack_lsp", "--lib"], {
  cwd: root,
  encoding: "utf8",
});

if (result.status !== 0) {
  console.error(result.stdout);
  console.error(result.stderr);
  throw new Error("trinity_jack_lsp tests failed");
}

console.log("[DEBUG] lsp-roundtrip ok", result.stdout.trim().split("\n").slice(-3).join(" "));
