// 🧪 c5c7 probe: runs the layout media-adoption inspection on the module-tree sources named by the fixture.
import {resolve} from "node:path";
import {readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const D = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export";
const corpus = await Bun.file(D + "/🧪️tests/🧫️fixtures/🔣️.json").json();
const {inspectLayoutMediaAdoptionSourceV1} = await import(D + "/🧪️testing/📥️source/🟦️.ts");
const sources = {
  editor: readModuleTreeSource(resolve(D + "/🧪️tests", "..", corpus.contract.sources.editor)),
  export: readModuleTreeSource(resolve(D + "/🧪️tests", "..", corpus.contract.sources.export)),
};
console.log("sources", sources.editor.length, sources.export.length);
console.log("admits", inspectLayoutMediaAdoptionSourceV1(corpus.contract, sources));
let bad = 0;
for (const row of corpus.negativeBindings) {
  const source = (sources as Record<string, string>)[row.source]!;
  const once = source.split(row.from).length - 1 === 1;
  const refused = !inspectLayoutMediaAdoptionSourceV1(corpus.contract, {...sources, [row.source]: source.replace(row.from, row.to)});
  if (!once || !refused) { bad += 1; console.log("FAIL", row.id, once, refused); }
}
console.log("negative rows", corpus.negativeBindings.length, "bad", bad);
console.log("comment-only refused", !inspectLayoutMediaAdoptionSourceV1(corpus.contract, {...sources, editor: `/* ${sources.editor} */`}));
console.log("dup editor refused", !inspectLayoutMediaAdoptionSourceV1(corpus.contract, {...sources, editor: sources.editor + "\nimpl ArtifactEditor for LayoutPlayApp {}"}));
console.log("dup export refused", !inspectLayoutMediaAdoptionSourceV1(corpus.contract, {...sources, export: sources.export + "\nimpl LayoutMediaExportJob {}"}));
