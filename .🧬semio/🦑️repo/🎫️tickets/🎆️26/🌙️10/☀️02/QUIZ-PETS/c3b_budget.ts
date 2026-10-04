#!/usr/bin/env bun
/** 💰️ C3b budget probe: snapshots the quiz sources whose pets parts are weighed and writes variants of them into
 * `🗑️generated/c3b/variants.json` (a map from variant name to a map from absolute path to source text), for
 * `c3b_budget.vite.ts`, which builds the site with one variant's texts loaded in place of the working tree's. Every
 * variant pins the same snapshot, so two builds differ only in what a variant takes out:
 *
 * - `asis`: the snapshot as it is;
 * - `nogroup`: without the "Play with the pets" group of the settings (`PetsPlayground`, where it is rendered and the
 *   hook that feeds it; the two key maps of its buttons and status line stay, the barrel re-exports them, and with
 *   nothing that uses them left they are shaken out of the entry);
 * - `norows`: `nogroup` and without the two pets preference rows (play, mischief) and their note;
 * - `nostrings`: `norows` and without the twelve keys per language C3 added to the i18n module.
 *
 * Nothing is written to the repository. The removals are exact text blocks; a block that is not found stops the probe,
 * so a variant is never silently the snapshot. Usage from the repository root: `bun <TK>/c3b_budget.ts [glue-moved]` —
 * `glue-moved` takes the blocks as they are once C3b moved the group into the lazy half of the glue: there `nogroup`
 * takes out the stand-in `PetsPlay` the settings render (the group itself is no longer in the entry). */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "../../../../../../..");
const react = "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react";
const preferencesPath = resolve(repoRoot, `${react}/🔨️modules/🎛️preferences/🟦️.tsx`).replaceAll("\\", "/");
const i18nPath = resolve(repoRoot, `${react}/🔨️modules/🌐️i18n/🟦️.ts`).replaceAll("\\", "/");
const moved = process.argv[2] === "glue-moved";

const without = (text: string, block: string | RegExp, what: string): string => {
  const found = typeof block === "string" ? text.includes(block) : block.test(text);
  if (!found) throw new Error(`not found in the snapshot: ${what}`);
  return text.replace(block, "");
};

const preferences = readFileSync(preferencesPath, "utf8");
const i18n = readFileSync(i18nPath, "utf8");
const keys = /^\s+pets(?:Play|Mischief|Resting|PlayWith|Hello|Trick|Pet|Toss|HelloSaid|TrickSaid|PetSaid|TossSaid): phrase\(.*\),\r?\n/gmu;
const removedKeys = i18n.match(keys)?.length ?? 0;
if (removedKeys !== 24) throw new Error(`expected the 24 keys of C3 in the i18n module, found ${removedKeys}`);

const groupBlocks: readonly (readonly [string | RegExp, string])[] = moved
  ? [[/\n      <PetsPlay text=\{text\} row=\{row\} name=\{name\} \/>/u, "the stand-in of the group's row"]]
  : [
      [/\n  const playing = usePetPlay\(\);/u, "the hook that feeds the group"],
      [/\n      \{playing === undefined \? null : \(\n[\s\S]*?\n      \)\}/u, "the group's row"],
      [/\n\/\*\* 🤹️ Play with the pets on stage without a pointer[\s\S]*?\n\}\n(?=\n\/\*\* 🧰️)/u, "PetsPlayground"],
    ];
const rowBlocks: readonly (readonly [string | RegExp, string])[] = [
  [/\n  const resting = pets === "off" \|\| pets === "still";\n  const restingId = useId\(\);/u, "resting, restingId"],
  [/\n  const allowance = cn\([^\n]*\);/u, "allowance"],
  [/\n      <label className=\{allowance\} data-pets-allow="play">[\s\S]*?\n      \) : null\}/u, "the two rows and their note"],
];

const nogroup = groupBlocks.reduce((text, [block, what]) => without(text, block, what), preferences);
const norows = rowBlocks.reduce((text, [block, what]) => without(text, block, what), nogroup);
const variants = {
  asis: { [preferencesPath]: preferences, [i18nPath]: i18n },
  nogroup: { [preferencesPath]: nogroup, [i18nPath]: i18n },
  norows: { [preferencesPath]: norows, [i18nPath]: i18n },
  nostrings: { [preferencesPath]: norows, [i18nPath]: i18n.replace(keys, "") },
};
const out = resolve(here, "🗑️generated/c3b");
mkdirSync(out, { recursive: true });
writeFileSync(resolve(out, "variants.json"), JSON.stringify(variants));
for (const [name, files] of Object.entries(variants)) process.stdout.write(`${name}: ${Object.values(files).map((text) => text.length).join(" + ")} characters\n`);
