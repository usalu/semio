# Interpreter native source boundary

Generation `w8YmuSf` failed descriptor attempt 3 while reading the WebAssembly interpreter host invocation source. The native source preflight measured the core interpreter owner at 20 runtime files and 272,761 bytes against a 32-file and 262,144-byte boundary.

The numeric execution, invocation control, invocation memory, and invocation host Rust unit laws now reside with their existing language-agnostic fixtures and TypeScript laws. Each Rust module retains its original name and visibility and is declared with `#[cfg(test)]`. Its fixture includes resolve relative to the new module file. The native runtime sources are 20,788 bytes smaller. The existing Wasmtime oracle, fixtures, and assertions are preserved.

The four production files remain in `INTERPRETER_SOURCE`. Its bytes change because the test bodies moved, which refreshes the owned-engine identity through the existing source digest. The source boundary limits and runtime behavior are unchanged.

The shared native source scanner verified 20 of 32 files and 251,973 of 262,144 bytes, leaving 10,171 bytes. It reported zero test files in the production closure. A static check verified all four cfg-gated declarations, all five fixture references, and the Wasmtime oracle in each moved module. Cargo was outside the scope of this release diagnostic.
