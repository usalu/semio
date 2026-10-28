# result-cmdid (lane cmdid, mirror prefix `ci`, wave 1027b; C = this folder)

FOR coordinator: ONE batch `identity` (4 files, all `base` on live at 18:44, template stage.py, mode law OK, scan 2 TS bad 0, both JSON parse, `tsc --strict` over the two TS files: only the known `🚧️operands` error). Security fix plus law and fixture changes, no new module.
- FOR coordinator: apply `python3 -I C/cmdid-tools/staged/identity/stage.py apply --root "$PWD" && sh C/cmdid-tools/staged/identity/verify.sh "$PWD"` (about 1 min; expect `VERIFY OK identity`). Revert `python3 -I C/cmdid-tools/staged/identity/stage.py revert --root "$PWD"`. Drift: `python3 -I C/cmdid-tools/make.py` rebuilds the batch from the current live bytes (anchored edits that fail loudly when an anchor moves).
- FOR coordinator: product proof: none needed. Production never calls the check under a pin (it runs first thing in a fresh captured child, before any invocation is received), so the only production change is that a RAW entry (no captured identities) now gets the plain refusal instead of exit 75 + restart. Optional: `bun repo run @semio-tech/repo-lib:test-build-command-line`.
- FOR coordinator / rusttests: order-independent with rusttests `envelope` (that batch adds 2 lines after line 21 of `⌨️command/🧪️tests/🟦️.ts`; my edits are anchored elsewhere). Whichever lands second must be rebuilt (`sh C/rusttests-tools/rebatch.sh` or `python3 -I C/cmdid-tools/make.py`). After BOTH land, rusttests `expect.tsv` line 10 (`2/4 command: real pin leak …`) must become `5/0` with no known-open note; verified in mirror `ci-a` with both batches: `5/0` bare and `5/0` beneath the genuine parent.

## The bug
`requireCapturedBuildCommandV1` (`🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts`) ran its ENTIRE identity check behind `if(!implementationPinnedV1()&&(…))`. zt-g's pin-guard sweep (law "every self-check of a loaded implementation lets a pinned invocation finish" in `🕸️graph/🧪️tests/🕰️generation-protocol`, which only requires that a line with `Stale loaded` also mention `implementationPinnedV1()`) wrapped the authentication part together with the stale part. So under any pin:
- an entry evaluated WITHOUT the captured loader (`import.meta.semioCapturedImplementationIdentity` absent) was admitted with a forged proof: the ingress law's raw entry with a fake proof ran its body instead of `Authenticated captured …`;
- an entry compiled for another snapshot or loader was admitted too.

Even without a pin it was wrong for the raw case: an absent identity was reported as `staleImplementationV1` (exit 75, the outermost command restarts), not as an authentication refusal.

The "pin leaking across tests" is not a leak in the pin code. `withBuildTestInvocationV1` receives ONE birth per test file through `receiveScriptProcessInvocation`, which wraps it in `withReceivedImplementationPinV1(run, processWide=true)`, and that birth stays open until `afterAll`. That is the documented "this process runs one invocation" pin. So every later test in the file, even a synchronous one outside the invocation, sees `implementationPinnedV1() === true`. Run bare, law 2 is refused before its birth, so law 3 passed. Beneath a parent, law 3 failed. Laws about pin-sensitive behaviour must set the pin state themselves; the new law does.

## Fix (schema-first)
- `🧬️schema/🔣️.json`: new `CommandIdentities` (both evaluated identities are 64-hex strings). `CommandCase` is now `{id, identity, loader, proof: loaded|peer-saved, unpinned: accepted|authentication|stale, pinned: accepted|authentication}`. The schema itself says that a pin never restarts and never admits.
- `requireCapturedBuildCommandV1`:
  1. A malformed proof or missing/malformed evaluated identities gives `Authenticated captured build command entry required`, always.
  2. Well-formed identities that differ from the proof, or a proof loader that differs from the loaded loader constant: unpinned → `staleImplementationV1(…)` (restart, as churn-g designed, loader-identity law unchanged); pinned → the same plain authentication refusal (`throw implementationPinnedV1()?Error(…):staleImplementationV1(…)`, so the pin-guard law stays green).
- Fixture `🚪️command/🔣️.json`: 7 cases (added raw-source-identity, raw-loader-identity, peer-saved-loader).
- Law `⌨️command/🧪️tests` 3: each case runs explicitly unpinned (`withoutImplementationPinV1`, asserting `implementationPinnedV1()===false`) and pinned (`withImplementationPinV1`, asserting `true`). The outcome is classified by error code and message, and SQLite (independent oracle) recomputes each outcome from the raw values.
- Laws 4 and 5 were stale and hidden by the cascade (they evaluate the actual `executeCapturedBuildSourceV1` text with injected ports, and it now reads `buildSourceClosureTextV1`, which raised a ReferenceError). They now get the real port, plus `missingCommandPortsV1`, which names any imported binding the function reads that the ports lack, so future drift fails with a name.

## Laws (mirror `ci-a`, live 18:35; `C/🗑️generated/cmdid/*.txt`)
| run | base code + new laws (`red.txt`) | batch (`green.txt`) | batch + rusttests envelope (`green-with-envelope.txt`) |
|---|---|---|---|
| command file beneath the genuine `repo-lib:test-build-command-line` parent (`wire.ts`) | 1/5 (`raw-entry unpinned`; original live file: 1/5, ingress resolves) | **5/0** | **5/0** |
| command file bare | 1/5 | 2/4 (only envelope-less laws run, as before) | **5/0** |
| loader-identity law | 3/0 | 3/0 | 3/0 |
| generation-protocol pin-guard law | 1/0 | 1/0 | 1/0 |

Neighbours (files that load the command module or run captured kernels: project-farm captured, memo edit/verified, process-chain, closure stream, sealed trampoline/processes). Their failure SETS are identical between base and batch, bare (`neigh-base.txt` = `neigh-batch.txt`) and beneath the parent (`parent-neigh-*.txt`): process-chain 3/7 bare and 6/4 parent, stream 8/12 and 16/4, all pre-existing (envelope, tree bounds, `Immutable implementation snapshot changed`). Every other file is green.

## Tools (`C/cmdid-tools/`)
`make.py` (batch builder, anchored), `stage.py` (template), `verify.sh`, `wire.ts` (genuine parent envelope from the cwd checkout), `neighbours.sh`, `parent-run.sh`. Mirror `ci-a` dropped at the end.
