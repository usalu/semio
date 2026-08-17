#!/usr/bin/env bun
/**
 * [DEBUG] Splice P3/M4 scanners + exemptions into root 📜️script.ts.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../..");
const scriptPath = join(repoRoot, "📜️script.ts");
const exemptions = readFileSync(join(ticketDir, "🧪p3-exemption-sets.ts.txt"), "utf8").trimEnd();
const scanners = readFileSync(join(ticketDir, "🧪p3-scanners.ts.txt"), "utf8").trimEnd();

let src = readFileSync(scriptPath, "utf8");
const before = src.length;

const oldFsImport =
  'import { existsSync, linkSync, mkdirSync, chmodSync, chownSync, copyFileSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";';
const newFsImport =
  'import { createHash } from "node:crypto";\nimport { existsSync, linkSync, mkdirSync, chmodSync, chownSync, copyFileSync, readFileSync, readdirSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";';
if (!src.includes('from "node:crypto"')) {
  if (!src.includes(oldFsImport)) throw new Error("fs import not found");
  src = src.replace(oldFsImport, newFsImport);
}

const allowlistsEnd = "//#endregion 🔧️PolicyAllowlists";
if (!src.includes("POLICY_SPEC_DISTINCTNESS_EXEMPTIONS")) {
  if (!src.includes(allowlistsEnd)) throw new Error("allowlists endregion missing");
  src = src.replace(allowlistsEnd, `${exemptions}\n${allowlistsEnd}`);
} else {
  console.log("[DEBUG] exemptions already present — skipping");
}

const policyExport = "//#region 🔖️PolicyExport";
if (!src.includes("function policySpecDistinctnessBreaches")) {
  if (!src.includes(policyExport)) throw new Error("PolicyExport region missing");
  src = src.replace(policyExport, `${scanners}\n\n${policyExport}`);
} else {
  console.log("[DEBUG] scanners already present — skipping");
}

const policyMarker = "  breaches.push(...policyTsFacadeBreaches(repoRoot));";
const policyInsert =
  "  breaches.push(...policyTsFacadeBreaches(repoRoot));\n  breaches.push(...policyHandcraftedSpecP3Breaches(repoRoot));";
if (!src.includes("policyHandcraftedSpecP3Breaches(repoRoot)")) {
  if (!src.includes(policyMarker)) throw new Error("policyTsFacadeBreaches call missing");
  src = src.replace(policyMarker, policyInsert);
} else {
  console.log("[DEBUG] policy export already wired — skipping");
}

const gateNeedle = '    console.log("[verify] dsl fixture laws…");';
const gateInsert = `    console.log("[verify] handcrafted grammar P3/M4 policies…");
    {
      const handcraftedBreaches = policyHandcraftedSpecP3Breaches(this.root);
      if (handcraftedBreaches.length > 0) {
        for (const b of handcraftedBreaches) {
          console.error(\`[verify] \${b.kind}: \${b.summary}\`);
        }
        throw new Error(\`[verify] \${handcraftedBreaches.length} handcrafted-grammar P3/M4 policy breach(es)\`);
      }
    }
    console.log("[verify] dsl fixture laws…");`;
if (!src.includes("handcrafted grammar P3/M4 policies")) {
  if (!src.includes(gateNeedle)) throw new Error("dsl fixture laws marker missing");
  // Insert only the first occurrence inside VerifyScript.runGate
  src = src.replace(gateNeedle, gateInsert);
} else {
  console.log("[DEBUG] verify gate already wired — skipping");
}

writeFileSync(scriptPath, src);
console.log(`[DEBUG] spliced 📜️script.ts ${before} → ${src.length} bytes (+${src.length - before})`);
