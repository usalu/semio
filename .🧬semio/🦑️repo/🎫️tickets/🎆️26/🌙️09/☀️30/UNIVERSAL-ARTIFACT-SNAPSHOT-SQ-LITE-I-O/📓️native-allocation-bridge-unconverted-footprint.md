# Native Allocation Bridge Unconverted Footprint

Read-only current source observation, 2026-10-03. No Cargo/build/tests. Concurrent mounts have changed since earlier reports. Three core laws are not universal integration proof.

## Current Shared Seam and Actual Consumption

`🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs` now has max_allocation_bytes (default512MiB), persistent allocation_bytes and allocation_stage (line142). It supplies remaining cumulative capacity and settles child owned bytes on success/refusal/cancellation. Current bounded search over framework and ✏️s finds only three allocation_stage callsites, all new core unit laws at sqlite-snapshot/🧪️tests/🔬️unit/🦀️.rs:189–191. No production consumption was found.

Both mounted shared snapshot-capability routes still create fresh Native controls from limits.max_value_bytes: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs:25` and sibling `🛫️native-encoding/🦀️.rs:19`. These controllers are cumulative inside one parser/binder/emitter but never settle their admitted backing into the SqliteSnapshotControl spanning transfer phases. Changing the argument to max_allocation_bytes alone would still reset the budget between phases. Wrap the whole producer operation with allocation_stage, preserve its result and settle actual owned_bytes even on errors. Existing native callbacks already adapt to parent phases.

## Sixteen Scoped Providers

Scope exactly follows current-stdio-native-route-readback: IFC4, IFC2x3, BMP, SVG, HTML, Deflate, STEP, MD, TIFF, Binary, PNG, JSON, LAS, GLTF, CSV, TSV. Source hook status now differs from that older report: HTML, MD, TIFF and JSON override both input and output; GLTF overrides input only. Other eleven override neither. All own SQLite capability opt-ins; this does not itself prove public factory registration or runtime behavior.

HTML mounted native module uses fresh semantic ceilings at lines37/47. MD provider hooks at sqlite:161/162 delegate owned_pack, whose native encoding/decoding relies on shared capability controls. JSON output similarly delegates owned_pack/shared control; explicit input at sqlite:51 uses max_value_bytes. GLTF input at sqlite:94 does likewise. TIFF mounted io decode:88 and encode:69 use max_value_bytes. All lack parent stage settlement. TIFF source hooks at sqlite:19/20 are now mounted: prior missing-hook report is stale.

Binary, CSV, TSV, BMP, Deflate and PNG individual 🚦️native modules remain unmounted drafts. Their twelve constructor callsites still use max_value_bytes. Do not convert these observations into active route coverage. SVG/IFC/STEP/LAS have no native constructor under the scoped snapshot roots; their absent hooks remain genuine unfinished ownership surfaces rather than successful bridges.

## Exact Snapshot Constructor Inventory

The following are non-test snapshot-root constructor source occurrences. This broader bounded sample includes builtin owners beyond the sixteen-provider cohort; it is not a universal repository count. Paths are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`.

