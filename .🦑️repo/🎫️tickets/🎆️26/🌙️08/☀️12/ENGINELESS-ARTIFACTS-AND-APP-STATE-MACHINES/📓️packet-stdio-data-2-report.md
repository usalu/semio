# 📓️ Packet: stdio data-2 (🎒️zip · 📑️tsv · 💾️binary · 🗜️deflate) — Engine Dissolution

Ticket: 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES
Scope: stdio's container/data formats — `zip`, `tsv`, `binary`, `deflate`.

## Starting state

`📑️tsv` was ~90% already dissolved by a prior session/wave: `subsets::any::io`
already carried `register()`/`io_registry`, `subsets::any::schema::snapshot`
already carried `decode_tsv`/`encode_tsv`/`sniff_real_bytes` + all 6 tests, and
`📑️tsv/🦀️component.rs` already called `io::register()` (not `engine::register()`).
Only the physical `⚙️engine/` directory + its `glue.rs` mount were still live
(fully superseded/dead content). `🎒️zip`, `💾️binary`, `🗜️deflate` had NOT been
touched — full engines, live callers, `.composers(engine::io_registry::entries())`
wiring, etc.

## Destination per region per artifact

Precedent used: `📷️png` (already-dissolved sibling with the same shape as zip —
binary+deflate deps, real byte-level codec, cross-artifact CRC32/zlib reuse) for
"codec → io/, algorithms stay with codec, DocumentHelpers → schema/ root,
conformance-laws/field-sweeps → schema/inferences/". `📊️csv`/`🔣️json` looked like a
different pattern (codec in `schema/snapshot/`) but turned out to be a red
herring — they never had an engine-owned byte-level codec to begin with; `📑️tsv`'s
own dissolution (predecessor's work, confirmed in its own doc comment) genuinely
does put the native text codec in `schema/snapshot/` because it IS the
`ArtifactDsl`/`ArtifactPack` impl directly, with no cross-artifact bridging.

| Region | 🎒️zip | 💾️binary | 🗜️deflate |
|---|---|---|---|
| `*Engine` struct | **deleted outright** (zero construction sites repo-wide, confirmed via `grep -rn "ZipEngine::new\|ZipEngine {"`) | same, `BinaryEngine` deleted | same, `DeflateEngine` deleted |
| Codec (decode/encode/sniff) | → `🚪️io/🦀️component.rs`: CRC32, `ZipError`, byte readers, CP437, extra-fields, EOCD, `decode_zip`/`encode_zip`/`sniff_zip_bytes`/`SniffConfidence` | already lived in `📸️snapshot/🦀️component.rs` (hex `ArtifactDsl`/`ArtifactPack`) — untouched | → `🚪️io/🦀️component.rs`: Adler32, BitIO, Huffman, LZ77 (~1,080 LOC), `zlib_compress`/`zlib_decompress`, `encode_deflate_snapshot`/`decode_deflate_snapshot` |
| `io_registry` | → `🚪️io/🦀️component.rs` (`ZipRawAnyComposer` + `ZipIso21320Composer`, both subsets) | → `🚪️io/🦀️component.rs` | → `🚪️io/🦀️component.rs` |
| Pure format algorithm w/ no snapshot dep | CRC32 kept with codec in `io/` (rule 6; reused by `📷️png`'s `png_crc32`) | n/a | Adler32/BitIO/Huffman/LZ77 kept with codec in `io/` (rule 6 — ticket's own "clearest case") |
| Conformance laws / field sweeps | → `🧬️schema/💡️inferences/🦀️component.rs` test mod (6 tests) | → same (5 dissolved-engine tests + 6 conformance laws = 11) | → same (7 conformance laws, incl. `schema_spec_registration_resolves`) |
| DocumentHelpers (`empty_*`/`demo_*`) | → `🧬️schema/🦀️component.rs` root | → same | → same |
| `register()`/`register_pilot_languages`/`register_artifact_schema`/`register_artifact_inferences` | **dead, not recreated** — zip already used `declaration()` builder; these were fully superseded | → `🚪️io/🦀️component.rs` (zip/binary/deflate `register()` aggregate lives beside `io_registry`, matching `📷️png`/`📑️tsv`) — binary is one of the 10 protected imperative calls so this one *is* live | **dead** except `register_schema_specs()`, which survives → `🚪️io/🦀️component.rs` (referenced by a `.setup()` gap-filler, not the protected-10 pattern) |
| Tests (codec-level) | → `🚪️io/🦀️component.rs` `codec_tests` mod (10) | n/a (no codec split — codec tests already live in `📸️snapshot`, untouched) | → `🚪️io/🦀️component.rs` `codec_tests` mod (9) |

## Consumer call sites (public surface)

`zip::engine::{decode_zip, encode_zip, crc32, sniff_zip_bytes, SniffConfidence}`
and `deflate::engine::{zlib_compress, zlib_decompress, deflate_raw, inflate_raw,
encode_deflate_snapshot, decode_deflate_snapshot}` had real external callers.
Every one repointed to the fully-qualified new path (never a bare
`io_registry::entries()`, never a leftover shim beyond the one protected case):

- `🎒️zip/📦️opc/🦀️component.rs` (6 refs — shared OPC layer docx/xlsx/pptx import)
- `🎒️zip/🏅️.../🧬️schema/{📸️snapshot,🧬️mutations}/🦀️component.rs` (own facets)
- `🎒️zip/🏅️.../🚪️io/{📥️import,📤️export}/.../{💾️binary,🗜️deflate}/🦀️component.rs` (4 refs)
- `💬️bcf/🏅️.../🧬️schema/🦀️component.rs` + `🚪️io/🦀️component.rs` (zip sniff + zip-wrap encode — bcf's OWN io/component.rs turned out to have 4 more refs than the schema-only ones caught on first grep)
- `📜️docx/🏅️.../🧬️schema/🦀️component.rs`, `📕️xlsx/🏅️.../🧬️schema/🦀️component.rs` (decode_zip)
- `🎞️pptx/🏅️.../{⚙️engine → later 🧬️schema}/🦀️component.rs` (decode_zip — pptx's own engine was dissolved by a **concurrent session** mid-task; re-grepped and fixed at its new location)
- `📷️png/🏅️.../🚪️io/🦀️component.rs` (`zip::engine::crc32` → `zip::…::io::crc32`; 8× `deflate::engine::zlib_{compress,decompress}` → `deflate::…::io::…`)
- `📄️pdf/🏅️1.4/…/🚪️io/🦀️component.rs`, `📄️pdf/🏅️1.7/…/{🚪️io,⚙️engine}/🦀️component.rs` (deflate zlib refs, pdf 1.7's own engine still transitionally present from another concurrent session — reference lines fixed regardless)
- `🎒️zip/🏅️.../✳️iso21320/🚪️io/🦀️component.rs` doc comment (prose only, updated for accuracy)

Zero remaining `artifacts::{zip,binary,deflate,tsv}::engine::` or
`standards::{v2_0,v_raw,v_rfc1950,iana::…::any}::engine` references repo-wide
except: (a) the one protected `binary::engine::register()` plugin-root call +
its `glue.rs` shim, (b) harmless historical doc-comment prose in `zip`/`deflate`
root `component.rs` files.

## The 10 protected imperative-`register()` artifacts

`💾️binary` is one of them (`crate::artifacts::binary::engine::register();` in
`🗄️stdio/🦀️component.rs` line 8, **left textually unchanged**). Since its physical
`⚙️engine/` is gone, `glue.rs`'s `v_raw::engine` mount was replaced with a single
minimal re-export (mirrors `📄txt`/`🌐️html`'s own established precedent for this
exact situation):
```rust
pub mod engine {
    pub use super::subsets::any::io::register;
}
```
`🎒️zip` and `🗜️deflate` are **not** among the 10 (both already used the
declarative `ArtifactDeclaration` builder) — their `engine` shims were removed
entirely (no compat layer left; CLAUDE.md forbids it), every call site
repointed directly. `deflate`'s `register_schema_specs()` `.setup()` gap-filler
(also not one of the 10 — a narrow single-fn survivor) was repointed directly
in `🗄️stdio/🦀️component.rs`.

## Bare `io_registry` shadow count

Repo-wide bare `io_registry::entries()` grep: **1 hit**, and it is safe —
`💾️binary/🚪️io/🦀️component.rs`'s own `register()` calling its own
same-file-local `io_registry::entries()` (unambiguous, only one `io_registry`
in scope at that point). Every artifact-root `.composers(...)` call and every
cross-file reference uses the full `standards::…::subsets::any::io::io_registry::entries()`
path. Bare-and-dangerous count: **0**.

## Assertion arithmetic (tests preserved)

| Artifact | Before (engine) | After | Where |
|---|---|---|---|
| 🎒️zip | 16 (10 codec + 6 conformance) | 18 (10 `io/` `codec_tests` + 6 conformance + 2 pre-existing inference laws) | `🚪️io/` + `🧬️schema/💡️inferences/` |
| 💾️binary | 11 (5 dissolved-engine + 6 conformance) | 13 (11 moved + 2 pre-existing inference laws) | `🧬️schema/💡️inferences/` |
| 🗜️deflate | 16 (9 codec + 7 conformance, incl. `schema_spec_registration_resolves`) | 18 (9 `io/` `codec_tests` + 7 conformance + 2 pre-existing inference laws) | `🚪️io/` + `🧬️schema/💡️inferences/` |
| 📑️tsv | 6 (already relocated pre-session) | 6 (unchanged, verified still present) | `🧬️schema/📸️snapshot/` |

`#[test]` grep counts on the new files confirm every number above exactly — no
test dropped, no test duplicated.

## `//!` mid-file doc-comment trap

Hit it exactly as warned: both new `zip` and `deflate` `🚪️io/🦀️component.rs`
region headers were first written as `//!` (E0753 ×18 on first `cargo check`).
Both auto-corrected to `//` (by the environment's format-on-save) before the
second check; verified by re-reading both files and by the clean second
`cargo check` run.

## Compiler output

`RUSTC_WRAPPER="" CARGO_TARGET_DIR=target/stdio_dat2 cargo check -p semio-s-plugin-stdio --all-targets`

First run (after initial content moves): 36 lib + 42 test errors. All traced to:
18× E0753 (the `//!` bug above, self-inflicted), 1× "expected item after doc
comment" (pre-existing `🏗️ifc` issue, unrelated), a batch of `crate::artifacts::
{deflate,zip,binary}::…` wrong-path imports (self-inflicted — see below), and
pre-existing `🏗️ifc`/🎞️gif/🎨️svg/📄️pdf errors from **other concurrent sessions'
in-flight dissolutions** (confirmed via `git status` showing their own
`⚙️engine/` dirs mid-deletion at the time of the check).

Self-inflicted import-path bugs fixed:
- `zip/🚪️io/🦀️component.rs`: `schema::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA}` doesn't
  re-export those (they live at the artifact root) → fixed to
  `crate::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA}`.
- `deflate/🚪️io/🦀️component.rs`: same shape, `STDIO_DEFLATE_DOCUMENT_SCHEMA` lives at
  the artifact root, not `schema::snapshot`.
- `binary/🚪️io/🦀️component.rs` + `💡️inferences/🦀️component.rs`: same,
  `STDIO_BINARY_DOCUMENT_SCHEMA` lives at the artifact root.

Final run: **`error: could not compile … due to 2 previous errors` (lib) / `6
previous errors` (lib test)** — all 6 unique errors are `🎨️svg`
(`STDIO_SVG_DOCUMENT_SCHEMA` not in scope), `🎞️gif` (`demo_gif_snapshot` not
found in `engine` — its own dissolution mid-flight), and `📄️pdf` 1.4
(`PdfMutation` not in scope, 4×) — **zero errors anywhere in `zip`/`tsv`/
`binary`/`deflate`**, confirmed by grepping the error output for those four
artifact paths (no hits). `git status` on `🎞️gif`/`🎨️svg`/`📄️pdf` shows their
`⚙️engine/` dirs mid-deletion by other sessions at check time — pre-existing/
concurrent, not mine, not fixed (out of scope, actively being worked by
someone else per CLAUDE.md's no-worktree/shared-tree collaboration model).

Full output saved: `scratch-data2-zip-tsv-binary-deflate-cargo-check-final.txt`
(this ticket folder).

## Verification

```
find ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🎒️zip,📑️tsv,💾️binary,🗜️deflate} -type d -name "⚙️engine"
```
→ **0 results** (all four deleted).

Dangling `#[path=...]` mount script: **0 dangling**.

## Deviations from the literal ticket text

1. Rule 2 ("codecs → io/") does **not** apply to `binary` — its hex codec was
   already living in `📸️snapshot/` pre-dissolution with no engine-owned
   byte-level parsing to move (a genuinely different shape from zip/deflate/png:
   no chunking, no framing, just a straight hex/base64 envelope already owned
   by the snapshot type). Confirmed correct by precedent (`txt` has the same
   shape and was never engine-owned either).
2. Deflate's Huffman/LZ77 went to "keep with the codec in `io/`" (first half of
   rule 6), not "a module engine one level up" — the ticket itself calls this
   the clearest case for the former.
3. Zip's own root `io_registry` doc comment / `register_subset_validators`
   helper I drafted turned out to be genuinely dead (subset validators are
   re-derived independently by `declaration()`, not called through
   `io_registry`) — removed before it shipped, not left as unreachable code.

## Files touched (created/updated/removed)

**Removed (directories):**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/⚙️engine/`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/⚙️engine/`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/⚙️engine/`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/⚙️engine/`

**Updated (content moves + reference repoints):**
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (4 physical mounts removed, 3 outer shims removed/repointed, 1 inline single-symbol shim added for binary)
- `🎒️zip`, `💾️binary`, `🗜️deflate` each: `🚪️io/🦀️component.rs`, `🧬️schema/🦀️component.rs`, `🧬️schema/💡️inferences/🦀️component.rs`, artifact-root `🦀️component.rs`
- `🎒️zip`: `🧬️schema/📸️snapshot/🦀️component.rs`, `🧬️schema/🧬️mutations/🦀️component.rs`, `📦️opc/🦀️component.rs`, `🏅️2.0/🪆️✳️any/🚪️io/{📥️import,📤️export}/.../🦀️component.rs` ×4, `🏅️2.0/🪆️✳️iso21320/🚪️io/🦀️component.rs` (doc only)
- `🗜️deflate`: `🧬️schema/📸️snapshot/🦀️component.rs`, `🧬️schema/🔺️diff/🦀️component.rs`, `🏅️rfc1950/🪆️✳️any/🚪️io/{📥️import,📤️export}/.../🦀️component.rs` ×2
- `💬️bcf/🏅️2.1/🪆️✳️any/{🧬️schema,🚪️io}/🦀️component.rs`
- `📷️png/🏅️1.2/🪆️✳️any/🚪️io/🦀️component.rs`
- `📜️docx/🏅️ecma-376/🪆️✳️any/🧬️schema/🦀️component.rs`
- `📕️xlsx/🏅️ecma-376/🪆️✳️any/🧬️schema/🦀️component.rs`
- `🎞️pptx/🏅️ecma-376/🪆️✳️any/🧬️schema/🦀️component.rs`
- `📄️pdf/🏅️1.4/🪆️✳️any/🚪️io/🦀️component.rs`, `📄️pdf/🏅️1.7/🪆️✳️any/{🚪️io,⚙️engine}/🦀️component.rs`
- `🗄️stdio/🦀️component.rs` (deflate's `register_schema_specs` setup call repointed)
