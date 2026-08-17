// 🧪️ One-off ticket-scoped codegen: converts every uniform root-scoped `change-<field>` scalar
// `🔺️diff` leaf across lane 3-B's five norm facets (en1992/en1991/en1996/en1994/en1990) from
// `pub fn diff(payload, base) -> XDiff` into `-> protocol::MutationOutcome<XDiff>` with a real
// change/set/update verb-family message policy: Fatal `mutation.invariant` on a non-finite f64
// input, Warning `mutation.no-op` when the new value equals the current snapshot value, else a
// clean `MutationOutcome::new(..)`. Run once via `bun run`, never committed as a permanent script
// (CLAUDE.md: temporary scripts live in the ticket folder only).
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

interface Facet {
  name: string;
  dir: string;
  snapshotType: string;
}

const ROOT = "/Users/ueli/Documents/semio";
const FACETS: Facet[] = [
  { name: "en1992", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992`, snapshotType: "En1992Snapshot" },
  { name: "en1991", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991`, snapshotType: "En1991Snapshot" },
  { name: "en1996", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996`, snapshotType: "En1996Snapshot" },
  { name: "en1994", dir: `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994`, snapshotType: "En1994Snapshot" },
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
    const match = DIFF_FN_RE.exec(content);
    if (!match) {
      skipped.push(diffFile);
      continue;
    }
    const [whole, payloadType, snapshotType, diffType, fieldName, payloadField] = match;
    const fieldRustType = fieldTypes.get(fieldName);
    const isNumeric = fieldRustType === "f64";

    // pull `entity` out of the sibling mutation leaf for a nicer label
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
    converted++;
  }
}

console.log(`Converted: ${converted}`);
console.log(`Skipped (${skipped.length}):`);
for (const s of skipped) console.log(`  ${s}`);
