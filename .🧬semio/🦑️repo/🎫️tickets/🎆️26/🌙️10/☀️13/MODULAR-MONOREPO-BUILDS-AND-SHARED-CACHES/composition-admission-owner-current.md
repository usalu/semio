# Composition Admission Owner

The original child-restore projection is now owned by `semio-framework-composition-admission`, under schema composition admission. Its unchanged 6,495-byte implementation body moved out of Store. The complete owner currently measures 6,581 bytes and 134 physical lines. It retains the original 64-reference, 256-byte field and 256-visit limits, borrowed identity rows, errors, complete-set admission and member matching.

The new package depends only on the existing schema-composition and artifact-reference owners. It has no Store or Kernel dependency. Its declared Source limits are 65,536 bytes, eight files and 1,000 active lines. Kernel and Plugin directly depend on the canonical package. Exported Store macros use Kernel's explicit reexport of that exact owning crate; Store does not retain old type aliases or a compatibility facade.

A second monorepo audit found 75 remaining Rust caller files outside Framework. Those callers now use the canonical owner, and 39 nearest physical Cargo owners declare their direct path dependency. Every caller resolved to an actual package. The exact roster is retained in `composition-canonical-callers-current.md`. Existing higher-level lifecycle tests remain at their genuine Store and Plugin owners.

## Verification

The schema-first ownership test initially failed because the package manifest did not exist (Original 12005). After extraction, the final prototype ownership gate passed three tests and 40 assertions in 369ms; release passed three tests and 40 assertions in 160ms. The focused strict TypeScript gate exited zero. These laws use Ajv, an independent TOML parser and SQLite identity-set queries. They verify actual body ownership, canonical Kernel/Plugin dependencies, macro paths and bounded production/native Source on macOS, Linux and Windows cfg inputs. They do not prove native runtime behavior on those hosts.

Three native tests are authored against the actual owning crate. They compare projection results with independent serde_json identity sets, test exact unordered complete-set admission, preserve 64-reference and UTF-8 byte boundaries, and check sparse traversal at 256/257 steps. They have not yet executed.

The first selected native attempt (27954) failed during actual preparation before the compiler; excessive per-step progress obscured its terminal cause. Ingress repaired the real preparation cadence and bounded diagnostic delivery. The corrected named-test successor (27539) then failed cleanly at Cargo update after 18.6s: BREP offset inherits `semio-framework-3d-brep-construction`, while the workspace still declares its old operations key. The construction package exists in the operations directory under its new package name. An earlier filename-pattern search missed it; the earlier absence inference was wrong. Multiple old BREP callers remain during that separate active refactor. No invented mapping, compatibility alias or unrelated BREP body change was added.

The direct owner-script attempt (28655) initially refused a missing explicit test policy. The new owner script now constructs the existing repository test policy; `runRepositoryCargoTestsV1` replaces physical paths with its genuinely issued compiler/capture storage before writing. Successor 6549 reached actual Cargo fetch and encountered the same workspace inconsistency before compilation. No native compilation or test pass is claimed.

All Root Original handles are terminal. The metadata and Source checkpoint is stable for the protected ordinary Nx pair. A LOW read-only agent is auditing ownership, callers, macro hygiene and dependency direction in `projection-cut-audit-current.md`.

Store still measures approximately 779,959 bytes and 11,432 physical lines. This cut is concrete progress toward removing giant crates; the larger Store SCC and whole-Kernel Source boundary remain unfinished.

Exact settled Root generated transcripts were removed after retaining these outcomes and causes. No shared generated directory, foreign output, process, lease or mount was removed.

## Independent Audit Follow-Up

The LOW read-only audit found no concrete ownership or macro-path regression, but correctly identified incomplete independent oracle and caller evidence. The shared identity corpus now includes standard/subset emptiness, exact 256/258-byte UTF-8 fields for every owned identity field, and an unknown slot. SQLite checks all those byte/identity constraints independently; the native serde oracle and actual visitor fixture consume the same expanded corpus. Native complete-set tests also reject a duplicated incoming reference.

