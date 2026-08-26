# S-Misplaced Ticket Roots

## Outcome

The ten framework path-budget findings reduce to six `CACHEDIR.TAG` leaves and four derived directories. Each leaf is beneath an accidentally nested `.🧬semio` ticket root inside a Rust package tree. No production implementation or unique build result exists in these roots.

All six leaves have the exact SHA-256 `6d9d1d216e0f83abc5e5662ca62c92b4f23009466b54fa27321a69acdb778bb2`, the standard Cargo cache-directory marker. Stripping the package prefix maps them to the authoritative ticket `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`, where all six destinations are currently unoccupied and range from 171 to 180 UTF-8 bytes.

Three copies map to the same `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-os-errors/CACHEDIR.TAG` destination. Because their content, role, mode, and ticket identity are identical, the transaction must retain one exact marker at that ticket destination and record the other two as redundant preimages. The other three markers have unique ticket destinations.

## Deterministic rule

An embedded `.🧬semio/🦑️repo/🎫️tickets/...` root outside the registered repository metadata root is a misplaced ticket-root violation. The normalizer may relocate only when:

- the suffix is a valid registered ticket identity;
- the canonical root is the repository metadata ticket root;
- every leaf has an exact preimage and is ticket evidence or a registered cache marker;
- destination occupancy is absent or byte/mode identical;
- any many-to-one group is byte/mode identical and recorded as redundant evidence;
- no reference resolves into the misplaced root.

Unknown content, conflicting occupancy, or a non-ticket suffix must block rather than be deleted.

## Acceptance

- zero nested `.🧬semio` roots under framework/package trees;
- four canonical ticket cache markers installed, with two redundant copies accounted for;
- zero path-budget findings from this class;
- package discovery never descends into an embedded ticket root;
- apply/rollback/empty-second-plan cover the relocations and redundant preimages.

The census was read-only and excluded Compose and `temp/compose`.
