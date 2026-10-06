#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const path = join(import.meta.dirname, "../../../../../../..", "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts");
let text = readFileSync(path, "utf8");
text = text.replace('title: phrase("Einstellungen"),\n      titleShort: phrase("Prefs"),', 'title: phrase("Einstellungen"),\n      titleShort: phrase("Einst."),');
writeFileSync(path, text);
console.log("de prefs short fixed");
