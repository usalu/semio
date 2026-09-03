# `semio-framework-ui-scene` serde blocker — DslValue char/bytes investigation

Follow-up to `📓️phase2-2d-graph-ui-scene-serde-removal-2026-09-03.md`, which left `ui-scene` untouched
and recommended either widening `DslValue` or narrowing the codec's contract. This pass verified
that claim from scratch (files read directly, not trusted) and measured the blast radius of the
"widen `DslValue`" option.

## 1. What `pack.rs` actually needs — confirmed, not just trusted

`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️pack.rs` (862 lines) implements
`serde::Serializer`/`serde::Deserializer` (`PackSerializer`/`PackDeserializer`) as a generic binary
codec over any `T: Serialize`/`Deserialize` — not a `serde_json` convenience wrapper, the actual
trait machinery. It has 14 wire tags including `TAG_CHAR = 11` and `TAG_BYTES = 7`
(`serialize_char`/`serialize_bytes`/`deserialize_char`/`deserialize_bytes` all implemented,
lines 175–190, 448–451, 497–530).

The claim that this is genuinely exercised (not dead code) is **confirmed**: the oracle test
`owned_scene_neutral_vectors_match_native_serde_packet` (line 719, inside the protected
`//#region 🎬️RetainedSceneOracle`) calls `to_bytes(&'🧹')` for a `"char"` case and
`to_bytes(&Bytes(&[0, 128, 255]))` (a local newtype whose `Serialize` calls
`serializer.serialize_bytes`) for a `"bytes"` case, and asserts the output equals pre-computed hex
bytes from `🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-scene.json`. A second oracle,
`scene_pack_numeric_widths_do_not_wrap`'s sibling hostile-input list, includes an `"invalid-char"`
case (`hex: "0b80b003"`, tag `0x0b = TAG_CHAR`) that must be *rejected* by the decoder. Both are
independent of the 15 `SceneDoc` payload types — they prove the codec's general `T: Serialize`
capability, which is the whole point of implementing the serde traits directly instead of hand
rolling per-type encoders.

## 2. `DslValue`'s current variants and bytes handling

`🧰️framework/🔨️modules/🌱️value/🦀️.rs:100-107`:
```rust
pub enum DslValue { Null, Bool(bool), Number(Number), String(String), Array(Vec<DslValue>), Object(Vec<(String, DslValue)>) }
```
Six variants, no `Char`, no `Bytes`. Grepped the enum's own module, its `🔁️codec/🦀️.rs`
(`ToValue`/`FromValue`, the serde-analog traits), and the framework's own JSON writer
(`🎒️pack/🔤️json/🦀️.rs`) — **bytes are not handled at all**, not base64, not array-of-numbers, not
anywhere in this type's surface. There is a first-party base64 codec crate
(`semio-framework-io-base64`, `🚪️io/🔤️base64/🦀️.rs`) but nothing wires it to `DslValue`.

`semio-framework-pack` (crate name `pack`, `🎒️pack/📦️packages/🦀️rust`) is a different thing from
what the ticket brief implied — it's the `.spk` binary container format (header/segment/manifest
framing), not a JSON-parity binary codec for arbitrary structs. Its `🔤️json` submodule is a
from-scratch JSON writer with its own byte-for-byte oracle against `serde_json`
(`write_float_matches_serde_json_byte_for_byte`), and its `from_dsl_value`/`to_dsl_value` bridge
(line 527-536, 544-553) is an **exhaustive 6-arm match with no wildcard** — the same shape problem
as `🌱️value/🦀️.rs`'s own bridge. Neither this crate nor anything else in the tree provides a
ready-made `char`/`bytes`-capable path onto `DslValue`.

## 3. Blast radius

`grep -rn 'DslValue::' --include='*.rs' . | wc -l` (excluding `target/`) → **2418** occurrences.
Narrowing to files with an exhaustive, non-wildcard `DslValue::Null =>` match arm (the cheapest
reliable proxy for "would need a new arm or a catch-all added") → **24 files**, spanning
`🌱️value` itself, `🎒️pack/🔤️json`, `🕸️graph` (dsl + manifest), `os-kernel`'s `🌊️flow`/`🔌️plugin`/
`🎒️pack/🔢️value`/`🗣️dsl/🧬️schema`/`🌉️mcp` (protocol + schema)/`🖥️shell`/`🔁️workflow`, `♾️infinite`'s
DAG board ports, `🧠️neural`, and eight `✏️s/🔌️plugins/*` artifact schema-mutation/interaction files
(raster, cad, gltf, procedural, forms, architect xlsx export, energy). Two of the most
load-bearing — `🌱️value/🦀️.rs`'s own `From<&DslValue> for serde_json::Value` (line 218-231) and
`🎒️pack/🔤️json/🦀️.rs`'s `from_dsl_value`/`to_dsl_value` — are both confirmed exhaustive with zero
wildcard arm, so adding variants breaks compilation at both of the framework's two independent
JSON bridges, not just at obscure call sites. This is a genuine framework-wide change, not a
localized one; verifying all 24 sites (plus whatever the raw `2418` count hides behind non-`Null`
partial matches, `if let`, or `matches!`) is multi-day work, not something to attempt inside this
investigate-first pass.

## 4. Would adding variants change the JSON wire encoding?

