# History Work Chain Review

Read-only current chain review; no edits/jobs.

Remaining concrete issues sent to root:

- Edit::accept and Source::accept still clear actual work whenever physical receipt is absent. This can erase genuine child work already forwarded; Metadata fixed this same pattern.
- Origin JSON captures work inside scoped_maximum but reads grammar.normal_step_progress outside the callback. A pre-callback refusal can therefore republish prior physical progress. Capture both local work and progress inside the callback, each initialized zero.
- Concurrent Source ObservedSource instrumentation adds source byte reads to semantic counter then takes max with native actual_work. It includes demands/preflight reads before grant admission and may count the same parser action twice. This is a changed work definition, not proof of the explicit-action neutral contract; its logical owner needs to reconcile definitions and denied-entry behavior.

Positive current evidence: Source forwards all four child getters; Edit forwards Metadata; Metadata forwards Origin JSON; Origin JSON forwards actual parser and Projection work; Projection/Partial and Edit/Conflict retain actual retirement/admission work including errors. Public resets and charge-denial resets exist at inspected boundaries. No native pass inferred.
