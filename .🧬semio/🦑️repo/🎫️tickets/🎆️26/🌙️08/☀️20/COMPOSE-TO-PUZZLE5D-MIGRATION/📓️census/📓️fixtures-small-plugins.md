# 🧪️ Handcrafted mutation fixtures — seven small plugins + the framework OS config tree

21 mutation leaves, 21 handcrafted cases, one per leaf. Every `after`, every
`🔺️diff/🔣️component.json` and every assertion was derived from a direct read of that leaf's own
`🔺️diff/🦀️component.rs` (the oracle), its `🦠️mutation/🦀️component.rs` payload struct and serde
attributes, and its `↩️inverse/🦀️component.rs`. No shared harness, no macro, no generic sweep: all
21 `🦀️component.rs` bodies are byte-distinct.

## 📋️ Coverage

| tree | leaf | case | outcome |
| --- | --- | --- | --- |
| `🪐️space` / `🪐️space` | `🌱create-artifact` | `appends-artifact-3-to-the-index` | applied |
| | `🗑️delete-artifact` | `removes-artifact-2-from-the-index` | applied |
| | `🏷️rename-artifact` | `renames-artifact-1` | applied |
| | `🕒touch-artifact` | `stamps-artifact-1-with-a-new-editor` | applied |
| `🪐️space` / `🏠️home` | `🔢️change-catalog-generation` | `bumps-the-catalog-generation-to-7` | applied |
| `📜️imperative` | `🌱create-step` | `rejects-a-duplicate-step-id-at-the-root-path` | rejected · `mutation.duplicate-id` |
| | `🗑️delete-step` | `rejects-a-root-step-id-addressed-inside-a-branch-body` | rejected · `mutation.target-missing` |
| | `🔀reorder-steps` | `warns-that-an-over-clamped-index-leaves-the-tail-step-in-place` | applied + `mutation.no-op` |
| | `🔧edit-step-params` | `warns-that-step-1-already-carries-the-requested-params` | applied + `mutation.no-op` |
| `✒️writer` | `🏷️rename-writer` | `renames-the-document-to-mission-brief` | applied |
| | `🔗change-uri` | `republishes-the-brief-under-a-new-uri` | applied |
| | `🌐change-language` | `switches-the-brief-from-plaintext-to-markdown` | applied |
| | `✏️edit-text` | `warns-that-the-brief-body-is-unchanged` | applied + `mutation.no-op` |
| `🪵️sourcing` | `🌱create-curated-item` | `appends-a-steel-plate-to-the-curation` | applied |
| | `🗑️delete-curated-item` | `removes-the-clt-panel-from-the-curation` | applied |
| | `🔢change-curated-item-count` | `raises-the-glulam-beam-count-to-20` | applied |
| `🔋️energy` | `♻️replace-model` | `degrades-an-empty-model-payload-to-a-no-op` | applied + `mutation.no-op` |
| `🎪️demonstrator` | `✒️change-schema` | `retags-the-playground-document-schema` | applied |
| framework `💻️os/🎚️config` | `📌️set-default-app` | `repins-the-cad-editor-to-the-drafting-app` | applied |
| | `🧹clear-default-app` | `unpins-the-cad-editor-and-keeps-the-viewer-pin` | applied |
| | `🛡️change-merge-policy` | `tightens-the-authority-to-vigilant` | applied |

Rejected cases carry an empty `🔺️diff/🚫️component.absent` (0 bytes) and an `➡️after` byte-identical
to `⬅️before`, per contract D6. Applied cases carry `🔺️diff/🔣️component.json`.

## 🔤️ Serde shapes verified per tree (not assumed)

- `SSpaceMutation` / `SHomeMutation` / `ImperativeMutation` / `WriterMutation` / `SourcingMutation` /
  `OpeningConfigMutation` / `MergePolicyConfigMutation`: `#[serde(tag = "mutation", rename_all = "camelCase")]`.
- **`PlaygroundMutation` has NO `#[serde(tag …)]` and its payload has no `rename_all`** — it encodes
  EXTERNALLY tagged with a snake_case field: `{"ChangeSchema": {"new_schema": "…"}}`. The
  demonstrator fixture is the pin on that; copying the puzzle5d shape here would have been wrong.
- `MergePolicy` carries no `rename_all` either — wire spellings are `"LaissezFaire"` / `"Normal"` /
  `"Vigilant"`, PascalCase.
