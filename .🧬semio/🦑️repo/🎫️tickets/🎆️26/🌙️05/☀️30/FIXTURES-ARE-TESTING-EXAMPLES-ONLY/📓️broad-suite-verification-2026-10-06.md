# Broad Test Suite Verification

The actual Nx repository-test long suite executed353tests:338passed,15failed,7946assertions,297seconds. This is not reported as an overall pass. Failures include live unrelated mutation vocabulary/oracle coverage/dependency debt and concurrent repository topology changes, alongside obsolete fixture-schema vectors and stale schema catalogs addressed by this refactor. The exact failures were reviewed from the execution log. Native jobs remain pending shared preparation and no native pass is inferred.

```text
pillow is linked by oracle lowpoly-io-png-pillow but is absent from the dependency baseline
```

```text
js:ignore is on a generated host's import path but is absent from the dependency baseline
```

```text
Expected: false
Received: true
```

```text
Test discovery requires real workspace ancestry: /var
      at workspaceAdmission (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:56:137)
      at taxonomy (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:46:21)
      at discoverCaseDirs (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:362:22)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:1515:14)
✗ 🧪️ projected vector storage > profile storage never becomes an executable Nx test project [14.32ms]
✓ 🔬️ subject selection > an oracle implemented in the case's own adapter is never also dispatched as that case's subject [126
```

```text
Expected: true
Received: false
```

```text
Expected: "schema-fixture-defines-schema:🌎️hub/💡️inference/🧫️fixtures/🧫️approval/🧬️.schema.json"
Received: "schema-fixture-defines-schema:🌎️hub/💡️inference/🧫️fixtures/🧫️approval/🧬️.schema.json,🌎️hub/💡️inference/🧫️fixtures/🧫️approval/🧬️schema/🔣️.json"
```

```text
Nested Cargo catalog digest drift
      at semanticPackageProjectionCatalog (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:5448:144)
      at projectNestedCargoPackages (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:4266:19)
      at inventoryTaxonomyWithSourceParentPruning (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:4823:3)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️mutation-fixtures/🟦️.ts:47:21)
✗ HTML source pair normalization accepts the primary leaf of a multi-extension kind [14831.91ms]
✓ HTML source pair manifests retain exact native-reader and parser inputs [1009.20ms]
✓ HTML source pair readers declare every exter
```
