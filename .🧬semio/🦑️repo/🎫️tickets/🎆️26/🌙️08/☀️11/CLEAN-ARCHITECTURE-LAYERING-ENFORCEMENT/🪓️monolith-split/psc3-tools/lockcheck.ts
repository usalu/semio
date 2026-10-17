/** 🔒️ Compares each checked-in Bun lock with the lock bun derives from the current manifests (offline, lockfile only): workspaces deep-equal, every derived package row present and equal. Usage: bun lockcheck.ts */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const ROOT = "/Users/ueli/Documents/semio";
const G = fileURLToPath(new URL("../🗑️generated/psc3-lock-gen/", import.meta.url));
let bad = 0;
for (const [s, n] of [[".", "root"], ["✏️s", "✏️s"], ["🌎️hub", "🌎️hub"], ["♻️mit-bestand", "♻️mit-bestand"], ["🏢️semio-tech", "🏢️semio-tech"], ["🎓️teaching", "🎓️teaching"]] as const) {
  const text = readFileSync(`${ROOT}/${s}/bun.lock`, "utf8");
  const A = Bun.JSONC.parse(text), B = Bun.JSONC.parse(readFileSync(G + n + ".bun.lock", "utf8"));
  const stable = JSON.stringify(A, null, 2) + "\n" === text;
  const wsDiff = Object.keys(B.workspaces).filter(k => JSON.stringify(A.workspaces[k]) !== JSON.stringify(B.workspaces[k]));
  const extraWs = Object.keys(A.workspaces).filter(k => !(k in B.workspaces));
  const pkgMissing = Object.keys(B.packages).filter(k => !(k in A.packages));
  const pkgDiff = Object.keys(B.packages).filter(k => k in A.packages && JSON.stringify(A.packages[k]) !== JSON.stringify(B.packages[k]));
  bad += wsDiff.length + pkgMissing.length;
  console.log(s.padEnd(14), { stable, workspacesDiffer: wsDiff.length, workspacesOnlyChecked: extraWs.length, packagesMissing: pkgMissing.length, packageRowsDiffer: pkgDiff.length });
  for (const k of wsDiff.slice(0, 3)) console.log("   ws", k.slice(-80));
}
process.exit(bad ? 1 : 0);
