// 📖️ The assertion of the registry guard `🔬️rust-policy-source-evidence` ("every declared split source root reads as one text"), run without the root 📜️script.ts that peers currently keep unloadable.
import {existsSync, readFileSync} from "node:fs";
import {join} from "node:path";
import {POLICY_SPLIT_SOURCE_ROOTS, policyModuleTreeFiles, policyReadModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
let bad = 0;
for (const path of POLICY_SPLIT_SOURCE_ROOTS) {
  const ok = existsSync(join(repoRoot, path));
  if (!ok) { console.log("MISSING", path); bad++; continue; }
  const files = policyModuleTreeFiles(repoRoot, path);
  const text = policyReadModuleTreeSource(repoRoot, path);
  const good = files[0] === path && text.length >= readFileSync(join(repoRoot, path), "utf8").length;
  if (!good) bad++;
  if (path.includes("wgpu/🧊️renderer")) console.log("renderer root tree files", files.length, files.filter(f => f.includes("📥️mounts")).map(f => f.split("/").slice(-2, -1)[0]).join(","));
}
console.log("roots", POLICY_SPLIT_SOURCE_ROOTS.length, "bad", bad);
