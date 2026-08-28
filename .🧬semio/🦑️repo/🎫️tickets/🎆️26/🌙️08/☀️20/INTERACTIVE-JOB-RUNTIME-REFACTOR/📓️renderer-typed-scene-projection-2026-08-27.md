# Typed Scene Projection

## Owned Contract

`contract/retained/scene/typed` now validates all15 exact native kind/schema roots against a strict schema-first catalog with26 nested record definitions. The catalog has fixed32-field metadata capacity. A prepared record holds source offsets and static default literals; it never reconstructs a user string, nested array or document object. Unknown fields remain in the original owned arena but are ignored by the typed projection, matching native serde struct handling. Unsupported schema or invalid fields produce an explicit retained failure, never an empty successful scene.

The projection captures the exact original scene root, traverses one task/lookup/field/index transition per1-item4096-byte grant, and retains every reader, task and candidate index until explicit close. Prepared record readers capture their own root. Text/value readers independently capture the original source, so they can outlive both prepared documents and record readers without rebinding to equal-valued documents. The prepared index is separate from wire nodes and does not alter wire hashing.

Static field strings remain catalog-owned; retiring a prepared record releases at most32 fixed metadata entries. The3072-byte record retirement envelope does not include copying or freeing a user string. User bytes remain in the original256-byte pages and retire through the existing typed component owner. `usize` values are retained as exact unsigned64-bit wire scalars; narrowing for a concrete32-bit session remains an explicit host-admission obligation, not an implicit Number conversion.

## Actual Tests

Canonical command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t TypedScene'`.

- R1: missing implementation module,4failed suites/0executed tests. This is a source TDD boundary, not behavioral evidence.
- R2:1PASS/555skipped/556total,7.47s; strictAjv catalog+fixture,15 valid schemas and6 hostile cases, exact record-reader ownership, Immer kind-list oracle.
- R3:2PASS/1FAIL/562skipped/565total,9.84s. The additional long-text test incorrectly bounded emitted UTF8 to128 rather than newly consumed source bytes. A scalar can retain3 prefix bytes across turns, so output can be131 bytes. Production already reads at most128 new bytes and accounts actual source reads plus UTF16 output bytes. No production budget changed.
- R4:3PASS/563skipped/566total,18.80s, start17:07:11. Fixture now explicitly records131 maximum emitted UTF8 bytes. Includes every observed projection prefix cancellation on a small text scene, zero/subgrant rejection, constructor mint forgery, frozen property shadowing,22000 UTF8-byte text, two independent text readers, equal-valued foreign root closure and final page invalidation. All observed work/close steps remain within1/4096. The exact prefix count is asserted greater than100, not printed or claimed as an exact census.

Full outputs are retained in `renderer-typed-scene-{red-r1,r2,r3,r4}` ticket logs. R4 is targeted, not a full renderer pass. Parent fullR16 independently passed553 before these three tests; strictR21 remained exactly7 existing tutorial joins.

Native generic ScenePack parity is now independently executed: child R2 passed1test/0failed/93skipped against19 shared vectors. R1 failed because a test used serde_json generic map serialization, which the production native serializer deliberately does not support; only test producers changed to typed serde structs. Production bytes and19 vectors were unchanged.

An additional isolated native test `typed_scene_neutral_catalog_matches_native_serde_contracts` is source-ready and queued with the sole compiler owner. It runs the same15 valid/6 hostile typed fixtures through actual `SceneDoc::decode_pack` and native serde structs. No native result is claimed yet.

## Remaining Work

Not mounted in Interpreter, UiNodeView or wgpu. Atomic parallel node/prepared-scene publication, paired read ownership, all15 host consumer projections, nested JSON/pack preparation and exact per-instance aggregate close remain active. Old synchronous Interpreter decoding is still present and is not credited. No8ms maximum-envelope timing certification is inferred from bounded logical work. No cleanup, file deletion, relocation, git mutation or ticket close was performed.
