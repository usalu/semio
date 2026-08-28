# Common Kernel Return Source Entries: Compile RED

Canonical `@semio-tech/framework-rs:test-wire-retirement-native --args='--lib return_source_entries_ -- --nocapture'` completed exit 1 against the existing shared native target.

```text
error[E0432]: unresolved import `super::return_source_entries`
error: could not compile `semio-framework` (lib test) due to 1 previous error; 19 warnings emitted
NX Running target test-wire-retirement-native for project @semio-tech/framework-rs failed
```

Raw: `🧪️member-kernel-return-source-entries-red-r1-native-2026-08-27.txt`.

This is the intended missing production API for Dag's four schema-first tests. No native test executed. Dag has been given source release to implement the module. No UI/Plugin behavior or guest readiness follows from this result.
