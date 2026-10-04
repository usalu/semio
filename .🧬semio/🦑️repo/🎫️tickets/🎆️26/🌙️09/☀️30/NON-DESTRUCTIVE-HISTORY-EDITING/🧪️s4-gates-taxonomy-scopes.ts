/**
 * 🗺️ S4-GATES: `verify taxonomy report` over many scopes in one process (the CLI takes one `--scope`), resumable: each scope's
 * violations are appended to `<out.jsonl>` with an `ours` flag (the path lies in a directory this ticket created,
 * `🧪️s4-gates-new-dirs.py`); scopes already recorded are skipped, and no new scope starts after `<seconds>` (a Bash call caps at 10 min).
 *
 *   bun 🧪️s4-gates-taxonomy-scopes.ts <out.jsonl> <new-dirs-all.txt> <scopes.txt> <seconds>
 */
import { appendFileSync, existsSync, readFileSync } from "node:fs";
import { verifyTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const [out, newDirsFile, scopesFile, seconds] = process.argv.slice(2);
const root = "/Users/ueli/Documents/semio";
const started = Date.now();
const lines = (path: string): string[] => readFileSync(path, "utf8").split("\n").filter(Boolean);
const newDirs = lines(newDirsFile!);
const ours = (path: string): boolean => newDirs.some((dir) => path === dir || path.startsWith(`${dir}/`));
const done = new Set(existsSync(out!) ? lines(out!).map((line) => (JSON.parse(line) as { scope: string }).scope) : []);
const pending = lines(scopesFile!).filter((scope) => !done.has(scope));
for (const scope of pending) {
  if ((Date.now() - started) / 1000 > Number(seconds)) break;
  const begun = Date.now();
  const verification = verifyTaxonomy({ repoRoot: root, scope, workers: 4 });
  const rows = verification.violations.map((violation) => ({ scope, ours: ours(violation.path), severity: violation.severity, code: violation.code, path: violation.path, message: violation.message }));
  appendFileSync(out!, [...rows, { scope, ours: false, severity: "done", code: "scope-complete", path: scope, message: `clean=${verification.clean} violations=${rows.length}` }].map((row) => `${JSON.stringify(row)}\n`).join(""));
  console.log(`[s4-gates taxonomy] ${scope} clean=${verification.clean} violations=${rows.length} ours=${rows.filter((row) => row.ours).length} seconds=${Math.round((Date.now() - begun) / 1000)}`);
}
console.log(`[s4-gates taxonomy] done=${done.size + pending.filter((scope) => lines(out!).some((line) => line.includes(`"scope":${JSON.stringify(scope)}`))).length} pending=${lines(scopesFile!).length - new Set(lines(out!).map((line) => (JSON.parse(line) as { scope: string }).scope)).size}`);
