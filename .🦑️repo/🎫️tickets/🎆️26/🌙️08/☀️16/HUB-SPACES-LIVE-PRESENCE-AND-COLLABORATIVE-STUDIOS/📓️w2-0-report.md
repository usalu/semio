# Lane 2-0 report — remediation: clear the two W1-barrier blockers

Scope: exactly the two blockers named in the brief. Nothing else attempted.

## Blocker 1 — hub directory tests (RESOLVED)

**Root cause confirmed: test-fixture bug, not a `decide()`/`append_events`/`project` bug** — lane
1-B's diagnosis was right. Evidence:

- `decide()`'s `CreateSpace` arm (`🌎️hub/📇️directory/🦀️component.rs:325-340`) derives `owner_user_id`
  from the actor id alone (`actor_user_id(actor)`) and never checks/creates a `hub_user` row for it —
  unlike `UpsertMember`, which *does* self-heal a missing user because it has an `email` to mint one
  from (`decide()`'s own docstring documents this asymmetry). `CreateSpace`'s actor carries no email,
  so it structurally cannot self-heal; it correctly assumes the owner's `hub_user` row already exists.
- That assumption is correct in production: contract §C2 requires `Authorization: Bearer <session>`
  on every directory command, and the only way to mint a session (`POST /auth/sessions` →
  `create_auth_session`, `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:340-348`) calls `HubDirectory::create_user`
  directly (a raw, non-event-sourced write — pre-existing, `🪶️sqlite/🦀️component.rs:270-288`) *before*
  any directory command can be issued. So in every real path, the owner's `hub_user` row predates
  `create-space` by construction.
- `SqliteDirectory::seed()` (`🪶️sqlite/🦀️component.rs:150-182`, pre-existing) reproduces exactly this
  precondition for its own `"seed"` user/`"default"` space by appending a bare `user.created` event
  via a `System` actor before appending `space.created`.
- `directory::tests::fresh_dir()` (lane 1-A's fixture, `🌎️hub/📇️directory/🦀️component.rs:623-625`)
  never did this — it opened a bare `SqliteDirectory::connect(":memory:")` and every test then called
  `create_space(&service, &user_actor("u-owner"), …)`, tripping `hub_space.owner_user_id`'s FK against
  a `hub_user` row that was never inserted. Lane 1-B's own `create_space_for_test`/`upsert_member_for_test`
  helpers in `bin.rs` hit and fixed the identical shape of bug by reusing the pre-seeded `"seed"` user.

**Fix** (inside my lease, `🌎️hub/📇️directory/**`): `fresh_dir()` now appends one `user.created` event
for `"u-owner"` under a `System` actor immediately after connecting, mirroring `seed()`'s own pattern.
`🌎️hub/📇️directory/🦀️component.rs:623-636`.

```rust
async fn fresh_dir() -> Arc<SqliteDirectory> {
    let dir = Arc::new(SqliteDirectory::connect(":memory:").await.expect("connect"));
    let seed_actor = DirectoryActor { kind: DirectoryActorKind::System, id: "system:test-seed".into() };
    let mut clock = HubClock::new();
    let events = vec![new_event(&mut clock, &seed_actor, None, Some("u-owner".into()), DirectoryEventBody::UserCreated { user_id: "u-owner".into(), email: "u-owner@example.com".into(), display_name: "Owner".into() })];
    dir.append_events(&events).await.expect("seed owner user");
    dir
}
```

### Verify

- `cargo test -p semio-hub --lib` (default features/sqlite): **11 passed; 0 failed** (was 4 passed / 7
  failed). Tail in `🧪️2-0-hub-lib-test.txt`.
- `cargo test -p semio-hub --bin os-hub`: **18 passed; 0 failed** — unchanged/still green. Tail in
  `🧪️2-0-hub-bin-test.txt`.

## Blocker 2 — `semio-s-plugin-space` link failure (RESOLVED)

### Diagnosis

`.document_app::<crate::engine::space::SpaceApp>(…)` at `✏️s/🔌️plugins/🪐️space/🦀️component.rs:394` is
genuinely pre-existing (`git log --date=iso` on that file shows nothing newer than the standing
`c8a29e41c5` auto-commit sweep at `2026-08-16 20:26:15`, and lane 1-E's own report already confirms it
added lines *around* that call without touching it). The bound it trips —
`A::Mutation: protocol::SemanticMutation<A::Snapshot>` on
`PluginBuilder::<Ready>::document_app` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:202-226`,
pre-fix) — was last touched in commit `0727b80aa6` (`2026-08-16 12:10:56`, over 12 hours before I
started; nothing touched it in the 30 minutes before my edit either).

I verified the bound is not just theoretically wrong but **already broken a second, unrelated
in-flight crate**: `cargo check -p semio-s-plugin-playbook-procedural` (baseline, before any of my
edits — `🧪️2-0-playbook-procedural-check-baseline.txt`) failed with the *exact same*
`ModulePayloadMutation: SemanticMutation<ModuleRenderPayload>` bound error at its own
`.document_app::<ModuleApp>(…)` call site — `ModulePayloadMutation` derives `dsl::DslOps` (which only
generates `DslVariants`), not `dsl::Mutations` (which is what actually generates the
`SemanticMutation` impl), so it never satisfied the bound and never could without a much bigger
refactor of that module's mutation type. This is the same shape of problem
`26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` already found and fixed for `.editor()`/`.viewer()`
("never required by the surface traits, silently blocked 32 stdio subsets") — `.document_app()` was
just never given the same treatment.

**Chose fix option 1** (relax the bound, mirroring the `.editor()`/`.viewer()` split exactly), not
option 2 (`SpaceApp`'s `Mutation` is framework-owned `WorkflowMutation`, which has **zero**
`SemanticMutation` impl anywhere in the tree — implementing one from our side, inside our lease, is not
possible; it would have to be a framework change either way).

### Foreign touches (framework, option 1 — coordinator-authorized)

Both files are outside my lease; both are the *minimal* change needed, confirmed via `git log --date=iso`
that neither was touched in the 30 minutes before editing (checked immediately before each edit,
re-read immediately before):

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`** — removed the
   `A::Mutation: protocol::SemanticMutation<A::Snapshot>` where-clause from `document_app` (it no
   longer pushes an owner-mutation-roster entry either); added a new opt-in
   `document_app_mutation_roster<A: ArtifactApp>()` method that carries the bound and does exactly
   what `document_app` used to do inline — this is the literal `editor`/`editor_mutation_roster` split,
   applied to `document_app`. One doc-comment update on the `owner_mutation_rosters` field to match.
   No other region touched.
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`** — the framework's own test
   `__semio_plugin_bundle()` (line ~16121) called `.document_app::<TestApp>(…)`; `TestApp::Mutation =
   TestMutation`, which *does* hand-implement `SemanticMutation` (a test fixture, line ~15870/15655),
   so to keep that test's owner-mutation-roster registration byte-identical to before my change I
   appended `.document_app_mutation_roster::<TestApp>()` right after it. One line, purely additive,
   behavior-preserving.

### Verify

- `cargo check -p semio-s-plugin-space`: **0 errors** (was 2). Tail in `🧪️2-0-space-check-final.txt`.
- Second error (predicted correctly by lane 1-E): `register_stdio_format_descriptors` was renamed to
  `stdio_format_descriptors` (now `Result<Vec<FormatDescriptor>, PluginAssemblyError>`) by the live
  `FULL-STDIO` peer. Fixed **our** call site,
  `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🖼️export-media/🦀️component.rs:57` (a `#[cfg(test)]`
  test), never touching `🗄️stdio/**`:
  ```rust
  let stdio_descriptors = semio_s_plugin_stdio::manifest::stdio_format_descriptors().expect("stdio format descriptors");
  semio_framework::register_format_descriptors(stdio_descriptors).expect("register stdio format descriptors");
  ```
  (`register_format_descriptors` is `semio_framework`'s own global-catalog registration fn, already
  the pair `format_descriptor()` — called two lines above in the same file — reads from.)
- `cargo test -p semio-s-plugin-space --lib`: **124 passed; 15 failed** (the crate could not link at
  all before this lane; this is the very first real run). Tail in `🧪️2-0-space-test-3.txt`.
  - **Fixed (ours — the new `s.space` artifact, lane 1-E's untested code)**: 1 failure,
    `artifacts::space::standards::v1::subsets::any::examples::demo::component::tests::bundled_example_parses_as_a_valid_space_index`.
    The bundled example DSL text was hand-written and wrong in two ways: `space_id=demo-space` should
    be `space-id=demo-space` (the DSL's kebab-case key, not the Rust field name), and it omitted the
    required `artifacts […] { }` empty-table block entirely — `dsl::parse` requires the table token
    even when empty. Regenerated the exact correct text with a throwaway `eprintln!` of
    `print_dsl()` on `empty_space_index_snapshot("demo-space")`, verified it round-trips, then removed
    the throwaway test. Every other test under `artifacts::space::*`, `space_shared::*`,
    `plugin::surface_tests::*`, `viewer::space_index::*`, and `editor::space_index::*` (all of lane
    1-E's new work) now passes.
  - **NOT fixed — out of scope, pre-existing, not the new artifact**: the remaining 15 failures are
    all in `engine::space::*` (the pre-existing "studio" engine, relocated out of `🎛️apps` by the
    ARTIFACT-VIEWERS ticket, not part of the new `s.space` index artifact). 14 of them panic identically
    at `🔌️plugin/🦀️component.rs:5167` — `parse_surface_app_id` rejecting bare app ids (`"draw"`,
    `"studio"`, `"puzzle5d"`, `"root-tool"`) that predate the canonical `<dialect>/<subset>@<v>/*#<role>`
    surface-id convention. That validation was introduced in commit `07873f842a` (`2026-08-16
    11:00:35`) — hours before this lane started and unrelated to either of my two blockers. The 15th
    (`commit_checkpoint_round_trips_projection`, "cannot create an empty checkpoint") is downstream of
    the same app-id churn in that test's fixture setup. Fixing these means migrating every
    `⚙️engine/🪐️space/**` app-id string literal to the canonical form — a real, non-trivial task, but a
    different one from "clear the two blockers," so left alone per the brief's explicit "Nothing else."
    Flagging for the next wave/coordinator.

### Regression guard

- `cargo check -p semio-framework-plugin`: **0 errors** (`🧪️2-0-framework-plugin-check.txt`).
- `cargo check -p semio-s-plugin-note`: **RED, 4 errors** (`🧪️2-0-note-check.txt`) — but confirmed
  **unrelated to my change**: `note` never calls `.document_app`/`.document_app_mutation_roster`
  anywhere (`grep` = 0 hits), and every error is a `SvgSnapshot`/`DwgSnapshot` field mismatch
  (`lexical`, `bytes`) inside `🗄️stdio/**`-owned deserializer code that `note`'s own `io` facet pulls
  in — `git status --porcelain` shows dozens of live, uncommitted `M`s under `🗄️stdio/**` right now
  (concurrent peer schema churn, same shape as the memory note on concurrent cargo workspace churn).
  Picked a second, actually-informative crate instead:
- `cargo check -p semio-s-plugin-dag`: **0 errors** (`🧪️2-0-dag-check.txt`) — clean, proves the builder
  change doesn't regress a plugin that uses `.editor()`/`.viewer()` (not `.document_app`).
- Bonus: `cargo check -p semio-s-plugin-playbook-procedural` went from **1 error** (the same
  `SemanticMutation` bound, confirmed pre-existing/unrelated to this ticket —
  `🧪️2-0-playbook-procedural-check-baseline.txt`) to **0 errors**
  (`🧪️2-0-playbook-procedural-check-postfix.txt`) after the framework fix — independent evidence the
  bound really was wrong, not just for `SpaceApp`.

## Changed files

- `🌎️hub/📇️directory/🦀️component.rs` — `fresh_dir()` test helper now seeds the `"u-owner"` user.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` — **foreign, authorized**:
  `document_app` bound relaxed; new opt-in `document_app_mutation_roster`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — **foreign, authorized**: one line,
  `__semio_plugin_bundle()` test now also calls `.document_app_mutation_roster::<TestApp>()`.
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🖼️export-media/🦀️component.rs` — adapted our call
  site to stdio's renamed `stdio_format_descriptors()`/`register_format_descriptors()`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` —
  no net change (temporary debug test added and removed within this session).
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` —
  corrected the bundled example's DSL text (`space-id` key, required empty `artifacts […] { }` block).

## Logs (this ticket folder)

`🧪️2-0-hub-lib-test.txt`, `🧪️2-0-hub-bin-test.txt`, `🧪️2-0-framework-plugin-check.txt`,
`🧪️2-0-note-check.txt`, `🧪️2-0-dag-check.txt`, `🧪️2-0-playbook-procedural-check-baseline.txt`,
`🧪️2-0-playbook-procedural-check-postfix.txt`, `🧪️2-0-space-check-1.txt`, `🧪️2-0-space-test-1.txt`,
`🧪️2-0-space-test-2.txt`, `🧪️2-0-space-test-3.txt`, `🧪️2-0-space-check-final.txt`.

## sharedFileRequests

None beyond the two foreign-file touches already authorized by the brief (documented above under
"Foreign touches").

## What is NOT done

- The 15 pre-existing `engine::space::*` (studio) test failures from the canonical-surface-id
  migration (commit `07873f842a`) — out of scope for this remediation lane, flagged above for the
  coordinator/next wave.
- Postgres/neo4j hub directory backends remain unverified by any compiler run (Amendment 2,
  pre-existing, not this lane's to fix).
- Ticket not closed (per brief — coordinator owns that).
