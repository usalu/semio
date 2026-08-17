# W6 — semio-framework-os-kernel relocation: investigation verdict

## Verdict: ABANDON. Same class of over-corrected assumption as W5b (flow-core).

The claimed "inversion" (`semio-framework` depending on `semio-framework-os-kernel`) is
**not a violation of any of this ticket's three real rules** once the ticket's own explicit
scope text and the crates' self-declared role metadata are checked. No relocation is
architecturally justified. Recommend closing this line item with no file changes, same
disposition as W5b's flow-core finding.

## 1. What the glue.rs comment (lines 33-39) actually says

`🧰️framework/📦️packages/🦀️rust/📦️glue.rs`:

```rust
// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: mounted
// HERE, not in the os-kernel crate — its `semio_framework::{AppDefinition, MediaClass, MediaType,
// ConfigSpec, Terminology, Locale, …}` references need this crate's full assembled surface (mesh's
// media vocabulary, manifest's kernel types, ui_wgpu's Locale/Terminology — all re-exported below),
// which the wasm-safe os-kernel crate cannot depend on without a real dependency cycle (see the
// os-kernel glue.rs's own comment at the site this used to be attempted). The run crate's own
// `extern crate ... as workflow;` alias points here now, not at the kernel.
```

This is **not** a comment saying "framework depending on os-kernel is backwards." It explains
why the `workflow` module (which needs the framework crate's *assembled* surface — mesh +
manifest + ui_wgpu re-exports) is mounted in `semio-framework`'s own glue.rs rather than in
`semio-framework-os-kernel`'s glue.rs: doing the latter would require os-kernel to depend
*upward* on the very crate that already depends on it, i.e. a real cycle. The comment
**confirms the current one-way direction (`semio-framework → semio-framework-os-kernel`) is
intentional and correct**, and explains why it must stay one-way. It is evidence *against*
the relocation premise, not for it.

The actual "inversion" framing comes from the ticket's own `🎫️ticket.json` description
("semio-framework depends on the os product's semio-framework-os-kernel crate (inversion)"),
not from this comment.

## 2. Real dependency shape — accurate count

**The 175-file grep count was inflated by a stray, currently-checked-out git worktree.**
`.claude/worktrees/agent-af15980ad8f731e73` (confirmed via `git worktree list`: locked, at
commit `47e1a1deab`, same as the live tree's HEAD) is a full mirror of the entire repo. Every
raw grep hit in the live tree is duplicated there. Excluding it (and `target/`,
`node_modules/`) drops the raw-mention count from 175 to **88**.

Of those 88, precise `[dependencies]`-table matches (`package = "semio-framework-os-kernel"`,
exact string with a closing quote immediately after — this deliberately excludes sibling
crates that merely share the name prefix, e.g. `semio-framework-os-kernel-db`,
`semio-framework-os-kernel-neural-engine`, `semio-framework-os-kernel-dsl-derive`, which are
physically separate crates in different directories, not the crate under investigation):
**64 files** with a genuine dependency edge.

Of those 64, **9 are stale scratch `Cargo.toml`s inside old closed-ticket folders**
(`.🦑️repo/🎫️tickets/…/…-proof-crate`, `verify-shims/…`, `wire-format-proof/…`, etc.) — each
is a standalone `[workspace]` with its own relative-path members, not a member of the root
workspace, left behind as historical residue per this repo's "never delete ticket-folder
scratch files" rule. They do not compile as part of `cargo check --workspace` and are
irrelevant to any real consumer count.

**Genuine active workspace consumers: 55.** Breakdown by role:

| Category | Count | Detail |
|---|---|---|
| `✏️s/🔌️plugins/*` (plugin crates) | 35 | e.g. cad, gis, puzzle, block, remodel, playbook, forms, etc. |
| `✏️s/🔨️modules/*` (s shared modules) | 2 | `imperative`, `lang` |
| `✏️s/…/🧩️extensions/*` (plugin extensions) | 1 | `playbook/🧩️extensions/🌀️procedural` |
| `🧰️framework/🛍️products/💻️os/*` (os-product tree) | 8 | infinite, flow, run, renderer/wgpu, plugin, plugin-host, db, **os-host** |
| `🧰️framework/🔨️modules/*` (generic framework modules) | 6 | editor, compiler, ui, surface, schema, math |
| `🧰️framework/📦️packages/🦀️rust` (the crate under investigation) | 1 | `semio-framework` itself |
| `🌎️hub/📦️packages/🦀️rust` | 1 | separate product |
| `compose/client/lib/rs` | 1 | separate product |

The root `Cargo.toml`'s `[workspace.dependencies]` table (line 135) also defines the
canonical `semio-framework-os-kernel` path alias, but per its own header comment ("Purely
additive: no existing member below adopts `.workspace = true` yet") it is not yet wired to
any consumer — a 65th file to touch if renaming, but not a live edge today.

**Conclusion: the real number is 55 genuine consumers, not 175, and not even the ~80
originally estimated.** The over-count was ~3x from the stray worktree plus a smaller
inflation from unrelated ticket-scratch files and same-prefix sibling-crate name collisions.

## 3. Does this violate any of the ticket's three real rules?

**No.** Two independent lines of evidence:

**a) The ticket's own rule text already folds the os product into "framework."** From
`🎫️ticket.json`: *"🧰️framework (incl. the 💻️os product) must not know ✏️s or any plugin;
✏️s must not know concrete plugins; plugins must not know their extensions."* The os product
subtree is explicitly declared part of the framework side of the boundary, by the ticket's
own scope statement — not a separate downstream layer like `✏️s`. An edge from
`semio-framework` into `semio-framework-os-kernel` (physically under
`🧰️framework/🛍️products/💻️os/`) is an edge entirely inside the declared "framework" layer.
None of the three rules says anything about internal ordering *within* that layer.

**b) The crates' own self-declared role metadata confirms this independently.** Checked
`[package.metadata.semio]` across the relevant crates:

- `semio-framework` (`🧰️framework/📦️packages/🦀️rust`): `role = "framework"`, `id = "framework"`
- `semio-framework-os-kernel` (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`):
  `role = "framework"`, `id = "os-kernel"`
- `semio-framework-os` — the *actual* OS host/shell app
  (`🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust`): `role = "product"`,
  `id = "os-host"`

Both `semio-framework` and `semio-framework-os-kernel` are tagged `role = "framework"` —
they're peers in the same layer per the codebase's own taxonomy, despite one being nested
under `🛍️products/💻️os/`. The directory name "products" is misleading here: it does **not**
mean "downstream product layer" for `os-kernel` specifically. The crate that *is* the real,
correctly-downstream product in this area is `semio-framework-os` (`role = "product"`,
the OS host), which depends on **both** `semio-framework` and `semio-framework-os-kernel` —
textbook-correct product → framework direction, confirmed in its own `Cargo.toml`.

The original plan's "inversion" label conflated physical directory nesting
(`🛍️products/💻️os/…`) with layer semantics. Once role metadata and the ticket's own explicit
scope are checked, there is no inversion: it's `framework → framework`, both roles agree,
the only genuinely downstream crate in the neighborhood (`os-host`) already points the
correct direction.

**c) Checked `os-kernel`'s own dependency edges for leakage (the flow-core-style check).**
Its `[dependencies]` are 100% external crates (`async-trait`, `base64`, `blake3`,
`futures-lite`, `serde`, `thiserror`, `tokio`, `zip`, wasm-bindgen family) plus
`semio-framework-hash` (also `role = "framework"`). Zero dependencies on `✏️s` or plugin
crates. Its module content (`dsl`/`pack`/`spr`/`store`/`vcs`, 58 files) is genuinely
generic **except** one already-known, unrelated issue: two hardcoded `"s.stdio.gif"`
schema-id string literals in `🏪️store/🦀️component.rs` (lines 4653, 4656) — this is the
pre-existing "os modules use s.* schema ids" naming issue the ticket description already
lists separately for `space`/`workflow`/`store`, unrelated to crate placement, not fixed by
any relocation. (Also checked
`🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`, which hardcodes `✏️s/🔌️plugins/*` paths by
string — but the entire module is `#[cfg(all(test, feature = "dsl-fixture-sweep-full"))]`,
a non-default-feature, test-only repo-wide fixture harness explicitly documented as "never
a real dependency of anything." Not a runtime/production violation.)

Neither of these affects the relocation question — both would need fixing (or not) at their
current location regardless of whether the crate moves.

`.dependency-cruiser.cjs`'s existing `frameworkNoSRule` (`from: { path: "^🧰️framework/" }`,
warns on imports into `✏️s`) already implicitly treats the whole `🧰️framework/` tree —
including everything under `🛍️products/💻️os/` — as one unit for the framework↔s boundary,
consistent with (a) above. There is no dependency-cruiser or cargo-level rule anywhere in
the repo that flags intra-framework-tree edges as a violation; no such rule was ever built
because none was needed.

## 4. Feasibility, if it were pursued anyway (for completeness — not recommended)

Even at the corrected real scope (55 consumers, not 175 or ~80), a mechanical
Cargo-path-edit wave would concentrate real risk in two places, not pure text substitution:

- **Alias sprawl in root `Cargo.toml`**: ~10 different `[workspace.dependencies]` alias names
  (`semio-framework-os-kernel-db-state`, `-db-storage`, `-db-wal`, `-flow`,
  `-infinite-board-port-directed-dag`, `-infinite-canvas`, `-neural-engine`, `-db`, etc.)
  point at *sibling* crates that live in neighboring directories
  (`🔨️modules/🛢️db`, `🔨️modules/🌊️flow`, `🔨️modules/♾️infinite`, `🔨️modules/🧠️neural/⚙️engine`)
  under the same `🛍️products/💻️os/` parent, not at the kernel crate itself. These would
  need to stay put and have their relative paths re-verified even though the kernel crate
  moves out from under them — a coupled-neighborhood problem, not a flat rename.
- **Colocated siblings with mixed roles undermine the "true bottom layer" framing**: the
  `🛍️products/💻️os/🔨️modules/` area the plan would leave behind is not exclusively
  `role = "product"` material that os-kernel needs distance from — it also contains
  `role = "framework"`-adjacent siblings. A clean single-crate move assumed a tidier
  neighborhood than actually exists.

If the rule violation were real, fan-out by consumer-crate-cluster (mirroring W3's
15-agent/39-plugin split) would be the right shape for the 35 plugin consumers. But since
there is no real violation to fix, this sizing exercise is moot.

## Recommendation

Abandon the relocation, no file changes. Mirrors W5b's flow-core disposition exactly: an
assumption based on directory-nesting optics, contradicted once actual role metadata and
the ticket's own explicit rule scope ("framework incl. the os product") are checked. The
one real, unrelated finding surfaced in passing — two hardcoded `"s.stdio.gif"` schema-id
literals in `os-kernel`'s own `🏪️store/🦀️component.rs` (lines 4653, 4656) — is a leftover
of the separately-tracked `s.*`→`os.*` schema-id rename (already covered in spirit by
W4a/W4b for `space`/`workflow`/`store`) and can be swept up there; it does not depend on or
justify any crate relocation.
