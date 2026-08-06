/**
 * @emoji 🔗 Rewires norm/cad/draw/note artifact `#[dsl(id)]` and `include_str!` example paths.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = join(import.meta.dir, "../../../../../../");

const normDsl = {
  en1990: ["📕️high-consequence-office", "EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT"],
  en1991: ["📕️retail-hydrocarbon-fire", "EN1991_RETAIL_HYDROCARBON_FIRE_EXAMPLE_TEXT"],
  en1992: ["📕️liquid-retaining-fem-anchor", "EN1992_LIQUID_RETAINING_FEM_ANCHOR_EXAMPLE_TEXT"],
  en1993: ["📕️high-strength-connection", "EN1993_HIGH_STRENGTH_CONNECTION_EXAMPLE_TEXT"],
  en1994: ["📕️composite-bridge-girder", "EN1994_COMPOSITE_BRIDGE_GIRDER_EXAMPLE_TEXT"],
  en1995: ["📕️glulam-footbridge", "EN1995_GLULAM_FOOTBRIDGE_EXAMPLE_TEXT"],
  en1996: ["📕️loadbearing-wall", "EN1996_LOADBEARING_WALL_EXAMPLE_TEXT"],
  en1997: ["📕️default", "EN1997_DEFAULT_EXAMPLE_TEXT"],
  en1998: ["📕️seismic-rc-frame", "EN1998_SEISMIC_RC_FRAME_EXAMPLE_TEXT"],
  en1999: ["📕️aluminium-roof-purlin", "EN1999_ALUMINIUM_ROOF_PURLIN_EXAMPLE_TEXT"],
  iso16757: ["📕️default", "ISO16757_DEFAULT_EXAMPLE_TEXT"],
  din18599: ["♻️default", "DEFAULT_EXAMPLE_TEXT"],
  din16798: ["♻️default", "DEFAULT_EXAMPLE_TEXT"],
  din4108: ["♻️default", "DEFAULT_EXAMPLE_TEXT"],
  vdi3805: ["♻️default", "REFERENCE_EXAMPLE_TEXT"],
};

const normRoot = join(REPO, "✏️s/🔌️plugins/📕️norm/🗿️artifacts");

for (const [slug, [leaf, constName]] of Object.entries(normDsl)) {
  const artifactDirs = {
    en1990: "📘️en1990",
    en1991: "📘️en1991",
    en1992: "📘️en1992",
    en1993: "📘️en1993",
    en1994: "📘️en1994",
    en1995: "📘️en1995",
    en1996: "📘️en1996",
    en1997: "📘️en1997",
    en1998: "📘️en1998",
    en1999: "📘️en1999",
    iso16757: "📓️iso16757",
    din18599: "📙️din18599",
    din16798: "📗️din16798",
    din4108: "📕️din4108",
    vdi3805: "📔️vdi3805",
  };
  const dir = join(normRoot, artifactDirs[slug]);
  const compPath = join(dir, "🦀️component.rs");
  let comp = readFileSync(compPath, "utf8");
  comp = comp.replaceAll(`#[dsl(extension = "${slug}",`, `#[dsl(id = "norm.${slug}",`);
  writeFileSync(compPath, comp);

  const dslPath = join(dir, "🗣️dsl/🦀️component.rs");
  let dsl = readFileSync(dslPath, "utf8");
  const inc = `include_str!("../../📚️examples/${leaf}/🗣️dsls/${leaf}/🧬️component.norm.${slug}.dsl.semio")`;
  if (dsl.includes("include_str!")) {
    dsl = dsl.replace(/include_str!\([^)]+\)/, inc);
  } else {
    dsl = dsl.replace(
      /use crate::artifacts::[^;]+;\n\n/,
      (m) =>
        `${m}/// 📜️ Bundled default example document (\`.semio\` envelope + DSL body).\npub const ${constName}: &str = ${inc};\n\n`,
    );
    dsl = dsl.replace(
      /fn document_dsl_round_trips\(\) \{\n        store::test_support::assert_dsl_round_trip\(&Document::default\(\)\);\n    \}/,
      `fn document_dsl_round_trips() {\n        store::test_support::assert_dsl_round_trip(&Document::default());\n    }\n\n    #[test]\n    fn bundled_example_fixture_parses_and_round_trips() {\n        let document = parse_dsl(${constName}).expect("parse bundled example");\n        store::test_support::assert_dsl_round_trip(&document);\n    }`,
    );
    if (slug === "vdi3805") {
      dsl = dsl.replace(
        /fn document_dsl_round_trips_the_reference_fixture\(\) \{\n        store::test_support::assert_dsl_round_trip\(&crate::artifacts::vdi3805::reference_fixture\(\)\);\n    \}/,
        `fn document_dsl_round_trips_the_reference_fixture() {\n        store::test_support::assert_dsl_round_trip(&crate::artifacts::vdi3805::reference_fixture());\n    }\n\n    #[test]\n    fn bundled_example_fixture_parses_and_round_trips() {\n        let document = parse_dsl(REFERENCE_EXAMPLE_TEXT).expect("parse bundled example");\n        store::test_support::assert_dsl_round_trip(&document);\n    }`,
      );
    }
  }
  writeFileSync(dslPath, dsl);
  console.log(`[rewire] norm.${slug}`);
}

const cadComp = join(REPO, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs");
let cad = readFileSync(cadComp, "utf8");
cad = cad.replace(`#[dsl(extension = "cad",`, `#[dsl(id = "cad.cad",`);
writeFileSync(cadComp, cad);

const cadDsl = join(REPO, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🗣️dsl/🦀️component.rs");
let cadD = readFileSync(cadDsl, "utf8");
cadD = cadD.replace(
  /include_str!\([^)]+\)/,
  `include_str!("../../📚️examples/♻️default/🗣️dsls/♻️default/🧬️component.cad.cad.dsl.semio")`,
);
writeFileSync(cadDsl, cadD);

const drawComp = join(REPO, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🦀️component.rs");
let draw = readFileSync(drawComp, "utf8");
draw = draw.replace(`#[dsl(extension = "draw",`, `#[dsl(id = "draw.draw",`);
writeFileSync(drawComp, draw);

const drawDsl = join(REPO, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🗣️dsl/🦀️component.rs");
let drawD = readFileSync(drawDsl, "utf8");
drawD = drawD.replace(
  /include_str!\([^)]+\)/,
  `include_str!("../../📚️examples/♻️semio/🗣️dsls/♻️semio/🧬️component.draw.draw.dsl.semio")`,
);
writeFileSync(drawDsl, drawD);

const noteComp = join(REPO, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️component.rs");
let note = readFileSync(noteComp, "utf8");
note = note.replace(`#[dsl(extension = "note",`, `#[dsl(id = "note.note",`);
writeFileSync(noteComp, note);

const noteDsl = join(REPO, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🗣️dsl/🦀️component.rs");
let noteD = readFileSync(noteDsl, "utf8");
noteD = noteD.replace(
  /include_str!\([^)]+\)/,
  `include_str!("../../📚️examples/♻️semio/🗣️dsls/♻️semio/🧬️component.note.note.dsl.semio")`,
);
writeFileSync(noteDsl, noteD);

console.log("[rewire] cad, draw, note");
