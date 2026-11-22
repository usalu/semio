# Carve Idempotence

Re-carving an already carved artifact root no longer plans a single `-schema` crate.

## What changed (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree`)

- `🧱️carved-layout`: reads the carved crates back (mirror nests, facade re-exports, moved-item regions, parent-file mounts), yields the declared root, file overlay, uncarved facade manifest and per crate owned declarations. A crate manifest now records every owned declaration (`[package.metadata.semio.carve] owned`), the schema-first carve record; crates carved before it count their nested mounts as owned.
- Planner: carved crates seed pinned groups (stable name, directory, description); only unowned slices are packed on top; pinned groups that form a ring (including manifest dependencies) merge, so the result is always acyclic. Mutation conversion and test rewrites do not run again.
- `♻️recarve-delta`: unchanged crates are not written, planned files equivalent to disk (TOML tables, order-free wiring, whitespace-free sources) are not written, superseded crates are deleted and dropped from workspace, router and lockfile.
- CLI prints `re-carve: N unchanged, N changed, N added, N superseded`.

## Tests

`🧪️tests/📦️crate-tree/♻️recarve` (language-agnostic `🔣️.json` + schema; tree-sitter and `@iarna/toml` as oracles): read-back module tree, zero writes on an unchanged re-carve, added source placed in a new crate with all other crates byte-identical and acyclic, cycle-closing source merges, next re-carve writes nothing. 6 pass.

## Real PDF root (dry-run only, nothing applied)

`repo ./📜️script.ts new crates ...📖️pdf/📦️packages/🦀️rust --dry-run` now plans against the carved layout, but is not yet usable there: 2 crates unchanged, 7 changed, 42 of 51 carved crates would be superseded (merged into 9) and the plan reports problems (`model` and `tests-2` over the hard budget, `tests-2` cannot be mirrored). `--verify --sandbox` therefore stops before the sandbox. Cause: the PDF sources were unmirrored and hand-edited, so cross-crate edges are `semio_*` extern paths the planner cannot see, and the recorded manifest does not exist for them; the merge ring pass then collapses crates. `io-export` is a hand-written crate outside the facade dependencies, so it is not read back; its declarations are not superseded by this run.

A real apply is therefore NOT recommended; next step is to carve the PDF once with the recorded manifest from a clean state, or teach the layout reader the unmirrored extern edges.

## Extern-edge attempt

The layout reader now re-points every `semio_*` provider path in the mounted sources back to `crate::` (the inverse of unmirror) so the planner sees the real cross-crate graph. Fixture tests still pass 6/6. The PDF dry-run is unchanged: 9 crates, 42 superseded, same three problems. So the collapse is not caused by the missing edges alone; the ring merge (manifest dependency edges of hand-edited crates plus the legacy nested-mount inheritance) still unions most crates. Not done: legacy-crate pin audit, `io-export` recognition, splitting `model`/`tests-2` (over budget only because they absorb merged crates), and the `tests-2` mirror error (`a::schema` is a file module that also owns `mutations::embed_font_file::tests` whose parent chain is cut elsewhere, a planner mirrorValid case that disappears once the merge stops). No `--verify --sandbox` run was possible.

## Ring instrumentation (PDF root)

- Legacy crates now own the declarations in the file closure of their mirror mounts (read from disk, so cross-crate children cut from a parent are not owned). Result: 41 pinned groups, 2 rings (15 and 5 groups) remain, still present with manifest edges removed, so they are source edges.
- Ring edges: the aggregate namespaces (`...::mutations`, `...::io::text`) of `mut-family-8/9`, `inferences`, `mut-base-*` reference their leaves in other crates, and leaf `tests` modules and leaves reference the aggregate/schema modules back (`a::schema::mutations::tests -> a::schema::mutations`, `base::io::text -> base::schema::mutations`). This is the aggregate/tests cycle that the fresh carve breaks by mutation-family conversion and test detaching; neither reproduces on the already converted sources, so the edges stay and the ring merge unions 15 crates. Enabling mutation families or test rewrites in the re-carve did not change it.
- Result after merge: 23 groups, 28 superseded, `inferences` and `tests-2` over the hard budget. Zero supersedes is not reached; `--verify --sandbox` not run; no regression fixture for the PDF ring shape was added.
- Next: make the re-carve treat edges into a pinned crate's aggregate namespace from `tests` modules as detached (as `detachAllTests` does for fresh carves) and ignore edges to namespace-only decls when the target crate already depends on the source crate in the manifest.

