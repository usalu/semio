# Jack LSP build graph resolution

Jack’s worker imports the workspace package `@semio-tech/trinity-jack-lsp`. The package exports wasm-bindgen’s JavaScript module and declarations from `pkg`. The package’s WASM build target declared its receipt and source owner, but omitted the generated module paths needed to resolve those exports in a fresh checkout.

The WASM target now promises both public JavaScript import specifiers through its existing `compileOutputs` contract. Each maps to the same generated module and declaration file. The build graph can discover the producer from authored metadata before the WASM build runs.

The Rust package observes its Rust sources, Cargo manifests, fixture data, and package build files. The worker source consumes the WASM package from its own TypeScript project, which prevents a circular producer dependency.

The build graph’s cold WASM inventory test now includes Jack LSP. All three focused tests passed, including the SQLite and TypeScript resolution oracle and the inventory check that refuses to read generated files. Using the full workspace owner registry, the real module resolver mapped Jack’s worker import to the expected WASM target without reading generated files.
