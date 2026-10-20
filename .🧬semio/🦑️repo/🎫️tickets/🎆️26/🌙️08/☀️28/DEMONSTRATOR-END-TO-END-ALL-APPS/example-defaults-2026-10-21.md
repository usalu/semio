# Demonstrator Example Defaults — 2026-10-21

Language-neutral fixture/schema and regression were authored before changing defaults. Current registered Rust ExampleSource IDs are CAD `demo`, sourcing `demo`, and process `concrete-forest`. The branding source still requests `hexagonal-cut-concrete-forest-left`, `demo-stock`, and `timber-beam-joinery`; the new fixture expectations therefore expose the source mismatch. The sourcing command rejects IDs absent from its registered example roster.

The regression compares exported pane defaults against the fixture, the registered Rust ID declaration, and an independent TypeScript AST projection of each brand default. Ajv validates the fixture schema. The repository test route will run once admitted.

Corrected the three brand defaults to their canonical authored registrations. No legacy aliases were introduced. The queued repository test had not received host capacity before the correction; test execution and runtime behavior remain unverified.
