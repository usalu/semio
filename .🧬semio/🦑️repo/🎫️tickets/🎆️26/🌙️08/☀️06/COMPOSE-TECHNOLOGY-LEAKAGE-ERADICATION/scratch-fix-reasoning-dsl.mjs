import { readFileSync, writeFileSync } from "fs";

const dsl = "✏️s/🔌️plugins/💡️reasoning/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.reasoning.wires.dsl.semio";
let text = readFileSync(dsl, "utf8");
const before = text;
text = text.replaceAll("compose.metabolism.", "metabolism.");
text = text.replaceAll("kit-path=compose/fixture/metabolism.kit.light.compose.json", "kit-path=embedded");
if (text === before) throw new Error("dsl unchanged");
if (text.includes("compose")) {
  const hits = text.split("\n").map((l,i)=>`${i+1}:${l}`).filter(l=>/compose/.test(l));
  console.log("dsl still has compose:", hits);
} else {
  console.log("dsl clean of compose");
}
writeFileSync(dsl, text);

const rs = "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🦀️component.rs";
let rust = readFileSync(rs, "utf8");
rust = rust.replace(
  "/// 📦️ `wires_fixture.source` — provenance of the compose kit this fixture was generated from;",
  "/// 📦️ `wires_fixture.source` — provenance of the kit this fixture was generated from;",
);
writeFileSync(rs, rust);
console.log("rust docstring updated");
