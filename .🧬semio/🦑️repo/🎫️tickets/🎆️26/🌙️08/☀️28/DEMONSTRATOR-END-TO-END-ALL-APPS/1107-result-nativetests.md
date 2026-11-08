STATUS: BLOCKED (not red): last proven GREEN = 1105b nativetests-93; 1107 rerun refused before compiler: native-owned test closure 272244 B > 262144 (value lane asked to split). UNVERIFIED on the current tree.

- Last proof: C/🗑️generated/1105b/nativetests-93.txt (2026-11-06 22:44, rc 0): native-owned all targets, persistent 25/25, snapshot-read 23/23, value-tests 62/62, value-tests-2 24/24; 15 binaries ok, 0 failed. The 3/5/7/2 failure counts from 1105b are stale (cleared in runs 69-93).
- 1107 reruns (`tst.sh`, five crates, --lib --tests): 101/102 CargoLockRefused (lock, fixed by value 04:23); 103 bad flag (`--keep-going` unsupported by test); 104-118 `Rust crate Source refused: semio-framework-job-context` (park 13686 > 4251, then owner/🦀️.rs 3702 > 940). Not a fault in my crates; runtime/jobsplit owns it.
- Auto-retry loop running (C/1105b-tools/nativetests/run105.sh, retries every 4 min until the refusal clears); final marker C/🗑️generated/1105b/nativetests-final.marker, log number in nativetests-final.n.
- No production or test edits made this wave.

## Update (nativetests-119)
- job-context gate cleared. New block: semio-framework-value-native-owned test closure 281695 B > maxSourceBytes 262144 (refused at 🔄️map/🦀️.rs, nativetests-119). Measured with C/1105b-tools/capsplit/closure.ts <Cargo.toml> testkit "" test.
- Fixed (lossless): removed duplicate lib mount of 🗂️map/tests.rs in 🗂️map/🦀️.rs (also unused cfg(test) RetireOwned import); the same tests still run via the native_map_receiving wrapper. Closure now 272244 B, still 10.1 KB over. Remaining lib unit tests read private fields; root fix is a non-test crate split (value lane asked via message). 
- Retry loop restarted (run105.sh 130, retries every 4 min while job-context/lock refusals appear; note it stops on any other result).
