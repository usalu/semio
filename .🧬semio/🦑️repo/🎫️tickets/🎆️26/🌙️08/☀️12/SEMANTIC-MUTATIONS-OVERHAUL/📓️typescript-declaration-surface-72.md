# TypeScript Descriptor-Owned Declaration Surface 72

## Boundary

This packet implements no owned binding projector. It provides only a closed,
ticket-local reference fixture for the declared-surface join proposed by
Correction 2 of `typescript-mutation-identity-binding-67.md`:

```text
descriptor declares TypeScript
  -> canonical aggregate module
  -> named aggregate import
  -> canonical leaf export
  -> declared literal-discriminant or keyed-object shape
```

It does not equate `aggregateVariant` with a TypeScript export, infer paths,
add an identity ledger, discover off-facet declarations, or claim monorepo
completeness.

## Artifacts

- `🧪️typescript-declaration-surface-72/🧬️schema/🔣️.json` is closed and
  checks binding syntax plus encoding-specific fields. SHA-256:
  `ea405ff74b1d9b620adb3a631afd09b2be6deb0fb11c75e5b25e004dcf9272ed`.
- `🧪️typescript-declaration-surface-72/🔣️.json` has thirteen canonical
  virtual cases. Semantically wrong export/provider/key/discriminant values
  remain schema-valid so the independent checker—not schema hardcoding—rejects
  them. SHA-256:
  `17349f94b6ae5af8b7a35e3806a39ba29fab2d978c6c2fd7a501af65dd18c234`.
- `🧪️typescript-declaration-surface-72/📜️script.ts` SHA-256:
  `224821016f37426e21bade3ef7f7a2d3234c46d884b755da46ebbff33b980044`.

The fixture covers VCS discriminated intersections, a named import alias, GIS
keyed objects, wrong export/provider, missing leaf/import, wrong
discriminant/key, namespace/default imports, incomplete syntax facts, and a
physical TypeScript surface omitted from `requiredLanguageSurfaces`.

## Reference Execution

The bounded scoped command completed in 7.46 seconds:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-surface-72/📜️script.ts'
```

Retained terminal evidence:
`🧪️typescript-declaration-surface-72/🧫️runs/🧫️run-05672b21-7b00-4bf8-93ad-93b130792e2d/🔣️result.json`.

Ajv accepted the complete fixture. The existing TypeScript 5.9.3 Program and
checker completed 42 actual checks: 3 bound, 9 rejected, and 1 unsupported.
All virtual authored module reads stayed under `/virtual`; standard libraries
were read only through the explicit existing TypeScript library boundary under
`node_modules/typescript/lib`, never through arbitrary filesystem fallback.
The controller captured stable before/after inputs.

## Captured Inputs

- TypeScript package manifest: `822ef7ca6452205657b6288b066481ecf508bfbf43455d715cf7d3ec457561e6`.
- VCS aggregate / rename leaf:
  `4ceac93ae733bb0662b97ea366e931db69412fabcd78612439b37f2479a86dbb` /
  `daa761eec82458eaf3fc1b31ea12d21827bd6a47ea8a50fe96b9a6219523a033`.
- GIS aggregate / change-exaggeration leaf:
  `88c924ba81426bdcbe2698f7e05f2b8a2aff3fd7afaab202cc2c7050e549009f` /
  `5733ab1ed677cefa4e4716ea828b1506481b512caff8bdabd7e251128db4ea58`.

This is test-only reference execution. No production descriptor, canonical
schema, source collector, Rust code, or binding projector was changed or run.
