import { readFileSync, existsSync, readdirSync } from "fs";
import { join } from "path";

const fwRoot = readdirSync(".").find((n) => n.includes("framework"));
const products = join(fwRoot, "🛍️products");
const osDir = readdirSync(products).find((n) => n.includes("os"));
const dsl = join(products, osDir, "🔨️modules", "🗣️dsl");
console.log("dsl", dsl, existsSync(dsl));

const paths = {
  dslCargo: join(dsl, "⚡️implementations/🦀️rust/Cargo.toml"),
  dslLib: join(dsl, "��️implementations/🦀️rust/📦️lib.rs"),
  grammarCargo: join(dsl, "📖️grammar/⚡️implementations/��️rust/Cargo.toml"),
  grammarLib: join(dsl, "📖️grammar/⚡️implementations/�Cargo: join(dsl, "⚡️implementations/🦀️rust/Cargo.toml"),
  dslLib: join(dsl, "⚡️implementations/🦀️rust/📦️lib.rs"),
  grammarCargo: join(dsl, "📖️grammar/⚡️implementations/🦀️rust/Cargo.toml"),
  grammarLib: join(dsl, "📖️grammar/⚡️implementations/🦀️rust/📦️lib.rs"),
};
for (const [k, p] of Object.entries(paths)) console.log(k, existsSync(p) ? "OK" : "MISSING", p);
if (existsSync(paths.dslCargo)) {
  console.log("--- dslCargo ---");
  for (const l of readFileSync(paths.dslCargo, "utf8").split("\n")) if (/grammar|dsl_/.test(l)) console.log(l);
}
if (existsSync(paths.grammarCargo)) {
  console.log("--- grammarCargo ---");
  console.log(readFileSync(paths.grammarCargo, "utf8").slice(0, 700));
}
if (existsSync(paths.dslLib)) {
  console.log("--- dslLib ---");
  readFileSync(paths.dslLib, "utf8").split("\n").forEach((l, i) => { if (/dsl_grammar|grammar::/.test(l)) console.log(i + 1 + ": " + l); });
}
if (existsSync(paths.grammarLib)) {
  const t = readFileSync(paths.grammarLib, "utf8");
  console.log("--- verify ---");
  console.log(t.slice(t.indexOf("fn verify_protocol_bytes"), t.indexOf("fn verify_protocol_bytes") + 1200));
  console.log("--- GrammarFile ---");
  console.log(t.slice(t.indexOf("pub struct GrammarFile"), t.indexOf("pub struct GrammarFile") + 500));
}
const engines = [
  "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/⚙️engine/🦀️component.rs",
  "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/⚙️engine/🦀️component.rs",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/⚙️engine/🦀️component.rs",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/⚙️engine/🦀️component.rs",
];
const three = readdirSync("✏️s/🔌️plugins/🏗️fem/🗿️artifacts").find((d) => d.includes("3d"));
if (three) engines.push(join("✏️s/🔌️plugins/🏗️fem/🗿️artifacts", three, "⚙️engine/🦀️component.rs"));
for (const e of engines) {
  if (!existsSync(e)) { console.log("MISSING", e); continue; }
  const t = readFileSync(e, "utf8");
  console.log("==== ENGINE", e, "register=", t.includes("register_language"), "====");
  console.log(t.slice(0, 1500));
}