| Exact path and line | Control | Current ceiling |
| --- | --- | --- |
| `📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:11` | NativeDecodeControl | `limits.max_value_bytes` |
| `📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:21` | NativeEncodeControl | `limits.max_value_bytes` |
| `🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:28` | NativeDecodeControl | `limits.max_value_bytes` |
| `🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:46` | NativeEncodeControl | `limits.max_value_bytes` |
| `🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:27` | NativeDecodeControl | `limits.max_value_bytes` |
| `🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:35` | NativeEncodeControl | `limits.max_value_bytes` |
| `📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:18` | NativeDecodeControl | `limits.max_value_bytes` |
| `📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:32` | NativeEncodeControl | `limits.max_value_bytes` |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:51` | NativeDecodeControl | `limits.max_value_bytes` |
| `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🚦️native/🦀️.rs:47` | NativeEncodeControl | `limits.max_value_bytes` |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛫️native/🦀️.rs:71` | NativeEncodeControl | `limits.max_value_bytes` |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛬️native/🦀️.rs:97` | NativeDecodeControl | `limits.max_value_bytes` |
| `📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🦀️.rs:75` | NativeEncodeControl | `limits.max_value_bytes` |
| `📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs:17` | NativeEncodeControl | `limits.max_value_bytes` |
| `📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs:37` | NativeDecodeControl | `limits.max_value_bytes` |
| `🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:37` | NativeDecodeControl | `limits.max_value_bytes` |
| `🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:47` | NativeEncodeControl | `limits.max_value_bytes` |
| `📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🦀️.rs:41` | NativeEncodeControl | `limits.max_value_bytes` |
| `📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs:33` | NativeEncodeControl | `limits.max_value_bytes` |
| `📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs:69` | NativeDecodeControl | `limits.max_value_bytes` |
| `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:13` | NativeDecodeControl | `limits.max_value_bytes` |
| `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:16` | NativeEncodeControl | `limits.max_value_bytes` |
| `💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:10` | NativeDecodeControl | `limits.max_value_bytes` |
| `💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:24` | NativeEncodeControl | `limits.max_value_bytes` |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:84` | NativeDecodeControl | `limits.max_value_bytes` |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:119` | NativeEncodeControl | `limits.max_value_bytes` |
| `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:94` | NativeDecodeControl | `limits.max_value_bytes` |
| `🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:36` | NativeDecodeControl | `usize::MAX` |
| `📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🦀️.rs:81` | NativeEncodeControl | `limits.max_value_bytes` |
| `📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs:16` | NativeEncodeControl | `limits.max_value_bytes` |
| `📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs:36` | NativeDecodeControl | `limits.max_value_bytes` |
| `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🛫️native/🦀️.rs:35` | NativeEncodeControl | `limits.max_value_bytes` |
| `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🛬️native/🦀️.rs:9` | NativeDecodeControl | `limits.max_value_bytes` |
| `📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:119` | NativeEncodeControl | `maximum` |
| `📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:120` | NativeEncodeControl | `maximum` |
| `📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:120` | NativeEncodeControl | `1` |

Additional TIFF constructor paths sit outside snapshot roots: `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🛬️decoding/🦀️.rs:88` and sibling `🛫️encoding/🦀️.rs:69`.

The bounded builtin sample demonstrates fresh max_value_bytes controllers in TXT, XML, semio base, ZIP, DOCX/XLSX/PPTX native encoding/decoding and subset validation. Subset validation creates a fresh controller even after preceding native phases, so shared parent allocation_stage must span it too. PLY ordinary pack decoder uses usize::MAX; it is an ordinary codec surface, not evidence that an erased SQLite native hook is bounded. PDF snapshot sqlite lines119/120 are inline test-local constructors despite residing in a production source file; exclude them from production integration totals.

## Semantic Contracts and Real Gaps

Preserve `check_database` and `check_value_bytes`: they count exact semantic SQLite cells, including scalar bytes and literal schema/fields. Those remain a separate ceiling after moving Native* backing to allocation_stage. Current `Reconstruction::reserve` in sqlite-snapshot/🧩️artifact still aggregates copies against reconstruction/scalar semantic allowance and does not charge slot/frontier backing to the new allocation ledger. `ordered_row_refs` allocates BTreeMap plus resultVec. Projection/backing helpers and physical SQL/page allocation need the common authority too; converting native hooks alone cannot prove all backing admission.

Small owners still clone/project schema and reconstruct with document.text(...).into(); CSV/TSV literal fields also clone uncontrolled, as recorded in small-stdio-current-api-readback-audit. Those require neutral owner schema/field interior-copy laws. Native semantic domains cannot be defended only by replacing a byte ceiling: enforce row counts and full literal semantic values at typed projection/reconstruction, while admission counts containers, histories, parser records, physical buffers and copies independently.

Public erased codec export/import must retain the same control through native decode, subset validation, semantic projection, physical file work, reconstruction, preflight and native encode. Public typed file mode must remain full literal SQL transport and must not call native codecs merely to account backing. The erased route and its actual public declaration need neutral tests showing max_value_bytes exact acceptance, independently too-small allocation refusal, cumulative two-stage exhaustion, and no fresh controller reset. Existing constructor-only corelaws cannot establish these owner/public-boundary properties.

Required owner tests: long literal schema and single-field text/blob cancellation; structural frontier rejection before slot ownership; successful first stage followed by second-stage backing refusal; admitted bytes retained on cancellation/error; independently adjustable value/file/allocation ceilings; exact native byte parity after bridging; typed full-file bypass of native phases; actual public erased import/export and subset validation. Run genuine owning runtime baselines rather than inferring coverage from allocation_stage presence.

No compiler/runtime success, universal cumulative bridge, public factory coverage or allocation completeness is claimed.
