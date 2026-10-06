# Production Policy Ownership

These files were authoritative inputs to actual source ownership and protocol generation, not testing examples. Their semantic owners now retain them as actual policy declarations. Independent negative/example data remains fixtures.

- 🧰️framework/🔨️modules/◻️2d/🧮️compute/🧫️fixtures/📍️binding-origin/🔣️.json → 🧰️framework/🔨️modules/◻️2d/🧮️compute/📏️ownership/🔣️.json
- 🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/🔖️channel-version.json → 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json
- 🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/📇️consumers.json → 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/📇️consumers/🔣️.json

## Updated Readers

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts
- 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🔍️census/🟦️.ts

## Verification

`bun nx run @semio-tech/repo-lib:test-rust-family-ownership --skip-nx-cache --outputStyle=static` passed 76 tests with 165 assertions with an explicit ticket-owned `SEMIO_TEST_ARTIFACT_DIR`. This suite includes the compute ownership declaration. The first run correctly required the caller-owned output directory. Native kernel store tests also cover the moved channel pin and synthetic support; final rerun is pending.

## Additional Changed Files

- `🧰️framework/🔨️modules/◻️2d/🧮️compute/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/◻️2d/🧮️compute/📋️project.json`
