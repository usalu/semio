# TC3d — the guest `codec.genesis` trap, and the first `s.note.note` document on a hub

Slice TC3d of ticket 26/09/18. Brief: decode and ROOT-FIX the `codec.genesis` trap TC3c measured at
16:14:12 on 2026-09-21 (`📓️tc3c-n-plugin-bootstrap-and-note-creation.md` §5f), then boot 7651 from
this tree and create the first `s.note.note` document ever created on a hub.

Started 2026-09-21 16:32. Predecessor state on arrival: no hub on 7651,
`.🧬semio/🌐hub/tc3c-boot/trusted-catalog/` EMPTY (the failed run framed no generation), and TC3c's
requeued driver still 7th in the ten-deep wasm mutex queue — it would have re-run the identical
failing chain, so it was killed by pid (55189 and its mutex waiter 55201) at 16:32 and its queue
entry `20260921161751-55201-tc3c` removed. One hold was requeued at 16:50 with the fix landed.

## 0. HUB HANDOFF

**No hub source changed by this slice.** Everything landed is in the plugin SDK / guest bridge, the
plugin host and the owned interpreter's WASI shim. `cargo check -p semio-hub` was therefore NOT run
and the coordinator needs no `semio-hub` rebuild on TC3d's account. (TC3c's own hub-fence changes
still need the coordinator's rerun — that handover stands in TC3c §0.)

| check | exit | warnings | capture |
|---|---|---|---|
| `cargo check -p semio-framework-plugin --lib` | 0 | 65 | `🗑️generated/tc3d-check-plugin.txt` |
| `cargo check -p semio-framework-plugin-host --all-targets` | 0 | 70 | `🗑️generated/tc3d-check-plugin-host.txt` |
| `cargo test -p semio-framework-plugin-host --lib owned_codec` (BEFORE the fix) | 101 | — | `🗑️generated/tc3d-repro-owned-codec.txt` |
| `cargo test … owned_codec_genesis_answers_on` (after the interpreter fix, still the pre-fix component) | 101 | — | `🗑️generated/tc3d-repro-decoded.txt` |

Warning counts are the proof the tree really type-checked rather than replaying a short-circuited
cache (memory *Require Warnings As Proof Of Type-Check*).

## 1. The trap, decoded — with the guest frame

TC3c had the fault text and nothing else: `codec.genesis(s.note.note): guest trapped: wasm trap:
unreachable executed`. **The reason it carried no frame is itself a defect, and it was the first
thing fixed.** The owned interpreter's WASI shim answered
`wasi:io/streams@0.2.0 [method]output-stream.write` by zeroing the result pointer and **discarding
the bytes** (`🖥️host/🦀️.rs`, the `reply_owned_host` match). A `panic = "abort"` wasm32 guest prints
its panic message to stderr immediately before executing `unreachable`, so the guest had already
said exactly what was wrong and the host threw it away. `GuestInstance::guest_diagnostics_text()`
answered `None` for every owned instance, where the wasmtime path has had a stderr pipe since
`guest_wasi_ctx`.

With the shim retaining that stream and the trap carrying its tail, the very same component answers
(`🗑️generated/tc3d-repro-decoded.txt`, 16:44):

```
Trapped("wasm trap: unreachable executed — the guest printed:
  thread '<unnamed>' (1) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18623:9:
  artifact store reached Drop without its exact terminal-empty shallow-shell witness")
```

`🏪️store/🦀️.rs:18623` is `impl Drop for ArtifactStore`'s assert. **The fault is not in genesis at
all.** Run against the note component TC3c itself built (`⚡️cache/cargo/target-tc3c/wasm32-wasip2/
wasm-release/semio_s_plugin_note.wasm`, 14:29), all four probes trap identically:

| probe | before the fix |
|---|---|
| `codec.genesis` by document schema (`note.document`) | `wasm trap: unreachable executed` |
| `codec.genesis` by artifact kind (`s.note.note`) | `wasm trap: unreachable executed` |
| `codec.pack-schema-hash` by document schema | `wasm trap: unreachable executed` |
| `codec.print-mirror` / `codec.apply-ops` (reached through genesis) | `wasm trap: unreachable executed` |

That third row **answers the discriminator TC3c §5f asked for**: `pack-schema-hash` traps too, so
the fault is in the shared RESOLVER, not genesis-specific, and it is not the dialect-kind fallback
either (the schema key traps identically).

