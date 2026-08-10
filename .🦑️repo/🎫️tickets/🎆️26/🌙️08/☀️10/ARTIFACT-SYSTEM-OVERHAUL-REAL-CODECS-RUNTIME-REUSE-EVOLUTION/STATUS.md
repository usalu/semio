# Status

Plan: `/Users/ueli/.claude/plans/the-current-import-export-snappy-thompson.md` (D1–D6, waves V0–V8). This is a multi-session effort; this file tracks real, verified state only — no aspirational claims.

## External blocker (affects all Rust verification this session)

A concurrent session is mid-refactor on `RENAME-DOCUMENT-TO-ARTIFACT-THROUGHOUT-CODEBASE` (renaming `AppDefinition.document`/`ExampleDefinition.document_json` → `artifact_json` across the whole tree). While in flight it leaves `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` non-compiling (3 call sites not yet repointed: lines ~1101, ~3018, ~3034), which transitively blocks `cargo check -p semio-s-plugin-stdio` (and anything depending on `semio-framework-plugin`). Confirmed external (not caused by this session) via `git diff --stat` on that file each retry — this session's own edit there stayed a stable 3-line diff throughout. Retried periodically over this session; still blocked at time of writing. **Do not fix that file — it's someone else's in-progress work.** Also blocks writing the 4 fixture example-definition leaves (V0 remaining item), since they'd need to construct `ExampleSource{document_json, ...}` against a field name actively being renamed out from under it.

Workaround used this session: extracted the two pieces of new/changed codec logic into standalone throwaway Cargo crates under the scratchpad (no workspace deps) and ran them directly with `cargo run` to get real compile+runtime verification despite the blocker. This is how the xml bugs below were actually caught — hand-tracing alone had missed them.

## V0 progress this session

**D1 shared contract** — done, compiled clean in isolation before the blocker appeared (`cargo check -p semio-framework` passed with only pre-existing warnings):
- `ArtifactDialect` (owned dialect twin, `to_coordinate`/`parse_coordinate`), `io_dispatch` (local-resolve-then-fallback-hook dispatch seam), `io_keys_for`, `list_composer_entries`, `set_io_fallback_dispatcher` in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`.
- Re-exported through `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`.

**Catalog SSOT move** — done and verified:
- Copied (plain fs copy, not `git mv`, per the "shared live tree" rule) `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/STDIO-ARTIFACTS-AND-IO/🧪owner-table.json` → `🧰️framework/🔨️modules/🚪️io/📇️registry/📇️catalog.json`.
- `POLICY_STDIO_OWNER_TABLE_REL` in root `📜️script.ts` repointed to the new path with a one-wave dual-read fallback (`POLICY_STDIO_OWNER_TABLE_LEGACY_REL`) so nothing breaks before every reader is repointed.
- Verified via `bun run ./📜️script.ts verify`: zero `stdio-catalog`/`owner-table` breaches (loader picks up the new location cleanly). The generator `.py` scripts under the ticket folder still reference the old path — left as-is, they're historical one-shot scripts already run, not re-run going forward.

**Deflate codec (LZ77, `🗜️deflate/🏅️standards/🔖️rfc1950/⚙️engine/🦀️component.rs`)** — verified correct via standalone harness (`scratchpad/deflatecheck`), 20/20 checks incl. window-boundary edge cases (len 0/1/2/3/257/258/259/32768/32769/37768) that weren't in the original test file. **No changes needed** — the hand-written LZ77 hash-chain + lazy matching + Huffman emission is genuinely correct.

**XML codec (`📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`)** — real bugs found and fixed via the same standalone-harness technique (`scratchpad/xmlcheck`):
1. `xml_unescape_text` (entity decode) was written but never called from `parse_node`'s text-capture path — text nodes still held raw escaped sequences, so decode→encode→decode would have grown `&amp;` into `&amp;amp;` exactly as the plan's "cumulative corruption" bug describes. Fixed: `parse_node`'s trailing text branch now calls `xml_unescape_text(raw)?`.
2. `parse_node`'s children loop never actually recognized `<![CDATA[`, `<!--`, or `<?...?>` — it only had `skip_misc` (discard-only) at the top of the loop, so a CDATA section anywhere in content caused a hard **panic** (tried to `parse_name` on `![CDATA[...`, got "expected XML name"). Fixed: the loop now dispatches CDATA/Comment/ProcessingInstruction to their own branches before falling through to element/text, and top-level `skip_misc` calls were removed from `parse_node` (redundant now that the loop handles them explicitly) — this also fixes the whitespace-eating bug (whitespace-only text between siblings is now preserved).
3. Numeric character reference parsing had the hex/decimal branch order backwards (checked for an `x`/`X` prefix on the *whole* entity instead of on the substring after `#`, so `&#x42;` mis-parsed as a decimal literal `"x42"` and errored). Fixed: strip `#` first, then check for `x`/`X`.
4. Also landed: `<!DOCTYPE ...>` (with bracket-depth-tracked internal subset) no longer hard-fails parsing and round-trips via `XmlDocument.doctype`; `Comment`/`ProcessingInstruction` are real preserved node types (previously silently discarded by `skip_misc`).

15/15 checks pass in the standalone harness (entity decode, no-cumulative-corruption across 3 decode→encode cycles, CDATA verbatim preservation, comment/PI-as-node, DOCTYPE-with-internal-subset, whitespace preservation, unknown-entity-is-hard-error). **Not yet re-verified against the real workspace crate** (still blocked — see above); the ported-back file is byte-for-byte the same logic that passed the harness.

## V0 remaining (not started / blocked)

- 4 fixture example-definition leaves + glue.rs test-mount region (blocked on the `ExampleDefinition.document_json`→`artifact_json` rename settling — see blocker above).
- Policy rules 1/2/4 (`policyFacetTraitImplBreaches`, `policyDialectLiteralPathBreaches`, `policySniffRealityBreaches`) with allowlists.
- No-0-byte-asset generator.
- Re-verify xml + deflate compile inside the real `semio-s-plugin-stdio` crate once the external refactor clears (`cargo test -p semio-s-plugin-stdio --lib "artifacts::xml"` and `"artifacts::deflate"`).

## Next session should

1. Retry `cargo check -p semio-s-plugin-stdio` first — if clean, run the two test modules above before touching anything else.
2. If still blocked, keep pivoting to independent V0/V1 work and re-verify via the standalone-scratch-crate technique (fast, catches real bugs, doesn't need the blocked workspace graph) rather than hand-tracing.
3. Once the rename settles, write the 4 example-definition leaves against whatever the final `ExampleDefinition` shape turns out to be (currently `artifact_json`).
