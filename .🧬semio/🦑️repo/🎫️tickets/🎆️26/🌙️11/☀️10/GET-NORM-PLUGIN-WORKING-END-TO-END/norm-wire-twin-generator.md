# Norm wire twin generator

Finding: the generator is not stale. It is the framework command `repo ./📜️script.ts schema twins generate|check` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection`), driven by `✏️s/🔌️plugins/📕️norm/🧬️twin-dialect/🔣️.json`. The old `🧪️s2-norm-ts-twins.ts` (ticket 26/09/30) is superseded. `repo ./📜️script.ts` cannot run now (no process envelope), so it was driven through `repo tools exec bun` with a scratch driver calling `runTwins` (scratchpad, not kept).

Done:
- `generate --under ✏️s/🔌️plugins/📕️norm --only typescript` rewrote 2630 twins; `check` now: identical=2631 style=0 drift=0.
- din18599 artifact twin is generated and contains `thermalMassClass` and `cWirkWhM2k` (the schema already declares them with defaults); the hand edit is now the generator's output.
- New drift gate `🧪️tests/🧪️wire-twins/🧬️drift/🟦️.ts` (`check --strict`, fails on any hand edit); `test-wire-twins` (`norm-js/📦️packages/🟦️typescript/📜️script.ts`) runs it with the wire-twin witness.
- Tests (via `repo tools exec bun -- test`): wire-twins 11853 pass + drift gate, 0 fail.
- An interim copy of the s2 generator I ported was deleted (it diverged from the framework dialect).
