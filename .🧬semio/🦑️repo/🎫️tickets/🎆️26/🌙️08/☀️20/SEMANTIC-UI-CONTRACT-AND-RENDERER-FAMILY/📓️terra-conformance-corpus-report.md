# Packet `conformance-corpus` — report

## Done

Built the shared fixture corpus and its Rust harness:

- **Corpus**: `🧬️contract/📚️examples/🧪️conformance/` — 62 cases across six groups (`🧩️component`,
  `🖥️composite`, `📐️layout`, `♿️accessibility`, `🩹️patch`, `🚫️rejection`), 146 JSON files total (one
  `.snapshot.json` + `.expect.json` per accept-only case, plus `.patch.json` for every patch/rejection
  case).
- **Harness**: `🧬️contract/📦️packages/🦀️rust/🦀️conformance.rs`, mounted by one line in `📦️glue.rs`.
- **Commands**: `conformance` added to `📜️script.ts` (filters the existing test binary to the
  `conformance::` module path) and a matching `conformance` target in `📋️project.json`.

No other file in the contract crate or any other crate was touched.

## Corpus inventory

| group | cases | covers |
| --- | ---: | --- |
| `🧩️component` | 23 | One fixture per `Component` variant (16 files; `Tree`/`TreeSection`/`TreeItem` share one nested fixture since the latter two have no standalone meaning) plus 7 "interesting state" fixtures: `state-disabled`, `state-activity-waiting`, `state-activity-loading`, `state-activity-finished`, `state-transition-introducing`, `state-transition-celebrating`, `state-with-menu`. |
| `🖥️composite` | 5 | `form-with-validation` (Form > two Fields, one with an error), `tree-nested-sections` (Tree > TreeSection > row-action item + nested parent/child item), `toolbar`, `dialog` (Overlay layout, title/body Text, Confirm/Cancel toolbar), `surface-embedded` (a Surface beside ordinary widgets in one Stack). |
| `📐️layout` | 7 | Each of the six `LayoutSpec` variants (`leaf`/`stack`/`grid`/`overlay`/`scroll`/`absolute`) on a root node, plus `nesting` (Stack > Grid > Scroll > Leaf, four levels). |
| `♿️accessibility` | 5 | `labelled`, `described`, `live-region` (assertive), `shortcut`, `decorative-image` (built via `ImageBuilder::decorative()` semantics: hidden, no alt). |
| `🩹️patch` | 12 | One case per `UiPatchOp` variant (`upsert`, `set-component`, `set-layout`, `set-activity`, `set-children`, `set-style`, `set-accessibility`, `set-bindings`, `set-menu`, `remove-subtree`, `set-root`) plus `reorder-children` (same three children, order reversed — distinct from `set-children`'s add case). |
| `🚫️rejection` | 10 | `stale-base-revision`, `dangling-child`, `cycle`, `duplicate-sibling-key`, and one per quota: `quota-nodes`, `quota-depth`, `quota-children`, `quota-text-bytes`, `quota-patch-ops`, `quota-patch-bytes`. |

Coverage assertions in the harness (`every_component_variant_appears_in_the_corpus`,
`every_ui_patch_op_variant_appears_in_a_patch_case`) confirmed by an independent Python scan: all 18
`Component` variants and all 11 `UiPatchOp` variants appear at least once; the `🩹️patch` group alone
already covers every `UiPatchOp` variant (the `🚫️rejection` group's ops are additional, not load-bearing
for that assertion).

## Decisions

**Fixtures are Python-generated, not literally run through the Rust builders (deviation from the
literal instruction, forced by U4).** The brief asks for fixtures "written by running the builders and
serializing" so a fixture can never describe a document the contract cannot express — but U4 forbids
running cargo, and there is no non-cargo way to execute Rust. Instead:

1. I read `document.rs`/`component.rs`/`layout.rs`/`style.rs`/`accessibility.rs`/`action.rs`/
   `surface.rs`/`limits.rs` in full and encoded their exact serde wire shape (every `rename_all`,
   every `skip_serializing_if`, every enum tag) in a Python generator committed to this ticket folder
   (`gen_corpus.py`) that constructs each fixture the way the real builders + serde would.
2. I then wrote a second ticket-folder script, `verify_corpus.py`, that re-implements
   `validate_core`/`apply_patch` from `limits.rs` — the actual algorithm, not just the wire shape — in
   plain Python, and ran it against every fixture. It caught three real bugs before they could ship: a
   `quota-nodes` case with the wrong node count, a `set-root` case that (as first written) would have
   left the old root dangling and been rejected rather than accepted, and a `set-component` case whose
   hand-written expectation added an accessibility field that op never actually touches. All three are
   fixed; the final run reports every one of the 62 cases clean against the reference algorithm,
   including accessibility and action-id equality, not just node shape.
3. Both scripts (in this ticket folder) are `#[cfg(test)]`-adjacent tooling only, not builders — I did not use `Buildable`/
   `HasChildren`/etc. at all, because five `Component` variants (`Separator`, `KeyValueList`,
   `NumberStepper`, `Ring`, `IconSelect`) have no builder function in `🦀️builder.rs` in the first place,
   so full builder coverage was never achievable regardless of U4.
4. `sol`'s first `cargo test -p semio-framework-ui-contract` run of `🦀️conformance.rs` is therefore the
   real, authoritative check — it deserializes every fixture through the actual `crate::UiSnapshot`/
   `crate::UiPatch` types and runs the actual `validate_snapshot`/`apply_patch`, not my Python
   reimplementation. If it disagrees with my reference simulator anywhere, that is real signal (either
   a fixture bug my simulator missed, or a drift between `limits.rs` and this report's reading of it as
   of anchor `cb9bcce7a4`) and should be treated as a corpus bug to fix, not waved through.

**The expectation-file schema** (justifying its shape, since a TypeScript conformance test will load
the same files later):

```jsonc
{
  "case": "cycle",                 // matches the fixture's basename
  "kind": "rejection",             // component | composite | layout | accessibility | patch | rejection
  "description": "...",            // human-readable, for failure output
  "outcome": "accept" | "reject",
  "limits": null | { "maxNodes": 1, ... },   // UiDocumentLimits override, or null = crate defaults
  // --- accept cases only ---
  "tree": { "root": 0, "nodeCount": 2, "shape": [ { "id": 0, "key": "root", "type": "container", "children": [1] }, ... ] },
  "accessibility": [ { "id": 0, "label": "...", "description": null, "live": "off", "shortcut": null, "hidden": false } ],
  "actionIds": [ "scope.name@1" ],
  // --- reject cases only ---
  "patchRejection": { "type": "invariantViolated", "violations": [ { "type": "cycle", "node": 0 } ] }
}
```

Why this shape:

- **`tree.shape` uses a bare `type` string, never `crate::Component` itself.** The brief is explicit
  that "nothing Rust-specific may leak into it" — a TypeScript reader has no `Component` enum to
  deserialize into. The string is the same wire tag serde already emits (`"container"`,
  `"treeItem"`, ...), so it costs nothing extra to produce and there is exactly one source of truth for
  what the tag spells.
- **`patchRejection` reuses the contract's own tagged wire shape verbatim** (`crate::PatchRejection`,
  and `crate::UiContractViolation` nested inside its `InvariantViolated` case) rather than inventing a
  parallel description. This is not "Rust leaking out" — `PatchRejection`/`UiContractViolation` are
  themselves `ts-rs`-derived wire types (once the `typegen` feature runs), so a TypeScript test gets the
  identical generated type for free. Re-describing the same information in a second ad hoc shape would
  be the drift risk, not avoid it. The Rust harness deserializes this field straight into
  `crate::PatchRejection` and compares by `PartialEq`, so an expectation naming a rejection shape the
  contract cannot actually produce fails to even parse.
