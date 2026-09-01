# 🧪️ TypeScript Declaration Boundary Recipes

This is schema-first, ticket-only preparation. No recipe has been executed yet. It supplements exact small fact goldens with a finite syntax-size and literal-isolation boundary; it is not a global census or mutation-identity test.

The five authored recipes require: 2,048 independent exported interfaces; one real interface after a 1,048,576-character opaque comment; 128 nested namespace segments; an unclosed comment that cannot become complete empty facts; and a source expression that must remain unevaluated before a real interface.

Recipe expansion is defined independently of either parser. `interface-sequence` emits `export interface EntryN { value: string }\n` for N from zero to count minus one. `opaque-comment` emits `/*`, padding repetitions of `x`, the inert text ` export interface Hidden {}`, then `*/\nexport interface Actual { value: string }`. `namespace-depth` emits `namespace N0 {` through the requested depth minus one, then `export interface Actual { value: string }`, then exactly depth closing braces. `literal-source` uses its bytes unchanged.

The future test must validate the neutral schema with Ajv, parse these same strings with the independent TypeScript compiler, and compare owned declaration names/counts, alias count, completeness and source-coordinate bounds. It must check exact namespace path segments, not only depth. The malformed case must prove a compiler parse error independently. The executable-expression case must leave an explicit process-local probe unchanged. Candidate source must never be evaluated. References and subject results need separate actual counts and elapsed times; a missing export or timeout is failure, not a skipped pass.

Run finite stress children with an unchanged explicit deadline and retain their outputs. The recipe bounds limit manufactured test input only; they do not establish a runtime source-size limit or permit a parser to return complete after truncation. All production scanner/grammar and canonical test source changes remain owned by the declaration61 packet.

## Ticket Controller And Executed Evidence

[`📜️script.ts`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/📜️script.ts) is ticket-only. It first validates the unchanged five recipes with strict Ajv 2020, then creates one owned Bun child process per recipe with a fixed two-second deadline. Parent receipts retain each child's stdout/stderr, exit code, timeout status, and elapsed milliseconds before any failure is rethrown.

The independent reference uses the installed TypeScript 5.9.3 compiler AST. It compares each recipe's fixed expected completeness/count/first/last/alias/parse-error facts, every authored declaration name and half-open UTF-16 span, and exact namespace segments. Recipe expansion is local and independent of the compiler parser. The candidate expression remains only source text; neither reference mode nor child setup evaluates it.

The exact reference command was:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/📜️script.ts' reference
```

It exited `0`; the [reference receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/🧫️runs/reference-270613a5-44a2-4a8d-92b9-bd5e7f34ba2d/🔣️result.json) records 5/5 independent compiler cases in 884 ms total. Every child completed below the fixed 2,000 ms deadline; its own retained child receipt files carry the per-case elapsed values.

The subject command intentionally reruns the same five reference children first, then checks the actual discovery export with a TypeScript AST declaration lookup. It does not count an absent export as a pass. With the current D source, it retained [a real nonzero missing-export failure](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/🧫️runs/subject-8302bec4-b419-4c2f-9698-02cd6403e249/🔣️failure.json) after 5/5 reference cases in 1,034 ms. The paired result has `subject.status: "missing-export"`; its D whole-file hash is retained in the result capture and its helper hash is `null`. This is a source-boundary RED, not a subject execution claim.

Every input capture uses nofollow ancestry and endpoint metadata before/after reads. The receipts record the current D whole hash and the inspected helper hash separately, so a future D declaration-region change can be distinguished from unrelated drift. Once the export exists, subject children compare authored completeness, every declaration name, namespace path, half-open UTF-16 span, and alias count while asserting that `globalThis.__semioUnsafeDeclarationInput` stays unchanged; no candidate input is evaluated. The malformed-recovery case intentionally does not demand diagnostic-code parity with TypeScript: it requires incomplete facts and one or more in-bounds subject diagnostic spans.

## Frozen Subject Probe And Precision Correction

The initial frozen-D subject probe at `subject-d8c79be0-5cde-48d4-91aa-8f6d73526377` exited zero with 5/5 subject cases, but it established only completeness, declaration names, and module paths. It did not compare subject spans or aliases, so it remains retained historical, narrower evidence rather than the release proof.

The ticket-only controller was then tightened without changing D, declaration57, the canonical schema, or the five recipes. It now records each actual subject summary in the corresponding subject child receipt, compares subject declaration spans and alias count against the independent TypeScript reference expectation, validates every returned declaration/diagnostic coordinate is in bounds, and uses TypeScript's `canHaveModifiers` narrowing plus an explicitly typed parse-diagnostics collection.

The one post-correction subject command was:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/📜️script.ts' subject
```

It exited `0`. The [final subject receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-boundaries-62/🧫️runs/subject-1e435ab8-1aae-47f7-b5fa-c400f638e1de/🔣️result.json) records 5/5 independent reference cases in 1,024 ms and 5/5 subject cases in 1,865 ms. The five subject children completed in 367, 432, 436, 336, and 290 ms; none timed out. Its before/after captures are identical: D is `2ef678c682fc621f14a7f8557ef16f98ce19a22ab6a10530c73c9a96c304ec3e`, the inspected helper is `1d2a9352b321ad4ae5af4d75bc9f0c1c479a93d9737adb6f4cb861f24d42f395`, and the final controller is `03c7eca43847648aec28dbcbaac390dbebb20f56575db6df9d39d0b8362505e2`.

This is finite source-level parser/reference evidence only. It neither proves broader declaration coverage nor runs native/runtime behavior.

Suggested registration commands, not registered or launch-edited by this packet, are the `reference` and `subject` commands above. No production, canonical test, declaration61, launch, or native source changed.
