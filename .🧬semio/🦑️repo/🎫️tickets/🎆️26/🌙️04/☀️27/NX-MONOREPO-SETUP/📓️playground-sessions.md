# Independent Playground Sessions

Nx generated Note and studio sessions in separate owned directories, then restored the deleted Note directory from its task cache with identical hashes. The studio directory remained byte-identical. Node independently imported both generated TypeScript modules and checked their variant identities, host modes and plugin membership.

[{"variant":"note","hostMode":false,"plugins":1},{"variant":"s","hostMode":true,"plugins":59}]


React development consumers now resolve these variant artifacts through the preparation graph. Remaining WGPU/build/E2E consumers still require migration.
