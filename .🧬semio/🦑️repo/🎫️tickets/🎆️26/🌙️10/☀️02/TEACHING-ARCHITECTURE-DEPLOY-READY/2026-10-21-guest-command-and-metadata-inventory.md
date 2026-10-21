# Guest Command and Metadata Inventory

Read-only inspection of actual entry.ts/core.ts/schema.json/tests.ts and interface report; no commands, functions, tests, transfer or cleanup executed.

## Existing Command Surfaces

Use the supported native repo binary through `repo exec --projects=workspace -- bun <absolute-ticket-file> ...`, release mode, from the workspace. The report currently specifies native binary `.🧬semio/🦑️repo/⚡️cache/🗃️bin/prototype/repo/repo`; this inventory does not validate or provision that binary. Exact script argument forms are:

- Unit: `🧪️guest/tests.ts <own-inputs-sha256>`; only one64hex argument. Current historical seal is039b8baae2fedd448bbf64191352c6353df6d6dbd1d5679e212ced817ba1d118 and must change after roster edit. Unit synthesizes Source rows from policy, checks own seven before/after and uses genuine owned Python oracle plus SQLite.
- Prepare: `🧪️guest/entry.ts prepare <operation32hex> <plan-sha256> <own-inputs-sha256>`.
- Run: `🧪️guest/entry.ts run <same-operation> <same-plan-sha256> <same-own-inputs-sha256>`.
- Cleanup: there is NO separate public cleanup entry mode. SIGINT/TERM requests cancellation of this owned run. Entry accepted physical cleanup authenticates exact container/engine, invokes fixed guest cancel and observe actions, and waits for actual bound join or completed failed receiver/no worker. It neither removes the builder nor stops peers. Do not invent an entry cleanup argument or directly invoke guest actions.

Prepare exclusively publishes generated/cg-<operation>/{manifest.json,package.json,prepare.json}. Run requires those exact prepared bytes and an exclusive claim; a repeated operation cannot silently overwrite them. Host terminal publication is separate from its own outer native terminal.

## Initial Metadata Gatherable Without Native Effects

Plan path is fixed generated/fresh-neutral-<operation>.json, at most32768B. Closed fields: version1, kind captured-source-neutral-plan, cohort/captureId/operation32hex, template pin and evidence pins for identity/source/manifest/git/provision. Each pin is exact raw bytes plus SHA256. Initial read-only work can inventory current authored83 file pins, copied combined schema roster, own seven seal, audited image/native policy constants, IDs and any already-existing genuine evidence. Missing successful evidence cannot be authored or simulated. Engine/container authentication itself is an actual owned external observation during run.

Evidence fixed filenames are fresh-create-receipt-<cohort>.json, fresh-source-receipt-<captureId>.json, fresh-capture-manifest-<captureId>.json, fresh-git-transfer-terminal-<captureId>.json and fresh-provision-receipt-<cohort>.json. Required existing proof: dedicated created identity/image/name/volume/runToken; successful Source receiver with actual client0/null, all hashes and matching captured manifest; accepted genuine Git proof; exact audited provision native path/bytes/SHA and help0. All cross-bind cohort/container/capture/identity and Source manifest. Raw manifest bound128MiB, other evidence1MiB.

## Roster and Raw Template Update

Within own seven, ONLY schema.json has the explicit operation Source roster: definitions.policy.const.sourcePaths currently59. Change it to exact reviewed67 in authenticated copied combined schema order. cases.json no longer has a duplicated Source roster; tests.ts derives current policy roster/count/marker, core.ts and guest.py enforce policy equality dynamically, entry.ts/oracle.py do not hardcode59. No code or cases edit is required solely by count expansion. Reseal inputs.json after schema change and refresh documentation hashes/count; do not retain historical own seal. Critical102 belongs to the separate production derive policy, not this interface Source roster.

Raw base template selection is FIXED host Ticket/📦️fresh/combined.inputs.json, read at most1MiB and matched to plan.template exact raw size/SHA. It cannot be selected by arbitrary path. Template must close exactly83 files and exact Source keys, and its pin for copied 📦️fresh/combined.schema.json authenticates that schema bytes. The reviewed new raw base template therefore must first carry current83 controls and current67 key roster. deriveManifest ignores its historical Source pin values: every output Source bytes/SHA is projected exclusively from actual authenticated captured Source manifest rows, requiring captureId/kind and exact permitted paths. No late mutable host Source read supplies projected pins.

Order remains actual capture/Git/provision, plan with actual evidence and reviewed base-template pin, own unit under new seal, prepare, run, actual terminal/receipt acceptance, then unchanged production controller21. Unit can run before genuine capture because it uses synthesized rows. No stage or receipt has been executed by this inventory.
