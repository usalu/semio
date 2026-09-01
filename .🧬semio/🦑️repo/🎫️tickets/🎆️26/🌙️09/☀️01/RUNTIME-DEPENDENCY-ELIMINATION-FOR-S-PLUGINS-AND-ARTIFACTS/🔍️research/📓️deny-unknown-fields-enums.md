# `#[value(deny_unknown_fields)]` enforcement on enum containers

Fixes the correctness regression flagged for this ticket: 27 enums (16 production) carry
`#[value(deny_unknown_fields)]` but the derive only enforced it for `Data::Struct`. Enum containers
parsed successfully regardless of the attribute. This document is the acceptance record for the fix
in `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs`.

## Semantics chosen, per enum representation

The derive supports four enum wire shapes (module docstring at the top of `🦀️component.rs` has the
authoritative, load-bearing copy — this section is the rationale behind it):

- **Unit-only (bare-string)** — `Widget::Neuron` etc. decode from a plain `DslValue::String`. There
  is no object and therefore no extra-key slot: an unrecognized string is already a hard
  `"unknown variant"` error independent of the attribute. **Decision: not applicable.** Setting
  `#[value(deny_unknown_fields)]` on a unit-only enum is accepted and does nothing beyond what was
  already enforced. No code path added.

- **Externally tagged** (no `tag`, mixed variants — serde's own default) — `{"VariantName":
  payload}`. The outer shape is inherently exactly one key: `expand_from_value` already had an
  unconditional `__entries.len() != 1` check, independent of `deny_unknown_fields`. **Decision:** the
  attribute's own job here is scoped to a NAMED-field variant's own payload keys — checked against
  that variant's known field names. A single-unnamed-field variant's payload is hand it whole to that
  field type's own `FromValue` (its own `deny_unknown_fields`, if any, is that type's business).

- **Adjacently tagged** (`tag` + `content`) — checked at two independent levels:
  1. The outer object's keys must be a subset of `{tag, content}` — checked ONCE, before the tag is
     even read, since the allowed set does not depend on which variant matches.
  2. A NAMED-field variant's `content` object keys must be a subset of `{tag} ∪ that variant's field
     names}` — wait, corrected: subset of just that variant's own field names (the tag never appears
     inside `content`, only alongside it at the outer level). Checked per matched variant.
  A single-unnamed-field variant's `content` payload is, again, that field type's own business.

- **Internally tagged** (`tag` only, fields inline beside it) — checked per matched variant, since
  the allowed key set depends on which variant the tag names:
  - unit variant: allowed set is exactly `{tag}`.
  - named-field variant: allowed set is `{tag} ∪ that variant's own field names`.
  - single-unnamed-field (newtype) variant: the WHOLE entries object (tag included, now STRIPPED —
    see the bug fix below) is handed to that field type's own `FromValue`; no check added here, that
    payload type decides.

Implementation: a new shared helper `deny_unknown_keys(entries_expr, allowed)` (component.rs, right
above `from_value_struct_fields`) emits the same "for each key, error if absent from `allowed`" loop
`from_value_struct_fields`'s `deny_check` already used for structs — that struct-side code now calls
the same helper instead of duplicating it.

## A real latent bug the new enforcement would have surfaced — found and fixed BEFORE it could bite

While wiring the internally-tagged newtype-variant case, inspection (not test failure — this would
have been silent corruption of the OPPOSITE kind, valid input rejected) turned up a real,
**pre-existing** bug unrelated to enum-level enforcement itself:

`expand_from_value`'s internally-tagged, no-`content`, single-unnamed-field arm handed the payload
type's `FromValue` the **whole** `__entries` object, tag key included:

```rust
Self::#variant_ident(<#payload_ty as FromValue>::from_value(DslValue::Object(__entries.clone()))?)
```

`expand_to_value`'s matching arm never puts the tag INTO the payload's own entries — it takes the
payload's own `to_value()` output and PREPENDS the tag afterward. So a payload struct that itself
carries `#[value(deny_unknown_fields)]` (e.g. `AddWidget`, `RemoveWidget`, … in `FlowMutation`; the
equivalent leaves in `DagMutation`, `WorkflowMutation`, `RunMutation`) would, on decode, see the
wrapper's tag key (`"operation"`) as an unknown field of ITS OWN and reject otherwise-valid input.
This was **never exercised** because every existing test decoded these enums through `serde_json`,
not the first-party `FromValue` path — the bug was invisible until this ticket's own acceptance
criterion (converting a decode test off `serde_json`) would have hit it immediately, on the FIRST
positive round-trip, not just the negative unknown-field case.

