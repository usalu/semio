import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";

const fw = readdirSync(".").find((x) => x.includes("framework"));
const os = join(fw, "🛍️products", readdirSync(join(fw, "🛍️products")).find((x) => x.includes("os")));
const dsl = join(os, "🔨️modules", readdirSync(join(os, "🔨️modules")).find((x) => x.includes("dsl")));

// Cargo.toml
const cargoPath = join(dsl, "⚡️implementations/🦀️rust/Cargo.toml");
let cargo = readFileSync(cargoPath, "utf8");
if (!cargo.includes("dsl_grammar")) {
  cargo = cargo.replace(
    /dsl_derive = \{ path = "[^"]+", package = "[^"]+" \}/,
    (m) =>
      m +
      `\ndsl_grammar = { path = "../../📖️grammar/⚡️implementations/🦀️rust", package = "semio-framework-os-kernel-dsl-grammar" }`
  );
  writeFileSync(cargoPath, cargo);
  console.log("cargo patched");
}

for (const rel of ["⚡️implementations/🦀️rust/📦️lib.rs", "🦀️component.rs"]) {
  const p = join(dsl, rel);
  let t = readFileSync(p, "utf8");
  if (!t.includes("pub use dsl_grammar::")) {
    // after other pub use dsl_*
    if (t.includes("pub use dsl_schema::*;")) {
      t = t.replace(
        "pub use dsl_schema::*;",
        "pub use dsl_schema::*;\npub use dsl_grammar::{parse_grammar, print_grammar, verify_protocol_bytes, GrammarFile, SemioDialect, Recognizer};"
      );
    } else {
      t = "pub use dsl_grammar::{parse_grammar, print_grammar, verify_protocol_bytes, GrammarFile, SemioDialect, Recognizer};\n" + t;
    }
    writeFileSync(p, t);
    console.log("reexport", rel);
  }
}

// rewrite pilot tests to use dsl::
import { existsSync, statSync } from "fs";
const pilots = ["🕸️dag", "🏗️fem", "🗒️note", "✒️writer"];
let n = 0;
for (const plug of pilots) {
  function walk(d) {
    for (const name of readdirSync(d)) {
      const p = join(d, name);
      if (statSync(p).isDirectory()) {
        if (name !== "target") walk(p);
      } else if (name.endsWith(".rs")) {
        let t = readFileSync(p, "utf8");
        if (!t.includes("dsl_grammar::")) continue;
        t = t.replaceAll("dsl_grammar::", "dsl::");
        writeFileSync(p, t);
        n++;
        console.log("test path", p);
      }
    }
  }
  walk(join("✏️s/🔌️plugins", plug));
}
console.log("rewrote", n);
