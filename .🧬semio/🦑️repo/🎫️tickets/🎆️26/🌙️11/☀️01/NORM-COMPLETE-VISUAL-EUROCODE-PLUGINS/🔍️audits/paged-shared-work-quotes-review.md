# Paged And Shared Retirement Work Quotes

Source review found no concrete quote/action mismatch. PagedListRetirement quotes one while unreleased after validating selected physical demand, zero after release. Its admitted bulk plain-element loop returns ProcessedBytes, child transfer returns Child, page release returns Bytes and final owner release returns Complete(1); these intrinsic operations define work 1 in RetirementStep::work, independent of physical progress. BudgetExhausted reports zero. The bulk loop is one bounded page action under its byte frontier.

SharedControlledRetirement quotes source lease handoff as one; once source is absent it delegates to nonterminal ControlledRetirement or quotes one for terminal owner clearance, zero at terminal. Its actual step reports last_work=1 for shared/unique handoff including a raced try_unwrap attempt, delegates the child last_turn_work on success or error, and reports one on owner clearance. Denials return zero. The quote does not infer CPU from allocation or release progress. The shared/unique behavior and concurrent alias race still require runtime coverage.

Task-resume delivery_fault now tests work_refusal as well as effects/receipt/context and preserves the original failure when populated. New native tests were not compiled or executed in this review.
