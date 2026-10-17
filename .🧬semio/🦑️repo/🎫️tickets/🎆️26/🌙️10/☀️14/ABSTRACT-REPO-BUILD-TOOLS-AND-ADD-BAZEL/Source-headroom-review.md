# Source Ownership Headroom Review

Source-only planning from historical complete Bazel36354 (512files) and tooling37335 (172/253/133). These are not current captures after later Native changes. No new scanner, SDK, Native or filesystem asset operation was run. Existing fourteen-contract ownership work is excluded. Current public filesystem/routing facades are intentional package boundaries; deleting them solely to save a count would weaken taxonomy and is not recommended.

The following are genuine same-component document folds; runtime subdomain leaves remain. Each moves every semantic definition/constraint into one existing component root and removes retired documents without aliases. Savings below are historical-file arithmetic only, not current closure acceptance.

## Owned subprocess wire

Potential historical Bazel reduction: 4 documents. stdout/output actual owner imports lines8–9; stream/consumer/exit remain wire subcontracts, not discarded functionality. Canonical destination retains its existing root behavior plus explicitly named definitions; no wrapper files.

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧬️schema/📤️stdout.json` — fold into component definitions; historical tooling roles reader,runtime,compiler.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧬️schema/📤️stream.json` — fold into component definitions; historical tooling roles none.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧬️schema/📥️consumer.json` — fold into component definitions; historical tooling roles none.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧬️schema/📪️exit.json` — fold into component definitions; historical tooling roles none.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧬️schema/📬️output.json` — destination; historical tooling roles reader,runtime,compiler.

## Resource lease authority

Potential historical Bazel reduction: 5 documents. actual leases owner imports retirement/ticket/custody/retention lines1,11–13; preserve proof/writer and GC semantics as named component definitions. Canonical destination retains its existing root behavior plus explicitly named definitions; no wrapper files.

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/♻️retirement.json` — fold into component definitions; historical tooling roles reader,runtime,compiler.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/🎟️custody.json` — fold into component definitions; historical tooling roles reader,runtime,compiler.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/🎟️proof.json` — fold into component definitions; historical tooling roles none.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/🎟️retention.json` — fold into component definitions; historical tooling roles reader,runtime,compiler.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/🎟️writer.json` — fold into component definitions; historical tooling roles none.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/📜️ticket.json` — destination; historical tooling roles reader,runtime,compiler.

## Build input storage

Potential historical Bazel reduction: 3 documents. actual storage mjs imports ledger/quota/dataset lines10–13; nested ledger runtime and tests select same canonical root. Canonical destination retains its existing root behavior plus explicitly named definitions; no wrapper files.

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/💾️quota.json` — fold into component definitions; historical tooling roles runtime.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/📒️ledger.json` — fold into component definitions; historical tooling roles runtime.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/📦️dataset.json` — fold into component definitions; historical tooling roles runtime.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json` — destination; historical tooling roles runtime.

## Dependency storage

Potential historical Bazel reduction: 2 documents. actual owner imports install/bindings lines17–18; preserve InstallManifest/Mutation/DelayedCustody/ReadProtocol/ProjectionPolicy. Canonical destination retains its existing root behavior plus explicitly named definitions; no wrapper files.

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️bindings.json` — fold into component definitions; historical tooling roles none.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json` — fold into component definitions; historical tooling roles none.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/🔣️.json` — destination; historical tooling roles none.

## Required Proof And Risks

Prefer the build-input storage fold for runtime headroom (four existing runtime documents→one), then same-component lease authority (four direct runtime documents→one, including queue protocol refs). Owned subprocess wire saves four historical Bazel docs but only one direct tooling doc with chosen destination; dependency storage helps backend count but none of the recorded tooling roles. Do not sum these as guaranteed new capacity: sibling component census and import topology can change actual captures. Future Windows X/H/C and schema co-location are separate additions.

Preserve every root and nested JSON pointer with recursive prefix rewrites; validators selecting nested schemas must retain the full canonical root. Ticket schema has an actual ResourceQueueTicketV1 id; change its real owner/id deliberately, remove retired aliases and update all readers, MJS/TS declarations, tests/corpora and published SDK manifests together. Handalign exact reader declarations after actual complete capture. Before edits retain normalized pre-move keyword/ref hashes; schema-first Source RED then actual production-validator/Ajv/SQLite equality and malformed/negative corpus prove all constraints unchanged. No format fallback, scanner exclusion, cap widening, copied Source or generic monolithic contract for unrelated domains.

These folds preserve DDD runtime ownership but do not imply all schemas are redundant concepts; only their separate documents are candidate fragmentation. Facade/document deletion without owner analysis is rejected. Scope must be coordinated with current shared Native holds; fresh unchanged-budget full tooling/Bazel captures and protected currentFD hashes, not this design arithmetic, determine acceptance.

Build-input concrete fold plan reviewed: existing2020 namespace enum remains root, complete quota/ledger/dataset definitions and recursive local refs retained, validation uses full canonical root without intersecting object values with namespace enum. Runtime/revision rosters are handaligned canonical once, all custom annotations/keywords and array order retained in normalized proof. No guessed reader change; actual full capture decides. No conceptual blocker for separately released schema-first Source implementation, no current capacity or runtime acceptance.
