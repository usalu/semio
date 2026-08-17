// 🧪️ Lane R2 (norm plugin resume), pass 2: converts en1993's 16 `update-<X>-inputs` diff leaves —
// atomic multi-field facet updates with no addressable target (per fan-out recipe: "root-scoped
// ones with no addressable target may be message-free" for the missing-target case only) — into
// `protocol::MutationOutcome<En1993Diff>` with Fatal `mutation.invariant` per non-finite f64 field
// and Warning `mutation.no-op` when every field already equals the snapshot value.
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const DIR = `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993`;

function findFiles(dir: string, predicate: (p: string) => boolean, out: string[] = []): string[] {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    const st = statSync(full);
    if (st.isDirectory()) findFiles(full, predicate, out);
    else if (predicate(full)) out.push(full);
  }
  return out;
}

const DIFF_FN_RE =
  /pub fn diff\(payload: &(\w+), _base: &En1993Snapshot\) -> En1993Diff \{\n\s*En1993Diff \{([\s\S]*?)\.\.Default::default\(\)\s*\}\n\}/;
const FIELD_RE = /(\w+): Some\(payload\.(\w+)(\.clone\(\))?\)/g;

let converted = 0;
let skipped: string[] = [];

const diffFiles = findFiles(DIR, (p) => p.includes("🧬️mutations/") && /update-.*-inputs\/🔺️diff\/🦀️component\.rs$/.test(p));

for (const diffFile of diffFiles) {
  const content = readFileSync(diffFile, "utf8");
  if (content.includes("MutationOutcome")) continue;
  const m = DIFF_FN_RE.exec(content);
  if (!m) {
    skipped.push(diffFile);
    continue;
  }
  const [whole, payloadType, fieldsBody] = m;

  const mutationFile = diffFile.replace("🔺️diff/🦀️component.rs", "🦠️mutation/🦀️component.rs");
  const mutSrc = readFileSync(mutationFile, "utf8");
  const structMatch = mutSrc.match(new RegExp(`pub struct ${payloadType} \\{([\\s\\S]*?)\\n\\}`));
  const payloadFieldTypes = new Map<string, string>();
  if (structMatch) {
    const lineRe = /pub (\w+): ([^,]+),/g;
    let fm: RegExpExecArray | null;
    while ((fm = lineRe.exec(structMatch[1]))) {
      payloadFieldTypes.set(fm[1], fm[2].trim());
    }
  }

  const pairs: { diffField: string; payloadField: string }[] = [];
  let fm2: RegExpExecArray | null;
  FIELD_RE.lastIndex = 0;
  while ((fm2 = FIELD_RE.exec(fieldsBody))) {
    pairs.push({ diffField: fm2[1], payloadField: fm2[2] });
  }

  const lines: string[] = [];
  for (const { payloadField } of pairs) {
    const ty = payloadFieldTypes.get(payloadField);
    if (ty === "f64") {
      lines.push(`    if !payload.${payloadField}.is_finite() {`);
      lines.push(
        `        return protocol::MutationOutcome::fatal("mutation.invariant", format!("{} must be a finite number, got {}.", "${payloadField}", payload.${payloadField}), Vec::<String>::new());`
      );
      lines.push(`    }`);
    }
  }
  const eqChecks = pairs.map(({ diffField, payloadField }) => `base.${diffField} == payload.${payloadField}`).join(" && ");
  lines.push(`    if ${eqChecks} {`);
  lines.push(`        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");`);
  lines.push(`    }`);

  const fieldLiterals = pairs
    .map(({ diffField, payloadField }) => {
      const clone = payloadFieldTypes.get(payloadField) === "String" ? ".clone()" : "";
      return `${diffField}: Some(payload.${payloadField}${clone})`;
    })
    .join(", ");

  const newFn =
    `pub fn diff(payload: &${payloadType}, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {\n` +
    lines.join("\n") +
    `\n    protocol::MutationOutcome::new(En1993Diff { ${fieldLiterals}, ..Default::default() })\n}`;

  const newContent = content.slice(0, m.index) + newFn + content.slice(m.index! + whole.length);
  writeFileSync(diffFile, newContent, "utf8");
  converted++;
}

console.log(`Converted: ${converted}`);
console.log(`Skipped (${skipped.length}):`);
for (const s of skipped) console.log(`  ${s}`);
