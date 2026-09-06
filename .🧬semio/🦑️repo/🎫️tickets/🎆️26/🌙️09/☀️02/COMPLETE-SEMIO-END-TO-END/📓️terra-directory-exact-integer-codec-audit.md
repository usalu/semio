# Exact Integer Decode Audit for Directory Authority

## Result

The actor-specific `byteLength: 1.5` rejection is now qualified by the existing actor-native law (root reported `pNZ7zK`, one native law and 49 cross-language corpus rows). It is a correct bounded mitigation for that field, but it cannot protect the many other `FromValue` integer leaves.

The shared scalar implementation is the root defect. Both macros accept all three `Number` variants through an unchecked Rust `as` conversion:

- [unsigned decoder](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:62) converts `UInt`, `Int`, and `Float` into `u8`/`u16`/`u32`/`u64`/`usize` at lines 73–75.
- [signed decoder](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:87) does the same at lines 98–100 for `i8` through `isize`.

That admits fractional truncation, signed-to-unsigned wrapping, narrow-width truncation, and Rust float-to-integer saturation before a domain validator receives the value. This contradicts the documented distinction between `Number::UInt`/`Int` and `Number::Float` in the same module’s tests ([lines 490–545](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:490)).

## Direct Directory Boundary Exposure

This is deliberately a short census of fields decoded from uncanonicalized request/response bodies, rather than every integer in the directory model.

| Boundary | Fields decoded before validation | Why current validation is too late |
| --- | --- | --- |
| `POST …/open-plan` and all execution-target body routes | `DocumentOpenIntentV1.version: u32` | `1.5`, `4_294_967_297`, or a suitable negative integer can reach `version == 1` after the scalar cast. The Hub parses the body then validates it at [bin.rs:2265](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2265). The intent type is at [schema.rs:1340](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1340). |
| Socket-grant exchange | `DocumentPlanSocketGrantIntentV1.version: u32` | The receipt remains the real authority, so this is not an authority escalation by itself, but malformed version syntax is admitted before [its `version == 1` validation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1606). |
| Authenticated open-plan response | `DocumentOpenPlanV1.version`, `expires_at_unix_ms`, all four `DocumentOpenRevalidationV1` generations, and optional checkpoint `ArtifactFrontier.head_edit_ordinal`/`last_commit_seq` | The client’s sole JSON choke point is [client.rs:474](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:474); it decodes the plan at [line 1137](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1137), then validates the already-coerced values. The relevant plan/revalidation types are [lines 1433–1463](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1433), and frontier leaves are [lines 2361–2366](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2361). Existing max-safe checks prevent many overflow values, not fractional values that truncate to a valid generation or expiry. |
| Execution-target manifest/lease | `DocumentExecutionTargetLeaseFieldsV1.version`; component and descriptor `byte_length`; the same revalidation and checkpoint frontier leaves | The byte-length caps at [schema.rs:1695](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1695) only inspect the coerced `u64`. These are integrity fields, so `1.5 → 1` is exactly the class of disagreement a byte identity must not permit. The types are at [lines 1622–1658](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1622). |
| Protected socket-grant response | `SocketGrantReceiptV1.expires_at_ms: i64` | The receipt is decoded at [client.rs:1103](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1103) and only compared to the clock at line 1104. A float such as `1e300` currently saturates through `as i64` to a future value, so this is the highest-priority response-side field. |

The authenticated directory event, command-receipt, and administration-page routes have a compensating raw canonical-reencode check (for example [event page lines 319–326](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:319)). That rejects numeric spellings which a cast changes. It does **not** cover the open-plan, target-selection, plan-response, or socket-grant paths above, and it should not be relied upon as the scalar contract.

The freshly added `DocumentBrowserActorByteLengthV1` at [browser-actor.rs:13](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🦀️.rs:13) rejects `1.5` before its 64 MiB range check. Its `as_f64` implementation is magnitude-safe because that range is below `2^53`; it is intentionally a domain exception accepting an integral float such as `1.0`. It must not be copied into authority DTOs: it does not preserve integer-token identity and cannot repair the common scalar leaves.

