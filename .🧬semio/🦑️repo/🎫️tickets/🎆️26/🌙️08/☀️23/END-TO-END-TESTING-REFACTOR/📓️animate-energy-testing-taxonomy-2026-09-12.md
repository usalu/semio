# Animate and Energy Testing Taxonomy — 2026-09-12

The browser environment is executable fixture input for jsdom tests. The three energy documents are neutral fixture data.

- `✏️s/🔌️plugins/🎞️animate/🪨️tests/🟦️.ts` → `✏️s/🔌️plugins/🎞️animate/🧫️fixtures/🌐️browser-environment/🟦️.ts` — SHA-256 `4de075a0b633230b689e500c3c0acc89796e04af24b61ca95bf4df78e282e61d`
- `✏️s/🔌️plugins/🔋️energy/🪨️tests/🧮️p7c1-energy-numerical-laws.json` → `✏️s/🔌️plugins/🔋️energy/🧫️fixtures/🧮️numerical-laws/🔣️.json` — SHA-256 `da546ce7d0149f106e46947a80e03ba0c419aff53af9e6d3dad8f64b437d1d45`
- `✏️s/🔌️plugins/🔋️energy/🪨️tests/🔗️p7c2-energy-retained-wire-laws.json` → `✏️s/🔌️plugins/🔋️energy/🧫️fixtures/🔗️retained-wire-laws/🔣️.json` — SHA-256 `460af8afab5d2a205fa844e7618a442aa6f72290efd557756957b63c1e1f87d1`
- `✏️s/🔌️plugins/🔋️energy/🪨️tests/🦠️p7c2-energy-retained-wire-mutations.json` → `✏️s/🔌️plugins/🔋️energy/🧫️fixtures/🦠️retained-wire-mutations/🔣️.json` — SHA-256 `8cf82b828e4f27b4abe6aa2b9fead008a07ba03027e3a85d5fe0d7bc2dfd0b89`

## baseline Animation Runtime

Actual Vitest selected the canonical presentation index case. The baseline selection overrides the stale legacy include path solely to compare the same assertions.

```json
{
  "passed": 0,
  "failed": 1,
  "success": false,
  "message": [
    ""
  ]
}
```

## Consumers

Updated `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/vitest.config.ts` and `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧪️tests/🔬️unit/🦀️.rs`. The obsolete test filename is removed from Vitest discovery.

## verify Animation Runtime

Actual Vitest selected the canonical presentation index case. The baseline selection overrides the stale legacy include path solely to compare the same assertions.

```json
{
  "passed": 0,
  "failed": 1,
  "success": false,
  "message": [
    ""
  ]
}
```

All four final files retain their captured hashes. The actual Rust include path resolves to the canonical numerical-laws fixture; independent Node JSON parsing accepted all three documents.

## Existing Runtime Failure Boundary

Before and after the relocation, the actual presentation index case fails with chapter undefined: parsePresentationSlideFilePath still accepts a plain slide segment while the fixture and production path formatter use 🎞️slide. The relocation preserves all setup bytes and the existing assertion. No animation runtime pass is claimed. The taxonomy filename exception list also had its three now-absent p7 energy JSON names removed; this edits 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json.
