#!/usr/bin/env bun
/**
 * [DEBUG] Splice POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS + policyGenericCodecDeriveBreaches into root script.ts.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const root = join(ticketDir, "../../../../../../");
const scriptPath = join(root, "📜️script.ts");
const exemptions = readFileSync(join(ticketDir, "🧪generic-codec-derive-exemptions.ts.txt"), "utf8").trimEnd();

let src = readFileSync(scriptPath, "utf8");

const allowlistAnchor = "//#endregion 🔧️PolicyAllowlists";
if (!src.includes("POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS")) {
  if (!src.includes(allowlistAnchor)) throw new Error("allowlist endregion missing");
  src = src.replace(allowlistAnchor, `${exemptions}\n\n${allowlistAnchor}`);
}

const scanner = `
/**
 * ⚖️P1/M3b staged ban on generic codec derives: flags NEW \`#[derive(...DslDocument|DslOps...)]\`
 * uses under plugin \`🗿️artifacts/**/*.rs\` outside \`POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS\`.
 * Those macros emit DocumentDsl/OpText/DocumentPack/OpBinary today; full deletion of that emission is P6.
 */
function policyGenericCodecDeriveBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, ["✏️s/🔌️plugins"], (relPath, name) => {
    return relPath.includes("/🗿️artifacts/") && name.endsWith(".rs");
  });
  const deriveRe = /#\\[derive\\s*\\(([^)]*)\\)\\]/g;
  const banned = /\\b(?:dsl::)?(?:DslDocument|DslOps)\\b/;
  for (const relPath of files) {
    if (POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS.has(relPath)) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    let match: RegExpExecArray | null;
    deriveRe.lastIndex = 0;
    while ((match = deriveRe.exec(content)) !== null) {
      const attrs = match[1] ?? "";
      if (!banned.test(attrs)) continue;
      const before = content.slice(0, match.index);
      const line = before.split(/\\r?\\n/).length;
      const which = [...attrs.matchAll(/\\b(?:dsl::)?(DslDocument|DslOps)\\b/g)].map((m) => m[1]).join("+");
      breaches.push({
        id: \`generic-codec-derive-\${relPath}-\${line}\`,
        summary: \`New generic codec derive \${which} in "\${relPath}" (line \${line})\`,
        kind: "handcrafted-grammar/generic-codec-derive",
        scope: relPath,
        line,
        priority: "high",
        reason: "DslDocument/DslOps still emit generic DocumentDsl/OpText/DocumentPack/OpBinary; new uses are frozen mid-migration. Full derive emission deletion is P6.",
        solution: \`Handcraft codecs for \${relPath} (or add to POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS only during migration), then remove the derive by P6.\`,
      });
    }
  }
  return breaches;
}
`;

if (!src.includes("function policyGenericCodecDeriveBreaches")) {
  const aggStart = "/** ⚖️Aggregates all P3/M4 handcrafted-grammar high-priority scanners for policy + verify gate. */";
  if (!src.includes(aggStart)) throw new Error("aggregator comment missing");
  src = src.replace(aggStart, `${scanner.trim()}\n\n${aggStart}`);
}

const oldAggBody = `function policyHandcraftedSpecP3Breaches(repoRoot: string): BreachRecord[] {
  return [
    ...policySpecDistinctnessBreaches(repoRoot),
    ...policyGenericSpecBreaches(repoRoot),
    ...policyDeclaredUseBreaches(repoRoot),
    ...policySpecWiringBreaches(repoRoot),
    ...policyEmptyExampleBreaches(repoRoot),
  ];
}`;

const newAggBody = `function policyHandcraftedSpecP3Breaches(repoRoot: string): BreachRecord[] {
  return [
    ...policySpecDistinctnessBreaches(repoRoot),
    ...policyGenericSpecBreaches(repoRoot),
    ...policyDeclaredUseBreaches(repoRoot),
    ...policySpecWiringBreaches(repoRoot),
    ...policyEmptyExampleBreaches(repoRoot),
    ...policyGenericCodecDeriveBreaches(repoRoot),
  ];
}`;

if (!src.includes("policyGenericCodecDeriveBreaches(repoRoot)")) {
  if (!src.includes(oldAggBody)) throw new Error("aggregator body missing");
  src = src.replace(oldAggBody, newAggBody);
}

writeFileSync(scriptPath, src);
console.log("[DEBUG] spliced generic-codec derive policy into", scriptPath);
