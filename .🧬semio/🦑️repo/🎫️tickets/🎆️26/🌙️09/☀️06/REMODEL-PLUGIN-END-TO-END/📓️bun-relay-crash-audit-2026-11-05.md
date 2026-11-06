# Bun relay crash audit

Retained reconnect-bun-current.log reports Bun 1.4.2 Windows x64 segmentation fault after the test file header, with workers_spawned and no individual test verdict. It does not establish whether a test body started or whether socket lifecycle caused the fault. The reported elapsed time is about 1.2 seconds after process startup.

In 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/📡️transport/🧪️tests/🟦️.ts, Ajv plus fixture and schema JSON are unique top-level imports relative to 🌐️worker.ts and 🟩️node.ts. All share the actual transport module; the unit and Node tests also share node:net and node:events. Worker uses Bun.serve, Playwright and browser preparation; Node uses node:test and assert. The unit has six tests, including cancellation/deadline and 2 MiB half-close backpressure. The other successful paths do not prove those six Bun tests.

Useful narrow normally admitted probes: import-only Ajv; then Ajv with these JSON inputs and compilation; import-only relay; then individually filtered unit cases with entry and cleanup evidence, especially cancellation/deadline and half-close. Compare exact outcomes under the same runtime; a fault before entry points to loading/discovery, while an observed in-body failure narrows lifecycle investigation. No crash cause is established from the retained log alone.
