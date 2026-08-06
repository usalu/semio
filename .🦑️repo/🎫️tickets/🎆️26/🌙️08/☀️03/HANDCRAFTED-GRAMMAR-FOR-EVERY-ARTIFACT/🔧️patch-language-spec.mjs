import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";

const fw = readdirSync(".").find((x) => x.includes("framework"));
const os = join(fw, "🛍️products", readdirSync(join(fw, "🛍️products")).find((x) => x.includes("os")));
const dsl = join(os, "🔨️modules", readdirSync(join(os, "🔨️modules")).find((x) => x.includes("dsl")));
const paths = [
  join(dsl, "🦀️component.rs"),
  join(dsl, "⚡️implementations/🦀️rust/📦️lib.rs"),
];

for (const p of paths) {
  let t = readFileSync(p, "utf8");
  const before = t;
  if (!/\bPack,\n\s*Spr,/.test(t)) {
    t = t.replace(
      /pub enum LanguageRole \{\n    Document,\n    Config,\n    Ops,\n    Embedded,\n    Diff,\n\}/,
      "pub enum LanguageRole {\n    Document,\n    Config,\n    Ops,\n    Embedded,\n    Diff,\n    Pack,\n    Spr,\n}"
    );
  }
  if (!t.includes("pub protocol:")) {
    t = t.replace(
      /pub grammar: Option<&'static str>,\n    pub grammar_path: Option<&'static str>,\n    pub hooks: IdiomHooks,\n\}/,
      "pub grammar: Option<&'static str>,\n    pub grammar_path: Option<&'static str>,\n    pub protocol: Option<&'static str>,\n    pub protocol_path: Option<&'static str>,\n    pub hooks: IdiomHooks,\n}"
    );
  }
  t = t.replace(
    /Self \{ id, extension: None, role, grammar: parent\.grammar, grammar_path: parent\.grammar_path, hooks: parent\.hooks \}/g,
    "Self { id, extension: None, role, grammar: parent.grammar, grammar_path: parent.grammar_path, protocol: parent.protocol, protocol_path: parent.protocol_path, hooks: parent.hooks }"
  );
  t = t.replace(/grammar_path: ([^\n]+),\n(\s*)hooks:/g, (m, gp, sp) => {
    if (m.includes("protocol")) return m;
    return `grammar_path: ${gp},\n${sp}protocol: None,\n${sp}protocol_path: None,\n${sp}hooks:`;
  });
  if (t !== before) {
    writeFileSync(p, t);
    console.log("updated", p, "delta", t.length - before.length);
  } else console.log("no change", p);
}

// writer LanguageSpec
const writer = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/⚙️engine/🦀️component.rs";
let w = readFileSync(writer, "utf8");
const wb = w;
w = w.replace(/grammar_path: ([^\n]+),\n(\s*)hooks:/g, (m, gp, sp) => {
  if (m.includes("protocol")) return m;
  return `grammar_path: ${gp},\n${sp}protocol: None,\n${sp}protocol_path: None,\n${sp}hooks:`;
});
if (w !== wb) {
  writeFileSync(writer, w);
  console.log("updated writer");
} else console.log("writer ok or already patched");
