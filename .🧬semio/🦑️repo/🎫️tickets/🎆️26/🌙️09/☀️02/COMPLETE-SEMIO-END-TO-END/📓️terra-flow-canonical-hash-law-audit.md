# Flow Canonical Hash-Law Audit

## Scope and evidence

Read-only review after native build `60581` compiled the Flow lib and its first registered law failed at `editor/🧵️retained/🗿️artifact/🦀️.rs:293`. No Flow law was rerun by this audit.

The selected gate is exactly eight FQNs in [Flow script](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:32). The selected functions are real exact test names; this is not a filter-vacuity finding.

## Historical Source RED: two production hash encoders disagreed

`flow_content_child_handle_bounded` constructs typed `DslValue`, converts it to `serde_json::Value`, and writes those bytes into the SHA-256 stream [Flow artifact](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:172). This repository uses `serde_json` without `preserve_order`; conversion therefore canonicalizes every JSON object by lexical key order. The retained `SceneHash`, however, reads `ArtifactCanonicalJson` [retained artifact](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/🦀️.rs:170), whose local object builder preserves its supplied field order.

The pre-repair retained traversal had three material deviations:

- Its documentation claims “serde declaration order,” and `widget` emits `input_ports`/`output_ports`, not typed DSL’s `inputPorts`/`outputPorts` [canonical traversal](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🧾️canonical/🦀️.rs:1) [widget fields](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🧾️canonical/🦀️.rs:79). The runtime `Widget` authority declares camelCase [widget authority](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifact/🦀️.rs:213).
- The retained helper supplies declaration order at every object, while direct typed DSL is rendered through default `serde_json::Map` lexical ordering. The root working-scene object is one visible instance [retained scene](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🧾️canonical/🦀️.rs:125).
- `neural::Neuron::to_value` always emits `tree`, using `null` for `None` [neural value authority](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:913). The retained traversal omits absent `tree` [retained neuron](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🧾️canonical/🦀️.rs:43). This copied a test-only serde omission rule, not the production DSL rule.

`neural::Dictionary::iter()` is safe to use for the corrected traversal: it is an AVL lexical key iterator [dictionary](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:73) backed by the ordered-map lexical comparator [ordered map](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:98). It agrees with default `serde_json` object-key order; it must not be replaced by insertion order.

## Historical Required Repair

Make the retained `ArtifactCanonicalJson` traversal produce precisely the typed-DSL-to-default-`serde_json` bytes which `flow_content_child_handle_bounded` hashes. In particular, lexically order every fixed object array before exposing it; retain AVL order for dynamic dictionaries/maps; use camelCase field names; and emit `tree:null` for an absent neural child. Update the source comment accordingly. Do not alter the hash domain, accept snake_case aliases, or make the tests compare semantic JSON values instead of bytes.

The pre-repair content fixture encoded `input_ports`/`output_ports` in several rows [identity fixture](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🪪️content-identity/🔣️.json:11); its schema treats `canonicalJson` as opaque text [identity schema](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🪪️content-identity/🧬️.schema.json:54), so the changed canonical strings and SHA-256s needed to land atomically with the traversal.

The independent Bun oracle currently hashes the committed text but only spot-checks `kind` and nested dictionary presence [fixture oracle](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/📜️script.ts:261). It should additionally reject snake_case port keys and reject any non-lexical canonical object order. Add at least one content-hash vector with multiple dictionary keys and a cluster carrying an inner neural `tree:null`; the present five vectors do not cover either. The fixed Rust test should compare the emitted retained bytes (or its fixture-pinned digest at each grant) to those neutral strings, not `serde_json::to_string(ToValue(root))` as a vague bridge.

## Selected-Law Audit Before Repair

| Selected law | Current canonical-byte exposure | Classification |
| --- | --- | --- |
| `scene_identity_matches_node_crypto_and_adopts_the_exact_root` | Direct obsolete typed-DSL/fixture string equality at line 293, then direct-handle and retained-reader SHA paths. | RED; primary detector. |
| `flow_parent_projection_and_child_identity_match_neutral_corpus` | Reuses the same fixture digest through direct child-handle construction. | Must receive new fixture hashes; no separate serializer assumption. |
| `flow_store_owners_retire_all_durable_lanes_with_neutral_byte_grants` | Store close accounting only. | No typed-DSL byte comparison. |
| `flow_presence_store_owners_preserve_readers_and_retire_neutral_byte_grants` | Presence retirement/readers only. | No typed-DSL byte comparison. |
| `flow_empty_transient_close_matches_neutral_trace_and_exact_owner` | Transient owner generation/terminal behavior only. | No typed-DSL byte comparison. |
| `flow_viewer_member_factory_and_full_store_close_match_neutral_contract` | Viewer manifest/owner closure only. | No typed-DSL byte comparison. |
| `flow_actual_surface_factories_close_all_owners_under_neutral_grants` | Actual `plugin()` factory/closure only. | No typed-DSL byte comparison. |
| `flow_render_fixture_projection_retires_populated_and_rejected_pages` | UI fixture JSON projection and page retirement only. | No Flow DSL content hashing. |

The unselected canonical-module unit tests compare the retained renderer against `ToValue` rendered through `serde_json` [canonical tests](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🧾️canonical/🦀️.rs:170). They are useful Rust parity checks, while the expanded selected canonical laws provide the current focused execution boundary.

## Verdict

## Current-Byte Supersession — 2026-09-04

The RED described above was reproduced before the canonical repair. It is now source-closed in the live tree:

- The retained encoder's fixed-object helper sorts keys lexically, matching the default `serde_json::Map` traversal used by the direct typed-DSL hash path [canonical encoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🧾️canonical/🦀️.rs:11).
- Flow port fields are now `inputPorts` and `outputPorts`, and an absent neural subtree is represented as `"tree":null`, matching `ToValue` rather than the old test-only serde omission behavior [canonical encoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🧾️canonical/🦀️.rs:43).
- The content fixture and independent Bun oracle now pin recursive lexical object order, nonempty camelCase port fields, and a nested null-tree leaf. The oracle uses the repository's first-party BLAKE3 helper; it does not claim a WebCrypto BLAKE3 implementation [fixture oracle](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/📜️script.ts:261).
- The focused native selector has been expanded from eight to ten laws, adding full-variant and Unicode canonical-byte parity. This is registered source coverage, not an observed terminal native result.

The original diagnosis remains historical evidence for why the representation had to change; it is not a current source RED. Runtime acceptance remains pending the expanded ten-law terminal. The only residual coverage recommendation is to retain at least one multi-sibling dictionary ordering vector in the language-neutral corpus; this is bounded strengthening, not evidence that the repaired encoder is presently divergent.

**Historical RED, source-closed; runtime pending.** The native compiler previously reached assertions and exposed the direct-handle versus retained-SceneHash serialization split. The current source repairs that split, but no successful expanded ten-law rerun was observed by this audit.
