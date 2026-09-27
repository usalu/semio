#!/usr/bin/env bun
/** 🎮️ R9: current playground variants matching an optional filter. */
const ROOT = "/Users/ueli/Documents/semio";
const { generatePlaygroundRegistry } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts`);
const filter = process.argv[2] ?? "";
console.log(JSON.stringify((generatePlaygroundRegistry(ROOT) as { variant: string }[]).map((entry) => entry.variant).filter((variant) => variant.includes(filter))));
