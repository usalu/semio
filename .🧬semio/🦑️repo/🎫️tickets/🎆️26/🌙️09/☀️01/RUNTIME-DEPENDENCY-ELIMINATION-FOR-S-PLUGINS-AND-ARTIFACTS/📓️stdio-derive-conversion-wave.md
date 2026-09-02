# 🗄️ stdio — derive/`#[serde(...)]` → `ToValue`/`FromValue`/`#[value(...)]` conversion wave

Scope handed to this session: convert PRODUCTION `#[derive(Serialize, Deserialize)]` /
`#[serde(...)]` sites in `✏️s/🔌️plugins/🗄️stdio` (crate `semio-s-plugin-stdio`) to
`#[derive(ToValue, FromValue)]` / `#[value(...)]`, excluding `🧪️oracle/`, `🧪️test/`, `🔬️probes/`,
`🏭️generator/`, `🧫️fixtures/`. **This is narrower than eliminating every `serde_json::` call
site** (that broader elimination is a separate, much larger, already-partially-done effort — see
`🔍️research/📓️serde-fanout-stdio-trinity-space.md` in this ticket — not attempted here beyond
what fell out of the derive conversion itself).

## Starting state (measured, not assumed)

Prior sessions (see the research doc above) had already converted the great majority of stdio's
derive sites across several waves. At the start of this session:

```bash
grep -rl "derive(.*Serialize" --include="*.rs" ✏️s/🔌️plugins/🗄️stdio/ | grep -v /target/ | \
  grep -vP '/🧪️(test|tests|oracle)/|/🔬️probes/|/🏭️generator/|/🧫️fixtures/'
```
→ **14 files.** (Real `serde_json::` production call-site file count, out of this wave's scope,
was separately 202 files / ~7091 grep hits before stripping `#[cfg(test)] mod` blocks, 142 files
after — dominated by the `🧊️gltf` inference-leaf family's `encode_result()` pattern and a handful
of viewer/editor `🪟️windows/🪟️main` files; left untouched, see Follow-up.)

## Disposition of all 14 files

