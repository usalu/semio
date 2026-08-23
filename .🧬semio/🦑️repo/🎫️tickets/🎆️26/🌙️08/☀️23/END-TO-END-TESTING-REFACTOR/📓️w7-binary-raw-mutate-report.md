# Wave 7 — 💾️binary standard 🔖️raw subset ✳️any — mutation oracle + exhaustive round-trip case

Executor report for the fleet brief's binary/raw subset assignment. Files touched are listed at the
bottom; nothing outside `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/…` and this ticket folder was
edited.

## 1. The honest situation, and the decision recorded

A raw byte buffer has no format. There is no grammar for a third-party crate to parse, and — unlike
every other subset this wave, which at minimum has an independent READER even without a writer — no
independent reader either, because there is nothing to read structurally. No crate was surveyed or
registered. This is recorded as `noOracleDecision` `raw-buffer-no-format` in
`🏅️standards/🔖️raw/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, substitutes
`specification-vectors` + `metamorphic-laws`, and the feature carries `@no-oracle-raw-buffer-no-format`
instead of an `@oracle-` tag. `BinaryMutation` has 5 variants (confirmed by reading
`🧬️schema/🧬️mutations/🦀️component.rs`): `NoMutation`, `SetSnapshot`, `Splice`, `AppendBytes`,
`TruncateAt`.

## 2. What "oracle" means when there is no third party

`🏅️standards/🔖️raw/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s `oracle_apply_mutation` is this
subset's own independently written implementation of the specification — a from-scratch
splice/append/truncate over `Vec<u8>`, with its own bounds validation (rejecting an out-of-range
`offset`/`remove_len`, defining `TruncateAt` past the length as the documented no-op), that never
calls into the subject's own `BinaryDiff`/`apply_binary_mutation`. 10 `#[test]` unit tests (plain
`#[test]`, not `async_test` — the whole-crate-breaking mistake the brief warned about) cover every
kind plus both rejection paths; verified with
`cargo test --features oracles --lib` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`:
**10/10 pass**, and the full crate suite is 124/125 (the one pre-existing failure is the JSON subset,
unrelated to this work, matches the brief's "other subsets currently have failures" note).

## 3. A framework finding: the `oracle` role never executes for a `@no-oracle-` feature

Read `oracleDecision()` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts`: when a
feature's `oracle` tag is `null`, `decision.implementation` stays `null` regardless of whether a
`noOracleDecision` is recorded — the `oracle` role is a THIRD-PARTY REFERENCE concept, and
`runPhases` only ever invokes it when `decision.implementation !== null`. Confirmed empirically
against an established precedent, not just by reading:

```
bun ./📜️script.ts oracle quick --owner 🧰️framework/🔨️modules/🎠️kernel --case reject-malformed-version-input
[test] level=quick cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0

bun ./📜️script.ts subject quick --owner 🧰️framework/🔨️modules/🎠️kernel --case reject-malformed-version-input
[test] level=quick cases=1 executed=1 passed=1 failed=0 errored=0 parity=0/0
```

So for any `@no-oracle-` feature, `subject` is the only role the runner ever actually executes, and
that's where the two existing no-oracle precedents (`reject-malformed-version-input`,
`merge-conflicting-utilities`) put their whole specification check, self-contained, registering NO
`oracle` role at all.

This case still registers the full oracle+subject shape the brief's §4 template specifies (a real,
independently written, unit-tested reference exists and is genuinely useful evidence), but — since
`oracle`'s role literally never runs for this feature — the SUBJECT handler
(`🧪️tests/mutate-binary-raw/🦀️component.rs`'s `subject::apply_and_encode`) additionally cross-checks
its own `apply_binary_mutation` result against `oracle_apply_mutation` internally and returns `Err`
on any disagreement, so the specification-vector/metamorphic-law evidence is actually discharged by
the one role the framework runs, not left stranded in a registration nothing invokes.

## 4. The test case

`🗿️artifacts/💾️binary/🧪️tests/mutate-binary-raw/component.feature`: 20 scenarios total.
- `mutate-<kind>` / `inverse-<kind>` × 5 kinds (`@mode-conformance` / `@mode-property`, never
  `@mode-differential`).
- `identity-round-trip` (`@mode-round-trip`) — deliberately weak and says so: for this one subset
  decode/encode really is the identity (`carrier_native_is_raw`), so the no-byte-pass-through
  tripwire every other wave-7 subset enforces cannot apply here. Byte equality is the correct answer.
- 6 specification-vector scenarios (`vector-*`): zero-length splice, splice at offset 0, splice at
  exactly the end, splice spanning the whole buffer, truncate to 0, truncate beyond the length (the
  vocabulary's own defined no-op, not an error).
- `append-to-empty-buffer`.
- 2 `@mode-error` scenarios (`invalid-splice-*`): offset beyond the buffer, `removeLen` past the
  end — both must be rejected cleanly, never silently corrupt the buffer.

Input: the real 483,496-byte JFIF/XMP floor-plan scan already committed at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.jpg`,
copied once into `🗿️artifacts/💾️binary/🧫️fixtures/` and referenced as `shared://`. Real bytes matter
here specifically because this subset does not parse structure: the `mutate-splice` scenario's
offset 6 lands inside the file's genuine `JFIF\0` identifier, not synthetic padding.

## 5. Verification (verbatim)

From `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-binary-raw
0 high-priority breach(es) across 0 rule(s):

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-binary-raw
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0
(exit 0)
```

No breach names binary. `executed=0` on the oracle phase is the CORRECT result for a `@no-oracle-`
feature (§3 above), not a failure to execute — confirmed against the kernel precedent's identical
`executed=0` on its own oracle phase.

**The Rust SUBJECT phase cannot compile this wave** — confirmed independently, not just taken on
faith: `cargo check --lib` in the stdio production crate fails on an unrelated concurrent workspace
break (`✒️writer` plugin, missing `workspace.dependencies.js-sys`), on top of the brief's own noted
os-kernel `semio_framework::` cycle. Not this ticket's bug. The subject half is written, sut-gated,
and (per §3) is the half that will actually discharge this decision's evidence once the crate builds
again.

## 6. Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS` + `kinds_cover_every_variant` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`
  — filled in `oracle_apply_mutation` + 10 unit tests (was a stub rejecting every kind).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`
  — new: catalog + `noOracleDecision`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.jpg`
  — new: real 483,496-byte fixture, copied from the jpg artifact's own committed fixture.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🧪️tests/mutate-binary-raw/component.feature` — new.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🧪️tests/mutate-binary-raw/🦀️component.rs` — new.
