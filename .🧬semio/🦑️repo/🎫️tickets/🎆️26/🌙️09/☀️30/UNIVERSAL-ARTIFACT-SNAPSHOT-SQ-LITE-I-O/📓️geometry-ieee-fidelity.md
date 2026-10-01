# Geometry IEEE Fidelity Repair

Native STL normals/vertices and OBJ vertex/texture/normal fields are unrestricted f64, including optional OBJ weights. PLY scalar/list Float and Double variants are unrestricted f32/f64. The initial relational providers refuse nonfinite values and have REAL-only columns; SQLite also normalizes signed zero under REAL affinity. These are actual domain fidelity gaps.

A direct Bun/JSC DataView oracle set the binary64 words 7ff0000000000001, 7ff8123456789abc and fff8123456789abc, read them as JavaScript numbers, then wrote them back. Every result was 7ff8000000000000. Binary32 words 7f800001, 7fc12345 and ffc12345 likewise became 7fc00000. Each run emitted a [DEBUG] line with input/output hex. Number-only owned fields therefore cannot preserve exact native NaN payloads, even before SQLite.

The parent authorized explicit bit-backed owned Binary64/Binary32 geometry scalar types and all required callers. No number-or-wrapper compatibility union will remain. Authored SQL retains each actual numeric field as a queryable REAL, with individually named IEEE integer and class companions. Primitive conversion operates only at literal declared column positions and native widths; no object/schema reflection or opaque snapshot payload is used. Language-neutral hexadecimal vectors will drive both implementations and independent Bun SQLite reserialization/edit/refusal tests.

## Implemented Owned Domain

STL normals and vertices, OBJ vertices/texture coordinates/normals/optional weights, and PLY Float/Double scalar and list values now preserve exact IEEE identities in both implementations. Rust retains its native f64/f32 fields. TypeScript owns `Binary64 { bits: bigint }` and `Binary32 { bits: number }`, requiring unsigned words of exactly the native width. Ordinary numeric constructors and numeric value accessors are explicit; exact NaN input uses its owned word. Snapshot, artifact and diff parsers were coherently updated where those geometry fields are actually consumed. OBJ source positions still retain their full unsigned64 domain through two queryable unsigned32 INTEGER columns.

Each affected SQL field retains its original queryable REAL position and gains individually named `*_ieee754_bits INTEGER` and `*_numeric_class TEXT` columns. Signed INTEGER64 stores the complete binary64 word by reinterpretation; unsigned INTEGER32 stores binary32. Classes are `finite`, `positiveInfinity`, `negativeInfinity` and `nan`. NaN uses NULL only in its numeric query column, preserving the exact payload and sign in the named bits column. Optional absence requires all three cells NULL and therefore remains distinguishable from a present NaN. Readers reject disagreement among query value, class and bits. SQLite zero normalization is allowed in the numeric query while exact signed-zero identity restores from the word. No opaque carrier or inferred model/schema traversal was added.

Native providers use the existing verified `Projection`, `FloatColumn`, `FloatRow` and `insert_ieee754` primitives. TypeScript's new primitive performs conversions only at caller-authored literal field positions and widths. Geometry row counts and aggregate logical cell bytes are preflighted before entity allocation; numeric query, word and class bytes are included, with bounded cancellation scans and existing semantic progress phases retained. Physical TypeScript SQLite now permits REAL infinities and rejects NaN REAL values, matching SQLite and native behavior.

All three native and TypeScript providers enforce exact four-argument owned dialect validation, document schema identity and cancellation: `s.stdio.stl@ascii/*`, `s.stdio.obj@3.0/geometry`, and `s.stdio.ply@1.0/*`. Native artifact capabilities remain explicitly declared.

## Test-First and Independent Evidence

Each family's neutral IEEE fixture contains positive/negative zero, an ordinary finite value, the smallest subnormal, both infinities, signaling NaN, quiet NaN with a nontrivial payload, and negative NaN with that payload. Both implementations consume the same hexadecimal vectors and exact coordinates. PLY covers binary32 and binary64 scalar and list variants independently.

New native regressions first failed against the old schemas because the independently queried named IEEE columns did not exist. The primitive TypeScript suite first failed for its missing implementation and then exposed the physical engine's infinity rejection before that guard was repaired. Owned dialect regressions also began with the missing TypeScript validator exports.

Independent Bun SQLite opens authored files, verifies `integrity_check` and foreign keys, queries named bits/classes, serializes the file itself and feeds that result to the owned reader. Exact native `to_bits()` and TypeScript owned words are compared after reconstruction. Independent SQL edits include coherent numeric/bit edits and negative-zero restoration. Valid SQLite files with incoherent NaN class/zero bits are rejected by semantic readers. These laws also exercise optional presence, entity ordering, PLY width/range constraints, ownership and cancellation.

The final direct current-source Bun run passed **27 tests with 360 assertions across five files**: shared IEEE primitive, STL SQLite, OBJ SQLite, PLY SQLite and OBJ diff parsers. Runtime tests emit `[DEBUG]` family identity-law evidence. A cleanup rename temporarily introduced an OBJ helper/local-variable collision; this direct run caught eight failures, the local binding was fixed, and the full fresh gate then passed. A cached Nx rerun had reused earlier outputs, so final package checks were explicitly restarted with `--skip-nx-cache`.

