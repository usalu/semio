# Norm Config Mutation Strict-Admission Audit

Status: source-closed for the one-leaf JSON/text/binary operation boundary; coordinator-reported registered source gate `36156` is green for the 13 text/25 binary corpus. The exact Rust `config-mutation-test` runtime remains unverified by this audit. This is a read-only current-source review; no compiler, Cargo, Nx, or runtime test was started here.

## Current authority and the split boundary

`NormConfig` is a single schema-owned type with one optional `u32` field at [`🎚️config/🧬️schema/🦀️.rs:4-14`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🦀️.rs:4). Its closed operation aggregate contains exactly `ChangeSelectedCheckIndex` ([`🧬️mutations/🦀️.rs:6-10`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:6)); the leaf declares `index: Option<u32>` and `#[value(deny_unknown_fields)]` ([`☑️change-selected-check-index/🦀️.rs:5-10`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🧬️mutations/☑️change-selected-check-index/🦀️.rs:5)). The JSON schemas correctly express one aggregate tag and `additionalProperties: false` at both levels ([leaf schema:5-6](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🧬️mutations/☑️change-selected-check-index/🔣️.schema.json:5), [aggregate schema:5-9](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🧬️mutations/🔣️.schema.json:5)).

That closure reaches JSON through a `serde(deny_unknown_fields)` oracle and retired/extra aggregate negatives ([`🎚️config/🧪️tests/🦀️.rs:6-10,39-48`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎚️config/🧪️tests/🦀️.rs:6)). The current source also routes terminal text and binary through shared exact boundaries, described below. `#[value(deny_unknown_fields)]` alone remains insufficient; it is the schema declaration, not the operation-text or record-body admission check.

## Current reread: text and binary closure is source-closed

The current text owner invokes `dsl::parse_exact` after choosing its one declared keyword ([`📝️text/🦀️.rs:5-14`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🧬️mutations/📝️text/🦀️.rs:5)). `parse_exact` lexes the complete input, parses one record, and requires EOF ([`dsl schema:668-674`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:668)). This closes valid-prefix unknown fields, duplicate fields, second operations, and arbitrary tail tokens without changing compositional `dsl::parse`.

The binary owner is now only the shared `dsl::variants_binary::{encode_op,decode_op}` ([`💾️binary/🦀️.rs:5-12`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:5)). Its decoder retains checked ordinal conversion, calls `decode_record_body_exact`, and requires byte-for-byte canonical re-encoding ([`dsl/🦀️.rs:366-385`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs:366)). The new shared terminal body decoder rejects both suffixes and any reported unknown ID, while retaining the old forward-preserving decoder for documents ([`pack value:2172-2210`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/🔢️value/🦀️.rs:2172)). Canonical equality additionally rejects non-minimal varints and duplicate known fields that might otherwise decode into the same typed value.

The fixture now carries 13 text and 25 binary vectors; the public Rust law consumes every one and checks state/inverse/canonical re-encoding ([`config test:53-79`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎚️config/🧪️tests/🦀️.rs:53)). The Bun source gate independently parses the fixed v1 bytes with `Buffer`/`BigInt`, applies AJV, and refuses a non-shared text/binary route ([`script.ts:102-161`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts:102)). This is appropriate independence: it does not import the Rust decoder. Coordinator-reported terminal session `36156` is green for this source gate; I did not independently execute it and do not treat that as Rust/runtime evidence.

## Superseded bypasses retained for audit trace

### Prior text valid-prefix acceptance

Before the current patch, `NormConfigMutation::parse_op` first identified the declared keyword, then called generic `dsl::parse` without requiring the cursor to reach EOF. `dsl::parse` delegates to `parse_record_body` only ([`dsl schema:658-666`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:658)); the record-field loop deliberately stops when the next token is not a remaining known field ([`dsl schema:1385-1421`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:1385)). That is correct for nested/statement parsing but was wrong at an operation-line boundary.

Consequently each of these previously parsed as `ChangeSelectedCheckIndex { index: Some(5) }`, rather than reject:

- `change-selected-check-index index=5 unexpected=0` — unknown attribute starts the next record/statement instead of failing;
- `change-selected-check-index index=5 index=6` — the first keyed field is removed from the parser candidate set, so the duplicate is left unread;
- `change-selected-check-index index=5 another-operation` — a second statement-like suffix is left unread; and
- `change-selected-check-index index=5 garbage` — arbitrary legal token tail is left unread.

`parse_expr_text` was the local correct model: it explicitly performs `cursor.expect(TokenKind::Eof)` after parsing ([`dsl schema:827-832`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:827)). The expanded fixture now covers the valid-prefix hostile forms.

### Prior binary trailing, unknown, duplicate, and non-canonical acceptance

Before the current patch, the handwritten decoder read the format and variant ordinal, took the rest of the input as a body, then discarded the returned `DecodeReport`. It also used `ordinal as usize`, so a 32-bit target could truncate a large ordinal into a valid variant index.

The shared `decode_record_body` returns after decoding one record without an EOF/`remaining()==0` check ([`pack value:2175-2195`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️value/🦀️.rs:2175)). Its default options deliberately preserve unknown fields ([`pack value:2052-2061`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️value/🦀️.rs:2052)), and `decode_record_fields` records unknown field IDs rather than rejecting them and overwrites duplicate known IDs (`RecordValue::insert`) ([`pack value:1491-1523`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️value/🦀️.rs:1491)). The generic decoder permits non-minimal LEB128; only a separate helper recognizes minimal encoding ([`codec:104-126,140-153`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs:104)).

