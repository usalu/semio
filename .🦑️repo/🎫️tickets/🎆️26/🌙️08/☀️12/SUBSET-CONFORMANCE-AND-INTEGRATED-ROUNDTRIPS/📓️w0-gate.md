# G0 Gate

## Census (coordinator + explorers)

| Metric | Value |
|--------|-------|
| Real subsets | 138 |
| Phantom trees | 2 (`md/🏅️标准`, `xml/🏅️标准`) |
| Missing inferences | 66 |
| Missing both IO | 54 |
| Meta-only TS (≤7 lines) | 42 |
| Missing subset engines | 138 |
| Artifact-level examples | 123 |
| Standard-level examples | 5 |
| Subset-level examples | 0 |

## Hot-file protocol

UCAS owns framework `🔌️plugin`, `🏪️store`, `🚪️io`, `🧬️schema`, `📡️spr` for W1 composition work.
APA released `📜️script.ts` into UCAS→SMO→IIF queue.
SMO RELEASED many domain plugins for fan-out (see live `SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`).
`🗄️stdio` held by UCAS.
HELD by SMO: architect, animate, process, reasoning; between-waves: writer, vcs, flow, sequence.

## Decision for W1

Mechanism edits are **additive appends only** at file tails / new regions, announced in freeze ledger.
Taxonomy ownership flip for subset examples/engines lands as medium-severity policies first.
`schemaChildDirs += 💡️inferences` remains owned by IIF — this ticket consumes inference APIs and places subset inferences without flipping that key until IIF seals.

## Gate status

**G0 PASSED** — proceed to W1 mechanisms on additive surfaces + migrate SMO-RELEASED plugins in parallel after W1-HARNESS lands.
