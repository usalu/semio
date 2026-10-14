# Current GUI Recorder Readiness Audit

Read-only audit on 2026-10-14. Only file text and inventories were read. No recorder was imported or run; no syntax check, test, factory, frontend, initializer, native operation, signal, Git operation or cleanup was performed. This report is the only authored file. Root must retain the dispatch hold until the concurrent Source cohort freezes.

## Findings

1. **P2 — Actor recorder can overwrite prior capture.** `ap13/📜️script.ts:14–18` creates its output recursively and opens `current.log` with `w`; lines 15/17/21/23 overwrite receipts. Unlike Product line 11 and Native lines 20/75/90, it has no fresh-output/caller existence refusal. A second accidental dispatch destroys original capture evidence. Require exclusive output/caller ownership before any write.

2. **P2 — Actor/Product inherited authority refusal is incomplete.** Actor line 7 refuses only invocation/capabilities/private-custody; Product line 7 additionally refuses graph path/digest. Both spread the entire inherited environment at Actor line 16/Product line 13. Issued authority such as `SEMIO_NX_SELECTED_GRAPH_RECEIPT`, `SEMIO_NX_GRAPH_SEAL_REUSE`, `SEMIO_BUILD_REUSE_GRAPH`, `SEMIO_OWNER_POLICY_RECEIPT`, `SEMIO_PROCESS_OWNER_CONTEXT`, source-closure/cargo-lock/task custody can survive into the configured factory and child. Native lines 13–19 already refuse this wider family. Extend refusal consistently; never delete or freshen inherited authority. A clean Root process is operational mitigation but not a recorder guarantee.

3. **Output-bound qualification required.** Actor line 21, Product line 16 and Native line 93 route stdout/stderr directly to regular file descriptors. These recorders do not enforce any aggregate byte/line bound on `current.log`. Native line 53 preserves the original native transport grant (134217728 bytes/1000000 lines), which bounds operations that actually consume it, not arbitrary frontend/recorder combined output. `process/🎛️owned-execution/🟦️.ts:101–113` enforces limits only where `options.output` exists. Do not describe the entire log file as proven bounded by original native caps. Native lines 105–106 then read/split the entire log in memory. If the dispatch criterion specifically requires a bounded total stdout file, these scripts do not meet it; do not silently tighten or change the GUI grant.

## Verified Source Facts

- Actual seed and mirror rows are structurally equal for all three selected commands. Mirror row starts: Actor 93712, Product 93668, Native 93890; seed: Actor 72318, Product 72274, Native 72496. Actor/Product initial policies remain 600000 ms; Native policy remains 3600000 ms, prototype, jobs2 and offline true. All literal commands have simple space-delimited arguments, so current split argv preserves the actual registered argv.
- Build configured factory `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts:59–63` parses/resolves selection and returns either the configured native grant or admission of full authored capabilities. Caller implementation lines 17–33 validates and normalizes the complete grant. Actor line 16/Product line 13 obtain complete return before spawning; child frontdoor creates its first envelope subsequently. Native lines 83–88 record complete factory return before explicitly creating the first envelope. No raw capabilities reconstruction is present.
- GUI environment merge preserves every current row value with workspace expansion and changes only the test artifact destination; Native subsequently adds issued envelope authority through the actual process helper. Actor/Product have no authored capabilities in their GUI rows. Native preserves the full authored declaration.
- Actor defining script `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts:42–44` requests 30000/60000/30000 ms under the original invocation. Product defining script `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:42–43` requests 30000/30000 ms. None was run.
- All three await `child.exited` and close recorder descriptors in `finally`. Native additionally records raw signal and qualifies failures/observations. Actor/Product have no terminal receipt if spawn or started-receipt writing throws; descriptors still close.
- Native only compares prior launch row and uses its 39-path observation roster; prior hashes are not admission authority. Native line 106 observes actual dependency messages and never reads or requires old expected6b generation. Genuine current50954 is not excluded by this code.
- Actor observations omit an explicit non-authority kind but are named observations, not used as an identity gate. Product has no source observation claim. Native explicitly labels before/after hashes and dependency logs non-authoritative.

## Snapshot Hashes

- `ap13/📜️script.ts`: `9da6d6ac0ff884382d1da36c4de138f7211d9df8936e992808cc389434e53226`
- `product-actor-integration-current2-inputs/📜️script.ts`: `be515218e15f7f43a12d068344a4ce2e085ce08d01d25672c2f2e502da90a028`
- `media-native11/📜️script.ts`: `6529f5d2a796a5a8890f6c6d5f2f770df00140784ad4c3bb1cd664c33f63cf06`
- `.vscode/launch.json`: `45630cbac1fd51533cc75f011982850f43fd33579ed870d7df5efc07ae158111`
- `.vscode/🧩️launch.seed.jsonc`: `e9ab5eb727a7a4adf6dfa094160c1af5aa4c02215b230533ce298a3dbc9a637b`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`: `72d89666c666f002b498123d7a978cd231d6f3d341b3aa89fff776f9f078eed4`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts`: `6059b852535e26bc0eda386c4f6af8a8be28bb92c5a4808d10c27bdbd5bc97ec`

Prepared notes only establish prior syntax parsing, not semantic checking or registered/native success. Current read-only observations do not freeze concurrently changing Root member metadata. Re-read relevant hashes after freeze before explicit dispatch.