**The mechanism.** `plugin_artifact_codec_app` (`🔌️plugin/🦀️.rs`) constructs EVERY app of the
installed bundle — it has to, because `A::DOCUMENT_SCHEMA` is only readable off a constructed app —
keeps the one that matches and lets the rest fall out of scope; each of the four `codec` entry
points then dropped the one it kept. A `VcsArtifactApp` owns an `ArtifactStore`, and that store's
`Drop` asserts an exact terminal-empty shallow-shell witness which only the bounded close cursor
satisfies. So **every plugin whose bundle declares more than one app trapped on every one of the
four `codec` calls** — and every artifact declares an editor and a viewer, note included
(`s.note.note@1/*#editor` and `…#viewer`, both on dialect kind `s.note.note`). This is not a note
defect: stdio and gis were simply never asked, because the hub links Rust codecs for them.

It is the identical defect TC3c fixed one level up in `store::ArtifactCodec::apply_ops_binary`
(TC3c §2) — a store built, used and dropped in one expression — found here one layer out, in the
app that owns the store rather than in the store.

## 2. The fix — three edits, all root-level

**(a) `🔌️plugin/🦀️.rs` — every throwaway codec app leaves through the close cursor.**
New `close_artifact_codec_app<PA: PluginApp>(app)` runs `close_step(1_024,
ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)` until `Complete` and then checks `close_terminal_is_empty()`
— the same contract the runtime's own instance-close job enforces
(`RUNTIME_CLOSE_ITEMS_PER_STEP` / `close_terminal_is_empty`), reduced to a synchronous drain because
a codec app was never opened, never bound an instance id and never attached a backbone.
`plugin_artifact_codec_app` now closes every candidate it constructs and does not return, closes the
role it rejects (`editor.or(viewer)` used to DROP the viewer), and closes the selected app before
returning any error; `plugin_artifact_{pack_schema_hash,genesis,print_mirror,apply_ops}` each close
the app after reading their answer. An app that cannot be closed is `std::mem::forget`ed and a fault
returned (`retain_unclosed_artifact_codec_app`) — dropping it would abort the guest, which is the
very `unreachable` this cursor exists to prevent, and the codec instance is thrown away per call so
the leak dies with the guest's linear memory.

**(b) `🔌️plugin/🦀️.rs` — `artifact_app_apply_ops` closes its store.** The guest-side twin of TC3c §2:
it built an `ArtifactStore`, printed from it and let it fall out of scope, so the first NONEMPTY
`codec.apply-ops` batch would have aborted the guest exactly the same way. It now drains through the
identical `close_owned_step` → `close_owned_terminal_is_empty` cursor.

**(c) `🖥️host/🦀️.rs` — the owned interpreter keeps the guest's own words.**
`retain_owned_guest_diagnostics` retains `output-stream.write` into a bounded
(`GUEST_DIAGNOSTICS_CAPACITY_BYTES`) buffer on `OwnedInstanceState`, `guest_diagnostics_text()`
answers for owned instances, and `owned_fault_text` appends the retained tail to every
`CoreStepOutcome::Fault`. Without this, no guest panic in the owned interpreter is diagnosable from
outside — which is precisely the two-hour wall TC3c hit.

**(d) `🖥️host/🦀️.rs` — a wasmtime `codec` path, as the A/B oracle.** `WasmtimeRuntime::codec_genesis`
and `::codec_pack_schema_hash` did not exist; only the owned interpreter could call the interface, so
there was no way to tell a guest fault from an interpreter fault. They now exist on the same
throwaway-instance shape, through `bindings.semio_framework_codec()`.

## 3. Laws

