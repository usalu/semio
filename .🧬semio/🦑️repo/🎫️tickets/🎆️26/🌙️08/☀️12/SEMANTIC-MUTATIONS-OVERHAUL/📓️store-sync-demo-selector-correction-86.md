# Sync Demo Selector Correction 86

Only the four approved JSON-string prefixes were changed in [verifier75 vectors](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🔣️.json>). No controller, schema, contract, candidate74, canonical source, launch, or native file was edited. No controller/Nx/source/native command was executed.

## Retained Failure and Preimage

Read [root report85](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-sync-demo-reference-first-85.md>), its complete structured review, and the original receipt/output. The actual first registered reference remains **FAIL**:14/25 cases reached,21 collected passing checks, then `Neutral replacement is not exactly one occurrence.`; zero final captures. This correction is not a rerun result.

The current pre-edit vector bytes were read with lexical Compose exclusion, full no-follow ancestry, and handle/endpoint checks, then compared byte-for-byte with [the original run's vector capture](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🧫️run-9jca2c/first-04/🔣️.json>). They were identical:

- Before:39,619bytes, SHA-256 `71af094f9d7ad5da7466ed8f72a1a4c26065cb72180ebb29f0d92a9ec1164ab8`.
- After:39,627bytes, SHA-256 `76bfeff252aa5a2d7fd7eeba53b4c16a22200fce155a728b0687090bc85837fe`.
- Exact delta:+8 physical bytes: four `5c 6e` JSON escape prefixes, each decoding to one leading U+000A newline.

Original receipt remains `32b7bd4c9b3467027cb08b24f93e21b13453752f57c0ed731229bf742c872041`; original output remains `c7360961140f4c7f37d375ae6f8e9dcca5371729f6e55fed9ee5feb3dcb4bcb2`. Neither was rewritten.

## Exact Four Token Edits

Offsets are zero-based. Token offsets include the opening quote; the inserted bytes follow it. These four positions have identical UTF-16 and UTF-8 byte offsets because the preceding vector-file text is ASCII.

| Case / field | Old token start | New token start | Inserted bytes at new offset |
| --- | ---: | ---: | ---: |
| changed-constructor-value / find | 28293 | 28293 | 28294 |
| changed-constructor-value / replacement | 28466 | 28468 | 28469 |
| comment-only-constructor / find | 28927 | 28931 | 28932 |
| comment-only-constructor / replacement | 29100 | 29106 | 29107 |

```json
[
  {
    "path": [
      "cases",
      14,
      "edits",
      0,
      "find"
    ],
    "oldToken": "\"            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN(SetN { n: 42 }).encode_op().expect(\\\"encode\\\")) }],\"",
    "newToken": "\"\\n            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN(SetN { n: 42 }).encode_op().expect(\\\"encode\\\")) }],\""
  },
  {
    "path": [
      "cases",
      14,
      "edits",
      0,
      "replacement"
    ],
    "oldToken": "\"            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN(SetN { n: 43 }).encode_op().expect(\\\"encode\\\")) }],\"",
    "newToken": "\"\\n            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN(SetN { n: 43 }).encode_op().expect(\\\"encode\\\")) }],\""
  },
  {
    "path": [
      "cases",
      15,
      "edits",
      0,
      "find"
    ],
    "oldToken": "\"            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN(SetN { n: 42 }).encode_op().expect(\\\"encode\\\")) }],\"",
    "newToken": "\"\\n            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN(SetN { n: 42 }).encode_op().expect(\\\"encode\\\")) }],\""
  },
  {
    "path": [
      "cases",
      15,
      "edits",
      0,
      "replacement"
    ],
    "oldToken": "\"//            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN(SetN { n: 42 }).encode_op().expect(\\\"encode\\\")) }],\"",
    "newToken": "\"\\n//            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN(SetN { n: 42 }).encode_op().expect(\\\"encode\\\")) }],\""
  }
]
```

Each `find` and `replacement` now starts with the same actual newline. This disambiguates the intended12-space line from a suffix of the20-space actor-test line, without changing target values or the controller's exactly-one-occurrence guard. No selector heuristic was added.

## Exact Inverse and Preservation

Read-only jsonc-parser token coordinates located the four corrected strings. Replacing just those four tokens with their recorded originals, in descending offset order, produced a buffer **byte-identical to the complete original failed-run vector capture**, SHA-256 `71af094f9d7ad5da7466ed8f72a1a4c26065cb72180ebb29f0d92a9ec1164ab8`. Equivalently, removing only the two bytes at corrected offsets29107,28932,28469,28294, in that descending order, restores the preimage.

The parsed document differs at exactly the four listed fields. All25 case identities/order and every expectation are unchanged; the other23 cases are completely unchanged. This was an edit/byte-inverse check, not execution of the source-verifier suite.

All15 other captured inputs retain their bytes and file identities across the edit:

| Protected input | Unchanged SHA-256 |
| --- | --- |
| controller | `f0d8784a40fa815f4651b427f1429779f4d1e2fa3796bfe314328b5424d3b0ea` |
| schema | `29d4ffeac496bae621e43fa01c6bddce0a926677f598e7d951c54d63d3c15795` |
| contract | `140b1773d791f4f04a5349183b64d0668794a8136aaff008460d59e50483a038` |
| before-image | `62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6` |
| discovery | `5ef65775df39b8a8e435ffb48d6a7b41070364911b7e398de0f22cdc5b138956` |
| sync | `62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6` |
| descriptor-authority | `db1c30ab7f19ab9a0f46539c71a427ba6ce51789c5c7904ea4d93dd9ea488aee` |
| reference-intrinsicSchema | `e6b29118436524c3e260202f24f4b16bfc5dbe826f039a5487af322685ca4f05` |
| reference-domainVectors | `1b7de5975bf3b91d38f34ffb3f0bddb41be12c7413c6644a63eaaaec1836d9e2` |
| reference-domainSchema | `13d7129fcd48776ad5eee2afa43531b06ac3df37a56e705d1aeb16c3a03b9d25` |
| reference-aggregateRust | `ac0d9d147982cd134f3597ee1c5b6caa7adc44db86916068a1fff40a4bffd20e` |
| reference-aggregateSchema | `233a6cd61161192e64af97fecc7e3671381953c06e67d67395790f1e5b10d93a` |
| reference-leafRust | `4a33d2fc4c2aa755168c6bec7ad85235b2cf6a5f312a983b30751bd7d14787f4` |
| reference-descriptor | `f72e787f968feb4c29a3e4b6ad9b5da7ce562db71d53399187ec019edaaec8fd` |
| reference-payloadSchema | `0b3a5bfe2c9c8d87ffcee1323de62f758b9299dbf3cce7183891ab30f64ecec1` |

The original source fixture still records12 functions/22 expressions, including `tests::actor_tests`5functions/7expressions. The correction changes no contract or source fixture.

Root owns the separately authorized single registered-reference rerun after independent inverse review. No passing25-case, source-mounted, Rust privacy, codec, metadata-provenance, or native behavior result is claimed here.

