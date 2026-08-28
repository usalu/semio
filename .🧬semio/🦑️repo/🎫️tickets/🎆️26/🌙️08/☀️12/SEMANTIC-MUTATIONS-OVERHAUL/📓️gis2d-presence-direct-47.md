# Gis2d Presence Direct Leaf 47

## Result

`Gis2dPresenceMutation` is now a transparent one-leaf aggregate over `SetCamera { camera_json: String }`. The former manual `Snapshot { presence }` replacement and whole-record `Gis2dPresence` diff are gone. `SetCamera` has semantic kind and text opcode `set-camera`, binary tag `0`, exact explicit inverse to the prior camera string, and the existing `mutation.no-op` warning code/message for an identical requested camera. Its sparse ordered diff uses `Gis2dPresenceDiff { steps }`; an empty step list is the identity.

The aggregate passes the full record line to the generic Dsl record parser/printer. It does not strip or manually restore a keyword.

## Sidecar reconciliation

The four stale sidecars were reconciled to the actual camera-only state. The removed stale fields were `selectedIds`, `featureSelectionJson`, `hoverJson`, `selectionMethod`, and `selectionMode` (snake case in Proto). Proto keeps `camera_json = 2` exactly; no compatibility or reserved alias was added. Production and Rust-schema serde now require `cameraJson` and reject extra fields. Explicit `Default` remains for empty artifact bootstrap through the existing artifact text/pack codec.

## Exact source scope

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🛰️component.proto,🔣️component.json}`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🔺️diff/{🦀️.rs,🔣️.json}`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🧬️mutations/🎥️set-camera/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🧪️tests/{🔣️vectors.json,🧬️schema/🔣️.json}`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🧬️mutations/🦀️.rs`

No renderer, runtime, gismap-config, GIS terrain, shared interaction, launch, or seed source was changed. The only pre-existing external typed join is the unchanged `Gis2dApp::PresenceMutation` associated type; the scoped gismap search found no old `Snapshot` constructor join to rewrite.

## Schema-first evidence

The first red run was retained at `🧪️gis2d-presence-direct-47/🧫️runs/1787858645706/📄failure.txt`: the expected direct mutation tree did not exist before implementation. It is an absence red, not a claim about a compiled native behavior.

The domain-owned fixture at `presence/🧬️schema/🧪️tests/🔣️vectors.json` is itself validated by its adjacent draft-2020 schema and is consumed by both the controller and authored native tests. It contains genuine camera JSON strings, a separately labeled opaque-string edge (the production field remains an unconstrained `String`), strict state/payload/aggregate cases, sparse diff missing/null/ordered cases, and fixed expected identity, no-op, absorb, outcome, and inverse results.

The controller derives the repository root from its own `import.meta.url`, verifies the workspace root itself is a canonical non-symlink, validates every source path with ancestor nofollow checks, first-hashes the controller plus every schema/source/test input, and rereads all of them before returning. It uses jsonc-parser as an independent JSON reader, Ajv 2020 against the actual state/payload/aggregate/diff schemas (including the aggregate `$ref`, not an in-memory replacement), and draft-07 Ajv against the authoritative descriptor schema. It separates 46 behavioral assertions from structural/no-follow/hash assertions.

The first attempted controller correction failed only because the same Ajv instance registered an already-added schema twice; the retained `🧫️runs/1787859026070/📄failure.txt` is a harness error, not a pre-change schema red. There is no retained pre-fix native execution for the nullable-diff mismatch. A later retained source-only RED at `🧫️runs/1787860254887/📄failure.txt` proved the native helper still selected the removed `fixture["envelope"]` key. The GREEN below follows the exact native-test fixture repair; it is still not native execution.

Executed command:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gis2d-presence-direct-47/📜️script.ts
```

It passed with `1019` total assertions, including `46` behavioral assertions. Retained result: `🧪️gis2d-presence-direct-47/🧫️runs/1787860314691/🔣️result.json`.

## Native test roster

Authored but not executed (no Cargo or rustc was run):

- `strict_state_and_payload_vectors_match_the_direct_camera_contract`
- `direct_payload_metadata_text_binary_and_inverse_match_neutral_fixture`
- `sparse_camera_diff_has_an_empty_identity_and_preserves_the_no_op_warning`
- `sparse_camera_diff_serde_order_noop_and_codec_rejections_match_neutral_fixture`
- leaf-local `direct_payload_metadata_text_binary_and_inverse_match_neutral_fixture`

These tests now consume the one domain fixture and cover strict state/payload serde, aggregate fixture selection, missing/null diff serde normalization and identity application, ordered application, no-op warning, invalid text type, malformed/truncated/trailing binary rejection, and the existing canonical text rejection. They are authored only; no Cargo, rustc, or native runner was invoked. This remains source-only and is not source-readiness acceptance. Suggested registered filter: `gis2d_presence` plus the five exact test names above after the owning root schedules the native slot.

## Post-change source hashes

| Path | SHA-256 |
| --- | --- |
| `presence/🦀️component.rs` | `52b39bdf16be84c81c2658ebedc03f639c771bea8d4189d65a56eee3ebc8b9d9` |
| `presence/🧬️schema/🔺️diff/🦀️.rs` | `250f3ee91326857c10bdf827a0b44ae3100ea5f01f54bc581bfd90525273b5b3` |
| `presence/🧬️schema/🧬️mutations/🦀️.rs` | `21c603943b1b59a614708c5b2cdadff45c8512e21984160ededb6e6324c1830e` |
| `presence/🧬️schema/🧬️mutations/🎥️set-camera/🦀️.rs` | `865b1c93a1388862f9f9b8fdc115a5bddb05d3c631e36ee947f95d6f2facb65f` |
| `presence/🧪️tests/🧬️mutations/🦀️.rs` | `7dd6564e41fb398da4e1892958c612a5fd7ec0dbf7f711487ab0657491bbf1fd` |
