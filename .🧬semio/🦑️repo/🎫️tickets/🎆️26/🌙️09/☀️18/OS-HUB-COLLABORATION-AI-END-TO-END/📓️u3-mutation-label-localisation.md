# U3 — Localising `MutationKind::label()` (history/undo panel)

Slice U3 of ticket 26/09/18. Sources: `📓️g1-goal-gap-audit.md` §3 (i18n drift risk),
`📓️g5-ux-completeness-audit.md` §1.3 item 1 (hard-coded English mutation labels).

Status: **landed and checked natively + on the TS side**; live shell verification deferred (§5).

## 1. Measured surface (own measurement, not the audit's)

G5 counted 2690 sites in `✏️s/🔌️plugins`. Measured over `✏️s` + `🧰️framework` (`grep -c "fn
label(&self) -> String"`, then region-anchored):

| metric | count |
|---|---|
| `fn label(&self) -> String` declarations in the tree | 2801 |
| of those, inside an `impl {Mutation,CompositeMutation,Semantic}Kind… for` region | 2795 |
| trait declarations (not impls) | 3 |
| inherent `label()` on unrelated types (`TouchedPaths`, `Planner`, `World3dStatusPill`) | 3 |
| files carrying at least one site | 2791 |
| crates carrying at least one site | 107 |
| distinct English template strings | 2368 |
| distinct content words in those templates | 1071 |

Body shapes: 1838 `format!(…)`, 774 `"…".to_string()`, 165 `"…".into()`, 34 multi-expression
(`match`/`if` over several literals). Worst crates: `semio-s-artifact-energy-model` 291,
`…-architect-program` 268, `…-stdio-semio` 233, `…-stdio-pdf` 148, `…-stdio-gltf` 120,
`…-norm-din16798` 62. (G5's per-plugin table under-counted: it sampled 17 of 34 plugins and missed
`🗄️stdio` (957 files) and `🔋️energy` (291) entirely.)

## 2. Design

**Carrier.** `LocalizedLabel` — the repo's existing locale × terminology matrix, built by
`LocalizedLabel::native(en, de)`, whose resolver matches on the generated `Locale` enum
**exhaustively with no catch-all arm**: adding a locale to `🖱️ui/🎚️axes/🔣️.json` fails every call
site until translated. No new type was invented and no `String` fallback path exists.

