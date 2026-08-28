# Independent Source R22 and Full Command Census R6 — 2026-08-27

## Current Outcome

The canonical self-test actually passes 1,009 checks, exit 0. The full command census completes and remains RED, exit 1: 270 live registrations and twelve distinct failure entries remain. A passing verifier self-test is not an all-app implementation or runtime pass.

The earlier bootstrap failure depended on a disappeared active-ticket input. Production verification now consumes the existing native domain checkpoint fixture and its corrected schema. No unknown lost bytes were reconstructed. The native five-vector checkpoint fixture and production codec were preserved. Independent schema and binary-oracle tests cover the exact-capacity admission boundary and hostile cases.

## Independent Self-Test

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'
```

```text
> nx run workspace:verify-interactivity --args=tool-jobs --self-test

> bun ./📜️script.ts verify interactivity tool-jobs --self-test

[verify interactivity tool-jobs] exact-factory-proof-owners=33 custom-rows=255 generic-rows=25 clean.
[verify interactivity tool-jobs] self-tests=1009 clean.



 NX   Successfully ran target verify-interactivity for project workspace
exit_code=0
```

## Independent Full Census

The canonical JSON was summarized from stdout without relying on the disappeared publication file.

```sh
set -o pipefail
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --format json' | bun -e 'const s=await Bun.stdin.text();const start=s.indexOf("\n{");const end=s.lastIndexOf("\n}");if(start<0||end<start)throw new Error("Canonical census JSON missing");const r=JSON.parse(s.slice(start+1,end+2));const counts=Object.fromEntries(Object.entries(r).filter(([,v])=>typeof v==="number"||typeof v==="boolean"));console.log(JSON.stringify({counts,remainingCount:r.remainingCommands.length,failures:r.failures,firstRemaining:r.remainingCommands[0]},null,2));'
```

```json
{
  "counts": {
    "macroHostFiles": 50,
    "macroInvocations": 50,
    "commandRows": 773,
    "uniqueCommandRows": 771,
    "fixtureMacroHostFiles": 1,
    "fixtureMacroInvocations": 2,
    "fixtureCommandRows": 4,
    "boundedRows": 350,
    "batchOnlyRows": 315,
    "forbiddenRows": 2,
    "deletedRows": 0,
    "productionFactories": 51,
    "productionRegistrations": 256,
    "productionDispatches": 3,
    "aliases": 4,
    "literalRegistrations": 708,
    "selfTests": 1009
  },
  "remainingCount": 270,
  "failures": [
    "typed command preparation lacks a fixed-width event-maintained immutable child-content root and no-default terminal-witnessed old-root retirement authority",
    "Jack `.spr`/`.ops` envelope caller lacks the shared retained edit decoder, exact child retirement, fixed-page ingress, initializer recovery, cancellation, or exact completion acknowledgement",
    "Trinity Rewrite envelope caller lacks the Jack-owned fixed-page operation store, generation handle, bounded progress/cancel/close, exact rejected-page handback, or completion acknowledgement",
    "child snapshot retirement domain cohorts or callsites do not match the exact machine-readable owner inventory",
    "runtime instance/actor authority still grows, hashes, scans, shifts, blocks, or drops the detached app on InstanceClose",
    "instance close still permits implicit nested payload destruction or lacks saturation-safe bounded cleanup job ownership",
    "extra bounded reducer proof lacks its exact Migrated declaration ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs\u0000semio_framework_plugin::EditorApp<CadPlayApp>\u0000s.cad.cad@1/*#editor\u0000CadRetainedCommandJobFactory\u0000CadRetainedCommandJobFactory\u0000setContributions\u0000cad.scene.tool-command.v1",
    "16 process-global payload store candidate(s) require operation-owned state or an explicit static exemption",
    "6 app-owned retained route(s) lack an exact nonempty publication-lane contract",
    "4 app-owned retained route(s) declare Store publication lanes without their exact app-owned preparation authority",
    "35 app-owned import-media route(s) remain fail-closed pending explicit resumable factories",
    "270 live command registration(s) remain fail-closed; see remainingCommands ledger"
  ],
  "firstRemaining": {
    "file": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
    "id": "addWidget",
    "source": "macro",
    "reason": "classification proof exists but no exact registered app-owned retained reducer factory and builder"
  }
}
```

Exit code: 1. Counts are overlapping source projections and do not establish that classified commands execute or meet the strict callback ceiling. The first remaining registration is Procedural3d `addWidget`.

## Authority and Remaining Work

The current source check recognizes 33 exact-factory-proof owners, 255 custom rows and 25 generic rows. Remaining failures still include immutable child-content and exact old-root retirement authority, retained Jack/Trinity edit admission and acknowledgement, exact child-snapshot owner inventory, instance close ownership, a CAD declaration join, operation-owned payload stores, explicit publication/preparation authority and resumable media imports.

The full all-app native/Wasm/browser, actual command, cancellation, replay, accessibility, platform and strict timing gates remain open. See `checkpoint-domain-fixture-join.md` for the executor's domain-fixture RED-to-GREEN work and `📓️coordinator-active-evidence-loss-2026-08-27.md` for the unresolved evidence disappearance.