## Small Coherent Repair

Change only the shared `FromValue` scalar contract, retaining domain range constructors such as `DocumentBrowserActorByteLengthV1`.

1. Define exact integer conversion in [the value owner](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs):

   - unsigned target: accept `Number::UInt(n)` and non-negative `Number::Int(n)` through `Number::as_u64()`, then `<T>::try_from(n)`;
   - signed target: accept `Number::Int(n)` and in-range `Number::UInt(n)` through `Number::as_i64()`, then `<T>::try_from(n)`;
   - reject every `Number::Float`, including `1.0`, `1e0`, `NaN`, and infinities.

   `Number::as_u64`/`as_i64` already encode precisely that cross-signed, no-float conversion at [value.rs:44](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:44). Using them plus `TryFrom` removes all `as` operations from `FromValue`; retain `as` only in `ToValue`, where the declared primitive is already representable.

2. Keep domain semantic bounds in domain types. The global codec says a value is an exact wire integer; `DocumentOpenPlanV1::validate` remains the owner of max-safe revision/expiry rules, and `DocumentBrowserActorByteLengthV1::new` remains the owner of `1..=64 MiB`.

3. Do not introduce a compatibility conversion. A float token is a distinct `DslValue` variant and `serde_json::from_str::<u64>("1.0")` rejects it. Accepting it in the generic integer codec would preserve the ambiguity that the current module’s own scalar tests say must remain visible. If browser actor JSON deliberately keeps semantic `1.0` acceptance, that remains confined to its named domain wrapper and needs a documented cross-language raw-body rule when it is wired to transport.

## Schema-First Corpus and Oracle

Add one owner-adjacent neutral corpus, e.g. `🌱️value/🔁️codec/🧪️fixtures/🔣️.json` plus its adjacent schema. Store the candidate as a **raw JSON number string**, not a JSON number, so `1`, `1.0`, and `1e0` cannot collapse while loading the fixture.

Required rows, for each applicable target family, are:

- accepted: `0`, positive integer, `u64::MAX`; `i64::MIN`; `i64::MAX`; positive `UInt` fitting an `i64`;
- rejected: `-1` for unsigned; `u64::MAX` for signed; each narrow-width `max + 1`; `1.5`, `-1.5`, `1.0`, `1e0`; `4_294_967_297` for `u32`; and a `u64` literal above `2^53` that must remain exact rather than be widened;
- direct `DslValue` hostile rows for `Number::Float(f64::INFINITY)`, `NEG_INFINITY`, and `NAN`, which JSON cannot spell.

For every textual row, the native law should parse the same raw token in two independent target paths:

```rust
let json: serde_json::Value = serde_json::from_str(row.raw)?;
let ours = T::from_value(DslValue::from(&json));
let serde = serde_json::from_str::<T>(row.raw);
assert_eq!(ours.is_ok(), serde.is_ok());
if let (Ok(ours), Ok(serde)) = (ours, serde) { assert_eq!(ours, serde); }
```

The direct `DslValue` rows supplement rather than replace the serde oracle. This avoids treating serde as the production codec, while proving the scalar acceptance/range contract against an independent implementation.

## Qualification Placement

The live actor route already has a registered native selector:
`@semio-tech/hub-rs:document-browser-actor-identity-native-check`, implemented by [DocumentBrowserActorIdentityCheckScript](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3472). It currently runs only the OS-kernel actor law at [line 3480](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3480).

Extend that existing exact-law run with a second group for package `semio-framework-replication`, target `lib`, and a new owner test named `value::codec::tests::integer_from_value_matches_serde_without_coercion`. The replication crate mounts this exact value owner at [replication crate root:33](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs:33). The one selector will then prove both the neutral actor schema and the scalar primitive that all plan/lease/receipt derives use. No new broad build target is needed.

No build was run for this audit.

## Re-review After the Shared Scalar Repair (2026-09-06)

### Current result

