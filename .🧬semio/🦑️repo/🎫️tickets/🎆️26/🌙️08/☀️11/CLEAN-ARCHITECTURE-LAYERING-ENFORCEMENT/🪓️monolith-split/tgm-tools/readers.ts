import { readFileSync } from "node:fs";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { TOOL_MACHINE_VOCABULARY } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

const live = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs";
const snap = process.argv[2]!;
const oldText = readFileSync(snap, "utf8");
const newText = readModuleTreeSource(live);
const fixture = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/🫳️node-graph-gestures/🔣️.json", "utf8")) as { operationFields: Record<string, string[]> };
const fieldsOf = (guest: string, operation: string) => {
  const arm = guest.slice(guest.indexOf(`"${operation}" =>`));
  const declared = /closed\(&\[([^\]]*)\]\)/u.exec(arm)?.[1] ?? "";
  return [...declared.matchAll(/"(\w+)"/gu)].map((m) => m[1]!).filter((f) => f !== "operation");
};
const disconnect = (guest: string) => /closed\(&\["operation", "(\w+)"\]\)/u.exec(guest.slice(guest.indexOf('"disconnect" =>')))?.[1];
const vocab = (guest: string) => TOOL_MACHINE_VOCABULARY.filter((word: string) => !new RegExp(`^pub (?:trait|struct|fn) ${word}\\b`, "mu").test(guest));
let bad = 0;
const check = (label: string, a: unknown, b: unknown) => { const same = JSON.stringify(a) === JSON.stringify(b); if (!same) bad++; console.log(label, same ? "SAME" : "DIFFERENT", JSON.stringify(a), JSON.stringify(b)); };
check("disconnect field", disconnect(oldText), disconnect(newText));
check("vocabulary missing", vocab(oldText), vocab(newText));
for (const op of Object.keys(fixture.operationFields)) check(`arm ${op}`, fieldsOf(oldText, op), fieldsOf(newText, op));
console.log("DIFFERENCES", bad, "words", TOOL_MACHINE_VOCABULARY.length);
