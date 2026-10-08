/** 🔎️ Lists the launch rows of the slice L-S4a targets with every policy-bearing field (env, extra arguments, cwd). */
import { readFileSync } from "node:fs";

const targets = ["test-rust-divergence-callback-source", "test-rust-divergence-callback-native", "test-rust-divergence-callback-syn", "test-taxonomy-pattern-compiler-reuse", "test-root-script-compiler", "test-rust-writable-path-authority", "test-registry-import-language", "test-kind-only-basename", "verify-taxonomy-implementation-report", "verify-taxonomy-implementation-enforce", "test-artifact-support", "test-tool-configuration-ownership", "test-vitest-configuration-ownership", "test-reference-coverage-selection", "test-cargo-target-discovery-skip", "test-historical-package-owner-identity", "test-readme-reviewed-fixture-inputs", "test-inventory-artifact-shards", "test-nested-cargo-collision-authority", "test-reference-coordinate-progress", "test-plugin-publication-source-ownership", "test-app-verification-source-ownership", "test-json-reference-owner-lookup", "test-readme-current-source-revision", "test-taxonomy-leading-grapheme", "test-frozen-markdown-coordinates", "test-historical-json-source-encoding", "test-testing-readme-coordinates", "test-readme-move-source-authority", "test-cargo-discovery-exclusions", "test-artifact-io-ownership", "lint-artifact-io-ownership", "test-artifact-source-residue", "test-artifact-source-commit", "value-resident-rs:test", "value-resident-rs:check-wasm", "value-resident:test", "ui-host-rs:test-source", "ui-host-rs:test", "ui-host-rs:check", "ui-host-rs:check-wasm"];
for (const file of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) {
  const document = Bun.JSONC.parse(readFileSync(file, "utf8")) as { configurations: unknown[] };
  const rows = document.configurations.filter((row): row is { name: string; command: string; cwd?: string; env?: Record<string, string> } => typeof row === "object" && row !== null && typeof (row as { command?: unknown }).command === "string");
  console.log(`## ${file}: ${rows.length} command rows`);
  for (const target of targets) {
    const matches = rows.filter((row) => new RegExp(`[:\\s]${target.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}(?:\\s|$)`, "u").test(row.command) || row.command.endsWith(`:${target}`) || row.command.includes(`${target} `));
    const policy = matches.map((row) => `${row.command.replace(/^bun (x )?nx run /u, "")}${row.env ? ` env=${JSON.stringify(row.env)}` : ""}${row.cwd && row.cwd !== "${workspaceFolder}" ? ` cwd=${row.cwd}` : ""}`);
    console.log(`${target}: ${matches.length} ${[...new Set(policy)].join(" || ").slice(0, 420)}`);
  }
}