All in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs`,
A2's permanent oracle file, driven against the REAL staged component:

* `owned_codec_genesis_answers_on_a_real_plugin_component`
* `owned_codec_genesis_answers_by_artifact_kind_too` (both resolver keys, same pair)
* `owned_codec_pack_schema_hash_answers_on_a_real_plugin_component` (the discriminator)
* `owned_codec_print_mirror_round_trips_a_genesis_pair` (mirror + empty apply-ops)
* `wasmtime_codec_genesis_answers_the_same_pair_as_the_interpreter` (the A/B)

`plugin_wasm` was generalised from the single shared `target/` to the NEWEST build across every
`⚡️cache/cargo/target*` root: preamble rule 25 gives each slice a private `CARGO_TARGET_DIR`, so the
component a slice just built is routinely not under `target/`, and the old lookup silently graded a
stale build. That is how these laws see the component TC3c built rather than the one from 09-20.

Status: **RED against the pre-fix component, by design — that is the reproduction** (§1). They go
green only against a component rebuilt with fix (a)/(b) compiled in, which is stage 1 of §4.

## 4. The rebuild + bootstrap hold

ONE detached hold, driver `📜️tc3d-hub-boot.sh 7651 stdio,gis,note`
(pid in `🗑️generated/tc3d-driver-pid.txt`, wrapper `📜️tc3d-mutex-work.sh`, capture
`🗑️generated/tc3d-hub-dev.txt`). Launched 16:50:28, un-throttled with `taskpolicy -B` (memory
*Background QoS Throttles Agent Builds*). Private `CARGO_TARGET_DIR=…/target-tc3d`, data root
`.🧬semio/🌐hub/tc3d-boot`.

**The pre-flight earned its keep on the first attempt.** It ran RED at 16:50:32 —
`error[E0063]: missing field `quote` in initializer of `XmlDeclaration`` in
`semio-s-artifact-stdio-{zip,svg}`, a peer's `XmlDeclaration` refactor landing half-finished
(preamble rule 3: not mine to revert). TC3c's narrower preflight (`semio-framework-ui-contract`
alone) would NOT have caught it and the hold would have died inside the mutex, as it did at
13:40:30 on 2026-09-21. The peer landed the call sites within 90 s and the preflight went green at
16:52:07 after 1 retry, so the queue slot cost nothing. TC3d's preflight is the two commands the
hold actually needs — `cargo check -p semio-s-plugin-note --target wasm32-wasip2` (which compiles
the whole stdio→note closure AND the guest halves of `codec`) and
`cargo check -p semio-framework-plugin-describe` (the native emitter stage 2 runs).

**Queue, measured, and the promotion.** First queued 16:52:07 as `20260921165207-94161-tc3d`,
TENTH of ten (holder `pz1` since 16:14:19; by 17:04 `rb1` since 16:56:02 with seven slices still
ahead). Never a deadlock — rule 27(b)'s test showed real rustc work on the machine throughout; it
was simply a long honest queue.

At ~17:15 the **coordinator moved this bootstrap onto the critical path by agreement with the peer
session**. The queue is ordered by ticket NAME (`ls "$queue" | sort | head -1`), so a promotion is
expressed as an earlier stamp: `📜️tc3d-mutex.sh` is a copy of `📜️wasm-build-mutex.sh` whose ONE
changed line fixes the ticket at `20260921135300-$$-tc3d` — everything else, including the
stale-ticket reaper and both EXIT/INT/TERM traps, is byte-identical (verified by `diff` with the
ticket line masked). TC3d's own queued wrapper (pid 94161) and driver (92210) were killed BY PID
first — no peer ticket was touched, and one hold only. The driver relaunched at 17:16:13 (pid in
`🗑️generated/tc3d-driver-pid.txt`), preflighted GREEN at 17:19:11 on the first try, and took queue
position **3 of 8** at 17:19: behind the running `rb1` (…135123) and the peer's `…135200-play`,
ahead of `c7`, `stdio-a`, `stdio-examples`, `jb1` and `s10`.

| stage | what | capture |
|---|---|---|
| preflight (outside the mutex) | note `wasm32-wasip2` + describe emitter | `🗑️generated/tc3d-preflight-{guest,host}.txt` |
| 1 | `wasm-release` cdylib `semio-s-plugin-{stdio,gis,note}` — the SDK change touches all three guests, so all three are batched into the one hold | `🗑️generated/tc3d-prebuild-*.txt` |
| 2 | `trusted-catalog-bootstrap --packages stdio,gis,note` | `🗑️generated/tc3d-hub-dev.txt` |
| hub | hold 7651 with `OS_HUB_CREDENTIAL_SIGN_IN=true` | pid in `🗑️generated/tc3d-hub-pid.txt` |

TC3c measured the same stages at 43 min (stage 1, cold) and 1 h 45 m (stage 2) — TC3d's stage 1 is
near-cold again because the plugin SDK changed, so the same order of magnitude applies.

**Restart line**, should the driver die:

```
cd /Users/ueli/Documents/semio
nohup zsh ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/📜️tc3d-hub-boot.sh" \
  7651 stdio,gis,note \
  > ".🧬semio/🦑️repo/…/🗑️generated/tc3d-boot-driver.txt" 2>&1 & disown
