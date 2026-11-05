# Native R3 Input Preparation Read

The sole run remains execution handle 4662, owned launcher PID 54688, repo PID 25420 and build door PID 46192. This is a read-only investigation; no process, source, policy or queue was changed.

The phase ledger records inputs begin at 1,458,606 ms and no inputs-ready marker. The owned launcher's `🚪️launcher/🟦️.ts:233` sets inputs-ready only inside the callback of `withRepositoryCargoInputsV1`, so the run has not yet entered graph/source admission or final compiler scheduling.

The input function at `📥️dependencies/🟦️.ts:86` performs lock receipt/manifest admission, retains configuration views, admits exact configured arguments, and establishes registry-ready custody before invoking that callback. The normal snapshot at about 2,946 s outer runtime shows configuration-view:50 shared held by PID 54688 for 245 s. Earlier scheduler preparation held configuration-view:48. The new reader proves effective configuration/flags/native compiler capture and immutable publication have completed for another real configuration scope; it is not a native test result.

The configuration reader is acquired at `⚙️configuration/🟦️.ts:55`, after configuration capture and publication. It then awaits storage recheck, selected Cargo/profile source rechecks, native compiler recheck, and physical immutable-view read before entering its body. Later input admission and registry-ready preparation also execute under that reader. Without an exposed per-await trace, the snapshot cannot identify which exact awaited I/O call is active. The proven interval is configuration-reader preparation/input admission before the inputs callback; it is not an observed compiler-capacity wait.

Fresh normal facts contain no lease waiters, no capacity waiter for this tree, and no cycles. The only descendant commands are ordinary unit-cache GC sidecars (34280, 32972 and 51852); there is no actual cargo/rustc child in that snapshot. Launcher RSS is 456 MB and the single-point CPU sample is zero. Those facts do not establish a hang. Source and original handle remain preserved.

Source references are beneath `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`:
- `🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:233`
- `🏃️process/🦀️cargo/📥️dependencies/🟦️.ts:86`
- `⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts:55`

Normal snapshot: `🗑️generated/headless-r3-phase.json`. Native log: `🗑️generated/native-headless-r3.log`.


## Queue-Front Ownership Check

At Root request, normal ancestry proved capacity waiter PID 45904 is foreign to neutral handle 4662: 45904 -> 36520 -> 30896 -> 38276 -> 54128, with none of own repo 25420 / door 46192 / launcher 54688. Its bound command is a registered rustfmt syntax check from AI-MCP-END-TO-END wrapped by its owned Bun helper. The capacity row labels it cargo, needing 3,584 MB and waiting 2,187 s for load; this does not make it our nested Cargo request. No foreign process was changed and no deeper investigation was performed.
