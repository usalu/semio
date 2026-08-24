# glTF 2.0 `any` mutation oracle — assessment, decision and result

Artifact: `🧊️gltf` standard `🔖️2.0` subset `✳️any`. The outlier of the whole wave-7/8 effort: no
`pub enum GltfMutation`, vocabulary is a descriptor table (`GltfMutationLeafDescriptor`).

## Phase 1 — assessment

- **120 real mutation leaves** exist under `🧬️schema/🧬️mutations/` (133 directories minus `💾️binary`,
  `📝️text`, the 4 `🔒️*-private` helper dirs, and 4 stale pre-migration duplicates —
  `🌳️reparent-node`, `🔄️transform-node`, `🔗️bind-node-mesh`, `🔗️bind-primitive-material` — dead,
  2-file leftovers superseded by their own kebab-case directories, unreferenced anywhere in
  `📦️glue.rs`). Every one of the 120 has complete `🦠️mutation`/`🔺️diff`/`↩️inverse` files (checked all
  120 for missing subdirectories and for `todo!`/`unimplemented!`/TODO — zero hits) — not stubs.
- **7 are mounted as production modules in `📦️glue.rs`**: `create-scene`,
  `change-material-alpha-mode`, `change-material-double-sided`, `bind-node-child`,
  `unbind-node-child`, `bind-scene-root-node`, `unbind-scene-root-node`. All 7 already had a
  passing fixture test under `mod fixture_tests` in `🧬️mutations/🦀️component.rs`.
- **Only 3 were in `GLTF_MUTATION_LEAF_DESCRIPTORS`** before this ticket (`create-scene`,
  `change-material-alpha-mode`, `change-material-double-sided`). The other 4 mounted leaves each
  already had a complete root `🦀️component.rs` descriptor adapter (`plan`/`plan_inverse`/
  `apply_diff`/`apply_inverse`, `pub const DESCRIPTOR`) sitting unused — the array entry was simply
  never added. Mechanical, zero new logic: added the 4 missing entries to
  `GLTF_MUTATION_LEAF_DESCRIPTORS` in `🧬️mutations/🦀️component.rs`, bringing the registered/
  dispatchable vocabulary from 3 to 7. No `📦️glue.rs` edit needed (already mounted there).
- **Realistic increment**: exactly those 7 kinds, honestly declared. The other 113 leaves are real
  but unmounted in `📦️glue.rs` — that is ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION's own
  wiring surface, out of this ticket's scope (touching `📦️glue.rs`'s *production* mounts is
  explicitly a shared/contended concern this ticket's own fixture-test comment already calls out).

## Phase 2 — the oracle decision

Investigated (background agent, direct evidence, see conversation): is the `gltf` crate (1.4.1,
MIT) — already linked by `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/Cargo.toml` and the
root framework crate — reachable from this subset's own codec?

**No.** `GltfSnapshot`/`GltfDocument`/`GltfJson` (`../🧬️schema/📸️snapshot/🦀️component.rs`) never
name `gltf::` anywhere, no `impl From<gltf::…>`. `decode_glb`/`encode_glb`/`parse_gltf_document`
(`../🚪️io/🦀️component.rs`) are hand-rolled over `serde_json` alone. Every real `gltf::` call site in
the repository lives in `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs`
(`mesh_to_glb`/`mesh_from_glb`/`GlbExporter`/`GlbImporter` — byte-in/byte-out, no `gltf::` type
crosses that boundary), reached inside `semio-s-plugin-stdio` only through the unrelated BREP/DWG
mesh-IO codecs, never through this artifact's own tree. That is a small, nameable production
surface, structurally identical to the `image`/`png` `productionDebt` precedent — registering
`gltf` here with a `productionDebt` naming that one file would have been legitimate.