**Relocation (the one structural change).** `LocalizedLabel` lived in
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🏷️label/🦀️.rs`, and `semio-framework-ui`'s `wgpu`
feature depends on `semio-framework-os-kernel` — which owns `MutationKind`. Returning the carrier
from the trait would have been a crate cycle. Cargo cycles are per-crate, not per-feature, so a
feature split could not break it either. The axes and the label carriers therefore moved **down**
into the kernel, at `🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/`:

| moved from | moved to |
|---|---|
| `🖱️ui/🎯️targets/🧊️wgpu/🤖️generated/🦀️.rs` | `💻️os/🔨️modules/🌐️locale/🤖️generated/🦀️.rs` |
| `🖱️ui/🎯️targets/🧊️wgpu/🏷️label/🦀️.rs` | `💻️os/🔨️modules/🌐️locale/🏷️label/🦀️.rs` |
| `🖱️ui/🎯️targets/🧊️wgpu/🌐️locale-terminology/🧾️value/🦀️.rs` | `💻️os/🔨️modules/🌐️locale/🧾️value/🦀️.rs` |

`semio-framework-ui`'s wgpu root now re-exports them (`pub use dsl::{AppLabels, Label, LabelText,
Locale, LocalizedLabel, Terminology};`), so every `app_labels!` call site and every
`ui_wgpu::wgpu::Locale` reference resolves unchanged. The axes generator
(`🖱️ui/🎚️axes/📋️plan/🟦️.ts`) and its parity test were retargeted at the new path — the generated
file is still generated from `🖱️ui/🎚️axes/🔣️.json`, only its emit destination moved.

**Wire.** `label` is a `LocalizedLabel` from the leaf all the way to the renderer, resolved against
the active locale at render time rather than at dispatch time — so switching the shell locale
re-renders the whole ledger instead of leaving already-logged rows frozen in the locale they were
dispatched in:

`MutationKind::label` → `CommandLogAppend.label` → `CommandLogEntry.label` → `CommandView.label`
→ `kernel::HistoryEntry.label` (serde/`ToValue` camelCase, the `{terminology: {locale: text}}`
object the carrier already serialises to) → TS `HistoryEntry.label` → React `ShellHost` history
panel / the wgpu shell, each picking `uiLocale`.

`ContributedMutationPlanOutput.label` (the type-erased contributed-mutation seam) carries the same
carrier rather than a pre-resolved string, for the same reason.

## 3. Codemod

`🐍️u3-localise-mutation-labels.py`, table `🐍️u3-de-glossary.json` (2368 entries), built by
`🐍️u3-build-de-glossary.py` from the authored term dictionary `🐍️u3-de-terms.json`
(168 adjectives, 92 compounding modifiers, 790 nouns, 49 phrases, plus a declared locale-invariant
set of SI units / physical symbols / format and standard names).

Invariants asserted before a single byte is written:

1. **Region anchor** — a `fn label` is rewritten only when the nearest preceding `impl … for …`
   header names `MutationKind`, `CompositeMutationKind` or `SemanticMutation`. The three inherent
   `label()` methods in the tree (`TouchedPaths`, `Planner`, `World3dStatusPill`) are untouched.
2. **Span-keyed edits, descending** — each site is replaced by its own byte span, never by a text
   match, so no name-keyed rewrite can reach unrelated code (the 2026-09-03 codemod incident).
3. **Exhaustive table** — every string literal inside a rewritten body must have a glossary entry;
   a miss aborts the whole run before writing. There is no English pass-through.
4. **Path derivation** — `LocalizedLabel` is qualified with the module prefix the file already
   spells the trait with (2017 `protocol::`, 757 derived from the file's own `use … MutationKind`,
   21 `crate::os_spr::`), so no `use` line is edited.
5. **Idempotence** — a rewritten file offers no further site; a second run reports 0.

Generated shape, one expression:

```rust
fn label(&self) -> protocol::LocalizedLabel {
    protocol::LocalizedLabel::native(&format!("Connect \"{}\" to \"{}\"", self.from, self.to), &format!("\"{}\" mit \"{}\" verbinden", self.from, self.to))
}
```

Multi-expression bodies (`match`/`if` arms) are duplicated as two blocks, English literals in the
first, German in the second, so placeholder count and order are structurally identical. The
glossary builder additionally asserts that the ordered list of `{…}` holes is identical in both
languages for every entry (0 breaks over 2368 templates).

**Run**: `rewrote 2795 sites in 2791 files`; re-run reports `would rewrite 0 sites in 0 files`.
Per-file counts: `🗑️generated/u3-codemod-per-file.txt`; run log `🗑️generated/u3-codemod-run.txt`.

## 4. Verification

One cargo at a time, foreground, `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-u3`.
Warnings are quoted as proof the type-checker actually reached the code.

| check | result | capture |
|---|---|---|
| `cargo check -p semio-framework-os-kernel` | **green**, 2 warnings (both pre-existing `unnecessary qualification` in `🏪️store`) | `🗑️generated/u3-check-os-kernel.txt` |
| `cargo check -p semio-framework-plugin` | **green**, 41 warnings | `🗑️generated/u3-check-framework.txt` |
| `cargo check -p semio-framework` | **green**, 1m54s | `🗑️generated/u3-check-framework.txt` |
| `cargo check -p semio-framework-ui --features wgpu` (proves the re-export façade) | **green**, 1 warning | `🗑️generated/u3-check-artifacts.txt` |
| `cargo check -p semio-framework-plugin --features component-guest` | **1 error, not mine** (§5) | `🗑️generated/u3-check-artifacts.txt` |
| os TS typecheck (`bun ./📜️script.ts typecheck`) | 63 errors tree-wide, **0 of them in any file U3 touched**; the 4 hits in touched files are pre-existing (`Board2dWasmSession.pointerCancelScreen`, `ResolvedActionDefinition`, `PluginWasmHandle`, `consumes`) | `🗑️generated/u3-typecheck-os.txt` |
| vitest `-t "history cursor"` (the bilingual history-projection test) | **1 passed** | `🗑️generated/u3-vitest-history.txt` |
| vitest `-t "Set Active Example"` (navbar row now keyed on the action id) | **1 passed** | `🗑️generated/u3-vitest-history.txt` |

The vitest fixture is language-agnostic by construction: it feeds ONE ledger whose rows carry
`{native: {en, de}, reuse: {en, de}}` and asserts the SAME projection yields
`["Add Widget", "Move Widget", "Delete Selection"]` for `locale: "en"` and
`["Widget hinzufügen", "Widget verschieben", "Auswahl löschen"]` for `locale: "de"`, with identical
`actionIds` — i.e. a locale switch re-renders the ledger with no guest round trip and no English leak.

**No English-only fallback remains on this path.** Removed or never introduced:

- `MutationKind::label`/`SemanticMutation::label`/`CompositeMutationKind::label` cannot return a
  bare `String` any more; `LocalizedLabel::native` matches `Locale` exhaustively.
- `record_command`'s label fallback used to be
  `def.label.resolve(Terminology::Native, Locale::En).to_string()` with an explicit
  *"a fallback label resolves native/English pending a protocol change"* comment. It now clones the
  definition's full `LocalizedLabel`; an id with no definition becomes `LocalizedLabel::data(action_id)`
  (locale-invariant data, not untranslated English).
- The wgpu history panel used to print `entry.label` verbatim; it now resolves against the shell's
  own `is_de` axis.
- `navbarExampleIdFromHistoryUpserts` used to fall back to `label.includes("Set Active Example")` —
  an English-prose match that would silently stop working for a German shell. It is keyed on
  `SET_ACTIVE_EXAMPLE_ACTION_ID` alone now.
- `historyEntryLabelText` deliberately does **not** `?? label.en`: an axis the carrier does not
  carry renders empty (visibly wrong) rather than silently English. The codemod likewise aborts on a
  missing glossary entry instead of passing English through.

## 5. Honest gaps

1. **107 crates carry rewritten sites; 4 crates were checked.** `cargo check` on every one of them
   is many hours under fleet load and was not attempted. The rewrite is uniform and machine-generated
   from one template, and the two type-level consumers it can break (the trait signature and the
   `#[derive(Mutations)]` arm) are both proven by the kernel/SDK/framework checks — but "every plugin
   crate compiles" is **not** measured here and should be treated as unverified.
