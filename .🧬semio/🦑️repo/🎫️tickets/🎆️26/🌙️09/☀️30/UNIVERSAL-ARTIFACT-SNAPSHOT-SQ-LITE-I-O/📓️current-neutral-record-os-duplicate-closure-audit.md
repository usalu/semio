# Current Neutral Record And OS Duplicate Closure Audit

Read-only current source; no tests or compiler. Complements the current-core-pack-error-context-os-prerequisite-map. Root owns the243 compiler receipt; current source is not assumed identical to that receipt.

## Selected Concrete Authorities

Core Pack package root `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/🦀️.rs`64–65 mounts the full neutral codec as `pack::record`, separately from protocol::value27. The neutral record uses exactly public semio_framework_dsl_record RecordSpec/RecordValue/FieldValue/Shape and semio_framework_value DslValue/Number/controls, not an OS domain type. Its ordinary and controlled record/document/intrinsic decoders return PackRefusal (neutral value root23–29,3059–3235), preserving structural and typed semantic errors without external transport.

Kernel root currently still mounts its own OS Value duplicate at135–136. That3281-line duplicate already imports the same public canonical Record and Value types (13–24) but returns PackError and constructs retired direct variants (25,33–39 and measured158 errors). Its controlled emitter has an extra OutputError Value/Pack bridge whose into_pack still calls retired PackError::ValueRefusal. The neutral162-line emitter directly returns PackRefusal, with otherwise inspected borrowed discovery/frontier/control algorithms corresponding to the OS174-line body. The neutral98-line schema facet likewise uses same firstparty controlled schema graph and semantic PackRefusal; the OS101-line facet retains obsolete product path spelling.

Neutral scalar witness is byte-identical to the actual OS `🎒️pack/🔎️scalar-witness/🦀️.rs` (366 lines). It is mounted at pack::record::scalar_witness (neutral root3272–3273). Thus it is concrete existing authority, not a guessed substitute or copied owner API.

The neutral codec includes intrinsic ordinary/controlled emitters, exact terminal decoder, options/report/schema graph/retained cursors, controlled schema hashing and the new borrowed preflight. All observed public type signatures are same Record/Value owners. The separate borrowed preflight supports its explicit CSV/TSV/List/Text/Bool/Enum scope; it does not replace every codec or imply arbitrary Shape visitors are supported by forecasting.

## Coherent Integration Direction And Limits

Actual canonical callers can target `pack::record::{...}` and `pack::record::scalar_witness::{...}` directly under their already existing Core Pack dependency. Retiring the duplicate OS mount and its forwarding family is a canonical authority consolidation, not a compatibility reexport under the obsolete OS value namespace. It must be paired with all actual consumers and owning selectors, not just suppressing158 diagnostics. No production pair is authored here because the current publication/caller family has open PackError return contracts in Store/SPR and IO.

Pure record callers should retain the narrower PackRefusal result. At a genuinely external transport boundary a semantic cause may be losslessly wrapped with From<PackRefusal> for PackError. An owned PackError transport cannot be projected into ValueError without returning the same original source. This is enforced by canonical PackError::into_value_error Result and existing tests. Broad conversion via string or InvalidValue would erase authentic transport cause and is not a safe prerequisite.

Current OS facade encode/decode forwards (`🎒️pack/🦀️.rs`25–66) depend only on canonical Record/controls. Core `pack::record` now provides the same authored grammar directly; borrowed terminal inputs and controls need no materialized copies or new contexts. FileSource and FileSink are separate real transport authorities; codec consolidation does not justify fabricated caller contexts for CLI/SPR.

## Owning Error Laws Read And Registered Gates

Canonical error refusal test33 lines checks every authored eight-kind message/path and pointer identity through ValueError→PackError, source downcast and complete reference output. Positioned TextError46-line law retains exact kind/span/expected/message allocation/source pointer. The36-line value-projection law checks eight kinds across Value/Text/Io semantic variants, zero observed projection requests, original message pointer, exact declared output and original external transport source returned unchanged. Its structural typed kind is authored independently of misleading canceled prose; no message classifier.

Core producer-authority laws are mounted at package root54–60; named selectors are `canonical_pack_producer_authority_is_required_and_preserves_owned_causes` and `canonical_pack_borrowed_factories_preserve_real_paged_metadata`. IO missing-filename source law is `actual_atomic_writer_missing_filename_retains_invalid_value_authority`, included in actual Core IO207. The latter checks real path refusal, not CLI context provenance.