```

Once `:7651/readyz` answers 200 the whole proof is one command:

```
zsh ".🧬semio/🦑️repo/…/📜️tc3d-provision.sh" 7651
```

**and it is already armed to run by itself.** `📜️tc3d-prove.sh` is a SECOND detached watcher (pid in
`🗑️generated/tc3d-prove-pid.txt`, launched 17:30) polling `:7651/readyz` for up to 10 h; on 200 it
(1) re-runs the codec laws against the rebuilt component
(`🗑️generated/tc3d-codec-laws.txt` — the A/B that closes §1), (2) runs `📜️tc3d-provision.sh`, `📜️tc3d-prove.sh`, and
(3) re-reads `readyz` so the capture ends with the hub still up. All of it lands in
`🗑️generated/tc3d-prove.txt`. It is a separate process because the boot driver's last act is `wait`
on the hub hold and it can never run a probe itself — so a worker session that ends before the lock
is granted still leaves a measured hub rather than an unmeasured one.

## 5. The 7651 proof table

**NOT REACHED while this report was written, and the reason is exact: the wasm mutex queue.** After
the coordinator's promotion the hold sits third (§4), behind `rb1` — running since 16:56 and
expected to hold until ~19:00–20:00 — and the peer's `play` activation. There is no hub on 7651 and
therefore no http code to report; nothing is running beyond the two detached processes this slice
owns, which finish the outcome without a session: the boot driver (pid 29431, bootstraps then holds
7651) and the prove watcher (pid 44737, laws + provision + create-and-attach on the first 200 from
`readyz`).

**Live state at 17:49**, the handover point: `rb1` released at 17:30, the peer's `play` holds since
17:30:37, and **tc3d is NEXT in the queue** — position 1 of 6 behind the holder. Read
`🗑️generated/tc3d-hub-dev.txt` for the stages, `🗑️generated/tc3d-prove.txt` for the proof, and
`cat /tmp/semio-wasm-build.lock/owner` for the current holder.

What IS observed at runtime here is the fault and its decode (§1), against the exact component TC3c
published-and-failed on. The fix is proven to TYPE-CHECK for both the guest (`wasm32-wasip2`) and
the native host; it is proven RED-then-fixed only at the source level until stage 1 rebuilds the
components. Per preamble rule 21 this slice does NOT clear the observed-at-runtime bar for document
creation, and says so.

## 6. Gaps

**(a) The laws are red until a rebuilt component exists.** `owned_codec_*` and
`wasmtime_codec_genesis_*` assert against whatever `semio_s_plugin_note.wasm` is newest in the cargo
cache. Until stage 1 of §4 lands, that is the pre-fix component TC3c built at 14:29, so the laws
reproduce the trap rather than pass. Re-run after the hold:
`CARGO_TARGET_DIR=…/target-tc3d cargo test -p semio-framework-plugin-host --lib codec -- --test-threads=1`.

**(b) Only note was asked.** stdio and gis have a linked Rust codec, so the hub never routes their
creation through the guest `codec` interface and the trap was invisible there. The fix is in the
shared resolver, so it covers them, but nothing in this slice DRIVES `codec.genesis` against the
stdio or gis component. A law that loops the four codec calls over every staged component is the
natural closure, and it needs all three components rebuilt first.

**(c) `codec.apply-ops` with a NONEMPTY batch is fixed but unexercised.** §2(b) is the same
store-drop class and is type-checked only; the law in §3 drives the EMPTY batch, which takes the
early `drop(envelope.into_owners())` return and never builds the store. A nonempty batch needs a
real encoded mutation for the note kind, which belongs with (b)'s per-component sweep.

**(d) The owned interpreter now retains guest stdout too, not only stderr.** The shim cannot tell
the two streams apart — `get-stdout` and `get-stderr` both mint an opaque resource id and nothing
downstream distinguishes them. Retaining both is deliberate (a guest that prints to stdout before
aborting is just as diagnostic) but it means `guest_diagnostics_text()` for an owned instance is
"everything the guest printed", where the wasmtime one is stderr only.

**(e) No unit law over `close_artifact_codec_app` itself.** Its contract (`close_step` until
`Complete`, then `close_terminal_is_empty`) is the runtime's own and is already exercised by
`close_registered_fixture_app`; the failure branch (`retain_unclosed_artifact_codec_app`) is
deliberately untestable from outside, because the alternative to retaining is the process abort.

## 7. Files changed

Source:
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `close_artifact_codec_app`,
  `retain_unclosed_artifact_codec_app`, `ARTIFACT_CODEC_APP_CLOSE_{MAXIMUM_STEPS,ITEMS_PER_STEP}`,
  the rewritten `plugin_artifact_codec_app` resolution and its four entry points, and the close
  cursor in `artifact_app_apply_ops`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` —
  `retain_owned_guest_diagnostics`, `OwnedInstanceState::diagnostics`, `owned_fault_text` +
  `OWNED_FAULT_DIAGNOSTICS_TAIL_CHARS`, `guest_diagnostics_text` for owned instances, and
  `WasmtimeRuntime::{codec_instance, codec_pack_schema_hash, codec_genesis}`

Laws:
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs` —
  the five codec laws of §3 and the newest-build `plugin_wasm` lookup

Ticket-owned: this report, `📜️tc3d-hub-boot.sh`, `📜️tc3d-mutex-work.sh`, `📜️tc3d-mutex.sh`,
`📜️tc3d-provision.sh`,
and `🗑️generated/tc3d-*`.
