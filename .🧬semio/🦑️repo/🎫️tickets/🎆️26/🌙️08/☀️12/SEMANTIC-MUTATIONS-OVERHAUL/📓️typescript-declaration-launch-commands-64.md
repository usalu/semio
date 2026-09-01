# TypeScript Declaration Launch Command Release

## Exact Current Observation

The original report is retained unchanged. Root directly read and parsed its JSON block at SHA256 `8ea9f1e590263da253b2a8f1d97e8a696f697c6fa8ab507d373f2d7454928c07`. At that observed endpoint every decoded command below has exactly two U+0022 delimiters and zero U+005C+U+0022 sequences. The reported escaped-delimiter observation is not reproduced by this direct-file parse; its historical bytes have not been restored or overwritten. This new block is an explicit serialization of the required plain-delimiter values, not a claim about another transport or an earlier endpoint.

## Superseding Exact Values

Only these five command values are released here. Names, order, arguments, cwd, and presentation remain unchanged. The two canonical rows at 410.512/410.513 stay as previously proposed. Canonical package controller's currently released SHA256 is `fcae555a1a3aab5ac29216075803aac8c6feec8b14329bd2483d5412bcdc1b7d`; ticket controller hashes are unchanged from report61. No command was executed by this byte check and no launch output was changed.

```json
[
  {
    "name": "⚖️gate📚️declarations🟦️captured-reference",
    "type": "node-terminal",
    "request": "launch",
    "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts\" reference",
    "cwd": "${workspaceFolder}",
    "presentation": {
      "group": "4_gate",
      "order": 410.514
    }
  },
  {
    "name": "⚖️gate📚️declarations🟦️captured-subject",
    "type": "node-terminal",
    "request": "launch",
    "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts\" subject",
    "cwd": "${workspaceFolder}",
    "presentation": {
      "group": "4_gate",
      "order": 410.515
    }
  },
  {
    "name": "⚖️gate📚️declarations🟦️boundaries-reference",
    "type": "node-terminal",
    "request": "launch",
    "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/📜️script.ts\" reference",
    "cwd": "${workspaceFolder}",
    "presentation": {
      "group": "4_gate",
      "order": 410.516
    }
  },
  {
    "name": "⚖️gate📚️declarations🟦️boundaries-subject",
    "type": "node-terminal",
    "request": "launch",
    "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/📜️script.ts\" subject",
    "cwd": "${workspaceFolder}",
    "presentation": {
      "group": "4_gate",
      "order": 410.517
    }
  },
  {
    "name": "⚖️gate📚️declarations🟦️malformed-subject",
    "type": "node-terminal",
    "request": "launch",
    "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/📜️script.ts\" check",
    "cwd": "${workspaceFolder}",
    "presentation": {
      "group": "4_gate",
      "order": 410.518
    }
  }
]
```

## Byte Inspection Receipt

Each command equals the current directly decoded original report value. JSON escape spelling in this Markdown is transport syntax; after JSON.parse the filename delimiters are byte 34, with no preceding backslash. This is command-byte validation only, not shell execution or native/census acceptance.

```json
{
  "sha256": "8ea9f1e590263da253b2a8f1d97e8a696f697c6fa8ab507d373f2d7454928c07",
  "commands": [
    {
      "name": "⚖️gate📚️declarations🟦️captured-reference",
      "order": 410.514,
      "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts\" reference",
      "backslashQuote": false,
      "quoteOffsets": [
        79,
        204
      ]
    },
    {
      "name": "⚖️gate📚️declarations🟦️captured-subject",
      "order": 410.515,
      "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts\" subject",
      "backslashQuote": false,
      "quoteOffsets": [
        79,
        204
      ]
    },
    {
      "name": "⚖️gate📚️declarations🟦️boundaries-reference",
      "order": 410.516,
      "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/📜️script.ts\" reference",
      "backslashQuote": false,
      "quoteOffsets": [
        79,
        198
      ]
    },
    {
      "name": "⚖️gate📚️declarations🟦️boundaries-subject",
      "order": 410.517,
      "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/📜️script.ts\" subject",
      "backslashQuote": false,
      "quoteOffsets": [
        79,
        198
      ]
    },
    {
      "name": "⚖️gate📚️declarations🟦️malformed-subject",
      "order": 410.518,
      "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/📜️script.ts\" check",
      "backslashQuote": false,
      "quoteOffsets": [
        79,
        197
      ]
    }
  ]
}
```

