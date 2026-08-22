# P9w Stdio BLAKE3 Boundary

## Outcome

Stdio no longer declares or calls `blake3` directly. Its two production consumers now use the repository-owned `semio-framework-hash` boundary while preserving both externally visible representations exactly:

- `semantic_fingerprint` still returns the same raw 32 BLAKE3 bytes.
- BREP geometry handles remain the same 64-character lowercase hexadecimal text.

No framework hash source, stdio test-only migration residue, or unrelated artifact implementation was changed in this packet.

## Implementation

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`
  - removed direct `blake3 = "1"`;
  - added the internal `semio-framework-hash` path dependency.
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`
  - `semantic_fingerprint` serializes exactly as before;
  - hashes through `semio_framework_hash::hash_bytes`;
  - decodes that boundary's canonical lowercase hexadecimal output back to the existing raw-byte return contract with a private, allocation-bounded decoder.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs`
  - BREP handle minting now uses `semio_framework_hash::hash_bytes` directly.

## Raw-Parity Evidence

Before the source change, the linked production stdio harness recorded:

```text
semantic_fingerprint=[250, 74, 204, 25, 91, 24, 60, 7, 57, 60, 240, 40, 61, 163, 253, 76, 239, 198, 182, 56, 230, 23, 142, 223, 149, 26, 49, 145, 142, 66, 200, 236]
brep_handle=a63eafbdcde2275214b073904583c60e7888e7c567c3fe41c5051f1bcc21dece
```

Evidence: `📝️p9w-blake3-before.txt`.

The ticket-local `p9w` binary links the changed production stdio crate and asserts those exact values. It exits zero and prints `blake3_boundary=pass`; its after values are byte-for-byte identical. Evidence: `📝️p9w-blake3-parity-final.txt`.

Command:

```sh
CARGO_TARGET_DIR='.../PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-stdio-compression' \
  cargo run --manifest-path '.../🧪️p9v-stdio-compression-harness/Cargo.toml' --bin p9w
```

The broader P9v linked compression binary also reached and passed both new hash assertions. On the saturated concurrent fleet it then stopped at its unchanged adversarial compression watchdog (22.479 ms, and 46.348 ms on a warm retry, against 8 ms). The watchdog was not weakened or masked; the previously completed P9v evidence remains the authoritative isolated compression timing result (1.939 ms). Logs: `📝️p9w-blake3-after.txt`, `📝️p9w-blake3-after-warm.txt`, and `📝️p9v-stdio-compression-harness3.txt`.

## Compiler Gates

All Cargo commands used the ticket-local `🧪️target-stdio-compression` target directory.

```sh
cargo check -p semio-s-plugin-stdio --lib --message-format=json
cargo check -p semio-s-plugin-stdio --lib --release --message-format=json
cargo check -p semio-s-plugin-stdio --lib --target wasm32-unknown-unknown --message-format=json
```

Results:

| Gate | Build | Errors | Existing warnings |
|---|---:|---:|---:|
| native | success | 0 | 658 |
| release | success | 0 | 658 |
| wasm32-unknown-unknown | success | 0 | 647 |

Structured logs: `📝️p9w-stdio-blake3-native.json`, `📝️p9w-stdio-blake3-release.json`, `📝️p9w-stdio-blake3-wasm.json`; exact aggregate: `📝️p9w-stdio-blake3-gate-counts.txt`.

## Dependency And Diff Census

- `📝️p9w-stdio-blake3-census.txt`: zero direct `blake3` manifest/source hits in the changed production boundary; exactly two `semio_framework_hash::hash_bytes` calls and one internal dependency row.
- A whole stdio tree search finds only three test strings containing the token `semantic-blake3`; these are forbidden-field assertions, not imports or calls.
- `📝️p9w-stdio-blake3-direct-tree.txt`: depth-one normal/build tree contains `semio-framework-hash` and no `blake3`. BLAKE3 remains transitively owned behind the framework hash module, as intended.
- `📝️p9w-stdio-blake3-diff-check.txt`: scoped `git diff --check` exits zero.

## Explicit Test-Wall Attribution

The monolithic stdio test target was not repaired, suppressed, or reclassified by this packet. The completed P9v structured boundary still reports exactly **898 primary test-only diagnostics across 241 file/code groups**, with zero diagnostics in the completed RFC1950 compression IO implementation. Those broad decorative-async and moved-testkit diagnostics remain independently owned and are preserved in:

- `📝️p9v-stdio-lib-tests-final.json`
- `📝️p9v-stdio-lib-tests-final-errors.tsv`

The P9w production native/release/WASM gates and linked raw-parity binary are green independently of that test-only wall.
