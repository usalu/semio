# Plugin registry close review

Source review identified two issues in the current adaptation. Private child group empty-backing close uses should_yield, which observes an expired semantic deadline even inside a genuine original-retirement loan; physical close should use retirement_should_yield so paid original cleanup can proceed. The archive empty-backing branch already uses that close-specific predicate. Both check the zero-fuel condition before the backing action and debit the actual returned work and physical progress; a failed producer physical receipt is passed to debit_original.

Archive retirement with no target and no backing work enters original_lifecycle_step, whose unchanged-fuel fallback consumes one unit even though drive_document_archive_original returns Complete with empty progress and performed no selected action. A terminal no-action return ahead of the helper preserves zero work. These are source findings, not runtime verification.

## Follow-up

The current private-group backing branch uses retirement_should_yield. Archive retirement returns Complete with empty progress before the lifecycle helper when no target and no applicable backing action exists. Both prior source findings are addressed. No runtime verification was performed.
