# Puzzle Fill Retained Preview JSON Third Adversarial Audit

## Verdict

**RED.** The second remediation correctly rejects hostile extra root, diagnostic, and ghost keys, and the diagnostic/ghost exact censuses are present.  The parser still fails the declared JSON schema at the fill root: it only checks that root keys are *allowed*, never validates any present root values.  It also does not enforce the requested exact nine-key root census.

No production source was edited.  Cargo, Nx/Bun/Vitest, Wasm, browser, cache, and runtime commands were not run.

## Bounded Defect

### RED-1 — Root fields are admitted without schema validation or an exact census

The schema declares nine root properties, `additionalProperties: false`, and types/limits for each property (including `color: string, maxLength 128` and `opacity: const 0.35`).  The root parser only executes:

```ts
censusAllowedOwnKeys(parsed, WORLD_FILL_ROOT_KEYS) < 0
```

at `World3dHost/🟦️component.tsx:1156`.  It has no root field predicates.  Therefore a payload with the fully valid 24-key diagnostic and `candidateGhost: null`, but root `color: 17`, `opacity: true`, `origin: "bad"`, or `sourceVortexIndex: -1`, passes the parser although those present values violate the schema.  No renderer law rejects these hostile known root fields.

The same `< 0` condition accepts a root containing only `fillBuildPreview`; it does not require `WORLD_FILL_ROOT_KEYS.size` (nine).  The renderer law labels this one-key root `exactRootAndDiagnostic` and asserts acceptance at `index.test.ts:2752-2755`.  This contradicts the requested exact nine-key root census.  The fixed encoder itself emits only `fillBuildPreview` when `candidateGhost` is absent (`fill/🦀️component.rs:300-329`), while the schema only lists `fillBuildPreview` as root-required.  The implementation must make this one contract coherent: if the retained wire shape truly requires nine root keys, make schema requirements, encoder, parser, and laws agree; otherwise retain the optional-root schema but validate every present root member and remove the exact-nine assertion from the contract.

## Green Static Evidence

- Schema diagnostic properties and required list are both 24; ghost properties and required list are both 6.  `candidatePage` is bounded to exactly eight by schema and parser (`World3dHost/🟦️component.tsx:1210-1212`), with seven/nine negative laws.
- Parser rejects unknown root keys, requires exactly 24 diagnostic keys, and requires exactly six ghost keys (`World3dHost/🟦️component.tsx:1156,1165`).  Renderer laws cover hostile extra root, diagnostic, and ghost keys (`index.test.ts:2756-2757,2795-2797`).
- The retained cursor has 4096-byte output, 128-byte color, and 256-byte status-label caps; it stops on cancellation/deadline/zero fuel, keeps the retained ready value, validates exact encoded length, and closes owners one at a time (`fill/🦀️component.rs:28-30,501-621,632-675`).  The page call uses 256 single-fuel grants and a two-millisecond deadline (`⏳️precompute/🦀️component.rs:1437-1453`).
- Puzzle3d and Puzzle5d pass their active localized `fill_progress` labels to the same retained-page API and keep brush-only output gated by the brush utility (`Puzzle3d main window:404-411,460-471`; `Puzzle5d 3d window:188-219`).
- The renderer requires a non-empty capped status label and renders that exact value visibly and in the status ARIA label (`World3dHost/🟦️component.tsx:1192-1195,2917-2949`).
- Scoped aggregate-serialization census found no matches for the three removed aggregate routes.

## Commands and Results

| Command | Result |
| --- | --- |
| `jq -c '{root:{propertyCount:(.properties|length),required:.required},diagnostic:{propertyCount:(."$defs".diagnostic.properties|length),requiredCount:(."$defs".diagnostic.required|length)},ghost:{propertyCount:(."$defs".ghost.properties|length),requiredCount:(."$defs".ghost.required|length)}}' preview-json.schema.json` | `{"root":{"propertyCount":9,"required":["fillBuildPreview"]},"diagnostic":{"propertyCount":24,"requiredCount":24},"ghost":{"propertyCount":6,"requiredCount":6}}` |
| `jq -e '(.properties|length)==9 and (."$defs".diagnostic.properties|length)==24 and (."$defs".diagnostic.required|length)==24 and (."$defs".ghost.properties|length)==6 and (."$defs".ghost.required|length)==6 and (.properties.opacity.const==0.35)' preview-json.schema.json` | `true`, exit 0 |
| `nl -ba World3dHost/🟦️component.tsx \| sed -n '1147,1228p'` | Root uses an allowed-only census; diagnostic and ghost have exact-count censuses; no root field validation exists. |
| `nl -ba renderer test \| sed -n '2725,2800p'` | The accepted `exactRootAndDiagnostic` payload has only `fillBuildPreview`; hostile extras and ghost/page/label negatives are covered. |
| `nl -ba fill/🦀️component.rs \| sed -n '240,412p;452,675p'` and `nl -ba ⏳️precompute/🦀️component.rs \| sed -n '1435,1453p'` | Fixed encoder/caps/cursor/fuel/deadline/retained-page paths present. |
| `rg -n 'serde_json::to_vec\\(&self\\.preview\\)|serde_json::to_value\\(build\\)|fill_progress\\(\\)\\.preview' Puzzle3d Puzzle5d` | No matches, exit 0. |

