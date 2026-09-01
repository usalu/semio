# Rejected Page Close — New Schema-First Preparation 66

## Release Status

Preparation only. Three new canonical test assets and one ticket-only controller are source-ready for review. The Store module is **not mounted or edited**. No Rust compiler or native test ran; native RED remains pending the root's explicit mount/compile window.

This is newly authored work. No missing retirement49 source, run directory, or report was recreated. Registry poison/quarantine, FreshField/FreshVcs retention, R17 codecs/backbone/retirement, command, Interaction, and return-path implementations were not changed.

## Complete Review Inputs

- [Strict canonical schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🧬️schema/🔣️.json)
- [Canonical neutral vectors](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🔣️.json)
- [Both unmounted native bodies and their actual-owner helpers](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🦀️.rs)
- [Ticket-only reference/source controller](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-rejected-page-close-66/📜️script.ts)

The canonical data contains **three layouts and 23 authored close observations**. Each of the two native laws consumes the same complete roster, so a future native run will execute 46 page-close observations in addition to field/lease setup and teardown checks. No expectations are generated from the production wrappers.

| Neutral case | Page lengths | Close observations |
| --- | --- | --- |
| full-page-zero-short-exact | 4096 | 8 |
| short-tail-keeps-page-granule | 4096,14 | 9 |
| three-full-pages-at-most-one | 4096,4096,4096 | 6 |

Normalization is explicit: each page is left-padded with ASCII spaces to its authored UTF-8 byte length, then the exact suffix is appended. Every nonterminal page is actually full4096. The short tail `{"page":"尾"}` is 14 UTF-8 bytes. Both reference and native fixture code independently check the normalized JSON document. These records are deliberately rejected **before parsing**; valid generic JSON is not claimed to be a fully decoded application envelope.

The cases cover zero items, positive items with zero/4095 bytes, a short-tail-length-only14-byte grant, exact4096 grants, large multi-page grants that still release at most one page, actual tail bytes rather than rounded accounting, and repeated terminal calls. A zero-item call after terminal preserves the existing Pending0/0 result while the owner remains terminal-empty; a positive-item call after terminal remains Complete, including zero bytes. No new terminal policy is imposed.

## Actually Executed Reference Results

Schema/vectors/controller were authored first. The first reference run completed before the Rust file was authored.

1. **Reference mode: Nx exit0, 57/57 checks**, new [run-KHtCPn receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-rejected-page-close-66/🧫️run-KHtCPn/📓️receipt.md), plus a complete [sibling receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-rejected-page-close-66-reference-KHtCPn.md). All four input endpoints stable.
2. **Source mode: Nx exit0, 63/63 checks**, new [run-1nwmS9 receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-rejected-page-close-66/🧫️run-1nwmS9/📓️receipt.md), plus a complete [sibling receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-rejected-page-close-66-source-1nwmS9.md). All five input endpoints stable.

