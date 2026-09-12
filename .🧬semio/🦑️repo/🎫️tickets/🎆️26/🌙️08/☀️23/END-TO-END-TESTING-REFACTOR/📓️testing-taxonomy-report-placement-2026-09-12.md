# Audit Report Placement

The auditor accidentally wrote its initial report under a Unicode-variant sibling of the actual ticket root. That report was moved byte-for-byte into the opened ticket for retention. Its classification is superseded by the corrected acceptance audit. No source changed.

```json
{
  "moves": [
    {
      "from": ".🧬️semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️framework-testing-taxonomy-acceptance-audit-2026-09-12.md",
      "to": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️framework-testing-taxonomy-acceptance-preclassification-2026-09-12.md",
      "sha256": "72b570158ea596c557d862fe2017b454b6e216077d5c957acf853fa5e259286b"
    }
  ]
}
```

## Retained Verification Inputs

The initial failed callback run had written two input files into a hardcoded historical ticket. Those exact inputs are now retained with this task. The guard census runner was also moved out of generated output before cleanup.

```json
{
  "moves": [
    {
      "from": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️energy-rust-reference-diagnostics/🧭️divergence-callback/🧾️runs/🔖️syn-120s-cold-ox6da4/Cargo.toml",
      "to": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/🔮️taxonomy-reference-oracles/🧾️baseline-syn-input/Cargo.toml",
      "sha256": "028b6d8e393f26bef506d164406d9f0dc294c28726bd4597b52b47d1819f74cf"
    },
    {
      "from": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️energy-rust-reference-diagnostics/🧭️divergence-callback/🧾️runs/🔖️syn-120s-cold-ox6da4/📝️.md",
      "to": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/🔮️taxonomy-reference-oracles/🧾️baseline-syn-input/📝️.md",
      "sha256": "5647de51dc3086d37969f9a6a1a08c215b1c17d1ec780ecf221346999262423e"
    },
    {
      "from": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🗑️generated/taxonomy-guard/📜️script.ts",
      "to": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/🧪️taxonomy-guard-census/📜️script.ts",
      "sha256": "10388646f0d10a6a8bc171e589175874b5fd44af187a371728265940935f5979"
    }
  ]
}
```