## Owned Paths

Shared IEEE primitive, neutral vectors and independent tests: `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/{🟦️.ts,🧫️fixtures/🔣️.json,🧪️tests/🟦️.ts}`. Minimal public exports live in `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`; the physical REAL guard changes are in `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts`. The parent registered the primitive suite in the framework source target and reported its fresh dependency-inclusive gate passing 28 tests/192 assertions.

Artifact roots under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`:

- `🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/`
- `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/`
- `🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/`

For each root, owned changes are `📸️snapshot/🟦️.ts`, `📸️snapshot/🪶️sqlite/{🗄️.sql,🦀️.rs,🟦️.ts}`, `📸️snapshot/🧪️tests/🪶️sqlite/{🦀️.rs,🟦️.ts}` and new `📸️snapshot/🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json`. STL and OBJ additionally update artifact `🟦️.ts`, diff `🔺️diff/🟦️.ts`, and OBJ's existing diff-parser test. Family TypeScript package `📦️packages/🟦️typescript/📜️script.ts` now includes its owned SQLite suite; OBJ retains its diff-parser suite. No Rust manifest, IO registration, root task/launch files, AGENTS file, git state or worktree was modified for this repair.

## Completed Checks

- Owned TypeScript package build/typecheck/test script seams passed for all three families: STL 7 tests/91 assertions; OBJ SQLite 9/121 plus diff 2/17; PLY 7/100. Each build emitted three outputs and one public export.
- Fresh registered native `bun nx run-many --target=test --projects=@semio-tech/stdio-stl-rs,@semio-tech/stdio-obj-rs,@semio-tech/stdio-ply-rs --parallel=3 -- --lib sqlite_snapshot_` passed all three packages and their three generators with zero cache hits in 13.8 seconds, including malformed companion and exact aggregate-budget changes.
- Per-package native runs passed STL 4 tests (Nextest `a79b3d61-06a6-460c-81eb-5db78f16a8f5`), OBJ 6 (`75e5f384-1eee-4e3d-92e8-b6a49cf3aaa9`) and PLY 5 (`309bcb42-7afc-4789-b8da-8243c2a746d0`). Subsequent combined registered gate includes the newly extended malformed-companion laws.
- `bun nx run @semio-tech/framework:typecheck` passed fresh, zero cache hits, in 57.8 seconds, covering public Binary32/Binary64 exports.
- Fresh direct five-suite Bun gate passed 27/360 after the cleanup collision fix.
- Final current-source `bun nx run-many --targets=check,test --projects=@semio-tech/stdio-stl,@semio-tech/stdio-obj,@semio-tech/stdio-ply --parallel=3 --skip-nx-cache` passed all six targets in 35.7 seconds.
- Final native cleanup rerun passed all three native package tests with only the three generator dependencies cached, in 10 minutes 23 seconds including concurrent Cargo build waiting. Every owned native provider was freshly built and exercised after final cleanup; no native package test output was reused from cache.

Generated logs stay under this ticket's `🗑️generated` during the active overall goal. This report is retained. The geometry repair does not claim completion of the entire 149-dialect goal; the parent assigned the full DWG TypeScript mirror next.
# Erased Snapshot Fidelity And Independent PLY Declaration State

An actual registered STL erased Binary/Text regression confirmed native file lowering discards the owned schema, even for zero-valued coordinates. Nextest `70356339-9e68-469f-9407-364de0f15872`: one law failed, fifty filtered (0.012 seconds assertions; 5.9 seconds Nx). The new language-neutral word-vector laws also require exact NaN words, signed zero, infinities, optional fields and OBJ unsigned64 source positions. Explicit typed STL point/facet, OBJ existing-record and PLY typed scalar/list/property snapshot codecs have been authored adjacent to their models; wiring and fresh verification remain required.

PLY's stale-count/full-count-kind law first failed at the source projection count equality guard: seven laws passed and the new law failed, 100 assertions, 2.22 seconds. The handcrafted schema now owns `declared_count_high` / `declared_count_low` unsigned32 columns, independent from occurrence rows, and admits all eight native count scalar kinds. Canonical count types are Rust `u64` and TypeScript `bigint`, with exact strict parsing, unsigned64 schema bounds, explicit reconstruction word checks and no compatibility union. Existing native file decoder/diff consumers were updated to the owned type. List occurrence length is independent of the declared count scalar width; normal file-format rules remain the file engine's responsibility.

The source SQL suite passed nine laws / 261 assertions in 1.176 seconds, including 32 shared declaration-state vectors and independent SQLite edits to maximum unsigned64 count and floating count metadata. The registered uncached PLY package source gate passed those nine laws and strict package checking in 6.5 seconds. This predates the new public facade regression suite. A direct Bun public facade red then demonstrated the package exported only `definition` and omitted its authored SQLite capability. Minimal explicit STL/OBJ/PLY facade bindings/types and public owner probes were added; fresh registered verification remains pending an unrelated Nx graph prerequisite (`resolvePlaygroundDistDir` missing, writer test host project absent, frame-worker generated output producer absent).
