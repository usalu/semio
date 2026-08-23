# 🏛️ Delivered architecture

## What was built

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/     ← the new testing domain
├── 🧬️schema/🔣️component.json                        TestCasePlan · TestResult · OracleRegistry · marker · baseline
├── 📇️registry/
│   ├── 🔣️component.json                             approved oracles + recorded no-oracle decisions
│   └── 🔒️migration.json                             shrink-only legacy backlog + owner status ladder
├── 🧬️protocol/🦀️component.rs                        owned JSON + owned SHA-256 + plan/result types (zero deps)
├── 🏃️runner/🦀️component.rs                          Rust host runner + adapter registration API
├── 🔮️oracle/🦀️component.rs                          the ONLY place pdf-writer/lopdf are linked (feature `oracles`)
├── 📦️packages/
│   ├── 🟦️typescript/📦️index.ts                      the coordinator: taxonomy, Gherkin, discovery, fixtures,
│   │                                                 comparison profiles, results, contract, clean, dependencies
│   ├── 🟦️typescript/🏃️host.ts                        TypeScript native host
│   ├── 🟦️typescript/🧪️index.test.ts                  31 platform self-tests (522 assertions)
│   ├── 🦀️rust/                                      `semio-repo-test-host` (own workspace root)
│   ├── 🐹️go/🐹️host.go                               `semio.tech/repo/test`
│   ├── 🐍️python/🐍️host.py                           owns non-compose Python test config outright
│   └── 🔷️dotnet/🔷️host.cs                           `Semio.Repo.Test`
├── 🧫️fixtures/📄️protocol-vector.txt                 shared conformance vector
├── 🧪️tests/host-protocol-parity/                    5-language self-conformance case
├── 🔌️nx-plugin.mjs                                  one virtual Nx project per test case
├── 📜️script.ts                                      discover · contract · oracle · subject · parity · run ·
│                                                     report · clean · dependency · nx · doctor
└── 📋️project.json
```

## Execution flow, as implemented

```
component.feature ─┐
🧫️fixtures        ─┼─► taxonomy discovery ─► parsed TestCasePlan ─┬─► oracle host  ─┐
adapters          ─┘        (parsed ONCE)                          ├─► rust subject ─┤
                                                                   ├─► ts subject   ─┼─► owned TestResult (JSONL)
                                                                   ├─► go subject   ─┤        │
                                                                   ├─► py subject   ─┤        ├─ semantic diff
                                                                   └─► .NET subject ─┘        ├─ JUnit
                                                                                              ├─ summary.json
                                                                                              └─ parity metrics
```

The coordinator parses the feature exactly once. No native host reads `component.feature`, discovers
anything, or decides what "equal" means — which is what makes five languages provably agree.

## Open/closed shape

The framework supplies MECHANISM; every owner supplies its own POLICY.

```
🧰️framework/…/🧪️test          ← closed for modification
├── comparison mechanism       (profiles are data: ignoreKeys, tolerance, arrays, text, bytes)
├── discovery, planning, hosts (knows adapters and levels; knows no format)
├── 📇️registry                 (domain-neutral decisions only — no oracle, no profile)
└── 📦️packages/🦀️rust          (ZERO dependencies)

<any owner>/🧪️oracle/          ← open for extension, discovered by convention
├── 🔣️component.json           oracles · comparisonProfiles · oracleHostPackages · no-oracle decisions
└── 📦️packages/<lang>/         the crate that links the reference libraries

✏️s/🔌️plugins/🗄️stdio/🧪️oracle   8 oracles · 6 profiles · semio-s-plugin-stdio-test-oracle
🧰️framework/🔨️modules/🖱️ui/🧪️oracle   clsx · cva · 1 no-oracle decision
```

Adding an artifact family touches no framework file. The root `📜️script.ts` likewise reads
`testDomainPath`, `testPhases` and `areas` from the taxonomy rather than naming a module, a phase or
an exempt area — so relocating the testing domain or exempting another area is a vocabulary edit.

## Key decisions and why

**Generated hosts, never committed wrappers.** The taxonomy filenames are deliberately not the names
`go test` / pytest / xUnit discover. Rather than committing a second `_test.go` / `test_*.py` / xUnit
hierarchy next to every adapter, the coordinator materializes a cache-local entrypoint under
`⚡️cache/tests/hosts/`, marked, deterministic and safe to delete.

**The Rust host crate is its own workspace root.** The root `Cargo.toml` is a heavily contended shared
file. Making `semio-repo-test-host` declare `[workspace]` means generated hosts can path-depend on it
without editing the root manifest at all — no lease needed, no collision with concurrent sessions.

**Comparison lives in profiles, not adapters.** Nine owned, versioned profiles. `semantic-pdf-v1`
canonicalizes object numbers, xref offsets, dictionary order, compression, timestamps and document
ids, and keeps version, page count, media boxes, content operators, extracted text and normative
metadata. An adapter cannot invent its own notion of equality.

**Both sides are read back by an independent parser.** A PDF produced by `pdf-writer` and a PDF
produced by this repository's `encode_pdf` are each parsed by `lopdf` before comparison, so no
producer is ever checked against its own reading of what it wrote.

**Pairwise subject parity carries the no-oracle cases.** Where no credible reference implementation
exists — the test platform itself — the recorded no-oracle decision is backed by five independently
written implementations that must project identically. That is real evidence, not a waiver: the run
refuses a no-oracle case that has fewer than two implementations.

**One writer for the dependency baseline.** Rather than layering a second classifier over
`🔒️dependencies.json`, the root freeze's own vocabulary was widened to the five phase classes and
three more ecosystems. The testing domain owns the *gate* (oracle purity, reachability), not a
competing writer.

**The legacy backlog is a shrink-only ratchet.** 50 unmanaged test files exist today. They are counted
per area in `🔒️migration.json`; the contract phase fails when a count grows, and an owner may not be
declared migrated while its count is above zero. No allowlist can turn a failure into a skip.

## The two mechanisms that used to share the word "clean"

| Before | After | Removes files? |
| --- | --- | --- |
| `clean-mechanism` breach kind (owner mounts, subset isolation, module consumers) | `taxonomy/owner-shape` | no — it is an architecture rule |
| `bun ./📜️script.ts clean` (ticket junk, misplaced mounts, oversized artifacts) | unchanged | yes, workspace junk |
| — | `bun ./📜️script.ts clean test [--dry] [--stale]` | yes, marked test outputs only |
| — | `bun ./📜️script.ts clean coverage [--dry]` | yes, coverage reports only |

`clean test` resolves every candidate, proves it lies beneath the canonical test-output root, requires
an ownership marker, never follows a symlink and never deletes an unmarked directory. Its self-test
plants a sentinel in an unmarked sibling and asserts the sentinel survives.
