# Shared Dynamic Value Ownership

The framework Value module owns the JSON projection of `DslValue`, including its JSON Schema, TypeScript type/parser, GraphQL scalar, and Protobuf value representation. Plugin documents and payloads reference that owner instead of defining an empty placeholder or incorrectly narrowing a dynamic value to an object.

The TypeScript parser validates a finite JSON tree using an explicit stack. It accepts the six JSON value kinds, rejects cycles, non-finite numbers, undefined values, sparse arrays, symbol/non-enumerable fields, accessor fields, and array extension properties. It reads property descriptors without evaluating accessor values. Reusing the same acyclic subtree at two locations remains valid.

## Validation

Nine language-neutral JSON vectors passed against Ajv and the owned parser. Nine invalid construction recipes cover the non-JSON states. The first focused test reproduced acceptance of a symbol field; after correction, all recipes rejected and the accessor counter stayed zero. The focused value check, Wires document oracle, and shared artifact-addressing oracle then passed together through Nx in 5.7 seconds, each emitting its expected diagnostic.

The permanent route is root `📜️script.ts verify shared-dynamic-value`, exposed as `workspace:shared-dynamic-value` and in both launch catalogs. This validates the JSON-facing TypeScript contract; native number representation and binary codec behavior were not changed by this parser correction.
