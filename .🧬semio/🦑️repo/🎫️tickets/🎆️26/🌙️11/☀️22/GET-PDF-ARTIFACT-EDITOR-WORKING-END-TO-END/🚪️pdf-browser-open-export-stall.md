# PDF browser Open File and Export Document stall (2026-11-24)

Native runs went through `native-cargo.sh` (one cargo, always filtered, crate `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib`, runner `shard-fuel-run.sh`). Browser checks ran against the already running host on 6260 with the existing wasm (no rebuild), through a throw-away copy of `pdf-pane-journey.ts` (deleted again).

## Result in one table

| Symptom | Verdict | Evidence |
| --- | --- | --- |
| Export Document produces no download within 90 s | **Journey defect, not a product stall.** `pdf-pane-journey.ts` pressed the actions `exportDocument` / `importDocument`; the framework ids are `exportArtifactDocument` / `importArtifactDocument` (`EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`, `IMPORT_ARTIFACT_DOCUMENT_ACTION_ID`). The press answered `absent` (no such button), so no download could ever arrive and no lane was held. | With the ids fixed, `transfer` passes on the current wasm: download `pdf-…semio-archive` (2,577 bytes), Import Document shows the inserted text again. Export is a plain guest read (`arm_read_document_archive`, `document_archive()` in one turn), it never touches work quotes. |
| Open File never publishes `file-opened` | **Real, still open.** Not caused by the probe quote (see below). Slow progress inside the guest's `MediaIn` command custody on the fresh instance: thousands of one-action turns in the descriptor receive (see Round 2). | Browser, new stall label: `stdio-pdf interaction has held stdio-pdf#2's lane for 5000 ms (operation: media-in, stage: continuation 21180/16 (guest status more-work, ingress command-pending, 0 acknowledgements pending))`, i.e. about 4,000 guest turns per second that each answer `more-work` and do nothing. |
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

## Round 2: the Open stall reproduced natively through the real `PluginRuntime`

### The test that reproduces it
`media_in_of_a_small_pdf_reaches_its_mount_through_the_real_runtime_under_{the_shard_budget,the_interactive_grant}` and `a_quiet_retained_command_is_faulted_as_stalled_by_the_reactor` (PDF editor unit tests, `#[ignore]`d until the receiver is redesigned, run them with `-- --ignored`). They do what `openNaturalFile` does:

1. `PluginBuilder` bundle of the PDF editor installed in a `PluginRuntime<VcsArtifactApp<EditorApp<Pdf17Editor>>>` (plugin id `stdio`, the surface owner), `Event::InstanceOpen` -> `Captured` -> acknowledgement: a fresh instance born through the real lifecycle;
2. `AppCommand::MediaIn { port: natural-file, descriptor: <Pack of the same descriptor the shell sends>, data: two-pages.pdf }` cut into command pages by `CommandBatchDriver`, one page per turn, then continuation turns without input, each turn through `poll_kernel` with the shard's 50,000,000 fuel (`arm_host_turn_fuel`), an armed host lifetime and (second test) the interactive per-turn grant;
3. success is the reactor spawning the reserved import job (`Effect::SpawnJob`, what the shell logs as `spawn-job`) within 64 turns.

New seams: `semio_plugin::reactor_turn_laws` (`poll_kernel`, ingress census, idle-limit knob), `laws::fixture_step_context_funded`. The test needs a `FixtureUiArena`, an `OriginalFixtureScope` renewed per turn (`fixture_renew_turn`) and a retained turn input that names the scope's own operation and generation.

### What it shows (before any fix: all 512, then 1,350 turns, never `SpawnJob`)
Every turn answers `CommandPending / MoreWork` and spends 12 to 28 work units, 2 to 17 retained items, so it is **not** an empty spin: the `MediaIn` custody makes tiny progress per turn and never reaches the mount. Temporary stage traces (removed again) located it in `OriginalDecodedPluginCommand::prepare_media_descriptor`, the very first stage of `plugin_exchange`'s `MediaIn` arm:

| descriptor phase (`OriginalPackReceiving`) | host turns |
| --- | --- |
| 0 parse of the ~100 byte Pack (`RetainedIntrinsicBody`, one parse action per turn) | 323 |
| 1, 2 admit the parsed `DslValue` | 13 |
| 6 typed receive into `MediaArtifactDescriptor` (`NativeReceivingOriginal`) | more than 1,013, still going at the 30 s lifetime wall; each turn is one quote-exact native action (`phase6 ... stepped=complete work=1..2`, copies of 1 to 2.7 KB, a 28 KB receiver reservation) |

The later stages (payload donation, `prepare_media_metadata`, `advance_original_media_import` -> reserved header/cache/request/mount) were never reached in 1,350 turns. The shell's `COMMAND_CONTINUATION` pacing explains the browser symptom exactly: the shell sees no observable output, so after 15 s of quiet it backs off 25 ms, doubling to 250 ms per turn; a custody that needs thousands of turns therefore takes hours, and the lane warning counted 21,180 near-empty `more-work` continuations in the first 5 s.