- Every diff container (`SSpaceDiff`, `SHomeDiff`, `ImperativeDiff`, `WriterDiff`, `CurateDiff`,
  `EnergyModelDiff`, `PlaygroundDiff`) is `#[serde(default)]` with no `skip_serializing_if` on any
  field, so the committed JSON emits **every** field, `null` for the untouched ones. Checked field by
  field against each struct's declaration order.
- `PathRef` (imperative) is the exception: both fields carry `skip_serializing_if = "Option::is_none"`,
  so a root-addressed payload serializes `"pathRef": {}`.
- `EnergyModelSnapshot::referenced_model` carries `skip_serializing_if`, so it is absent from the
  committed snapshots entirely.
- `Dictionary` (neural) is `#[serde(transparent)]` — `newParams` rides as a bare JSON object.

## 🕸️ The content-addressed-child wall (why some cases are no-ops / rejections)

Four of these artifacts persist their substantive content as **content-addressed CHILD handles**
whose `child_id` is `format!("…-{hash:016x}")` over a `DefaultHasher` of the child snapshot's JSON:

| artifact | minting fn | field(s) |
| --- | --- | --- |
| `📜️imperative` | `imperative_flow_child_handle` | `flow` (and `text`) |
| `✒️writer` | `document_child_handle` | `document` |
| `🔋️energy` | `energy_children_from_model` | `structure` + `zones` |

A mutation that really changes content mints a NEW handle, so its `➡️after` and its
`🔺️diff/🔣️component.json` would have to contain a `DefaultHasher` digest. That digest cannot be
hand-authored: it is not computable without running the crate, and committing one would (a) be a
guess and (b) break on any Rust std change to `SipHash`. **Hand-forging it would be exactly the kind
of parallel-implementation fake the recipe forbids for the binary codecs.**

So for those leaves the fixtures drive the branches of the SAME oracle that are fully determined
without a hash — and they are real, load-bearing branches, not filler:

- **`🌱create-step`** — Fatal `mutation.duplicate-id` at the root path (the container-check branch is
  deliberately unreachable there).
- **`🗑️delete-step`** — Error `mutation.target-missing` where the id EXISTS at the root but the
  payload addresses `step-3`'s `then` body: pins that `resolve_steps` honours the `PathRef`, not mere
  existence.
- **`🔀reorder-steps`** — the `to_index.min(ids.len())` clamp landing a tail step back on itself.
- **`🔧edit-step-params`** — the whole-dictionary equality guard.
- **`✏️edit-text`** — the body-equality guard; the assertion that matters is
  `applied.document.child_id == base.document.child_id`, i.e. an unchanged body must not re-mint a
  content address.
- **`♻️replace-model`** — both of that oracle's chained behaviours at once: `unwrap_or_default()`
  degradation of a payload that is not a full `Model`, meeting `energy_model`'s documented fail-soft
  to `Model::default()` on an unresolved handle.

Where the working scene is reachable, the test **hydrates it** rather than pretending it is empty:
imperative's cases call `cache_imperative_flow(&snapshot.flow.child_id, &cached_program())` and
writer's calls `cache_writer_document_text(…)`, each with its own handcrafted program/body. Energy
exposes no cache-seeding entry point keyed by an arbitrary `child_id` (only
`energy_children_from_model`, which mints its own id), so its fixture documents and exercises the
unresolved-handle state instead.

`🪐️space`, `🪵️sourcing`, `🎪️demonstrator` and the framework config tree have no content-addressed
children in the mutated fields, so all of their cases are ordinary applied mutations with real,
non-empty diffs.

## 🔌️ Wiring

`#[cfg(test)] #[path = …] mod tests_<case_with_underscores>;` added immediately after that leaf's
`pub mod inverse;` line, at the same indentation, in each plugin's own
`📦️packages/🦀️rust/📦️glue.rs`:

- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` — 5
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/📦️glue.rs` — 4
- `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs` — 4
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs` — 3
- `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` — 1
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📦️glue.rs` — 1

