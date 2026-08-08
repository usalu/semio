#!/usr/bin/env bun
/**
 * Generates 🧬️mutations triad stubs + glue block for a plugin artifact.
 * Usage: bun 🧪wave4-migrate-plugin.mts <repo-relative-plugin-artifact-dir> <ArtifactPrefix> <ProjectionType> <variants-json-file>
 */
import { mkdirSync, writeFileSync, readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../..");
const artifactDir = join(repoRoot, process.argv[2] ?? "");
const prefix = process.argv[3] ?? "";
const projection = process.argv[4] ?? "";
const variantsPath = process.argv[5] ?? "";

if (!prefix || !existsSync(artifactDir)) {
  console.error("usage: wave4-migrate-plugin.mts <artifact-dir> <Prefix> <Projection> <variants.json>");
  process.exit(1);
}

type Variant = { emoji: string; kebab: string; mod: string; variant: string };
const variants: Variant[] = variantsPath ? JSON.parse(readFileSync(join(repoRoot, variantsPath), "utf8")) : [];

const mutRoot = join(artifactDir, "🧬️mutations");
mkdirSync(mutRoot, { recursive: true });

const diffMod = `${prefix.toLowerCase()}::diff::${prefix}Diff`;
const art = `crate::artifacts::${prefix.toLowerCase()}`;

for (const v of variants) {
  const base = join(mutRoot, `${v.emoji}${v.kebab}`);
  for (const leaf of ["🦠️mutation", "🔺️diff", "↩️inverse"]) {
    mkdirSync(join(base, leaf), { recursive: true });
  }
  writeFileSync(
    join(base, "🦠️mutation", "🦀️component.rs"),
    `//! ${v.emoji} ${prefix} mutation — \`${v.variant}\` apply stub.
use ${art}::${projection};
use ${art}::mutations::${prefix}Mutation;

pub fn apply(projection: &mut ${projection}, mutation: &${prefix}Mutation) {
    ${art}::mutations::apply_${prefix.toLowerCase()}_mutation(projection, mutation);
}
`,
  );
  writeFileSync(
    join(base, "🔺️diff", "🦀️component.rs"),
    `use ${art}::diff::${prefix}Diff;
use ${art}::mutations::${prefix}Mutation;
use ${art}::${projection};
use protocol::MutationDiff;

pub fn diff_for(mutation: ${prefix}Mutation) -> ${prefix}Diff {
    <${prefix}Mutation as protocol::Mutation<${projection}>>::diff(&mutation, &Default::default())
}
`,
  );
  writeFileSync(
    join(base, "↩️inverse", "🦀️component.rs"),
    `use ${art}::${projection};
use ${art}::mutations::${prefix}Mutation;

pub fn inverse(base: &${projection}, mutation: &${prefix}Mutation) -> Vec<${prefix}Mutation> {
    <${prefix}Mutation as protocol::Mutation<${projection}>>::inverse(mutation, base)
}
`,
  );
  writeFileSync(join(base, "🦠️mutation", "🟦️component.ts"), `export {};\n`);
}

writeFileSync(join(mutRoot, "🟦️component.ts"), `/** 🧩 ${prefix.toLowerCase()} 🧬️mutations WASM facade. */\nexport {};\n`);

const glueLines = variants
  .map(
    (v) => `            #[path = "."]
            pub mod ${v.mod} {
                #[path = "../../🗿️artifacts/${process.argv[2]!.split("/").slice(-2, -1)[0] ?? ""}/${process.argv[2]!.includes("🌿️vcs") ? "🌿️vcs" : "?"}/🧬️mutations/${v.emoji}${v.kebab}/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
            }`,
  )
  .join("\n");

console.log("created", variants.length, "triads under", mutRoot);
console.log("glue snippet length", glueLines.length);
