# Shared Native Source Inventory

2026-10-28. Canonical readRustSourceClosureV1, macOS ARM64/unix, no extra feature flags. Normal source owners plus actual engine cfg(test) owner mounting the new Report authority law. No compiler, graph or test launched. Limits32inputs/262144bytes/4000active lines.

```json
[
  {
    "name": "engine",
    "entry": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🦀️.rs",
    "cfg": [],
    "files": 13,
    "bytes": 257193,
    "lines": 2871,
    "lawMounted": false
  },
  {
    "name": "engine-test",
    "entry": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🦀️.rs",
    "cfg": [
      "test"
    ],
    "files": 16,
    "bytes": 266514,
    "lines": 2904,
    "lawMounted": false
  },
  {
    "name": "engine-core",
    "entry": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️store-components/🦀️.rs",
    "cfg": [],
    "files": 10,
    "bytes": 191504,
    "lines": 2850,
    "lawMounted": false
  },
  {
    "name": "engine-live",
    "entry": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🎞️live/🦀️.rs",
    "cfg": [],
    "files": 11,
    "bytes": 201143,
    "lines": 2880,
    "lawMounted": false
  },
  {
    "name": "read",
    "entry": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🦀️.rs",
    "cfg": [],
    "files": 8,
    "bytes": 120596,
    "lines": 1312,
    "lawMounted": false
  },
  {
    "name": "raw-replay",
    "entry": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🦀️.rs",
    "cfg": [],
    "files": 12,
    "bytes": 153242,
    "lines": 2131,
    "lawMounted": false
  },
  {
    "name": "history-publication",
    "entry": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/🦀️.rs",
    "cfg": [],
    "files": 9,
    "bytes": 190058,
    "lines": 587,
    "lawMounted": false
  }
]
```

## Actual New Law Ownership Blocker

The engine cfg(test) owner exceeds the262144byte ceiling by4370bytes even before mounting the new authority law. More importantly, the new report/🧪️tests/🦀️.rs and authority.json have no Rust module mount in the bounded Store source tree: report implementation ends without a test mount, engine unit test root is only a docstring, and the only original_report_replay_store mount includes the implementation alone. Therefore the new law currently has no executable owning test target. The lawMounted=false observations agree with this explicit source inspection. This report does not conflate unrelated oversized test suites with this actual ownership gap.

All six changed normal shared owners fit current quotas. A focused public raw EditReplay test companion is the narrow route to executable law ownership; register its package/workspace/build owner and check its own cfg(test) closure before the exact native test. Merely putting the law in an unreachable test file is insufficient, and adding it to the already oversized engine cfg(test) owner would worsen that admission refusal. Exact results sent to Rust, mutations and coordinator before native retry.

## Neutral Fixture and Independent Oracle Linkage

The neutral authority.json has two cases: original authority without a producer refuses UnsupportedOwner and retains genesis state; explicit cold authority executes the authored Zoom. Native code parses that JSON and the sibling operation fixture through serde_json, checks unchanged pointer/cursor/operation/progress/extents/zero receipt for refusal, and checks authored Zoom for cold success. It now includes complete controlled lifecycle cleanup.

serde_json is an independent serializer, but decoding expected values is not an independent implementation of authority or replay behavior. No separate behavior oracle currently consumes authority.json in the inspected packet. The fixture schema and operationFixture fields are descriptive; the native law hardcodes its sibling operation file. User-required third-party same-output validation therefore remains to be wired to a genuine behavioral oracle (for example an independently applied neutral patch/state witness), rather than claiming JSON decoding alone proves semantic equivalence. Mutations worker was notified of this exact distinction.