Actual registered routes from current project/script readback:

- `@semio-tech/framework-pack-error-rs:test-ownership`: strict TypeScript check then independent ownership Source leaf.
- `@semio-tech/framework-pack-error-rs:test-native`: owning error package, extra selector args forwarded via canonical runCargoTestsV1.
- `@semio-tech/framework-pack-rs:test`: complete selected Core package library/tests (caller args via canonical test policy).
- `@semio-tech/framework-pack-rs:test-absolute-varint-native`: `--lib pack_absolute_unsigned_varint_complete_boundaries --no-fail-fast`.
- `@semio-tech/framework-pack-rs:test-borrowed-preflight-native`: `--lib record_borrowed_preflight_ --no-fail-fast` (two prospective/actual laws; Physical independently reported Nextest965f1d65...2/2, not this audit execution).
- Matching absolute-varint and borrowed-preflight Source routes use real strict tsc+Bun tests, no Cargo.

No gate was invoked here. No complete OS replacement, universal allocator proof or full publication replay is claimed. The exact thirteen-file measured locator census remains generated/independent-current-os-243-error-location-census.json; retaining aliases as duplicate codec authorities is not required by any inspected actual type contract.

## Additional Current Publication/Command Caller Readback

Store currently6137 still imports options+PackError from OS; pack_rt6143 onward supplies derived public factories. SpaceHistory native295/263 and shared snapshot-capability decode38 /encode38 call semantic Record functions then `map_err(PackError::into_value_error)`, producing the measured nested Result mismatch. Direct canonical pack::record returning PackRefusal can preserve ValueError through `PackRefusal::into_value_error` without any transport-erasing fallback. This is the actual pure record producer route, not a guessed runtime source classifier.

Command reader typed origin is also explicit: ByteReader primitives now return PackRefusal, but CommandDecodeError only has From<PackError> at Store14776. Its Layout stores actual ProtocolError. Canonical replication wire55–57 ALREADY implements From<PackRefusal> by wrapping PackError::Refusal and preserving source. A direct `impl From<semio_framework_pack_error::PackRefusal> for CommandDecodeError { fn from(error: semio_framework_pack_error::PackRefusal) -> Self { Self::Layout(error.into()) } }` is a verified lossless ordinary conversion for the measured20 ByteReader `?` sites. It introduces no old tuple alias or message mapping. Existing operation-list error indices remain separate Operation variants; do not replace those with Layout.

Store11129–11130 currently `text_error_to_pack_error` constructs retired direct TextRefusal; actual canonical `PackError::from(error)` is lossless and preserves TextSpan/expected/message source. Current generic FromValue sites26451/28383 still stringify typed ValueError as retired Schema; `PackError::from(error)` likewise preserves the full actual kind and message. These direct constructor joins are safe regardless of broader codec consolidation.

Other current Store base64/string compatibility-shaped helpers6277–6286 still construct retired Malformed and parse scene JSON. Their own accepted-domain authority requires a separate field review before declaring full eight-kind mappings; this report does not bless that old bridge or change its legality. Current command enum/from body and all measured primitive scopes were reread; no command output/assertions were changed.

## Store Bridge Limits Independently Reread

Store6143–6252 actual bridge signatures were reread in full. RetainedValue public cursors currently come from OS6155, while real Pack physical cursors already come from Core6156–6160. The schema-dependent document/body forwarding surface6163–6190 has no OS-only owner types. Its controlled terminal intrinsic forwarders6245/6250 directly borrow DslValue and reuse the actual supplied Native controls; neutral Record has these exact APIs. This supports canonical consolidation without materializing any synthetic record or copying the intrinsic tree.

The ordinary full-document bridge6218–6223 remains a distinct preexisting permissive path: it clones the extracted value and returns DslValue::Null when the required Value field is missing. It ignores DecodeReport. Ordinary encoder6210 clones whole DslValue into an owned record, and ordinary wire6227 does likewise. By contrast strict ordinary terminal wire6240 and controlled wire6250 call exact terminal decoders with no Null fallback or second clone. Do not claim consolidation alone repairs full-document ordinary admission/ownership, or silently redirect envelope grammars. A future strict full-document law needs malformed missing/wrong/extra field and full output occurrence/word assertions against the actual ArtifactPack for DslValue. No test or producer change was made here.
