#!/usr/bin/env bun
// Standalone seed generator for S2's S-8 policy rules — mirrors 📜️script.ts's own
// policyNormalizeRelPath/policyStripEmoji/policyFileSuffix/policyCanonicalComponent logic exactly
// (copied, not imported, to avoid touching script.ts's export surface just for tooling) so the
// allowlist keys hardcoded into script.ts match byte-for-byte what the real rule will compute.
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const repoRoot = process.cwd();

function policyStripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}
const POLICY_COMPONENT_ALIASES: Record<string, string> = { protocol: "spr" };
function policyCanonicalComponent(segment: string): string {
  const ascii = policyStripEmoji(segment);
  return POLICY_COMPONENT_ALIASES[ascii] ?? ascii;
}
function policyFileSuffix(tailSegments: readonly string[], defaultFile: string): string {
  if (tailSegments.length === 1 && tailSegments[0] === defaultFile) return "";
  return `#${tailSegments.map((s) => policyStripEmoji(s.replace(/\.rs$/, ""))).join("-")}`;
}
function policyNormalizeRelPath(relPath: string): string {
  const norm = relPath.startsWith("./") ? relPath.slice(2) : relPath;
  const segments = norm.split("/");
  const artifactsIdx = segments.indexOf("🗿️artifacts");
  if (artifactsIdx > 0 && segments.length > artifactsIdx + 2) {
    const pluginId = policyStripEmoji(segments[artifactsIdx - 1] ?? "");
    const artifactId = policyStripEmoji(segments[artifactsIdx + 1] ?? "");
    const component = policyCanonicalComponent(segments[artifactsIdx + 2]!);
    const suffix = policyFileSuffix(segments.slice(artifactsIdx + 3), "🦀️component.rs");
    return (artifactId && artifactId !== pluginId ? `${pluginId}/${artifactId}/${component}` : `${pluginId}/${component}`) + suffix;
  }
  return norm;
}

function policyStripRustCommentsAndStrings(content: string): string {
  let out = "";
  let i = 0;
  const n = content.length;
  while (i < n) {
    const two = content.slice(i, i + 2);
    if (two === "//") {
      while (i < n && content[i] !== "\n") i++;
      continue;
    }
    if (two === "/*") {
      i += 2;
      let depth = 1;
      while (i < n && depth > 0) {
        const pair = content.slice(i, i + 2);
        if (pair === "/*") { depth++; i += 2; continue; }
        if (pair === "*/") { depth--; i += 2; continue; }
        i++;
      }
      continue;
    }
    if (content[i] === '"') {
      out += '"';
      i++;
      while (i < n && content[i] !== '"') {
        if (content[i] === "\\") { i += 2; continue; }
        i++;
      }
      if (i < n) i++;
      out += '"';
      continue;
    }
    out += content[i];
    i++;
  }
  return out;
}
function policyRustFileHasRealTraitImpl(body: string, traitName: string): boolean {
  const stripped = policyStripRustCommentsAndStrings(body);
  return new RegExp(`\\bimpl\\s*(?:<[^>{]*>\\s*)?${traitName}\\b\\s*(?:<[^>{]*>\\s*)?for\\b`).test(stripped);
}

const STDIO_ARTIFACTS_REL = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts";

type Entry = { artRel: string; artifactId: string; standardSlug: string; subsetRel: string; subsetId: string };
function listStdioEntries(): Entry[] {
  const out: Entry[] = [];
  for (const art of readdirSync(join(repoRoot, STDIO_ARTIFACTS_REL), { withFileTypes: true })) {
    if (!art.isDirectory()) continue;
    const artRel = `${STDIO_ARTIFACTS_REL}/${art.name}`;
    const standardsRel = `${artRel}/🏅️standards`;
    if (!existsSync(join(repoRoot, standardsRel))) continue;
    for (const std of readdirSync(join(repoRoot, standardsRel), { withFileTypes: true })) {
      if (!std.isDirectory() || !std.name.startsWith("🔖️")) continue;
      const standardRel = `${standardsRel}/${std.name}`;
      const standardSlug = std.name.slice("🔖️".length);
      const subsetsRel = `${standardRel}/🪆️subsets`;
      if (!existsSync(join(repoRoot, subsetsRel))) continue;
      for (const sub of readdirSync(join(repoRoot, subsetsRel), { withFileTypes: true })) {
        if (!sub.isDirectory()) continue;
        out.push({
          artRel,
          artifactId: policyStripEmoji(art.name),
          standardSlug,
          subsetRel: `${subsetsRel}/${sub.name}`,
          subsetId: sub.name,
        });
      }
    }
  }
  return out;
}

