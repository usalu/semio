import { readFileSync, writeFileSync, existsSync, readdirSync, statSync } from "fs";
import { join } from "path";

const ticket = process.argv[1] || ".";
const evidence = [];

function ensureRegion(src, marker, block) {
  if (src.includes(marker)) return src;
  // insert before trailing tests region or at end
  if (src.includes("//#region 🧪Tests") || src.includes("#[cfg(test)]")) {
    const idx = src.search(/#\[cfg\(test\)\]/);
    if (idx >= 0) return src.slice(0, idx) + block + "\n" + src.slice(idx);
  }
  return src.trimEnd() + "\n\n" + block + "\n";
}

function patchTextFacet(path, kind /* document|op|diff */) {
  if (!existsSync(path)) return;
  let t = readFileSync(path, "utf8");
  if (t.includes("COMPONENT_GRAMMAR_SEMIO")) {
    evidence.push({ path, status: "already" });
    return;
  }
  const constBlock = `
//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (\`dialect grammar\`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
`;
  const testBlock = `
#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = dsl_grammar::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, dsl_grammar::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}
`;
  // Avoid duplicate cfg(test) if we inject before existing — append const near top after uses
  if (!t.includes("COMPONENT_GRAMMAR_SEMIO")) {
    // after first use block
    const useEnd = t.indexOf("\n\n", t.indexOf("use "));
    if (useEnd > 0) t = t.slice(0, useEnd) + "\n" + constBlock + t.slice(useEnd);
    else t = constBlock + t;
  }
  if (!t.includes("semio_grammar_conformance")) {
    t = t.trimEnd() + "\n" + testBlock + "\n";
  }
  writeFileSync(path, t);
  evidence.push({ path, status: "wired-grammar", kind });
}

function patchBinaryFacet(path, kind /* pack|spr */) {
  if (!existsSync(path)) return;
  let t = readFileSync(path, "utf8");
  if (t.includes("COMPONENT_PROTOCOL_SEMIO")) {
    evidence.push({ path, status: "already" });
    return;
  }
  const constBlock = `
//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (\`dialect protocol\`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol
`;
  const testBlock = `
#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[test]
    fn component_protocol_semio_is_protocol_dialect() {
        let g = dsl_grammar::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, dsl_grammar::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }
}
`;
  const useEnd = t.indexOf("\n\n", t.indexOf("use "));
  if (useEnd > 0) t = t.slice(0, useEnd) + "\n" + constBlock + t.slice(useEnd);
  else t = constBlock + t;
  if (!t.includes("semio_protocol_conformance")) t = t.trimEnd() + "\n" + testBlock + "\n";
  writeFileSync(path, t);
  evidence.push({ path, status: "wired-protocol", kind });
}

const pilots = [
  { plugin: "🕸️dag", arts: ["🕸️dag"] },
  { plugin: "🗒️note", arts: ["🗒️note"] },
  { plugin: "✒️writer", arts: ["✒️writer"] },
];

// fem artifacts
{
  const femArts = join("✏️s/🔌️plugins/🏗️fem/🗿️artifacts");
  if (existsSync(femArts)) {
    const arts = readdirSync(femArts).filter((n) => statSync(join(femArts, n)).isDirectory());
    pilots.push({ plugin: "🏗️fem", arts });
  }
}

for (const { plugin, arts } of pilots) {
  for (const art of arts) {
    const base = join("✏️s/🔌️plugins", plugin, "🗿️artifacts", art);
    if (!existsSync(base)) continue;
    for (const facet of ["🗣️dsl", "🔧️op", "🔺️diff"]) {
      patchTextFacet(join(base, facet, "🦀️component.rs"), facet);
    }
    for (const facet of ["🎒️pack", "📡️spr"]) {
      patchBinaryFacet(join(base, facet, "🦀️component.rs"), facet);
    }
  }
}

writeFileSync(join(ticket, "🧪e2e-pilot-include-str.md"), `# Pilot include_str wiring

Grammars (\`dialect grammar\`) on text facets; protocols (\`dialect protocol\`) on binary facets.

## Wired

${evidence.map((e) => `- \`${e.status}\` ${e.kind || ""} \`${e.path}\``).join("\n")}

## Counts
- entries: ${evidence.length}
- grammar: ${evidence.filter((e) => String(e.status).includes("grammar")).length}
- protocol: ${evidence.filter((e) => String(e.status).includes("protocol")).length}
`);
writeFileSync(join(ticket, "🧪e2e-pilot-include-str.json"), JSON.stringify(evidence, null, 2));
console.log(JSON.stringify({ count: evidence.length, evidence }, null, 2));