So a canonical valid encoding plus a byte suffix was admitted; a body with an unknown numeric field was reported then ignored by the derived `from_named_record`; duplicate known fields followed last-write-wins; and non-minimal ordinal/field/count varints were admitted. The current 25-vector binary fixture covers those hostile paths.

## Landed schema-first repair

The following recommendation is now materially landed. It correctly does not change `DecodeOptions::default()` or make document decode reject unknown fields: full documents intentionally use forward-preserving decode/report behavior. The terminal operation is a separate shared boundary.

1. In the shared pack module, add a closed-record decoder which consumes exactly one `encode_record_body` payload, requires `reader.remaining() == 0`, and rejects a nonempty `DecodeReport::unknown_field_ids`. It must use the caller's `RecordSpec` and standard limits; it must not drop unknowns and continue. Keep existing `decode_record_body` semantic for document/forward-compatible callers. Its error should name the record-body field/offset rather than silently canonicalizing an invalid input.
2. In the DSL operation helper, make `variants_binary::decode_op` use that closed decoder, retain its existing `usize::try_from(ordinal)`, and require its canonical `variants_binary::encode_op(&decoded)` to equal the original bytes. Route Norm’s bespoke `OpBinary` through that one helper or mirror it exactly with a single shared closed helper; do not maintain two competing operation grammars.
3. The equality is legitimate **only for binary operation wire**: `encode_record_body` is documented deterministic ([`pack value:2143-2169`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️value/🦀️.rs:2143)) and catches duplicate fields and non-minimal outer/body varints that still have the same semantic record. It is not a replacement for strict decode/report rejection, and it must not be imposed on human text, where harmless whitespace and keyed-field order are noncanonical presentations.
4. Add `dsl::parse_exact` (or an equivalently named record-terminal API) beside `parse`: lex, parse one record, then require EOF. Preserve `parse` for compositional document/statement parsing. `NormConfigMutation::parse_op` must use the exact API after its current declared-keyword check.

This is a single authority packet: the schema remains the owner of allowable field IDs and shape; Pack owns exact byte completion; DSL owns exact text completion; the operation wrapper merely chooses the declared variant. It adds no aliases, permissive migration branch, generic JSON patch, or second hand-written schema parser.

## Required neutral fixture and gates

Extend the existing neutral `semio.norm.config-mutation/v1` fixture rather than inventing a parallel contract. Retain the five good JSON semantics and five hostile JSON values, then add a `text` vector collection and a byte-hex `binary` vector collection with `{id, accepted, canonical?}`. Required rows:

- valid clear (missing `index`), `null`, `0`, and `u32::MAX` in every representation;
- text valid-prefix unknown, duplicate key, second operation, and arbitrary tail; each rejects;
- binary canonical valid clear/select, then valid plus trailing byte, unknown field ID, duplicate field ID, non-minimal outer ordinal, non-minimal field/count/value varint, wrong ordinal, bad format, and all truncation positions; every hostile input rejects;
- an accepted binary row must re-emit exactly its supplied canonical hex. An accepted text row need only produce the correct typed operation; it need not equal `print_op()` byte-for-byte.

The Rust `config_mutation` integration test should consume every row and assert decode/parse success or failure, post-state, inverse, and binary canonical equality. The Bun/AJV gate should continue independent schema checks and add a small fixture-only byte oracle that implements this one fixed v1 frame grammar without importing Rust/Pack code: minimal ULEB128, exact symbol/body EOF, one known optional `u32` field, and no unknown/duplicate field IDs. That makes malformed acceptance detectable from a second implementation rather than merely from the same decoder’s encoder.

The existing registrations are the right owners after expanding the vectors: `config-mutation-source` validates schemas/AJV and route ownership at [`📜️script.ts:123-161`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts:123), and `config-mutation-test` runs the exact public Rust target at [lines 165-170](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts:165). The project’s standard Nx invocation is `bun nx run @semio-tech/norm-plugin:config-mutation-source --skip-nx-cache` followed by `bun nx run @semio-tech/norm-plugin:config-mutation-test --skip-nx-cache`, subject to verifying the current project name before execution. Neither was run here.

## Public factory-law check before 120 bodies

The current public-surface test is source-credible in its factory selection but runtime-unverified. It gets the actual `plugin()` result, compares fixture IDs to the manifest and definition registry, creates each app via `plugin.create_app`, and renders every fixture key under explicit `Locale::En`/`Terminology::Native` ([`🖥️app-surface/🧪️tests/🦀️.rs:55-100`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:55)). The runner detects an omitted inherited history body because the fixture and definition-derived keys must be equal before rendering. The framework really intercepts `framework.body.history` before app key matching ([`plugin/🦀️.rs:24566-24576`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24566)).

Its current direct projection correctly avoids serializing a populated `BuiltChildren`, uses the producer's 384-node capacity, and drains retained pages on both success and assertion panic. The source-only details are recorded in [`📓️terra-norm-strict-plugin-assembly-and-public-harness-audit.md`](📓️terra-norm-strict-plugin-assembly-and-public-harness-audit.md). Its material nonclaims are intentional: it creates default snapshots only, exercises no interactive command/effect or child-content load, and tests English/native only. The exact factory runner remains unexecuted, so the claimed 30 factories/120 body outcomes are not a runtime result yet.

## Acceptance boundary

Do not call config mutation runtime-strict until both exact registered gates terminally pass on the expanded neutral corpus. The current source no longer relies on JSON/AJV pass, Rust compile, or a mere `decode(encode(x))` round-trip: it has exact hostile text/binary vectors and a separate byte oracle. It still does not claim document-pack strictness, all other operation codecs, native rendering, localization parity, or config-action publication.
