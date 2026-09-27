#!/usr/bin/env bun
/** 🏷️ R9: launch name prefixes of current playground variants (filter by variant substring or prefix emoji). */
const ROOT = "/Users/ueli/Documents/semio";
const { generatePlaygroundRegistry } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts`);
const { playgroundLaunchNamePrefix } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🏷️name-prefix/🟦️.ts`);
const playgrounds = generatePlaygroundRegistry(ROOT) as { variant: string }[];
const filter = process.argv[2] ?? "";
for (const playground of playgrounds) {
  const prefix = playgroundLaunchNamePrefix(playground, ROOT, playgrounds);
  if (playground.variant.includes(filter) || prefix.includes(filter)) console.log(JSON.stringify({ variant: playground.variant, prefix }));
}
