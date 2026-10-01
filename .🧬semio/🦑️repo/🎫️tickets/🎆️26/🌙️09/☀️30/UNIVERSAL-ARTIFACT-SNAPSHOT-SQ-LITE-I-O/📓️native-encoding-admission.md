# Native Encoding Admission

The actual IFC owner exposed an expansion hazard: its exact Part21Decimal stores a u32 scale in SQLite, while native Text printing expands the decimal to its full digit count. A small semantic database can therefore request gigabytes of native output. Aggregate database bounds checked after encoding do not prevent this allocation.

The canonical ArtifactSqliteSnapshot now has `preflight_sqlite_snapshot_encoding(&self, SnapshotEncoding, &mut SqliteSnapshotControl) -> Result<(), String>`. The default checkpoints cancellation and rejects the missing owner guard. The default erased import invokes this method after semantic reconstruction and before both Text and Binary encoders. Owned typed SQLite import does not encode a native payload and remains independent of this requirement.

The public `sqlite_snapshot::artifact::NativeEncodingBound` helper only accumulates explicit owner estimates. It performs checked addition/multiplication, checks aggregate max_value_bytes and max_file_bytes, and calls EncodeNative checkpoints before traversal, every 256 estimates, before a >64KiB admitted estimate, and at completion. It neither walks snapshots nor infers a schema or field mapping.

The schema-first neutral admission fixture stores only a typed output byte count. Native seam laws invoke the actual erased provider import for both encodings, checking strict default rejection, expansion rejection, cancellation before the encoder, and bounded success. The independent Bun SQLite oracle verifies integrity and queries the typed count. The fixture encoders count invocations, so rejection cannot silently happen after printing. No runtime green is claimed until the registered target executes these laws.

PDF1.4 explicitly counts its borrowed schema/pages/text and primitive page geometry. PDF1.7 reuses its owner-authored complete semantic projection before creating the tagged native value tree. Each semantic entity reserves conservative wrapper/key allocation; explicit storage-class values reserve text escaping, octet value allocation, and maximum numeric formatting. The SQLite tables remain independently authored and unchanged.

## Verification

- Actual registered kernel admission law passed for both encodings: missing-owner, expansion-budget and cancellation cases rejected before either encoder, while the bounded case encoded successfully. Nextest run `a6f1c625-e70f-4c32-affd-ab05235a0c2c` also reproduced the adjacent allocation defect: a 131072-byte payload nested inside 32 objects allocated 5474049 bytes during printing, exceeding the 2097152-byte fixture allowance.
- The shared DSL printer now sorts borrowed object entries instead of cloning complete descendant trees. The neutral regression also checks stable binary key order against Bun SQLite, exact u64/signaling-NaN parsing, and malformed-text rejection. The first green attempt stopped before compilation on the concurrent Nx `resolvePlaygroundDistDir is not defined` graph failure; no repaired runtime green is claimed yet.
- Initial registered framework I/O attempt stopped before compilation on a concurrently removed OS flow core Cargo manifest. No runtime red or green was observed.
- Initial PDF erased retry stopped before compilation because Nextest could not resolve the fundamental profile. Root owns the explicit Nextest config path repair.
- Native owner guards are required to rerun actual erased Binary/Text laws. Earlier erased green results are historical evidence from before strict admission enforcement, and do not prove current guarded behavior.
- Shared DSL regression is now GREEN: all 251 native `dsl::` laws passed in Nextest `c9be70cc-b73d-46f0-942c-4ea3ffc94c64`, including the neutral allocation and independent SQLite key ordering law, in a 58.9-second uncached owner task. The earlier patch accidentally targeted the adjacent Map arm; the actual Object arm is now corrected and the unrelated Map arm restored.
- Latest shared I/O lane executed both kernel admission/allocation laws successfully. Its subsequent plugin libtest compilation failed only because the concurrent history-label-reload caller passed an already awaited manifest to the canonical Future-accepting helper. That one caller is repaired; a new registered I/O gate is running before any plugin-green claim.

## Remaining Owner Checklist

- Root: MP4, AVI, MP3, EPW, WAV, JPG, GIF bounds and their actual erased reruns.
- TypeScript codec worker: native LAS, ZIP, DWG, STL, OBJ, PLY bounds and actual erased reruns.
- Rust codec worker: IFC decimal expansion, Semio, JSON/XML/HTML/SVG/TIFF bounds and actual erased reruns.
- I/O owner: PDF1.4/PDF1.7, Note/DXF, core fixture bounds and actual erased reruns.
- Remaining owner providers require explicit authored estimates; no missing/default guard is accepted toward universal coverage.

## Owned Files

- Canonical store `🦀️.rs`: required strict owner seam and pre-encoding invocation.
- Shared SQLite artifact `🦀️.rs`: arithmetic-only NativeEncodingBound.
- Store snapshot-capability/native-encoding: handcrafted SQL, neutral JSON schema/fixture, real erased Binary/Text and Bun oracle tests.
- Store snapshot-capability tests: native-encoding test mount.
- Store unit DemoSnapshot: explicit constant bound over its nullable i32 record.
- Framework Rust task router: the existing SQLite I/O lane also runs kernel admission laws; the existing launch command already covers this lane.

## Final Shared Runtime Gate

The registered existing framework SQLite I/O lane passed both kernel admission/allocation laws and all four plugin registration/I/O laws, uncached. Kernel run `fc335130-94bf-4d5b-8031-1fe7c46938b5`; plugin run `db3ed945-a2a8-4105-a51b-898a5c924fce`. Actual assertions took 0.122 and 0.041 seconds; aggregate 4m45 included Cargo preparation/compilation. The concurrent history-label-reload manifest caller is corrected to pass the canonical Future without prematurely awaiting it. Explicit fixture bounds are authored in plugin app-declarations-fixture. This verifies the current shared seam, while individual owner guards still need their actual erased laws.

Parent reassigned JPG/GIF guard, erased-model and public/native asset audits to this I/O owner after PDF/DXF/Note gates. Root retains MP3/MP4/WAV/EPW/AVI/BCF/CommonMark. No missing guard is accepted as universal coverage.
