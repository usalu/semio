# New Regression Test Paths

Read-only source validation on 2026-11-04. No native compile or test was started; this report does not claim a native pass.

## Findings

No missing fixture, Cargo mount, runtime package scope, or newly introduced private visibility issue was found in the inspected wiring. All path checks used path.resolve against the actual containing file and fs.existsSync.

- Editor private retained tests mount from retained-commands/🦀️.rs through cfg(test) into retained-commands/🧪️tests/🔬️unit/🦀️.rs. Its ../../../🧪️tests/🧰️support/commands.rs resolves to the existing editor shared fixture. The test module is a descendant of the private retained implementation; use super::* exposes its parent names to the tests. The fixture every_command is pub(super), sufficient for its immediate test parent. The public editor unit integration also mounts this shared fixture through ../🧰️support/commands.rs.
- Modes locale regressions mount through cfg(test) into the edit-mode library. Its Cargo [lib] path resolves to the actual edit-mode source. locales.json exists, parses as JSON, and carries explicit en/de zero, singular and plural cases plus the shared paint preset label. serde_json and the async test macro are declared dev dependencies.
- Viewer locale regression mounts through cfg(test) into the viewer library. Its Cargo [lib] source and locales.json both exist; JSON parsing succeeds. Its expected window title override is false and its expected native labels are en Mesh and de Netz. serde_json and the async test macro are declared dependencies/dev dependencies.
- Paint terminal regression is an editor integration test: editor Cargo.toml explicitly mounts lowpoly_commands_paint at the paint unit file. Both the context support path and ../🧫️fixtures/terminal.json resolve. terminal.json parses and specifies brush to eyedropper abort/commit cases. The tested lowpoly_paint_step function is public in the editor crate, so the integration boundary is valid.
- Every explicit editor Cargo path entry and modes/viewer [lib] path checked exists.
- Permanent 🌎️hub/🧩️compositions/💠️lowpoly/📦️packages/🦀️rust/📜️script.ts RuntimeTestScript explicitly selects hub, editor, engine, io, mut-objects, modes and viewer and forwards remaining arguments to runRepositoryCargoTests. Thus the new modes/viewer library tests and editor retained library/paint integration tests are in its package scope. This review did not independently execute the helper or validate native results.

## Limits

This is a narrow path, fixture syntax, mount and source visibility review, not Rust type checking or execution. Existing unrelated compile issues are outside the review.