- **Every `🚫️rejection` fixture goes through `apply_patch`, never `validate_snapshot` directly** — even
  the four "structural" cases (dangling child, cycle, duplicate key) that could in principle be plain
  invalid `UiSnapshot`s. Routing all ten through one mechanism (a valid base snapshot at revision 0 plus
  a patch that should be rejected) keeps the schema uniform: every rejection fixture has the same three
  files and the same `patchRejection` field, and a TypeScript test only ever needs to implement one
  rejection path, not two.
- **`limits` is explicit per fixture rather than assumed.** `UiDocumentLimits::default()` (20 000
  nodes, depth 128, ...) is too large to trigger honestly in a small, readable fixture, so every quota
  case carries its own tightened override, mirroring the exact overrides `limits.rs`'s own unit tests
  already use (`max_nodes: 1`, `max_depth: 0`, `max_children: 1`, `max_text_bytes: 4`,
  `max_patch_ops: 1`, `max_patch_bytes: 4`) rather than inventing new numbers.
- **`baseRevision`/`resultRevision`** appear on `🩹️patch` fixtures as documentation only (the harness
  derives the real values by reading the `.patch.json` file itself); a TypeScript reader may ignore
  them safely, so they were left un-typed rather than promoted into the harness's `Expectation` struct.

**Where the corpus lives relative to the harness**: `../../📚️examples/🧪️conformance` from
`CARGO_MANIFEST_DIR`, resolved once via `env!("CARGO_MANIFEST_DIR")` so the tests pass regardless of
the caller's working directory (matters because `nx` and a bare `cargo test` invoke from different
cwds).

