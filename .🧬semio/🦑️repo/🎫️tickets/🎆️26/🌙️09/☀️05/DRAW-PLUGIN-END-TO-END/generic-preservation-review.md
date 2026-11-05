# Generic Root-Family Preservation Review

Read-only current POSIX/Windows descriptor implementation, schema and focused Process law inspection. No tests/builds/queues/source edits.

## Concrete Finding

POSIX original-root preservationPathUnits seed uses root.replace(/[\\/]$/u, empty). POSIX permits a literal trailing backslash filename character, and current Generic root validation only requires absolute normalized path/length; it does not reject that root. Removing the backslash undercounts real full path by one, potentially admitting257units under256. POSIX must remove only its actual slash separator or explicitly reject such roots. Runtime/root notified. Windows slash/backslash root handling is separately appropriate.

## Authority And Custody

Private optional preservationPathUnits is seeded only at original root and propagated only through authenticated withDirectoryPath endpoint construction. Ordinary withDirectory, creation and capsule child calls omit it. Ordinary-child.withDirectoryPath also propagates undefined, so it cannot escalate. The value is lexical private runtime metadata, not caller DATA/Native authority. Path proof maps and first-frame/later-parent lineage checks remain; copied/expired receiver live checks still execute before path operations.

Capture/publish/retain still require exact expectation proof, parent identity, bounded UTF8 intent, current live frame, expectation/capsule bounds and existing tuple/record/payload checks. Capsule lifecycle, actual close/drain behavior and identity/roster guards were not removed by inspected diff. POSIX/Windows full path sums use JavaScript UTF16 name.length and inherited root units; nonBMP contributes two units consistently with existing256-unit policy. Root slash stripping correctly avoids double separator for POSIX slash root and Windows drive root, subject to the POSIX literal-backslash finding.

## Focused Proof Limits

New declared Process root-family-preservation suite admits darwin/linux/win32, targets the existing cache test file by exact testNamePattern and preserves300000ms budget. Six neutral vectors exercise genuine issued-path publish/retain, ordinary-child refusal, ordinary-child→path refusal, path-bound refusal and cancellation. Actual Generic native calls assert custody returns baseline, peak within32, payload inode/device preserved, exact i/p/r capsule roster and target resolution. Independent Python verifies retained inode/device/payload. These are useful real native assertions when executed.

The new six-vector corpus does not itself include copied/expired receiver negatives, exact256 accepted boundary, trailing filesystem-root separator or nonBMP path-bound cases. Existing live/receiver checks remain by inspection; no current runtime success is inferred. Pair/size owner-reported greens and12of13portable progress are separate evidence. Original full goal remains pending actual remaining gates.
# Root Independent Follow-Up

Root inspected both private directory constructors, original-root seeds and withDirectoryPath propagation. Qualification is private and only propagated from an already qualified receiver through actual issued path proofs. Plain nested/create frames receive no qualification and cannot gain it by another path call. Existing exact receiver, live-frame, whole-lineage identity and same-device checks remain. Capture/publish/retain use the full derived path-unit bound; ordinary payload/capsule/roster/close guards were retained in the inspected snapshot. The POSIX seed must strip only '/', with literal backslash preserved; Windows strips its native trailing separator. This review supplies no actual native or strict result. Retained46620, focused gate21858 and25-root strict83171 are still required.

