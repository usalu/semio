# Assembly precheck for the eight component describes

Scope: gis, flow, process, cad hubs; cad-extension-aec-building, -energy, -structure, cad-extension-spatial-shape; flow-extension-brep.

## Result
No assembly violation found. The native assembly run could not be executed, so this is a static audit; native confirmation is still open.

## Native path (exists, blocked)
Every component crate carries the generated `descriptor_is_fresh` test (`🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs`). It installs the bundle and runs the same `describe_plugin` / `describe_extension` the guest runs; a failed assembly yields the `assembly-failed` stub and the test then fails against the committed descriptor.
Command per component (one at a time):
`repo cargo --store=isolated --configuration=prototype -- test --manifest-path <component>/📦️packages/🦀️rust/Cargo.toml --lib descriptor_is_fresh -- --nocapture`
(use the manifest path, `-p` does not resolve: components are own workspaces.)
Blocked: the running build (process 56789, `components describe gis flow process ca...`) holds the exclusive lease `semio.cargo.custody-lease/v1 ... release`; the isolated test sat 10 minutes on `semio.compiler.host-capacity/v1` without compiling and was killed. The build is already in its describe phase, so a precheck is no longer ahead of it.

## Static audit (all clean)
| Rule | Check | Result |
|---|---|---|
| `composer_identity_claims` (`🪪️identity/🦀️.rs`) | every `ComposerEntry` of gismap (svg, pdf, png, json, geojson, dwg, dxf), gisterrain (las, ply, stl, gltf, obj), process3d, cad, flow reads only its own dialect; each foreign writer has one `composition-route` capability; all length frames parse and match the dialect strings | ok |
| `cancellation-proof` / `-incomplete` | proof tool ids per controller: no duplicates, all are command ids; flow 36/36, process 31/31, cad 39/39, gisterrain 3/3, gismap editor 18/18 | ok (migrated-set intersection not evaluated) |
| input schema `$ref` | 75 external `$ref` documents in the four plugin trees (incl. extensions) all have a `$id` in the repo | ok |
| kind identity (emitter) | committed gis descriptor: app io kind and schema match declared kinds | ok (descriptor predates the route edits) |
| flow-extension-brep | unique contribution ids, manifest fields non-empty | ok |
| aec-* / spatial-shape | contribute mutations and inferences to `s.cad.cad`; no composers or proofs of their own | not checked beyond schemas |

## Residual risk
Unverifiable without compiling: `migrated_tool_ids` coverage, contribution gates of the cad extensions, `first_owned_codec` for gis. cad itself was re-described after the route fix (descriptor mtime after the edit), so the same declarations pass.

## Patch
`apply-assembly-precheck.py` carries an empty patch list (idempotent, nothing to change).
