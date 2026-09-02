# Framework-OS Test Import Diagnosis

The AppChannelClient local-interaction fixture was moved to `🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json`; four tests still referenced its removed flat name. The linked-session engine tests similarly referenced the removed singular `🧪️fixture/🔣️s.schema.json` rather than the existing `🧪️fixtures/🔣️.schema.json`.

The product-root `🟦️backbone-worker.ts` import from `./🟦️component` has already been repointed to `./🟦️` in the concurrently staged taxonomy migration. This ticket preserves that work and validates it through the framework-os suite.

The existing Vitest tests exercise the repaired file reads and use Ajv as an independent schema oracle for the linked-session-engine vectors.
