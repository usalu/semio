# Program Mutations TypeScript Surface

## Scope

Make `🏛️program/…/🧬️schema/🧬️mutations/🟦️component.ts` self-contained and type-safe for its direct standalone TypeScript compile. The ticket remains open.

## Change

- Added the local `//#region 🔖️Entities` block used by the mutation payloads, copied exactly from the Rust-derived sibling program schema block.
- Removed the sibling type import so the mutations schema follows the FEM mutations convention of locally declaring every payload dependency.
- Updated `CreateFunction` and `ReplaceFunction` to use the local `Function` domain entity instead of the former import alias.
- Kept the existing 266 payload interfaces and their operation wrappers flat under their discriminant, matching Rust's internally tagged serde representation.

## Contract Verification

A one-time Rust/TypeScript contract check read every primary mutation leaf, excluding `↩️inverse` and `🔺️diff` implementations, and compared each Rust `pub struct` field list to its TypeScript payload interface after the Rust serde camel-case mapping:

```json
{"rustPayloadStructs":266,"matchingTypeScriptInterfaces":266,"failures":[]}
```

The mutation entity block is byte-identical to the sibling program schema's entity block and the mutations file has no sibling type import.

## TypeScript Verification

```sh
bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler --esModuleInterop --skipLibCheck --allowImportingTsExtensions "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts"
```

Result: exit code 0, with 0 errors.
