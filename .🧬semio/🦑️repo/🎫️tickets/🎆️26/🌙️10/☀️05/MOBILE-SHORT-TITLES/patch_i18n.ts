#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../..");
const path = join(root, "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts");
let text = readFileSync(path, "utf8");
const pairs: [string, string][] = [
  ['      title: phrase("Einstellungen"),\n      language:', '      title: phrase("Einstellungen"),\n      titleShort: phrase("Prefs"),\n      language:'],
  ['      howItWorks: phrase("So funktioniert’s"),\n      readMore:', '      howItWorks: phrase("So funktioniert’s"),\n      howItWorksShort: phrase("So"),\n      readMore:'],
  ['      title: phrase("Rangliste"),\n      caption: phrase("Lernende mit einem abgegebenen Quiz', '      title: phrase("Rangliste"),\n      titleShort: phrase("Liste"),\n      caption: phrase("Lernende mit einem abgegebenen Quiz'],
];
for (const [needle, replacement] of pairs) {
  if (!text.includes(replacement)) {
    if (!text.includes(needle)) throw new Error(`missing: ${needle.slice(0, 40)}`);
    text = text.replace(needle, replacement);
  }
}
writeFileSync(path, text);
console.log("i18n patched");