A closed caller corpus now records 87 actual canonical Rust callers with their physical Cargo manifests. The contract verifies every declared Source uses the canonical crate and its manifest directly depends on that owner. Final prototype 25211 passed four tests and 400 assertions in 224ms; final strict 4072 exited zero. Release is running. This closes those static audit gaps for the recorded current caller inventory. Native execution, resolved Cargo direction and external macro expansion remain unproven until genuine compilation succeeds.

Final expanded release68983 passed four tests and400 assertions in184ms; prototype25211 passed four/400 in224ms; strict4072 exited0. Actual native tests remain pending the workspace rename frontier. All Root handles are terminal. The protected ordinary Nx prototype99411 is live on fresh SDK5b6cb1c1, with release scheduled only after terminal settlement. Root will preserve shared compiler/tooling/graph Source during that pair.

The second LOW audit verified current mode/profile and writer literals in build-mode-writer-audit-current.md. It confirms remaining effective-config flag admission and dedicated watcher custody frontiers without claiming an observed external write. Storage is preparing a new independent schema/fixture leaf for config admission and will defer wiring until the pair settles.

Exact settled Root transcripts removed after this record: root-composition-oracle-prototype.log, root-composition-callers-prototype.log, root-composition-callers-types.log, root-composition-callers-release.log.


## Native Qualification Successor

The earlier protected ordinary Nx attempt 99411 terminated at its genuine 240-second deadline before its first Nx child. An unrelated ticket held the broad repository-graph writer. Nx has since qualified a separate funded admission scope, retaining the real image and collection lock; that work is recorded in the bootstrap report. The prior timeout is not a current ordinary cache qualification.

Root corrected the actual Framework workspace dependency declaration to the existing `semio-framework-3d-brep-construction` package and updated its five remaining manifest inheritors. The next genuine native attempt (38027) exposed the missing workspace dependency declaration for the existing split sweep package. Framework now explicitly binds the three physical boolean, sweep and offset package manifests. No package alias or implementation substitute was added. Remaining BREP Rust callers of removed operation modules are outside this selected crate qualification and are not claimed to compile.

Original 44836 then reached the real pinned compiler and exposed a syntax error in the newly authored integration test. Root corrected its Rust let declarations and supplied the required child-slot field in the bounds fixture. The production projection body did not change. Final prototype 53401 passed all three actual native tests through `crate-frontier-composition-admission-tests` in 50.5 seconds overall. Release 85374 passed all three in 45.3 seconds overall. The actual compiler receipt identifies the physical composition-admission manifest in both modes. The tests cover the shared neutral corpus, exact unordered complete sets, duplicate incoming references, 64/65 references, 256/257 UTF-8 bytes, and 256/257 sparse traversal steps.

These results qualify the extracted lower owner and its actual production dependencies. They do not qualify Kernel/Plugin compilation, external Store macro expansion, all BREP crates, a complete Store SCC split or the latest ordinary Nx cache pair.

A focused physical workspace-package identity law is being qualified separately. A broad existing preparation suite ran and genuinely failed (37541: two passing, thirteen failing tests); failures include stale Source-string assumptions, synthetic authority and a simultaneous workspace member mismatch. No current Original grant or production guard was relaxed to satisfy that suite.


## Borrowed Reference Dependency Cut

The current production dependency changed from owned artifact-reference to `semio-framework-artifact-reference-borrow`, a new zero-dependency canonical identity floor. This removes value/codec/registry imports from the projection's production graph. The exact new evidence and remaining caller proof are in `borrowed-reference-owner-current.md`. Native prototype successor 42518 passed the updated three-test projection target; predecessor native results above describe the earlier dependency graph. The public membership operation now consumes an explicit borrowed view rather than an owned reference type.
