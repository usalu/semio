# Plugin Declaration Capture Hardening

## Scope

Only the ticket controller changed. Production fixtures, schemas, and Plugin source remained frozen.

## Red Then Green

The retained pre-fix scoped Bun/Nx RED, [run-J4xDSt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-declaration-fixtures-43/🧫️run-J4xDSt/📓️result.md), completed `570/571`: the virtual repeated-read assertion exposed that later reads replaced the first captured digest.

`captureDigest` now preserves the first digest and rejects a later different digest. The virtual regression checks an identical repeat preserves the first digest and a differing repeat throws. The final controller reread therefore cannot overwrite its initial entry before the nofollow final comparison.

The scoped Bun/Nx GREEN, [run-dNdMJX](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-declaration-fixtures-43/🧫️run-dNdMJX/📓️result.md), completed `571/571` with every captured input unchanged between first and final reads.

## Endpoint

The endpoint controller SHA-256 is `322383dfa45125dd790f95bf52d28277e7f403f967b637267d6895dcba863a7d`. The consumed Plugin component SHA-256 was `12bc97e01166b3c50fccdd5221264174c14aaaa8a7aae36d11587f3cf4a9345d`.

This was an Ajv/jsonc-parser source gate only. Native Rust tests were not executed.
