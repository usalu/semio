import {policyReadRustPolicySource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import {policyReadModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
for (const p of ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs"]) {
  try { const s = policyReadRustPolicySource(root, p); console.log("policy", p.slice(-40), s.length, s.includes("struct SurfaceSlot"), s.indexOf("struct SurfaceSlot") < s.indexOf("impl PatchTracker")); }
  catch (e) { console.log("policy throws", String((e as Error).message).slice(0, 300)); }
  const t = policyReadModuleTreeSource(root, p); console.log("tree", t.length);
}
