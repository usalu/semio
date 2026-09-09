# Directory Administration Wire Admission

## Outcome

The OS private worker decoder now rejects every unknown `directory-administration-*` kind and decodes the six declared administration request variants through closed field sets. Operation epochs are safe nonnegative integers; transfer epochs are safe positive integers; `copied` is boolean; request identifiers are nonzero lowercase 32-hex; space identifiers are nonempty, control-free, and at most 256 UTF-8 bytes; cursors follow the existing administration page grammar and 1024-character bound.

Directory commands use the new first-party `parseDirectoryCommandV1` from the canonical directory schema module. It retains the existing declaration-order canonical JSON law, closed fields and control rejection, and adds scalar, enum, positive-integer, descriptor hash, and descriptor frontier validation. The distinct shared `canonicalDirectoryCommandV1` reconstructs a validated command after an order-independent carrier. The Pack boundary uses it because canonical Pack maps decode in lexical key order; it rebuilds the complete command, descriptor owner, and bootstrap frontier declaration order without duplicating the ten-variant field list in OS.

## Neutral and independent evidence

- Neutral worker fixture: `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🏛️administration-worker-wire-v1.json`.
- The law round-trips all seven envelope rows covering the six variants, then wraps all ten canonical directory command variants from the shared language-neutral administration corpus.
- AJV independently validates every accepted command against `DirectoryCommand` in the draft-07 directory schema; `fast-deep-equal` independently compares each Pack round trip.
- Twenty-four neutral hostile rows cover unknown kinds, per-variant missing/extra fields, negative/fractional/unsafe operation epochs, zero transfer epochs, non-boolean capability results, zero/uppercase request IDs, invalid space/cursor controls, wrong scalar types/enums, and an impossible descriptor frontier. Ten shared malformed commands plus two generated maximum-bound hostiles cover null/array/scalar/missing/extra/noncanonical commands and the 256/1024 limits.

## Execution evidence

- RED: `🗑️generated/directory-administration-wire/red.log` — the old cast-through decoder accepted `directory-administration-forged`.
- Intermediate RED: `🗑️generated/directory-administration-wire/green-focused.log` — exposed lexical Pack map ordering against declaration-ordered canonical JSON.
- First GREEN: `🗑️generated/directory-administration-wire/green-focused-2.log` — one selected law passed, 339 skipped, exit 0.
- Final nested-byte qualification: `🗑️generated/directory-administration-wire/green-focused-final.log` — asserts exact `JSON.stringify(decoded.command)` equality for all ten commands, including the nested announced descriptor.

Command:

```text
bun nx run @semio-tech/framework-os:test-long -- --testNamePattern='admits only closed semantically valid directory administration requests'
```

No worker admission code, server route, Cargo target, browser process, or native artifact was changed or qualified by this packet.
