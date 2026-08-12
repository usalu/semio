# Status

## Wave 0 — Mechanism: DONE

- Core traits/derive/testkit laws/DiffKit: see `📓️wave0-mechanism-report.md`. Verified via
  `cargo check -p semio-framework-os-kernel` (clean), `cargo test -p semio-framework-os-kernel --lib
  -- command::` (23/23 pass incl. the full derive-macro fixture), and a downstream plugin sanity
  check (`semio-s-plugin-draw`, `semio-s-plugin-cad` both clean).
- Policy rules: see `📓️wave0-policy-rules-report.md`. `policySemanticVocabularyBreaches` (342-entry
  seeded allowlist),  `policyMutationDispatchCoverageBreaches` (real impl, medium priority),
  `policyMutationTsMirrorBreaches` (low priority). Verified `bun ./📜️script.ts policy` clean (0 new
  high-priority breaches) and `bun ./📜️script.ts verify gate` unaffected (identical pre-existing
  failure before/after, from unrelated `no-circular` TS glue violations — not this ticket's scope).
- Vocabulary docs for fan-out waves: `📓️taxonomy.md`, `📓️derivation-rules.md`.

## Concurrent-repo-churn note (observed during Wave 0, not caused by this ticket)

While verifying, found several plugins transiently broken for reasons unrelated to mutations:
`semio-s-plugin-note`/`semio-s-plugin-writer` (missing `STDIO_JSON_DOCUMENT_SCHEMA` / missing
`📌️panels/📄️document` files) — consistent with a concurrent, repo-wide panel-restructuring pass
running elsewhere. `draw`, `cad`, `gis`, `fem` confirmed stable and picked as Wave 1 exemplars
instead of the original note/draw/flow/writer picks (note and writer/flow were unstable at
selection time).

## Wave 1 — Exemplars: LAUNCHING

4 plugins / 6 facets (draw, cad, gis×2, fem×2), each with a migrate agent + adversarial review
agent, via a background Workflow. Script: see the workflow run started after this doc was written
(check `/workflows` or the next task notification for its run id).

## Remaining (waves 2–4, per the plan)

Mass fan-out across the other ~101 facets, allowlist burn-down (342 semantic-vocabulary entries +
whatever Wave 1 doesn't clear), and final integration verification (undo/redo round-trips, full
workspace test). Not started — scale (~90M tokens, days of wall-clock per the original plan
estimate) means these continue across subsequent turns/notifications, launched automatically as
each prior wave completes, per the "full run, no pause" instruction.