The shared primitive repair now visible at [value codec lines 62-106](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:62) is the coherent fix described above: unsigned targets use `Number::as_u64()` plus `TryFrom`, signed targets use `Number::as_i64()` plus `TryFrom`. Those accessors accept only `UInt`/compatible `Int` or `Int`/fitting `UInt`, respectively, and reject every `Float` at [value lines 44-60](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:44). There is no cast between a dynamically decoded number and an integer target left in this shared `FromValue` path.

Root reported the source check GREEN and native receipt `yYxoQH` for the 38 raw tokens by ten primitive targets plus direct non-finite/whole-float denials. I did not run that qualification.

The current neutral corpus is correctly lexical rather than pre-parsed: [fixture JSON](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️fixtures/🔣️.json) stores raw strings including `1.0`, `1e0`, overflow integers, and `-9223372036854775809`; [the native law](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:422) compares both the production bridge and `serde_json::from_str::<T>` for each. The actor identity exact-law runner includes this test under the actual `semio-framework-replication` `protocol` target at [Hub script line 3481](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3481); it is not a disconnected unit-only target.

### Downstream authority recheck

No further numeric truncation choke point was found in the current public Directory/HuB path:

- Raw authenticated `open-plan`, plan-exchange, execution-target, and admin intent bodies call the first-party variant-preserving parser at [Hub lines 2266, 2385, 2449, and 5771](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2266). `pack::json` emits `UInt`/`Int` only for bare integer spelling and emits `Float` for decimal, exponent, or integral-overflow spelling at [its lexer lines 629-693](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:629). Derived `uN`/`iN` fields therefore now reject before `validate` rather than being truncated first.
- The generic Axum `DirectoryJson<T>` bridge converts `serde_json::Value` to `DslValue` without widening its stored `u64`/`i64` representation at [Hub lines 162-165](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:162) and [value lines 242-260](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:242). Decimal/exponent values become `Float`, hence cannot enter a generic integer leaf after this repair.
- SQLite recovery takes the same `serde_json::Value -> DslValue -> FromValue` path for persisted directory event and checkpoint JSON at [SQLite lines 2114-2144](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2114). It no longer has a reopen-only integer coercion route.
- The only current manual Directory byte loop, `ArtifactHash`, uses the already strict `DslValue::as_u64()` followed by `u8::try_from` at [schema lines 103-115](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:103), so it did not retain the old float cast.

### Intentional semantic-number exception

`DocumentBrowserActorByteLengthV1` deliberately has a different, bounded public contract: it accepts a finite mathematical integer in `1..=67_108_864`, including JSON spellings `1.0` and `1e0`, through [its named decoder](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🦀️.rs:30). That is not a truncation hole: the maximum is far below `2^53`, it checks `fract() == 0`, and the TypeScript twin intentionally admits `Number.isSafeInteger` in the identical range at [browser-actor TS lines 50-52](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts:50). A browser receives a parsed JavaScript `number` and cannot retain whether its source token was `1` or `1.0`; forcing token strictness here would make the Rust/TS public schema disagree.

This exception is correctly confined to the named actor-length type. It must not be copied to versions, receipts, revisions, plan/lease byte lengths, sequence fields, or hash bytes, which retain the primitive integer-token contract.

### Small remaining test gap (P1)

The actor corpus currently rejects `1.5`, negative, unsafe, zero, and maximum-plus-one lengths, but its [49 current rows](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🧪️fixtures/🔣️.json) do not explicitly establish the intended semantic whole-float compatibility. Add raw-input rows exercised by both twins:

1. lease `byteLength: 1.0` → accepted;
2. lease `byteLength: 1e0` → accepted;
3. lease `byteLength: -0.0` → rejected by the lower bound;
4. retain `1.5` → rejected.

The Rust side must feed the raw JSON text directly to `os_pack::json::from_json_str`; the TypeScript side may feed `JSON.parse(raw)` to its public parser. This proves the deliberately distinct semantic contract without weakening the new generic integer-token law.
