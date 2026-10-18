import type {Spec} from "../tssplit.ts";
const CV = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification";
const spec: Spec = {
  orig: "cv",
  file: CV + "/🟦️.ts",
  children: [
    {folder: "📚️navbar-example-audits", doc: "📚️ Navbar example audits of the catalog verification: the playground owners that ship without an example picker, the picker coverage audit of a published manifest and the example artifact payload audit.", ranges: [[20, 59]]},
    {folder: "🎛️commands", doc: "🎛️ Commands of the catalog verification: the registry test, catalog contract test, native catalog selection check and catalog complete scripts that package routing lazily loads.", ranges: [[61, 77], [1006, 1052]]},
    {folder: "🛡️guards", doc: "🛡️ Guards of the catalog verification: the bounded diagnostic text, the path containment check, the strict json value check, the descriptor enum normalisation and the duplicate json object name refusal.", ranges: [[227, 266], [365, 432]]},
    {folder: "🗺️verification-plan", doc: "🗺️ Verification plan of the catalog verification: the bounded dependency order of catalog nodes and the cancellable, progress reporting execution of the plan.", ranges: [[269, 350]]},
    {folder: "🧬️descriptor-pair", doc: "🧬️ Descriptor pair of the catalog verification: the execution protocol and descriptor shape validation, the placeholder identity refusal, the descriptor pair decoding and the semantic and on disk descriptor pair verification.", ranges: [[444, 482], [502, 560]]},
    {folder: "📥️artifact-io", doc: "📥️ Artifact io of the catalog verification: the contained catalog file read, the chunked and cancellable artifact sha256, the verified artifact read and the artifact receipt.", ranges: [[353, 362], [637, 700]]},
    {folder: "🔎️source-audit", doc: "🔎️ Source audit of the catalog verification: the audit of every plugin catalog source against its strict descriptor, its dependency order and its node cap.", ranges: [[563, 634]]},
    {folder: "🏗️fresh-build-verifier", doc: "🏗️ Fresh build verifier of the catalog verification: the fresh catalog commit marker, the exact catalog row requirement and the verifier of one fresh catalog build root.", ranges: [[703, 812]]},
    {folder: "🎯️native-selection", doc: "🎯️ Native selection of the catalog verification: the native catalog selection types, its plan and preview, the registry schema validator and the selection oracle.", ranges: [[814, 1003]]},
  ],
  patch: [
    {folder: "🎯️native-selection", old: 'join(import.meta.dir, "..", "🧬️schema/🔣️.json")', new: 'join(import.meta.dir, "..", "..", "🧬️schema/🔣️.json")'},
    {folder: "🎯️native-selection", old: 'join(import.meta.dir, "..", "🧪️tests/🧫️fixtures/📦️native-catalog-selection")', new: 'join(import.meta.dir, "..", "..", "🧪️tests/🧫️fixtures/📦️native-catalog-selection")'},
  ],
};
export default spec;
