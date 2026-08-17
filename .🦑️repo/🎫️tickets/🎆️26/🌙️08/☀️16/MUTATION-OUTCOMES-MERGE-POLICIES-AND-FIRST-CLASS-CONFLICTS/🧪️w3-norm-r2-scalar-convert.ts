// 🧪️ Lane R2 (norm plugin resume), pass 1: converts every uniform single-field scalar `🔺️diff` leaf
// (`pub fn diff(payload, base) -> XDiff { XDiff { field: Some(payload.field), ..Default::default() } }`)
// across the six remaining norm facets (din4108, din18599, en1993, en1995, iso16757, vdi3805) from a
// bare `XDiff` return into `protocol::MutationOutcome<XDiff>`, per the change/set/update verb-family
// rule: Fatal `mutation.invariant` on a non-finite f64 input, Warning `mutation.no-op` when the new
// value equals the current snapshot value, else `MutationOutcome::new(..)`. Only touches leaves that
// match the uniform shape (confirmed by 🧪️w3-norm-survey.ts) — the remaining 55 non-uniform leaves
// (record updates, create/delete/rename/replace/resize/reorder/insert/add/remove) are hand-converted.
// Lives inside the ticket folder per CLAUDE.md (temporary scripts never live outside a ticket).
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

interface Facet {
  name: string;
  dir: string;
  snapshotType: string;
}

const ROOT = "/Users/ueli/Documents/semio";
const FACETS: Facet[] = [
  { name: "din4108", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108`, snapshotType: "Din4108Snapshot" },
  { name: "din18599", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599`, snapshotType: "Din18599Snapshot" },
  { name: "en1993", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993`, snapshotType: "En1993Snapshot" },
  { name: "en1995", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995`, snapshotType: "En1995Snapshot" },
  { name: "iso16757", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757`, snapshotType: "Iso16757Snapshot" },
  { name: "vdi3805", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805`, snapshotType: "Vdi3805Snapshot" },
];

function findFiles(dir: string, predicate: (p: string) => boolean, out: string[] = []): string[] {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    const st = statSync(full);
    if (st.isDirectory()) findFiles(full, predicate, out);
    else if (predicate(full)) out.push(full);
  }
  return out;
}

function parseSnapshotFieldTypes(snapshotFile: string): Map<string, string> {
  const content = readFileSync(snapshotFile, "utf8");
  const map = new Map<string, string>();
  const structStart = content.indexOf("pub struct ");
  const body = content.slice(structStart);
  const lineRe = /^\s*pub (\w+): ([^,]+),\s*$/gm;
  let m: RegExpExecArray | null;
  while ((m = lineRe.exec(body))) {
    map.set(m[1], m[2].trim());
  }
  return map;
}

function toLabel(entity: string, fieldName: string): string {
  const words = (entity || fieldName).split("-").join(" ").split("_").join(" ");
  return words.charAt(0).toUpperCase() + words.slice(1);
}

const DIFF_FN_RE =
  /pub fn diff\(payload: &(\w+), _?base: &(\w+)\) -> (\w+) \{\n\s*\3 \{ (\w+): Some\(payload\.(\w+)(?:\.clone\(\))?\), \.\.Default::default\(\) \}\n\}/;

let converted = 0;
let skipped: string[] = [];

for (const facet of FACETS) {
  const snapshotFile = findFiles(facet.dir, (p) => p.endsWith("📸️snapshot/🦀️component.rs"))[0];
  if (!snapshotFile) {
    console.log(`NO SNAPSHOT FOUND for ${facet.name}`);
    continue;
  }
  const fieldTypes = parseSnapshotFieldTypes(snapshotFile);

  const diffFiles = findFiles(facet.dir, (p) => p.includes("🧬️mutations/") && p.endsWith("🔺️diff/🦀️component.rs"));

  for (const diffFile of diffFiles) {
    const content = readFileSync(diffFile, "utf8");
    if (content.includes("MutationOutcome")) continue; // already converted
    const match = DIFF_FN_RE.exec(content);
    if (!match) {
      skipped.push(diffFile);
      continue;
    }
    const [whole, payloadType, snapshotType, diffType, fieldName, payloadField] = match;
    const fieldRustType = fieldTypes.get(fieldName);
    const isNumeric = fieldRustType === "f64";

    const mutationFile = diffFile.replace("🔺️diff/🦀️component.rs", "🦠️mutation/🦀️component.rs");
    let entity = fieldName;
    try {
      const mutContent = readFileSync(mutationFile, "utf8");
      const entityMatch = /entity: "([^"]+)"/.exec(mutContent);
      if (entityMatch) entity = entityMatch[1];
    } catch {
      // fall back to fieldName
    }
    const label = toLabel(entity, fieldName);

    const finiteCheck = isNumeric
      ? `    if !payload.${payloadField}.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "${label} must be a finite number.", Vec::<String>::new());\n    }\n`
      : "";

    const newFn =
      `pub fn diff(payload: &${payloadType}, base: &${snapshotType}) -> protocol::MutationOutcome<${diffType}> {\n` +
      finiteCheck +
      `    if base.${fieldName} == payload.${payloadField} {\n` +
      `        return protocol::MutationOutcome::empty().warn("mutation.no-op", "${label} already has this value.");\n` +
      `    }\n` +
      `    protocol::MutationOutcome::new(${diffType} { ${fieldName}: Some(payload.${payloadField}.clone()), ..Default::default() })\n` +
      `}`;

    const newContent = content.slice(0, match.index) + newFn + content.slice(match.index + whole.length);
    writeFileSync(diffFile, newContent, "utf8");

    // --- Mutation leaf: only the fn diff return-type annotation changes. ---
    const mutSrc = readFileSync(mutationFile, "utf8");
    const mutFnRe = new RegExp(`fn diff\\(&self, base: &${snapshotType}\\) -> ${diffType} \\{`);
    if (mutFnRe.test(mutSrc)) {
      const newMutSrc = mutSrc.replace(
        mutFnRe,
        `fn diff(&self, base: &${snapshotType}) -> protocol::MutationOutcome<${diffType}> {`
      );
      writeFileSync(mutationFile, newMutSrc, "utf8");
    } else if (!mutSrc.includes("MutationOutcome")) {
      console.log(`WARN: no mutation fn sig match for ${mutationFile}`);
    }

    converted++;
  }
}

console.log(`Converted: ${converted}`);
console.log(`Skipped (${skipped.length}):`);
for (const s of skipped) console.log(`  ${s}`);
