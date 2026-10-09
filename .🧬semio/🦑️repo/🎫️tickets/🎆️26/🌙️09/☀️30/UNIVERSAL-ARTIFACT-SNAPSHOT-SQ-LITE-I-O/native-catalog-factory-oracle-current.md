# Native Catalog Factory Oracle Current

Read-only 2026-10-09; no execution.

Actual Source already retains factoryId. `🌎️hub/🧩️compositions/🗄️stdio/📇️publication/✅️trusted-stdio-catalog/🟦️.ts:9–20` defines native codec and receipt with required string factoryId. Original JSON receipt load110 preserves row.factory_id; verify projection141 copies row.factoryId; catalog210 retains codecs. OpenTarget extends the same native codec shape23. This is an actual runtime projection, not Rust text inspection.

Existing independent Source law `🧪️tests/✅️trusted-stdio-catalog/🟦️.ts:47–79` validates original receipt JSON with Ajv, builds actual catalog, validates it with catalog schema, checks current JSON factory stdio.native.json.v1, and uses neutral fixture multipleStandards.factoryIds to distinguish real same-kind different-standard identities. Node crypto SHA-256 then independently checks every protocol source80 onward. Add exact row-by-row `(artifactKind,artifactSchema,factoryId)` equality between actual verified receipts and built catalog, preserving all selected rows; do not just assert counts or read Rust source strings.

Best existing shared neutral case: publication `🧫️fixtures/🧬️trusted-stdio-catalog/🔣️.json` multipleStandards actual schemas/factory IDs plus schema sibling. Native provider law should compare returned NativeCodecBinding factory_id with original receipts, including these exact same-kind identities where available. Authentic None nonfactory fixture case can be authored in neutral contract without inventing an ID.

An additional actual runtime receipt projection exists at publication `♻️native-receipt/🟦️.ts:3–12,22`: NativeCodecPublicationReceiptV1 carries factoryId and projectNativeCodecReceiptPublicationV1 validates it before publication. Its existing fixture/schema/test siblings are useful for per-receipt retention/hostile mismatch tests, but do not alone prove Rust binding capture.

Exact original Nx route from publication `📋️project.json`: `bun nx run @semio-tech/hub-stdio-publication:test --skip-nx-cache`. It calls existing `📜️script.ts test`, which runs the original Vitest config. Supply caller-owned absolute SEMIO_TEST_ARTIFACT_DIR under ticket generated output for tests that create publication artifacts. No new scripts needed.

These Source and Native executable projections can share semantic vectors and independent Ajv/Node oracle. They qualify factory preservation, not complete selected binding/open-target ledger denominator. No Source run claimed.
