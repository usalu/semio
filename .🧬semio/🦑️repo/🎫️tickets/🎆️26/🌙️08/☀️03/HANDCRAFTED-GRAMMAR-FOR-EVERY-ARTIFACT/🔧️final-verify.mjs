import { readFileSync, readdirSync, existsSync, writeFileSync, appendFileSync } from "fs";
import { join } from "path";

function findNamed(root, needle) {
  const entries = readdirSync(root);
  const hit =
    entries.find((e) => e === needle || e.endsWith(needle)) ||
    entries.filter((e) => e.includes(needle)).sort((a, b) => a.length - b.length)[0];
  if (!hit) throw new Error(`no ${needle} in ${root}`);
  return join(root, hit);
}
function facetFile(pl, art, facet) {
  const s = readdirSync(".").find((e) => e.length <= 4 && e.endsWith("s") && !e.includes("story"));
  const plugins = findNamed(s, "plugins");
  const plugin = findNamed(plugins, pl);
  const artifacts = findNamed(plugin, "artifacts");
  const artifact = findNamed(artifacts, art);
  const facetDir = findNamed(artifact, facet);
  return join(facetDir, "🦀️component.rs");
}

const report = [];
function ok(msg) { report.push("OK  " + msg); }
function bad(msg) { report.push("BAD " + msg); }

// dsl LanguageSpec
const dslComp = findNamed(findNamed(findNamed(findNamed(findNamed(".", "framework"), "products"), "os"), "modules"), "dsl");
const dslFacade = join(dslComp, "🦀️component.rs");
const dslTxt = readFileSync(dslFacade, "utf8");
for (const needle of [
  "protocol: Option<&'static str>",
  "protocol_path: Option<&'static str>",
  "LanguageRole",
  "Pack,",
  "Spr,",
  "passthrough_hooks",
  "protocol: parent.protocol",
]) {
  (dslTxt.includes(needle) ? ok : bad)(`dsl facade has ${needle}`);
}
ok(`passthrough_hooks count=${(dslTxt.match(/pub fn passthrough_hooks/g)||[]).length}`);

// grammar
const grammarComp = join(findNamed(dslComp, "grammar"), "🦀️component.rs");
const gTxt = readFileSync(grammarComp, "utf8");
(gTxt.includes("start == \"frame\"") ? ok : bad)("verify branches on frame");
(gTxt.includes("start == \"record\"") ? ok : bad)("verify branches on record");
(gTxt.includes("SPK magic mismatch") ? ok : bad)("SPK check for pack");
(gTxt.includes("spr bytes empty") ? ok : bad)("spr empty check");
(gTxt.includes("parse_grammar_sets_dialect_grammar_vs_protocol") ? ok : bad)("dialect unit test");
(gTxt.includes("verify_protocol_bytes_branches_pack_spk_vs_spr_record") ? ok : bad)("verify branch unit test");

const pilots = [
  ["dag", "dag"],
  ["note", "note"],
  ["fem", "2d"],
  ["fem", "3d"],
  ["writer", "writer"],
];
for (const [pl, art] of pilots) {
  for (const facet of ["dsl", "op", "diff"]) {
    const f = facetFile(pl, art, facet);
    const t = readFileSync(f, "utf8");
    (t.includes("COMPONENT_GRAMMAR_SEMIO") && t.includes("include_str!") ? ok : bad)(`${pl}/${art}/${facet} grammar include`);
    (t.includes("SemioDialect::Grammar") ? ok : bad)(`${pl}/${art}/${facet} Grammar dialect assert`);
  }
  for (const facet of ["pack", "spr"]) {
    const f = facetFile(pl, art, facet);
    const t = readFileSync(f, "utf8");
    (t.includes("COMPONENT_PROTOCOL_SEMIO") && t.includes("include_str!") ? ok : bad)(`${pl}/${art}/${facet} protocol include`);
    (t.includes("SemioDialect::Protocol") ? ok : bad)(`${pl}/${art}/${facet} Protocol dialect assert`);
    (t.includes("verify_protocol_bytes") ? ok : bad)(`${pl}/${art}/${facet} verify_protocol_bytes`);
  }
  const eng = facetFile(pl, art, "engine");
  const et = readFileSync(eng, "utf8");
  const regs = (et.match(/register_language/g) || []).length;
  const hasPack = et.includes("LanguageRole::Pack");
  const hasSpr = et.includes("LanguageRole::Spr");
  (regs >= 5 && hasPack && hasSpr ? ok : bad)(`${pl}/${art}/engine LanguageSpec regs=${regs} pack=${hasPack} spr=${hasSpr}`);
  if (et.includes("register_artifact_languages") || et.includes("register_note_languages") || et.includes("register_fem2d_languages") || et.includes("register_fem3d_languages")) {
    bad(`${pl}/${art}/engine still has duplicate register_*_languages`);
  }
}

const ticket = process.argv[2];
writeFileSync(join(ticket, "🧪e2e-pilot-wire-checklist.txt"), report.join("\n") + "\n");
console.log(report.join("\n"));
const bads = report.filter((l) => l.startsWith("BAD"));
console.log("\nBAD count", bads.length);
