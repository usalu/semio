# 📓️ `s.stdio.obj@3.0/✳️any` — reader-oracle retrofit

Scope: close this subset's one remaining gap for the wave — a genuinely independent READER oracle
for its 22-kind mutation vocabulary, distinct from the already-committed `tobj-obj-3-0-mutate`
(`kind: cross-semio-implementation`, which COMPUTES what a mutation should produce via its own
`🧪️oracle/🦀️component.rs` and must stay untouched). That file was NOT modified. This closes the
mismatch the brief described: every mutation's `oracleRequirements` already said
`qualifyingKind: "third-party-library"` but pointed `oracle` at the cross-semio id.

## What was built

```
🏭️generator/🦀️tobj-obj-reader/        NEW standalone crate, own [workspace], depends on tobj = "4" only
  Cargo.toml
  src/main.rs                          build <recipe-id> <out-dir> | project <path> | list-recipes
🏭️generator/📜️script.ts                EXTENDED: generate/manifests now dispatch by --only to either
                                        the untouched legacy pattern-shell path (../🦀️engine) or the
                                        new tobj-obj-reader corpus — a bare `generate` (no --only)
                                        still does ONLY pattern-shell, byte-identical to before
🔬️probes/📜️script.ts                   NEW: obj-import / obj-project / obj-compare
🧫️fixtures/<recipe>/{before,after}.obj 20 NEW recipe directories (32 files), + pattern-shell untouched
🧪️oracle/🔣️.json                       oracles +1, probes 0→3, comparisonPipelines []→1,
                                        mutationManifests[*].oracleRequirements retargeted (22),
                                        fixtureManifests +20 (pattern-shell entry untouched)
```

Ticket-local scratch (not part of the repository tree): `🗑️temp/obj-tobj-probe/` — a throwaway Rust
probe used to empirically determine real `tobj` 4 behavior (does it split models on `g` alone? on
`o`? does a bare reset line fall back correctly? are referenced texcoords/normals exposed with
`single_index`?) before any recipe text was written. Kept as an input script per the ticket rules;
its `target/` build output was removed.

## Why 12 of 22 kinds are witnessable and 10 are not — measured, not assumed

`tobj` is a MESH reader. Its `Mesh` output drops every `v`/`vt`/`vn` row no face resolves through
triangulation/single-indexing, and this crate's material loader is a no-op (`|_| Ok(Default::default())`,
matching the shared `mesh::project_obj`). I did not trust the prior research note's own "0 kinds
uncarried" conclusion for this — that note's 0-uncarried finding was against `oracle_document_projection`,
a SEPARATE Rust parser registered under the `cross-semio-implementation` oracle, not against `tobj`
itself. For a genuine third-party READER I built the reader crate first and empirically tested real
`tobj` 4 behavior on hand-written OBJ snippets (`🗑️temp/obj-tobj-probe`) before writing a single recipe:

* `g`-only documents DO split into separate `tobj::Model`s, named by the group name (`g alpha` / `g
  beta` → 2 models `"alpha"`/`"beta"`). A bare `g` reset line falls back to `tobj`'s own
  `"unnamed_object"`. Two `g alpha` bands with no reset between them merge into ONE model.
