#!/usr/bin/env bun
/**
 * Extract top-level inline `pub mod <name> { ... }` bodies from ui wgpu lib.rs
 * into sibling `🦀️<name>.rs` files; leave lib.rs as wiring-only.
 */
import fs from "fs";
import path from "path";

const pathsFile = "/tmp/wgpu-split-paths.txt";
const env = Object.fromEntries(
  fs.readFileSync(pathsFile, "utf8").trim().split("\n").map((l) => {
    const i = l.indexOf("=");
    return [l.slice(0, i), l.slice(i + 1)];
  }),
);

const LIB = env.LIB;
const WGPU = env.WGPU;
const TICKET = env.TICKET;
const CRAB = "🦀️"; // U+1F998 U+FE0F

const text = fs.readFileSync(LIB, "utf8");
const lines = text.split("\n");
// Preserve final newline behaviour
const hadTrailingNewline = text.endsWith("\n");
if (lines.length && lines[lines.length - 1] === "") lines.pop();

function attrsBefore(i) {
  let j = i - 1;
  const attrs = [];
  while (j >= 0) {
    const t = lines[j].trim();
    if (t === "") {
      j--;
      continue;
    }
    if (t.startsWith("//") || t.startsWith("#[")) {
      attrs.unshift({ idx: j, line: lines[j] });
      j--;
      continue;
    }
    break;
  }
  return attrs;
}

function findMatchingBrace(startLineIdx) {
  let depth = 0;
  for (let k = startLineIdx; k < lines.length; k++) {
    const line = lines[k];
    // naive: count braces ignoring strings/comments — matches prior ticket practice
    for (let c = 0; c < line.length; c++) {
      const ch = line[c];
      if (ch === "{") depth++;
      else if (ch === "}") {
        depth--;
        if (depth === 0) return k;
      }
    }
  }
  throw new Error(`unbalanced brace from line ${startLineIdx + 1}`);
}

function dedentBody(bodyLines) {
  return bodyLines.map((line) => {
    if (line.startsWith("    ")) return line.slice(4);
    return line;
  });
}

// Find Label region
let labelStart = -1;
let labelEnd = -1;
for (let i = 0; i < lines.length; i++) {
  if (lines[i].includes("//#region") && lines[i].includes("Label") && !lines[i].includes("Localized")) {
    labelStart = i;
  }
  if (labelStart >= 0 && lines[i].includes("//#endregion") && lines[i].includes("Label") && i > labelStart) {
    labelEnd = i;
    break;
  }
}

