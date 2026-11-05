# Fold Split Neutral Review

Added strict foldTransitions cases to the permanent History work corpus: separate teardown1 then handoff1; cancellation after teardown retains result, paid capture1, childwork3, terminalclear1. SQLite independently checks ordered action/owner trace and Ajv validates it. Actual focused suite2passes/100assertions/152ms exit0. No production Fold edits.

Read-only source review: completed uses completed_final after control release; next_demand and currency getters guard control_closed or retained original_output before dereferencing control. Original ready_copy now uses closed+successful-result state without released control. No concrete released-control dereference found in these inspected paths.

Remaining cancellation boundary: request_cancel after teardown sets cancelling=true, but step_granted_result closed+resultOk branch does not inspect it before handing output out. The close_original route retains it into ControlledRetirement and paid closure correctly; normal step after cancellation can still deliver unless caller exclusively chooses closure. Root notified to pin actual contract with native regression. Native execution remains pending.
