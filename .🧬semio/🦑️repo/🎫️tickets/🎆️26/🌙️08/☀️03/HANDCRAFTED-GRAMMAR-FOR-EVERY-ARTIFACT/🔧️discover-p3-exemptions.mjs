#!/usr/bin/env bun
/**
 * [DEBUG] Temporary discovery for P3/M4 policy exemption seeds.
 * Writes exemption lists into ticket folder; does not edit 📜️script.ts.
 */
import { createHash } from "node:crypto";
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../..");
const SKIP = new Set(["node_modules", ".git", ".🦑️repo", "target", "dist", ".claude", "vendor", ".venv", ".turbo", ".nx", ".storybook", "storybook-static"]);
const FACETS = new Set(["🗣️dsl", "🔧️op", "🔺️diff", "🎒️pack", "📡️spr"]);
const GRAMMAR_SPEC = "📖️component.grammar.semio";
const PROTOCOL_SPEC = "📡️component.protocol.semio";
const RS_COMPONENT = "🦀️component.rs";

function walk(relDir, pred, out = []) {
  const abs = join(repoRoot, relDir);
  let entries;
  try {
    entries = readdirSync(abs, { withFileTypes: true });
  } catch {
    return out;
  }
  for (const ent of entries) {
    const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
    if (ent.isDirectory()) {
      if (SKIP.has(ent.name) || ent.name.startsWith(".")) continue;
      walk(childRel, pred, out);
      continue;
    }
    if (pred(childRel, ent.name)) out.push(childRel);
  }
  return out;
}

function normalizeSpec(content) {
  return content
    .split(/\r?\n/)
    .map((line) => {
      if (/^(grammar|protocol|extension|schema)\s+\S+/.test(line)) return line.replace(/^(grammar|protocol|extension|schema)\s+\S+/, "$1 __NAME__");
      if (/^start\s+\S+/.test(line)) return "start __START__";
      return line;
    })
    .join("\n");
}

function hashOf(content) {
  return createHash("sha256").update(content).digest("hex");
}

// 1. Distinctness
const specFiles = [
  ...walk("✏️s", (p, n) => n.endsWith(".grammar.semio") || n.endsWith(".protocol.semio")),
  ...walk("🧰️framework", (p, n) => n.endsWith(".grammar.semio") || n.endsWith(".protocol.semio")),
].sort();
const byHash = new Map();
for (const rel of specFiles) {
  const raw = readFileSync(join(repoRoot, rel), "utf8");
  const h = hashOf(normalizeSpec(raw));
  if (!byHash.has(h)) byHash.set(h, []);
  byHash.get(h).push(rel);
}
const distinctnessPairs = [];
const distinctnessFiles = new Set();
for (const [, group] of byHash) {
  if (group.length < 2) continue;
  for (let i = 0; i < group.length; i++) {
    for (let j = i + 1; j < group.length; j++) {
      distinctnessPairs.push([group[i], group[j]]);
      distinctnessFiles.add(group[i]);
      distinctnessFiles.add(group[j]);
    }
  }
}

