// 📖️ c4p15 reader check: the module-tree text the re-pointed hub readers return versus the pre-edit text, as a stripped non-blank line multiset (wiring, child headers, widening prefixes and impl wrappers aside).
import { readFileSync } from "node:fs";
import { readTrustedCatalogSource } from "/Users/ueli/Documents/semio/🌎️hub/🧪️testing/📖️source-readers/🟦️.ts";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio", TOOLS = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p15-tools";
const bag = (text: string) => { const m = new Map<string, number>(); for (const raw of text.split("\n")) { const l = raw.trim().replace(/^pub\(super\) /u, "").replace(/\(\s*"\.\.\//gu, '("'); if (l) m.set(l, (m.get(l) ?? 0) + 1); } return m; };
const wiring = (l: string) => /^(#\[path = ".*"\]|mod \w+;|(pub(\(crate\))? )?use \w+::\*;|use super::\*;|\/\/! .*|include!\(".*"\);|\/\/ .*|impl InferenceJobLedgerV1 \{|\}|#\[cfg\(any\(test, feature = "integration-fixtures"\)\)\])$/u.test(l);
function compare(label: string, oldText: string, newText: string) {
  const a = bag(oldText), b = bag(newText);
  const lost: string[] = [], gained: string[] = [];
  for (const [k, n] of a) if ((b.get(k) ?? 0) < n) lost.push(`${n - (b.get(k) ?? 0)}x ${k.slice(0, 90)}`);
  for (const [k, n] of b) if ((a.get(k) ?? 0) < n && !wiring(k)) gained.push(`${n - (a.get(k) ?? 0)}x ${k.slice(0, 90)}`);
  console.log(`${label}: old lines ${[...a.values()].reduce((s, n) => s + n, 0)} new lines ${[...b.values()].reduce((s, n) => s + n, 0)} lost ${lost.length} gained(non-wiring) ${gained.length}`);
  for (const l of lost.slice(0, 8)) console.log("   LOST", l);
  for (const l of gained.slice(0, 8)) console.log("   GAINED", l);
}
const orig = (n: number) => readFileSync(`${TOOLS}/orig/${n}.rs`, "utf8");
const tc = ROOT + "/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog";
compare("readTrustedCatalogSource", [orig(5), readFileSync(tc + "/📜️contract/🦀️.rs", "utf8"), orig(7)].join("\n"), readTrustedCatalogSource(ROOT));
compare("ledger sqlite tree", orig(2), readModuleTreeSource(ROOT + "/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs"));
compare("trusted-catalog tree", orig(5), readModuleTreeSource(tc + "/🦀️.rs"));
compare("descriptor tree", orig(7), readModuleTreeSource(tc + "/🧾️descriptor/🦀️.rs"));
compare("directory unit tree (tests)", orig(3), readModuleTreeSource(ROOT + "/🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs", { tests: true }));
compare("trusted-catalog unit tree (tests)", orig(6), readModuleTreeSource(tc + "/🧪️tests/🔬️unit/🦀️.rs", { tests: true }));
compare("runtime unit tree (tests)", orig(1), readModuleTreeSource(ROOT + "/🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs", { tests: true }));
