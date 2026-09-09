# Jack Baseline Runtime

The compiled unboxed baseline ran the semantic law and failed at result equality after four earlier cases passed. The layout law was then run from that exact SHA-256 verified executable.

{"result":{"status":101,"signal":null,"reason":"exit","stdout":"\nrunning 1 test\ntest executor::tests::query_preparation_layout_matches_neutral_budget ... FAILED\n\nfailures:\n\nfailures:\n    executor::tests::query_preparation_layout_matches_neutral_budget\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 196 filtered out; finished in 0.00s\n\n","stderr":"[DEBUG] Query preparation step occupies 992 inline bytes; neutral maximum is 16\n\nthread 'executor::tests::query_preparation_layout_matches_neutral_budget' (9590976) panicked at ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs:383:5:\nquery preparation must transfer a compact execution owner\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n"},"executable":{"package":"semio-s-artifact-trinity-jack","target":{"kind":"lib"},"path":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/debug/deps/semio_s_artifact_trinity_jack-ea8ab2530db7a20b","sha256":"3ee9857a4999945dc8cdd0fa14632e0fea7789744d4c2aa721c2b2bfdc7f6586","byteLength":40977792}}

```

running 1 test
test executor::tests::query_preparation_layout_matches_neutral_budget ... FAILED

failures:

failures:
    executor::tests::query_preparation_layout_matches_neutral_budget

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 196 filtered out; finished in 0.00s

[DEBUG] Query preparation step occupies 992 inline bytes; neutral maximum is 16

thread 'executor::tests::query_preparation_layout_matches_neutral_budget' (9590976) panicked at ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs:383:5:
query preparation must transfer a compact execution owner
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```