**Registered anyway, chose not to.** Linking `gltf` needs a `Cargo.toml` edit this ticket must not
make itself ("stop and report — I will batch it" was the offered path). Rather than block the whole
deliverable on that hand-off, used the already-linked `json` (json-rust) 0.12 instead — already in
the oracle crate's `oracles` feature list, already proven independent for `stdio.json`'s own RFC
8259 oracle (appears nowhere in this repository's production dependency graph), and this subset's
own codec is *also* ruled off `serde_json` for the identical reason `stdio.json` was: the snapshot's
own doc comment documents `serde_json::{from_str,to_vec}::<GltfDocument>` as its wire codec.

`json` is domain-BLIND (no glTF schema awareness), unlike `gltf`. Every one of the 7 kinds' real
semantics — index bounds, self-parent/cycle/duplicate-root rejection, `alphaMode` enum validity,
no-observable-change rejection, the `document/scene` remap `create-scene` performs — is
reimplemented from scratch in the oracle module against a hand-parsed GLB container (own 12-byte
header + `JSON`/`BIN\0` chunk walk, own chunk-bounds checks) and a plain `json::JsonValue` tree,
never this subset's own types. `json` supplies only the JSON tokenizer/serializer underneath that.
No `noOracleDecision` was needed — a real second producer exists (`@mode-differential`), it is
just independent of glTF-schema domain knowledge by design, exactly as the OBJ/STEP precedents used
a domain-aware-but-write-incapable reference plus a from-scratch writer.

## Phase 3 — the case

Real fixture: `📚️examples/🌱️metabolism/🖼️assets/🧊️base.glb` (284 KB, 271 nodes, 2 materials, 1 scene
with all 271 nodes as flat roots — no existing parent/child edge anywhere). One minimal,
real-data-preserving derivation was applied once (Python, committed alongside its own working):
node 1 moved out of the scene's 271-entry root list into node 0's own `children`, since 2 of the 7
kinds (`bind-node-child`/`unbind-node-child`) need an existing or creatable edge and the real export
has none. Every other byte — including the whole BIN chunk (skinning/mesh geometry) — is untouched.
Both the derived fixture (`local://🧊️base-with-nested-node.glb`,
`🧪️tests/mutate-gltf-2-0/🧫️fixtures/`) and the pristine real source stay committed, so the
substitution is auditable.

`create-scene` has no separate `delete-scene` catalog kind: production inverts it through the SAME
descriptor's own `phase: Inverse`, not a different command
(`🧬️mutations/create-scene/↩️inverse/🦀️component.rs`). The oracle mirrors that with its own
`undo_create_scene`, called directly by the case adapter for that one kind's inverse scenario
instead of routing through another catalog kind.

New files:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`
  — the oracle: independent GLB codec, independent JSON-tree mutation semantics, `project_gltf`.
- `.../🧪️oracle/🔣️component.json` — oracle registration (`json-rust-gltf-2-0-mutate`) + `gltf-2-0-any`
  catalog (7 kinds) + `semantic-gltf-v1` comparison profile.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧪️tests/mutate-gltf-2-0/component.feature` +
  `🦀️component.rs` — the case: 7×mutate, 7×inverse, 1×identity-round-trip.
- `.../🧪️tests/mutate-gltf-2-0/🧫️fixtures/🧊️base-with-nested-node.glb` — the derived fixture.

Edited files (both additive, mirroring the pattern every other wave-7/8 subset already used):
- `.../🧬️schema/🧬️mutations/🦀️component.rs` — added the 4 missing `GLTF_MUTATION_LEAF_DESCRIPTORS`
  entries, `pub const KINDS`, and a plain `#[test] kinds_match_registered_descriptors` (this file
  lives in the production crate, currently blocked from compiling by the unrelated os-kernel
  refactor per the fleet brief — could not run it this wave; audited by hand instead, KINDS and the
  catalog's `kinds` are identical 7-item lists by construction).
- `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/📦️lib.rs` — added this subset's own
  `pub mod gltf { … }` mount block (alphabetically between `gif` and `html`), the same per-artifact
  addition every one of the 38 prior subsets already made to this file.

## Verification (real output)

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-gltf-2-0
0 high-priority breach(es) across 0 rule(s)

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-gltf-2-0
[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts contract --owner 🗄️stdio        # repo-wide, unaffected
0 high-priority breach(es) across 0 rule(s)

$ cd ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust
$ cargo test --features oracles --lib
test result: ok. 138 passed; 0 failed; 1 ignored   # was 131 passed before this subset (+7)
```

15/15 = 7 mutate + 7 inverse + 1 identity-round-trip, all green. `parity=0/0` is expected and
honest: the subject phase does not compile this wave (unrelated os-kernel refactor, per the fleet
brief), so no case in the whole sweep claims a subject result — the subject half here is written and
`sut`-gated (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧪️tests/mutate-gltf-2-0/🦀️component.rs`,
dispatching each of the 7 real leaves' own `DESCRIPTOR` function pointers directly rather than the
full command-id/phase/envelope registry) but unverified.

## Honest limits

- Only 7 of 120 real leaves are covered — declared honestly, not inflated. Expanding requires
  mounting more leaves in `📦️glue.rs`, which is COMPOSE-TO-PUZZLE5D-MIGRATION's own surface.
- Subject phase unverified this wave (repo-wide blocker, not specific to this subset).
- The derived fixture's node-parenting is structurally representative, not from a real authored
  scene hierarchy — the real export (like most procedurally-generated architecture `.glb` files in
  this repository) has none. The substitution is minimal (one node moved) and auditable against the
  untouched real source committed alongside it.
