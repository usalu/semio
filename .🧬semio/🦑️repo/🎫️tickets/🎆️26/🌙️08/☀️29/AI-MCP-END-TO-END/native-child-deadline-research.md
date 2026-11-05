# Native Child Deadline Evidence

## Actual outcomes

Native runtime request session12882 ended as owned MCP ok:false/exitCode1 after5539911ms, response generated/mcp-38664.json. Guest-contract resident-output request67933 ended ok:false/exitCode1 after4866478ms, response generated/mcp-48596.json. Both refused before their requested native tests with Original child phase deadline exhausted. Runtime refusal was at process/owned-execution/capture, resident at native-build/units/capture. These are neither client120min timeouts nor Rust law failures.

Wake request3600 ended ok:false/exitCode1 after6110957ms, response generated/mcp-41588.json. Its actual compiler child returned101. Owned compiler fingerprint recovered E0599 in job/context/encoding.rs14: NativeEncodeControl::new_scoped_source absent from captured encoding unit. Current authored API already contains that method. This is a mixed concurrent-source dependency snapshot failure, not a wake law result. Core request91770 remains alive; no duplicate launched.

## Current authored authority

Default build invocation maximumElapsedMilliseconds is0 (ordinary progress-managed invocation). Build preparation default is21600000ms(6h). Native-child policy idle3600000ms(1h), nonrenewable maximum14400000ms(4h)+600000ms perdeclaredunit capped86400000ms(24h). The120min ticket client timeout is separate and grants no process authority. These are current schema values, not a claim about the older captured snapshots. No budget was increased during this investigation.

Current routing/invocation/process scriptResourceProcessEnvironmentV1 first authenticates actualcontrol and computes active-time remaining. It retains carried serialized bounds, actualparent restrictions and priorprojecteddeadline, while projectedRemainingV1 credits only privately measured admissionwait once and preserves monotonic elapsed. Current finished-run stack shows an older multiline projection using deadline=Math.min(deadline,now+floor(available)), differing from the current privately credited projection. A stale absolute serialized bound can expire during creditedqueuewait while realcontrol remainslive, producing exactly the separate childphase refusal. Exact priorcarrieddeadline/creditedwait numericalfacts were not persisted and cannot be honestly recovered from the final diagnostic.

## Verification and next execution

Executed existing owned current-source admission/projection suite:10pass58assertions. It independently compares deadline equations with SQLite and Python, covers open/overlapping waits countedonce, narrowedwire preservation, retiredissuerdata, siblingcancellation, monotonicrollback and unchanged ordinarysequential scopes. No source/control mutation was needed. Next native retry should retain the same boundedpolicy and use existing SEMIO_PROGRESS_DEADLINE_TRACE=1 to publish declared phase maxima/cost; it must not widen, reissue or removeOriginal deadlines. Wait for actualpriorlineage drain before retry.

## Preparation costs

Wake immutable48104 phase ledger: store0→278094ms; inputs941415→3694136ms (2752721ms); graph3694138→3795389ms(101251ms); source3795389→3878716ms(83327ms); capacity3927774→4653076ms(725302ms); child4653913→5209623ms(555710ms); consume5209678→5249438ms(39760ms). This disproves attributing its whole65minpreparation to ordinaryRustsource reading. Inputs includes compilerconfiguration/planning/foreignsource inventory and tooling/storage preparation; queue intervals are separate.

Actual source-admission owner is process/artifacts/native-build/source/admission: cfgcache peractualargumentvector; memo recall reuses wholeunit inputs; workspace Source closures retain existing finite32file/262144byte/4000line caps; durable storage rechecks are gated15seconds and mandatory atcompletion. Foreign inventory owner native-build/units/capture hashes everyselectedforeignpackagefile with stamped digestmemo and package-level deduplication. Core latestobserved source-admission cfgunit7/60 had0readOperations and46rememberedfiles479397bytes; that proves memo reuse of those files, not largeactiveSource reads. Further performance attribution requires actualstageclock/profiler evidence, not blanket cap expansion.
