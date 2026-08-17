# Initial Debt Census

This census was taken from the current combined tree immediately after the Wave 0 leases were assigned. It is a baseline, not a completion claim.

| Signal | Baseline |
|---|---:|
| Catalog artifact roots expected | 36 |
| Filesystem artifact roots | 37 |
| Files containing `[DEBUG]` under stdio | 13 |
| `[DEBUG]` lines under stdio | 30 |
| `set-snapshot` command directories | 50 |
| `no-mutation` command directories | 1 |
| Files naming `CollectionMutation` | 0 |
| Paths below mutations matching `*set-*` | 268 |
| Rust inference child leaves | 193 |
| Rust inference roots | 58 |
| Zero-byte TypeScript leaves | 0 |
| Zero-byte Rust leaves | 0 |
| Files matching `placeholder`, `not implemented`, or `unsupported` | 264 |

The placeholder term count intentionally over-approximates real debt because standards-compliant diagnostics may legitimately say “unsupported” for malformed or opaque input. Closure policy must classify semantic placeholders rather than merely grep the word.

The `[DEBUG]` census includes live scratch/tuning output in the DEFLATE implementation and a CSV one-shot fixture generator, plus comments/fixtures that memorialize prior temporary probes. The final program gate requires removal of temporary runtime logging and a scoped absence check after retained evidence has been written into this ticket.

The mutation census shows that the program-wide semantic-command rollout is not a small cleanup: whole-snapshot replacement remains in 50 artifact/subset schemas, and generic setter vocabulary spans many command trees. Each replacement needs explicit domain commands with command-local sparse diff, inverse, touched-path, reference-integrity, rejection, replay, and serialization laws; directory deletion alone does not close the debt.