## Edge detaching

Group edges of pinned crates now skip edges from test-only components and source edges that contradict an existing manifest dependency (the manifest is acyclic by Cargo). PDF dry-run: 10 unchanged, 17 changed, 24 superseded (was 28), `inferences` and `schema-5` over the hard budget, `inferences` not mirrorable (`io::text` file module owns `snapshot::date::tests` whose parent chain is cut elsewhere). Zero supersedes not reached; the remaining unions most likely come from slice-level components that span several carved crates (`mergeCoupledComponents` merges source cycles before packing), which is the next place to detach. `--verify --sandbox` not run (plan still has problems); PDF ring-shape fixture not added. Fixture suite 6/6.

## Coupling pass (latest state)

Done in the planner: source edges are filtered before cycles merge (test-only slices, edges contradicting the manifest dependency order); legacy crates own the declarations of their mount closure with dependencies winning shared files; derived `pub use` leftovers in carved source files are stripped and provider paths re-pointed to `crate::`; test kids of a root namespace wrapper no longer union its other kids; pinned crates trust manifest reachability for group edges. PDF dry-run: 17 unchanged, 25 changed, 0 added, 9 superseded (was 28 at the start of this round, 51 collapsing to 9 before that). Fixture suite 6/6.

Remaining blockers: one component spans `modules::colour` (conformance-support), `modules::images` (mut-base-4) and `io::foreign_artifacts` (inferences) and unions those crates, which then ring with mut-family, mut-base-2/3, fonts and graph-projection (members of the 9); `mut-family-7` owns no mounts at all (re-export only) and so cannot be pinned; `graph-projection` exceeds the hard budget after the union. The component merge cause lies in the orphan/coherence pre-grouping (`collectComponentBlockers`), not yet instrumented. `--verify --sandbox` and the PDF ring fixture are therefore not done.

## Pre-grouping fix (latest)

Cause found: (1) a coherence merge of `modules::images` and `io::foreign_artifacts` (a `pub(crate)` item the carve had already widened) and (2) a source cycle `colour`/`images` between crates with no manifest relation. Both now yield to the pinned crates' manifests: coherence merges across two pinned crates are skipped, and a source edge between pinned crates only counts when the manifest reaches the target. Retained re-export-only crates keep their facade dependency row and feature forwarding. Unchanged carved crates exceeding the hard budget are noted, not blocked.

PDF dry-run: 31 unchanged, 18 changed, 0 added, 2 superseded: `model-profiles` (hand-written lib outside the carve, the intended superseded layer) and `mut-family-7` (its mounts are shared files owned by another crate, so it has no pinned slice). One problem remains: `conformance-support` test closure 33 files over the hard budget of 32, because four `v1_4 ... artifacts::{binary,deflate}` declarations are packed into it as unowned slices (they are mounted by several crates, so their owner is ambiguous). `--verify --sandbox` is blocked by that problem; the PDF ring fixture is not added. Fixture suite 6/6.

## Declarer ownership attempt

