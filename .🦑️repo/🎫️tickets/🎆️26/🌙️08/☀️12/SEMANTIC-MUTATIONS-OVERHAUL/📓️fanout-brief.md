# Fan-out brief — migrating one mutation facet

Normative recipe for every facet-migration agent on this ticket. Read together with
`📓️taxonomy.md` (verb table + naming mechanics), `📓️derivation-rules.md` (how to derive the
vocabulary from a snapshot), and `📓️remaining-work-map.md` (what state your facet is in).

## Hard rules

- NEVER run git-modifying commands (`commit`, `stash`, `checkout`, `reset`, `add`, `rebase`, …).
  Other developers and agents are editing this same working tree live.
- NEVER call `ticket_close` / `ticket_open` / `ticket_reopen`. Only the coordinator closes tickets.
- NEVER use git worktrees or `isolation: "worktree"`.
- NEVER edit the root `📜️script.ts`. Report allowlist keys; the coordinator applies them.
- NEVER edit files outside your assigned plugin. If you need a change elsewhere, report it as a
  `sharedFileRequest`.
- NEVER create new files outside your facet's own taxonomy or the ticket folder. Extend existing
  files. No new test files — extend the existing `#[cfg(test)]` regions. No new example files.
- Structure code with `//#region Name` / `//#endregion`. Docstrings start with a fitting emoji.
  No comments inside definitions. Concise code.
- Scratch, logs and reports go in the ticket folder as `.txt`/`.md` — NEVER `.log` (gitignored,
  and `ticket_close` silently drops gitignored paths).
- Do NOT fix other sessions' breakage. Repo-wide `cargo check` failures in `🧰️framework/**`,
  `🛢️db`, or missing `📌️panels/📄️document/🦀️component.rs` files are concurrent churn from other
  devs. Retry up to 3× spaced ~5 min if a foreign failure blocks your gate, then record it as
  `blocked-churn` with the verbatim error and move on. Never `cargo update`.
- NEVER claim a test passes or a feature works without having run it and seen the output.

## Step 1 — derive the vocabulary

Read the facet's `🧬️schema/📸️snapshot/🦀️component.rs` FIRST. The vocabulary comes from the
snapshot shape, not from the old mutation enum. Apply `📓️derivation-rules.md`:

- root scalars → `rename-<artifact>` (identity field) + `change-<field>` per remaining scalar;
  `update-<facet>` only for an inseparable ≥2-field facet.
- id-keyed collection → `create-<singular>`, `delete-<singular>` (+ `delete-<plural>` if the
  editor multi-selects), `rename-<singular>` if it has a name/key/code,
  `change-<singular>-<field>` per scalar, `add-`/`remove-<singular>-<member>` per `Vec` field,
  `replace-<singular>-<payload>` per large structured field, `reorder-<plural>` only if list
  order is user-meaningful.
- index-keyed ordered collection → `insert-<singular>{index FINAL}`,
  `remove-<singular>{index BASE}`, `reorder-<plural>{from,to}`, plus `edit`/`move`/`resize`.
- edge collection → `connect-<nouns>` ↔ `disconnect-<noun>`; parameterization → `bind`/`unbind`.
- hierarchy → `move-to-<container>`; `group`/`ungroup`, `flatten`/`unflatten` only for real
  gestures.
- **No whole-document mutation.** `SetSnapshot` dies with NO replacement. Whole-doc replace goes
  through `ArtifactStore::reset` (non-history), never through the mutation enum.
- `NoMutation` dies. A mutation with nothing to undo returns `Vec::new()` from
  `MutationKind::inverse`. Drop any `#[default]` that depended on it.
- Verbs must come from `APPROVED_VERBS` (`📓️taxonomy.md`). `set` survives ONLY for a narrow,
  addressed, single-field setter. Bulk = separate plural mutations, never a `Vec` arg on a
  singular verb.

## Step 2 — author the triad

Target directory shape (one dir per mutation, emoji-prefixed kebab slug, **emoji unique within
the facet** — a policy rule enforces uniqueness):

