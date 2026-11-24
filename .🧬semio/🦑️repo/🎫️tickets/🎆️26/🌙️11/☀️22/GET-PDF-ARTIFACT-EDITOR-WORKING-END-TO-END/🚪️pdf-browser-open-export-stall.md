# PDF browser Open File and Export Document stall (2026-11-24)

Native runs went through `native-cargo.sh` (one cargo, always filtered, crate `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib`, runner `shard-fuel-run.sh`). Browser checks ran against the already running host on 6260 with the existing wasm (no rebuild), through a throw-away copy of `pdf-pane-journey.ts` (deleted again).

## Result in one table

| Symptom | Verdict | Evidence |
| --- | --- | --- |
| Export Document produces no download within 90 s | **Journey defect, not a product stall.** `pdf-pane-journey.ts` pressed the actions `exportDocument` / `importDocument`; the framework ids are `exportArtifactDocument` / `importArtifactDocument` (`EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`, `IMPORT_ARTIFACT_DOCUMENT_ACTION_ID`). The press answered `absent` (no such button), so no download could ever arrive and no lane was held. | With the ids fixed, `transfer` passes on the current wasm: download `pdf-…semio-archive` (2,577 bytes), Import Document shows the inserted text again. Export is a plain guest read (`arm_read_document_archive`, `document_archive()` in one turn), it never touches work quotes. |
| Open File never publishes `file-opened` | **Real, still open.** Not caused by the probe quote (see below). Stuck inside the guest's `MediaIn` command custody on the fresh instance, with near-empty turns. | Browser, new stall label: `stdio-pdf interaction has held stdio-pdf#2's lane for 5000 ms (operation: media-in, stage: continuation 21180/16 (guest status more-work, ingress command-pending, 0 acknowledgements pending))`, i.e. about 4,000 guest turns per second that each answer `more-work` and do nothing. |
| `[DEBUG] big work quote 49999995 available=Some(49999995)` | **Unrelated to open and export.** It is printed once per instance birth: it is the first line of every console, before any command, also in sessions that never open or export anything. | `console-en-US.log` of the new-document journey (line 5, before `HistorySnapshot`), and the second print at the start of the open is the birth of the fresh app `#2` that `openNaturalFile` creates (`createApp`) before it sends `MediaIn`. |

## The `probe_effect` hypothesis is refuted

1. Native, exact browser contract: fuel 50,000,000 per turn (`DEFAULT_SHARD_BUDGET.fuel`, `🎭️actor/🧵️shard-runtime`), one armed host lifetime (`arm_host_lifetime`, 58,720,256 fuel, 30 s), the interactive grants of `mounted_policy()`. `two-pages.pdf` opens in all four combinations (paged natural source or media route, warm or cold head projection) in 2.6 s for the four, at most 512 turns each, and no turn spends more than 2^20 work units.
2. With the cap on `probe_effect` removed (the exact starting point of the previous change: `available_work - 1`), the same open still completes natively and the probe is **never called**: a temporary `[DEBUG]` print in `probe_effect` (env gated, since removed) produced zero lines during the open. The open path of the paged natural source and the media route does not reach the replay operation module.
3. A quote is read-only: `OriginalJobWorkRecipient::admits` and `ActualWorkTurn::admit_work` only test `observed + additional <= fuel`; nothing is reserved or charged until `performed(..)` is recorded. A probe that opens the whole turn spends only what its closure performs, so it cannot starve the turn by quoting.

## Fix: bounded probe ceiling (hardening, not the Open cure)

`probe_effect` still sized its first ceiling from the turn: `hint - 1`, i.e. 49,999,994 units in the browser. Producers that size demands or loops from `effect.maximum_work_units()` (`demands(.., maximum_work_units)` in the mutation, field and receiving cursors) therefore planned against a whole shard turn, which is exactly the "turn must not reserve its whole fuel for a probe" case even if it is not the Open stall.

- `🏪️store/🔁️replay/🎮️operation/🧬️native/🦀️.rs`: `PROBE_CEILING_CAP = 1 << 20` and `probe_ceiling(available)` = `min(available - 1, CAP)`, at least `PROBE_CEILING_FLOOR`. `probe_effect` uses it. With a 1 MiB turn (1,044,891) the start is unchanged (1,044,890), with 50 M it is 2^20.
- Unit law `the_probe_ceiling_never_reserves_a_whole_shard_turn` in `…/🧬️native/🧪️tests/🦀️.rs` (not run: the store crate is not a member of any workspace I can address with a filtered cargo here; the function itself is compiled in the PDF test build).

## Native reproduction under the browser contract (kept)

