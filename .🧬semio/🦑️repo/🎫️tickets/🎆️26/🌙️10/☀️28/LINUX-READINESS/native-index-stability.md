# Native Index Projection Stability Review

Read-only review of current shared FieldText delta, 2026-10-29. No tests, queries, builds or Source edits by reviewer.

The prior installed candidate completed cold97.631s/warm76.774s with identical JSON and physical snapshot reuse, but rebuilt the warm index. Owner’s eight persisted vectors showed identical tokens/facets in different order. Current FieldText sorts map keys lexically and preserves sequence order; both canonical tree and duplicate client call the shared helper instead of separate map-range serializers. fingerprintSearchProjection walks the current tree, hashes length-delimited json.Marshal documents, and uses the same document text builder as indexing. json.Marshal provides stable string-map key ordering. No additional nondeterministic map walk was found in these inspected projection functions.

Current cached codebase Data consists of string/aliased-string fields and already-float64 section/definition positions, so the shared helper preserves their JSON-roundtrip text. Fresh domain trees are built each request; their richer typed contributor sequences are not reused from the authored-codebase snapshot. Tree child sequences are preserved by serialization; this review did not launch a new runtime traversal to claim measured reuse.

Concrete generic helper parity edge sent to owner: a typed nil []string renders empty text through strings.Join, while JSON roundtrip turns it into nil interface and fmt.Sprint renders <nil>. Other typed nil slices similarly render empty before snapshot and <nil> afterwards. This is a real input-type difference, but no such value was found in the currently cached codebase projection, so it is not established as another cause of this native warm rebuild. Add a neutral nil-slice/null contract vector or explicitly narrow/normalize the helper’s accepted projection values before claiming general snapshot-type equivalence.

Owner reports repeated-map and JSON-roundtrip regression red→green, all five Go modules765passed97skipped and independent SQLite6tests27assertions green; cross-platform batch47833 was queued at review request. These are owner-executed results. Fresh installed-native index reuse and progress throttling remain separate acceptance gates.

## Null Container Review Edge

The independent read-only review found typed nil slices serialize as an empty field, while JSON roundtrip yields null and previously rendered a literal nil marker. This is not established as the observed codebase index miss, but the neutral contract now explicitly declares empty text for absent fields. The regression covers nil string slices, aliased slices, maps and pointers alongside non-null aliased sequences. Its red execution is pending normal capacity (handle 85113), before implementation normalization. Windows compilation passed for the ordered-field implementation; the remaining platform batch is queued.

The null regression reached a real red result: indexed text contained a literal nil marker where the declared absent field text is empty (generated/native-index-null-red.log). FieldText now normalizes nil slices, maps and pointers to the same absent scalar representation, preserving non-null sequence order. Formatting completed successfully. The preceding Windows/macOS and complete oracle batch completed status 0. The final null-inclusive five-module suite is waiting for its normal 2,048 MiB admission on handle 43097; no final native install or query is running.

## Frozen Candidate Proof

The null-inclusive five-module suite completed status 0: 765 passes and 97 skips. The exact normalized Source subsequently compiled for Windows amd64 and macOS arm64 and passed each independent oracle envelope: model 1 test/21 assertions, search 6 tests/27 assertions, discovery 2 tests/22 assertions (9 tests/70 assertions total). Handles 43097 and 48479 are terminal 0. The reviewer found no further concrete issue after the nil parity correction. Source implementation and runner inputs are frozen; the next step is metadata revalidation and one normal native installation, followed by a separately coordinated quiet-window acceptance gate.

## Installed Final Candidate

Normal-admission repo-mcp:install handle 59748 completed status 0. Installed ELF: 20,361,290 bytes, mode 0755, SHA-256 3f61eed0b685ab69a43f5fe9d079806bdd09b865efeaa60d51ef069380fdb268; the actual closed Receipt schema validates, its digest matches, receipt mode is 0600. The pre-install selected metadata covered 1,257 files including all 800 Go files. Post-install verification correctly refused two concurrent TypeScript edits in startup service and patch tests; no Go file changed. The build-time metadata is preserved separately.

A fresh current selected metadata census covers 1,262 files, all 800 Go files, totaling 19,197,888 bytes. Its strict private Source:O comparison passes with zero drift. The difference comprises those two TypeScript edits plus five new non-Go authored configuration/test inputs; no Go addition or change occurred. No Source payload was copied, no cache was reset, and no reinstallation or search retry was run. Current ELF is ready for the separately coordinated Source quiet-window gate. Logs: generated/native-index-install.log, native-index-installed.log, native-index-source-after-install.log, and native-index-source-current-after-install.log.

## Deferred Quiet Window

The coordinated freeze was authorized for 2026-10-29 01:18–01:24 UTC, then released early at 01:21 UTC because prerequisite Docker metadata revalidation remained capacity-queued. No native query was launched, so this window yielded no search pass or failure. One existing metadata handle (88852) remains pending; its eventual result requires a new Source freeze/revalidation before acceptance. No reservation bypass or duplicate attempt was made.
