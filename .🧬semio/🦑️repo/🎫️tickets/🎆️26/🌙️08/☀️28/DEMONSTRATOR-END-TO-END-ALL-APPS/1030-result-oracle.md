# 1030 oracle result
STATUS: DONE. No cargo run. All new tests green; existing suites re-run green.

## Per feature
| feature | existing | gap closed | oracle |
|---|---|---|---|
| (a) headroom `classPriors` | `...unit-cache/🧹️gc/🏦️headroom/🧪️tests/{🟦️.ts,🐍️.py,🧫️fixtures/🔣️.json}`: `priors` vectors (cold/warm component, general cargo, prefix must lead) checked in TS and in the Python oracle; `bun test ./🟦️.ts` 11 pass | none (already schema+fixture, 2 languages) | independent Python implementation (`🐍️.py`); no third-party library exists for this policy |
| (b1) OutputDirectories exclusion | `🟦️bun/🧪️tests/🟦️.ts`: fast-glob + `git check-ignore` | new `🟦️bun/🧪️tests/🐍️.py` (3 tests): schema lists every ignored name, members = stdlib `glob` outside output dirs == fixture `authored`, `git ls-files --others --exclude-standard` in a scratch repo with the repo's own `**/pkg/`, `**/🕸️bindings/**` rules keeps exactly the authored dirs | fast-glob (TS), git (TS+Py), stdlib glob (Py) |
| (b2) hoisted scope-root resolution | `💾️storage/🧪️tests/🔭️providers/🟦️.ts` (Node `createRequire`, needs repo test runner env) | new `🔭️providers/🐍️.py` (2 tests) rebuilding the `hoisted` fixture row on disk: scope-root-owner model gives the expected edge; Node `require.resolve` agrees. Finding: Node is NOT misled by the dangling stray link (it skips unreadable entries), so the stray only matters to the Bun/ancestor-climb policy, not to Node; the test asserts agreement only | Node `createRequire` |
| (c) stdio-semio codec | `🧪️tests/🎒️record-document/{🟦️.ts,🐍️.py,fixtures}` | none | noble BLAKE3, system zlib, Rust-written packs. Re-run: bun 13/13 (needs `--timeout 120000`; the default 5 s flakes on the 39-pack round trip under load: 7.3 s), Python 13/13 (157 s). `spec.sh` is the cargo Rust law, not re-run |
| (d) energy schema hash | `...📦️pack/🧪️tests/{🟦️.ts,🔣️.json,🦀️.rs}`: noble BLAKE3 of canonical bytes == fixture | new `...📦️pack/🧪️tests/🐍️.py` (4 tests): canonical bytes rebuilt from fields, codec's independent BLAKE3, Rust `SCHEMA_HASH` constant parsed from `🦀️.rs` == fixture digest, noble via bun | @noble/hashes |
| (e) mutations derive | Rust only: `roster_uniqueness_hashes_a_bounded_tail...`, `mutation_descriptor_roster_failures` | new fixture `✨️derive/🧪️tests/🧫️fixtures/🧭️mutation-roster/🔣️.json` + `🧭️mutation-roster/{🟦️.ts,🐍️.py}` (2 tests each): 48-byte tail buckets collide by construction yet full-identity comparison reports `(first 7, index 299)`; the 306 energy leaf records each hold exactly the 14 required fields (+ optional `editable`), `owner` equals its own directory, and owner/semanticKind/textOpcode/binaryTag/aggregateVariant/displayName are unique | fast-glob (TS), `os.walk` + set/map (Py) |

## Open
- (d) The strongest law (derived owned spec hash == `SCHEMA_HASH`, `bestest-load`) is Rust-only and was NOT run (energy lane t5 still pending; one-field graph, so the fixture covers the digest, not the derived graph). An energy `🗺️record-spec/🔣️.json` (Rust derive output) would let the stdio-semio TS/Py codecs recompute it language-agnostically; needs a cargo run, not started.
- (e) The Rust `mutation_descriptor_roster_failures` compares the const-evaluated DESCRIPTORS to the records; the new data tests cover only the record side. Rust could additionally consume `🧭️mutation-roster/🔣️.json` (not edited: mutderive lane active).

## Run
- `bun test ./🟦️.ts` in `🧭️mutation-roster` (2 pass), `python3 🐍️.py` in it, energy pack, bun workspaces, providers (all OK).
