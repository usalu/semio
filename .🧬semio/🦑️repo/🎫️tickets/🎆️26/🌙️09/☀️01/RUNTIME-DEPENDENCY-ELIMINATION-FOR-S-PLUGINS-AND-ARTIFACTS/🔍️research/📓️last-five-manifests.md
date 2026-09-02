# Last Five Manifests — re-measured after the `app_commands!` and `ArtifactApp::Snapshot` fixes

Batch: `➗️mathematical`, `🔋️energy`, `🌀️procedural`, `📖️playbook/🧩️extensions/🌀️procedural`, `🗄️stdio`.
Companion reading: `📓️final-plugin-manifests.md` (predecessor's per-crate findings, this doc's
baseline), `📓️app-commands-seam.md` (the macro fix), `📓️serde-fanout-playbook.md` (the recipe).

## Headline

**1 of 5 manifests is now written as fully third-party-free**: `➗️mathematical`. Its predecessor-
documented "stdio `JsonSnapshot` bridge" blocker had already been eliminated by a peer/earlier pass
before this session started (the io/import leaf now decodes through `pack::json::from_json_str`,
not `serde_json`) — grep-confirmed zero real `serde_json::`/`serde::` call sites anywhere in the
crate outside its fenced `🏭️generator` — so the leftover `serde_json = { workspace = true }` line
was dead weight, not a live blocker. Removed.

