# W1b — `semio-s-plugin-dag` → `.artifact(declaration())` conversion

`apa-status: partial` — mechanism applied exactly per the `🗒️note` exemplar, both edited files
verified individually error-free by the compiler; the crate as a whole does not currently reach a
clean `cargo check --all-targets` because of pre-existing/concurrent errors in three files I did not
touch and must not touch (see "Verification" below).

## Clearance (Step 0)

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`.
`🕸️dag` appears only in SMO's **RELEASED** table (`🕸️dag` | `🕸️dag` | "14 triads; generic collection
wraps + whole-collection setters decomposed") — not in any **HELD** section. Per that file's own
stated default, this means APA is free to edit `🕸️dag`. Proceeded.

That same file's caveat section is directly relevant to what Step 6 found: *"`cargo check` does not
compile `#[cfg(test)]` code... Proof this is not theoretical: `🕸️dag` passes the workspace check and
still fails to build its tests."* My own `--all-targets` run reproduces failures, though not
exclusively in test code — see below.

## Step 1 — `register()` → `declaration()`

One `register()` in one place: `🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
(mounted at `crate::artifacts::dag::standards::v1::engine`, re-exported as
`crate::artifacts::dag::engine` via `📦️glue.rs:387-389`'s `pub use super::standards::v1::engine::*;`
shim — traced before editing, same double-mount shape as note's exemplar). One standard, one subset
(`✳️any`) — no fan-out needed.

`register()` called five things: `io_registry::register()` (composer entries), `register_pilot_languages()`
(5 `dsl::LanguageSpec`s), `register_artifact_schema()`, `register_artifact_inferences()`,
`crate::apps::dag::config::schema::register_app_schema()` (app-scope), plus a bare
`register_document_codec_for_app::<DagPlayApp>(DAG_DOCUMENT_SCHEMA)` call inline. Replaced with:

```rust
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.dag")
        .schema(crate::artifacts::dag::schema::dag_artifact_schema_descriptor())
        .inferences([crate::artifacts::dag::standards::v1::subsets::any::schema::inferences::dag_artifact_inference_descriptor()])
        .composers(crate::artifacts::dag::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::dag::DagPlayApp>()
        .build()
}
```

`pilot_languages()` (private, `OnceLock`-backed `&'static [dsl::LanguageSpec]`, same reason as note's:
`dsl::passthrough_hooks` isn't `const fn`) replaces the old public `register_pilot_languages()` —
same 5 language specs (`dag.document`, `dag.op`, `dag.diff`, `dag.pack`, `dag.spr`), unchanged content,
just data instead of five `dsl::register_language` calls.

`register_artifact_schema()` and `register_artifact_inferences()` (the two 3-line wrapper fns around
`::schema::register_artifact_schema_descriptor`/`register_artifact_inference_descriptor`) had **zero
call sites left** once `declaration()` calls the underlying descriptor functions directly (confirmed:
`grep -rn "engine::register_artifact_schema\|engine::register_artifact_inferences" ✏️s/🔌️plugins/🕸️dag`
→ empty) — deleted, exactly matching note's own precedent ("all confirmed to have zero other call
sites repo-wide before deleting").

Composer entries: `.composers(...)` takes `crate::artifacts::dag::standards::v1::engine::io_registry::entries()`
directly (`&'static [ComposerEntry]`, 3 entries: the native `DagAnyComposer` bridge writing dialect
`{artifact_kind: "s.dag", standard: "1", subset: "*"}`, plus md/json export bridges reading that same
dialect and writing `s.stdio.md`/`s.stdio.json`) — **not** through the top-level artifact shim's own
`io_registry::register()` (`🗿️artifacts/🕸️dag/🦀️component.rs:80-102`), which wraps the identical entries
as `&'static [&'static ComposerEntry]` (a different type, incompatible with `.composers()`'s
`&'static [ComposerEntry]` signature) purely for its own `compose()` helper. `ArtifactDeclaration`'s
ownership check (`register_all`) verified by hand against these three entries before compiling:
native entry `writes.artifact_kind == "s.dag" == kind` (import direction, always-enforced check
passes); md/json entries `reads` contains `s.dag` (export direction, passes). `kind = "s.dag"` does
not parse as canonical `s.<plugin>.<artifact>` (`ArtifactKindId::parse` requires 3 segments; `"s.dag"`
has 2 — mirrors note's own pre-migration `"s.note"`), so only the always-enforced layer applies today,
same situation as the exemplar.

The top-level shim's `io_registry::register()`/`compose()`/`entries()` (`🗿️artifacts/🕸️dag/🦀️component.rs:80-102`)
is now **orphaned** — `grep -rn "artifacts::dag::io_registry"` across the whole repo returns only the
`use ... as v1` import inside that same module (which still legitimately reads `v1::entries()` for its
own `compose()` fn) and the deleted call from the old `register()`. Left in place, not deleted — same
call note's W1 report made for its own analogous orphan (`pub mod io_registry` in note's artifact
shim), flagged here for whoever next touches dag's IO shim rather than treated as in-scope cleanup.

