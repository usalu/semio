# Nakagin Storybook Fixture Separation

The shared Nakagin example is static test data consumed by the canonical Table and Textarea Storybook cases. It now lives at the UI elements semantic owner under canonical fixtures. The two consumers import its JSON values. The old executable-looking TypeScript data module and empty legacy directory were removed.

Validation ran through Bun and Nx: the TypeScript compiler evaluated the retained original module, Node strict assertions confirmed identical values in the JSON fixture, and esbuild independently resolved the canonical fixture from both Storybook cases. Both consumer checks passed. This verifies the example data and import boundary; a full browser render was not run.

The original source was retained temporarily under ticket generated output for the reference comparison and is removed during final cleanup. Fixture SHA-256: `75d0f4963f2a6b5dd78e5fa2aadbb3ae55c05159f2ad8c90ed20698d0ea652b5`.

Authored paths:

- `.storybook/fixture/nakagin.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧫️fixtures/🏢️nakagin/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🧪️tests/📚️storybook/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️tests/📚️storybook/🟦️.tsx`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧪️nakagin/📜️script.ts`
