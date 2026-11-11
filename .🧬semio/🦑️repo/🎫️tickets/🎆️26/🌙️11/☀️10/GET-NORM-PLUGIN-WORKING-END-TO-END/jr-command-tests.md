# job-receiving and command tests

## State
- job-receiving: 3/9 (denial, underfunded, intrinsic pass). Compiles; transaction/pack quote API (`advance_demands` pub(crate), `*_grant`) fixed for the coordinator note.
- command --lib: 6/10 before last edit (close-after-take added, not yet rerun). check --all-targets not run.

## Done
- Projection error root cause: owned admissions (`admission_owned`) must use `project_owned`/`owned_projection_demand` (+`discard_projection` on cancel), not the plain `advance`; and a two-turn `take_output` must target one resident slot and drop the admission only once `terminal_is_empty()`. Fixed in copies and pack via steps `admission_*` helpers.
- Owners quote their own steps: `advance_grant`/`close_grant`/`take_grant` (pack, intrinsic, copies, transaction) over `settled_grant::<O>`; the candidate type O matters (Option<O> bytes are in the settled quote).
- Tests: per-step grants from those quotes, decoder teardown via `original_decoder_teardown_quote`, `retain_dispatch`. STOCK stays only as the caller wallet (cumulative cap, not the bound under test); per-turn grant capacity/release/depth now come from quotes.
- Command tests: per-step grants from the producer's own `demands/take_demands/close_demands`, `funded` = max over candidate outputs; refusal tests treat the quote error as the refusal.

## Open
- job-receiving phases that touch native-bound objects across turns fail ("native projection backing changed its original ownership", "native receiver backing changed original Work scope", pending receiving needing recovery): bindings are per work scope, so these must be driven inside the persistent `with_native_receiving` scope; the scope quote (items 1) is also too small for nested record receivers. Needs an owner redesign decision.
- command: bridge-refusal close never reaches terminal (settled admission keeps an untaken failure), ordinary-limit refusal message assertion.
