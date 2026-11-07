STATUS: UNDER CAP (static, 259485 B / 262144 B, 22 files, margin 2659 B); wasm --lib check UNVERIFIED (waiting for admission-ok)

# 1105b Session source cap

- Native-equivalent census (`C/1105-native-source-census.mjs <session Cargo.toml> s1105b --normal`): 259485 B in 22 files. Literal module census (`C/1105-rust-census.cjs`) agrees: 259485 B / 22 files, zero missing. Output `C/🗑️generated/1105-rust-census-s1105b.json`.
- The 266605 B figure is obsolete: the flow-owned relocation of WindowKey producer, cold hash, latch kernel and invocation dispatch encoder into the source owner is already mounted. No further extraction or public path move was needed by this lane; no edits made.
- Margin is 1 %. Any growth of session-closure files (eval-session-state 37401, preview 25655, baseline-tick-sync 22829) by more than 2659 B re-breaks admission; flow lane must re-run the census before adding code there.
- Cap untouched.
