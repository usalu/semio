# Current Pack Thirty-Seven Typed Error Authority Prerequisites

Actual supplied owning log reached zero assertions,37 compiler errors. Exact machine census is `🗑️generated/independent-pack-current-typed-error-diagnostic-census.json`. No replay/compiler/Source executed by this audit. Current source still has the measured mismatched contracts; these are not assertions proving varint or whole Pack behavior.

| Actual Pack test path | Diagnostics | Measured line set |
| --- | --- | --- |
| 🌐️http/🧪️tests/🧭️producer-authority/🦀️.rs | 5 | 3, 11, 25, 27 |
| 🧪️tests/🧭️producer-authority/🦀️.rs | 19 | 2, 36, 37, 38, 39, 42, 47, 48, 55, 56, 60, 77, 78, 83, 84 |
| 🧪️tests/📡️codec-authority/🦀️.rs | 9 | 32, 37, 45, 48, 58, 60, 76 |
| 🔌️io/🧪️tests/🧭️producer-authority/🦀️.rs | 2 | 9, 10 |
| ⏳️async/🧪️tests/🧭️producer-authority/🦀️.rs | 2 | 19, 29 |

## Current Primary Authority

`🧰️framework/🔨️modules/🎒️pack/⚠️error/🦀️.rs:9–24` owns semio_framework_pack_error::PackError but currently declares tuple LimitExceeded(&str), Io(String), untyped RetainedMalformed/Malformed, and ValueRefusal/TextRefusal. It has into_value_error29, not refusal_kind/cause_kind, PackCauseKind/PackRetryDisposition or RetainedAllocation. Its TypeScript sibling6–19 still has old-shaped PackErrorData as well. The newer closed neutral cause schemas/tests exist, but their presence is not the newer production API.

Actual replication `⚙️codec/🦀️.rs:11–27` separately defines protocol::PackError, including Schema(String), with old tuple/error shapes. Actual Pack package root14/44 glob reexports protocol::codec, while format root14 and async/IO/HTTP consequently use that crate::PackError. Pack manifest has pack-error ONLY under dev-dependencies33, so changing a test import cannot change production's returned error authority. The explicit producer-authority test imports semio_framework_pack_error; other codec/I/O tests call protocol-returning production APIs. Two nominal types and two incomplete old contracts coexist.

## Minimal Coherent Work, Not a Namespace Shortcut

No current structured Rust authority exists for a safe before/after import-only capsule. The absent retry/cause/refusal/retained-allocation API must be implemented at its actual canonical Pack Error schema owner and its typed producer joins, following the already authored neutral tests. Then replication's actual primitive codec error route and the actual Pack package exports must select that ONE authority through real direct dependencies. Do not reexport a duplicate local enum as compatibility, add old aliases, drop retry/allocated-byte expectations, replace typed kinds with prose classifiers, or rewrite new laws against old tuple fields.

The nineteen producer-authority diagnostics cover both new variant fields and missing observation methods; nine codec-authority cover actual producer refusal methods/typed allocation state; HTTP five cover new retry authority and old Io(String), IO/async two each cover current returned protocol error refusal methods. Thus they are all part of this missing coherent typed feature, not independent edits on37 unrelated bodies. Existing actual ValueError and TextError are canonical complete owners; preserve their message/metadata ownership transfers. Production migrations need typed cause at each actual producer, not a late into_value_error string mapping.

Concurrent canonical Value/ordered-retirement diagnostics earlier in the log are warning output or separate earlier source joins, not part of this measured37 error roster. Count physical High owns those regions. This audit does not mount or overlap them. Source presence and parser receipts do not prove the missing Pack production authority.
