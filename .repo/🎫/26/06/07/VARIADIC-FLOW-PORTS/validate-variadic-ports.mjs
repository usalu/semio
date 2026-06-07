/** @emoji 🧪 Validates variadic flow ports via targeted Rust tests. */
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../../../..");
const cargo = spawnSync(
  "cargo",
  [
    "test",
    "-p",
    "neural_engine",
    "-p",
    "flow_core",
    "-p",
    "flow_module_dictionary",
    "-p",
    "mathematical_graph_port_directed_dag",
    "variadic",
    "--",
    "--nocapture",
  ],
  { cwd: repoRoot, encoding: "utf8" },
);

process.stdout.write(cargo.stdout ?? "");
process.stderr.write(cargo.stderr ?? "");
if (cargo.status !== 0) {
  console.error("[validate-variadic-ports] cargo test failed");
  process.exit(cargo.status ?? 1);
}
console.log("[validate-variadic-ports] ok");
