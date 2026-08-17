# Gate Stdio S2 Report

## Scope

Shard S2 of the serialized stdio compiler gate. Exclusive kinds (under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`):
`📄️pdf` `🔣️json` `📄txt` `🎞️pptx` `📊️csv` `📕️xlsx` `📜️docx` `🌐️html` `🗜️deflate` `🎒️zip` `🌦️epw` `📰xml` `📑️tsv` `💾️binary` `📝️md`.

Contract: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/📋️mutation-diff-result-stdio-residual.md` — source frozen for the residual `MutationDiff::apply` → `MutationApplyResult<Snapshot>` migration, compiler gate deferred to the owning artifact shard (me).

## Error counts (anchored to my kinds only, via `grep "🗿️artifacts/<kind>/🏅️standards"` on `--message-format=short` output)

- **Start**: 119 errors (112×`E0308`, 6×`E0609`, 1×`E0277`) — all call sites still treating `MutationDiff::apply`'s `MutationApplyResult<Snapshot>` as a bare `Snapshot`.
- **After Result-typing fixes**: a second `cargo check` pass surfaced 93 *additional*, unrelated pre-existing latent errors baked into the frozen residual source (see below) — these were never part of the 119 but blocked full compilation of my kinds regardless.
- **End**: **0 errors** in my kinds. Crate-wide total is 203 (non-zero, expected — sibling shards S1/S3 own the rest and are still working; verified with both a strict `.../<kind>/🏅️standards` path filter and a loose `.../<kind>/` filter — the one loose-filter hit left is a false positive, a `🧿️semio` PPTX-deserializer file whose path happens to contain `🗿️artifacts/🎞️pptx/` as a nested segment, not owned by my kind).

## What was fixed

### 1. Typed-rejection call-site fallout (the ticket's actual assignment)
Every production `apply_<kind>_mutation` function in my kinds (`pdf` 1.4/1.7, `json`, `txt`, `pptx`, `csv`, `tsv`, `html`, `zip`, `epw`) was **already correctly migrated** by the peer ticket: `match MutationDiff::apply(...) { Ok(next) => ..., Err(error) => MutationOutcome::error(...) }`. The 119 baseline errors were exclusively in **test code** that called `.apply(&base)` and used the result as a bare snapshot. Fixed by adding `.unwrap()` at each test call site (never in production code) — per the ticket's "tests may unwrap a known-valid result" rule. Representative files:
- `📄️pdf/🏅️standards/🔖️1.4/…/🔺️diff`, `…/🧬️mutations`, `…/📚️examples/🎓️bachelor-thesis/🧪️tests/🦀️test.rs`
- `📄️pdf/🏅️standards/🔖️1.7/…/🔺️diff`, `…/🧬️mutations`
- `🔣️json/…/🔺️diff`, `…/🧬️mutations`
- `📄txt/…/🔺️diff`, `…/🧬️mutations`, `…/🧬️schema/🦀️component.rs`
- `🎞️pptx/…/🧬️schema/🦀️component.rs`, `…/🔺️diff`
- `📊️csv/…/🧬️mutations`
- `📑️tsv/…/🧬️mutations`
- `🌐️html/…/🧬️mutations`
- `🎒️zip/…/🚪️io`, `…/🧬️mutations`
- `🌦️epw/…/🧬️mutations`

### 2. Latent pre-existing bugs uncovered once (1) compiled cleanly
Fixing the Result-typing exposed ~93 further errors already present in the frozen residual source, unrelated to the `MutationApplyResult` migration itself — a mechanical `#[cfg(test)]` had been misattached to production-required imports (so a plain `cargo check --lib` build, or the non-test half of `--all-targets`, couldn't see the trait), plus a couple of missing-import/missing-use bugs in test modules. Fixed, never touching validation/atomicity/preflight logic:
- Removed a wrongly-scoped `#[cfg(test)]` in front of `use protocol::{OpBinary, OpText}` / `use protocol::DiffCodec` in: `📄️pdf` 1.7 mutations, `📄txt` mutations, `🗜️deflate` mutations, `💾️binary` mutations, `📊️csv` diff, `📑️tsv` diff, `🌦️epw` diff (html/epw-mutations/tsv-mutations had already been auto-fixed upstream by the time I checked).
- Restored a dropped `PdfStreamFilter` import in `📄️pdf` 1.7 mutations (used by a test fixture, not exported).
- Added missing local imports (`PdfDiff`, `PdfMutation`, `PdfPathSegment`, `DiffAlgebra`) inside two test functions in `📄️pdf` 1.7 `🚪️io/🦀️component.rs` that relied on `use super::*` but needed additional explicit imports.
- Added missing `STDIO_TXT_DOCUMENT_SCHEMA`/`TxtDiff`/`TxtMutation` imports to `📄txt`'s outer `🧬️schema/🦀️component.rs` test module.
- Fully-qualified an `XmlDocument::default()` reference in `🎞️pptx`'s diff test module (type lives in the `📰xml` artifact, wasn't imported).

## Verification
Final `cargo check -p semio-s-plugin-stdio --all-targets --keep-going` output saved to `🧪️gate-stdio-s2.txt` in this ticket folder. Filtered for my 15 kinds: 0 errors, both with a strict `<kind>/🏅️standards` path anchor and a loose `<kind>/` anchor (the sole loose-filter hit is a same-named-segment false positive belonging to `🧿️semio`, not mine).

## Files touched (all within my exclusive kinds)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🧪️tests/🦀️test.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

`📕️xlsx`/`📜️docx`/`📰xml`/`📝️md` were already clean at baseline (0 errors) and needed no changes.

## Not touched (out of scope, reported per instructions)
No shared stdio code (`📦️glue.rs`, plugin root, shared helpers) needed changes — every fix landed inside my exclusive kind directories.
