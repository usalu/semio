from specs.lib import *
FH = PL + '/🧪️tests/🧪️history-edit-acceptance'
CH = [
    ch('🚧', 'acceptance-error-limits-and-verdicts', 'acceptance_error_limits_and_verdicts', [(53, 134)],
       'Acceptance error, limits and verdicts: the acceptance error with its conversions, the per-leaf limits, the acceptance case, the verdict and the seed fault.', glob=False),
    ch('📚', 'acceptance-fixtures-cases-and-change-buckets', 'acceptance_fixtures_cases_and_change_buckets', [(136, 363)],
       'Acceptance fixtures, cases and change buckets: the failure debit law, the fixture directories, records, operations and compositions, the acceptance cases, the example cases and the change buckets.'),
    ch('🎛', 'acceptance-policy-pump-and-drive', 'acceptance_policy_pump_and_drive', [(365, 585)],
       'Acceptance policy, pump and drive: the policy, meta and verb helpers, the pump, close, seeded document, head and the drive of one leaf.'),
    ch('🔁', 'withdraw-restore-and-alternative-scenarios', 'withdraw_restore_and_alternative_scenarios', [(587, 903)],
       'Withdraw, restore and alternative scenarios: the withdraw and restore round trip, the session, the row label, the overwrite scenario, the alternative scenario, the first pass and the withdraw-only kinds.'),
    ch('🔢', 'end-to-end-assertion-census-and-search', 'end_to_end_assertion_census_and_search', [(905, 1146)],
       'End to end assertion, census and search: the history edits end to end assertion, the offered leaves, the census and the leaf search.'),
    ch('💾', 'reload-and-child-history-scenarios', 'reload_and_child_history_scenarios', [(1148, 1448)],
       'Reload and child history scenarios: the archive operation, the reloaded documents and heads, the document view and its difference, the reload identity assertion and the child history scenario with its assertion.'),
    ch('📐', 'input-schema-resolution-and-conflict-classification', 'input_schema_resolution_and_conflict_classification', [(1450, 1642)],
       'Input schema resolution and conflict classification: the input schema resolution failures and assertion, the applied ids, blocks, classification, withdraw accept, resolve and overwrite folds.'),
    ch('⚔', 'cascade-withdraw-edit-and-conflict-resolution', 'cascade_withdraw_edit_and_conflict_resolution', [(1644, 1916)],
       'Cascade withdraw, edit and conflict resolution: the cascade withdraw, the cascade edit, the cascade, the conflicts resolve assertion and the blocking executor.'),
]
SPEC = dict(orig='fhe.rs', file=FH + '/🦀️.rs', children=CH, wiring='block', external=mounted(FH + '/🦀️.rs', orig='fhe.rs'))
