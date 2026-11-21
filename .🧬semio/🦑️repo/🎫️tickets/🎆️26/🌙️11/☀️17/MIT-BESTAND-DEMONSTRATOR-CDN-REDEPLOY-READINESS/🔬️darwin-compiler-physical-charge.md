# Darwin Compiler Memory Charge

Apple’s \`RUSAGE_INFO_V2\` gives separate resident and pressure-accounted byte counts. The rustc scheduler previously streamed resident bytes alone. On 2026-11-20, an SDK C probe and the exact Rust production probe both observed the following active compiler processes. The matching C and Rust layout measured 160 bytes, resident offset 64, and footprint offset 72.

| Live compiler | PID | Resident bytes | Charged bytes |
| --- | ---: | ---: | ---: |
| Procedural | 25811 | 395,034,624 | 5,239,804,872 |
| Demonstrator | 45697 | 838,615,040 | 5,202,188,024 |

Both charged samples exceeded the broker’s nominal 4.9 GiB budget while RSS was below 0.8 GiB per process. The native Darwin wrapper now streams \`ri_phys_footprint\` alongside resident bytes. Linux and Windows use their existing RSS or working-set sources in both fields. The broker uses the sum of measured charges for pressure freeze and the greater of predicted or measured charged bytes for each running reservation. A measurement reevaluates admission immediately against the last observed swap rate. Progress separately labels RSS and measured charge. The historical process peak and measurements explicitly named RSS retain their resident meaning. The existing budget, swap hysteresis, first-grant policy, and rustc flags stay in force.

## Verification

- The real socket regression reproduced the old failure before the fix. It reported 0.5 GiB RSS and left admission unfrozen for the 5.3 GiB charge under a 4.9 GiB budget. The language-neutral replay now also exercises an unfrozen under-budget charge that exhausts room for a new waiter, a charge that freezes admission, a hysteresis hold, and a subsequent release. Its independent SQLite decision oracle agrees. The focused broker suite passed 10 tests and 174 assertions, with six unrelated integration cases filtered.
- The production Rust wrapper passed its standalone suite of 13 laws. A second standalone build of the exact Rust process probe agreed with Apple SDK C on the layout and both live compiler readings. Temporary evidence is under \`🗑️generated/darwin-physical-charge-*.log\`.
- Verification sampled active compiler processes read-only. No module build, job intervention, or optimizer replay was performed. Runtime charge sampling improves admission on subsequent wrapper invocations. No historical physical charge is inferred from RSS peaks.
