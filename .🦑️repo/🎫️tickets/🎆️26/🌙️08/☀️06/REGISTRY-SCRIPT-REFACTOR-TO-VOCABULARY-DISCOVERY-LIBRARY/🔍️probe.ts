#!/usr/bin/env bun
/** 🔍️ Ticket-local probe: what `discoverPackages()` actually sees on disk today, grouped by role/lang,
 * next to what the pre-refactor registry regexes matched. Read-only. */
import { discoverPackages, discoverPackageProblems, discoverOwners, getWorkspaceRoot } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

const root = getWorkspaceRoot();
const packages = discoverPackages(root);
const byRole = new Map<string, typeof packages>();
for (const pkg of packages) byRole.set(pkg.role, [...(byRole.get(pkg.role) ?? []), pkg]);
for (const [role, rows] of [...byRole.entries()].sort()) {
  console.log(`\n### role=${role} (${rows.length})`);
  for (const row of rows) console.log(`  ${row.lang}${row.target ? `/${row.target}` : ""}  ${row.id}  <- ${row.packageRel}  [area=${row.area || "-"} maturity=${row.maturity}]`);
}
console.log(`\n### owners=${discoverOwners(root).length} packages=${packages.length}`);
console.log(`\n### problems (${discoverPackageProblems(root).length})`);
for (const problem of discoverPackageProblems(root)) console.log(`  [${problem.kind}] ${problem.message}`);