```
🧬️mutations/
  🦀️component.rs                 <- dispatch enum ONLY
  <emoji><verb>-<entity>/
    🦠️mutation/🦀️component.rs    <- payload struct + impl MutationKind<Snapshot, Mutation>
    🔺️diff/🦀️component.rs        <- pub fn diff(payload, base) -> XDiff   (real, handcrafted)
    ↩️inverse/🦀️component.rs     <- pub fn inverse(payload, base) -> Vec<XMutation>  (real)
    (🟦️component.ts mirrors — non-stub)
```

- `🦠️mutation` holds the payload struct (`Clone, Debug, PartialEq, Serialize, Deserialize`) with
  a real `const SEMANTICS: SemanticDescriptor { verb, entity, kind, record }`. `kind` MUST equal
  the triad-dir stem with emoji stripped AND the kebab of the enum variant name — the derive
  enforces this as a compile error. `record` is past tense (`CreatedTile`, `RenamedLayer`).
  `diff`/`inverse` DELEGATE to the sibling leaves; never inline logic here.
- `🔺️diff` builds the artifact's sparse `XDiff` DIRECTLY from `(payload, base)`. Never
  apply-then-capture. Never clone the snapshot. Return `XDiff::default()` when the target is
  absent from `base`.
- `↩️inverse` reconstructs from `base` (the pre-state) only — never structurally inverts the
  diff. `delete`/`remove` capture the full removed payload plus any severed cascade, re-`connect`ed
  after `create` in reverse dependency order. Missing target ⇒ `Vec::new()`.
- Addressing: id-keyed by default; name/code-keyed where the format's native key IS the name
  (then rename-aware inverses); index-keyed only for intrinsically ordered anonymous collections,
  with removed/modified indices in BASE state and inserted indices in FINAL state;
  `reorder`'s inverse is `reorder{from: min(to, len-1), to: from}`. Nested targets concatenate
  address fields outermost first.

Reference implementations to copy the shape from (all real, all compiling):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` — region
  `🧪️MutationsDeriveLaws`, the `MiniDoc`/`MiniMutation`/`RenameMini` fixture: the canonical
  end-to-end pattern.
- `✏️s/🔌️plugins/🎬️sequence/…/🧬️mutations/🌱create-step/🔺️diff/🦀️component.rs` and
  `…/🗑️delete-step/🔺️diff/🦀️component.rs` (cascade capture).
- `✏️s/🔌️plugins/📋️forms/…/🧬️mutations/➕add-step/🔺️diff/🦀️component.rs` (idempotent early return).

## Step 3 — dispatch enum

`🧬️mutations/🦀️component.rs` shrinks to a variant list. Every variant is EXACTLY one unnamed
field wrapping a payload struct — anything else is a compile error from the derive.

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = XSnapshot, diff = XDiff, schema = "<plugin>.<artifact>")]
pub enum XMutation { RenameX(RenameX), ChangeXTitle(ChangeXTitle), … }
```

**Check your crate's actual alias first.** Most plugin crates do
`extern crate semio_framework_os_kernel as dsl;` in `📦️glue.rs`, so the path is `dsl::Mutations`,
NOT `dsl_derive::Mutations`. Confirm in your plugin's `📦️glue.rs` before writing the derive.

Do NOT hand-write `impl Mutation<XSnapshot>` or `impl SemanticMutation<XSnapshot>` — the derive
generates both, plus `register_<enum>_descriptors()` and the compile-time kind/verb assertions.
Delete the old hand-written `apply_*`/`inverse_*` match dispatch functions once the derive covers
them (grep for callers first; app code may call them).

## Step 4 — wiring (`📦️glue.rs`)

Each new triad dir must be `#[path]`-mounted from your plugin's
`📦️packages/🦀️rust/📦️glue.rs`, one `pub mod <snake_slug> { pub mod mutation; pub mod diff;
pub mod inverse; }` block per slug, mirroring the `#[path]` prefix depth of the existing blocks in
that file. Remove mounts for dirs you delete. **You own your plugin's glue.rs exclusively** — no
other agent may touch it during your run. If you are a sub-lane agent explicitly told NOT to touch
glue, emit `sharedFileRequests` instead and the funnel agent applies them.

