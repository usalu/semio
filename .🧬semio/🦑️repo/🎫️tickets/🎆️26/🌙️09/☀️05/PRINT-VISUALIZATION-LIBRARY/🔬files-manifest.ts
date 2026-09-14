#!/usr/bin/env bun
/** 📄 Writes `📓️files.md`: every file the fleet created, updated or removed, grouped by area.
 *
 * The status marks are git's: `A` added, `M` modified, `D` removed, `R` renamed, `??` untracked,
 * two letters meaning staged-then-modified. `git status` is read ONLY — nothing here writes to git.
 */
import { execFileSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "C:/git/semio";
const TICKET = join(import.meta.dir);
const HEADER = `# Files — everything the fleet touched

Derived from a read-only \`git status --short\` over the areas the ticket owns, cross-read against
the agents' status files. The whole print product is staged as *added*: it has no committed
predecessor in \`HEAD\`, so \`A\` here means "this file is the ticket's work", not "new this hour".

Marks are git's: \`A\` added, \`M\` modified, \`D\` removed, \`R\` renamed, \`??\` untracked,
\`AM\`/\`MM\`/\`AD\` staged-then-changed.

`;

function status(paths: readonly string[]): { readonly mark: string; readonly path: string }[] {
  const raw = execFileSync("git", ["status", "--short", "-z", "--", ...paths], { cwd: ROOT, maxBuffer: 64 << 20 }).toString("utf8");
  const records = raw.split("\0").filter((entry) => entry.length > 0);
  const rows: { mark: string; path: string }[] = [];
  for (let index = 0; index < records.length; index += 1) {
    const record = records[index]!;
    if (record[0] === "R") index += 1;
    rows.push({ mark: record.slice(0, 2).trim(), path: record.slice(3) });
  }
  return rows.sort((left, right) => left.path.localeCompare(right.path));
}

function section(title: string, note: string, rows: readonly { readonly mark: string; readonly path: string }[]): string {
  const counts = new Map<string, number>();
  for (const row of rows) counts.set(row.mark, (counts.get(row.mark) ?? 0) + 1);
  const summary = [...counts].sort().map(([mark, count]) => `${count} ${mark}`).join(", ");
  return [`## ${title}`, "", note, "", `${rows.length} entries — ${summary}.`, "", "```", ...rows.map((row) => `${row.mark.padEnd(2)}  ${row.path}`), "```", ""].join("\n");
}

const print = status(["🧰️framework/🛍️products/📓️print"]);
const platform = status([
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️viz-kernel",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
]);
const workspace = status([".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc", "package.json", "bun.lock", "♻️mit-bestand/📋️bericht/📎️anhang/📈️skalierung.tex"]);

const body = [
  HEADER,
  section(
    "The print product — `🧰️framework/🛍️products/📓️print/`",
    "The library itself: the LaTeX packages, the catalogue and its schema, the generated gallery documents, the TypeScript modules and commands, and the 104 test cases. Every `D` under `🖋️latex/` is a package of the previous library that a rewritten one replaced; every `D` under `🧪️tests/` is a case that TESTS-2 renamed to carry its emoji identity (the rename shows as a delete plus an add because the whole tree is staged as added).",
    print,
  ),
  section(
    "The repository test platform — `🧰️framework/🛍️products/🦑️repo/`",
    "Print's share of the platform: the test runner and its TypeScript package (TESTS-HARNESS, TESTS-2), the TypeScript kernel twin `📊️viz-kernel` (TS-TWIN), and the taxonomy the case-slug rule lives in. The rest of the `🦑️repo` churn in the working tree belongs to other tickets and is deliberately not listed here.",
    platform,
  ),
  section(
    "Workspace files",
    "The launch configurations every dev uses (`.vscode/🧩️launch.seed.jsonc` is the source, `launch.json` its build), the workspace manifest and lockfile for the test-only oracle devDependencies, and the one `♻️mit-bestand` document that calls the library: `📈️skalierung.tex`, ported onto `\\SemioVizLayout`. The 37 other `♻️mit-bestand` files in the working tree are the presentation package and are not this ticket's work.",
    workspace,
  ),
  [
    "## Ticket folder",
    "",
    "Inputs kept (scripts, probes, fixtures) and reports kept (markdown); tool output under",
    "`🗑️generated/` is removed at close. FINALIZE added `🔧️standalone-load.py`,",
    "`🔧️docstring-gaps.py`, `🔬gallery-lane.ts`, `🔬gallery-merge.ts` and `🔬files-manifest.ts`,",
    "and the reports `📓️status-FINALIZE.md`, `📓️completion.md` and this file.",
    "",
  ].join("\n"),
].join("\n");

writeFileSync(join(TICKET, "📓️files.md"), body, "utf8");
console.log(`[DEBUG] files manifest: print=${print.length} platform=${platform.length} workspace=${workspace.length}`);
