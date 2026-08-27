# Sol Zero-Dependency Compiler Validation — 2026-08-27

## Scope

This leased validation covers only the completed `byteorder`, `fast-glob`, `@types/semver`, and Hound/owned-PCM16 packets. No dependency baseline or unrelated source was edited.

## Formatting

```text
rustfmt --edition 2021 <framework retained-command source> <stdio audio oracle source> <WAV subset oracle source>
  exit 0
```

Only those three Rust source paths were passed to rustfmt. The framework source was already clean relative to its staged state. Rustfmt made only formatting changes in the two stdio Rust sources: three added/one removed line in the WAV subset adapter and three added/fifteen removed lines in the shared audio boundary. No manifest, lock, fixture, TypeScript, feature, or registry file was formatted.

## Frozen-lock integrity

```text
cargo metadata --locked --no-deps --format-version 1
  exit 0

cargo metadata --manifest-path <stdio-oracle>/Cargo.toml --features oracles --locked --no-deps --format-version 1
  exit 0; 24 optional oracle dependencies; no hound feature or dependency

bun install --lockfile-only --frozen-lockfile
  exit 0; Bun 1.3.14 accepted bun.lock; 1555 packages
```

The Bun command reported `Saved lockfile`, but produced no unstaged `bun.lock` delta. The repository already contained a staged lock change owned by the packet/coordinator; validation did not use a modifying Git command.

## Focused Cargo validation

```text
cargo check -p semio-framework-plugin --locked
  exit 0; finished dev profile in 6.27s

cargo test -p semio-framework-plugin checkpoint_binary_matches_schema_fixture_and_owned_oracle --locked
  exit 0; 1 passed, 0 failed, 407 filtered out

cargo test -p semio-framework-plugin owned_little_endian_oracle_preserves_every_hostile_byte_lane --locked
  exit 0; 1 passed, 0 failed, 407 filtered out

cargo check --manifest-path <stdio-oracle>/Cargo.toml --features oracles --locked
  exit 0; finished dev profile in 7.63s

cargo test --manifest-path <stdio-oracle>/Cargo.toml --features oracles --locked owned_pcm16_
  exit 0; 4 passed, 0 failed, 374 filtered out
```

The four PCM16 tests cover the frozen Hound byte golden, ordered odd auxiliary chunks, hostile framing/format fields, incomplete frames, and invalid chunk identifiers.

Warnings were not masked. The framework package emitted its existing broad warning inventory, including 408 lib-test warnings and a future-incompatibility notice for `semio-framework-plugin`; examples include unused/dead items, unnecessary qualifications, unused futures/results, redundant clone, and dropping a `Copy` value. The isolated stdio crate emitted exactly three unrelated existing warnings: two deprecated `quick_xml::Attribute::unescape_value` calls in SVG/markup sources and one unused DOCX `RELS_CONTENT_TYPE` constant. No warning names `byteorder`, `hound`, or either owned seam.

## Relevant Nx validation

```text
bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache
  exit 1; target hard-killed its Bun test process at 30,000ms after 52 displayed passes and no displayed assertion failure

bun nx run @semio-tech/repo-lib:test-transaction-v2 --skip-nx-cache
  exit 1; its aggregate hard limit fired at 14 seconds while all displayed shard assertions were passing; shard 1 completed 1 pass/0 fail and shards 2-4 reported completion around 12 seconds

bun nx run @semio-tech/repo-test:lint --skip-nx-cache
  exit 1; two TypeScript errors in framework UI styling: `ImportMeta.env` at line 594 and `ImportMeta.glob` at line 916
```

These are genuine target-level failures and are not claimed as passes. The first two are fixed-budget exhaustion rather than a discovered assertion mismatch. The third reproduces the pre-packet Vite ambient-type mismatch already isolated by the direct package typecheck: no Semver type or resolution error was reported.

The earlier focused direct Bun tests remain the acceptance evidence for owned filesystem discovery and transaction-v2 discovery. The prior focused repo-test registry suite also remains a genuine broader blocker: repository contribution discovery returned zero oracles even though the stdio root manifest directly contains its remaining registry entries.

## Fresh dependency counts

```text
bun ./📜️script.ts verify dependencies summary --format json
  exit 0
```

| Ecosystem/measure | Fresh count |
| --- | ---: |
| Rust raw / literal external | 75 |
| JavaScript raw | 69 |
| JavaScript corrected / literal external | 66 |
| Python literal external | 15 |
| Total raw | 161 |
| Total third-party | 159 |
| Total first-party | 2 |
| Total corrected / literal external | 156 |
| Total production-reachable | 106 |

The summary contains zero oracle conflicts, zero toolchain conflicts, and zero unauthorized toolchain rows.

## Conclusion

Both Rust leaf replacements pass their exact package checks and golden/hostile test filters under frozen locks. The dependency inventory remains at the expected post-Hound counts. Relevant Nx entrypoints were exercised and their three existing budget/ambient-type failures are recorded without suppression.