Inline `#[path = "."] pub mod <slug> { … }` self-wiring inside the dispatch file is NOT acceptable
as an end state; real dirs + real glue mounts only.

## Step 5 — call sites (the part most often missed)

Grep your ENTIRE plugin for constructions of the old variants and for the banned tokens:

```
grep -rnE "SetSnapshot|NoMutation|CollectionMutation(<|::)" ✏️s/🔌️plugins/<plugin> --include="*.rs" --include="*.ts"
```

Fix every hit inside your plugin, not just the ones under `🎮️commands`: app roots
(`🎛️apps/*/🦀️component.rs`), `📌️panels`, modes/windows, `⚙️engine`, `🏗️builder`, `📚️examples`
fixtures, `🌉️wasm` bridges, `#[cfg(test)]` fixtures, and `🟦️component.ts` glue.

- App commands that replaced the whole document (`setSnapshot`, `setSnapshotJson`,
  import/load-example/clear) must be re-routed to `ArtifactStore::reset` (outside history, clears
  undo/redo) or re-expressed as targeted semantic mutations. Remove the now-dead command routes
  and their payload structs.
- App-local `NoMutation` command structs are cosmetic debt — rename them to something honest
  (e.g. `NoOperation`) so the token disappears.
- Comments count: the policy rule greps raw file content INCLUDING comments and doc-comments.
  Reword any prose that names the banned tokens.

## Step 6 — schema description files

Rewrite the facet's grammar and protocol descriptions honestly — one rule/record per mutation
slug, no leftover generic shapes:

- text set: `📖️component.grammar.semio` (+ any `.g4`/`.ebnf` siblings), `🔗️component.graphql`,
  `🔣️component.json`, `🛰️component.proto`
- binary set: `📡️component.protocol.semio` (+ `.abnf`/`.ksy`/`.spicy` siblings) — one
  `record <VariantPascal> tag N`, tags assigned 1..N in variant order (= grammar alternation
  order), append-only afterwards.
- grammar keyword = slug without emoji; args address-first, then `new-*`, then payload blocks:
  `rename-layer name=walls new-name=partitions`.
- `🟦️component.ts` mirrors must export real types, not `export {};`, and must exist beside every
  triad `🦀️component.rs`.

## Step 7 — tests

Extend the facet's EXISTING `#[cfg(test)]` / `🧪️Tests` region — never add a new test file. Per
mutation kind, call the testkit law helpers from
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`:
`assert_mutation_inverse_law`, `assert_mutation_diff_absorb_law`,
`assert_diff_algebra_between_law`, `assert_diff_algebra_inverse_law`, plus
`assert_op_text_binary_equivalence` where the facet has text/binary codecs. Also implement
`DiffAlgebra` for the artifact's diff type if it is missing (per-field fold).

## Step 8 — gate and report

1. `cargo check -p <your crate>` — must be clean of errors caused by your edits. Capture the
   error list before your first edit so you can prove the delta.
2. `cargo test -p <your crate> --lib 2>&1 | tail -40` — run it, read it, report the real numbers.
3. `bun ./📜️script.ts policy 2>&1 | tail -20` — no NEW high-priority breach kinds.

Write `📓️<wave>-reports/<facet-key>-report.md` in the ticket folder containing:
`facet`, `status` (done / blocked-churn / partial), `mutationsCreated` (slug → verb → superseded
old variant), `genericVariantsRemoved`, `filesTouched` (created/updated/removed),
`sharedFileRequests`, `allowlistKeysToRemove` (repo-relative paths now free of banned tokens),
`gates` (exact command outcomes), `lawTests` (which laws, which kinds, pass counts), and any
deviations with justification.

Reply in chat with at most 3 lines pointing at the report — never paste the report into chat.
