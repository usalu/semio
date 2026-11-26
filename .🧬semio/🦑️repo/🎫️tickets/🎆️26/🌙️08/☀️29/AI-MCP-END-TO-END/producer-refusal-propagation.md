# Producer refusal propagation

The plugin router copied terminal native causes as text. The inference artifact channel represented every router error as `app.command.rejected`, which the gateway surfaced as `SIDE_EFFECT_REJECTED`. This path discarded original cancellation and finite resource refusal kinds.

The paid router capture now carries the original value kind along with its diagnostic. The same capture handles both early operation refusal and a native job failure. A plain cancellation maps to `CANCELLED`. Work, ownership, allocation and depth refusals map to `BUDGET_EXCEEDED`. Text and other native kinds still use the command-refused status. Compound evidence retains the original primary and independent receipt, context, work, settlement or closure diagnostics within the paid capture. A distinct cleanup failure prevents a standalone resource or cancellation classification.

The fixture covers each lower-level kind, disconnected secondary evidence, and text that merely names cancellation or resource exhaustion. Bun, Ajv and SQLite passed all 11 cases and the schema check via `repo test-file`. Native tests cover the paid cause capture and the resulting MCP fault projection. Both production crates compiled and the native gateway linked during the successful `folder_reopen` test run.

The native SDK replay through that linked gateway returned `BUDGET_EXCEEDED` after 20.963 seconds for the one-pixel WFC pair and verified the exact persisted pack and SPR afterward.

The filtered native router test build reached the production crates and stopped on the existing artifact-mutation router test. That test still used removed relay helper imports and an outdated instance constructor, registration object and wake-provider call shape. The native SDK result and neutral refusal cases remain independently verified.