No subset validators, no format descriptors, no dialect migrations, no composition slots (`DagSnapshot`
has no children/links) exist for this artifact — `.subset_validators()`, `.formats()`, `.migrations()`,
`.composition()` are all correctly left uncalled (confirmed by grep: zero
`register_subset_validator`/`register_format_descriptors`/`register_dialect_migration` calls anywhere
in the plugin before this change).

One standard × one subset × one `declaration()` call — no fan-out was needed.

## Step 2 — plugin root

`✏️s/🔌️plugins/🕸️dag/🦀️component.rs`:

```rust
pub fn plugin() -> Plugin {
    Plugin::builder("dag")
        .label("DAG")
        .version("0.1.0")
        .setup(crate::apps::dag::config::schema::register_app_schema)
        .artifact(crate::artifacts::dag::engine::declaration())
        .register_document_app::<crate::apps::dag::DagPlayApp>(crate::apps::dag::create_dag_app())
        .build()
}
```

`.setup()` survives for **exactly one call**: `crate::apps::dag::config::schema::register_app_schema`
— registers `DagPlayApp`'s own `s.dag.dag` config/presence schema
(`🎛️apps/🕸️dag/🎚️config/🧬️schema/🦀️component.rs:19-37`), the one §6 function
(`register_app_schema_descriptor`) `ArtifactDeclaration` deliberately has no field for (app-scope, not
artifact-scope — see the struct's own doc). No other reason for `.setup()` to remain; no other
`.setup()` call exists in this plugin's tree.

## Step 3 — plugin root closure

Already closed before I started: `ls` on the plugin root shows only `AGENTS.md`, `🎛️apps`, `📦️packages`,
`🗿️artifacts`, `🦀️component.rs` — no `🛂️manifest/`, `🎟️capabilities/`, or `🔧️setup/` dirs to delete,
no stray root data files. Nothing to do for this step.

## Step 4 — escape hatches and deps

`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_"
✏️s/🔌️plugins/🕸️dag --include='*.rs'` → **zero hits**. Nothing to relocate or delete.

`grep -n "semio_framework_os::" ✏️s/🔌️plugins/🕸️dag --include='*.rs' -r` → **zero hits**, and
`📦️packages/🦀️rust/Cargo.toml` does not even depend on `semio-framework-os` (only
`semio-framework-os-kernel` and `infinite_canvas`/`semio-framework-os-infinite`) — nothing to purge.

## Step 5 — inventory

- `thread_local!` — zero occurrences in the plugin.
- Interior-mutable app state / derived caches: two `OnceLock`s, both plain memoized-data caches, not
  host/engine handles: `pilot_languages()`'s `OnceLock<Vec<dsl::LanguageSpec>>` (new, mirrors note's
  identical pattern) and the pre-existing `io_registry::ENTRIES: OnceLock<Vec<&'static ComposerEntry>>`
  (top-level shim) / `OnceLock<Vec<ComposerEntry>>` (engine-level `io_registry::entries()`). None hold
  a host/engine handle (no `OnceLock<...Host>` or similar anywhere in the plugin — grepped, zero hits).
- `std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]`: **one** `std::env::var`
  call, at `🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs:39`
  (`DUMP_DAG_EXAMPLE`) — confirmed **inside** `#[cfg(test)] mod tests { ... }` (line 33), so it is not
  an inventory finding under this step's own scope. No other `std::fs`/`std::env`/`std::process`/
  `Command::new` anywhere in the plugin.
