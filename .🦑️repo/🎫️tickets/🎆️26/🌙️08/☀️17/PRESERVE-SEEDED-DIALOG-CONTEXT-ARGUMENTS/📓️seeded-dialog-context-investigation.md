# 📓️ Seeded Dialog Context Investigation

## Result

The checked-out framework already implements the requested behavior.

- `effectiveActionArgs(defs, staged, seed)` initializes its result from `seed`, preserving non-declared context keys such as `spaceId`.
- Staged values override a seed with the same declared id; otherwise declared defaults apply.
- A zero-field confirmation dialog returns the complete `{ ...seed, ...staged }` payload.
- `UIDialog` passes `seedArgs` as the resolver's third argument.
- The Rust manifest implementation is the matching contract twin.

All other TypeScript resolver calls omit `seed`; therefore, no caller relies on filtering seeded keys. Focused validation passed:

```text
bun x nx run @semio-tech/framework:test -- --run glue.ts
2 test files passed; 158 tests passed.
```

No product files were changed because both implementation and focused regression coverage already exist.