# Retained Layout and Drawing Validation

## Current Result

The borrowed Layout graph cursor and genuine typed Draw admission facet are implemented in TypeScript and Rust. Their runtime paths do not call the cold validator, clone snapshots, copy authored keys, parse raw Drawing payloads, or use vector lowering as validation. The full Layout artifact remains incomplete: current Layout Document/Page fields still contain raw Drawing content and cannot produce a genuine typed member source. Those children receive an explicit funded refusal until Core completes the member/ref/genesis source migration. Structural virtual post-state validation and all 59 live mutation kinds are not claimed here.

## Source and Custody Audit

The inspected source is Core's actual ChildEditorSourceCustody at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪆️child/🎯️editor/📖️read/🔗️custody/🦀️.rs`. Its first funded stage duplicates the original ChildContentEntry SnapshotRead lease; its second calls SnapshotRead.admit_retained_clone_source with the original remaining grant. Failed issuance restores the same read. Its opaque_source erases the issued source with fixed tag 0x4348494c. Draw binds that exact erased incoming source before native downcast, so a different issuer, projected discriminator or native type cannot replace it. Bare borrowed snapshots and Arc copies cannot issue admission authority.

Draw's cursor holds an original source binding while its immutable borrowed snapshot and scratch slots are lent to the fixed continuation. No index growth occurs after the continuation is created. Funded closure drops the continuation first, then each scratch slot and backing, then ancestry slots/backing, then the source binding. The neutral opaque wrapper drops its exact native concrete cursor box only after that cursor is empty. Caller must then begin custody closure and retire custody; the issued alias closes before its held source read lease. The source producer and consumer contracts fit; the actual Layout-to-child route is not integrated yet.

## Work and Storage

Layout owns a fixed first-party scratch hash index containing source addresses, numeric marks and counters. Every UTF-8 key hash/collision comparison is bounded to 256 bytes; thread graph edges and style-run boundaries yield independently. Native UTF-8 boundaries borrow the original text. TS uses bounded checkpoint indexes for long style-run text. Linear thread and dense style-run scaling are checked. The framework scratch API now supports funded reserve, one-slot initialization, original slot probes, vacant insertion, and one-slot/backing retirement. No ordinary all-key pairwise search remains.

Draw owns a fixed index of original paged layer/asset key references and a paged borrowed ancestry walker. It validates identities, all authored geometry and paint variants, finite composed transforms, asset sample extents, reference resolution, and group/boolean graph acyclicity. It emits optional canvas dimensions only after positive finite authored dimensions pass admission. Its policy accepts authored degeneracy and large finite coordinates; scene/render work limits are separate. An actual continuation future allocation is measured with size_of_val, funded before Box::pin, retained through cancellation or semantic error, and released exactly.

Native parent wrappers add their own bounded copy/depth work, forward the actual remaining grant, and preserve child receipts on errors. Initial Layout binding now quotes its own bookkeeping rather than accepting a zero-copy turn. TypeScript storage accounting is an explicit logical model; native Rust accounts physical cursor/future/index/ancestry allocation capacities. Native physical claims remain authored and unexecuted.

## Executed Evidence

The durable input harness `retained-validation-check.mjs` runs strict TypeScript and the in-process checks without a shell fallback. Latest result: zero strict diagnostics; 67 Layout cases agree with the existing owned AJV/Kahn oracle and cold test-only semantic owner; 12,259 turns, six cancellation cuts, 532 long-key turns. A 256-to-512-frame chain takes 8,524 to 17,012 turns; dense Unicode runs take 11,515 to 18,119 turns.

The Draw corpus has 51 shared cases compared with an independently owned AJV projection and Kahn topology oracle: 4,321 turns and 14,279 denied-grant attempts. Eight cancellation cuts pair all logical births with release. Long paged keys take 384 turns; 128 nested groups take 2,969 turns. Cases include every shape kind, solid/linear/radial paint, curved paths, missing references, self/ancestor cycles, duplicate keys, asset shape, affine overflow, large finite coordinates and degeneracy. Source issuer/projection/value substitution is rejected. Six neutral opaque-envelope laws verify actual original remaining grant forwarding, denied child calls, receipt limits, child-before-box closure and exact paired release.

The existing and incremental scratch TS tests were also executed in-process with existing AJV/Map/fast-json-patch witnesses: two passed. Rust source has only been parsed with tree-sitter: All 16 changed native sources, including the final denial laws, parsed without syntax errors. This is not native compilation or execution. Required repo execution remains unavailable because repo --help is CommandNotFound; no raw cargo or bun bypass was used.

## Exact Agent-Owned Changed Paths

These are this agent's exact source/test/audit paths, not a claim of exclusive ownership of shared mount files. Existing unrelated content was preserved. All paths are below the 256-character limit.

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/📇️index.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/📇️index.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/📋️program.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/🔮️oracle.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/🧪️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/🧪️tests.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/🧪️tests.ts`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/📇️index.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/📋️program.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🔮️oracle.mjs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🟦️.ts`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️tests.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️tests.ts`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧵️source.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🔣️.json`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🧪️.json`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🧪️tests.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🧪️tests.ts`
- `🧰️framework/🔨️modules/🌱️value/🔑️hashed/🧮️scratch/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🔑️hashed/🧮️scratch/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🔑️hashed/🧮️scratch/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🔑️hashed/🧮️scratch/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🔑️hashed/🧮️scratch/🧪️tests/🧫️fixtures/📇️incremental.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/📦️packages/🟦️typescript/🚪️io/🔌️ports/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/retained-validation-check.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/RETAINED-VALIDATION-DESIGN.md`