| file | disposition | why |
|---|---|---|
| `✳️mesh/…/📦aabb/🦀️.rs` | already done (prior session) | no `Serialize`/`Deserialize` present |
| `✳️drawing/…/🎛flattened-scene/🦀️.rs` | already done (prior session) | same |
| `✳️any/🧬️schema/🧮️geometry/🦀️.rs` | **FIXED this session** | `SemioPoint3`/`SemioQuaternion`/`SemioTransform` had unconditional `Serialize, Deserialize` alongside `ToValue`/`FromValue`, left over from an incomplete prior pass (siblings `SemioPoint2`/`SemioUv`/`SemioRgba` in the same file were already `ToValue`/`FromValue`-only). The file's own `#[cfg(test)] mod tests::identity_transform_round_trips_through_json` uses `serde_json::to_string`/`from_str` as a round-trip oracle over `SemioTransform` — sanctioned per the ticket's own rule. Converted all 3 structs to `#[cfg_attr(test, derive(Serialize, Deserialize))]` + `#[cfg_attr(test, serde(rename_all = "camelCase"))]`, moved `use serde::{Deserialize, Serialize};` behind `#[cfg(test)]`. |
| `✳️table/…/🎲entropy/🦀️.rs` | **left as-is, verified correct** | Explicit in-file doc comment: `store::InferredField::Value` (framework trait, out of this plugin's scope) bounds on `Serialize + DeserializeOwned` — a genuine, permanent requirement, not leftover. Not a conversion target. |
| `✳️table/…/📊moments/🦀️.rs` | left as-is, verified correct | same `InferredField::Value` rationale, same doc comment pattern |
| `✳️brep/…/✅validation-report/🦀️.rs` | left as-is, verified correct | same |
| `✳️graph/…/🔗connectivity/🦀️.rs` | left as-is, verified correct | same |
| `✳️object/🧬️schema/🦀️.rs` | already correct (prior session) | `ArtifactSchema` + `#[cfg_attr(test, derive(Serialize, Deserialize))]` sanctioned pattern already in place |
| `✳️object/📸️snapshot/🦀️.rs` | already correct (prior session) | same pattern |
| `✳️kit/🧬️schema/🦀️.rs` | already correct (prior session) | same pattern |
| `✳️kit/📸️snapshot/🦀️.rs` | **FIXED this session** | `SemioKitSnapshot` itself already had the sanctioned `#[cfg_attr(test, ...)]` pattern, but its 4 sibling field types (`SemioKitType`/`SemioKitPiece`/`SemioKitConnection`/`SemioKitDesign`) — nested inside `SemioKitSnapshot`'s own `Vec<...>` fields and therefore required by the SAME test-oracle round trip (`serde_json::from_str::<SemioKitSnapshot>` in several `🧪️tests/` mutation fixtures) — still had unconditional `Serialize, Deserialize`. Extended the same `#[cfg_attr(test, ...)]` pattern to all 4, moved the `use serde` import behind `#[cfg(test)]`. Verified zero real (non-test) `serde_json::` usage of these types anywhere in the plugin first (the two non-test files that reference `SemioKitSnapshot`, both `🪟️windows/🪟️main/🦀️.rs` viewer/editor files, build ad hoc JSON via `serde_json::json!` literals — they never call `serde_json::to_value`/`Serialize` on the kit types themselves, so nothing production depends on this derive). |
| `🎒️zip/📦️opc/🦀️.rs` | **FIXED this session** | `OpcPart`/`OpcContentTypes`/`OpcTargetMode`/`OpcRelationship`/`OpcPackage` all carried unconditional `Serialize, Deserialize` alongside `ToValue`/`FromValue` with no documented rationale. Confirmed zero `serde_json::` usage anywhere in the file and zero cross-file usage of any of these 5 types with `serde_json` (`grep -rl` for the 5 type names piped into `grep -l serde_json` → no matches at all, not even in test dirs). Removed `Serialize, Deserialize` from all 5 derive lists, deleted all 6 `#[serde(...)]` attribute lines, deleted the `use serde::{Deserialize, Serialize};` import. |
| `🧊️gltf/…/🚪️io/💡️inferences/📝️text/🦀️.rs` (`GltfInferenceLeafEnvelope`) | **left as-is, verified correct — real blocker, documented** | The file's own doc comment records that a prior session tried the `ToValue`/`FromValue` swap and reverted it: the struct's `value: serde_json::Value` field has no `ToValue` impl, and the crate-root `infer_gltf_leaf_cold` builds this envelope directly from every inference leaf's `encode_result() -> Result<serde_json::Value, serde_json::Error>`. Fixing this for real means retyping `value` to `pack::JsonValue` AND updating all ~55 `encode_result` leaf functions AND rewriting `write_canonical_json`/`canonical_number` (RFC 8785 canonical-JSON serializer, pattern-matches `serde_json::Value`/`Number` variants directly) against `pack::JsonValue`'s shape — a self-contained, substantial follow-up, explicitly out of the derive-only scope of this wave. |
| `🧊️gltf/…/🧬️schema/📸️snapshot/🦀️.rs` | left as-is, verified correct (prior session's own finding, re-confirmed) | `GltfDocument` and its ~32 nested types are the literal wire model for real `.gltf`/`.glb` files, read via `serde_json::to_vec`/`from_str` in the sibling `🚪️io/🦀️component.rs` (not touched). `value_derive::ToValue`/`FromValue` were already added ADDITIVELY alongside the pre-existing `Serialize`/`Deserialize` (never replacing it) in a prior session — correct, permanent, not leftover. |

## Verification

`cargo check -p semio-s-plugin-stdio --message-format=short` run in the foreground. The repo had
~15-20 other agents' `cargo check`/`cargo build` processes running concurrently against the same
workspace target dir at the time (confirmed via `ps aux`), matching this ticket's own documented
"~10 agents compile concurrently, be patient" expectation.

<!-- VERIFICATION_RESULT_PLACEHOLDER -->

## Follow-up (not attempted this session, out of the derive-only scope)

- **`🧊️gltf` inference-leaf family real conversion** (the `GltfInferenceLeafEnvelope`/`encode_result`
  chain above) — the single largest remaining block of real `serde_json::` production call sites in
  this plugin (~55 near-identical `encode_result` leaf functions + the RFC 8785 canonical-JSON
  writer). Needs `pack::JsonValue` to grow the same `Index`/`Display`/canonical-number-formatting
  surface `serde_json::Value` has today before it's a safe mechanical swap.
- **`🪟️windows/🪟️main` viewer/editor family** — remaining files use `serde_json::json!`/`to_string`
  for ad hoc UI JSON, not derived serialization; convertible to `pack::json!`/`pack::to_json_string`
  per the prior session's own plan, independent of any derive.
- Regenerate the real remaining call-site list (excludes `#[cfg(test)] mod` blocks, which a naive
  grep does not):
  ```bash
  grep -rl "serde_json::" --include="*.rs" ✏️s/🔌️plugins/🗄️stdio/ | grep -v /target/ | \
    grep -vP '/🧪️(test|tests|oracle)/|/🔬️probes/|/🏭️generator/|/🧫️fixtures/'
  ```
- `Cargo.toml`'s `serde`/`serde_json` `[dependencies]` entries were NOT touched (per instruction:
  don't clear until the code compiles without them — still hundreds of real call sites remain).

## Files touched this session

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️.rs`
