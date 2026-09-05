# Sequence Plugin Emoji Repair

## Scope

Owned tree: `✏️s/🔌️plugins/🎬️sequence`, including the Sequence artifact, browser/Wasm support, three subsets, generators, package mounts, mutation catalogs, fixtures, and central exact overrides.

The initial strict audit covered 390 files, 281 directories, and 654 governed entries. It found 50 breaches: 27 missing identities and 23 sibling collisions. Every physical identity was selected and moved explicitly; no automatic emoji chooser, rename planner, migration script, or modifying Git operation was used.

Browser/Wasm identities are now `🌐️sequence-browser-consumer.test.js`, `🔮️sequence-protocol-oracle.test.js`, `📜️sequence-browser.d.ts`, `🌐️sequence-browser.js`, `🖥️sequence-host.js`, `🧩️component.rs`, and `📡️protocol.rs`. The two Wasm fixture owners are `📌️retained-actions` and `👣️trace`. All four window option authorities use `☑️options`.

The generator engines are `📊️csv-engine` and `🧾️json-engine`; JSON source identities are `✨️generate.rs` and `📖️reader.rs`. The generator now writes directly to the handpicked subset, fixture-owner, and arrow-prefixed file identities rather than recreating the former unprefixed paths. Its manifest command registered eight committed carrier fixtures into their actual Step and Dependency oracle owners.

The subset roster is `✳️any`, `🔗️dependency`, and `🪜️step`, each with `🔮️oracle`. Dependency fixtures are `🔗️connect-steps/{⬅️before.json,➡️after.json}` and `✂️disconnect-steps/{⬅️before.json,➡️after.json}`. Step fixtures are `🗂️change-step-collapsed`, `🌱️create-step`, `🗑️delete-step`, `🧬️duplicate-step`, `🔧️edit-step-params`, and `📍️move-step`, with each before/after asset carrying the corresponding `⬅️`/`➡️` identity.

All eight mutation payload sidecars are `🧬️.schema.json`; their sibling carrier directories are the distinct `🛜️wire`, whose internal `🔣️.schema.json` wire schemas remain unchanged. Mutation owner presentation was normalized to `🔗️connect-steps`, `🌱️create-step`, `📍️move-step`, `🔧️edit-step-params`, and `🧬️duplicate-step`. The eight scenario catalog names were reconciled to their existing physical `🚫️…` and `📖️…` scenario identities.

## Verification

- Final strict scoped audit: 390 files, 281 directories, 654 governed entries; missing, generic, presentation, spacing, duplicate, multiple, reserved-emoji, and oracle findings are all zero.
- `validateTaxonomy(loadCatalogTaxonomy())` returns `[]` after the exact Sequence subset and three oracle overrides.
- All 88 non-dot Rust `#[path]` package mounts resolve.
- All three oracle manifests, eight mutation owners, eight scenario directories, eight payload schemas, and 16 registered fixture files resolve.
- `bun …/🏭️generator/📜️script.ts manifests` completed and registered eight handpicked fixture manifests.
- A stale-reference check finds none of the former local subset, oracle, browser/Wasm, generator, mutation-owner, payload-sidecar, wire, fixture-owner, or fixture-file paths.
- `bun nx run @semio-tech/sequence-plugin:test-quick`: exited 1 after `cargo nextest list --list-type binaries-only --message-format json --profile fundamental -p semio-s-plugin-sequence` reached the 1,200,000 ms budget and was killed. The runner explicitly identified likely shared Cargo target-directory lock contention; it did not produce a Sequence-specific compile or assertion failure.

## Exact Central Overrides

```json
"✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets": {
  "*": "✳️any",
  "dependency": "🔗️dependency",
  "step": "🪜️step"
}
```

Each of those three exact subset paths maps to `🔮️oracle` in `testContributionDirectoryOverrides`.
