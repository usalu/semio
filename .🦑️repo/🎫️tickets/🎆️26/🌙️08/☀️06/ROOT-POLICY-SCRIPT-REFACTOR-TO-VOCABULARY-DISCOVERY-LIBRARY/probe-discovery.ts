// 🔎️ Scratch probe (ticket 26/08/06/ROOT-POLICY-SCRIPT-REFACTOR-TO-VOCABULARY-DISCOVERY-LIBRARY):
// prints what M1's discoverPackages/discoverOwners see today, to size the root-policy refactor.
import { discoverPackages, discoverOwners, discoverPackageProblems, loadTaxonomy } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/🟦️discovery.ts";

const root = process.cwd();
const tax = loadTaxonomy();
const pkgs = discoverPackages(root, tax);
console.log("packages:", pkgs.length);
const byRole: Record<string, number> = {};
for (const p of pkgs) byRole[`${p.role}/${p.lang}`] = (byRole[`${p.role}/${p.lang}`] ?? 0) + 1;
for (const k of Object.keys(byRole).sort()) console.log("  ", byRole[k], k);
console.log("\nrust packages by role:");
for (const p of pkgs.filter((p) => p.lang === "🦀️rust").sort((a, b) => a.role.localeCompare(b.role) || a.ownerRel.localeCompare(b.ownerRel))) {
  console.log(`  ${p.role.padEnd(10)} area=${(p.area || "-").padEnd(8)} mat=${p.maturity.padEnd(6)} ${p.packageRel}`);
}
console.log("\nowners:", discoverOwners(root, tax).length);
console.log("problems:");
for (const pr of discoverPackageProblems(root, tax)) console.log("  ", pr.kind, pr.path);
