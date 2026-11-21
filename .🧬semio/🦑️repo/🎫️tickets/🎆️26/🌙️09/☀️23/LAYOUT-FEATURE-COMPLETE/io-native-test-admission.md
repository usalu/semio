# Native Layout IO Test Admission

The repository source scanner reproduced the native library-test refusal at 54 paths and 404,544 bytes. The hard limits are 32 paths and 262,144 bytes per Rust target. The existing language-neutral fixtures and independent PDF, STEP, ZIP, XML, Three.js, and Sharp expectations stay in place.

The six files moving to the `interchange` integration target keep their current paths.

- The interchange suite and its nested PDF oracle.
- Drawing lift.
- External import source planning.
- Cold vector export.
- JSON serialization.

The interchange suite gets its own typed test setup using the model's public empty snapshot constructor. It supplies the same document schema, page and layers, single spread, and seeded text. The four former private dependencies only build these fixtures. All import, projection, export and resumable execution assertions already use public Layout IO entry points. The Drawing, source-plan, vector and JSON laws need explicit imports of their existing public API. STEP geometry, STEP background geometry, PDF marked text and PDF path laws continue as the four private unit modules.

The existing Framework fixture reader can load the same committed fixtures at runtime. Resolving each original source-relative fixture and its proposed manifest-relative path led to the same file, including the binary PDF and STEP witnesses. The fixture-reader package is already present in the Layout workspace lock. Only the Layout IO dev-dependency edge needs to be added. The Bun TOML parser independently confirmed the draft declares both integration targets and the fixture reader is locked as a test-only dependency.

The repository scanner ran against the exact 15-file draft in memory while the authentic browser guest held the shared source hash.

| Cargo target | Source paths | Bytes | Active lines |
| --- | ---: | ---: | ---: |
| Production library | 28 | 229,857 | 2,108 |
| Test library | 32 | 236,531 | 2,135 |
| Drawing birth | 2 | 31,911 | 195 |
| Interchange | 6 | 70,581 | 641 |

The draft keeps five private laws in the library, 35 public laws in Interchange, and three laws in Drawing birth. The scanner verified each moved source belongs exclusively to Interchange. Native execution remains pending until the shared source and compiler lane are released.

## Live Patch Recheck

The saved `🗑️generated/io-native-test-admission.patch` passes the repository Git tool's read-only apply check for all 15 paths. A separate Bun diff parser applied the same hunks to copies in memory. The resulting production library, test library, Drawing birth and Interchange closures reproduced all four path, byte and active-line totals above. The test library is exactly at the 32-file cap.

All 17 unique migrated fixture references still resolve to their original files, including the prior manifest-relative vector export fixture. The live fixture-reader manifest still names the intended crate. Independent TOML parsing confirmed that the patch adds its dev dependency and adds the corresponding edge to the `semio-s-artifact-layout-layout-io` package in the current Layout lockfile. The pinned Rust grammar accepted all 13 patched Rust files. The live snapshot constructor and the existing public I/O exports used by the moved tests are still present. Native compilation and assertions were outside this read-only recheck.

The live Layout lockfile had received newer workspace changes. Those changes leave the intended dependency edge and all hunks applicable. For application after the publishing owners finish, re-read the 15 files and repeat the read-only Git check and in-memory overlay validation. Use repository-mediated file edits against those live bytes. If a hunk stops matching, preserve the file and transplant only its intended change, adding the fixture dependency to the exact package row or moving the exact test declaration and equivalent fixture reference. Re-run all four closure checks on that result before native verification.

## macOS and Windows Recheck

The current 15-file patch passes `repo tools exec --light git -- apply --check`. The only shifted hunk starts at live line 12881 in the Layout workspace lockfile, four lines after its saved header. Its complete context still matches, so the patch needs no content changes.

The [read-only source verifier](io-native-source-check.ts) applies each hunk to live file contents in memory. When a header's line no longer matches, it locates the unique exact context after the previous hunk. Run it through `repo tools exec --light bun -- <verifier> <Layout IO directory> draft macos` and repeat with `draft windows` for the Windows MSVC configuration. Both runs reproduced all four closure totals in the table and validated the fixture reader's test-only lockfile edge.

All 17 unique fixture migrations resolve to their original files. The fixture reader joins each reference against the test crate's manifest directory. All 33 edited or referenced paths resolved to their canonical files. The longest repository-relative path is the existing JSON serializer unit test at 175 UTF-16 code units and 251 UTF-8 bytes. Under `C:\src\semio` it is 188 UTF-16 code units. A checkout root of up to 80 UTF-16 code units keeps that path within 256. The patch preserves every existing path. These checks used the Windows source configuration and path rules on macOS. Native test execution remains pending.

Repeat the read-only checks just before applying the split through guarded file writes against the same live contents.

## Applied Source Verification

After applying the split, the macOS and Windows MSVC source scans both reproduced the four closure totals above. Bun independently parsed the applied Cargo target declarations and the locked fixture-reader dependency. The library test closure remains at the 32-file limit.

The first locked native macOS run passed source admission and compiled into the Layout dependency graph. It exited 101 at `semio-plugin-document-media` with E0599 for `BorrowedProjectedPackCursor::next_intrinsic_advance_work_demand`. The run captured sources before the synchronized 15:02 UTC update to both the consumer and provider. The live consumer's `semio_os_kernel::os_pack::record` export resolves to the same provider that now publicly defines that method. A fresh managed run reuses the warmed shared target store against the synchronized sources.

The fresh run compiled the document-media dependency successfully. Its first attempt later encountered a missing `ActionKind` import in `semio-plugin-instance-commands`. The live command source acquired its explicit `semio_framework::ActionKind` import on 16:05 UTC, after that attempt captured sources. The framework export already resolves to the manifest action enum. The managed command started a new source-admission child under the same native test owner.

The automatic retry compiled instance commands successfully, then reached both Layout integration targets. Rust reported missing public drawing imports and two test-only attempts to deserialize the typed Drawing snapshot with Serde. The two affected integration files now import the existing public types and construct the same text and image nodes and pixel asset directly on the original Drawing snapshot. The library source totals are unchanged. The updated Drawing birth target is 2 files and 31,933 bytes. Interchange is 6 files, 70,943 bytes, and 655 active lines. The applied macOS scanner and scoped diff check passed again. A new locked native run is validating the repaired targets.

The applied Windows MSVC source scan reproduced all four updated macOS closure totals. At `C:\src\semio`, the two changed test paths use 132 and 152 UTF-16 units, within the 256-unit cap. Their repository-relative byte lengths are 171 and 199.
