#!/usr/bin/env bun
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = join(import.meta.dir, "../../../../../..");
type V = { emojiKebab: string; rustMod: string };
type Cfg = {
  pluginRel: string;
  artFolder: string;
  artMod: string;
  prefix: string;
  proj: string;
  applyFn: string;
  mutType: string;
  variants: V[];
};

const cfg: Cfg = JSON.parse(readFileSync(process.argv[2] ?? join(import.meta.dir, "🧪wave4-note-triads.json"), "utf8"));
const artRoot = join(repo, cfg.pluginRel, "🗿️artifacts", cfg.artFolder, "🧬️mutations");
const artPath = `crate::artifacts::${cfg.artMod}`;

for (const v of cfg.variants) {
  const base = join(artRoot, v.emojiKebab);
  mkdirSync(join(base, "🦠️mutation"), { recursive: true });
  mkdirSync(join(base, "↩️inverse"), { recursive: true });
  mkdirSync(join(base, "🔺️diff"), { recursive: true });
  writeFileSync(
    join(base, "🦠️mutation", "🦀️component.rs"),
    `use ${artPath}::${cfg.proj};\nuse ${artPath}::mutations::${cfg.mutType};\n\npub fn apply(projection: &mut ${cfg.proj}, mutation: &${cfg.mutType}) {\n    *projection = ${artPath}::mutations::${cfg.applyFn}(projection, mutation);\n}\n`,
  );
  writeFileSync(
    join(base, "↩️inverse", "🦀️component.rs"),
    `use ${artPath}::${cfg.proj};\nuse ${artPath}::mutations::${cfg.mutType};\nuse protocol::Mutation;\n\npub fn inverse(base: &${cfg.proj}, mutation: &${cfg.mutType}) -> Vec<${cfg.mutType}> {\n    <${cfg.mutType} as Mutation<${cfg.proj}>>::inverse(mutation, base)\n}\n`,
  );
  writeFileSync(join(base, "🔺️diff", "🦀️component.rs"), "//! stub diff leaf\n");
  writeFileSync(join(base, "🦠️mutation", "🟦️component.ts"), "export {};\n");
}

const glueLeaves = cfg.variants
  .map(
    (v) => `            #[path = "."]
            pub mod ${v.rustMod} {
                #[path = "../../🗿️artifacts/${cfg.artFolder}/🧬️mutations/${v.emojiKebab}/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/${cfg.artFolder}/🧬️mutations/${v.emojiKebab}/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }`,
  )
  .join("\n");

writeFileSync(join(import.meta.dir, `🧪wave4-glue-${cfg.artMod}-snippet.rs`), glueLeaves);
console.log("triads", cfg.variants.length, "snippet written");
