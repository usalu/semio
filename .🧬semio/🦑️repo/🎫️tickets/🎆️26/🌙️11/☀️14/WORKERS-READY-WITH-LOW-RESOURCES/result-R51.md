# R51: the group read that died at 17:11 UTC

## Culprit (proven by reproduction)
Tool row `typescript-package-index` (added by R29 in `SI/🔌️provider-closures/🔣️.json`; SI = `LIB/🏗️build/🕸️graph/📥️inference/🧭️script-inputs`). Its entry `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` re-exports `🔨️modules/🛂️manifest/*`. Three manifest files import the generated `./🤖️generated/🎚️ui-axes/🟦️.ts`, whose producer is `@semio-tech/ui-rs:generate`:
- `🛂️manifest/🟦️.ts`
- `🛂️manifest/📥️mutation-input-schema/🟦️.ts`
- `🛂️manifest/🔁️host-effects-choices-and-fault-notices/🟦️.ts`

`toolClosureGroupV1` refuses that (`closure.provider-generated-import`, port `typescript-package-index`, targets `@semio-tech/ui-rs:generate`). Scratch repro: `scratchpad/R51/repro3.mjs` and `repro4.mjs`. Only this one of 31 declared tools failed.

R29's hub rows had never been derived against the real tree with generated-output resolution; its tests used a fixture.

## Fixes (landed under landing.lock; see landings.md, R51 landings 1 to 4)
1. Landing 1: tool row removed from the policy and its corpus. The 25 nodes that referenced it walk the script themselves again, and their generated dependency is declared.
2. Lost message (landing 2):
   - The child now writes its refusal as data (code, key, values), and the parent re-raises it as a `RepoErrorV1`, rendered in en/de at the CLI boundary.
   - Any other child failure becomes the new bilingual `graph.heavy-question-failed` {exit, detail}, built from the error line and not the last three stack lines. The old `BUILD_HEAVY_QUESTION` plain Error is gone.
   - Law tests: `heavy-questions/🧪️tests`, 6 pass (child refusal with code/key/values in both languages, generic failure, refused-list mapping).
3. Blast radius (landing 2):
   - New `preflightToolClosuresV1` (script-inputs) walks all tool closures plus all provider ports in one shared cache and returns every refusal.
   - `preflightToolClosuresInChildV1` runs it in the capped child as the first step of `primeSharedClosuresOfWorkspaceV1`, which is the first step of every `buildOnce`.
   - It runs once per process and policy and refuses with the single refusal, or with `closure.tool-closures-refused` listing the tools. Cold cost is about 25 s under load 25.
4. Standing test (landing 2): `provider-closures/🧪️tests/🛃️preflight/🟦️.ts`.
   - Fixture: an injected generated import is refused with tool and producer, and the same tree without it is accepted.
   - Real tree: every tool closure and provider port is derivable (about 30 s).
   - The header of the ticket's `landings.md` now lists it as a standing landing test.
5. Landing 3 repaired two test bugs. Landing 2 had first imported the error module through the package re-export, which lacks `readRepoErrorDataV1`. The live tree was red for the sources of that import from about 18:26 to 18:29 CET (landing 2 released the lock without a fix; landing 3 fixed the import path under a new lock). Tests of the touched members were green again afterwards.

Tests after (all green): heavy-questions 6, preflight 2, provider-closures 3 dirs, script-inputs tests, build texts tests.

## Read state
- The server started a new read by itself at 17:14 UTC (`reason: request`, pid 91761), before my fixes landed. It passed the closure verification and the rederivation/flush phases without the refusal.
- It was still running at 17:32 UTC, in a later verification pass (5767 of 7392). Its in-memory policy is the old one; its children load the new code.
- Closure verification ran about 7392 closures in roughly 12 min, against the 90 min of the first read. That suggests vouching works, but it is not proven.

## Not verified
- The `settle` line (closures/slow/vouched/failed/wall) of the new read: not yet seen.
- What the dead read persisted in its owner records and `pending.json`: not inspected. `pending.json` is written only after a plan completes, so a read that dies earlier writes none.
- No test induces the real failure end to end (a child dying with a refused closure inside a running read).
- The whole-repo read was not started by me. I ran no `repo tools setup`.