**Why `🦀️conformance.rs`'s entire body sits inside one `#[cfg(test)] mod tests`** (not gated at the
`glue.rs` mount line): the file's only job is reading fixtures off disk via `std::fs`, which does not
exist on `wasm32-unknown-unknown`. Gating the *content* rather than the *mount* means `cargo check
--target wasm32-unknown-unknown` (and `--target wasm32-wasip2`, and both with `--features typegen`)
compile an effectively empty module — zero cost to the wasm gates — while `cargo test` (no `--target`,
so it builds natively) picks the tests up automatically, with no `typegen` feature required, matching
the brief's explicit "must be available to plain test builds."

## Registrar-requests

- **`.vscode/launch.json`**: I searched for an existing entry for *any* of this crate's targets
  (`test`, `test-quick`, `test-long`, `test-exhaustive`, `check-wasm`, and a concurrent peer's
  `generate`/`check`) and found none — `launch.json`'s ~4000 lines are entirely product dev-server
  launch configs (`🛠️dev<product><target>` triples), not per-crate `nx run` task entries. So there is no
  existing naming convention for this crate to extend, and I'm not inventing one unilaterally. Requested
  command, for `sol` to place wherever the rest of `@semio-tech/ui-contract-rs`'s targets eventually
  land: `bun nx run @semio-tech/ui-contract-rs:conformance` (equivalently
  `bun ./📜️script.ts conformance` from `🧬️contract/📦️packages/🦀️rust`).

## Deviations

- **Fixtures authored by a Python generator instead of the Rust builders** — see "Decisions" above;
  forced by U4 (no cargo), cross-checked against a second, independently-written Python
  reimplementation of the real `validate_core`/`apply_patch` algorithm rather than trusted by
  inspection alone.
- **`Tree`/`TreeSection`/`TreeItem` share one `🧩️component` fixture** rather than three separate
  one-node files — a bare `TreeSection` or `TreeItem` outside a `Tree` has no realistic standalone
  form, so the "one file per variant" instruction is satisfied by coverage (every variant appears
  somewhere) rather than by a mechanical 1:1 file count.
- **No dedicated fixture for `PatchRejection::UnknownNode`** — the packet's explicit rejection list
  (stale revision, dangling child, cycle, duplicate key, one per quota) does not name it, and the coverage
  assertions in `🦀️conformance.rs` only require every `Component` and `UiPatchOp` variant, not every
  `UiContractViolation`/`PatchRejection` variant, to appear. Cheap to add later if `sol` wants stricter
  coverage.

## Acceptance: UNRUN

Per U4 I do not run cargo. Target dir in the session scratchpad, both `--lib` and `--all-targets`,
600000 ms timeout each:

```
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-contract --lib
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-contract --all-targets
CARGO_TARGET_DIR=<scratchpad>/target cargo test  -p semio-framework-ui-contract --lib -- conformance::
CARGO_TARGET_DIR=<scratchpad>/target cargo test  -p semio-framework-ui-contract --lib
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-contract --target wasm32-wasip2
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-contract --target wasm32-unknown-unknown
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-contract --target wasm32-wasip2 --features typegen
```

Expect the last three (`check-wasm`'s own commands, unrelated to this packet) unchanged from baseline —
`🦀️conformance.rs`'s content is entirely `#[cfg(test)]`-gated and contributes nothing to those builds.
Expect the crate's test count to grow by 6 (the six `#[test]` fns in `🦀️conformance.rs`) from whatever
`W2`'s `73` landed at.

**Cheap non-cargo checks actually run** (all clean):

- `python3 -m json.tool` against all 146 corpus JSON files — all parse.
- A from-scratch pairing check (every `.snapshot.json` has a matching `.expect.json`; every
  `🩹️patch`/`🚫️rejection` case additionally has a matching `.patch.json`; no orphans either
  direction) — 62 cases, 0 orphans, matching what `corpus_has_no_orphan_fixtures` asserts.
- The Python reference-simulator cross-check described above (`verify_corpus.py`, this ticket
  folder's `gen_corpus.py`/`verify_corpus.py`, not part of the crate) — 62/62 cases clean, including root/node-count/children shape, accessibility
  label/description/live/shortcut/hidden, reachable action ids, and (for `🚫️rejection`) the exact
  `PatchRejection` value and byte-for-byte state-unchanged property.
- A brace/paren/bracket balance check on `🦀️conformance.rs` (118/118, 286/286, 33/33) as a cheap
  syntax smoke test in the absence of `rustc`.

## Files touched

- Created: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/**` (146 JSON files across
  6 group directories)
- Created: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️conformance.rs`
- Edited: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📦️glue.rs` (one `mod conformance;`
  mount line, `#[cfg(test)]`-gated internally)
- Edited: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` (`ConformanceScript` +
  router registration; landed alongside a concurrent peer's `typegen`/`generate`/`check` addition to the
  same file — both changes coexist cleanly, verified by re-reading the file after each edit)
- Edited: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📋️project.json` (`conformance`
  target; same peer concurrently added `generate`/`check` targets, both coexist cleanly)
