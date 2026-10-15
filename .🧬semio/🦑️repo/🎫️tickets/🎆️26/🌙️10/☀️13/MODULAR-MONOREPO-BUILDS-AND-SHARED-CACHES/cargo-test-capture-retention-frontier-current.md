# Cargo Test Capture Retention Frontier

Read-only changing shared-host checkpoint, regular files only and no directory symlinks followed. No capture or cache was deleted.

| Mode | Direct semio-nextest Namespaces | Regular Files | Logical Bytes | Allocated Bytes |
|---|---:|---:|---:|---:|
| prototype | 11 | 420 | 32061174 | 32870400 |
| release | 7 | 207 | 17629084 | 18046976 |

Actual repository nextestArtifactLocation treats every nonempty SEMIO_TEST_ARTIFACT_DIR as retain:true. The actual compiler store always projects its bounded capture directory into that environment variable. Therefore ordinary repeated native tests retain each unique nextest metadata directory; the lower runner already owns exact mkdtemp cleanup when retainArtifacts:false. Reusable compiler outputs and temporary test evidence currently share the Rust16 backing. The physical image is bounded but this retention choice consumes reusable-cache headroom indefinitely.

A future schema-owned evidence lifetime must distinguish a destination from retention intent, default to finite scoped cleanup for routine tests, and preserve intentional bounded diagnostics through explicit ownership. It must not delete other agents retained/live captures, infer ownership from age or simply repurpose a path variable into implicit authority. This frontier has not been implemented or runtime-qualified.

## Scoped Evidence Successor

Initial neutral 3840 caught an incorrect test import. Corrected neutral 85946 genuinely failed: an explicit destination returned retain:true. The production policy now requires an explicit destination, returns scoped routine retention:false and removes the OS temporary fallback. An explicitly authored lower policy can still request retention inside issued bounded backing. A closed portable corpus independently checked with Ajv and SQLite covers destination normalization and missing/empty refusal. Real native qualification uses the complete Original/actual compiler backing, exact dependency preparation and original PackSource laws twice inside its own short evidence parent. Qualification is pending. An early native invocation only reached unknown private command before route registration; no native result is claimed. No historic captures were deleted.

## Actual Scoped Evidence Qualification

Initial neutral3840 had an incorrect test import; corrected85946 genuinely failed implicit retention. Initial native62620 reached actual storage path refusal because the private lower-runner fixture carried the repository policy coverage path outside its issued backing; coverage is now explicitly disabled/null for this noncoverage law. Successor native50031 and73858 both exited0 and each ran the real PackSource assertions twice with original dependency preparation, actual storage admission and exact owned evidence parent empty after every run. No unrelated/historic captures were removed. Neutral4256/85616 and expandedstrict80774 passed before adding the missing-env default.

Independent review found policy construction precedes actual compiler opening for ordinary callers. Genuine default-law15970 failed before implementation. The successor purely projects the authored Rust partition and canonical compiler layout for missing path, retains false, and lets actual execute replace with issued backing before any write. Blank explicit paths still refuse. Final default neutral86514/21050 and expandedstrict18469 exited0. The native final-policy recheck and independent audit are next.

## Final Current Evidence Policy

Independent audit found eager uncharged coverageDir creation even for disabled coverage, the canonical rebasing of all coverage owners to one shared coverage.lcov, and private fixture setup outside finally. Genuine neutral86715 passed routine evidence but failed disabled coverage receiving the metrics report path. The factory now creates no metrics directory: disabled coverage is null; explicit existing SEMIO_COVERAGE=1 projects a bounded owner filename including a stable16hex manifest digest. Actual execute rebases only that filename into issued capture backing, preserving owner separation. This is explicit report publication, separate from routine temporary nextest evidence. No new retention flag, cap increase or third-party runtime dependency was introduced. Full LLVM coverage execution is not qualified by these policy tests.

Final neutral84471/54236 both pass2laws with independent Ajv/SQLite/Bun SHA checks; expanded strict2218/55865 both exit0. Private owned-parent finally now covers policy construction and no longer overrides coverage fields. Final actual native97461/82890 both exit0: original PackSource assertions run twice in each mode through genuine issued compiler/dependency admission and each owned evidence parent is empty after both settlements. These calls use the final production disabled-coverage policy naturally. Every Root operation is terminal before the next SDK capture.
