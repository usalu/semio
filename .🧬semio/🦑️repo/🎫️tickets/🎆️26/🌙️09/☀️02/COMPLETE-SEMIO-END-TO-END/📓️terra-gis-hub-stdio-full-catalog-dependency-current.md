# GIS/Hub Stdio Full-Catalog Dependency Frontier

## Scope and evidence

Read-only source audit on 2026-09-06. No Cargo, Nx, browser, cache, or process operation was run. This corrects the older `terra-hub-headless-stdio-catalog-dependency-frontier.md`: the current Hub core already has a real headless/native feature separation. The remaining expensive edge is not one uniform Hub problem.

| Consumer and configuration | Current edge | What the code actually needs | Is `full-artifact-catalog` logically required? |
| --- | --- | --- | --- |
| GIS plugin, every build | `semio-s-plugin-stdio` with `default-features = false, features = ["full-artifact-catalog"]` | Stdio artifact models, schema identifiers, mutations, and the drawing-to-SVG pure interop surface | No. It does not call Stdio plugin assembly, registry discovery, manifest, editor/viewer, or native factory receipts. |
| Hub core, `--no-default-features --features sqlite` | no Stdio/GIS/VCS dependency | Catalog authority port only; no configured native provider | Already isolated. A configured catalog without a provider rejects rather than pretending to execute. |
| Hub configured native provider/default build | `native-artifact-execution` forwards `stdio/full-artifact-catalog`, plus GIS and VCS | All 26 Stdio native codec receipts, which are instantiated and checked as part of a fixed 29-receipt provider closure | Yes for the present provider contract. |
| Hub `test-support` binary laws | forwards `native-artifact-execution` | Constructs the real GIS Map trusted-catalog/profile support and native provider | Intentionally yes today; this is not evidence that headless Hub needs the full catalog. |

Primary source anchors:

- [Hub features](../../../../../../🌎️hub/📦️packages/🦀️rust/Cargo.toml:21) make `native-artifact-execution` explicitly opt-in, although it is in the default set; `test-support` deliberately forwards it at [line 35](../../../../../../🌎️hub/📦️packages/🦀️rust/Cargo.toml:35).
- [GIS Cargo](../../../../../../✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:77) is the unconditional avoidable full-catalog edge.
- [Stdio features](../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:31) show that `full-artifact-catalog` currently gates a root that hosts both the plugin and all taxonomy models.
- [Hub’s native provider](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:21) has a fixed 29-provider set. Its Stdio branch obtains the receipts at [line 123](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:123), requires exactly 26 at [line 131](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:131), and invokes every factory at [line 150](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:150).
- [Hub startup](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6569) only links that provider under the native feature; without it, the provider is `None` at [line 6573](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6573).
- [Hub’s own source law](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:2732) explicitly declares headless `sqlite` as having no direct plugin dependencies and production as `stdio/full-artifact-catalog`, GIS, and VCS. Its headless native group uses `--no-default-features --features sqlite` at [line 2970](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:2970); native-provider groups deliberately turn on `native-artifact-execution` at [line 3752](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:3752).

## What makes Stdio full today

The feature is not a narrow marker: the Stdio root’s `plugin-root` forwards it, and the full root compiles the broad artifact taxonomy. The registry loads 36 source definitions under the full feature at [registry line 247](../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:247), creates 26 native factories at [line 982](../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:982), and publicly makes native receipts only in the full configuration at [line 1117](../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:1117). That is a real full-provider closure, not merely a re-export accident.

GIS uses this same crate as an artifact-model library. Its current imports cover:

- external artifact families: `dxf`, `dwg`, `gltf`, `json`, `las`, `obj`, `pdf`, `ply`, `png`, `stl`, `svg`, and `txt`;
- Semio families: `base`, `drawing`, `image`, `mesh`, and `value`;
- the drawing snapshot/diff/mutation APIs and the actual `SemioDrawingToSvg` bridge, not only passive DTOs.

Examples are the Map aggregate’s drawing/image/value members at [GIS Map root lines 8–10](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:8), the Map schema’s drawing-to-SVG interop at [lines 9–12](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:9), Map mutation/inverse code at [inferences lines 9–12](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:9), and Terrain’s geometry/mesh use at [Terrain root lines 5–6](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🦀️.rs:5). The lone `semio::register()` occurrence found in GIS is test-only at [Map schema line 539](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:539).

No audited non-test GIS import uses `plugin()`, `manifest`, `registry::sources`, Stdio component exports, or `native_codec_factory_receipts`. Thus the Stdio plugin/provider plane is an avoidable compilation dependency of GIS, while its model/interop plane is not.

## One bounded schema-first refactor

Create one first-party crate: `semio-s-stdio-artifact-models`, with a single source-defined feature surface `gis-interop`.

1. Define a neutral `semio.stdio.gis-interop-surface/v1` corpus. Its sorted authority list is the twelve external families and five Semio families above, plus exact schema identifiers and the `drawing -> svg` interop. It must expressly exclude `plugin`, manifest assembly, registry source discovery, editor/viewer matrices, component exports, and native codec receipts.
2. Move the corresponding model, mutation/diff, schema, and pure interop implementation from Stdio’s taxonomy root into this crate. The generated Rust module tree must be driven by that neutral surface, rather than another empty Cargo feature on the present giant root.
3. Change GIS to depend only on `semio-s-stdio-artifact-models` with `gis-interop`; move all current typed imports in that crate transition. Do not keep a compatibility re-export: this is greenfield and the separation is meant to make plugin/provider APIs unavailable to GIS at compile time.
4. Change the Stdio plugin to consume the model crate’s complete model surface, while retaining its current full registry, manifest/editor/viewer, component root, and 26-factory native provider closure. Hub’s `native-artifact-execution` remains exactly attached to that full plugin.

This is preferable to a GIS feature on the existing Stdio plugin crate: Cargo feature unification can re-enable full when another dependency asks for it, and the plugin/registry authority would remain reachable from GIS. The dedicated model crate creates the needed type boundary without weakening the real Hub provider.

## Required acceptance before claiming any speedup

1. A language-neutral `gis-interop-surface/v1` fixture accepts exactly the named 12+5 surface and bridge; mutations adding a full-only family or a provider/plugin symbol reject.
2. GIS Map drawing/value and Terrain model/import-export laws retain their exact packed/type behavior through the new crate, including drawing-to-SVG and inverse mutation paths.
3. The GIS component/native target’s dependency assertion shows `stdio-artifact-models/gis-interop` and does not contain `semio-s-plugin-stdio/full-artifact-catalog`.
4. Existing Stdio native receipt law continues to instantiate all 26 real receipts; Hub’s native provider continues to admit its exact 29 bindings. This prevents a performance refactor from silently changing configured execution capability.
5. Existing Hub headless `--no-default-features --features sqlite` law remains provider-free and configured-catalog fail-closed. `test-support` may stay full until a separate fixture proves it can be narrowed without losing its real profile construction.

## Scope conclusion

The maximum verified coverage gain is to remove the full Stdio taxonomy from GIS compilation/materialization. It will improve standalone GIS native and component lanes. It will **not** shorten a Hub target that intentionally enables `native-artifact-execution` or `test-support`, because the current configured provider’s admission contract genuinely consumes the full Stdio closure. No compiler failure or live build duration is asserted here.