`📖️playbook/🧩️extensions/🌀️procedural` narrowed further: its `FlowFixture` parse (3 call sites) is
now `pack::json::from_json_str` instead of `serde_json::from_str`, because `FlowFixture` itself
picked up `ToValue`/`FromValue` (a peer's `🌊️flow` module work) since the predecessor's pass.
`serde_json` stays in `Cargo.toml` for exactly one remaining crossing: `flow::playbook::visible_blocks`
is still hard-typed to `serde_json::Map<String, serde_json::Value>` — confirmed still live by reading
`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs:276` directly.

`🔋️energy` — re-verified the predecessor's "architecturally pinned" claim directly against source
(not re-derived from the doc): still true, and now doubly confirmed. `crate::model::Model`
(`🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs:766`) still has 40 fields, still derives
`Serialize`/`Deserialize` (additively with `ToValue`/`FromValue`), and `🗿️artifacts/🔋️model/🦀️.rs`'s
`energy_structure_from_model`/`energy_model_from_structure` still call `serde_json::to_value`/
`from_value` directly on `&Model`. **New finding this session**: the file itself now carries a
docstring (already written, uncommitted, presumably by a concurrent pass on the same ticket) stating
that `DslValue::Number`'s new `UInt`/`Int`/`Float` fidelity removes the *original* reason
(int/float-fidelity loss) for staying on `serde_json` — but the bridge itself was never rewritten to
prove it, so the dependency is deliberately kept, out of scope for a manifest-cleanup batch. Added
the missing `Cargo.toml` inline comment stating this precisely (the source docstring already existed;
the manifest didn't yet explain itself).

`🌀️procedural` and `🗄️stdio` were re-measured, not touched — both remain genuinely too large for a
single session's remaining budget, matching the predecessor's own conclusion.

**Verification blocked, batch-wide, by live peer churn — not by any defect in this session's edits.**
Every `cargo check` this session ran hit one or both of:
1. A **repo-wide "Kind-Only Basenames" rename in progress** (git status showed ~41,000 changed paths
   at the time of this session) — intermittently breaks `semio-framework-graph`'s build script
   (`generatorContracts["wgpu-frame-worker"]` reports 5 tracked output paths "missing" because the
   rename hasn't finished landing the renamer targets) and, separately, made `semio-framework-schema`
   fail to read a `../../🦀️.rs` path outright. Neither error names anything in this batch.
2. The ticket brief's own pre-flagged **`🌱️value`'s `OrderedMap` churn**: `error[E0277]: the trait
   bound `OrderedMap<component::Value>: serde::Serialize` is not satisfied`, inside
   `semio-framework-os-kernel-neural-engine` — exactly the "known live churn... not yours" case the
   brief names explicitly.

Both blockers were re-checked several times, spaced by the rest of this session's work, to rule out a
one-off lock-contention fluke (per the ticket's own "beware stale results" warning) — they recurred,
sometimes together, sometimes only the `OrderedMap` one (the taxonomy/rename error cleared between
some attempts, confirming it actually is transient peer churn and not a permanent break). Neither
crate this session edited (`➗️mathematical`, `📖️playbook/🧩️extensions/🌀️procedural`) ever produced
its own compiler error in any attempt.

---

## `➗️mathematical` — WRITTEN AS FULLY THIRD-PARTY-FREE, UNVERIFIED

**Re-measured from scratch** (not carried forward from `📓️final-plugin-manifests.md`): grepped the
entire crate (`✏️s/🔌️plugins/➗️mathematical`, both `📦️packages/🦀️rust` and the `🗿️artifacts` tree,
excluding the fenced `🏭️generator/🦀️json-engine` — its own separate Cargo.toml, correctly untouched)
for `serde`/`serde_json`. Result: **zero real references** — every hit was a code comment (`"interim
(not-yet-serde-free) state"`, `"serde-friendly type"`, doc-comment mentions of the *method name*
`to_serde_value`) or was inside the fenced generator crate.

The predecessor's documented blocker #2 (`🚪️io/📥️import/…/json/…/🦀️.rs` decoding through
`semio_s_plugin_stdio::artifacts::json::JsonSnapshot`, hard-typed to `serde_json::Value`) **no
longer exists in source** — that file now reads:

```rust
let fixture: MathematicalFixture = pack::json::from_json_str(text).map_err(...)?;
```

`MathematicalFixture` (`🗿️artifacts/➗️mathematical/🦀️.rs:127`) derives `ToValue`/`FromValue`
(confirmed by reading the type), so `pack::json::from_json_str<T: FromValue>` — a real, generic,
first-party function at `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1107` — resolves without
`serde`/`serde_json` at all. `JsonSnapshot` is not referenced anywhere in the crate any more (grepped
directly, zero hits). Blocker #1 (`app_commands!`'s 3 nested types + 7 command payloads) was already
resolved before this session by the framework-wide `app_commands!` seam fix
(`📓️app-commands-seam.md`) — `serde` was already absent from the manifest when this session started.

**Converted this session**: removed the now-dead `serde_json = { workspace = true }` line (plus its
stale explanatory comment, which described the `JsonSnapshot` blocker that no longer exists) from
`Cargo.toml`. No source files touched — nothing referenced `serde_json` any more. Confirmed no
reverse dependents exist (`grep -rl "semio-s-plugin-mathematical\b" ✏️s --include=Cargo.toml` outside
the crate's own manifest returns nothing — it's a leaf plugin, safe to edit its dependency list
without a fan-out check).

**Cargo.toml final state**: no `serde`, no `serde_json`. Zero third-party runtime dependencies from
this ticket's serde/serde_json angle.

**Verification**: WRITTEN BUT UNVERIFIED. `cargo check -p semio-s-plugin-mathematical
--message-format=short` never reached the crate's own compilation in any of 4 attempts this session
— blocked upstream by `semio-framework-schema` (file-not-found, rename churn) and/or
`semio-framework-os-kernel-neural-engine` (`OrderedMap: Serialize`, the ticket's own pre-flagged
`🌱️value` churn). Verbatim tail of the last attempt:

```
error: could not compile `semio-framework-schema` (lib) due to 1 previous error
error: could not compile `semio-framework-os-kernel-neural-engine` (lib) due to 1 previous error
```

High confidence in the removal despite this: the deleted line's only justification (a specific,
now-nonexistent call site) was verified false by reading the current source directly, not inferred,
and a full-crate grep for the literal string `serde` (not just `serde_json`) outside the fenced
generator returns only comments.

---

## `📖️playbook/🧩️extensions/🌀️procedural` — serde_json narrowed to its one remaining genuine crossing

**Starting state this session**: matched `📓️final-plugin-manifests.md` exactly — `serde` already
removed, `serde_json` retained for two stated reasons (`flow::playbook::visible_blocks` AND
`FlowFixture`'s own parse, both cited as blocked on the same in-progress `🌊️flow` framework wave).

**Re-verified both reasons against current source, not assumed from the doc**:
- `flow::playbook::visible_blocks` — **still genuinely live**. Read
  `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs:276` directly:
  `pub fn visible_blocks<'a>(step: &'a PlaybookStep, values: &serde_json::Map<String,
  serde_json::Value>) -> Vec<&'a PlaybookBlock>` — unchanged, hard-typed, a framework module this
  extension doesn't own.
- `FlowFixture`'s own parse — **no longer true**. Read `FlowFixture`'s definition
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📄️artifact/🦀️.rs:288`):
  `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]` — it now derives
  `ToValue`/`FromValue` additively, with every `#[serde(...)]` field attribute already mirrored by a
  matching `#[value(...)]` one (checked field-by-field: `rename_all = "camelCase"` on the container,
  `default` on `layout`). This must have landed via a peer's concurrent `🌊️flow` conversion pass
  after the predecessor's doc was written — not something this session's own edits caused.

**Converted this session**: the extension's 3 `let fixture: FlowFixture =
serde_json::from_str(fixture_json)...` call sites (`render_preview_body`,
`handle_export_solid`/`evaluated_preview_geometry_handles`'s shared caller, `render_params_body` —
`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:453,490,649`) rewritten to
`pack::json::from_json_str(fixture_json)`, same `.unwrap_or_else(|_| FlowFixture::default())`
fallback preserved. `fixture_json: &'static str` (from `fixture_json_for_slug`) matches
`from_json_str<T: FromValue>(text: &str)`'s signature directly — no adapter needed. Updated the
file's own top-of-file docstring (previously claimed `FlowFixture` "not yet `ToValue`/`FromValue`",
now corrected) and the `Cargo.toml` inline comment (previously named `FlowFixture`'s parse as a
co-reason for keeping `serde_json`; now names only `visible_blocks`).

The one remaining `serde_json` call site (`render_params_body`'s `values_serde: serde_json::Map<...>
= serde_json::from_str(&json_to_string(...))`, feeding `visible_blocks`) is untouched — genuinely
required, confirmed live above.

**Cargo.toml**: `serde_json = "1.0.140"` retained, comment rewritten to name only the live blocker.

**Verification**: WRITTEN BUT UNVERIFIED — same two upstream blockers as `➗️mathematical` above
(`semio-framework-schema` rename churn / `semio-framework-os-kernel-neural-engine`'s `OrderedMap`
churn), verbatim tail identical:

```
error: could not compile `semio-framework-schema` (lib) due to 1 previous error
error: could not compile `semio-framework-os-kernel-neural-engine` (lib) due to 1 previous error
```

The edit itself is low-risk: mechanical, type-checked by inspection against `pack::json`'s real
signature (already used elsewhere in the same file for other calls), and `FlowFixture: FromValue` was
confirmed by reading the derive line directly rather than assumed.

---

## `🔋️energy` — confirmed still architecturally pinned, even after the `DslValue::Number` fidelity fix

**Re-verified the predecessor's claim from scratch, not re-derived from the doc.** Grepped the whole
crate: 55 files still reference `serde`/`serde_json` outside fenced dirs (engine tree + schema/
snapshot/diff/inferences/editor — same shape as documented). `crate::model::Model`
(`🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs:766`) still has exactly 40 fields, still
`#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]`
(additive — both). `🗿️artifacts/🔋️model/🦀️.rs:115-126`'s `energy_structure_from_model`/
`energy_model_from_structure` still call `serde_json::to_value(model)`/
`serde_json::from_value(json_from_semio_value(&structure.root))` directly, unchanged. Predecessor's
finding is accurate: this is not "not yet converted," it's a real production bridge whose only
existing implementation goes through `serde_json::Value` on the full `Model` tree.

**New finding this session — the fidelity argument itself is now stale, but nobody has acted on it**:
the file already carries an uncommitted docstring (found in the live working tree, not written by
this session — likely a concurrent pass on the same ticket/batch) stating explicitly that
`DslValue::Number`'s new `UInt`/`Int`/`Float` fidelity (this exact ticket's own framework change)
removes the original int/float-fidelity-loss reason for staying on `serde_json` at the `DslValue`
layer — `Model` already derives `ToValue`/`FromValue` too. But the bridge function itself was never
rewritten to route through `ToValue`/`DslValue` instead of `serde_json::Value` — that would be a real,
unstarted code change (swap `serde_json::to_value`/`from_value` for `model.to_value()`/
`Model::from_value(...)`, and reconcile `DslValue`'s tree shape against `SemioValue`'s, which are
currently bridged only via the JSON-shaped intermediate). Doing that rewrite was judged out of scope
for a manifest-cleanup batch — it is a genuine, scoped, one-function migration for a future session,
not a "some files aren't done" gap.

**Converted this session**: none (no file needed it — the predecessor's prior session already
converted the crate's one remaining stray-fixture file). Added the missing `Cargo.toml` inline
comment stating the precise, current reason (the source docstring already existed; the manifest
itself didn't yet explain itself to a reader who only looks at `Cargo.toml`).

**Cargo.toml**: `serde`, `serde_json` both retained (`{ workspace = true }`), now with an inline
comment naming the exact function, exact field count, and the now-stale-but-unactioned fidelity
argument.

**Verification**: comment-only Cargo.toml change plus zero source edits — cannot regress compilation.
Not independently re-run against a passing build (blocked by the same upstream churn as the other two
manifests this session touched); no new risk introduced.

---

## `🌀️procedural` — re-measured, still too large for this session

Fresh grep this session: **159 files reference `serde` outside fenced dirs** (down from the
predecessor's 187 — likely fixture/test-file churn from concurrent work, not this session's doing;
not reconciled, not material), **1229 `serde_json::` call sites** (identical to the predecessor's
count — exact same number). `app_commands!` usage confirmed present in 9 files (multiple `✏️editor/…`
command modules), so — per the `➗️mathematical`/`🏗️fem` finding this ticket's earlier work
established — even a full conversion would still need `serde` for the crate's own `Command` payload
types, unless those specific types get the same `ToValue`/`FromValue`-only treatment `➗️mathematical`
already proved out. Left untouched: `Cargo.toml` unchanged (`serde.workspace = true`, `serde_json`
with `float_roundtrip`). No source files touched. Confirmed, not assumed: this remains its own
multi-session wave, exactly as both the predecessor and this ticket's own brief already concluded.

## `🗄️stdio` — re-measured, still too large for this session

Fresh grep this session: **584 files reference `serde` outside fenced dirs, 7084 `serde_json::` call
sites** — matches the predecessor's ~583/7084 almost exactly (one-file difference, not material).
Not started this session — an order of magnitude beyond what remained of the session's budget after
`➗️mathematical`/`🔋️energy`/`📖️playbook`'s extension. The predecessor's flagged highest-leverage
target (the `🪟️main` viewer/editor template, ~78 files sharing one shape, and the specific `MeshData:
ToValue` gap surfaced in `📓️app-commands-seam.md`'s `fem` verification run) was not investigated
further this session — genuinely out of the remaining time budget, not a judgment that it's wrong.
`Cargo.toml` unchanged (`serde`/`serde_json` both `{ workspace = true }`).

---

## Final Cargo.toml state — verbatim dependency lines

```toml
# ➗️mathematical/📦️packages/🦀️rust/Cargo.toml — CHANGED this session
pack = { path = "...", package = "semio-framework-pack" }
# serde REMOVED (already absent) — serde_json REMOVED this session (dead: JsonSnapshot blocker no longer exists)
# → NO serde, NO serde_json

# 🔋️energy/📦️packages/🦀️rust/Cargo.toml — comment ADDED this session, deps unchanged
pack = { path = "...", package = "semio-framework-pack" }
serde = { workspace = true }        # Model<->SemioValue bridge, 40-field/44-file engine tree — see inline comment
serde_json = { workspace = true }   # same bridge

# 📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml — comment NARROWED this session
pack = { path = "...", package = "semio-framework-pack" }
# serde REMOVED (prior session)
serde_json = "1.0.140"              # flow::playbook::visible_blocks ONLY now — see inline comment

# 🌀️procedural/📦️packages/🦀️rust/Cargo.toml — UNCHANGED
serde.workspace = true
serde_json = { workspace = true, features = ["float_roundtrip"] }

# 🗄️stdio/📦️packages/🦀️rust/Cargo.toml — UNCHANGED
serde = { workspace = true }
serde_json = { workspace = true }
```

**Net manifest delta this session**: `➗️mathematical` lost its last third-party line (`serde_json`)
— **first of the six original manifests to reach zero**. `📖️playbook/🧩️extensions/🌀️procedural` kept
`serde_json` but for a strictly narrower, re-verified reason (1 call site instead of 4). `🔋️energy`
unchanged at the dependency level, now with its reasoning made precise and current (including the
fact that the technical blocker for a real fix — `DslValue::Number` fidelity — is gone, even though
the bridge rewrite itself hasn't happened). `🌀️procedural`/`🗄️stdio` unchanged, re-measured, both
confirmed still too large for a single session.

---

## Verbatim tails of every verification attempt this session

All `cargo check` commands run in the foreground, one at a time, no `CARGO_TARGET_DIR` override, no
background/Monitor use, per the ticket's hard constraints.

**`cargo check -p semio-s-plugin-mathematical --message-format=short`** — 4 attempts, spaced by the
rest of this session's work (to rule out a one-off lock/stale-check fluke):

- Attempt 1 (before edits): failed inside `semio-framework-graph`'s build script —
  `generatorContracts["wgpu-frame-worker"]` reports 5 tracked outputs "missing" under
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/...` — a live,
  in-progress repo-wide basename-rename (git status showed ~41,000 changed paths at the time).
- Attempt 2 (after the `Cargo.toml` edit): identical failure, same missing paths.
- Attempt 3: the `wgpu-frame-worker` error cleared (confirming it actually is transient rename
  churn), but `semio-framework-schema` then failed: `error: couldn't read
  🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/../../🦀️.rs: No such file or directory`, and
  separately `semio-framework-os-kernel-neural-engine` failed:
  `error[E0277]: the trait bound OrderedMap<component::Value>: serde::Serialize is not satisfied`
  (the ticket brief's own pre-flagged `🌱️value`/`OrderedMap` churn).
- Attempt 4 (final, verbatim):
  ```
  error: could not compile `semio-framework-schema` (lib) due to 1 previous error
  error: could not compile `semio-framework-os-kernel-neural-engine` (lib) due to 1 previous error
  ```

**`cargo check -p semio-s-plugin-playbook-procedural --message-format=short`** — 2 attempts, both
after the source/manifest edits, same two upstream blockers, final verbatim tail identical to
attempt 4 above.

**No error in any attempt, across either crate, named anything in `➗️mathematical`,
`📖️playbook`, or its `🌀️procedural` extension.** Both recurring blockers are pre-existing,
attributable to live peer work this ticket's own brief explicitly pre-flagged as out of scope
(`OrderedMap`) or independently confirmed as transient rename churn (the taxonomy/`wgpu-frame-worker`
generator failure, which cleared and reappeared across attempts rather than being a permanent break).

A final round of retries (both crates, after this doc's own body was written) confirms the
`semio-framework-os-kernel-neural-engine` blocker is itself actively being edited live: its own
error count moved from 1 to 53 between consecutive attempts, seconds apart, with no edit from this
session anywhere near that crate — a peer mid-fix, not a stable break. Still zero errors named in
either crate this session touched.

## Honest status per manifest

| manifest | serde? | serde_json? | reason if present | status |
|---|---|---|---|---|
| `➗️mathematical` | **no** | **no** | — | WRITTEN, UNVERIFIED (blocked by live peer churn) |
| `🔋️energy` | yes | yes | `Model`↔`SemioValue` bridge, 40-field/44-file engine tree, structural | unchanged, comment added |
| `📖️playbook/🧩️extensions/🌀️procedural` | no | yes | `flow::playbook::visible_blocks` ONLY now | WRITTEN, UNVERIFIED (blocked by live peer churn) |
| `🌀️procedural` | yes | yes | not attempted — 159 files / 1229 sites, own wave | measured only |
| `🗄️stdio` | yes | yes | not attempted — 584 files / 7084 sites, own wave | measured only |

Whoever picks this up next: re-run `cargo check -p semio-s-plugin-mathematical
--message-format=short` and `cargo check -p semio-s-plugin-playbook-procedural
--message-format=short` once `semio-framework-schema` and
`semio-framework-os-kernel-neural-engine` compile clean (both are pre-existing, someone-else's-wave
breakage, not introduced by this session). If either of this session's two edited crates still
doesn't compile once those clear, the error list will be small and load-bearing — this session's own
edits were traced against real signatures (`pack::json::from_json_str`'s generic bound,
`FlowFixture`'s actual derive line, `MathematicalFixture`'s actual derive line), not guessed.
