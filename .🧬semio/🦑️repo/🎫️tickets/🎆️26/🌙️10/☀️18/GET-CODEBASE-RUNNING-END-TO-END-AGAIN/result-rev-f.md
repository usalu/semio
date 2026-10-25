FOR alloc-f: (1) `cleanup/🎛️command` foreign mount must not be `transient`: new non-retried error with EN+DE text naming mount, backing image, device, `hdiutil detach <device>` and the owner from the nearest `owner.json` (review-1025f.md #2). (2) `repoTestArtifactEnvironment`/`withProcessTestOutputV1` with a preset `SEMIO_TEST_ARTIFACT_DIR` leave TMPDIR inside the managed parent; override TMPDIR/TMP/TEMP via `managedNativeParentOfV1` and refuse a preset artifact dir inside a managed parent (#3). (3) 7 image tests still use `mkdtemp(tmpdir())`; tighten rule `native-image-unsandboxed` (#3). (4) one-time reclaim of pre-fix `st` dirs inside managed parents, per-user short base, `fixed.dir` fixture case (#4).
FOR knot-f: validate `options.bootstrap` eagerly in `holdServiceInvocationV1`, give `ServiceBootstrapPortV1` a structural return type (#6).
FOR dash-f: `bind_guard` is sound; optional bounded `try_lock` deadline instead of blocking `lock()` (#5).
FOR crate-f / whoever runs pets-rs: `@semio-tech/pets-rs` differential lane must now build `🧪️tests/🏞️terrain-walking/🦀️.rs` (E0255 fixed by rev-f, not compiled).

# result-rev-f (wave 1025f)
- 20:58 DONE. Review in `review-1025f.md` (7 ranked findings). One certain defect fixed: pets terrain-walking Rust adapter name clash (E0255). Nothing running; no processes, no generated files.
- Checks: sandbox 20/20, cleanup-command 5/5, service-invocation law 2/2, service admission 117/117, pets company vectors 39/39 (TS) and numpy oracle 5/5 scenarios (scratchpad scripts only).
- Not verified: Rust compile of the fix, vitest layer law, Bazel TEST_TMPDIR path, dashboard cargo tests.