const mods = [];
for (let i = 0; i < lines.length; i++) {
  const m = lines[i].match(/^(pub )?mod (\w+) \{/);
  if (!m) continue;
  const attrs = attrsBefore(i);
  const hasPath = attrs.some((a) => a.line.includes("#[path"));
  if (hasPath) continue;
  const end = findMatchingBrace(i);
  // Collect cfg attrs and preceding doc comments that are "attached"
  // Doc comments immediately before cfg/mod should stay with the declaration in lib.rs
  // OR move into the file. We'll keep /// docs that sit between previous content and cfg/mod
  // on the #[path] declaration in lib.rs (module docs work on external mods).
  let declStart = i;
  for (const a of attrs) {
    if (a.line.trim().startsWith("#[cfg") || a.line.trim().startsWith("#[path")) {
      declStart = Math.min(declStart, a.idx);
    }
  }
  // Also include contiguous /// or //! doc lines and blank lines immediately above declStart
  let docStart = declStart;
  let j = declStart - 1;
  while (j >= 0) {
    const t = lines[j].trim();
    if (t === "") {
      // peek further — only keep blank if docs above
      let k = j - 1;
      while (k >= 0 && lines[k].trim() === "") k--;
      if (k >= 0 && (lines[k].trim().startsWith("///") || lines[k].trim().startsWith("//!"))) {
        docStart = j;
        j = k;
        continue;
      }
      break;
    }
    if (t.startsWith("///") || t.startsWith("//!")) {
      docStart = j;
      j--;
      continue;
    }
    // region comments attached? keep // #region that sits right above
    if (t.startsWith("// #region") || t.startsWith("//#region")) {
      // don't pull region markers into the wiring — they live with old body
      break;
    }
    break;
  }

  mods.push({
    name: m[2],
    modLine: i,
    end,
    declStart,
    docStart,
    cfgs: attrs.filter((a) => a.line.trim().startsWith("#[cfg")).map((a) => a.line),
    n: end - i + 1,
  });
}

console.log("Found mods:", mods.map((m) => `${m.name} L${m.modLine + 1}-${m.end + 1} (${m.n})`).join("\n"));
console.log("Label region:", labelStart + 1, "-", labelEnd + 1);

// Build replacement plan: ranges to replace (inclusive line indices) -> wiring lines
// Process by reconstructing: walk through lines, when hitting a replace range, emit wiring and skip body.

const replacements = [];

// Label extraction
if (labelStart >= 0 && labelEnd >= 0) {
  const labelBody = lines.slice(labelStart + 1, labelEnd); // exclude region markers
  // Label body is already at crate-root indent (no extra 4 spaces for the region contents that aren't nested)
  // Looking at the file: contents after //#region are at column 0 — no dedent needed
  const labelFile = path.join(WGPU, `${CRAB}label.rs`);
  const labelHeader = [
    "//! 🎗️ Compile-time-checked UI labels (`Label` / `LabelText` / `LocalizedLabel` / `AppLabels`).",
    "//! Extracted from wgpu target `📦️lib.rs` (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE).",
    "",
  ];
  fs.writeFileSync(labelFile, labelHeader.concat(labelBody).join("\n") + "\n");
  replacements.push({
    start: labelStart,
    end: labelEnd,
    wiring: [
      "//#region 🔖️Label",
      `#[path = "${CRAB}label.rs"]`,
      "mod label_impl;",
      "pub use label_impl::*;",
      "//#endregion 🔖️Label",
    ],
    name: "label",
  });
  console.log("Wrote", labelFile, "lines", labelBody.length);
}

for (const mod of mods) {
  const bodyLines = lines.slice(mod.modLine + 1, mod.end); // exclusive of closing brace line
  // Drop trailing blank lines that only existed before `}`
  while (bodyLines.length && bodyLines[bodyLines.length - 1].trim() === "") bodyLines.pop();
  const dedented = dedentBody(bodyLines);

  // Convert leading `// #region name` / `#endregion` — keep them
  // If first non-empty is `// #region X` and file has `//!` from region, fine.

  // Build file header from existing module docs if any were inside body as `//!`
  const outFile = path.join(WGPU, `${CRAB}${mod.name}.rs`);
  let content = dedented.join("\n");
  if (!content.endsWith("\n")) content += "\n";
  // Ensure file has a module doc if missing
  if (!content.trimStart().startsWith("//!") && !content.trimStart().startsWith("// #region") && !content.trimStart().startsWith("//#region")) {
    content = `//! 🧩 \`${mod.name}\` engine module — extracted from wgpu \`📦️lib.rs\` (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE).\n\n` + content;
  }
  fs.writeFileSync(outFile, content);
  console.log("Wrote", outFile, "lines", content.split("\n").length);

  // Wiring: keep docs (docStart..declStart-1), then cfg, then #[path], then pub mod name;
  const wiring = [];
  // docs attached above cfg
  for (let i = mod.docStart; i < Math.min(mod.declStart, mod.modLine); i++) {
    const t = lines[i].trim();
    if (t.startsWith("///") || t.startsWith("//!") || t === "") {
      wiring.push(lines[i]);
    }
  }
  for (const cfg of mod.cfgs) wiring.push(cfg);
  wiring.push(`#[path = "${CRAB}${mod.name}.rs"]`);
  wiring.push(`pub mod ${mod.name};`);

  replacements.push({
    start: mod.docStart,
    end: mod.end,
    wiring,
    name: mod.name,
  });
}

// Sort replacements by start ascending; verify no overlap
replacements.sort((a, b) => a.start - b.start);
for (let i = 1; i < replacements.length; i++) {
  if (replacements[i].start <= replacements[i - 1].end) {
    throw new Error(`overlap between ${replacements[i - 1].name} and ${replacements[i].name}`);
  }
}

const outLines = [];
let cursor = 0;
for (const r of replacements) {
  while (cursor < r.start) {
    outLines.push(lines[cursor]);
    cursor++;
  }
  outLines.push(...r.wiring);
  cursor = r.end + 1;
}
while (cursor < lines.length) {
  outLines.push(lines[cursor]);
  cursor++;
}

// Collapse excessive blank lines (max 2 consecutive)
const cleaned = [];
let blankRun = 0;
for (const line of outLines) {
  if (line.trim() === "") {
    blankRun++;
    if (blankRun <= 2) cleaned.push(line);
  } else {
    blankRun = 0;
    cleaned.push(line);
  }
}

const beforeCount = lines.length;
const afterCount = cleaned.length;
fs.writeFileSync(LIB, cleaned.join("\n") + "\n");

const report = {
  beforeLines: beforeCount,
  afterLines: afterCount,
  modsMoved: replacements.map((r) => r.name),
  files: replacements.map((r) => (r.name === "label" ? `${CRAB}label.rs` : `${CRAB}${r.name}.rs`)),
};

fs.writeFileSync(path.join(TICKET, "scratch-w6-wgpu-engine-split-result.json"), JSON.stringify(report, null, 2));
console.log("DONE", report);
