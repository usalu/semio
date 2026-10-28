# Canonical Text Thread Consistency

Canonical native and TypeScript admission now require every text edge to join frames of the same story, with at most one predecessor per target. Cycles and missing/non-text targets remain refused. Multiple independent heads may reference the same story. Local threads may span physical pages; each parent template remains its own content graph and is later instantiated per page.

The targeted helper now accepts the proposed story explicitly: `validate_thread_edge(document, source, story, next)` / `validateThreadEdge(document, source, story, next)`. It checks incoming story identity, target predecessor uniqueness and the reachable outgoing chain. It scans local edge headers without revalidating unrelated typography, bounds, links or story content. CreateFrame supplies the candidate story before insertion. UpdateTextFrame always calls the helper, including when threadNext is unchanged; changing either an outgoing source or incoming target's story can no longer create a cross-story edge.

## Test-First Evidence

Nineteen language-neutral mutation witnesses were added before the guard repairs: unchanged outgoing/incoming story changes, new cross-story edges, local forks through updates/creation/page creation, parent creation/content replacement and whole-document replacement refusals, plus valid disconnected heads, same-story reconnection, cross-page connection and ordinary edits retaining an existing edge. The actual pre-repair TypeScript evaluator failed the first witness: `reject source story change with unchanged outgoing edge: semantic publication`. Six document admission vectors and three parent-content vectors were also added.

The test-owned independent oracle projects mutations using RFC 6902 and checks story identity and maximum predecessor count using Ajv facts before Kahn cycle ordering. It does not call production admission. New parent-content projections independently normalize membership and stale overrides. Existing native semantic graph laws consume all new cases through actual mutation diff, central application, central inverse, reversed semantic inverse and text/binary transport.

## Executed Evidence

- Canonical evaluator: full strict diagnostics 0 over 373 actual sources; 115 portable semantic cases, 51 golden scenarios, all58 variants,40 IEEE word cases and two drawing-owner cases pass.
- Whole/targeted validator: strict diagnostics0 over234 actual sources;67 document vectors,12 parent vectors and16 targeted assertions pass.
- Independent graph checker:76 semantic graph cases (22 accepted,54 refused),three nonfinite gutter word witnesses;67 documents/12parents; all102 golden snapshots valid;16 independent IEEE identities pass.
- The existing Rust grammar audit passes42 affected sources. Native compilation and native laws remain unrun.

## Files

Shared owners: `🧬️schema/✅️validation/🦀️.rs` and `🟦️.ts`; callers: native `🧬️mutations/📑️update-text-frame/🦀️.rs`, TypeScript `🧬️mutations/⚙️evaluate/🟦️.ts`, frame-creation helper and targeted validation test. Neutral fixtures: `🧪️tests/🧫️fixtures/🕸️graph/🔣️.json` and `✅️validation/🔣️.json`. Independent oracles: `🧬️mutations/🧪️tests/{🕸️graph,✅️validation}/oracle.mjs`. Retained execution input: `thread-validation-check.mjs`.

The bounded canonical frame/thread projection into TextFlowCursor is the next reserved task; this report does not claim that integration is complete.