Not necessarily, if designed carefully: `serde_json`'s own generic `Serialize` path already
encodes a bare `char` as a one-character JSON string, and `serialize_bytes` (its default codepath
for any `T` without `serde_bytes`) writes a JSON array of numbers — so `DslValue::Char` could map
to `serde_json::Value::String` and `DslValue::Bytes` to `serde_json::Value::Array` of
`Number::UInt` without disturbing the existing byte-for-byte parity property (verified by
`write_float_matches_serde_json_byte_for_byte`, which is float-only and untouched by this). The
real risk is not the JSON encoding — it's the 24+ exhaustive match sites above, and the fact that
`ui-scene`'s own binary wire format (`TAG_CHAR`/`TAG_BYTES`) is fixture-pinned
(`🔣️owned-scene.json`, protected) and could not be reproduced by a `DslValue`-tree-walking codec
without either (a) `DslValue::Char`/`Bytes` round-tripping to the *exact* same tag bytes the
fixture already hard-codes, which is a large rewrite of the 640-line codec, or (b) changing the
fixture, which is forbidden.

## 5. Cheaper alternative?

Investigated and rejected: representing `char` as a 1-char `String` and `bytes` as base64/array
inside `ui-scene`'s own codec would change `to_bytes(&'🧹')`'s output from a `TAG_CHAR`-prefixed
4-byte varint payload to a `TAG_STR`-prefixed UTF-8 payload — different bytes than the fixture's
pinned hex, failing the oracle without touching it directly (the oracle asserts equality against
fixture-supplied hex, so the test still runs, it just fails). Keeping `serde` as a
`[dev-dependencies]`-only oracle (the pattern already used for `semio-framework-3d`) does not
apply here: `pack.rs`'s `Serializer`/`Deserializer` impls are the production wire codec itself, not
a test-only comparison — moving `serde` to dev-only would delete the actual encoder, not a
convenience layer.

## Recommendation

**Change nothing.** This confirms the phase2 analysis was accurate (its "char/bytes" claim holds
under direct re-verification) and that neither shortcut works: `DslValue` widening is a genuine
framework-wide change (24+ exhaustive match sites across `os-kernel`, `graph`, `pack`, `infinite`,
and eight plugin artifact schemas) that cannot be verified safely inside one investigate-first
pass, and the "keep serde as dev-only oracle" pattern doesn't apply because `pack.rs`'s serde usage
*is* the production codec. `ui-scene`'s `Cargo.toml` is unchanged (`serde` stays a real
dependency, `serde_json` stays dev-only, unchanged from before this pass).

**For whoever picks this up next**: the concrete, scoped next step is adding
`DslValue::Char(char)`/`DslValue::Bytes(Vec<u8>)`, auditing all ~24 exhaustive-match files above
one by one, adding matching arms to `🌱️value/🦀️.rs`'s and `🎒️pack/🔤️json/🦀️.rs`'s JSON bridges
(`Char -> String`, `Bytes -> Array<UInt>`), then rewriting `ui-scene/🦀️pack.rs` to walk
`ToValue`/`FromValue` trees instead of implementing `serde::Serializer`/`Deserializer` directly,
reproducing the exact `TAG_CHAR`/`TAG_BYTES` wire bytes the fixture already pins. That is a
multi-day, dedicated-ticket effort, not an extension of this one.

## Implementation status

Not implemented. No source files were edited by this pass.

## Verification (this pass, in the shared/contended `iso3` target dir)

- `cargo check -p semio-framework-os-kernel --message-format short` → **0 errors**.
- `cargo metadata --no-deps --format-version 1` → exit **0**.
- `cargo test -p semio-framework-ui-scene` → **108 passed; 0 failed** (one clean run; an earlier
  run in the same session spuriously reported 3 failures with no diff on our side — re-run came
  back 108/108, consistent with the concurrent build churn noted below, not a real regression).
- `cargo test -p semio-framework-value-derive` → **could not get a clean/deterministic run this
  session.** Six consecutive attempts produced three *different* failure signatures (`E0464
  multiple candidates for rlib dependency semio_framework_os_kernel`; `E0277 serde_json::Value:
  From<&DslValue> not satisfied` — despite that impl existing, unedited, in `🌱️value/🦀️.rs:218`;
  and once `cannot find attribute value in this scope` on the derive macro itself), with zero
  code changes on our end between attempts. Root-caused, not guessed: `git diff --stat -- Cargo.lock`
  showed active concurrent modification (110 insertions/2 deletions, `MM` status) during this
  session, and `ls` on the target dir's `debug/deps` showed **5 different hash-suffixed
  `libsemio_framework_os_kernel-*.rmeta`** files with mtimes 5 minutes apart, i.e. multiple
  concurrent agents rebuilding os-kernel against shifting dependency resolutions in the shared
  `iso3` target dir at the same time. This matches the ticket's own documented warning
  ("`iso3` target dir is heavily contended by concurrent agents") and is not attributable to this
  pass — no files were edited. Recommend re-running
  `cargo test -p semio-framework-value-derive` once workspace churn settles; the phase2 doc's
  prior 26/26 baseline was not contradicted by anything found here (no plausible source-level cause
  was found — `DslValue`, `🌱️value/🦀️.rs`, and the derive crate all read as expected on disk).
