import { writeFileSync, readFileSync, readdirSync, statSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
const ticket = dirname(fileURLToPath(import.meta.url));
const hits = [];
for (const plug of ["🕸️dag", "🏗️fem", "🗒️note", "✒️writer"]) {
  function walk(d) {
    for (const n of readdirSync(d)) {
      const p = join(d, n);
      if (statSync(p).isDirectory()) {
        if (n !== "target") walk(p);
      } else if (n.endsWith(".rs") && /COMPONENT_(GRAMMAR|PROTOCOL)_SEMIO/.test(readFileSync(p, "utf8"))) {
        hits.push(p);
      }
    }
  }
  walk(join("✏️s/🔌️plugins", plug));
}
writeFileSync(
  join(ticket, "🧪e2e-pilot-include-str.md"),
  [
    "# Pilot include_str wiring (parent follow-up)",
    "",
    "Previous [Pilot include_str registration] agent exited with no edits. Parent landed wiring.",
    "",
    "## Rule",
    "- COMPONENT_GRAMMAR_SEMIO on text facets (dsl/op/diff)",
    "- COMPONENT_PROTOCOL_SEMIO on binary facets (pack/spr)",
    "",
    `## Files (${hits.length})`,
    ...hits.map((h) => `- \`${h}\``),
    "",
  ].join("\n")
);
writeFileSync(
  join(ticket, "🧪e2e-language-protocol-wire.md"),
  [
    "# LanguageSpec protocol fields",
    "",
    "- Added `protocol` / `protocol_path`",
    "- Added `LanguageRole::Pack` and `LanguageRole::Spr`",
    "- Updated `derived` + LanguageSpec literals (dsl + writer)",
    "",
  ].join("\n")
);
console.log("wrote evidence", hits.length);
