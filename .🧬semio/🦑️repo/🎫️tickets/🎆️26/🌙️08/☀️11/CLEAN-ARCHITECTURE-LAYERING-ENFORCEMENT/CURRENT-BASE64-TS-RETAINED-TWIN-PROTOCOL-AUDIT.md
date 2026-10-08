# TypeScript Retained Twin Protocol Audit

Read-only source: General IO Base64 `🟦️.ts`. No production changes, execution or physical VM budget claims.

## Existing behavior that matters

The standard controlled decoder currently allocates one four-number array and one three-byte scratch typed array before validation, then its final typed array. These are per-call, not per-quartet allocations. Public decodeBase64Quad validates codec errors before checking caller output storage and writes nothing for malformed/noncanonical quartets. Preserve that precedence when routing it through the new primitive.

Current public malformed carrier is Base64DecodeError with first-party Base64Error detail. Limit/cancellation use plain Error strings. User progress can throw any JavaScript value, including undefined, null or a non-Error object. Current synchronous code propagates that exact value.

Current TS validation scans quartets in order; Rust scans each nonfinal4096-byte chunk for padding before decoding its quartets. Therefore an earlier invalid byte plus later misplaced padding in the same nonfinal chunk can have different refusal precedence today. A twin implementing Rust work demand must explicitly choose the shared nonfinal precheck and add a neutral competing-error vector; do not describe that as preserving existing TS precedence. TS accepts string input and reports UTF16 charCodeAt units for non-ASCII; Rust bytes report UTF8 octets. ASCII is the common admitted transport. Add explicit Unicode refusal witnesses and avoid claiming identical Unicode error byte/index without a chosen source representation.

## Minimal primitive and retention contract

A private packed quartet function should take four numeric code units, absolute source index and last flag; return first three decoded octets in low24 bits and count in bits24..25. The maximum value remains below2^26; extract with unsigned shifts and masks. No array/object/scratch return per quartet. Public decodeBase64Quad keeps its original length check, invokes this primitive, checks output admission, then writes count bytes. The retained class calls the same primitive directly from immutable string charCodeAt values and writes only in Decode.

Keep input immutable string private; validation must not reconstruct a full byte copy. Capture length and final padding only in constructor. Retained work phases/demand should match Rust: start/transition1; at most4096 source code units per pass; nonfinal Validate precheck doubles demand up to8192. Zero/undersized valid grants must not invoke control/callback or mutate written/progress. Validate grants as finite nonnegative safe integers, with an explicit first-party refusal for invalid grant shape before work. That admission is distinct from Rust usize typing.

Parts need input:string, output:Uint8Array|undefined, written:number, complete:boolean and an explicit refusal discriminant. An allocated output retains full admitted size while written denotes the initialized meaningful prefix. Withdrawal must return the exact original Uint8Array object, never slice/subarray/new Uint8Array, and set the cursor consumed. A partial output can contain a zero-filled suffix; consumers must use written. Completion permits one exact output transfer. After a transfer, parts output is absent, complete remains true and written remains the final count. Repeated consuming withdrawal or use after withdrawal should throw a first-party state error rather than restart or invent empty ownership.

Freeze first refusal with a separate hasRefusal boolean or discriminated union. A field of type unknown alone cannot distinguish no refusal from `throw undefined`. Store and rethrow the exact first thrown unknown, preserving object identity and primitive value. Catch callback exceptions as retained refusal before any retry; do not normalize them to cancellation or wrap them. A callback-false cancellation may use a first-party typed control error. Codec, allowance, allocation and callback errors remain distinguishable. Explicit cancel should be inert once terminal/refused/consumed.

## Finite TDD vectors

Use shared neutral lengths/grants and existing Buffer test oracle; compare final bytes, exact phase checkpoint roster, callback counts and written state. Add thrown undefined/null/object callbacks at Validate0, Decode0 and Decode4096, assert frozen original value across later changed control and withdrawal. Assert output object identity after partial withdrawal without copying; constructor text equality demonstrates retained string value, not physical VM pointer identity. Add malformed competing errors across4096 boundary, Unicode, invalid grants, cancel lifecycle, malformed parts and allowance shrink after partial output. No allocation-count claim follows from JavaScript object identity alone.