- No `fn seed(...)`/`fn genesis(...)` override anywhere in the plugin (`DagPlayApp` uses the trait
  default) — the M4 `seed`→`genesis()` rename (already landed framework-side) needed no follow-up here,
  unlike vcs's flagged case in the W1 report.

## Step 6 — verification

**1. `#[path]` mounts in `📦️glue.rs` resolve.** Scripted check (Python, resolves every non-`"."`
`#[path] = "..."` string relative to `glue.rs`'s own directory and stats the file):

```
Total #[path] entries: 173, non-dot checked: 92, missing: 0
```

**2. `include_str!`/`include_bytes!` targets resolve.** Scripted check over every `.rs` file in the
plugin (resolves each target relative to its own source file, per Rust's own resolution rule, not
pattern-substituted):

```
Total include_str!/include_bytes! checked: 49, missing: 0
```

This directly covers the plugin-specific note's warning ("Its snapshot-text `include_str!` was
recently repaired — verify it still resolves rather than assuming") — `DAG_EXAMPLE_TEXT`'s
`include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio")` at
`🧬️schema/📸️snapshot/📝️text/🦀️component.rs:20` resolves; separately confirmed by direct `ls` on the
resolved path before running the batch script.

**3. `cargo metadata`:**

```
$ cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-dag --all-targets` (`RUSTC_WRAPPER=""`, ticket target dir):**

Full output saved to
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-w1b-dag-cargo-check.txt`.
Real result: **5 errors (lib) / 8 errors (lib test, overlapping the same 5 plus 3 more), 13-14
warnings**, none of them in either file this ticket touched (`🦀️component.rs` plugin root,
`⚙️engine/🦀️component.rs`) and none of them about `ArtifactDeclaration`, `.artifact()`, `declaration()`,
`pilot_languages`, or `document_codec`. The errors, and the evidence each predates this session:

| error | file:line | mtime | last commit before my session |
|---|---|---|---|
| `MdSnapshot` has no field `body` | `🚪️io/📤️export/…/📝️md/🔖️commonmark/✳️any/🦀️component.rs:8` | Aug 12 10:50:30 | `c31024cc6c` |
| no field `body` on `&MdSnapshot` | `🚪️io/📥️import/…/📝️md/🔖️commonmark/✳️any/🦀️component.rs:9` | Aug 12 10:50:30 | `c31024cc6c` |
| `DagFixtureEdge` missing `properties`/`route_style` | `🧬️schema/💡️inferences/🦀️component.rs:88` | Aug 12 10:50:31 | `16619a9699` |
| `DagFixtureEdge` missing `properties`/`route_style` | `🧬️schema/💡️inferences/🧭topology/🦀️component.rs:93` | Aug 12 10:50:31 | `16619a9699` |
| `DagDiff` has no method `apply` (missing `use crate::store::MutationDiff`) | `🧬️schema/🧬️mutations/🦀️component.rs:64` | **Aug 12 19:33:43** | `a445617cae` |
| `DagMutation` has no method `inverse` (missing `use crate::store::Mutation`) | `🧬️schema/🧬️mutations/🦀️component.rs:144` | **Aug 12 19:33:43** | `a445617cae` |
| `JsonSnapshot { value }` type mismatch (`JsonValue` vs `Value`) | `🚪️io/📤️export/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9` | Aug 12 10:50:30 | `c31024cc6c` |
| `serde_json::from_value` type mismatch (`Value` vs `JsonValue`) | `🚪️io/📥️import/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9` | Aug 12 10:50:30 | `c31024cc6c` |

Every mtime is hours before this session's first edit; every last-touching commit predates this
ticket's work here. `🧬️schema/🧬️mutations/🦀️component.rs`'s mtime (19:33:43) matches, almost to the
minute, the exact same-timestamp observation the W1 mechanism report made about note's own
`🧬️mutations/🦀️component.rs` gaining an unrelated `use protocol::SemanticMutation;` import "from a
session I did not initiate — almost certainly SMO's semantic-mutations work" — the same signature
here (missing `store::Mutation`/`store::MutationDiff` trait imports on the *mutation* trait methods
themselves) is consistent with SMO's in-flight semantic-mutations rewrite of this exact file, not
with anything this ticket's `.artifact()`/`declaration()` change touches or could cause. Per this
ticket's own hard rule, `🧬️mutations/**` is explicitly SMO's territory ("Do NOT touch `🧬️mutations/**`
— another ticket owns it") — I did not enter it, and the two `store::Mutation`/`store::MutationDiff`
import errors in it are not mine to fix. The remaining four errors (md/json serializer field
mismatches, `DagFixtureEdge` missing fields) sit in `🚪️io`/`🧬️schema/💡️inferences`, outside that
explicit no-touch list but equally outside this ticket's scope (no relation to artifact registration)
and equally pre-dating this session — left unfixed, reported here rather than silently patched.

**Both files this ticket actually edited compile without any error being attributed to them** — no
line in the output references `🦀️component.rs` (plugin root) or `⚙️engine/🦀️component.rs`, and since
`declaration()` calls directly into `dag_artifact_schema_descriptor()`, the inference descriptor
function, and `io_registry::entries()`, a real type error in any of those call targets would have
surfaced at the `declaration()`/`pilot_languages()` call sites themselves — it did not. This is
consistent with, not proof beyond, a clean compile of the declaration mechanism; a fully green
`cargo check --all-targets` for this crate cannot be produced until the three pre-existing/concurrent
files above are fixed, which is not this ticket's work.

## Honest pass/fail

- `register()` → `declaration()`: **done**, one standard/one subset, matches the note exemplar's
  builder shape exactly (schema → inferences → composers → languages → document_codec).
- Plugin root `.artifact()` wiring: **done**; `.setup()` narrowed to exactly the one app-schema call,
  same justification as note's exemplar.
- Plugin root closure: **already satisfied**, nothing to change.
- Escape hatches / `semio-framework-os` purge: **nothing found to remove or purge** (measured, not
  assumed).
- Inventory (Step 5): **done**, nothing beyond ordinary data-caching `OnceLock`s and one
  `#[cfg(test)]`-scoped `std::env::var`.
- `#[path]`/`include_str!` resolution: **done**, 0 missing out of 92 + 49 checked.
- `cargo metadata`: **OK**.
- `cargo check -p semio-s-plugin-dag --all-targets`: **NOT green** — 5/8 errors, all in 3 files this
  ticket did not touch, confirmed pre-existing/concurrent by mtime + git log, one of them
  (`🧬️mutations/🦀️component.rs`) explicitly off-limits to this ticket by its own hard rules.

## sharedFileRequests

- `🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (lines 64,
  144) — `DagDiff::apply`/`DagMutation::inverse` calls are missing `use crate::store::MutationDiff;`/
  `use crate::store::Mutation;`. This is inside `🧬️mutations/**`, explicitly SMO's territory per this
  ticket's own rules — flagging for SMO, not fixing.
- `🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs:8` and the paired
  import deserializer at `🚪️io/📥️import/…/📝️md/🔖️commonmark/✳️any/🦀️component.rs:9` — both construct/read
  `MdSnapshot { body: ... }`/`.body`, but the real `MdSnapshot` (presumably owned by `🗄️stdio`) has been
  renamed to a `blocks` field. Pre-existing (mtime Aug 12 10:50, commit `c31024cc6c`), unrelated to
  this ticket — likely stdio's own field-rename churn (matches the "stdio was red" pattern from the W1
  mechanism report).
- `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9` and the paired
  import deserializer at `🚪️io/📥️import/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9` — `JsonSnapshot.value`
  field type mismatch (`serde_json::Value` vs some other `JsonValue` type alias), same mtime/commit as
  the md pair above — same likely stdio-side cause.
- `🧬️schema/💡️inferences/🦀️component.rs:88` and `🧬️schema/💡️inferences/🧭topology/🦀️component.rs:93` —
  both construct `DagFixtureEdge { .. }` missing `properties`/`route_style`, fields the
  `infinite_board_port_directed_dag` kernel crate's `DagFixtureEdge` now requires. Pre-existing
  (mtime Aug 12 10:50/10:51, commit `16619a9699`) — the kernel crate gained fields these two fixture
  builders were never updated for.

None of the four sharedFileRequests items are in files this ticket edited or needed to edit for the
`.artifact()`/`declaration()` conversion itself.
