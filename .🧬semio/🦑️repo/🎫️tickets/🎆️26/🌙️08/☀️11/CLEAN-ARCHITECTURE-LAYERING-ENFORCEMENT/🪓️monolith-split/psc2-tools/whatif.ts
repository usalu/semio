/** 🔮️ What-if for carving a new package owner K out of its parent owner P: counts edges from files inside K to files that stay owned by P (they would become flagged by P's rule), and from outside into K (the edges K's facades would fix). Usage: bun whatif.ts <policy.json> <K> */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, scanSource } from "../sdkb-tools/lib.ts";
import { loadPolicy, groupsOf, listSourceFiles, nfc } from "./model.ts";
import { skipReason } from "./skip.ts";
const [policyPath, kArg] = process.argv.slice(2);
const K = nfc(kArg);
const policy = loadPolicy(policyPath);
const groups = groupsOf(policy);
const parent = groups.filter(g => K.startsWith(g.sourceOwner + "/")).sort((a, b) => b.sourceOwner.length - a.sourceOwner.length)[0];
if (!parent) { console.log("no parent owner"); process.exit(1); }
const files = listSourceFiles();
const inK = (p: string) => p === K || p.startsWith(K + "/");
const nestedUnderK = groups.filter(g => g.sourceOwner.startsWith(K + "/")).map(g => g.sourceOwner);
const inKOwned = (p: string) => inK(p) && !nestedUnderK.some(n => p.startsWith(n + "/"));
const ownedByParentAfter = (p: string) => parent.toPath.some(x => x.test(p)) && !inKOwned(p);
let leaving = 0, entering = 0;
const leave = new Map<string, number>(), enter = new Map<string, number>();
for (const f of files) {
  let text: string; try { text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
  if (!/(?:from|import|require)\s*\(?\s*["'`]/u.test(text)) continue;
  const fromK = inKOwned(f);
  for (const r of scanSource(f, text)) {
    if (!r.resolved) continue;
    const res = nfc(r.resolved);
    if (fromK && ownedByParentAfter(res)) { leaving++; const k = res.split("/").slice(0, 6).join("/"); leave.set(k, (leave.get(k) ?? 0) + 1); }
    if (!fromK && inKOwned(res) && !parent.fromPathNot.test(f) === false) { /* inside parent region */ }
    if (!fromK && inKOwned(res) && !skipReason(f) && !inK(f)) { entering++; const k = f.split("/").slice(0, 5).join("/"); enter.set(k, (enter.get(k) ?? 0) + 1); }
  }
}
console.log({ parent: parent.sourceOwner, leavingEdgesNewlyFlagged: leaving, enteringEdgesFromOutside: entering });
console.log("LEAVING", [...leave].sort((a, b) => b[1] - a[1]).slice(0, 8));
console.log("ENTERING", [...enter].sort((a, b) => b[1] - a[1]).slice(0, 8));