Assigning a shared module to the crate that declares it (its mirror mounts the file) was tried: it moved the failure (`schema-5` superseded, `mut-family-12` over the hard budget) instead of removing it, so the file-closure ownership with dependency-first tie-break stays. State: 31 unchanged, 18 changed, 0 added, 2 superseded (`model-profiles`, hand-written, intended; `mut-family-7`), one problem (`conformance-support` test closure 33 files vs 32). `io-export` is not a facade dependency, so this plan neither reads nor replaces it; the planned diff touches only the 18 changed carved crates, the facade root and manifest, workspace members, lock and router. `--verify --sandbox` and the PDF ring fixture remain open; the next step is a per-declaration owner record in the manifest (written by the carve), which makes shared-file ownership explicit instead of inferred.

## Owner record and workspace scope

The per-declaration owner record already exists as schema-first data: every carve writes `[package.metadata.semio.carve] owned` (all declarations of the crate) and the re-carve reads it, preferring it over inference. For the existing PDF crates the record can only be derived by writing their manifests, which is a live apply and was not done.

Extending the plan to every workspace member was tried in the dry-run and fails at once: hand-written crates (`io-export`, editor crates) reference assets outside the carve (`NimbusMonoPS-Italic.cff` is missing from the measured closure), so the planner cannot measure them. Reverted. Doing this needs an explicit decision: (a) allow writing the derived record into the live PDF crate manifests, and (b) say which hand-written crates the carve may delete, since planning them as providers supersedes (deletes) their manifests. State unchanged: 31 unchanged, 18 changed, 0 added, 2 superseded, 1 budget problem.

Last re-run after removing the experiments: 14 unchanged, 32 changed, 2 superseded, `graph-projection` and `conformance-support` over budget. The numbers differ from the 29/19/0 run before the experiments with unchanged planner logic, so the PDF tree (a shared worktree) changed between runs; the layout must be re-measured on a quiet tree before records are derived.

## Frozen snapshot results

Snapshot (path-length safe, outside the ticket folder): scratchpad `snap/` = copy of the PDF artifact tree, every other repository entry symlinked; plans run with `snap-plan.ts` (ticket `🗑️generated/carve`). Fixes that made it converge: a slice is pinned by its attach declarations first (a child eff shared with another crate no longer unions crates), and a pinned crate over the hard budget gets a second, tighter plan (`tight`) that releases test slices.

Snapshot dry-run: 48 planned crates, 28 unchanged, 0 superseded, 0 problems, 229 file updates (carved crates and their sources), 0 creates, 0 deletes. io-export and the other hand-written crates are untouched (out of scope by decision).

`--verify --sandbox` on the snapshot: module tree 0 problems, orphan 0, glue 0, unresolved crates 0. Two findings left: (1) the facade crate measures 177 lib / 234 test files (over the hard budget; its root still mounts kept modules and tests), (2) one visibility error: `mut-base-4` uses `foreign_artifacts::qualified_image_dictionary`, which is `pub(crate)`; the carve widens such items only when it splits them, and the re-carve skips the coherence merge between two pinned crates, so the widening to `pub` is missing. Both need a fix before an apply. Font asset folders (`NimbusMonoPS-Italic.cff` under `🖼️mono-italic`) only mattered for the rejected workspace-wide scope.

Owner records were NOT written into the live manifests: the plan is not yet clean (facade budget, visibility), so freezing the layout now would be premature. Regression fixture: the existing cycle case covers the ring merge; the PDF ring shape (manifest order contradicting a source edge) is not yet a fixture because trusting the manifest also hides a genuinely new dependency, which needs a decision.

## Decisions applied (new edges, widening)

- New source edges between pinned crates now count; only an edge that contradicts a manifest dependency (target already reaches source), test-only edges and cycles are detached or merged (rings merge minimally). Snapshot plan: 48 crates, 26 unchanged, 0 superseded, 0 problems, 234 updates.
- Cross-crate uses of `pub(crate)` items are widened to `pub` in the written source (recorded in the plan, applied after the cuts so offsets stay valid). Snapshot `--verify --sandbox`: visibility 0, module tree 0, orphan 0, glue 0, unresolved crates 0.
- Regression fixture `edge` added: a pinned crate that starts to use another pinned crate gains the manifest dependency, the crate graph stays acyclic, every other crate stays byte for byte, the next re-carve writes nothing. Suite 7/7.
- Open: the facade crate still measures 177 lib / 234 test files. Its root keeps about 170 file mounts below `standards/*/subsets` (the editor-mode wrappers added by hand after the carve); the planner leaves them in the facade, so the layered-facade split is not done. Owner records were therefore not written into the live manifests (the plan is not yet clean).

