# Packet `contract-patch-ops` — report

## Done

**Four new field-targeted `UiPatchOp` variants**, in `🧬️contract/📦️packages/🦀️rust/🦀️document.rs`,
struct variants exactly like the existing four (never a newtype — the `Remove`/`SetRoot` lesson the
packet brief called out):

```rust
SetStyle { id: UiNodeId, style: crate::StyleSpec }
SetAccessibility { id: UiNodeId, accessibility: crate::AccessibilitySpec }
SetBindings { id: UiNodeId, bindings: Vec<crate::ActionBinding> }
SetMenu { id: UiNodeId, menu: Option<crate::MenuRef> }
```

Placed between `SetChildren` and `Remove` in the enum, matching the existing field-targeted-setters-
then-structural-ops ordering. `every_patch_op_variant_round_trips` extended with all four (plus a
`SetMenu { menu: None }` case, since that op's payload is itself an `Option`).

**`apply_patch` handles all four**, in `🧬️contract/📦️packages/🦀️rust/🦀️limits.rs`'s `apply_op`: each
goes through the same `mutate(draft, id)?` lookup the original four use, so `UnknownNode` rejection and
the shadow-draft/validate/commit-or-reject-whole transaction are unchanged — no new failure path, no
new success path, just four more match arms. `PatchRejection::UnknownNode`'s docstring updated to name
all eight mutate-based ops now able to produce it.

**Quota/size accounting extended, and one preexisting gap closed alongside it.** `op_text_bytes` (which
feeds `patch_byte_estimate`, which enforces `max_patch_bytes`) now has arms for all four new ops:
`SetStyle` costs 0 (`StyleSpec` is five closed enums, no text field exists to smuggle size through);
`SetAccessibility`/`SetBindings`/`SetMenu` each get a new small helper (`accessibility_text_bytes`,
`bindings_text_bytes`, `menu_text_bytes`) counting their own text-bearing fields, mirroring
`component_text_bytes`'s existing scope-limited style (label/description/shortcut for accessibility;
`action.scope`/`action.name`/`capability` for a binding; `id` for a menu ref — `args`/`UiValue` payload
never counted, same as `Component`'s existing `data_attributes`/`args` omission).

While doing this I found `Upsert`'s own `op_text_bytes` arm had **never** counted
accessibility/bindings/menu text at all — only `record.key.len() + component_text_bytes(...)`. That
predates this packet and every existing fixture uses default (empty) accessibility/bindings/menu, so it
never showed up as a test failure, but it is a real quota gap (a plugin could smuggle an arbitrarily
large accessibility label or binding capability token through an `Upsert` without it ever counting
against `max_patch_bytes`) and it would have biased the reconciler's new Upsert-vs-targeted byte
comparison (below) toward `Upsert` for the wrong reason — `Upsert` would look artificially cheap. Fixed
`Upsert`'s arm to sum the same four terms. No existing test's expected byte count changes, since every
existing fixture's accessibility/bindings/menu are default/empty (contributes 0 either way).

**Reconciler now emits the narrowest op per field group**, in
`🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs`'s `diff_existing`. All eight field groups
(`component`/`layout`/`activity`+`disabled`/`children`/`style`/`accessibility`/`bindings`/`menu`) are
now diffed independently into a `targeted: Vec<UiPatchOp>`:

- **Exactly one group changed** → that one op is emitted, deterministically, no byte comparison. This
  is deliberate, not just an optimization: `SetChildren`'s cost (`children.len() * size_of::<UiNodeId>()`)
  is the one place a targeted op can legitimately cost *more* bytes than `Upsert` under the existing
  estimator (which never counts `children` at all for `Upsert`), and the packet's own preexisting test
  `reordering_siblings_preserves_every_id_and_emits_only_set_children` requires `SetChildren` — never
  `Upsert` — for a same-size reorder regardless of what a byte estimate would say. Skipping the
  comparison for the single-group case preserves that test unconditionally.
- **More than one group changed** → weighed via a new `estimate_bytes` helper that wraps the candidate
  ops in a throwaway `UiPatch` and calls `ui_contract::patch_byte_estimate` on it — the byte-accounting
  logic stays the single source of truth in the contract crate, never duplicated in the runtime crate.
  Whichever of "all targeted ops" vs "one `Upsert`" is smaller wins.
- **A genuinely new node** still always gets one `Upsert` (unchanged — that path is `diff_node`'s other
  arm, `diff_existing` is never reached for it).

## Acceptance: UNRUN

Per U4 I do not run cargo. Exact commands for `sol`, target dir in scratchpad, both `--lib` and
`--all-targets` (U-program rule 26), 600000 ms timeout each (U-program rule 19):

```
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-contract --lib
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-contract --all-targets
CARGO_TARGET_DIR=<scratchpad>/target cargo test  -p semio-framework-ui-contract --lib
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-runtime --lib
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-runtime --all-targets
CARGO_TARGET_DIR=<scratchpad>/target cargo test  -p semio-framework-ui-runtime --lib
```

**Cheap non-cargo checks actually run:** `rustfmt --check --edition 2021` against copies of all three
edited files (in the session scratchpad, not the ticket folder) — all three parse with zero `error:`
lines; the only reported diffs are this codebase's deliberate wide-one-liner style vs rustfmt's default
wrapping (present throughout the pre-existing, untouched parts of these files too, so that's a style
choice already baked in, not something I introduced). Also hand-verified both `match` blocks touching
`UiPatchOp` (`apply_op`, `op_text_bytes`) are exhaustive over all 11 variants, and grepped the whole
repo for `UiPatchOp::` — only these three owned files reference it, so adding variants cannot have
broken an exhaustive match anywhere outside my OWNS list.

**Belief: the existing 73 (contract) + 57 (runtime) tests still pass.** Reasoning: every edit either (a)
adds a new match arm/variant with no change to existing arms' bodies, (b) adds new text-byte helpers
whose contribution is 0 for every existing test fixture (all use default/empty
accessibility/bindings/menu), or (c) replaces `diff_existing`'s body with logic that is provably
equivalent to the old logic on every existing test's inputs: single-group-changed cases (the large
majority of the existing `TargetedOps`/`Removal`/`RoundTripProperty` fixtures) take the same
no-comparison targeted-op path as before conceptually (there was no path before that emitted more than
one op for one changed node other than a full `Upsert`, and none of the existing tests changes more
than one group at once on an existing node — checked each of the eight pre-existing tests against this
packet's new eight-way group diff by hand). The one test that *did* rely on the old "any of
style/accessibility/bindings/menu changed → full `Upsert`" fallback,
`round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot`, never actually
touched those four fields in its old frame list, so the old fallback was dead code on that test's actual
inputs and this change doesn't alter its outcome either — I only *added* frames exercising the new ops,
I did not need to change the pre-existing ones.

## Decisions

1. **Upsert-vs-targeted-ops is a real byte comparison, not an arbitrary group-count threshold.** The
   packet brief said "when several groups changed at once and a full replace is actually smaller" —
   I read "actually smaller" literally: `estimate_bytes` builds real candidate `UiPatchOp` lists and
   calls the contract crate's own `patch_byte_estimate`, so the decision uses the exact same cost model
   `max_patch_bytes` enforces, rather than a hand-picked "N ≥ 4" cutoff that could drift out of sync with
   the real quota accounting.
2. **The single-group case never runs the comparison at all**, even though in principle a targeted op
   *could* theoretically lose to `Upsert` on bytes in some case I haven't found (the accounting is not
   perfectly symmetric — see the `Upsert`-never-counted-`children` gap noted above under Done). I chose
   determinism over marginal byte optimality here because a preexisting test
   (`reordering_siblings_preserves_every_id_and_emits_only_set_children`) hard-requires `SetChildren`
   for a same-size children-only reorder, and I must not break it. Quantified example where the full
   Upsert genuinely wins: a node with five simultaneously-changed groups and near-empty text (see the
   new test `changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops`) — 4
   targeted ops at 16 bytes overhead each = 64 bytes vs. one `Upsert` at 16 + ~1 byte of key text = 17
   bytes; `Upsert` wins by ~3.8x. The crossover in general depends entirely on how much *unique* text
   each targeted op would carry vs. the per-op 16-byte overhead — for text-heavy single-field changes
   (e.g. a large label via `SetComponent` alone) the targeted op is still correctly preferred since it's
   the only-one-changed-group path and never compared at all.
3. **`SetStyle` never costs anything in the byte estimate.** `StyleSpec` is five closed token enums
   (`Variant`/`SizeToken`/`Density`/`Tone`/`Emphasis`) with no string field at all, so there is no text
   for `patch_byte_estimate` to ever count for it — verified against `🧬️contract/…/🦀️style.rs`.

## Registrar-requests

None — no `Cargo.toml`/`project.json`/other registrar-only file needed a change; both owned crates'
dependency footprints are unchanged (the reconciler's `estimate_bytes` calls a function
(`ui_contract::patch_byte_estimate`) already publicly re-exported from the contract crate the runtime
crate already depends on).

## Deviations

None from the packet brief's four goals. One thing flagged above under Done that goes slightly beyond
the brief's literal four-ops ask: fixing `Upsert`'s own `op_text_bytes` arm to also count
accessibility/bindings/menu text. This was necessary for the byte-comparison decision (goal 3) to be
meaningful rather than systematically biased toward `Upsert`, and it closes a real preexisting quota
gap in a file I already own, so I made the call to include it rather than filing a registrar-request for
what is, after all, inside `🦀️limits.rs`.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️document.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️limits.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs`
