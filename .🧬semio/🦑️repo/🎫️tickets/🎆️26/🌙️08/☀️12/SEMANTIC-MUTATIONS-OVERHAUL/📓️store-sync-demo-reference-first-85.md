# Sync Demo Verifier First Reference Failure

## Actual Outcome

The first registered `410.536` reference command exited **1**. It evaluated **14 of 25** authored cases and collected **21 passing checks, no failed check rows** before a fixture-construction exception. This is an incomplete failed reference run, not a successful suite or a production-source RED.

The exact exception was `Neutral replacement is not exactly one occurrence.` at controller lines 512 and 585. The next case was `changed-constructor-value`. Its twelve-space-indented `ops:` substring matches twice in the supplied mounted-source string, at UTF-16 offsets **8046** and **8700**: once in the intended direct function and once as a suffix of the actor-test indentation. `comment-only-constructor` uses the same ambiguous selector.

## Preserved Evidence

- [Original complete receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🧫️run-9jca2c/🔣️receipt.json), SHA-256 `32b7bd4c9b3467027cb08b24f93e21b13453752f57c0ed731229bf742c872041`.
- [Original controller output](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🧫️run-9jca2c/🧪️output.log).
- [Complete terminal tool output](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-sync-demo-reference-first-output-85.md).
- [Root structured review and registration readback](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-reference-first-85/🔣️root-review.json).

The controller retained sixteen first input captures but **zero final captures** because the exception bypassed final recapture. A subsequent independent root read found the same bytes, hashes, and file identities for all sixteen inputs. That later observation does not manufacture a controller endpoint. The first tool return and terminal tool result are retained; no exact command duration or start timestamp is claimed.

## Narrow Correction Proposed

Prepend a newline to the exact find and replacement strings in only `changed-constructor-value` and `comment-only-constructor`. The newline distinguishes the actual twelve-space line from a suffix of the twenty-space line. Preserve the exactly-one-occurrence guard, all expectations and cases, controller, schema, contract, candidate74, original failure captures, and production sources. Review the exact inverse before a separately coordinated rerun.

## Scope

No `source` command, canonical eight-leaf mount, Sync mutation splice, Rust compiler, native test, codec/provenance execution, or whole-monorepo census occurred. The private tests topology remains a proposal awaiting the independently required source desired-law boundary and Runtime approval.
