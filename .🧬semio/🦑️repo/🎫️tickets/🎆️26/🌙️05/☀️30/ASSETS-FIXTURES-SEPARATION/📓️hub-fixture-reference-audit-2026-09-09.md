# Hub Rust Fixture Reference Audit

Current literal include targets, resolved relative to each caller. This checks file existence, not complete compilation/feature reachability.

```json
[]
```

## Split-String Script Consumer Repair

The Hub script contained five residual classes of stale path expression after literal include relocation. Redundant fixture wrappers, the local-bootstrap legacy directory, and package-root fixture joins were corrected against their actual moved owners. Runtime verification of these branches remains pending.

```json
[
  {
    "old": "🌎️hub/📇️directory/🧫️fixtures/🧪️fixtures/",
    "new": "🌎️hub/📇️directory/🧫️fixtures/",
    "count": 1
  },
  {
    "old": "\"🌎️hub\", \"📇️directory\", \"🧫️fixtures\", \"🧪️fixtures\",",
    "new": "\"🌎️hub\", \"📇️directory\", \"🧫️fixtures\",",
    "count": 1
  },
  {
    "old": "🌎️hub/🚀️local-bootstrap/🧪️fixtures/",
    "new": "🌎️hub/🚀️local-bootstrap/🧫️fixtures/",
    "count": 1
  },
  {
    "old": "join(this.root, \"🧪️fixtures/🏛️admin-directory-authority-v1\")",
    "new": "join(this.repoRoot, \"🌎️hub/🧫️fixtures/🏛️admin-directory-authority-v1\")",
    "count": 1
  },
  {
    "old": "join(this.root, \"🧪️fixtures/🌐️directory-message-authority-v1\")",
    "new": "join(this.repoRoot, \"🌎️hub/🧫️fixtures/🌐️directory-message-authority-v1\")",
    "count": 1
  }
]
```

## Actual Hub Consumer Results

All four repaired paths executed successfully through Bun/Nx: administrative authority (Ajv plus SQLite ordering 5, short actions 8, bindings 15), message authority (Ajv plus ordering 4, wire scopes 6, hostile source 5), socket-grant oracle (2 schema exports, 19 decisions, 3 hostiles, 3 close states, 3 relay checks), and ordered publication (9 checks). All four processes exited zero. These are source/oracle phases; full Hub native compilation remains separately unverified.

## Final Snapshot Follow-up

The physical snapshot exposed duplicate browser-broker lifecycle data and schema under the old spelling. The data matched canonical bytes; the schema differed only in JSON formatting. Canonical inputs were retained and duplicate legacy files removed. Additional composed Hub and integration roots were updated to canonical fixture spelling and semantic owners. Focused runtime follow-up is pending.

```json
{
  "moves": [
    {
      "oldPath": "🌎️hub/🧪️fixtures/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
      "newPath": "🌎️hub/🧫️fixtures/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
      "sha256": "c316a333dc5a9634902ddad52bc39832ca046764213f6d317f49b18aecf91504",
      "disposition": "removed byte-identical legacy duplicate"
    },
    {
      "oldPath": "🌎️hub/🧪️fixtures/🔐️browser-broker-proof-lifecycle-v1/🧬️.schema.json",
      "newPath": "🌎️hub/🧬️schema/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
      "sourceSha256": "8b4dd58b52ed03cb42ba2b68627313660ed6d8cf5c07fdd7c303124eef483d12",
      "targetSha256": "d54b96485b1be501d9bd7388a4dda12122db670cd758222ef5d05ac055e80908",
      "disposition": "removed JSON-equivalent legacy duplicate; formatting only difference"
    }
  ],
  "consumers": [
    {
      "path": "🌎️hub/🧪️tests/🤝️integration/🟦️.ts",
      "legacyDirectoryReferences": 7
    },
    {
      "path": "🌎️hub/📦️packages/🦀️rust/📜️script.ts",
      "legacyDirectoryReferences": 23
    }
  ]
}
```

Runtime follow-up: the actual Hub Vitest integration case passed all 10 selected quick contract tests through Bun and Nx after the composed-root corrections. Two real-server tests were excluded by the requested name filter and HUB_E2E=0. The passed checks include canonical checkpoint framing, bootstrap HMAC, typed auth, trusted catalog, authority adapter, chunk-CAS, and lag rebootstrap fixtures. No server/end-to-end pass is claimed.