## Facade (open) and apply plan

The ~170 facade lib files are not editor-mode wrappers that the planner refuses to carve: on the snapshot the planned facade root (`carveArtifactRootText`) still contains full `#[path]` mount text for the `standards/4.1.4` and `7.1.7` subtrees (the root is rewritten, not unchanged), i.e. the rebuilt virtual root re-mounts files inline that the carved crates also own, and the root edits that replace those decls with `pub use` do not fire for them. Until that is understood, the planned facade is wrong and must not be applied. The next step is to compare the decl ids the planner attaches for those mounts with the ids of the rebuilt wrappers (`Rebuilder.wrapper`).

Exact snapshot plan (not for live apply): 234 file updates, 0 creates, 0 deletes: the facade root, facade manifest, workspace manifest, lock, router, the manifests/roots of the 22 changed carved crates, and source files whose derived `pub use` leftovers were stripped or whose `pub(crate)` items were widened. Owner records were not written to the live manifests.

## Facade fixed, owner records written (latest)

Cause of the ~170 mounts in the planned facade: the carved-layout reader (`🧱️carved-layout`, `Rebuilder`) rebuilds the declared root from the live facade (`facade()`), but never marked the inline wrappers it walked through (`pub mod standards { ... }`) as placed. The leftover scan then appended a second full copy of the `standards` wrapper (all `#[path]` mounts) at the root end. Attached declarations and wrappers were consistent; the root edits fired, but on the appended copy, while the first copy stayed untouched. Fixes (all in the reader): walked inline wrappers count as placed; a node counts as covered when any descendant is placed (`touched`); re-exports of a whole module and of glob-extended nodes cover their subtree (`cover`); private leaf modules owned by a crate (cfg(test) mounts) are not leftovers.

Regression fixture `nested` (language-agnostic data in `🔣️.json` + schema): components inside a root wrapper that holds further wrappers; the first re-carve writes nothing, the facade has exactly one wrapper, no `mod component;` mount and no component file path, and re-exports `demo_weave_io::standards::io::*`. The fixture fails with the fixes reverted (`update 🦀️.rs`). Suite 8/8.

Fresh snapshot (rsynced from the live PDF tree; the earlier snapshot had drifted): 48 crates, 27 unchanged, 0 superseded, 0 problems, 234 file updates, facade within the hard budget (census over hard: none). `--verify --sandbox` equivalent (`snap-verify.ts`): module tree 0, visibility 0, orphan 0, glue 0, unresolved crates 0.

Owner records: `[package.metadata.semio.carve] owned` written into the manifests of all 48 carved PDF crates (metadata only, inserted after `[package.metadata.semio]`, each manifest re-read right before writing). Hand-written crates (`io-export`, `model-profiles`, editor crates) were not touched.

## Live re-carve apply plan (NOT applied)

Planned from the fresh snapshot: 0 creates, 0 deletes, 234 updates: the facade root `🦀️.rs` and facade `Cargo.toml` of the PDF artifact, the workspace manifest and lock, the router, the `Cargo.toml` and `🦀️.rs` of the 21 changed carved crates (27 are unchanged and not written), and source files whose derived `pub use` leftovers are stripped or whose cross-crate `pub(crate)` items are widened to `pub`. Apply with `repo ./📜️script.ts new crates ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust` (re-run `--dry-run` first on a quiet tree; the tree drifts while others work).
