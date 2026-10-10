#!/usr/bin/env bun
/** 🏗️ Wave W2 `w2-wp19-frame`: gives the extra cases of the WP the folder emoji no sibling case uses (`bun r3-f1-check-names.ts` reports a case whose emoji a sibling already has). Renames the fixture folder and the test folder of
 * each case and the paths that name them (the mounts of the root, the fixture includes of the case test). Run once. */
import { existsSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../../..");
const subset = join(root, "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any");
const mutations = join(subset, "🧬️schema/🧬️mutations");
const fixtures = join(subset, "🧫️fixtures/🧬️mutations");
const rootFile = join(subset, "../../../../🦀️.rs");
const em = (code: number) => String.fromCodePoint(code) + "️";

const renames: [string, string, string, number][] = [
  ["➖️create-beam", "adds-an-arc-beam", em(0x2705), 0x1f4aa],
  ["➖️create-beam", "flat-arc", em(0x1f6ab), 0x1f9f1],
  ["🏛️create-column", "adds-a-leaning-column", em(0x2705), 0x1f4aa],
  ["🏛️create-column", "tilt-too-steep", em(0x1f6ab), 0x1f9f1],
  ["🥖️split-beam", "splits-an-arc-beam", em(0x2705), 0x1f4aa],
  ["🦖️delete-curtain-wall", "cascades-its-overrides", em(0x2705), 0x1f4aa],
];

let text = readFileSync(rootFile, "utf8");
for (const [leaf, name, old, fresh] of renames) {
  const from = old + name;
  const to = em(fresh) + name;
  for (const base of [join(fixtures, leaf), join(mutations, leaf, "🧪️tests")]) {
    if (existsSync(join(base, from))) renameSync(join(base, from), join(base, to));
  }
  text = text.replaceAll(`${leaf}/🧪️tests/${from}/`, `${leaf}/🧪️tests/${to}/`);
  const test = join(mutations, leaf, "🧪️tests", to, "🦀️.rs");
  if (existsSync(test)) {
    const source = readFileSync(test, "utf8");
    writeFileSync(test, source.replaceAll(`${leaf}/${from}/`, `${leaf}/${to}/`).replaceAll(`/${from}/`, `/${to}/`));
  }
}
writeFileSync(rootFile, text);
console.log("cases renamed");
