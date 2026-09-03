# 🐛️ Codemod spliced `.await` into doc comments — 24 sites, compiler-invisible

Found by a peer session while reading `📡️replication/📡️wire/🦀️.rs:1901`, then confirmed repo-wide by me.

Example (prose, not code):
> `🗺️ Own persisted-local selection (`Interaction` history lane).await + ephemeral-local hover, keyed by domain id`

An automated async-conversion codemod matched inside `///` doc comments and inserted `.await` into
English sentences. **24 occurrences across `🧰️framework/`.**

Find them:
    grep -rn '///.*)\.await' --include='*.rs' 🧰️framework | grep -v target

## Why it matters even though it compiles
- Zero compiler signal — doc comments are not type-checked, so this will never surface in CI.
- It corrupts the documentation that this ticket has repeatedly had to rely on. Stale/false comments
  have misled this ticket at least six times: one claimed types had "migrated off serde" when they had
  not; `📡️replication`'s Cargo.toml claimed "3 blockers" when removal actually produced 206 errors
  across 9 files; a kernel pass found 4 of 4 named blockers false; and a `🕸️graph` comment claimed a
  field needed no custom hook when removing it broke 3/183 fixtures.
- If one codemod matched inside doc comments, others may have too. Worth a wider audit of automated
  edits than just this pattern.

NOT fixed in this pass — flagged for a dedicated cleanup so it does not get mixed into the serde
migration diff.