* `o` behaves identically (confirmed with `o alpha` / `o beta` and a bare `o` reset).
* Referenced `vt`/`vn` rows ARE exposed per-corner when `single_index: true` (unlike the SHARED
  `mesh::project_obj`, which only reports positions+triangles for cross-format comparability — my
  reader's own projection is richer since it exists only for this one format).
* `usemtl` with no resolvable `mtllib` file causes NO model split (confirms `set-usemtl`/`set-mtllib`
  are genuinely invisible here, not just untested).
* Smoothing group (`s`) values have zero effect on `tobj`'s output.

That gave 12 witnessable kinds, each then confirmed for REAL on the actual committed before/after
fixture pair (not merely reasoned about):

```
$ tobj-obj-reader project <before.obj> vs project <after.obj>, for each of the 12:
no-mutation-no-op        IDENTICAL   (correct — the invariant a no-op must hold)
set-snapshot-applied     DIFFERS
set-vertex-applied       DIFFERS
set-texcoord-applied     DIFFERS
set-normal-applied       DIFFERS
insert-face-applied      DIFFERS
remove-face-applied      DIFFERS
set-face-applied         DIFFERS
set-group-applied        DIFFERS  (modelCount 2→1, "alpha"+"unnamed_object" merge into "alpha")
remove-group-applied     DIFFERS  (2nd model's name "beta"→"unnamed_object")
set-object-applied       DIFFERS  (modelCount 1→2, "alpha" splits into "alpha"+"beta")
remove-object-applied    DIFFERS  (2nd model's name "beta"→"unnamed_object")
```

The other 10 (`insert-vertex`, `remove-vertex`, `insert-texcoord`, `remove-texcoord`, `insert-normal`,
`remove-normal`, `set-mtllib`, `set-usemtl`, `set-smoothing-groups`, `set-unknown-statements`) keep an
`oracleRequirements` naming `obj-3-0-mutate-uncarried` with NO `oracle` field — the exact convention
already used repo-wide (confirmed against `gltf@2.0/✳️any`, `mathematical@1/any`, `sequence@1/any`,
`fem2d@1/any` — 100 pre-existing occurrences of the identical `testing/oracle` "requires a
third-party-library for capability X-uncarried, and none is registered" finding before this session
touched anything).

**Note on ADD/REMOVE vertex-family kinds specifically**: these are uncarried not because the target
row happens to be unreferenced in some particular fixture, but structurally — an isolated
insert/remove of a `v`/`vt`/`vn` row never creates or removes a face reference on its own (that's a
different mutation kind), so the row stays unreferenced and invisible to any pure mesh reader
regardless of where in the list it's inserted. `set-vertex`/`set-texcoord`/`set-normal`, by contrast,
replace an EXISTING row in place — no index-shift ambiguity — so targeting a row a real face
references makes them cleanly witnessable, which is what their recipes do.

## Fixtures — 20 new recipe directories, hand-authored grammar (never `encode_obj`)

12 `<kind>-applied` (before.obj + after.obj) + 8 `<kind>-rejected-<reason>` (before.obj only), for
every kind whose real dispatch has a traced, code-confirmed rejection path
(`🧬️schema/🔺️diff/🦀️component.rs`'s `validate_indexed_targets`/`validate_named_targets`):
`set-vertex`/`set-texcoord`/`set-normal`/`set-face`/`insert-face` (index out of range),
`remove-face`/`remove-group`/`remove-object` (target missing). **No `-rejected-` fixture for
`set-group`/`set-object`/`set-snapshot`** — traced `diff_set_group`/`diff_set_object`'s own
`validate_named_targets` call and found no path the real `ObjMutation::diff` call site can actually
trigger (both are upsert: `existed` is read off `base` itself before the diff is built, so the
"duplicate add" error condition is unreachable from a real mutation); `set-snapshot`'s rejection
condition was not traced (out of scope given time — a real, stated gap, not silently dropped).

Every byte was admitted through the real `tobj` 4 (the exact registered crate) before being written —
`tobj-obj-reader`'s own `admit_or_panic` — mirroring `../🦀️engine`'s already-established precedent for
`pattern-shell.obj`. A `cargo test` over the crate confirms all 20 recipes admit (`every_recipe_id_resolves_and_before_admits`, 1 passed).

## Oracle registration (`🧪️oracle/🔣️.json`, additive)

* New oracle `tobj-obj-3-0-mutate-reader`, `kind: "third-party-library"`, package `tobj` 4. The old
  `tobj-obj-3-0-mutate` entry (`cross-semio-implementation`) is byte-for-byte untouched.
* 3 new probes: `obj-import`, `obj-project`, `obj-compare` — each with a `qualification` block citing
  real measured evidence from this session (see below).
* 1 new `comparisonPipelines` entry, `obj-3-0-tobj-compare-v1` (GATING): `obj-import` asserts
  `bothImport: true`, `obj-compare` asserts `equal: true`.
* All 22 `mutationManifests[0].mutations[*].oracleRequirements` retargeted: 12 kinds now point at
  `tobj-obj-3-0-mutate-reader`; 10 kinds now say `capability: "obj-3-0-mutate-uncarried"` with no
  `oracle` field.
* `fixtureManifests` grew from 1 to 21 (pattern-shell entry byte-identical, never touched).

## Verified with the real framework commands — actual output, quoted

```
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture verify --artifact s.stdio.obj --standard 3.0 --subset any
[fixture verify] 21 fixture(s), 0 file problem(s)

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture audit --artifact s.stdio.obj --standard 3.0 --subset any
[fixture audit] 21 fixture(s), 0 with contract problems
```

`fixture reproduce` — run ONCE PER FIXTURE (21 separate invocations, `--mutation <m> --outcome <o>`
selectors, never a whole-corpus batch — the reproducibility note's own lesson 2 about order-dependent
generator state):

```
no-mutation/no-op                    -> [fixture reproduce] 1 generated fixture(s), 0 problem(s)
set-snapshot/applied                 -> 1 generated fixture(s), 0 problem(s)
set-vertex/applied                   -> 1 generated fixture(s), 0 problem(s)
set-vertex/rejected                  -> 1 generated fixture(s), 0 problem(s)
set-texcoord/applied                 -> 1 generated fixture(s), 0 problem(s)
set-texcoord/rejected                -> 1 generated fixture(s), 0 problem(s)
set-normal/applied                   -> 1 generated fixture(s), 0 problem(s)
set-normal/rejected                  -> 1 generated fixture(s), 0 problem(s)
insert-face/applied                  -> 1 generated fixture(s), 0 problem(s)
insert-face/rejected                 -> 1 generated fixture(s), 0 problem(s)
remove-face/applied                  -> 1 generated fixture(s), 0 problem(s)
remove-face/rejected                 -> 1 generated fixture(s), 0 problem(s)
set-face/applied                     -> 1 generated fixture(s), 0 problem(s)
set-face/rejected                    -> 1 generated fixture(s), 0 problem(s)
set-group/applied                    -> 1 generated fixture(s), 0 problem(s)
remove-group/applied                 -> 1 generated fixture(s), 0 problem(s)
remove-group/rejected                -> 1 generated fixture(s), 0 problem(s)
set-object/applied                   -> 1 generated fixture(s), 0 problem(s)
remove-object/applied                -> 1 generated fixture(s), 0 problem(s)
remove-object/rejected               -> 1 generated fixture(s), 0 problem(s)
pattern-shell (no-mutation/no-op, the pre-existing fixture) -> 1 generated fixture(s), 0 problem(s)
```

21/21, 0 problems, each fixture regenerated in its own fresh `SEMIO_FIXTURE_OUT` root by its own
invocation. (A first attempt used `--only <id>` on the outer test CLI, which that command does not
accept — it silently ran the whole corpus every time and my generator script did not yet support the
new recipes at all, producing 32 "generator produced no X" problems. Root-caused, fixed by adding
proper `generate --only <recipe-id>` support to `📜️script.ts` reading `SEMIO_FIXTURE_OUT`/`--out`, and
switching to the framework's real `--mutation`/`--outcome` selectors — the ones the brief itself
named.)

`matrix --json`, `measurements[]` entries, filtered for `s.stdio.obj` in each dimension's `missing`:

```
subsetOwnershipCoverage        658/658  obj-missing: []
externalOracleCoverage         431/658  obj-missing: []
oracleEvidenceCoverage         351/658  obj-missing: []
oracleCapabilityCoverage        30/54   obj-missing: []
fixtureProvenanceCoverage      705/705  obj-missing: []
fixtureReproducibilityCoverage 705/705  obj-missing: []
runtimeMutationCoverage         30/68   obj-missing: ["s.stdio.obj@3.0/any (no runtime inventory)"]
```

The four release-gated dimensions the brief names all show an empty `obj`-missing list.
`runtimeMutationCoverage` still lists this subset — expected, unrelated to this work: the runtime
inventory needs the production bridge (`semio-s-plugin-stdio`) to compile, and that crate's in-flight
peer refactor was never touched (confirmed not needed: both the reader crate and the generator's
existing `../🦀️engine` are standalone `[workspace]`s, `cargo build --release` succeeds cleanly for
both, no dependency on the production crate at all).

**Compare probe, demonstrated BOTH ways, real JSON quoted:**

```
$ bun 🔬️probes/📜️script.ts obj-compare --input .../set-vertex-applied/after.obj --input .../set-vertex-applied/after.obj
{ "status": "ok", "measurements": { "equal": true, "diffCount": 0, "diffs": [] } }

$ bun 🔬️probes/📜️script.ts obj-compare --input .../set-vertex-applied/before.obj --input .../set-vertex-applied/after.obj
{ "status": "ok", "measurements": { "equal": false, "diffCount": 1,
  "diffs": ["$.models[0].positions[1][2]: 0 ≠ 5"] } }
```

(`set-vertex-applied`: `SetVertex{index:1, vertex:{x:1,y:0,z:5}}` on a vertex both declared faces
reference — before has `v 1 0 0`, after has `v 1 0 5`; the diff names the exact resolved-position
field that moved.)

`bun 🧰️framework/…/🧪️test/📜️script.ts contract` (repo-wide, exits 1 — pre-existing, unrelated
breaches across dozens of other subsets under construction by concurrent sessions). Every line this
session's own change is responsible for:

* 10× `testing/oracle … requires a third-party-library for capability obj-3-0-mutate-uncarried, and
  none is registered` — the exact, expected shape of the `-uncarried` convention (identical wording
  appears 100 times repo-wide for other subsets already using it, `gltf@2.0/✳️any` included).
* 1× `testing/oracle … tobj-obj-3-0-mutate-reader is registered as a qualifying third-party oracle,
  but this owner predicts mutation output in its own Rust` — the SAME finding, word-for-word modulo
  the id, that `riff-avi-1-0-mutate-reader` (avi), `three-gltf-2-0-mutate-reader` (gltf),
  `quick-xml-1-0-mutate-reader` (xml), `jszip-bcf-2-1-mutate-reader` (bcf) and
  `jszip-docx-ecma-376-mutate-reader` (docx) all already carry — a known, accepted limitation of that
  gate (it flags any `third-party-library` oracle sharing a directory with a predicting
  `component.rs`, regardless of whether that specific oracle is the one predicting), not something
  introduced or fixable here.
* 22× `testing/contract … Mutation X is owned by "any" and s.stdio.obj@3.0 declares no narrower
  subset at all` (`unsplit-artifact-subset`) — pre-existing, already documented in
  `📓️obj-3-0-any-fixture-corpus.md` as deliberately not silenced (OBJ has a genuine polygonal/
  free-form split this 22-kind vocabulary only covers half of; declaring `"subsetPolicy": "single"`
  would assert something false). Unchanged by this session.

No new `testing/contract` (blocking) breach traces to any file this session touched; `grep`ing the
full contract output for `obj-3-0-tobj-compare-v1`, `obj-import`, `obj-project`, `obj-compare` and
`tobj-obj-3-0-mutate-reader` returns exactly the one expected `reimplementation-registered-as-third-party`
line above — no malformed-pipeline or malformed-probe complaint.

## Open / not verifiable this session

1. **No `-rejected-` fixture for `set-group`/`set-object`/`set-snapshot`** (see Fixtures section) —
   stated rather than papered over.
2. **`runtimeMutationCoverage` stays un-produced** for this subset — needs
   `semio-s-plugin-stdio` to compile; that crate's peer refactor is out of scope and was not touched.
3. **`.vscode/launch.json` was not touched** — neither this generator nor any of ~15 sibling
   generators is registered there yet, and both `.vscode/launch.json` and
   `.vscode/🧩️launch.seed.jsonc` were already flagged by a prior session (`spawn_task
   task_7f6b0695`) for one coherent pass over the whole family; redoing that here piecemeal would
   conflict with it.
4. A concurrent peer session is mid-refactor of
   `🧬️schema/🧬️mutations/📄set-snapshot/{↩️inverse,🔺️diff,🦠️mutation}/*` in this same subset
   (visible in `git status` as deletions/additions this session did not make). Not touched, not
   waited on, per the live-concurrent-devs rule.
