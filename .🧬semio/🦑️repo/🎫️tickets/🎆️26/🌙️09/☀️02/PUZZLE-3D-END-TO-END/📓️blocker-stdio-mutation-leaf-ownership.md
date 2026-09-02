# 🧩️ Puzzle 3d end-to-end — blocker analysis

## Symptom

`bun dev:puzzle:3d` dies before serving: `cargo build` of `semio-s-plugin-stdio` fails with 11
errors, which fails the flow-core wasm engine build (`flow` depends on `semio-s-plugin-stdio`, as
does `puzzle`), so no plugin or engine wasm can be produced at all.

## The 11 errors

Captured with `cargo check -p semio-s-plugin-stdio --message-format short` (twice, ~1h apart —
identical both times, so this is stable HEAD breakage, not a transient mid-write tree).

**4 × missing `include_str!` targets** — `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🦀️.rs:82-85`
references `🧬️mutations/{🟦️.ts,🔗️.graphql,🔣️.json,🛰️.proto}`, which no longer exist.

**7 × `error[E0080]` from `dsl::Mutations`** — "Mutations leaf source must match its aggregate
workspace and direct owner", at the base aggregate of each of: avi (`✳️hdrl`), bcf (`✳️markup`),
gif 89a (`✳️base`), dxf (`✳️header`), obj (`✳️geometry`), las (`✳️header`), gltf (`✳️any`).

## Root cause

Ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
(shards A6/A7, committed in `a807c0706c`) physically moved per-mutation leaf directories out of each
artifact's base subset into new satellite subset folders, and repointed the base aggregate's module
mounts at them: `#[path = "../../../✳️<satellite>/🧬️schema/🧬️mutations/<leaf>/🦀️.rs"]`. 43 such
cross-subset mounts across the six stdio artifacts, plus the gltf equivalent.

That structure cannot compile. `validate_mutation_leaf_source`
(`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:649`) requires, for every leaf an aggregate
wraps:

- `provenance.mutation_root == scope.mutation_root`, and
- `descriptor.owner` to be an **immediate child** of the aggregate's own mutation root.

A leaf's `PROVENANCE` is derived from its own physical file location
(`🗣️dsl/✨️derive/🦀️.rs:expand_mutation_leaf`), so a leaf sitting under a sibling subset can never
satisfy either. `workspace_token` and `taxonomy_path` are repo-global (both resolve through the root
`📋️project.json` → `📚️library/🔣️taxonomy.json`), so those two are NOT the failing fields — only
location is.

The shard's own note (`📓️a7-stdio-format-subsets.md`, "Compile evidence") records that a clean
`semio-s-plugin-stdio` compile was never obtained, and attributes the failure to an unrelated
pre-existing error in `semio-framework-plugin`. That attribution is wrong for the current tree: all
11 remaining errors are this shard's.

## Why the relocation was not needed

The gate the shard was closing — `unsplit-artifact-subset`
(`🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:4677`) — reads subset ownership from the
**mutation manifest**, not the filesystem:

```ts
export function owningSubsetOf(manifest, mutation) { return mutation.subset ?? manifest.subset; }
```

Its own remedy text says "Declare the real subsets … and give this mutation its smallest owner, or
record `subsetPolicy: single`". Declaring the satellite subsets and setting each mutation's manifest
`subset` field is sufficient; moving the Rust leaf directory is not, and is what breaks the derive.
The per-leaf descriptor `🔣️.json` carries no `subset` field at all — only `owner`, which must equal
its physical directory.

## Chosen fix

Restore the invariant without undoing the shard's semantic work:

1. Move the cross-mounted leaf directories back under their base subset's `🧬️mutations/`.
2. Rewrite each moved leaf descriptor's `owner` to its restored path.
3. Repoint the base aggregates' `#[path]` mounts to local paths.
4. Leave the satellite subsets, their `🧪️oracle/🔣️.json` manifests (`subset` fields) and the split
   feature files untouched — the gate stays satisfied at manifest level.
5. Restore the four gltf `✳️any/🧬️schema/🧬️mutations/` facet leaves the a6 script deleted.

Rejected alternatives: (a) giving every satellite its own aggregate enum (the PDF `PdfAMutation`
pattern) — correct for genuine conformance profiles, but it would drop `set-frame-delay`,
`set-usemtl`, … from the artifact's own mutation vocabulary, which is a functional regression for
formats that are one coherent document; (b) relaxing `validate_mutation_leaf_source` to accept
same-standard sibling subsets — a deliberate compile-time contract, not mine to weaken, and not
needed once (1)-(3) hold.

## Downstream finding (independent of the above)

Booting the app against the prebuilt (Sep 1) wasm with `SKIP_ENGINE_BUILD=1 … dev 3d served` on port
6081 shows puzzle3d loading its shell, example picker ("Nakagin Capsule Tower"), panels and tour —
then faulting on **every** actor turn:

```
{"origin":"plugin","code":"plugin.internal","severity":"error",
 "message":"runtime live cleanup faulted for instance 1","retryable":false}
```

