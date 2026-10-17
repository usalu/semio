# Bounded Owned Capture Overflow Diagnostics

Root's actual recovery91721 saved log Generated/coordination-17/parent-recovery-after-mapping.log confirms the prior mapping exact-body error is gone. It records the same exact current payload/raw-journal observations, then Owned data capture output limit exceeded. The old stack identifies only the generic capture loop, so it does not establish which executable, stream or limit failed. No current physical mapping, allocation phase, zero-charge or retirement completion is inferred from that log. No agent Native/protected/cache operation was run.

## Source Candidates and Limit Preservation

The successful-child retirement Source path first calls requireNativeSuccessfulChildReadersV1: global unfiltered lsof -nP -F0pcfatn, bounded at1048576bytes and16384lines. The persistent backend later calls hdiutil detach/info with1048576bytes and32768lines. Both use the same owned capture engine. The saved91721 error lacks the facts needed to distinguish these candidates. A future coordinator-owned physical diagnostic is required; lsof being global is not itself evidence that it caused this failure.

No limit, command, Native option, timeout, cleanup reserve, decoder, captured payload, failure precedence or termination rule is changed. The existing exact refusal message remains Owned data capture output limit exceeded. The new frozen Error.cause contains only version/kind, bounded executable program identity with explicit truncation flag, triggering stream, observed cumulative raw bytes/logical lines, and unchanged caps. It contains no argv, environment, raw stdout/stderr or authority. A long program is represented by its bounded256-code-unit suffix and programTruncated true. This diagnostic does not authorize retries or larger captures.

The existing raw counter loop is extracted into one private function used by actual capture. It still counts both streams' raw bytes and starts a logical line whenever that stream's previous byte ended its prior line, preserving independent stdout/stderr continuation across chunks. The same strict greater-than byte-or-line predicate stops the child at the same point. Error.cause is diagnostic DATA only; first failure/control/cancellation/real child drainage remain owned by the same engine.

## Schema-first Actual Source Qualification

The closed OutputOverflow definition and three byte/line cases preceded the runtime change. The genuine child semantic RED completed terminal1: all four preexisting capture tests passed, then the new test failed because the old error had no frozen cause. The initial inline editing command's Python syntax error occurred before edits and is a harness failure, not semantic evidence.

Final stdio-phase --capture-strict session30590 completed0: three authored TypeScript roots with zero diagnostics, then6tests/0fail/133assertions in9.94seconds. The genuine Original is unchanged finite60000ms. Existing tests retain CrossSpawn/Ajv comparisons, exhausted/cancelled/copied controls, progress failures including undefined, native signal/status and output sink drainage. New tests run all three overflows through both separated and Native-style tail capture, obtaining actual failed child Original status1/null and frozen cause evidence. Each child verifies the old message and exact stream/count/cap values; the parent validates the closed diagnostic with Ajv.

Eight additional language-neutral raw chunk vectors cover line continuation, separate streams, CRLF splits, NUL fields, split UTF8 bytes, empty chunks and blank lines. The test executes the exact private production counter body and compares it to independent SQLite byte counts and per-stream predecessor window queries. Scoped whitespace check with fsmonitor disabled completed0. The first strict attempt reported only an untyped dynamic helper callback; its explicit own ScriptInvocation type was added before the final terminal0.

The existing generic EN/DE stdio-phase helper launch exposes --capture and --capture-strict through its argument input; no new executable script or TypeScript file was added. Prior stdio phase/ownership branches retain their behavior.

## Release Scope

The five files below are settled for independent nofollowFD/hash review. This qualifies bounded diagnostics and counters on real Source children, not Native recovery or an attribution of91721. Mapping/lease six-file release remains unchanged. Graph/Products administration is still pending and is not accepted by this gate.

## Settled Five-file Manifest

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts` — 20939B SHA256 `57b283149344dd307c05fe8b228ec217cd37e94eaf8b26e09d9b67ff75617cab`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧬️schema/🔣️.json` — 2358B SHA256 `2df7576e05d9d10f087a90e6269dd1484acd7cbec145c0bf687c3808377af9a1`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🟦️.ts` — 12094B SHA256 `3e657464c79d0acaa083c53a592ec16dbf106f3fb61ce6f48247d44c18e16051`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🔣️.json` — 3713B SHA256 `61b21cf2d68779f5b09bf2734395674f3b134dd1a199fba137147a60c5ec66e4`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/stdio-phase/📜️script.ts` — 2800B SHA256 `fa63059809bc88f04fe3dfb575b47955d9fa73684d407c94e3ca8ab8a32014a0`
