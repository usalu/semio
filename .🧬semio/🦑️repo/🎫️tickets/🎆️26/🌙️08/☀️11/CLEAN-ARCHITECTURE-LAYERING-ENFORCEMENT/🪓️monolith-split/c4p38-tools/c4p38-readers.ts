// 📖️ c4p38 reader check: the text the re-pointed hub readers return versus the pre-edit text, as a stripped non-blank line multiset (wiring, child headers, widening prefixes and the include re-base aside).
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readHubBinTestSource } from "/Users/ueli/Documents/semio/🌎️hub/🧪️testing/📖️source-readers/🟦️.ts";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio", TOOLS = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p38-tools";
const bag = (text: string) => { const m = new Map<string, number>(); for (const raw of text.split("\n")) { const l = raw.trim().replace(/\bpub\(super\) /gu, "").replace(/\(\s*"\.\.\//gu, '("'); if (l) m.set(l, (m.get(l) ?? 0) + 1); } return m; };
const wiring = (l: string) => /^(#\[path = ".*"\]|mod \w+;|(pub(\(crate\))? )?use \w+::\*;|use super::\*;|\/\/! .*|include!\(".*"\);|\/\/ .*)$/u.test(l);
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
const base = join(ROOT, "🌎️hub/🧪️tests/🔬️bin-unit");
const snapshotOf = new Map<string, string>([["🧰️testkit/🦀️.rs", orig(7)], ["🧭️open-plan/🔬️unit/🦀️.rs", orig(6)], ["🔐️session/🔬️unit/🦀️.rs", orig(5)], ["🎲️hostile-input/🔬️unit/🦀️.rs", orig(4)]]);
const files = ["🧰️testkit/🦀️.rs", "🚀️boot/🔬️unit/🦀️.rs", "🧭️open-plan/🔬️unit/🦀️.rs", "📇️directory/🔬️unit/🦀️.rs", "👥️presence/🔬️unit/🦀️.rs", "🔐️session/🔬️unit/🦀️.rs", "🎲️hostile-input/🔬️unit/🦀️.rs"];
compare("readHubBinTestSource", files.map((file) => snapshotOf.get(file) ?? readFileSync(join(base, file), "utf8")).join("\n"), readHubBinTestSource(ROOT));
compare("chunk-cas tree (production)", orig(1), readModuleTreeSource(ROOT + "/🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs"));
compare("local-bootstrap tree (production)", orig(2), readModuleTreeSource(ROOT + "/🌎️hub/🚀️local-bootstrap/🦀️.rs"));
compare("testkit tree (production)", orig(7), readModuleTreeSource(ROOT + "/🌎️hub/🧪️tests/🔬️bin-unit/🧰️testkit/🦀️.rs"));
const spaceRoot = ROOT + "/🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space";
compare("space children + root (interactive-job oracle text)", orig(3), ["🦀️.rs", ...["🧰️document-helpers", "🕹️space-command", "🏠️space-app", "📜️space-manifest"].map((c) => c + "/🦀️.rs")].map((f) => readFileSync(join(spaceRoot, f), "utf8")).join("\n"));
for (const [n, rel] of [[4, "🎲️hostile-input"], [5, "🔐️session"], [6, "🧭️open-plan"]] as const) compare(`${rel} unit tree (tests)`, orig(n), readModuleTreeSource(`${base}/${rel}/🔬️unit/🦀️.rs`, { tests: true }));
