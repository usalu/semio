# Playbook Retained View Factory

> Superseded on 2026-08-27 by the exact publication-lane audit. The earlier source proof bounded the reducer but did not prove that its Config completion could be published through an app-owned retained Config preparation state machine.

## Scope

This packet originally attempted to admit `setLocale` and `setContributions`. Both emit `PlaybookConfigMutation`; their inverses snapshot the full prior `PlaybookConfig`, including unbounded contribution JSON. Playbook supplies no retained Config preparation factory, so both routes are now fail-closed together with the seven document mutation commands.

## Implementation

- The incomplete factory, proof rows, `Migrated` annotations, and retained-factory test were removed.
- The ordinary command declarations remain live and explicitly unclassified, so release verification fails closed.
- Re-admission requires a retained Config preparation owner that incrementally constructs the post snapshot, inverse snapshot, edit, cursor, and digest without a whole-config clone in any turn.
- Playbook's test helpers await the framework boundaries that remain genuinely asynchronous: app creation, typed dispatch, render, action handling, and artifact-store dispatch.

## Language-Neutral Law

`playbook-view-command-limits.json` and its JSON Schema cover exact-empty, exact-maximum, maximum-plus-one, and checkpoint cases. Ajv 2020-12 accepted the fixture:

- `🧪️coordinator-playbook-view-command-limits-ajv-2026-08-26.txt`

## Source Gate

The earlier second census accepted both concrete reducer identities but predated publication-lane verification and is therefore not admission evidence. The corrected verifier is documented in `📓️coordinator-exact-publication-lane-admission-2026-08-27.md`.

- all nine Playbook commands have no complete reducer-plus-publication proof;
- 29 process-global owner findings remain repository-wide;
- 53 scan-then-monolith reducers remain repository-wide;
- 35 import-media routes remain fail-closed;
- 680 live command registrations remain fail-closed.

Evidence: `📊️coordinator-official-tool-jobs-playbook-view-r2-2026-08-26.json`.

## Runtime Status

There is no Playbook runtime-pass claim. The routes remain fail-closed until both reducer and publication state machines are executable and verified.