const entries = listStdioEntries();
const anyEntries = entries.filter((e) => e.subsetId === "✳️any");
console.log(`total subset entries: ${entries.length}, "any" subset entries: ${anyEntries.length}`);

//#region DIFF_ALGEBRA
const diffAlgebraSeed: string[] = [];
for (const e of anyEntries) {
  const rustRel = `${e.subsetRel}/🧬️schema/🔺️diff/🦀️component.rs`;
  const abs = join(repoRoot, rustRel);
  if (!existsSync(abs)) continue;
  const content = readFileSync(abs, "utf8");
  if (!policyRustFileHasRealTraitImpl(content, "DiffAlgebra")) {
    diffAlgebraSeed.push(policyNormalizeRelPath(rustRel));
  }
}
console.log("\n=== POLICY_DIFF_ALGEBRA seed (" + diffAlgebraSeed.length + ") ===");
for (const k of diffAlgebraSeed.sort()) console.log(`  "${k}",`);
//#endregion

//#region FIELD_SWEEP
function stdioStandardKey(artifactId: string, standardSlug: string): string {
  return `stdio/${artifactId}/standards#${standardSlug}`;
}
function walkRsFiles(relDir: string, out: string[]): void {
  let entries: ReturnType<typeof readdirSync>;
  try {
    entries = readdirSync(join(repoRoot, relDir), { withFileTypes: true });
  } catch {
    return;
  }
  for (const ent of entries) {
    const childRel = `${relDir}/${ent.name}`;
    if (ent.isDirectory()) {
      walkRsFiles(childRel, out);
      continue;
    }
    if (ent.name.endsWith(".rs")) out.push(childRel);
  }
}
const fieldSweepSeed: string[] = [];
for (const e of anyEntries) {
  const standardRel = e.subsetRel.split("/🪆️subsets/")[0]!;
  const rsFiles: string[] = [];
  walkRsFiles(standardRel, rsFiles);
  let found = false;
  for (const f of rsFiles) {
    const content = readFileSync(join(repoRoot, f), "utf8");
    if (/fn\s+\w*field_sweep\w*\s*\(/.test(content)) {
      found = true;
      break;
    }
  }
  if (!found) fieldSweepSeed.push(stdioStandardKey(e.artifactId, e.standardSlug));
}
console.log("\n=== POLICY_FIELD_SWEEP seed (" + fieldSweepSeed.length + ") ===");
for (const k of fieldSweepSeed.sort()) console.log(`  "${k}",`);
//#endregion

//#region GRAMMAR_HONESTY
const LEAF_MARKERS: Record<string, string> = {
  "🅰️component.g4": "DOCUMENT: 'schema' [ ]+",
  "🔤️component.ebnf": "header = 'schema', space,",
  "📖️component.grammar.semio": "payload = *OCTET",
  "🔠️component.abnf": "payload = *OCTET",
  "📡️component.protocol.semio": "payload = *OCTET",
  "🥋️component.ksy": "size-eos: true",
  "🌶️component.spicy": "payload: bytes &eod;",
};
const grammarSeed: string[] = [];
function walkGrammarLeaves(relDir: string): void {
  let ents: ReturnType<typeof readdirSync>;
  try {
    ents = readdirSync(join(repoRoot, relDir), { withFileTypes: true });
  } catch {
    return;
  }
  for (const ent of ents) {
    const childRel = `${relDir}/${ent.name}`;
    if (ent.isDirectory()) {
      walkGrammarLeaves(childRel);
      continue;
    }
    const marker = LEAF_MARKERS[ent.name];
    if (!marker) continue;
    const content = readFileSync(join(repoRoot, childRel), "utf8");
    if (content.includes(marker)) grammarSeed.push(policyNormalizeRelPath(childRel));
  }
}
walkGrammarLeaves(STDIO_ARTIFACTS_REL);
console.log("\n=== POLICY_GRAMMAR_HONESTY seed (" + grammarSeed.length + ") ===");
for (const k of grammarSeed.sort()) console.log(`  "${k}",`);
//#endregion

//#region FACET_MIRROR_DRIFT
const FACETS = ["📸️snapshot", "🔺️diff", "🧬️mutations"];
const SIBLINGS = ["🟦️component.ts", "🔗️component.graphql", "🔣️component.json", "🛰️component.proto"];
const FIELD_RE = /(?:^|[\s{,(])(?:pub\s+)?([a-z][a-z0-9_]*)\s*:\s*[A-Za-z_&[<('"]/gm;
const KEYWORDS = new Set(["self", "where", "if", "else", "match", "for", "while", "let", "fn", "return", "in", "as", "dyn", "mut", "ref", "impl", "type"]);
function snakeToCamel(name: string): string {
  const parts = name.split("_");
  return parts[0] + parts.slice(1).map((p) => p.charAt(0).toUpperCase() + p.slice(1)).join("");
}
function extractFields(content: string): Set<string> {
  const stripped = policyStripRustCommentsAndStrings(content);
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  FIELD_RE.lastIndex = 0;
  while ((m = FIELD_RE.exec(stripped))) {
    const n = m[1]!;
    if (KEYWORDS.has(n) || n.startsWith("r#")) continue;
    names.add(n);
  }
  return names;
}
const driftSeed: string[] = [];
for (const e of anyEntries) {
  for (const facet of FACETS) {
    const facetRel = `${e.subsetRel}/🧬️schema/${facet}`;
    const rustRel = `${facetRel}/🦀️component.rs`;
    const abs = join(repoRoot, rustRel);
    if (!existsSync(abs)) continue;
    const fields = extractFields(readFileSync(abs, "utf8"));
    const camelFields = [...fields].map(snakeToCamel).filter(Boolean);
    let anyMissing = false;
    for (const sib of SIBLINGS) {
      const sibAbs = join(repoRoot, facetRel, sib);
      if (!existsSync(sibAbs)) {
        anyMissing = true;
        continue;
      }
      const sibContent = readFileSync(sibAbs, "utf8");
      if (camelFields.some((f) => !sibContent.includes(f))) anyMissing = true;
    }
    if (anyMissing) driftSeed.push(policyNormalizeRelPath(rustRel));
  }
}
console.log("\n=== POLICY_FACET_MIRROR_DRIFT seed (" + driftSeed.length + ") ===");
for (const k of driftSeed.sort()) console.log(`  "${k}",`);
//#endregion

//#region VCS_MACHINERY_BAN sanity
const banned = ["CollectionDiff", "CollectionMutation", "Patchable", "Identified"];
const bannedPattern = new RegExp(`\\b(${banned.join("|")})\\b`);
let vcsHits = 0;
function walkAllRs(relDir: string, out: string[]): void {
  let ents: ReturnType<typeof readdirSync>;
  try {
    ents = readdirSync(join(repoRoot, relDir), { withFileTypes: true });
  } catch {
    return;
  }
  for (const ent of ents) {
    const childRel = `${relDir}/${ent.name}`;
    if (ent.isDirectory()) {
      walkAllRs(childRel, out);
      continue;
    }
    if (ent.name.endsWith(".rs")) out.push(childRel);
  }
}
const allStdioRs: string[] = [];
walkAllRs(STDIO_ARTIFACTS_REL, allStdioRs);
for (const f of allStdioRs) {
  const content = readFileSync(join(repoRoot, f), "utf8");
  if (bannedPattern.test(content)) {
    vcsHits++;
    console.log("VCS MACHINERY HIT:", f);
  }
}
console.log("\n=== POLICY_STDIO_VCS_MACHINERY_BAN current hits: " + vcsHits + " (expect 0) ===");
//#endregion