The framework OS config tree is **not** mounted from the kernel crate — it is mounted from
`semio-framework-plugin-host`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📦️glue.rs`, module
`opening_config::mutations::…`). The two opening-preferences fixtures are wired there, beside the
existing `pub mod mutation;` lines — 2.

`🛡️change-merge-policy` is the exception: it is a **self-contained facet** (schema + dispatch enum +
`MutationDiff` impl all folded into its own `🦀️component.rs`) that is mounted by **no crate at all**,
by design — its own module doc records that its lease was scoped to
`🧬️mutations/🛡️change-merge-policy/**` and that it matches the "NOT YET wired into any crate's
`📦️glue.rs`" precedent. It is also already written in the **de-async target style** (`fn diff`,
`fn apply`, no `.await`), whereas the opening-config facet next to it is still fully async
(`async fn diff`, `.await` at every call site). Mounting the merge-policy facet into plugin-host
today would therefore introduce a sync/async mismatch into a crate that is currently consistent —
out of scope for this ticket and explicitly the kind of workspace repair this lease must not attempt.
Its fixture is instead declared **inside the facet's own `🦀️component.rs`**, right after the
`pub use super::change_merge_policy::mutation::{…};` re-export:

```rust
#[cfg(test)]
#[path = "🧪️tests/tightens-the-authority-to-vigilant/🦀️component.rs"]
mod tests_tightens_the_authority_to_vigilant;
```

`#[path]` on a non-inline `mod` resolves relative to the directory of the file declaring it, i.e.
`🧬️mutations/🛡️change-merge-policy/`, so the fixture travels with the facet and starts running the
moment anyone mounts it — 1. Total: 21 wired test modules.

## ⏳️ Async style

The recipe asks for the de-async target style (no `.await`). That is what 19 of the 21 fixtures use,
matching the code they sit next to. The two **opening-preferences** fixtures are the exception: that
facet — and everything it calls, down to `MutationOutcome::diff`/`worst_level`/`messages` and
`MutationDiff::apply` — is still genuinely async in its own files, and it IS mounted, so a no-await
test there would not compile. Those two use `#[semio_framework_async_macros::async_test] async fn`
with `.await`, exactly as `📌️set-default-app`'s and `🧹clear-default-app`'s own committed
`#[cfg(test)] mod tests` already do. The repo-wide de-async sweep will convert them along with the
files they mirror. `🛡️change-merge-policy`'s fixture is sync (`#[test] fn`), matching its own
already-de-asynced facet.

## ✅️ Verification performed

```
cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust && bun ./📜️script.ts fixtures lint --by-tree
🧬️ 115 artifact mutation trees · 1558 mutations · 623 covered · 935 uncovered
⚠️ 4872 derived-encoding gap(s) pending `fixtures generate`
❌️ 957 error(s)
```

None of the 957 errors and none of the by-tree uncovered rows name any of the eight trees in this
slice (grep for `🪐️space|📜️imperative|✒️writer|🪵️sourcing|🔋️energy|🎪️demonstrator|💻️os/🎚️config`
over the full lint output returns 0 matches) — all eight are at 0 uncovered with zero errors. The
totals are much larger than at the start of this session because peers are landing their own trees
concurrently.

Structural checks (all green):

- 21 test `🦀️component.rs` files; all 105 `include_str!` targets resolve on disk.
- All five/four per-case JSON files present per contract D6; every one parses.
- Both rejected cases carry a 0-byte `🔺️diff/🚫️component.absent` and no `🔺️diff/🔣️component.json`.
- Every `#[path]` in all eight wiring files resolves; 21 `mod tests_*;` declarations found.
- `rustfmt --edition 2021 --emit stdout` parses all 21 test files and all eight wiring files, rc 0.
- All 21 test bodies are byte-distinct (sha256): no case would pass unchanged for another mutation.

**No `cargo` was run** — not workspace-wide and not per-package. The framework work is in
`semio-framework-plugin-host`, not `semio-framework-os-kernel`, so the one permitted
`cargo check -p semio-framework-os-kernel` did not apply and was not invoked. **No test in this slice
is claimed to pass.** No modifying git command was run; no ticket was opened, closed or reopened.

## ⚠️ Notes for the ticket owner

1. The brief said the framework OS config tree has 3 mutations. On disk it has **5** leaf
   directories; `🪪️sign-in` and `🚪️sign-out` carry only `🦠️mutation/🟦️component.ts` (no
   `🦀️component.rs`), so the fixture lint does not count them and neither did I. The three Rust
   leaves are covered.
2. The brief said that tree is wired from the OS kernel crate. It is actually wired from
   `semio-framework-plugin-host` (see 🔌️Wiring above).
3. `🛡️change-merge-policy` is unmounted repo-wide and already de-asynced while its siblings are not —
   flagged above; someone owns finishing that facet's wiring.
4. The `📜️imperative` / `✒️writer` / `🔋️energy` content-hash wall means those five leaves can never
   get a hand-authored *content-changing* applied case. If richer cases are wanted there, the
   fixtures must be **generated** from the crate (a `fixtures generate` mode that runs the oracle and
   writes the resulting handle), not hand-written.
