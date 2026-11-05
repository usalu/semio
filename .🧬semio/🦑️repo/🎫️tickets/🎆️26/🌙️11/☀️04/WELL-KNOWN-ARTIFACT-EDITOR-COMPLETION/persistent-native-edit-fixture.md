# Persistent Native Edit Fixture

The one-item store testkit now provides `PagedEditFixture<S,M,E>`. Its constructor receives an existing original caller, decoder, admitted before/mutation sources, mutable post and edit. These owners persist throughout the case. Each advance uses the same original receiving ledger and settles actual retained effects; canonical semantic work is paid once from the caller's original fuel. A narrower zero-work loan pauses the same authority.

Owning outcomes remain in a persistent pending slot until successful settlement. Explicit paid cleanup covers inverse/error outputs, the edit, original sources, post, decoder and caller. A refused cleanup retains caller-owned retirement cursors. Physical heap allocation/release must match the genuine caller receipt, including semantic errors.

## Ports

- Mesh and Brep preparation tests now use the persistent fixture, with normal scanning, absent-target error, denied grants, work reset and zero-work-loan cases.
- ZIP preparation tests no longer create fresh controls/sources per turn. Rename/comment, stale/colliding rename and denial cases retain one source pair.
- DOCX adds a real minimal-document SetRunText case, checking reserve work 2, apply work 3, zero-work pause and denial reset.
- All four owning Cargo manifests explicitly enable the first-party one-item/value testkit dependencies. Brep preparation belongs to its IO crate; DOCX preparation belongs to the base crate.

## Verification

The schema-first neutral corpus is checked with Ajv and an independent SQLite work oracle: **2 tests, 18 assertions pass**. This began with a failing source law before the fixture implementation. Final Rust syntax-only parsing passes for the helper and all four preparation files (**5/5**). These checks do not establish Rust semantic correctness or physical execution.

Authoritative Source closure checks passed before the final loop-bound spelling changes:

| Source | Files | Bytes | Active lines |
|---|---:|---:|---:|
| One-item testkit | 3 | 31399 | 412 |
| Mesh preparation owner | 8 | 59343 | 937 |
| Brep IO owner | 28 | 207737 | 2939 |
| ZIP editor owner | 18 | 142494 | 2321 |
| DOCX base owner | 25 | 168682 | 2650 |

Explicit cleanup loops allow up to 1,000,000 attempts while preserving unchanged original stock and fuel. This is a test loop bound, not additional authority.

## Native Gate

The shared native driver has these exact finite jobs queued behind the corrected core:

| Owning working directory | Package | Filter |
|---|---|---|
| Semio artifact root | semio-s-artifact-stdio-semio-mesh-editor | --lib preparation::tests |
| Semio artifact root | semio-s-artifact-stdio-semio-brep-io | --lib preparation::tests |
| ZIP artifact root | semio-s-artifact-stdio-zip-editor | --lib preparation::tests |
| DOCX artifact root | semio-s-artifact-stdio-docx-base | --lib native_xml_edit_uses_one_persistent_original_caller |

No native execution pass is claimed. Mesh/Brep absent-target production paths currently construct dynamic owning errors; the strict physical comparison may expose unreported diagnostic allocation. That must be fixed in the production owner if confirmed, rather than hidden by this fixture.

The parent's new third pending-page native case remains under its ownership. Its normal library closure (17 files, 239707 bytes, 2544 active lines) and pending integration target (2 files, 34468 bytes, 127 active lines) remain within current Source bounds. No FullOp lifecycle or publication pump was changed by this fixture task.