render, readConflicts and `setActiveExample` all fail with it, so the viewport shows the raw fault
dump instead of a scene. Reconstructed worker stderr:

```
[DEBUG] cooperative-maintenance instance=1 turn=8 generation=1 status=1->3 entries=1
        phase=Some(51) clock=true pool=Some(CooperativePoolSnapshot { pump_calls: 8,
        selections: 1, no_selection: 7, selected_by_lane: [0,0,0,1,0,0], … })
```

`clock=true` rules out the missing-monotonic-clock path; `status=1->3` is QUEUED→FAULT on the single
entry (`entries=1`); `phase=51` decodes to session+outcome+**faulted**+**terminal**. So
`RuntimeLiveCleanupJob::step` (`🔌️plugin/🦀️.rs:28769`) returned `Fault`/`Cancelled` on its very first
step — i.e. `EditorApp::maintenance_step` (`:23595`) errored, or returned a `Pending` exceeding its
one-item/byte contract. The inner `JobFault` detail is swallowed by the outer
`plugin_internal_fault`, which is itself a diagnosability gap worth closing.

This cannot be diagnosed further or fixed against the prebuilt binary: `🔌️plugin/🦀️.rs` was
rewritten at 12:19 and 13:31 today, after the Sep 1 10:54 puzzle wasm was built. It needs a rebuild
— which is exactly what the stdio blocker prevents.

## Applied

Two idempotent scripts, kept in this ticket folder:

- `🔨️restore-leaf-ownership.py` — the six stdio artifacts whose leaves are mounted from the
  aggregate file itself. **43 leaf directories** moved back (avi 6, bcf 5, gif 89a 9, dxf 15, obj 2,
  las 6); each leaf's `🔣️.json` `owner` rewritten; each `#[path = "../../../✳️<satellite>/…"]` mount
  rewritten to `#[path = "<leaf>/🦀️.rs"]`; emptied satellite `🧬️schema/🧬️mutations/` directories
  pruned.
- `🔨️restore-crate-root-leaf-ownership.py` — gltf, whose leaf modules are mounted from
  `📦️packages/🦀️rust/🦀️.rs` instead. It reads each generated `pub mod mutations { … }` block, takes
  the subset of that block's `mod component;` mount as the aggregate's owner, and moves every
  sibling leaf mounted from a different subset. **120 leaf directories** moved back into
  `✳️any/🧬️schema/🧬️mutations/`.

Then:

- The four gltf aggregate facet leaves the a6 script deleted were restored from `a807c0706c^`
  (`🟦️.ts`, `🔗️.graphql`, `🔣️.json`, `🛰️.proto`). Their `./<leaf>/🟦️.ts` imports resolve again now
  that the leaves are back beside them.
- The eight gltf satellite subsets keep their own per-subset facet aggregates; their `🟦️.ts`
  imports were repointed from `./<leaf>/🟦️.ts` to
  `../../../✳️any/🧬️schema/🧬️mutations/<leaf>/🟦️.ts`. The other three facet formats carry no
  relative references and needed no edit.

Untouched, deliberately: every `🪆️subsets/🔣️.json` subset declaration, every satellite
`🧪️oracle/🔣️.json` (the `subset` fields those manifests declare are what the
`unsplit-artifact-subset` gate actually reads), every split `🥒️.feature` + adapter, and every
`🧫️fixtures/` tree. The satellite subsets remain declared and remain the manifest-level owners of
their mutations — only the Rust leaf directories moved.

## Verification

- `cargo check -p semio-s-plugin-stdio --message-format short` → **0 errors** (was 11). Confirmed
  downstream too: `libsemio_s_plugin_stdio-*.rmeta` is being produced and consumed as an `--extern`
  by `semio_s_plugin_puzzle`, `semio_s_plugin_norm` and `semio_s_plugin_gis` again.
- `nx run @semio-tech/repo-test-domain:test-contract` → `unsplit-artifact-subset` **0**,
  `wildcard-subset-owner` **0**, `duplicate-mutation-owner` **0**. The A6/A7 gate wins survive the
  move-back intact, as predicted by `owningSubsetOf` reading declared manifest ownership.
- The gate still exits non-zero on 1953 pre-existing repo-wide breaches (1190 testing/oracle, 443
  testing/contract, 316 testing/dependency, 3 discovery, 1 fixture) across many plugins. The 29 that
  touch the seven artifacts here are all `runtime-inventory-missing` and `binary-protocol-drift` —
  the exact classes `📓️a7-stdio-format-subsets.md` already recorded as pre-existing, and they
  include gif 87a, which was never split or touched.

The owning session (`semio-e3`) confirmed the fix stands and will not revert it, adopted
"declare ownership in the manifest, leave leaves next to their aggregate" as the rule for the rest
of their ticket, is auditing the five plugins they split the same way (note, draw, mathematical,
sequence, fem), and has taken back the step `payloadSchema` finding. They also identified why their
compile checks read clean: a `--keep-going` workspace run died in `semio-framework` (E0119
`ToValue`/`FromValue` on `ResourceSelector`) and never reached the plugins.
