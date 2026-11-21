# Puzzle WASM Graph Resolution

The full S host stopped during project graph discovery after three slice workers failed to resolve `@semio-tech/puzzle-wasm`. The Hub dependency link resolved to the authored workspace package. Its browser runtime and declaration paths were absent as expected before generation. The S editor and stories defer the actual import until the browser requests a session. The graph's source walk nevertheless resolves literal dynamic imports, and the Puzzle project previously declared its WASM producer without listing its compile outputs.

The `@semio-tech/puzzle-plugin:wasm` project now declares both JavaScript exports and their declaration file from the existing package metadata. The graph can resolve those promises directly from authored files. The cold-output inventory test reproduced the missing promises before the project change with 12 indexed modules against 14 expected. After adding the promises, all three tests passed with 76 assertions using the production WASM output index. A filesystem check confirmed the two export mappings while both generated files were absent.

The full S host replay remains the integration check.
