# Browser Shell Birth and Setup Drain

The independent audit found a real Source gap: the actual Hub Shell was spawned before the timer's second control check and before installing its terminal observer. A deadline crossing in that interval could reject the requested phase while its child remained alive. This report supersedes only the earlier staging lane's child-setup boundary; its earlier successful gates retain their historical Source epoch.

The actual requested-phase callback now installs both error and close observers as its first synchronous statement after `spawn`. All later daemon/abort/stream/timer/progress setup runs inside the actual catch/finally boundary. Cleanup closes progress births, clears the timer, requests termination if the child is still live, awaits the exact terminal promise, joins the already accepted progress promise, and removes the abort listener before the phase returns. A separate boolean preserves the first primary even when it is `undefined`; later termination, progress or listener cleanup errors cannot replace it. Diagnostic tail capture now happens inside the guarded and byte/line-bounded stream observer, with no post-start unguarded stream setup. Normal Shell shutdown still awaits this lifetime before relay release, and the paid artifact reader remains outside the entire browser consumer.

No new Original, phase factory, authority registrar, provider, clock budget, process-policy exception or free-path staging API was added. The changes are confined to the real Hub callback plus a closed neutral corpus/schema and its existing ordinary browser-host Source test root. The actual reusable close helper moved by CPU is `Core filesystem/observation/📁descriptor/🟦.ts`; the audit's reference to the general observation owner was imprecise. Neither Core owner was edited here.

## Qualification

The first test attempt43473 had an inline test-schema syntax error and is retained as a failed authoring attempt. After moving that schema into its own actual JSON owner, the genuine pre-implementation RED43127 (`browser-shell-setup-red2-prototype.log`) completed shared130pass/1fail/2195assertions. Its actual TypeScript AST inspection failed because immediate post-birth terminal observation was absent. The production fix was authored only after this RED.

The closed language-neutral corpus contains Error, undefined and success event traces. Ajv validates the corpus; independent SQLite ordering checks require observation immediately after birth and close before phase retirement. The TypeScript AST law inspects the actual Hub requested-phase body, requiring immediate error/close observation, a guarded setup block, and explicit terminal/progress joins in finally. It does not fabricate a child, Native provider, store or control.

Current Source gates prototype69432 and release81069 both collected exit0. Per mode: Core2/128, SDK archive6/40, Print48/713, shared131/2202, default Cargo4/301. Full unchanged94-root strict prototype96120 collected exit0; its capture started before the final diagnostic-tail consolidation, so it retains that small moving-epoch caveat. Current full release strict39923 collected exit0 with no diagnostics. No roots, strict flags or authored120s budgets changed.

This is an actual Source/control-flow regression proof, not a runtime birth-to-deadline process injection. No browser, listener, Native image, Cargo, compiler, SDK publication or maintenance ran. A future coordinated actual small owned-child setup-failure probe must still show the real close event before requested phase retirement for Error/undefined/cancellation. This Source change does not claim arbitrary OS descendant termination/join, whole Hub lifetime correctness, or closure of previously documented prepared-GIS/peer-data funding frontiers.

## Current Source Hashes

| Path | SHA256 | Bytes | Absolute UTF16 Path Units |
| --- | --- | ---: | ---: |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | `001462c193dbc513ab221871a8a65da1d08fe5d8858d5b75329f3782fbce1fea` | 1280054 | 67 |
| `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🧪️tests/🟦️.ts` | `87a857abd599762bdf202b5a57632b51a25c895dcd802867063b9e346e891c9c` | 18161 | 83 |
| `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🧪️tests/🔚️terminal/🔣️.json` | `93e3b1d3df2006669145501c8130ec1c08c69cda299d19f6dd912beb25cca5c6` | 732 | 97 |
| `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🧪️tests/🔚️terminal/🧬️schema/🔣️.json` | `3abc6a848d541f464d40af5f75ff5d96bae53a6edc0dd38102e207c42f515278` | 976 | 107 |

## Final Collected Evidence

All own handles are terminal. No further run is queued. The prototype strict timing caveat above remains explicit; both actual strict commands retain all94 explicit roots and their transitive imports.

| Retained Log | SHA256 | Bytes |
| --- | --- | ---: |
| `browser-shell-setup-red-prototype.log` | `7adec061652660899f457cb78a7cf01df243d8bc6de11efe448d6433b641438d` | 80250 |
| `browser-shell-setup-red2-prototype.log` | `6276a1e4e1edb60f35b0a29797965f4239f933e27b3a77a7abba16ffac7ea038` | 80251 |
| `browser-shell-setup-green2-prototype.log` | `87d9b29ea2365bf16e8486317158f35d6e7edab8f0a558a686a167e45d7be276` | 79986 |
| `browser-shell-setup-green-release.log` | `052fc0f8e27465e834f0faa936dacfd8586455ca1db5a01870b8f9cbae974728` | 80106 |
| `browser-shell-setup-types-prototype.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | 0 |
| `browser-shell-setup-types-release.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | 0 |
