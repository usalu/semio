# Source Owner Factoring and Bounded Closure Qualification

The unchanged Source guards exposed a real closure-capacity regression. The first current backend capture refused file 513 at 4,133,149 bytes against 512 files/16 MiB. The authorizer reader completed at 178 files, while runtime refused its unchanged 256-file/16 MiB guard; the initial runtime refusal did not expose an exact final count.

Three ownership changes remove actual reachable files, with no import scanner, follow rule, cap, captured-byte check, or manifest omission changes:

1. The private tooling read-context definitions now live beside the concrete tooling-store owner. All eight moved non-import declarations retain their exact Source hashes. The old scope module is removed; all callers use the concrete owner directly.
2. Publication Slot and Namespace policy now has one canonical definition in the existing physical image schema. The administrative publication schema no longer defines those policies. Ordinary physical maintenance consequently no longer imports the administrative publication schema. The publisher imports canonical policy aliases; all 24 existing publisher function bodies retain their exact hashes. Graph administrative/helper/tests consume the same policy. The Graph helper still legitimately imports administrative publication policy for the actual bounded refusal reporter and prepared-service selector.
3. The mixed SDK capacity re-export module is removed. Callers import the existing pure formula or physical publication owner directly. Both actual implementation files retain their complete byte hashes. No forwarding or compatibility module remains.

The declared reader roster is hand-aligned with the complete actual capture: add the retirement schema and continuation module/schema, remove the deleted capacity re-export module. No declarations omit reachable imports.

## Actual Source Evidence

- Scope ownership semantic RED: the concrete tooling owner initially lacked the required export. The new schema-owned ownership law and independent SQLite comparison then passed. Intermediate reader capture decreased 178 → 177.
- Metadata schema semantic RED: physical schema initially lacked Slot/Namespace. The actual schema/SQLite law then passed. Intermediate reader decreased 177 → 176.
- Capacity owner semantic RED: the combined facade still existed. The closed owner policy/SQLite law then passed after direct role imports; reader decreased to 175 and runtime completed at 256.
- Final complete per-role double captures naturally exited 0, with independently recomputed Bun SHA and SQLite reader-roster equality: reader 175 (`48c0b0fe3dbbfaebacefb892f970b9248ebf026a24bee71922f76d91c6fd52af`), runtime 256 (`64ac27710a438ba0788120df7fd4d29deb3230021d0c63025ba6d5f74c47cf22`), compiler 137 (`135b7c2c7609b9b696fb40a41b28342a81f7c58cd775b066313f5e19289d2e9a`). Each role is a complete stable pair; this does not claim a simultaneous atomic snapshot across all three roles.
- Final backend complete double capture naturally exited 0: 512 files, 4,147,404 bytes, digest `5d8ec5701134d1ec675c176ce25ea875387af186d10acfcdd1d0aaa1d81f9a8f`, and the genuine same-wire receiver exited 0. Its aggregate report intentionally retains `passed:false` because no backend runtime phases were executed.
- The full existing backend Source gate then naturally exited 0: five explicit strict roots with zero diagnostics, 55 passing Source laws, 9 deliberately skipped actual runtime phases, 1,613 assertions, 25.52 seconds. No installed SDK/runtime phase was executed.
- A previous backend pair correctly refused a concurrent dependency-owner edit (98,484 → 98,504 bytes), even though both captures fit at 512. It is retained as refusal evidence, not success.
- Graph strict mode: three explicit roots with zero diagnostics and 28 passing Source laws/1 isolated skip/1,988 assertions.
- Publication dispatch subset: 4 passing laws/526 assertions.
- Full publication Source-only semantic suite after precise test-harness repairs: 61 passing laws/11,993 assertions, one filtered heavy test, 77.32 seconds. The observer AST fixture needed its real publication-schema port, repaired by Root. The fixed-kernel AST fixture needed actual `import.meta.url` substituted with the Source file URL before Function compilation; no generic descriptor body changed.
- The combined Tools strict+full Source helper compiled with zero diagnostics but exhausted its existing finite 60-second Original near the final integrity law. A separate role-strict attempt and full Source attempt under concurrent large suites also expired. These are not passing tests. No timeout/cap was increased. Serial full Source qualification then naturally exited 0: 35 passing laws/1 isolated worker skip/2,917 assertions in 39.05 seconds, plus derivative reader completion 1 law/81 assertions in 0.62 seconds. The same five explicit strict roots then passed with zero diagnostics through the existing narrower role-strict mode; its 13 actual Source laws passed with 1,778 assertions in 14.64 seconds.

## Boundaries and Refused Test Lane

No SDK runtime, installed authorizer, Bun installer, service operation, or backend task execution was accepted by these capture gates. Existing actual runtime/publication qualification remains Root-owned and separate. File counts are at the current upper limits; any later actual reachable import must still be captured and may correctly refuse.

One incorrectly selected regression lane, `original-fixtures --derivative-phase`, reached existing Native bootstrap admission and refused pending publication before the requested test phase. It was stopped without retry. Its log is retained as `Generated/bazel-planner/capacity-fold-derivative-phase.log` (the actual directory is `🗑️generated`). Low independently reviewed the exact call order in `2026-10-17-derivative-helper-admission-boundary-review.md`: an actual shared persistent guard precedes pending-publication admission, so lease/SQLite effects are possible. The pending guard precedes receipt/backing/root reads and image commands on this receiver. Zero prior admission/effects or globally closed custody cannot be inferred. This lane is not Source acceptance evidence.

The current whole-file manifest is `🗑️generated/bazel-planner/source-owner-factoring-manifest.json` (35 paths, two removed). Shared-file edits after its point-time are independent work and require a fresh hold/capture before future qualification. Preservation evidence consists only of hashes and manifests; no checked-in Source files were copied into the ticket.

## Independent Review

Low completed the owned factoring review in `2026-10-17-source-factoring-independent-review.md`: publisher 24 body hashes match, both concrete capacity implementation files match whole bytes, and all eight moved scope declarations match. Current bounded descriptor/name/seven-stamp comparison matches 32 existing manifest files and confirms both removed files absent. The shared ADMIN file has legitimate subsequent peer-owned preserving-completion edits and is deliberately not claimed as part of a current complete 35-file hold. A final combined shared-file hold and fresh Source captures are required after that integration settles.

The shared ADMIN preserving-completion peer checkpoint subsequently settled with its own strict/Source gates. Its manifest hash was refreshed to 194,419 bytes / `3f891d3d5cefe3abbfd6410690f70a1cb91286705681f60e936730ae252b7f1d`. Low then completed the full current 35-input bounded hold (33 existing files, two absent removed files) in `🗑️generated/source-factoring-final-independent-hold.json`. The earlier complete closure captures remain their prior point-time; this final hold does not relabel them after the peer integration.