- Law helpers (`🔌️plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs`): `arm_host_turn_fuel(fuel)` returns a `HostTurnFuelGuard`; every `fixture_renew_turn` then uses the host's fuel instead of the fixture's 2,000,000 and meters the original work each renewed turn spent (`peak_turn_work()`, `turns()`, `settle(cx)`). All fixture helpers (settle, poll, submit) go through `fixture_renew_turn`, so the whole operation runs under the shard fuel.
- PDF tests (`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`): `open_registered_pdf_with(.., warm)` (the browser opens into a freshly created app whose head was never rendered, the native fixtures always pre-refreshed it) and four tests `shard_fuel_open_through_{the_paged_natural_source,the_media_route}_with_a_{warm,cold}_head`, asserting completion within `SHARD_TURN_LIMIT = 512` turns and `SHARD_TURN_WORK_BOUND = 2^20` work units per turn. All pass (5 passed with the existing `natural_source_route_opens_a_pdf_under_the_browser_turn_fuel`).
- Export Document is not a native work-quote path (a plain guest archive read), so it has no native fuel test; it is covered by the journey `transfer` step (now passing in the browser).

## Observability (kept)

The shell warning names what a held lane waits for:
`[os-shell] command stalled: <program> <command|interaction> has held <actor>'s lane for <ms> ms (operation: <actionId|media-in|config-command|…>, stage: <where the turn is>)`.

- `⏱️command-stall/🟦️.ts`: `CommandStallLabelV1` gains `operation?` and `stage?: () => string` (read when the warning fires, so it is the live stage); the band text is unchanged.
- `📥️invocation-frames/🟦️.ts`: `inspectEncodedAppCommand` returns `operation` (`media-in`, `config-command`, the action id, `undecodable`).
- `🚚️handle-turn-drive/🟦️.ts` (`runQueuedTurn`): tracks `stage` (`queued behind the actor's ingress lane`, `offering ingress page i/n`, `continuation k/ceiling (guest status …, ingress …, n acknowledgements pending)`, `settling the acknowledged turns`).
- Test `⏱️command-stall/🧪️tests/🟦️.ts` (bun, 2 pass).

## Journey fix

`pdf-pane-journey.ts`: `transfer` presses `exportArtifactDocument` / `importArtifactDocument`. The earlier `FAIL` of the `transfer` step in `🌐️pdf-browser-journey.md` (and the claim "Export Document / Import Document FAIL") was this defect; the section "Diagnosed on the way", item 3 (quote of the whole fuel in the decoder or encoder) should be read as superseded by this note.

## Where the Open stall is (next step)

The stalled state is the reactor's `MediaIn` custody of the fresh instance: `plugin-exchange` `AppCommand::MediaIn` → `prepare_media_descriptor` → `prepare_media_payload` (natural file: `DslValue::Bytes`) → `prepare_media_metadata` → `advance_original_media_import` → `advance_original_reserved_media`, answering `command-pending` + `more-work` with no effect and no work quote after the birth quote. It fails for a 2 KB file as for `two-pages.pdf`, so it is independent of size and fuel.

No native test drives `AppCommand::MediaIn` through a `PluginRuntime` (`AppCommand::MediaIn` appears only in protocol and host tests); the native PDF tests call `consume_media` or `advance_original_reserved_natural_source` directly and therefore skip that custody entirely. The reactor tests (`⚛️reactor/🧪️tests/📤️picker`) show how to mount a real app in a `PluginRuntime` (`RuntimeAppCell`, `insert_admitted`); a PDF `MediaIn` test needs the same mount plus the command ingress pages of `createShardCommandIngressPages` and the shard budget `{ fuel: 50_000_000, wallMs: 100 }`. Candidates to check first, in order: (1) a step of the custody that needs a grant larger than the turn's (`media_donation_funded`, `maximum_copy_bytes < MEDIA_DONATION_BYTES`, `OriginalManifestMetadata` limits from `mounted_policy.preparation`); (2) `cx.should_yield()` being true on every turn of the 100 ms wall budget in wasm (every `prepare_*` returns `Ok(false)` on it, and 4,000 turns per second fits a turn that returns at once); (3) `with_instances_original_mut(...).resolve_ready(advance_original_media_import)` dropping a future that awaits `prepare_reserved_media_cache`/`refresh_cache` (`plugin_job_yield_once`) every turn, so it restarts and never completes.

## Re-verify in the browser after the next wasm rebuild

1. `bun pdf-pane-journey.ts transfer` passes (already true on the current wasm).
2. `open-en` / `open-de`: the shell warning, if the stall remains, now carries `(operation: media-in, stage: continuation …)`; use the count and the ingress/guest status in it to see whether the turns are still near-empty.
3. The console line `[DEBUG] big work quote …` is the instance-birth probe: after the rebuild it should read at most 1,048,575 (the cap), once per instance.
4. `en`/`de` new-document edit and Save stay green (probe ceiling changed).

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🦀️.rs` (probe cap) and `🧪️tests/🦀️.rs` (law).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` and `🧪️artifact-app-laws/🦀️.rs` (`arm_host_turn_fuel`, `HostTurnFuelGuard`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`open_registered_pdf_with`, shard tests).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/` `⏱️command-stall/🟦️.ts`, `⏱️command-stall/🧪️tests/🟦️.ts`, `📥️invocation-frames/🟦️.ts`, `🚚️handle-turn-drive/🟦️.ts`.
- Ticket: `pdf-pane-journey.ts` (action ids), `shard-fuel-run.sh`, this note.