### Why one action per turn, and why the three candidates were not it
- Candidate (1), a grant larger than the turn's: no. The typed receive quotes its exact next action (`original_receiving_scope_quote(demand)`) and the scope is settled at every turn boundary (`fixture_renew_turn` -> `settle_receiving_scope`, the same as the host dropping a turn's context): **one receiving scope per host turn is a framework invariant**, so chaining several typed-receive moves inside one turn is impossible by design. Wide fixture grant and interactive grant behave identically.
- Candidate (2), `should_yield` true on every turn: no (the fixture has no deadline, the stall is the same; the browser's 100 ms wall only shortens turns).
- Candidate (3), a dropped `resolve_ready` future: no; no stage after the descriptor was reached, and every stage keeps its state in the retained owner.
- The command never waits for an acknowledgement and the fresh instance's first turn is not missing a prerequisite (`prepare_reserved_media_cache` is not reached).
- Why the native `consume_media` tests never saw it: they hand the decoded `MediaArtifact` straight to the dispatcher and skip the whole reactor custody (descriptor receive, payload donation, metadata, import progress).

### Fixes applied
1. **Reactor progress law** (`⚛️reactor/🔄️turn/⌨️command-ingress-turn`): a retained generic command whose turn consumed no fuel, moved no retained stock and produced no frame, effect or event is *quiet*; after `COMMAND_IDLE_TURN_LIMIT` (16,384) consecutive quiet turns the reactor faults it with the named `plugin.command-stalled` (`COMMAND_STALLED_FAULT_CODE`) through `retain_decoded_exchange_failure`, so the host's retire path ends the command instead of the shell's 300 s wall. 16,384 is chosen so a host that merely waits on a job (turns paced at 25 ms or more) never reaches it inside the 300 s wall, while a spinning guest (4,000 turns/s) reaches it in about 4 s. The law did **not** fire in the browser because it did not exist before; the only bounds were the TS ceiling of 16 continuations (a floor, the real bound is the 300 s wall) and, in the guest, nothing. It also would not have fired on this particular stall, which is *slow progress*, not quiet: the descriptor custody spends work every turn.
2. **Localized fault** (`🔌️PluginRuntime/⏱️command-stall`): `commandStalledTextV1(program, operation, locale)` (en: "... made no progress, so the command was stopped. Nothing was changed.", de: "... keinen Fortschritt gemacht; der Befehl wurde abgebrochen. Es wurde nichts verändert."), `commandStalledMessageV1` (code plus both languages) and `isCommandStalledFaultV1`; `handle-turn-drive` throws it when the ingress answers the stalled fault. Bun tests: 3 pass.
3. **Chained custody moves** (`plugin-exchange`, `MEDIA_IN_MOVES_PER_TURN = 1,024`): the `MediaIn` arm repeats its moves inside one turn while each move still moves retained stock and the turn has not yielded. Moves that are not a receiving scope (the parse of phase 0, payload donation, metadata, reserved preparation) collapse into a few turns (the 323 parse turns became one turn in the trace); the typed receive (phase 6) is untouched by it.

### Not fixed (honest status)
The typed receive of the descriptor (phase 6) still needs more than 1,013 turns natively, so **Open File is still unresolved**: the three new tests stay red/ignored. The only root fix is to stop receiving a ~100 byte descriptor through the original typed receiver: decode it (and the tiny manifest metadata, which uses the same machinery) in one bounded, quoted step, charging its work and retained bytes once (`copied_bytes` = descriptor length, `capacity` = the strings), then donate it to `media_descriptor`, whose retirement already uses the generic typed-owner retirement. That changes the framework's receiving ownership model (`👷️job/🛬️receiving/📥️pack`), so it needs the owner of that crate; I did not attempt it blind. Alternatively the shell could send the descriptor as part of the already-typed command header (the descriptor is a pure function of the picked file's format), which removes the stage entirely.

## Re-verify in the browser after the next wasm rebuild

1. `bun pdf-pane-journey.ts transfer` passes (already true on the current wasm).
2. `open-en` / `open-de`: the shell warning, if the stall remains, now carries `(operation: media-in, stage: continuation …)`; use the count and the ingress/guest status in it to see whether the turns are still near-empty.
3. The console line `[DEBUG] big work quote …` is the instance-birth probe: after the rebuild it should read at most 1,048,575 (the cap), once per instance.
4. `en`/`de` new-document edit and Save stay green (probe ceiling changed).
5. `open-en`: with the chained custody moves the descriptor parse should shrink; the lane warning stage still shows `continuation k/16 (guest status more-work, ingress command-pending ...)` until the typed descriptor receive is replaced; a command quiet for 16,384 turns now ends with `plugin.command-stalled` instead of waiting for the 300 s wall.

## Files changed (round 2 additions first)

- `🔌️plugin/🦀️.rs` (`reactor_turn_laws`), `⚛️reactor/🔄️turn/🦀️.rs` (re-export of the idle-limit knob and fault code), `⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs` (`COMMAND_IDLE_TURN_LIMIT`, `COMMAND_STALLED_FAULT_CODE`, `note_command_turn`, the quiet-turn law in `exchange_generic`), `⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs` (`MEDIA_IN_MOVES_PER_TURN`, chained `MediaIn` moves), `🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` (`fixture_step_context_funded`).
- `⏱️command-stall/🔣️.json`, `⏱️command-stall/🟦️.ts`, `⏱️command-stall/🧪️tests/🟦️.ts`, `🚚️handle-turn-drive/🟦️.ts` (named, localized stalled fault).
- PDF editor unit tests: `shard_turn`, `shard_open_instance`, `shard_close_instance`, `shard_media_in_open`, the three ignored `MediaIn` laws.
- Ticket: `shard-fuel-run.sh <log> <filters...>` (the gated runner).

## Files changed (round 1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🦀️.rs` (probe cap) and `🧪️tests/🦀️.rs` (law).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` and `🧪️artifact-app-laws/🦀️.rs` (`arm_host_turn_fuel`, `HostTurnFuelGuard`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`open_registered_pdf_with`, shard tests).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/` `⏱️command-stall/🟦️.ts`, `⏱️command-stall/🧪️tests/🟦️.ts`, `📥️invocation-frames/🟦️.ts`, `🚚️handle-turn-drive/🟦️.ts`.
- Ticket: `pdf-pane-journey.ts` (action ids), `shard-fuel-run.sh`, this note.