**Fix**: strip the tag key before handing the object to the payload type —
`__entries.iter().filter(|(k, _)| k != tag).cloned().collect()` — restoring the encode/decode
symmetry `expand_to_value` already had. This is unconditional (not gated by `deny_unknown_fields`):
correctness, not just the new check, required it — a payload type could technically have owned a
same-named legitimate field and silently read the wrapper's tag value instead of its own.

**Blast radius**: every internally-tagged (`tag`, no `content`), single-unnamed-payload production
enum — `FlowMutation`, `DagMutation`, `WorkflowMutation`, `RunMutation` (all `#[value(tag =
"operation", …)]`), plus the `DependencyTestOp` test fixture. Adjacently-tagged enums
(`Txt/Pdf/PdfA1/PdfX1/ChangeNodeNameMutation`, `PublicationPresence/TransientMutation`) were never
exposed to this — their `content` key isolates the payload from the tag by construction.

## Tests

### Derive crate unit tests (all four representations, plus a negative control)

New `tests/🛡️deny-unknown-fields-enums.rs` (crate name `deny_unknown_fields_enums` — Cargo derives an
ASCII crate name from the file stem so an explicit `[[test]] name = … path = …` pair is required, the
pattern this repo already uses for the `dispatch`/`surface` crates' integration tests). Genuine
integration-test crate (not `#[cfg(test)] mod` inside `src`) because a proc-macro crate cannot invoke
its own derives from inside itself. Added `[dev-dependencies] semio-framework-os-kernel = { path =
… }` to the derive crate's `Cargo.toml` — deliberately cyclic (os-kernel depends on this crate in
`[dependencies]`; this only reaches back in `[dev-dependencies]`, which Cargo resolves fine since
dev-deps never enter the normal build graph) because the derive hardcodes
`::semio_framework_os_kernel::…` paths, so exercising the actual generated code needs the real
runtime crate.

Covers, with one test type per representation plus a `…Lax` sibling WITHOUT the attribute as the
negative control proving the flag is doing the work (not just "nothing ever conflicts"):

- `UnitOnly` — known variant decodes; unrecognized string still errors (attribute is a no-op here).
- `ExternallyTagged` / `ExternallyTaggedLax` — known keys round-trip; unknown NAMED-variant payload
  key → `Err`; the lax sibling accepts the same unknown key.
