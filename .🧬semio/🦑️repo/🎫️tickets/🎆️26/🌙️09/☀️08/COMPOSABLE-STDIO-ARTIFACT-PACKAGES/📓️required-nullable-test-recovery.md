# Required Nullable Test Recovery

The recovered native retry log `owned-native-enospc-retries-current.txt` records E0369 in the serde oracle assertion. The test compared two `Result<RequiredNullableField, serde_json::Error>` values; serde_json errors do not implement equality. This was a test compilation error, not evidence that required-nullable decoding failed.

The coordinator changed only the successful-null oracle assertion to require successful serde decoding with `expect` and compare the decoded struct. Omitted-key rejection assertions and the independent FromValue result remain unchanged. The neutral omission/null cases still compare our behavior against serde. Scoped `git diff --check` passed.

The ordinary Nx target retries exactly `--test flatten_with_skip required_option_rejects_omission_and_accepts_explicit_null_like_serde`, with offline shared Cargo output and jobs2. Raw receipt: `value-derive-required-nullable-recovery-4.txt`. Runtime result is pending.

## Files

- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧪️tests/🪗️flatten-with-skip/🦀️.rs`

## Native Result

The exact ordinary Nx retry completed with exit0. Nextest executed the selected test:1 passed,14 skipped,0 failures (0.235s native runtime). The total Nx duration4m22s includes compilation/queue. This proves omission rejection and explicit-null acceptance agree with serde for the retained required-nullable fixture.
