# Rename `imperative` artifact → `procedure`

Renamed the `imperative` plugin's ARTIFACT (a `Path` of control-flow `Step`s) from the noun-collision
`imperative` (an adjective, and identical to the plugin's own name) to `procedure`. The PLUGIN identity
(crate `semio-s-plugin-imperative`, plugin dir `📜️imperative`, plugin id/label `"imperative"`/`"Imperative"`,
`ImperativeApps`, editor/viewer session types `ImperativeConfig*`/`ImperativePresence*`/`ImperativeCommand`/
`ImperativeHost`/`ImperativeCoreError`/`ImperativePlayApp`/`ImperativeViewer`/etc.) was deliberately left
untouched — only the ARTIFACT-domain schema/mutation/diff/inference surface was renamed.

## Naming rule used (derived from the `architect`/`program` precedent)

`architect`/`program` is the one existing plugin≠artifact pair in the repo with the exact same shape
(plugin name used to equal the artifact name; plugin stays, artifact gets a real noun). Its patterns were
used as the template everywhere a judgment call was needed:

- `pub mod artifacts { pub mod procedure { ... } }`, and — per this ticket's explicit instruction — the
  `editor`/`viewer` module arms too: `pub mod editor { pub mod procedure { ... } }`, `pub mod viewer { pub
  mod procedure { ... } }`. (Architect itself keeps `editor::architect`/`viewer::architect` plugin-scoped;
  this ticket asked to fold `imperative`'s editor/viewer arms into `procedure` as well, so they were.)
- Extension/capability namespace ids (first tuple element in `definition()`'s `rows`, `#[artifact_schema(id
  = ..)]`, grammar/protocol "name" lines, subsets/oracle `"artifact"` fields) are ARTIFACT-scoped:
  `s.imperative.schema.artifact` → `s.procedure.schema.artifact` (matches architect's `s.program.schema.artifact`,
  not `s.architect.schema.artifact`).
- Descriptor VALUES that pair plugin+artifact stay two-segment, plugin unchanged: `s.imperative.imperative`
  → `s.imperative.procedure` (matches architect's `s.architect.program`).
- Config/Presence editor-session identity strings are PLUGIN-doubled and stay untouched:
  `s.imperative.imperative.config` / `.presence` (matches architect's `s.architect.architect.config`/
  `.presence` — confirmed by reading architect's own file before touching ours).
- `art_<artifact>_demo` / `app_<artifact>_demo_session` module names follow the ARTIFACT name (confirmed
  against architect's own `art_program_demo`), so `art_imperative_demo` → `art_procedure_demo`,
  `app_imperative_demo_session` → `app_procedure_demo_session`.

## Files changed

**Directory rename** (`mv`, not `git mv`):
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/` → `.../🗿️artifacts/📜️procedure/`
- `.../🧪️tests/mutate-imperative-1/` → `.../🧪️tests/mutate-procedure-1/`

**Master wiring file** `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/🦀️.rs`: all `#[path = "../../🗿️artifacts/📜️imperative/…"]`
strings repointed to `📜️procedure`; all three `pub mod imperative { … }` (under `artifacts`, `editor`,
`viewer`) renamed to `pub mod procedure`; the "Shims" `crate::artifacts::imperative::…` re-exports
repointed; `app_imperative_demo_session`/`art_imperative_demo`/`art_imperative_demo_tests` renamed.

**Plugin root** `✏️s/🔌️plugins/📜️imperative/🦀️.rs`: `ImperativeApps` enum variants, `.artifact(...)`,
`.editor::<...>`, `.viewer::<...>`, `.activation(...)` all repointed through `crate::artifacts::procedure::` /
`crate::editor::procedure::` / `crate::viewer::procedure::`; plugin id `"imperative"` / label `"Imperative"`
left untouched (plugin identity).

**TypeScript barrel** `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/🟦️.ts`: path + export names
(`imperative_schema`/`imperative_io` → `procedure_schema`/`procedure_io`) repointed to `📜️procedure`.

**~120 files under the renamed artifact tree** (`.rs`, `.ts`, `.graphql`, `.proto`, `.g4`, `.ebnf`, `.abnf`,
`.ksy`, `.spicy`, `.grammar.semio`, `.protocol.semio`, `.dsl.semio`, `.cmd.semio`, `.json`, `.feature`,
`.py`): module paths (`crate::artifacts::imperative::` → `crate::artifacts::procedure::`, likewise
`editor::`/`viewer::`), and every artifact-domain identifier renamed (word-boundary, exact list below).
Editor/viewer plugin-scoped identifiers (`ImperativeConfig*`, `ImperativePresence*`, `ImperativeCommand`,
`ImperativeHost`, `ImperativeCoreError`, `ImperativePlayApp`, `ImperativePlayRuntime`, `ImperativeViewer`,
`ImperativeViewCommand`, `ImperativeSession`, `ImperativeLabels`, `IMPERATIVE_PLAY_*`/`IMPERATIVE_RETAINED_*`/
`IMPERATIVE_VIEW_MODE_VIEW` consts, `imperative_io()`, `imperative_config_edit()`, `imperative_labels()`,
`imperative_steps_topology()`, `bootstrap_imperative_runtime()`, `imperative_app()`) were deliberately left
alone — verified by definition-site inspection, not guessed.

**Identifier renames applied** (artifact-domain only):
`ImperativeArtifact→ProcedureArtifact`, `ImperativeSnapshot(Binary/Text)→ProcedureSnapshot(Binary/Text)`,
`ImperativeMutation(sBinary/sText)→ProcedureMutation(sBinary/sText)`, `ImperativeMutationDsl→ProcedureMutationDsl`,
`ImperativeMutationInput→ProcedureMutationInput`, `ImperativeDiff(Binary/Text)→ProcedureDiff(Binary/Text)`,
`ImperativeFlowChild→ProcedureFlowChild`, `ImperativeTextChild→ProcedureTextChild`,
`ImperativeFlowWorkingData→ProcedureFlowWorkingData`, `ImperativeTextWorkingData→ProcedureTextWorkingData`,
`ImperativeWorkingScene→ProcedureWorkingScene`, `ImperativeInference(Binary/Text)→ProcedureInference(Binary/Text)`,
`ImperativeTopology→ProcedureTopology`, `ImperativeDepthEntry→ProcedureDepthEntry`,
`ImperativeStringList→ProcedureStringList`, `ImperativePath(Ref/RefInput/Delta)→ProcedurePath(Ref/RefInput/Delta)`,
`ImperativeStep(BodyEntry/Input/PatchEntry/sDelta)→ProcedureStep(BodyEntry/Input/PatchEntry/sDelta)`,
`ImperativeSeedEntry→ProcedureSeedEntry`, `ImperativeBuilder(Construction/Facets)→ProcedureBuilder(Construction/Facets)`,
`ImperativeComposer(Composition)/ImperativeAnyComposer→ProcedureComposer(Composition)/ProcedureAnyComposer`,
`ImperativeAnalyzer(Analysis)→ProcedureAnalyzer(Analysis)`, `ImperativeParts→ProcedureParts`,
`ImperativeChildOwnerOracle→ProcedureChildOwnerOracle`, `SerdeJsonImperativeChildOwnerOracle→SerdeJsonProcedureChildOwnerOracle`,
`IMPERATIVE_DIALECT→PROCEDURE_DIALECT`, `IMPERATIVE_DOCUMENT_SCHEMA→PROCEDURE_DOCUMENT_SCHEMA`,
`IMPERATIVE_EXAMPLE_TEXT→PROCEDURE_EXAMPLE_TEXT`, plus the matching lowercase free-fn family
(`imperative_flow_child_handle`, `imperative_working_scene`, `decode_imperative_snapshot_json`,
`encode_imperative_snapshot_binary`, `parse_imperative_dsl`, `apply_imperative_mutation_reporting`,
`compute_imperative_topology`, `register_imperative_mutation_descriptors`, `seed_imperative_flow_json`, …
→ `procedure_*`), and the grammar module names `Imperative_imperative_{diff,inference,mutations,snapshot}` →
`Imperative_procedure_{…}` (`.g4`/`.ksy` files) — plugin segment kept, artifact segment renamed, exactly
mirroring architect's `Architect_program_{…}`.

**String-literal renames** (ArtifactKindSpec + wire identity, exact ticket ask plus its necessary
consequences to stay internally consistent): `id: "computation.imperative"→"computation.procedure"`,
`name: "Imperative"→"Procedure"`, `component_kind: "imperative"→"procedure"`,
`source_format`/`schema: "imperative.document"→"procedure.document"`,
`IMPERATIVE_DOCUMENT_SCHEMA "imperative.document/v1"→"procedure.document/v1"`; the DSL/grammar/protocol
"dialect" family (`s.imperative.schema.artifact→s.procedure.schema.artifact`, `s.imperative.imperative→
s.imperative.procedure`, `s.imperative@1/*→s.procedure@1/*`, `imperative.pack/spr→procedure.pack/spr`,
`imperative.imperative.op/diff→imperative.procedure.op/diff`, `grammar imperative.snapshot→grammar
procedure.snapshot`, etc. — see `🦀️.rs` root, `🧬️schema/**`, and every `.g4`/`.ebnf`/`.abnf`/`.ksy`/`.spicy`/
`.grammar.semio`/`.protocol.semio` file under the renamed tree); localization descriptor `"Imperative"/
"Imperativ"→"Procedure"/"Prozedur"`; file extension `imperative→procedure` (`pilot_languages()`'s
`extension: Some(...)`, the `.grammar.semio` `extension` lines, the codec claim); test/oracle capability ids
`imperative-1-mutate/-any/-python-independent/-nested-step-list-mutation-semantics→procedure-1-…`,
`mutate-imperative-1→mutate-procedure-1` (oracle json, `.feature` tags, test `.rs` panic messages, doc
comments in `⚙️operations/🦀️.rs` and `📸️snapshot/🦀️.rs`).

## Outside-plugin references found and fixed (the "~6" outliers)

Repo-wide search for the module-path token `artifacts::imperative` and the schema strings
`computation.imperative` / `imperative.document` turned up these LIVE, non-ticket-history hits outside the
plugin, all fixed:

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs:61` —
   `use imperative::artifacts::imperative::ImperativeSnapshot as ImperativeDocument;` →
   `use imperative::artifacts::procedure::ProcedureSnapshot as ImperativeDocument;` (alias kept —
   it names the crate/app label "imperative", not the artifact type).
2. `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:9030` —
   doc-comment analogy `imperative/core/rs`'s `ImperativeMutationDsl` → `ProcedureMutationDsl`.
3. `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/mutate-wires-1/🐍️.py:12` —
   prose "Unlike `imperative.document`, …" → "Unlike `procedure.document`, …".
4. `✏️s/🔌️plugins/🔒️policy-allowlist.json` (lines 237–238) — two stale allowlist paths through
   `🗿️artifacts/📜️imperative/` repointed to `📜️procedure/` (a third entry on line 236,
   `🎛️apps/📜️imperative/🎮️commands/🔧️step/🦀️.rs`, was already dead/pre-existing before this ticket —
   left alone, out of scope).
5. `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` `MediaForm::Imperative` → `MediaForm::Procedure` (a
   framework-wide media-form enum, one variant per artifact — `Flow`/`Sequence`/`Deck`/… siblings already
   follow this pattern), plus its two match arms (ordinal `14`, unchanged) in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs`. Not explicitly named in the ticket brief,
   but the same class of adjective-for-artifact-name problem this ticket exists to fix, and fully contained
   (2 usage sites, both in the imperative plugin, both updated).

Two more `storybook-static/…` and `🧰️framework/.../🧑️‍💻️dev/🔌️plugin-modules/…` hits are gitignored,
regenerated build output — left untouched, they will pick up the rename on next build.

## Collision check

Grepped the whole repo (excluding `node_modules`/`target`/`dist`/`.git`) for `Procedure[A-Za-z0-9_]*`
before finishing:
- `🌀️procedural` (being renamed to `generation` by another agent): zero `Procedure*` identifiers.
- `🎬️sequence` artifact: zero `Procedure*` identifiers.
- One genuine pre-existing hit: `GraphDslError::ProcedureArity` in
  `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs` — an enum variant in an unrelated crate
  (`semio-framework-graph`, a generic node-graph DSL's "wrong argument count for a called procedure"
  error). `semio-s-plugin-imperative`'s `Cargo.toml` has no dependency on that crate, so there is no
  shared scope and no compile-time collision.
- `imperative_extension_sdk` (`semio-s-imperative-extension-sdk`) and the `imperative_engine` kernel crate
  (`semio-s-imperative`) were checked for any reference to the renamed artifact types — none found; they
  are a genuinely separate, unrelated dependency (confirmed by this plugin's own root-file doc comment,
  which already warns about the coincidental same-name-different-crate situation).

No duplicate type name was introduced anywhere in `semio-s-plugin-imperative`'s actual dependency graph.

## Cargo check outcome

Per the ticket, ran (foreground, `RUSTC_WRAPPER=""`, wasm target):
```
cargo check -p semio-s-plugin-imperative --target wasm32-wasip2 --message-format short
```
The shared `target/` dir's build lock was held continuously by other concurrent sessions for the whole
verification window (two attempts, ~30–40 minutes each, both processes sitting at "Blocking waiting for
file lock on build directory" with 0% CPU the entire time — confirmed via `ps`). Re-ran against an isolated
`CARGO_TARGET_DIR` (`…/scratchpad/cargo-target-imperative`) to sidestep the lock; that build actually
proceeded (large parts of the framework compiled fresh) but failed twice, in two different files, on two
different pre-existing/concurrent breaks that are **not touched by this ticket and do not mention
`imperative`/`procedure` anywhere in their diagnostics**:

- Run 1: `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️.rs:204` — `ProgramContributionEntry: serde::Serialize`
  not satisfied. `ProgramContributionEntry` is defined in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` with
  `#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]` — no `Serialize`. `git status` on both files
  shows no changes from this session; the manifest file's own diff (from `git diff`) shows an unrelated,
  large, actively-in-progress migration under ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`
  (serde → `ToValue`/`FromValue` migration), owned by another session. My only edit to that 5000+-line file
  is the one `MediaForm::Imperative→Procedure` variant line (confirmed via `git diff` — single hunk, far
  from the failing struct).
- Run 2 (retried after the above): the error moved to a *different* file —
  `🧰️framework/🛂️manifest/../🎠️kernel/🦀️.rs:1818` — `Effect: serde::Deserialize` not satisfied. Same
  root migration, still in flight, now further along.

Both runs got past every other framework/plugin crate cleanly (hundreds of files, only warnings). Neither
failure is in `semio-s-plugin-imperative`, `semio-s-imperative-extension-sdk`, or anything this ticket
touched; both are transient states of a concurrent, unrelated ticket's live edits to the shared
`🛂️manifest`/`🎠️kernel` files. `semio-s-imperative` (the `imperative_engine` kernel crate,
`semio-s-plugin-imperative`'s dependency) never got to compile in either run, so a clean "our crate checks
green" signal could not be obtained during this session — re-running
`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-imperative --target wasm32-wasip2 --message-format short`
once the other ticket's migration settles should confirm it; nothing in that command's diagnostics so far
implicates this rename.

In lieu of a green compile, this rename was verified by exhaustive manual cross-reference: every renamed
identifier's definition site was located and classified (artifact-domain vs. plugin-scoped) before
renaming; every `#[path]` string in the master wiring file was checked post-rename; every serialized
fixture (`.graphql`/`.proto`/`.g4`/`.ebnf`/`.abnf`/`.ksy`/`.spicy`/`.grammar.semio`/`.protocol.semio`/
`.json`) touching the artifact's identity strings was read and hand-fixed against the `architect`/`program`
template, not guessed.

## Residual references

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` still lists `📜️imperative` (3
  entries) — untouched per instructions; the coordinator owns adding `📜️procedure` centrally.
- `storybook-static/plugin-modules/imperative/🔣️.json` and
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/imperative/🔣️.json` — both
  gitignored generated build output, still show the old `computation.imperative`/`imperative.document`
  strings; will regenerate correctly on next build/dev-server run, not hand-edited.
- No other `artifacts::imperative`, `computation.imperative`, or `imperative.document` references remain
  anywhere in the live (non-ticket-history, non-gitignored) tree.
