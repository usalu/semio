STATUS: FAIL, do not apply. 12 blocking findings in 9 files (8 new `.unwrap()` sites, 1 raised test deadline, 1 hard-coded production language, 1 new `#![allow]` plus a machine-specific absolute path in the apply script). 2 medium, 3 low.

Audit of C/1102-tools/*/patches/ (skip `_superseded/`). Snapshot taken 13:26 to 13:29 (policy/patches/index.json mtime 13:26:54). The policy lane was still changing during the audit: `semio-framework-actor.patch`, `semio-framework-actor-retained-turn.patch`, `semio-framework-dispatch-macros.patch` and `semio-framework-value-derive.patch` were picked up in the same window. Re-run before the next patch window if any file changes.

Scope: 67 `.patch` files (`cadi18n` 22 incl. the 2520-line `cad-spec-json-1.patch`, `policy` 38, `uii18n` 6, `testkit` 1), 3 `.py` scripts and 3 `.md` notes read in full. The 38 `.check` files and `index.json` are verification markers and were not reviewed as patches. Line numbers are line numbers inside the patch file.

| # | Patch | Line | Rule | Snippet (<=80 chars) |
|---|---|---|---|---|
| 1 | policy/semio-framework-os-store-engine-branch.patch | 8 | BLOCKING no `.unwrap()` in non-test code | `let branch=self.branch.as_ref().unwrap();` |
| 2 | policy/semio-framework-os-store-engine-replay.patch | 35 | BLOCKING no `.unwrap()` | `settle_replacement(self.key.unwrap(),&mut self.checkpoin...` |
| 3 | policy/semio-framework-os-store-replay.patch | 63 | BLOCKING no `.unwrap()` | `source:job.take_rejection().unwrap(),text:None` |
| 4 | policy/semio-framework-os-store-replay.patch | 69 | BLOCKING no `.unwrap()` | `let owner=self.retirement.as_mut().unwrap();` |
| 5 | policy/semio-framework-plugin-job-receiving.patch | 41 | BLOCKING no `.unwrap()` | `let admission=self.admission.as_mut().unwrap();` |
| 6 | policy/semio-framework-plugin-reactor-core.patch | 26 | BLOCKING no `.unwrap()` (metadata.rs) | `self.receiver.as_mut().unwrap().advance(control,body,g)` |
| 7 | policy/semio-framework-plugin-reactor-core.patch | 27 | BLOCKING no `.unwrap()` (metadata.rs) | `self.receiver.as_mut().unwrap().take(g).result?` |
| 8 | policy/semio-s-artifact-stdio-ifc-2x3-base-details.patch | 19 | BLOCKING no `.unwrap()` (field_edit.rs) | `self.collection.as_mut().unwrap()[index]` |
| 9 | cadi18n/patches/cad-engine-1.patch | 78 | BLOCKING no hard-coded language, no default language. Persisted object names pinned to en (used at line 88) | `const OBJECT_NAME_LOCALE: Locale = Locale::En;` |
| 10 | cadi18n/patches/web-acceptance-1.patch | 10 | BLOCKING no raised deadlines (brief: do not raise deadlines). Test budget grows by 3x the surface timeout | `test.setTimeout(TEST_TIMEOUT_MS + 3 * SURFACE_CONTENT_TIMEOUT_MS);` |
| 11 | stdio/patches/decouple-mesh-export-from-pdf.py | 61 | BLOCKING no new `#[allow(` (the actor and dispatch patches remove this allow, this script re-adds it) | `"#![allow(async_fn_in_trait)]\n\nextern crate semio_framework_os_kernel` |
| 12 | stdio/patches/decouple-mesh-export-from-pdf.py | 8 | BLOCKING zero-touch and cross-platform: absolute machine path, script fails on any other host | `R = pathlib.Path("/Users/ueli/Documents/semio/...")` |
| 13 | testkit/patches/publish-framework-schemas.patch | 10 | MEDIUM no `.expect(` in non-test code. Hunk is in `🧪️testing/` (support crate, not a `tests/` dir). Confirm it is cfg(test)/dev-only, else use a refusal | `PluginRuntimeRegistry::publish_framework_schema_documents().expect(` |
| 14 | rtpanes/patches/laws-publish-framework-schema-documents.md | 10 | MEDIUM same as row 13, planned for the `🧪️testing/` law constructors | `publish_framework_schema_documents().expect("the OS's own ...` |
| 15 | 26 policy patches, cadi18n/cad-engine-1.patch, testkit/publish-framework-schemas.patch | 106 (cad-engine-1), 8 (testkit), 62 lines in policy | MEDIUM concise code: rustfmt-hostile multi-statement one-liners (>300 chars). Not a listed rule, but must be reformatted before merge | e.g. `semio-framework-os-store-engine-replay.patch` lines 8, 12, 35 |
| 16 | testport/patches/cursor-dev-vcs-testkit.py | 2 | LOW docstring must start with an emoji | `"""Apply from the repo root in a patch window. store-cursor's ...` |
| 17 | cadi18n/patches/web-acceptance-1.patch | 8 | LOW emoji reused in same file (existing docstring at target line 528) | `/** 🔤️ Switching the language leaves an opened pane running...` |
| 18 | cadi18n/patches/cad-spec-json-1.patch | 9 | LOW verbose: `reuse` matrix is identical to `native` in 615 of 615 labels | `"native": { "en": "Construct Reinforced Concrete Column", ...` |

Checked and clean:
- Comments inside function bodies: none in any added line (Rust, TS, Python). The only removed comments are in the actor and dispatch patches.
- `[DEBUG]` lines, `println!`/`dbg!`/`console.log` in production: none. The `print(` calls in `cad-test-languages.patch` belong to its test report.
- `#[allow(` / `#![allow(`: none added as attributes. Three policy patches remove them (actor, actor-retained-turn, dispatch-macros). The dispatch-macros doc paragraph mentions `#![allow(...)]` as text only. Row 11 is the only real new allow.
- `unreachable!`, `panic!`, `todo!`: none added. `value-derive` replaces `unreachable!()` with a `syn::Error`, which is good.
- `unsafe`: the two storage patches rewrite existing unsafe blocks, they do not add new ones.
- Compat, deprecation, legacy, shims, aliases, `pub use ... as`, type aliases, `#[deprecated]`: none. Migration scripts in the source tree: none.
- Raised numeric limits in production: none. A numeric-literal diff per hunk found changes only in new test assertions. Row 10 is the only raised deadline.
- Paths of 256 chars or more: none. Max patched target is 202 chars (215 UTF-16 units). New crate paths in the decouple script are at most 185 chars (202 UTF-16 units).
- UI strings: all `uii18n` labels carry en and de in both `native` and `reuse`. The 615 JSON label matrices have no single-language or blank cell. 55 native rows have identical en and de text, within the 85% threshold of the oracle test (560 of 615 differ).
- Test-only targets: `cad-tests-1..4`, `cad-test-languages`, `web-shellhost-1`, `web-acceptance-1` touch only `🧪️tests/` paths. No test-only patch changes production files. The `🧪️testing/` hunks are rows 13 and 14.
- Docstrings: every new `///`, `//!` and `/**` block starts with an emoji except row 16. `procedural-docstring-1.patch` moves an existing docstring, it does not reuse one.

Per-lane counts (reviewed files / added lines / findings: blocking, medium, low):
- cadi18n: 22 patches / 2665 added lines (2520 in cad-spec-json-1) / 2, 0, 2
- policy: 38 patches / 150 added lines / 8 sites in 6 files, 0 medium (dense lines: 62 in 26 files), 0 low
- testkit: 1 patch / 23 added lines / 0, 1, 0
- uii18n: 6 patches / 35 added lines / 0, 0, 0
- stdio: 1 script (0 `+` lines, read in full) / 2, 0, 0
- testport: 1 script / 0, 0, 1
- flowext: 1 script / 0, 0, 0 (explicit facade re-exports of two names, no docstring)
- describe: 1 note (prose only, dev-dependency) / 0, 0, 0
- rtpanes: 2 notes / 0, 1, 0

Patches with no blocking row (uii18n, flowext, describe, and the remaining policy patches) pass the rules above. The six policy patches of rows 1-8 and the two cadi18n patches of rows 9-10 each need a fix before they are applied, and the apply script of rows 11-12 needs a fix before it is run.
