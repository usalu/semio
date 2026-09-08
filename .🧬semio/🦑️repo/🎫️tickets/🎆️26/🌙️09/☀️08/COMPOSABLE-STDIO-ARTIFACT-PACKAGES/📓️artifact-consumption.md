# Consuming Artifact Packages

## Ownership

The artifact owns its implementations and schemas beneath its domain root. Each `📦️packages` child owns only the language package declaration and command router. For example, PDF Rust declares `[lib].path = "../../🦀️.rs"`; the library root and standard-specific schema, codec, editor and viewer modules remain under `🗿️artifacts/📖️pdf`.

A plugin composes these artifact packages. An artifact does not depend on its owning plugin. Shared registration types live in the stdio contract package, which does not depend on the format catalog.

## Selecting Formats

A Rust workspace consumer can declare `semio-s-artifact-stdio-pdf = { workspace = true }` and use its PDF artifact, snapshot, mutation, difference, definition, assembly and contribution APIs directly. The package has no default features. Its optional `component-app-assembly` feature enables the UI contract needed for app assembly. The PDF dependency closure includes its actual Binary, Deflate and shared-contract requirements; it does not include JPG.

The stdio parent retains explicit compositions:

| Composition | Format selection |
| --- | --- |
| `full-artifact-catalog` | All 36 formats and Semio conversion features |
| `home-io` | Binary, TXT, JSON, XML, CSV, Deflate, ZIP and XLSX |
| `component-app-assembly` | Full catalog plus each format's app assembly |
| Default `plugin-root` | Component app assembly and component guest entry |

Semio has explicit conversion features. Its default **normal runtime dependency** tree contains the shared stdio contract and no other stdio format. Its test dependencies include codec packages needed for existing conversion tests; this distinction is intentional.

## Nx And TypeScript

Package routers use the common Bun implementation, and the repository's Nx integration derives native targets and prerequisites from the package declarations. Native input discovery follows domain sources outside `📦️packages`; build, check and test route through the package's `📜️script.ts`. Launch configurations are regenerated from the launch seed.

The TypeScript PDF package is `@semio-tech/stdio-pdf`. Its export points to emitted domain entrypoint JavaScript and declarations. The shared builder includes referenced JSON and handwritten declaration sidecars in package outputs; downstream package checks resolve those outputs.

## Verified Scope

The final package contract verifies the production inventory, domain ownership, Cargo/Nx agreement, package declaration boundaries and dependency cycles. The current inventory is 99 Rust artifacts, two shared artifact contracts and 40 TypeScript artifact packages. All 40 TypeScript Nx test targets passed. PDF TypeScript and Sequence declaration-sidecar cache invalidation and output restoration passed. Native input isolation was checked against all transitive PDF prerequisites with an independent glob matcher. Native build/cache acceptance is tracked separately in `📓️results.md` and remains in progress.

Nx native input discovery conservatively includes declared optional dependency sources. Cargo feature selection controls actual compilation; inactive optional dependencies can still affect a package's conservative Nx task hash. The verified PDF/JPG isolation is specific to their observed dependency closure.
