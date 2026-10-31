# Native Fixture Retirement Scan

The actual SDK four-package batch stopped before runtime at host-configuration test E0507. Its cleanup helper now owns the actual OriginalJobFixtureCaller and retires source, output and decoder before consuming that caller. Both fixture callers move their same real owner into cleanup; assertions and grants are unchanged.

A bounded current-source scan covered the four selected SDK owners, the original-job declaration subtree containing its external test mounts, and the Layout artifact test/source subtree: 591 Rust files and 41 direct caller.retire call sites. No remaining helper parameter borrowed as &mut OriginalJobFixtureCaller consumes that same parameter. Selected scrub-host and typed-ops finish helpers already own their callers by value; host testkit declares its own local owned caller. The two ownership leaf test mounts retain their existing actual cleanup paths. This source check is not a native pass.

Only the observed host fixture file changed. Original SDK log sdk-original-native-1.log remains pre-runtime failure evidence; the retry requires coordinator release and returns to HOLD after terminal.
