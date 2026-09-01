# Txn Command Close Native 55 Review

Reviewed packet: `🧪️txn-command-close-native-55/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}`. No files in that packet were edited and no Cargo/native command was run.

The Rust source is compile-shaped for the stated native RED: it keeps `Box<TxnCommand>` intact, measures `size_of_val(command.as_ref())` rather than allocator overhead, makes command release first, and leaves `completion` in the job. `PendingExternal` creates a real clone and confirms shared ownership. `PendingOwner` places a real nonempty `Emit::mutations` payload in the completion and retrieves it through `take_emit`; this proves payload reachability after command release, but does not claim final completion close semantics.

One concrete schema gap remains: `minItems`/`maxItems` enforce six rows but do not enforce the six distinct required IDs. Six duplicate valid `before-begin-close` rows satisfy the current schema and Rust `check` only detects duplicate IDs for the selected test call, not missing other IDs. Add `uniqueItems` is insufficient because IDs could differ while coverage is still absent; the independent controller should assert the exact six-ID set, or schema `contains`/`allOf` should require each ID. This is a test-closure defect, not a production behavior finding.

No allocation claim is made for `ArtifactToolCompletion`: external clone and pending payload behaviour are distinguished, while final-owner close remains explicitly unresolved.

## Independent Oracle

`🧪️txn-command-close-native-55/📜️script.ts` is the independent strict-Ajv oracle. Its `validate` mode requires exactly six distinct case IDs, one of the two allowed closed outputs for every row, and retained completion ownership; it captures packet hashes before and after. The scoped Nx/Bun run retained [run-0phuNA](🧪️txn-command-close-native-55/🧫️run-0phuNA/🔣️.json): six cases, no native execution. The optional `mounted` mode compares only exact source/schema/vector bytes against the proposed transaction test location and makes no mount or copy.
