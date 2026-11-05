# Descriptor Hint Timing Review

Root re-read the current generation store implementation while descriptor19405 retains its second verification pass. This was a read-only check of a concrete timing hypothesis, not a change to graph policy.

In refresh(), the reader verdict is awaited before the hint is created. Its atMs is set with now() after planning (current production generation module lines513 and519). Therefore a long first verification pass does not itself age the hint before creation. The proposed pre-plan timestamp defect is contradicted by current source.

build() first reads the current generation, then accepts the hint only when the identity/manifest is usable, the pointer generation still matches, and its age is within the schema policy (lines425–429). A changed peer generation or elapsed builder wait can legitimately require another plan. Existing plan-once tests explicitly include age boundary cases. Current logs do not identify which predicate refused reuse, so repeated progress counts alone do not prove a loop or a missing cache.

No source, policy limit, freshness window, custody, resource grant or receipt was changed. Existing descriptor/native/browser handles remain retained. Native19689 was independently confirmed live by its owning tool poll during this review, still before source admission. Actual emitter/compiler/test results remain required.

## Actual Second-Pass Boundary

The owning integration worker authoritatively observed retained descriptor19405 complete1397/1397 in pass two and enter graph refresh for550 projects at generation897. It reported3896 stale facts, including the new public PatchQuestions owner, mounted TextEditor test, archive target and cold-Wasm resolver. There were exactly two full-pass starts at that boundary, no third pass, and no compiler/refusal/terminal result. This is concrete inference progress and contradicts treating the repeated count as an established infinite loop. The existing handle remains retained.
