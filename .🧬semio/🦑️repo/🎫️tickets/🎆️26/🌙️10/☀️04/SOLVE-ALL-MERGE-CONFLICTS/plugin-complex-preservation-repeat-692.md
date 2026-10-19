# Plugin Complex Preservation — Repeat 692

Resolved assigned conflict indices **71, 73 and 75** against immutable base `36fbe9da786b71ec7b6946a1fc1cbb4e9d4d25ec`, HEAD `b84bf526c7ba26f99c4054b47da1c1a4d5064e3d` and incoming stash `890df375465b64231521ef2c74d80c91efc7a2b3`. No Git state was modified.

## Disposition

- **71 — Reserved reactor job:** retained HEAD’s public fields, public registry drivers, extracted crate qualifications and single bounded-job implementation. Preserved incoming `OriginalSourceBinding`, exact source/clock validation, original recipient admission, yielding/fuel accounting, original construction, original retirement and incoming test mount. The factory now obtains its original binding before original admission.
- **73 — Reactor:** retained HEAD’s dissolution of the local turn module. The existing wp4 facade carries incoming `pub(crate)` settlement reexport intent through `semio_framework_plugin_reactor_turn`. No turn implementation was remounted.
- **75 — Full operation:** retained HEAD’s public fields, concrete owner qualifications and split helper custody. Preserved incoming original producer binding/request custody and retirement, original page presentation/physical closure, diagnostic ownership/ACK/retirement changes and the producer include. Restored imports of the existing bounded ownership admission/fault helpers required by these incoming uses. Qualified new request and producer paths to their extracted owners.
- Both changed host continuation helpers remain solely in the existing wp3 retained-command test host. Their authored original receiving job, original result-page presentation/close and deferred ACK behavior were ported there with incoming documentation. No helpers were revived in the full-operation owner.

## Incoming Hunk Audit

All **38** base-to-incoming edit opcodes were checked against their preserved destination after explicit owner/visibility qualification mapping: **5** reserved-job opcodes, **1** reactor reexport opcode and **32** full-operation opcodes. Full-operation helper changes map to the wp3 test host; the reactor addition maps to the wp4 facade; all other edits map to their existing source owner. Deletion-only opcode removes the old immediate result-page release/reset.

## Inherited Missing Definitions

Whole-tree searches of the immutable incoming stash, using `git grep -n <symbol> 890df375465b64231521ef2c74d80c91efc7a2b3` without a path restriction, prove:

- `settle_failed_original_publication` has exactly one occurrence: reactor root line 468, its authored reexport. There is no authored definition anywhere in the stash. HEAD has zero occurrences. The reexport is preserved through the current extracted turn owner; its missing definition remains an inherited source-only limitation.
- `admit_original_reserved_producer` has exactly one occurrence: full-operation producer leaf line 21, its authored call. There is no authored definition anywhere in the stash. The call is preserved through HEAD’s extracted reserved-job owner. Its missing definition remains an inherited source-only limitation.

No semantic helper body was invented for either missing definition. These are preservation findings, not compiler fixes.

## Files Updated

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/⚛️wp4-reactor/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🏭️producer/🦀️.rs`

## Static Verification

Fresh original-byte guards preceded each atomic replacement. All six final file hashes match the write ledger. No conflict placeholders or merge markers remain in these six files. All nine pre-existing reserved-job fields stay public; both moved host helper names occur exactly once; the reactor has no remounted local turn module; the full-operation root has no moved host helper definitions; all six paths are within 256 characters.

No builds, compilers, tests, dependency resolution, product runtime, installs or modifying Git commands were run. Source checks do not establish runtime or compilation success.

Generated evidence (temporary): `plugin-complex-writes-repeat-692.json` and `plugin-complex-hunk-audit-repeat-692.json` under the ticket’s `🗑️generated`.
