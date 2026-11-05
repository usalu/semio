# Graph Preview Source Identity

Both graph editors now carry sourceRevision as a byte array from fill publication and completion into SetSolve and the app transient. The source is the original ToolRunIdentity.base_revision, stamped before publication and completion. Field-sparse diffs, exact inverse, retained row patching and retained SetSolve include the field. Native DSL/pack derive codecs therefore carry it without parallel wire formats. Both mutation schemas, tick schemas, committed transient before/after/mutation/diff vectors and portable partial tick fixtures were manually updated.

The hosted adapters use doc.render_operation_ref().canonical_base_revision. Cached assignments and contradiction status are visible only for an exact 32-byte source match. Missing render authority suppresses inferred data. Live runs additionally require run.identity.base_revision to match current mounted authority; decoded payload sourceRevision must match that run identity. Seed, rule or pin changes preserving all slot/tile IDs can no longer retain stale solved media. Graph3D now falls back to authored slot pins when no fresh inferred assignment exists.

Graph2D's existing host-render regression now creates an actual mounted render context. Graph3D exposes the equivalent exact host projection and tests it through the same context. Each covers fresh, changed-source and unbound cache behavior. Both also exercise actual mounted live previews against all six shared language-neutral revision cases. Fill tick tests check sourceRevision against decoded tick identity on each published tick, including successful completion.

## Actual Carrier Bounds

The framework ToolRunTick ceiling is 262144 bytes, lower than the engine's separate commit envelope. The graph retained raw command envelope is now 2 × tick ceiling + 16384 = 540672 bytes, covering payloadJson escaping. The result envelope is 4 × tick ceiling + 16384 = 1064960 bytes, covering complete solve records and sparse-diff/inverse framing. Both factory contracts and typed proof contracts use these same constants.

Completion now admits the final tick before dispatching commit-fill or retaining SetSolve. Failed oversized tick encoding produces a named wfc2d-fill-payload-capacity / wfc3d-fill-payload-capacity fault and zero completion effects. The portable envelope fixture and native tests use 100 valid 1024-byte tile IDs: about 104 KiB, above the former 64 KiB raw command ceiling and below the actual tick carrier. A 300-assignment case exceeds the tick ceiling and must produce no completion.

The existing 4096 combined document-row admission and 1024-byte identifier checks remain. This does not claim every combination of those maxima is solvable: a result that cannot fit the actual tick carrier is explicitly refused. Original caller grants and lifecycle policy were not increased.

## Verification

Passed:
- Pinned nightly-2026-07-20 rustfmt syntax-only checks over 24 graph Rust sources, including source propagation, retained diffs, host adapters, mounted regressions and envelope laws.
- Independent Bun/Ajv oracle: six revision cases, two graph tick/mutation schema carriers, two envelope cases.
- Independent Python byte-comparison and whole-transient oracle: six revision cases and two graph mutation carriers.
- Scoped read-only git diff --check HEAD over graph2D, graph3D and bitmap.
- Bitmap Python/Pillow oracle rerun: 14 committed vectors, 9 pin-batch cases, 52 retained cases, zero failures.

Native execution is owned by the consolidated verifier. No native compiler or passing runtime verdict has been observed for these tests. No duplicate native target was queued.

An initial Python schema attempt found jsonschema unavailable. The optional uv request was cancelled while awaiting machine capacity; no install was needed. The retained Python input now uses builtin byte comparison; the passing Ajv input performs third-party schema verification.

## Files

Both graph transient sources, mutation aggregates/leaves/schemas and committed vectors; fill sources/tick schemas/fixtures/tests; preview live filtering; host adapters and editor tests; shared graph-fill-envelope fixture; ticket graph-preview-oracle.py / graph-preview-oracle.ts inputs and this report.

The coordinating parent owns ticket lifecycle, browser verification and final acceptance.

## Late Completion Cache Preservation

A late old completion could previously overwrite a newer fresh app cache even though render filtering would subsequently hide the stale value. Bitmap CommitFillSolve and both graph CommitFill retained work paths now compare the completion source against current command authority before producing SetSolve. Stale internal callbacks settle quietly with no transient rows or cache overwrite. Graph completion also requires done=true.

Actual owner Work::step regressions receive authentic OriginalWorkFixture turns, collect the returned transient rows, apply them to an existing complete cache and check fresh completion changes it while stale completion emits zero rows and preserves every prior field. Shared six-case source fixtures cover exact/mismatched/malformed sources. The unbound case is checked at an optional-authority admission helper: a real retained Work input itself requires AppOperationContext and is not forged for that case. Tests therefore verify row effects and cache preservation rather than fault wording.

Pinned parser checks pass ten affected Rust files. The qualified public cfg=[test] preflight now measures bitmap editor 3906 active lines, 244364 bytes,26 files; pin leaf455 lines,36675 bytes,six files. These remain non-authoritative and are not an upper bound. The next actual native Source Admission and compiler run remain required.

The auditor reran the identical qualified public cfg=[test] preflight after late-completion tests: graph2D 3209 active lines,183397 bytes,18 files; graph3D3758 lines,223406 bytes,21 files. These are not authoritative native gate results or upper bounds.
