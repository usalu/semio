// 🔧 One-off scripted pass for lane 3-A (norm/{din16798,en1998,en1999,en1997}) — converts every
// `change-<field>` 🔺️diff/🦠️mutation triad from bare `XDiff` returns to `protocol::MutationOutcome<XDiff>`
// with real Fatal `mutation.invariant` (non-finite / out-of-percent-range) and Warning `mutation.no-op`
// (unchanged value) messages, per the frozen verb-family table (`change|set|update`).
// Lives inside the ticket folder per the fan-out recipe's "one-off scripted pass" allowance.

import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

const root = "/Users/ueli/Documents/semio";
const facets = ["📗️din16798", "📘️en1998", "📘️en1999", "📘️en1997"];

function walk(dir: string, out: string[]) {
  for (const entry of readdirSync(dir)) {
    const p = join(dir, entry);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else out.push(p);
  }
}

type FieldKind = "f64" | "u32" | "u8" | "String" | "bool" | "opaque";

function classify(t: string): FieldKind {
  if (t === "f64") return "f64";
  if (t === "u32") return "u32";
  if (t === "u8") return "u8";
  if (t === "String") return "String";
  if (t === "bool") return "bool";
  return "opaque";
}

function capitalize(s: string): string {
  return s.length ? s[0].toUpperCase() + s.slice(1) : s;
}

const DRY = process.env.DRY === "1";
let converted = 0;
let errors: string[] = [];
let printed = 0;

for (const facet of facets) {
  const facetDir = join(root, "✏️s/🔌️plugins/📕️norm/🗿️artifacts", facet);
  const files: string[] = [];
  walk(facetDir, files);
  const mutationFiles = files.filter((f) => f.endsWith("🦠️mutation/🦀️component.rs"));

  for (const mutFile of mutationFiles) {
    const diffFile = mutFile.replace("🦠️mutation/🦀️component.rs", "🔺️diff/🦀️component.rs");
    const mutSrc = readFileSync(mutFile, "utf-8");
    const diffSrc = readFileSync(diffFile, "utf-8");

    // Extract payload struct name + single field name/type.
    const structMatch = mutSrc.match(/pub struct (\w+) \{\s*pub (\w+): ([\w<>:]+),\s*\}/);
    if (!structMatch) {
      errors.push(`NO STRUCT MATCH: ${mutFile}`);
      continue;
    }
    const [, payloadName, fieldFull, fieldType] = structMatch;
    if (!fieldFull.startsWith("new_")) {
      errors.push(`FIELD DOES NOT START WITH new_: ${mutFile}`);
      continue;
    }
    const fieldName = fieldFull.slice(4);

    // Extract snapshot/diff type names + description from the diff leaf's existing fn signature.
    const diffFnMatch = diffSrc.match(
      /pub fn diff\(payload: &(\w+), _?base: &(\w+)\) -> (\w+) \{([\s\S]*?)\n\}/
    );
    if (!diffFnMatch) {
      errors.push(`NO DIFF FN MATCH: ${diffFile}`);
      continue;
    }
    const [fullFnMatch, , snapshotType, diffType] = diffFnMatch;

    // Extract human description from the mutation leaf's first doc line's trailing "(...)".
    const firstLine = mutSrc.split("\n")[0];
    const descMatch = firstLine.match(/\(([^()]+)\)\.?\s*$/);
    if (!descMatch) {
      errors.push(`NO DESC MATCH: ${mutFile}`);
      continue;
    }
    const desc = capitalize(descMatch[1]);

    const kind = classify(fieldType);
    const isPercent = /percent/i.test(fieldName);

    // --- Build the new diff() body ---
    const lines: string[] = [];
    if (kind === "f64") {
      lines.push(`    if !payload.${fieldFull}.is_finite() {`);
      lines.push(
        `        return protocol::MutationOutcome::fatal("mutation.invariant", format!("${desc} must be a finite number, got {}.", payload.${fieldFull}), Vec::<String>::new());`
      );
      lines.push(`    }`);
      if (isPercent) {
        lines.push(`    if payload.${fieldFull} < 0.0 || payload.${fieldFull} > 100.0 {`);
        lines.push(
          `        return protocol::MutationOutcome::fatal("mutation.invariant", format!("${desc} must be between 0 and 100 percent, got {}.", payload.${fieldFull}), Vec::<String>::new());`
        );
        lines.push(`    }`);
      }
      lines.push(`    if base.${fieldName} == payload.${fieldFull} {`);
      lines.push(
        `        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("${desc} is already {}.", payload.${fieldFull}));`
      );
      lines.push(`    }`);
    } else if (kind === "u32" || kind === "u8") {
      lines.push(`    if base.${fieldName} == payload.${fieldFull} {`);
      lines.push(
        `        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("${desc} is already {}.", payload.${fieldFull}));`
      );
      lines.push(`    }`);
    } else if (kind === "String") {
      lines.push(`    if base.${fieldName} == payload.${fieldFull} {`);
      lines.push(
        `        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("${desc} is already \\"{}\\".", payload.${fieldFull}));`
      );
      lines.push(`    }`);
    } else if (kind === "bool") {
      lines.push(`    if base.${fieldName} == payload.${fieldFull} {`);
      lines.push(
        `        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("${desc} is already {}.", payload.${fieldFull}));`
      );
      lines.push(`    }`);
    } else {
      // opaque (AnnexChoice etc.) — Debug-format, no-op only.
      lines.push(`    if base.${fieldName} == payload.${fieldFull} {`);
      lines.push(
        `        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("${desc} is already {:?}.", payload.${fieldFull}));`
      );
      lines.push(`    }`);
    }
    lines.push(
      `    protocol::MutationOutcome::new(${diffType} { ${fieldName}: Some(payload.${fieldFull}.clone()), ..Default::default() })`
    );

    const newFnBody = `pub fn diff(payload: &${payloadName}, base: &${snapshotType}) -> protocol::MutationOutcome<${diffType}> {\n${lines.join(
      "\n"
    )}\n}`;

    const newDiffSrc = diffSrc.replace(fullFnMatch, newFnBody);
    if (newDiffSrc === diffSrc) {
      errors.push(`DIFF REPLACE NO-OP: ${diffFile}`);
      continue;
    }

    // --- Mutation leaf: only the fn diff return-type annotation changes. ---
    const mutFnRe = new RegExp(
      `fn diff\\(&self, base: &${snapshotType}\\) -> ${diffType} \\{`
    );
    if (!mutFnRe.test(mutSrc)) {
      errors.push(`NO MUTATION FN SIG MATCH: ${mutFile}`);
      continue;
    }
    const newMutSrc = mutSrc.replace(
      mutFnRe,
      `fn diff(&self, base: &${snapshotType}) -> protocol::MutationOutcome<${diffType}> {`
    );

    if (DRY) {
      if (printed < 3) {
        console.log("=====", diffFile);
        console.log(newDiffSrc);
        console.log("-----", mutFile);
        console.log(newMutSrc);
        printed++;
      }
    } else {
      writeFileSync(diffFile, newDiffSrc, "utf-8");
      writeFileSync(mutFile, newMutSrc, "utf-8");
    }
    converted++;
  }
}

console.log(`Converted: ${converted}`);
console.log(`Errors: ${errors.length}`);
for (const e of errors) console.log(e);