The actual commands were the existing safe Nx route, with the new controller and `reference` or `source`:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun <ticket>/🧪️store-rejected-page-close-66/📜️script.ts reference
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun <ticket>/🧪️store-rejected-page-close-66/📜️script.ts source
```

The 57 reference checks comprise strict Ajv2020 schema validation and 14 actual invalid-shape rejection probes; unique IDs and sufficient field setup grant; explicit normalization and jsonc-parser document parity; Decimal.js page-grant/ordinal/prefix-sum accounting for all23 observations and total byte conservation; three deliberately wrong zero-byte/tail-rounding/batch expected sequences rejected by the reference; stable input endpoints and exact source capture.

Decimal.js performs the arithmetic and page-sequence calculation without calling the production wrappers or reading authored expected outputs. The calculation limits page work to `min(items,floor(bytes/4096),1)`, removes pages from the tail, and uses decimal prefix sums for retained bytes. The neutral fixture supplies the desired outputs. This is a meaningful independent numeric/sequence check, **not a Rust ownership oracle**.

Source mode adds only six explicitly named source-marker checks: the exact two test names, canonical include, unmounted status, absence of forget/suppressed-drop constructs, actual constructors, and actual returned-owner close calls. These checks do not parse/type-check Rust and are not native test passes.

## Exact Future Include Boundary — Not Applied

The test file is intended as a direct child module of Store, adjacent to the existing `owned_schema_record_tests` mount near [Store line19604](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:19604), before the existing inline `mod tests`:

```rust
#[cfg(test)]
#[path = "🧪️tests/🧬️rejected-page-close/🦀️.rs"]
mod owned_field_rejected_page_tests;
```

This exact placement makes `use super::*` refer to Store and permits the test-only white-box record/lease observations. Mounting it inside the existing inline tests module instead would change the parent/import boundary and is not the reviewed arrangement. No package, launch, or production join is needed for this preparation.

Exactly two native test functions exist:

- `owned_field_rejected_page_tests::registered_rejected_pages_obey_zero_short_and_exact_grants`
- `owned_field_rejected_page_tests::unadmitted_rejected_pages_obey_zero_short_and_exact_grants`

## Native Bodies: Actual Owners, Not a Decoder Model

Both bodies instantiate actual `OwnedSchemaDecodePage`, `OwnedSchemaDecodePages`, and `artifact_envelope_decode_record`.

- Registered construction uses actual `ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new` followed by its existing public `reject`.
- Unadmitted construction uses actual `ArtifactEnvelopeUnadmittedDecodeRejected::new`.
- The registered case observes the original lease's field address, exact ticket/generation, blocked page access before reclamation, then the existing `next_returned_ticket` → `take_returned_ticket` → returned owner's `close_step` sequence.
- Detachment is explicitly checked as **not** returned-owner terminal completion. The detached owner is separately closed under bounded grants.
- A genuine field owner contains one boxed counted token with the authored byte payload. Its bounded close releases that exact token; token address, ID, original payload, field address, exact drop count, and callback counts are checked.
- Field and token Drop guards assert real terminal release. No production Drop was weakened; no `mem::forget`, suppression, replacement ownership, or cleanup clone exists.
- Native record witnesses read original slot storage address, capacity, page/byte counts, sealed state, and actual retained byte prefix. Inline pages are Copy values: the assertion concerns original retained storage/content, not an invented non-copy identity.
- Grant/outcome/content discrepancies are collected. Each subject receives bounded sufficient-grant teardown before the final test assertion. Setup rejection has its own bounded actual-owner cleanup. An unrecoverable unrelated close defect still retains the existing Drop guards and may cause a native failure; it is not hidden as successful teardown.

The small `Subject` enum is test-only dispatch to the two concrete wrappers. It implements no decoding, page retirement, or registry policy; those operations all call the actual current Store methods.

## Expected Native RED and Remaining Limits

The current [registered page branch](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7850) and [unadmitted page branch](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7914) still call the record cursor without checking the byte grant. The current source therefore predicts positive-item zero/4095-byte failures, and early page removal will be reflected in later retained-prefix observations. **That predicted RED has not been executed.**

The Store byte-granule repair is not part of this packet. If the root later mounts these tests and observes the desired native RED, the already reviewed production footprint remains the two wrapper record-page branches. No cursor contract, lease-return, registry API, Fresh decoder, or R17 change is authorized here.

Other limits are explicit:

- The Rust file has been read back and reviewed for actual API shape, but has not been type-checked or compiled.
- The helper code uses the current private Store fields/methods only from the proposed child-module mount. An upstream field/API change requires review, not a claim of unchanged compilation.
- The counted field owner is a test fixture for real transfer/close accounting, not proof of Fresh decoder Err/unwind or poison recovery.
- This packet does not replace the missing full49 20-group/17-law plan or give credit for its unexecuted laws.
- Independent schema/reference success and source markers do not establish memory safety, native compilation, native acceptance, or full owned-retirement completeness.

## Final Source Fingerprints

All first/final inputs in both actual runs were stable. No production Store write occurred.

| File / source boundary | SHA256 |
| --- | --- |
| canonical schema,5454 bytes | `b26e851b5cd1317b4ca799dbbfc117ed33df010ad0178bcf5a2e5db3820bb9a1` |
| canonical vectors,10143 bytes | `efe7c7d8de5e99f140b606c58134afab3e4d375dbb8a0489b543a92aab0524bb` |
| native test source,20374 bytes | `3183a23b62aa769835dad0d1a01da6513c0f5161c8c005b056b97c8e81eed34a` |
| controller,14931 bytes | `5fb860042a37e7a511a127f814aa349d33dbd0d67063ea07d036b5545604306e` |
| current Store,1540921 bytes | `7450f9d6837055d0766a55c5fc98aae22d068ac813acda09c1385a1df48d4c9c` |
| exact two-wrapper source capture | `c178a7dbb964b48bd921f9523625930b67b9f24e7ad493704355a5eaed44f959` |
| source-mode complete JSON receipt | `2dd51eb1fd07fcce6ccf49a8f4cb3cb2aea11c59b128800a0b2eb090e87f563d` |

The controller locates the workspace from the `.🧬semio` ancestor, uses fixed lexical case-insensitive Compose exclusion, full no-symlink ancestry and O_NOFOLLOW/fstat/endpoint checks, and records first and final hashes separately. Every run is exclusive and retains its complete JSON/Markdown receipt, including raw normalized page hex and the exact wrapper source slice. A unique sibling Markdown repeats that complete receipt; this is loss-resilient duplication, not a guarantee against external loss.

