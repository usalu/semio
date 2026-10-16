// 📣️ Read-only: runs the repository's own membership publisher in check mode for the given artifact workspaces (strict and retaining authored exclusions).
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const W = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts");
const lib = await import(W);
const t0 = Date.now();
const owners = lib.discoverCargoWorkspaces(ROOT);
console.log(`discovered ${owners.length} workspaces in ${Date.now() - t0} ms`);
let stale = 0;
for (const artifact of process.argv.slice(2)) {
  const manifest = `${artifact}/Cargo.toml`;
  const owner = owners.find((o: any) => o.manifest === manifest);
  if (!owner) { console.log(JSON.stringify({artifact: artifact.split("/").slice(-2).join("/"), admitted: false})); stale++; continue; }
  const res: Record<string, string> = {};
  for (const [label, retain] of [["strict", false], ["retain", true]] as const) {
    try { res[label] = lib.publishSelectedCargoWorkspaceMembershipV1(ROOT, owner, "check", owners, retain) ? "stale?" : "current"; } catch (e) { res[label] = String((e as Error).message).slice(0, 80); }
  }
  if (res.retain !== "current") stale++;
  console.log(JSON.stringify({artifact: artifact.split("/").slice(-2).join("/"), admitted: true, ...res}));
}
console.log("not current (retain):", stale);