- `AdjacentlyTagged` — known keys round-trip; unknown OUTER key → `Err`; unknown CONTENT key → `Err`.
- `InternallyTagged` / `InternallyTaggedLax` — known keys round-trip; unknown key on a UNIT variant →
  `Err`; unknown key on a NAMED variant → `Err`; the newtype variant (`Wrapped(InnerPayload)`, where
  `InnerPayload` itself carries `#[value(deny_unknown_fields)]`) decodes correctly with the tag
  present (proving the tag-stripping fix) AND still rejects a genuinely unknown key inside the
  payload (proving the payload's own enforcement still applies); the lax sibling accepts an unknown
  key on its named variant.

Verbatim passing output, the OFFICIAL command against the real workspace (see Verification for how
long and why this was blocked before it cleared):

```
$ cargo test -p semio-framework-value-derive --test deny_unknown_fields_enums
running 14 tests
test externally_tagged_without_attribute_accepts_unknown_payload_key ... ok
test adjacently_tagged_denies_unknown_content_key ... ok
test adjacently_tagged_denies_unknown_outer_key ... ok
test externally_tagged_denies_unknown_payload_key ... ok
test adjacently_tagged_known_keys_round_trip ... ok
test internally_tagged_denies_unknown_key_on_unit_variant ... ok
test internally_tagged_denies_unknown_key_on_named_variant ... ok
test internally_tagged_known_keys_round_trip ... ok
test internally_tagged_newtype_variant_payloads_own_deny_check_still_applies ... ok
test externally_tagged_known_keys_round_trip ... ok
test internally_tagged_newtype_variant_strips_tag_before_reaching_payloads_own_deny_check ... ok
test internally_tagged_without_attribute_accepts_unknown_key ... ok
test unit_only_known_variant_decodes ... ok
test unit_only_unrecognized_string_still_errors_attribute_or_not ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Getting here surfaced one more real bug — this time in the TEST file, not the derive — worth
recording since it is a genuine trap for the next person writing a `tests/*.rs` file against
`semio-framework-os-kernel`: the crate root re-exports BOTH the `ToValue`/`FromValue` TRAITS (from
`os_dsl::schema`) AND the `#[derive(ToValue, FromValue)]` proc-macros THEMSELVES under the identical
two names (`📦️glue.rs` lines 337 and 347, deliberately — the second re-export exists so
`#[mutations(...)]`-style downstream macros can spell `$crate::ToValue` unambiguously). A first draft
of the test file additionally imported `semio_framework_value_derive::{FromValue, ToValue}` directly
(reasonable — that's literally where the derive macros live) and got `error[E0252]: the name
'FromValue' is defined multiple times ... in the macro namespace of this module`: the two imports
resolve to the exact same macro but Rust does not collapse re-imports of an identical item under one
local name. This is invisible in the standalone reproduction rig described below, because that rig's
hand-written stand-in runtime crate defines the TRAITS only, not the re-exported macros — a reminder
that a faithful-looking stand-in can still diverge from the real crate in exactly the way that matters
for a specific test. Fixed by importing everything from `semio_framework_os_kernel` alone (single
`use`, see the test file's own comment at its top).

### `FlowMutation`/`FlowDelta` — the real acceptance criterion

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🦀️.rs`:

- `assert_leaf_contract<T>`'s two unknown-field assertions converted from `serde_json::from_value`
  to `T::from_value(DslValue::from(&unknown))` / `FlowMutation::from_value(DslValue::from(&unknown))`
  — still `Err` on the first-party path. `T`'s bound gained `+ FromValue`.
- `assert_codecs` gained a first-party round-trip (`FlowMutation::from_value(mutation.to_value())`)
  run for all 10 real leaves via `all_ten_codecs_and_descriptors` — this is what actually proves the
  tag-stripping fix: a `from_value` that silently always errored would make the unknown-field
  `is_err()` assertions pass for the WRONG reason, and this catches that.
- `diff_json_contract_matches_native_serde` (FlowDelta/FlowDiff, adjacently tagged `tag = "delta",
  content = "value"`) gained a first-party round-trip for every `valid` vector and a first-party
  `is_err()` assertion for every `invalid` vector — including vectors.json's
  `"unknown-delta-envelope-field"` (`{"deltas":[{"delta":"layout","value":[],"unknown":1}]}`), which
  is exactly the adjacently-tagged OUTER-key check this ticket added.

**Execution status, honestly**: `cargo check -p semio-framework-os-kernel` (no `--tests`, the
ticket's actual guardrail) is clean — 0 errors, confirmed above. Actually RUNNING this file
(`cargo test -p semio-framework-os-kernel`, which needs `--cfg test` to compile the whole lib) hits
21 unrelated pre-existing `E0277` errors in a completely different module,
`🏪️store/🔄️sync/🦀️component.rs` (`BackboneWorkerRequest`/`BackboneWorkerResponse`/`PresencePeer`/
`PathBuf` missing `ToValue`/`FromValue`/`Serialize`/`Deserialize` impls) — this is exactly the
"`🔌️plugin`/`🏪️store` JSON call sites" area this ticket's own hard constraints named as having two
other live agents; not this ticket's to fix, and not touched by anything in this document. So the
flow-file edits above are verified by (a) `cargo check` passing clean, (b) careful manual review, and
(c) the derive's own `deny_unknown_fields_enums` suite exercising the IDENTICAL logic shape
(`InternallyTagged::Wrapped(InnerPayload)`, both internally tagged with a `deny_unknown_fields`
payload struct — structurally the same as `FlowMutation::AddWidget(AddWidget)`) — but NOT by an
actual `cargo test` run of this specific file, which remains blocked by the unrelated store/sync
churn as of this session's end.

### One stdio production mutation — `TxtMutation`

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
(`#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]`,
adjacently tagged): new test `aggregate_denies_unknown_outer_key_via_first_party_from_value` —
decodes a known-good `TxtMutation::SetLine` via `TxtMutation::from_value`, then the same object with
an extra `"extra": true` key at the OUTER level → `Err`. `semio-s-plugin-stdio` is flagged
mid-conversion repo-wide (very active concurrent churn — see Verification); this specific file and
its neighbor `✏️set-line/🦀️.rs` were untouched by that churn (`git status` clean) at edit time, so the
addition is low-risk (purely additive, one new `#[test]` fn plus one `use dsl::FromValue;`).

Also corrected a stale docstring on `ChangeNodeNameMutation`
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/…/✏️🔘️change-node-name/🦀️.rs`) that explicitly documented
this exact gap ("`deny_unknown_fields` here is parsed but NOT enforced … no top-level-extra-key test
exists for THIS enum's own wrapper") — now correctly says it IS enforced.

## Docstring correction

`🦀️component.rs`'s module docstring (`deny_unknown_fields` paragraph) rewritten: previously stated
flatly "on an enum container it is still parsed (no error) but not enforced". Replaced with the full
per-representation breakdown above (this is now the authoritative, load-bearing copy of these
semantics — the summary above just restates it with more rationale).

## Verification

All three of the ticket's required commands pass, run against the real workspace:

```
cargo check -p semio-framework-value-derive --message-format=short
  → clean (2 pre-existing unrelated `unnecessary qualification` warnings, lines 202/290, untouched
    by this change)

cargo test -p semio-framework-value-derive --test deny_unknown_fields_enums
  → 14 passed; 0 failed (verbatim output above)

cargo check -p semio-framework-os-kernel --message-format=short
  → 0 errors

cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm
  → 0 errors
```

These were blocked for most of the session by unrelated concurrent breakage that has SINCE CLEARED
— recorded here because it consumed real time and because the standalone reproduction rig built to
work around it (below) is worth keeping for the next person who hits the same wall. All three
real-workspace commands transit `semio-framework-os-kernel`, which hard-depends (unconditionally, not
feature-gated) on `semio-framework-replication`. For roughly the first half of this session
`semio-framework-replication` failed to build for reasons entirely unrelated to this change — a
concurrent peer edit (`⚠️diagnostic/🦀️component.rs`, `⚠️diagnostic/📍️span/🦀️component.rs`, migrating
`FaultOrigin`/`FaultScope`/`FaultCause`/`TextSpan`/etc. from `serde` onto
`#[derive(semio_framework_value_derive::ToValue, FromValue)]`) went through two distinct failure
modes as the peer iterated, both since resolved:
1. Missing the new dependency wire in `semio-framework-replication`'s own `Cargo.toml` (`error[E0432]:
   unresolved import semio_framework_value_derive`).
2. Then `error[E0433]: cannot find semio_framework_os_kernel in the crate root`, 32 errors, all
   inside `⚠️diagnostic/*` — this derive's generated code always references the concrete
   `::semio_framework_os_kernel::…` path (by design — see the derive's own module docs, "mirrors the
   subset of `#[serde(...)]` actually used under `✏️s/`"), but `⚠️diagnostic` is mounted inside
   `semio-framework-replication`, which `os-kernel` itself depends on — so `replication` cannot ALSO
   depend on `os-kernel` without a genuine (non-dev) cycle. How the peer resolved this is not
   recorded here (not this ticket's file); the point for the record is that neither failure named
   any file this ticket's enum work touched.

**Independent corroboration, built while blocked**: a standalone `[workspace]`-isolated reproduction
rig at `/private/tmp/…/scratchpad/verify-value-derive/{derive,runtime,tests}` (an earlier session in
this ticket's own scaffolding, `runtime` being a byte-verified-faithful copy of
`semio-framework-os-kernel`'s `DslValue`/`ToValue`/`FromValue`/`ValueError` surface, `derive`
refreshed to a literal `cp` of this session's final `component.rs`) ran the same test file to
14/14 passing well before the real blocker cleared — genuine evidence, independent of the blocker,
though it also caught nothing the real run didn't (the E0252 import collision above was found by the
REAL run, not the rig — the rig's hand-written stand-in doesn't re-export the derive macros the way
the real crate root does, so it couldn't have caught that). Left in place in the scratchpad; not
part of the repo, no cleanup needed there.

**A second production-enum decode test, beyond the required two**: while diagnosing, also confirmed
`DagMutation`, `WorkflowMutation`, and the `DependencyTestOp` test fixture share `FlowMutation`'s
exact shape (`#[value(tag = "operation", …)]`, no `content`, newtype variants wrapping
`#[value(deny_unknown_fields)]` structs) and would have hit the same tag-leak bug — not additionally
tested individually (the derive-level fix and its dedicated `internally_tagged_newtype_variant_*`
tests already cover the general case; `FlowMutation`'s own production test is the deeper proof), but
worth naming here as confirmed blast radius.
