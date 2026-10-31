# Durable Typing Gate Outcomes

The ticket logger now writes an observed child-close receipt under generated in addition to stdout. It preserves the exact child status and signal. It does not normalize an unknown close to success, infer a terminal result from a missing API handle, or change the existing signal exit policy. Existing running logger instances retain their original loaded code.

Three neutral JSON cases were executed through the actual registered repo tools ingress and validated with independent Ajv against a closed outcome schema. Actual Bun result:3 tests,33 assertions,0 failures.

| Payload | Observed Logger Child Status | Observed Signal | Observed Frontend Status |
| --- | --- | --- | --- |
| zero | 0 | null | 0 |
| seven | 7 | null | 7 |
| self-term | 143 | null | 143 |

The self-term payload signals only its own Node process behind repo tools exec; the existing frontend returns143 to the logger, whose actual observed receipt is status143/signalnull. No signal normalization was added. Tests also reject nullstatus/nullsignal and status0/non-nullsignal as authoritative successful close receipts. Every test verifies exact args, log path, positive PID, timestamp, durable JSON and stdout equivalence.

Inputs typing-logger-outcomes.json, typing-logger-outcomes.schema.json and typing-logger-outcomes.test.ts remain in this ticket. All tool output and outcome JSON files remain under generated for final ticket cleanup.