2. **A peer blocks per-artifact-crate checks right now.** `cargo check -p semio-s-artifact-flow-flow`
   and `-p semio-s-artifact-norm-din16798` both fail inside `semio-framework-plugin` under
   `component-guest`, on slice TC3b's uncommitted `artifact_app_genesis_pair` calling the private
   `dispatch_apply_exact` (`🔌️plugin/🦀️.rs:32857`) and a second missing symbol at 31101/31113.
   Zero label-related errors appear. Once TC3b lands, re-run those two checks.
3. **No wasm rebuild, no live shell.** S10/PZ1 own the wasm mutex; nothing was rebuilt, so the
   history panel was NOT observed rendering German at runtime. That verification belongs to the next
   `s` rebuild: boot a shell, switch the locale, confirm the History panel's rows change language
   without a new dispatch.
4. **Translation quality is generated, then hand-curated at the term level, not proof-read per row.**
   The 2368 German templates come from an authored dictionary plus German compounding rules
   (Fugen-`s`/`-n`, adjective declension, infinitive-final imperative). Spot-checked output is good
   ("Kante \"{}\" löschen", "Ebene \"{}\" in \"{}\" umbenennen", "Personengewinnzeitplan von
   Personengewinn {} ändern"), but long Eurocode compounds and a handful of hyphenated forms
   ("eigen-Gewicht") read stiffly. `🐍️u3-de-glossary.json` is the checked-in, hand-editable source of
   truth: fixing a row is a one-line edit there plus a codemod re-run on a fresh tree.
5. **English prose was not touched.** ~324 kebab-case op ids (`set-layer`, `remove-block`) remain the
   English side's text while the German side is real prose. Normalising the English is a separate
   slice; doing it here would have churned unrelated tests.
6. **`Locale::En` is still `#[default]`** in the generated axes enum (G5 §1.4's "implicit default
   language" finding). Nothing on the mutation-label path uses that default any more, but the
   `#[default]` attribute itself survives and is out of this slice's scope.

## 6. Files changed

**Moved** (3): the axes + label carriers from `🖱️ui/🎯️targets/🧊️wgpu/{🤖️generated,🏷️label,🌐️locale-terminology/🧾️value}`
into `💻️os/🔨️modules/🌐️locale/{🤖️generated,🏷️label,🧾️value}`, plus a new module root
`💻️os/🔨️modules/🌐️locale/🦀️.rs`.

**Framework (Rust)**
- `💻️os/📦️packages/🦀️rust/🦀️.rs` — mounts `os_locale`, re-exports the six names at the crate root.
- `💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` — the three trait signatures (`MutationKind`,
  `SemanticMutation`, `CompositeMutationKind`).
- `💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` — both generated `fn label` arms.
- `💻️os/🔨️modules/🌐️locale/🏷️label/🦀️.rs` — `Default` + `texts_mut()` on `LocalizedLabel`.
- `💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `CommandLogAppend`, `CommandLogEntry`, `CommandView`,
  `ContributedMutationPlanOutput`, `ChildEmit.labels`, `record_command`, both backfill paths, the
  wgpu history panel row, the retained close cursor.
- `🔨️modules/🎠️kernel/🦀️.rs` — `HistoryEntry.label`.
- `🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs` — re-export façade replacing the two mounts.
- 42 kernel-internal mutation files re-qualified from `crate::os_spr::` to `crate::`.

**Framework (TS)**
- `🔨️modules/🎠️kernel/🟦️.ts` — `HistoryEntry.label: LocalizedLabel` + `historyEntryLabelText`.
- `🔨️modules/🖱️ui/🎚️axes/📋️plan/🟦️.ts` and `🔨️modules/🖱️ui/🧪️tests/🎚️axes/🟦️.ts` — generator emit path.
- `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — `shellHistoryCursorDomV1` axes parameter,
  `navbarExampleIdFromHistoryUpserts` keyed on the action id.
- `…/🧱️elements/🏛️ShellHost/🟦️.tsx` — history panel row label, `data-history-json`.
- `…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — the bilingual history fixture + navbar test.

**Plugins/artifacts**: 2791 `🦀️.rs` files, 2795 rewritten sites, across 107 crates
(`🗑️generated/u3-codemod-per-file.txt`). 3749 files in the tree now contain `LocalizedLabel::native`.

**Ticket folder**: `🐍️u3-localise-mutation-labels.py` (codemod), `🐍️u3-build-de-glossary.py`
(table builder), `🐍️u3-de-terms.json` (authored dictionary), `🐍️u3-de-glossary.json` (2368-entry
checked-in EN→DE table), this report, and the five captures under `🗑️generated/`.
