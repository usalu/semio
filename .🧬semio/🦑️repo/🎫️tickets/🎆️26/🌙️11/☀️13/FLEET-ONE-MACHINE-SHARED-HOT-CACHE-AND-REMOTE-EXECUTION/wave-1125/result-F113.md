# F113: owner collision errors name the field, value and both owners

- Time: 11-26 17:41-17:45, lane F113, executor Sonnet, foreground, no sub-agents, no graph/run-many/cargo.
- Lock: `landing.lock.graph` taken only for the two atomic writes (temp + rename, via `T/🧰️scripts/F113/apply-owner-collision-details.py`), released right after. Typecheck and tests ran without the lock.

## Grep before change
`owner collision` occurs only in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts` (the three throws). No catalog (`🔣️.json`) or test referenced the strings.

## Change
- `readBuildOwnerRegistryV1` keeps the three first-line texts as the message prefix. Each throw is now a multi-line Error:
  ```
  Canonical build owner identity collision
  field: name|root|manifest mismatch
  value: "<colliding value>"
  earlier owner: "<name>" (root "<root>")
  current owner: "<name>" (root "<root>")
  ```
  Native-root and package collisions use `field: native root` and `field: package`.
- Maps keep the owner label per name, root (including native roots) and package, so the earlier owner is the one that holds the value.
- Deviation: a `manifest mismatch` is a single-owner check, so there is no earlier owner. The detail says `earlier owner: none (single-owner check)`. The current owner is still named.
- Bilingual catalog: `📖️texts` exists in this module, but these throws are plain `Error` values with English literals (the existing prefixes and the other `Canonical ... required` throws use the same form). Moving them into the catalog would change the error type and the message shape, so I appended a structured detail instead.
- Helpers `identityCollisionV1`, `nativeCollisionV1`, `packageCollisionV1`, `ownerLabelV1`, `collisionV1` sit above the function (docstring starts with an emoji, no comments inside definitions).

## Tests added (admission test file)
- `a build owner identity collision names the field, the colliding value and both owners` (field name and field root, exact line list).
- `a native owner collision names the native root and both owners`.
- `a package owner collision names the package and both owners`.
Each asserts the exact split lines, so both owners are named.

## Results
- `repo test-file 🧰️framework/.../🕸️graph/🚪️admission/🧪️tests/🟦️.ts`: 1 passed, 0 failed, 13 tests, 493 expect() calls, 2.5 s.
- `repo typecheck` on `admission/🟦️.ts` and `admission/🧪️tests/🟦️.ts`: passed, 55 files (53 importers), 0 errors, 77.5 s.
- `python3 T/🧰️scripts/coordinator/dangling-imports.py <admission folder>`: exit 0.

## Files (after sha256 prefix; the before hash was not recorded)
- `…/🕸️graph/🚪️admission/🟦️.ts`: after `b5b54d4976f8`
- `…/🕸️graph/🚪️admission/🧪️tests/🟦️.ts`: after `29216619851a`
- Kept script: `T/🧰️scripts/F113/apply-owner-collision-details.py`

## Not done
- No product behaviour changed beyond the error detail. The repo's 18:15-19:00 quiet window was not in effect at landing time (17:41), but the change is in graph code, so the coordinator should be aware.
