# 🧊️ Frozen Test Contract — v1

Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. This file is the normative contract every
adapter, host, policy and Nx target in the new testing architecture is written against.
Nothing below may be re-decided locally by an implementation.

## 1. Ownership

A test is owned by the **nearest language-neutral domain owner** — the taxonomy entity that
defines the behaviour — never by a language package.

```
<owner>/
├── 🧫️fixtures/            immutable, shared by all of this owner's cases
├── 🧪️tests/
│   └── <kebab-case>/       one capability / behaviour
│       ├── 🧫️fixtures/     immutable, private to this case
│       ├── component.feature
│       ├── 🦀️component.rs
│       ├── 🟦️component.ts
│       ├── 🐹️component.go
│       ├── 🐍️component.py
│       └── 🔷️component.cs
└── 📦️packages/<lang>/
```

* `🧪️tests` MUST NOT appear under `📦️packages/**`.
* Case slug pattern: `^[a-z0-9]+(?:-[a-z0-9]+)*$`.
* Exactly one `component.feature` per case (deliberate language-neutral filename exception).
* One adapter per implementation that claims the feature's capability, no more, no fewer.
* No test-generated output may be written into either fixture directory.

## 2. Stable identity

```
<owner-relative-path>::<case-slug>::<scenario-id>::<implementation>::<role>
```

Display names may change; the identifier may not.

## 3. Feature profile (restricted Gherkin)

Feature-level tags (all required):
`@capability-<id>`, `@oracle-<id>` **or** `@no-oracle-<decision-id>`, `@comparison-<profile-id>`.

Scenario-level tags:
`@id-<stable-id>` (required, unique per feature),
exactly one of `@level-fundamental|quick|long|exhaustive`,
exactly one `@mode-differential|conformance|round-trip|property|error`.

Optional: `@platform-<id>`, `@seed-<digits>`, `@requires-<tool>`, `@implementation-<id>`.

Supported keywords: `Feature`, `Background`, `Scenario`, `Scenario Outline` (+ `Examples`),
`Given`/`When`/`Then`/`And`/`But`, doc strings (`"""`), data tables (`|`), comments (`#`).
Missing tools fail setup — never a silent skip.

## 4. Fixture URIs

```
shared://<name>     → <owner>/🧫️fixtures/<name>
local://<name>      → <owner>/🧪️tests/<case>/🧫️fixtures/<name>
```

Lookup is explicit; a local basename can never shadow a shared one.

## 5. Result schema

Every native host emits one JSON object per executed `(scenario, implementation, role)`:
see `🧬️schema/🔣️component.json` → `TestResult`. Binary payloads are referenced by
cache-relative path + digest, never embedded.

## 6. Comparison profiles

`exact-bytes-v1`, `utf8-text-v1`, `ordered-json-v1`, `unordered-json-v1`,
`floating-point-v1`, `semantic-pdf-v1`, `filesystem-tree-v1`, `diagnostic-v1`,
`event-stream-v1`. Owned, versioned, self-tested. Adapters never compare.

## 7. Generated output root

```
.🧬semio/🦑️repo/⚡️cache/tests/{work,hosts,oracles,results,diffs,reports}
```

Every generated root carries `🧾️marker.json`:
`{"kind":"semio-test-output","testId":…,"cacheKey":…}`. Only marked directories are
deletable by `clean test`.

## 8. Dependency classes

`production-runtime`, `production-build`, `repository-tooling`, `test-runner`, `test-oracle`.
Target final state: zero `production-runtime` and zero `production-build` external deps.
Classification is by **reachability**, not manifest placement. Shrink-only baseline.

## 9. Nx granularity

One virtual project per test case, named `test-<slugified owner path>-<case>`, with targets
`lint`, `test-contract`, `test-oracle`, `test-subject`, `test-parity`, `test`, `test-quick`,
`test-long`, `test-exhaustive`. Levels are cumulative.

## 10. Hard exclusion

`compose/**` is excluded in the discovery library itself — not by workflow path filters —
for discovery, execution, cleaning, coverage, dependency classification and every report.
No other area is permanently exempt.

## 11. Migration status ladder

`discovered → surveyed → contract-ready → oracle-green → subject-green → parity-green →
coverage-green → dependency-clean → legacy-removed → ci-enforced → complete`.