// 2. Generic
const genericOffenders = [];
const grammarUnderS = walk("✏️s", (p, n) => n.endsWith(".grammar.semio"));
for (const rel of grammarUnderS) {
  const content = readFileSync(join(repoRoot, rel), "utf8");
  const reasons = [];
  if (/prop\s*=\s*IDENT\s*"="\s*\(/.test(content) || /prop\s*=\s*IDENT\s*=/.test(content)) reasons.push("catchall-prop");
  const hasProp = /\bprop\b/.test(content);
  const hasList = /^\s*list\s*=/m.test(content) || /\blist\s*=/.test(content);
  const hasMap = /^\s*map\s*=/m.test(content) || /\bmap\s*=/.test(content);
  const hasValue = /^\s*value\s*=/m.test(content) || /\bvalue\s*=/.test(content);
  // untyped productions named exactly list/map/value used as catch-alls together with prop
  const untypedCatchalls = [];
  for (const name of ["list", "map", "value"]) {
    const re = new RegExp(`^\\s*${name}\\s*=`, "m");
    if (re.test(content)) untypedCatchalls.push(name);
  }
  if (hasProp && untypedCatchalls.length > 0) reasons.push(`untyped-${untypedCatchalls.join("+")}`);
  if (/=\s*IDENT\s+assign\*\s+block\?/.test(content) || /=\s*IDENT\s+assign\*/.test(content)) reasons.push("bare-stmt-shell");
  if (/-(json|blob|base64|payload)"/.test(content) || /-(json|blob|base64|payload)/.test(content)) {
    // field/name literals — only flag if looks like a field name in grammar
    if (/-(json|blob|base64|payload)/.test(content)) reasons.push("json-blob-field");
  }
  if (reasons.length) genericOffenders.push({ rel, reasons });
}

// 3. Declared use
const familyRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family";
const familyFiles = walk(familyRoot, (p, n) => n.startsWith("📖️family-") && n.endsWith(".grammar.semio"));
const familyById = new Map();
for (const rel of familyFiles) {
  const base = rel.split("/").pop().replace(/^📖️/, "").replace(/\.grammar\.semio$/, "");
  // family-graph etc
  familyById.set(base, rel);
  familyById.set(base.replace(/^family-/, ""), rel);
}
const declaredUseOffenders = [];
for (const rel of grammarUnderS) {
  const content = readFileSync(join(repoRoot, rel), "utf8");
  const useMatches = [...content.matchAll(/^use\s+(family-[\w-]+)/gm)];
  if (!useMatches.length) continue;
  for (const m of useMatches) {
    const fam = m[1];
    const famRel = familyById.get(fam) ?? familyById.get(fam.replace(/^family-/, ""));
    if (!famRel) {
      declaredUseOffenders.push({ rel, fam, reason: "missing-family" });
      continue;
    }
    const frag = readFileSync(join(repoRoot, famRel), "utf8");
    const prods = new Set();
    for (const pm of frag.matchAll(/^([A-Za-z_][\w-]*)\s*=/gm)) {
      prods.add(pm[1]);
    }
    // also production names after grammar header lines? keep simple
    let referenced = false;
    for (const prod of prods) {
      if (prod === "start" || prod === "grammar" || prod === "use" || prod === "extension" || prod === "dialect") continue;
      const re = new RegExp(`\\b${prod.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\b`);
      // Exclude the use line itself and family file references - check artifact body
      if (re.test(content)) {
        referenced = true;
        break;
      }
    }
    if (!referenced && prods.size > 0) {
      declaredUseOffenders.push({ rel, fam, reason: "no-prod-ref", prods: [...prods].slice(0, 20) });
    }
  }
}

// 4. Spec wiring
const wiringOffenders = [];
const registerOffenders = [];
const pluginsRoot = "✏️s/🔌️plugins";
function listDirs(rel) {
  const abs = join(repoRoot, rel);
  if (!existsSync(abs)) return [];
  return readdirSync(abs, { withFileTypes: true })
    .filter((e) => e.isDirectory() && !SKIP.has(e.name) && !e.name.startsWith("."))
    .map((e) => `${rel}/${e.name}`);
}
for (const pluginRel of listDirs(pluginsRoot)) {
  for (const artRel of listDirs(`${pluginRel}/🗿️artifacts`)) {
    // skip stubs carefully: demonstrator/playground, energy/model — only check facets that exist
    const isStubish =
      artRel.includes("/demonstrator/") ||
      artRel.includes("/playground/") ||
      artRel.includes("/energy/") ||
      /\/model(\/|$)/.test(artRel);
    let hasAnySpec = false;
    let hasRegisterAnywhere = false;
    // scan all rs under artifact for register_language
    const artRs = walk(artRel, (p, n) => n.endsWith(".rs"));
    for (const rs of artRs) {
      const body = readFileSync(join(repoRoot, rs), "utf8");
      if (body.includes("register_language")) hasRegisterAnywhere = true;
    }
    for (const facet of FACETS) {
      const facetRel = `${artRel}/${facet}`;
      if (!existsSync(join(repoRoot, facetRel))) continue;
      const grammarSpec = `${facetRel}/${GRAMMAR_SPEC}`;
      const protocolSpec = `${facetRel}/${PROTOCOL_SPEC}`;
      const hasGrammar = existsSync(join(repoRoot, grammarSpec));
      const hasProtocol = existsSync(join(repoRoot, protocolSpec));
      if (!hasGrammar && !hasProtocol) continue;
      hasAnySpec = true;
      const rsRel = `${facetRel}/${RS_COMPONENT}`;
      if (!existsSync(join(repoRoot, rsRel))) continue;
      const rsBody = readFileSync(join(repoRoot, rsRel), "utf8");
      const specName = hasGrammar ? GRAMMAR_SPEC : PROTOCOL_SPEC;
      // also check both if both exist
      const specs = [];
      if (hasGrammar) specs.push(GRAMMAR_SPEC);
      if (hasProtocol) specs.push(PROTOCOL_SPEC);
      for (const sn of specs) {
        if (!rsBody.includes(`include_str!`) || !rsBody.includes(sn)) {
          wiringOffenders.push({ rel: rsRel, spec: sn });
        }
      }
    }
    if (hasAnySpec && !hasRegisterAnywhere) {
      // engines under ⚙️engine/🦀️component.rs — also "or anywhere under the artifact"
      registerOffenders.push({ artRel, isStubish });
    }
  }
}

// 5. Empty examples
const emptyExamples = [];
const exampleFiles = walk("", (p, n) => {
  if (!p.includes("/📚️examples/") && !p.startsWith("📚️examples/")) return false;
  return n.endsWith(".pack.semio") || n.endsWith(".spr.semio");
});
for (const rel of exampleFiles) {
  const st = statSync(join(repoRoot, rel));
  if (st.size <= 64) emptyExamples.push({ rel, size: st.size });
}

const report = {
  distinctness: { collidingFiles: [...distinctnessFiles].sort(), pairCount: distinctnessPairs.length, groups: [...byHash.values()].filter((g) => g.length > 1).map((g) => g.sort()) },
  generic: genericOffenders,
  declaredUse: declaredUseOffenders,
  wiring: { includeStr: wiringOffenders, registerLanguage: registerOffenders },
  emptyExamples,
  counts: {
    distinctnessFiles: distinctnessFiles.size,
    distinctnessPairs: distinctnessPairs.length,
    generic: genericOffenders.length,
    declaredUse: declaredUseOffenders.length,
    wiringInclude: wiringOffenders.length,
    wiringRegister: registerOffenders.length,
    emptyExamples: emptyExamples.length,
    familyFiles: familyFiles.length,
    specFiles: specFiles.length,
  },
};

writeFileSync(join(ticketDir, "🧪p3-exemption-discovery.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report.counts, null, 2));
console.log(`[DEBUG] family files: ${familyFiles.slice(0, 5).join(", ")}`);
console.log(`[DEBUG] sample generic: ${genericOffenders.slice(0, 3).map((o) => o.rel).join(", ")}`);
console.log(`[DEBUG] sample declaredUse: ${JSON.stringify(declaredUseOffenders.slice(0, 2))}`);
