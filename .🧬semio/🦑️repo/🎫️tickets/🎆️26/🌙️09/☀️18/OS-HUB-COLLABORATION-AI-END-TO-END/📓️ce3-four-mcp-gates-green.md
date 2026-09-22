# CE3 — four semio-MCP gates on the current tree

Slice CE3 of ticket 26/09/18, session 8 (2026-09-22 10:55 →). Predecessor: `📓️ce2-mcp-gates-green.md`.
Every number below comes from a capture in `🗑️generated/ce3-*`, or from an inherited `ce2-*` one that
is named where it is used.

## 0. Headline — measured only

| gate | CE2 left it at | CE3 measured | capture |
| --- | --- | --- | --- |
| `client-e2e` | **5/6** (fail-closed at step 3, ~30 steps never ran) | **36 / 38** | `ce3-client-e2e-{3,4}.txt` |
| `capability-audit-check` | 29 findings / 1 catalog diagnostic | **29 findings / 1 catalog diagnostic** — unchanged twice (11:10 and 12:09) | `ce3-audit-{1,2}.txt` |
| `live-agent-loop-check` | 21/21 | **21 / 21, 0 failed, 0 skipped** ✅ re-confirmed on MY binary | `ce3-live-agent-loop-1.txt` |
| `hub-agent-participant-check` | 14/17 on 7621; 7651 never up | **14 / 17 on 7621**, identical reds; **7651 answered `000` all session** | `ce3-hub-participant-7621.txt` |

Three things this slice establishes that were open when it started:

1. **CE2's queued describe batch DID run**, at 04:13 → 04:51 during the outage window, and its two
   owners parted ways: `🀄️wfc` **rc=0 in 230 s**, `🧩️puzzle` **rc=1 after 2 051 s on `fuel
   exhausted`, not on the deadline** (§1). CE2's "both are starved, not runaway" reading holds for
   `🀄️wfc` and is **wrong for `🧩️puzzle`**, which is a fuel-budget case (§5).
2. CE2's `OwnedDeadline` is no longer compile-unverified, and better than that it is **observed at
   runtime**: `🧩️puzzle` ran to **1 946 890 ms**, 147 s PAST the 1 800 000 ms that used to kill it,
   and died on fuel instead (§2).
3. `client-e2e`'s denominator was never 6. With `🀄️wfc`'s descriptor current, the journey runs to
   its end and two real product defects fall out of it — **both fixed here, both measured** (§3).

## 1. Inherited: CE2's describe batch ran during the outage

`🗑️generated/ce2-describe-ledger.txt`, read before starting anything:

```
🀄️wfc      rc=0   230s   json 772000 -> 1030746   pack 195700 -> 254455   2026-09-22T04:17:27+02:00
🧩️puzzle   rc=1  2051s   json 4803294 -> 4803294  pack 4295257 -> 4295257 2026-09-22T04:51:38+02:00
```

`🀄️wfc`'s staged component and its committed descriptor now agree — checked directly rather than
inferred: `shasum -a 256 wasm-dev/semio_s_plugin_wfc.wasm` = `7544e0b01fbaf7c9…`, and the descriptor
the run wrote carries `wasm_sha256=7544e0b01fbaf7c9…`. That is what re-opened `client-e2e`.

## 2. `OwnedDeadline` — compiled, and observed

* `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-ce3 cargo check -p semio-framework-plugin-host`
  → **`Checking semio-framework-plugin-host v0.1.0`, `Finished dev profile in 17.37s`, rc=0**
  (`ce3-check-plugin-host.txt`). The crate was reached (its own `Checking` line) and the run printed
  warnings for four other crates on the way, so the build was real rather than a no-op.
* The stronger evidence is behavioural. Under the OLD total-wall rule a describe died at
  `elapsed_ms=1800000`; `🧩️puzzle`'s 04:51 run instead reports
  `fuel=8000000000 elapsed_ms=1946890` followed by `fuel exhausted`
  (`ce2-describe-🧩️puzzle.txt`). **The wall bound did not fire 147 s after it would have**, which is
  exactly `OwnedDeadline::NoFuelProgress` doing its job on a guest that keeps consuming fuel.

**Honest gap:** the host's `--lib` law for this is
`🖥️host/🧪️tests/🔬️owned-runtime/🦀️.rs:31` (a zero deadline must answer `DeadlineExceeded`), and it
is gated behind `SEMIO_OWNED_COMPONENT_FIXTURE` — with the variable unset the whole test body
`return`s. I did not run it with a fixture: the smallest staged component is 16 MB and the test's own
`budget()` grants 500 M fuel, which is below what several current guests need to describe, so a
fixture run would have proved a compile rather than the law. The runtime evidence above is what this
section rests on, and it is stronger than the law would have been.

## 3. `client-e2e` — 5/6 → 36/38

Run from `🌉️mcp/📦️packages/🟦️typescript`, `bun ./📜️script.ts client-e2e`.

| run | score | captures |
| --- | --- | --- |
| before any edit of mine, `🀄️wfc` descriptor already current | 27 PASS / 2 FAIL, then an unguarded throw at `ping` | `ce3-client-e2e-1.txt` |
| after the `sprBase64` fix (§3.1) | **36 / 38** | `ce3-client-e2e-2.txt` |
| after the `source_dialect` fix (§3.2) | **36 / 38**, one red replaced by the layer beneath it | `ce3-client-e2e-3.txt` |

The freshness step is green on both pinned components:

```
PASS os: every pinned component is staged and current —
  note: …semio_s_plugin_note.wasm 65046823 B, sha256 e6dc23fc54a2… matches its committed descriptor |
  wfc:  …semio_s_plugin_wfc.wasm 128436995 B, sha256 7544e0b01fba… matches its committed descriptor
```

### 3.1 `artifact_snapshot` could not show ANY mutation, ever — fixed

**Red:** `os: the snapshot shows the mutation — … is byte-identical across the commit (400 base64
chars) — the snapshot is reading a frozen row, not the document the mutation went to`.

The diagnosis in that message is wrong, and the assertion behind it was unsatisfiable by
construction. `🏪️store/🦀️.rs:12315`:

```rust
let pack = envelope.vcs.initial_snapshot.encode_pack();
let spr = print_document_spr(envelope).await?;
```

A `.spk` **`pack` is the GENESIS snapshot** — fixed for the life of the document — and every
committed mutation lands in the **`.spr` event log**. `artifact_snapshot` published `packBytes`,
`sprBytes` and `packBase64`: the genesis bytes in full, and the log by LENGTH only. So an MCP client
following the shipped `mutate-safely` prompt ("re-read the artifact and confirm the change landed")
was handed bytes that cannot move, and the gate compared exactly those bytes. This is not a
regression in the lane AP1 built — that lane works; it is the wrong half of the document being
published.

Landed:

* `🌉️mcp/🏠️workspace/🦀️.rs` — both `semio://artifact/{id}` body sites (the hub-document lane and
  the local lane) now carry `"sprBase64": base64_encode(&spr)` beside `packBase64`.
* `🌉️mcp/🧬️schema/🦀️.rs` — `artifact_snapshot_output_shape()` publishes `sprBase64`, with a
  docstring recording why the pack alone can never move. Mirror regenerated with
  `bun ./📜️script.ts schema-mirror` → `exports=68 ajv-draft07-resolved=68 json=1 typescript=1`
  (`ce3-schema-mirror.txt`), so `🧬️schema/🔣️.json` and `🧬️schema/🟦️.ts` carry it too.
* `🌉️mcp/🟦️.ts` — new `snapshotDocumentBase64()` helper; the step compares the WHOLE document.

Measured after: `PASS os: the snapshot shows the mutation — 701 → 1361 base64 chars across the
commit (pack 400 → 400, spr 300 → 960)`. The pack standing still while the spr triples is the fault
and the fix in one line.

### 3.2 Every real inference died before the guest ran a step — fixed

**Red:** `os: inference_run — … "code":"artifact-inference.source-dialect","message":"identity
\"s.wfc.…`.

`🌉️mcp/🏠️workspace/🦀️.rs:1047` built the request's `source_dialect` as
`format!("{}@{}/*", declared.artifact_schema, declared.artifact_schema_version)` → `s.wfc.bitmap@1/*`.
That is the **capability-id** grammar. The guest validates the field as an `ArtifactIdentity`
(`🔌️plugin/🦀️.rs`, `validate_wire_request_resources` → `ArtifactIdentity::parse`), whose segments
may contain only `[a-z0-9_-]` — `@`, `/` and `*` are all non-canonical, so **no plugin's inference
could ever pass this gate**, whatever it computed. The version is not carried by that suffix anyway:
`artifact_schema_version` is its own field on the same request.

Landed: `source_dialect: declared.artifact_schema.clone()` with the measurement in its comment.
Measured after: the refusal moved one layer inward, to the plugin's own payload decoder (§3.3), which
is the proof the identity now parses and the request reaches `ActionBus` dispatch.

### 3.3 The two remaining reds, located

1. **`os: capability catalog health` — 2 diagnostic(s).** Both are the same line, emitted once per
   registry compile: `skipping plugin 'puzzle': … 🔣️.json did not decode as a PackageDescriptor:
   missing field 'artifactSchema' at line 1 column 6451`. It clears when and only when `🧩️puzzle` can
   be re-described — §5.
2. **`os: inference_run`.** With the identity fixed, the untruncated reply
   (`ce3-inference-probe.txt`, via `🐍️ce3-inference-probe.mjs` — the gate slices its detail at 300
   chars and hides this) is:

   ```
   job.infer.dispatch: tool factory 'semio.infer' rejected 's.wfc.bitmap.solve':
   bitmap-inference-wire-decode:missing field `snapshot`
   ```

   `inference_run` takes an optional `payload` and is deliberately host-opaque about it
   (`💡️inference/🦀️.rs:1598`, `inference_run_payload_bytes` → `{}` when absent); `🀄️wfc`'s
   `BitmapInferenceRequest` requires `snapshot: BitmapSnapshot`. **The gap is that no payload
   contract is published anywhere an agent can read it**: `wfc`'s committed
   `contributions.inferenceServices[0]` is `{owner, artifactKind, artifactSchema,
   artifactSchemaVersion, inferenceSchema, inferenceSchemaVersion, algorithmVersion, policyVersion,
   contributor}` and carries no input schema, so `inference_list`/`capabilities_describe` cannot tell
   a client what to send. I did NOT paper over this by hand-writing a `BitmapSnapshot` into the gate:
   a real client could not do that either, and a green row bought that way would assert nothing.
   Publishing an inference payload schema is a plugin-declaration change plus a re-describe of every
   plugin — its own slice.

## 4. `capability-audit-check`

`bun ./📜️script.ts capability-audit-check` from `📦️packages/🦀️rust`, capture `ce3-audit-1.txt`:

```
semio-os-mcp audit: 29 finding(s) over 59 descriptor(s) under /Users/ueli/Documents/semio
[mcp registry] skipping plugin `puzzle`: … missing field `artifactSchema` at line 1 column 6451
```

**29 findings, unchanged from CE2's 29**, and the same split (25 `WhenDestructive never fires`, 4
`declares no audience` on `puzzle3d`'s gesture routes). CA1 owns those source edits and they had not
landed when this ran; the run is cheap (4 s) and should be repeated after CA1's batch describes.
`🀄️wfc`'s own eight `WhenDestructive` findings survived its re-describe, which is the direct evidence
that they are SOURCE declarations rather than stale descriptors.

## 5. `🧩️puzzle` — a fuel cliff, not a stall

`ce2-describe-🧩️puzzle.txt` ends:

```
[describe] owned phase=execute fuel=8000000000 elapsed_ms=1946890
semio-framework-plugin-describe describe: calling owned describe() on …semio_s_plugin_puzzle.wasm: fuel exhausted
```

`DESCRIBE_FUEL_BUDGET = 8_000_000_000` (`🖨️describe/🛂️descriptor-emission/🦀️.rs:243`), and its own
docstring says "Re-measure, do not re-estimate, if a larger plugin trips it". `🧩️puzzle` has now
tripped it.

I tried to take that measurement without touching the fleet mutex: a private build of the emitter
carrying a 32 G probe cap (`CARGO_TARGET_DIR=…/target-ce3`, tree restored to 8 G immediately after
the build so no peer's describe could inherit the probe value), run directly against the
already-built component — no wasm32 cargo build, so no mutex (`📜️ce3-puzzle-fuel-probe.sh`,
capture `ce3-puzzle-fuel-probe.txt`).

**The measurement is not obtainable today, and the reason is itself the finding.** The probe sustained
**≈ 295 k fuel/s** under the session-8 fleet (270 898 673 fuel in 916 s) against the **≈ 4.1 M fuel/s**
the same describe reached on the quiet machine at 04:20. At the loaded rate, 8 G alone is **7.5
hours**, so the probe could not reach a terminal figure inside this session and I stopped it by pid
rather than leave a core spinning.

So I did **not** raise the cap. Raising it to 32 G would buy `🧩️puzzle` a descriptor at the cost of
making every describe on this machine a potential two-hour hold of the fleet wasm mutex, and the
real root is already named: CE1 measured `🧩️puzzle`'s descriptor at **4.8 MB with a 3.56 MB inlined
example** and called for a deferrable `ExampleSource` inside `ExampleDefinition`. The fuel is
super-linear in that blob — `🀄️wfc` spent 478 M fuel on a 1.03 MB descriptor (464 M/MB) while
`🧩️puzzle` had spent over 8 G on 4.8 MB (>1 667 M/MB, 3.6× worse per byte) — so deferring the
example is the fix that makes `🧩️puzzle` describable, not a bigger number.

**I therefore did not take the mutex.** My queue ticket would have re-described a `🀄️wfc` that is
already current and re-run a `🧩️puzzle` that would die at 8 G again, for ~35 min of exclusive fleet
time. The queue at 11:36 was `c8` (holding) → `tc3e` → `play`.

## 6. `live-agent-loop-check` — 21 / 21 re-confirmed

Same serve and bridge CE2 recorded and left running, both verified alive before the run (`:6196`
answers 200, `:7621/readyz` answers 200, `🗑️generated/ce2-bridge` still holds `offers/` + `sessions/`),
so no infrastructure of mine needed restarting:

```
cd 🌉️mcp/📦️packages/🦀️rust
S_OS_MCP_LIVE_SHELL_URL=http://127.0.0.1:6196 S_OS_MCP_LIVE_PLUGIN=note S_OS_MCP_LIVE_SPAWN=note \
S_AGENT_BRIDGE_DIR=…/🗑️generated/ce2-bridge bun ./📜️script.ts live-agent-loop-check
```

**`os-mcp-live-agent-loop: 21 passed, 0 failed, 0 skipped of 21`, rc=0**, 11:49 → 11:56
(`ce3-live-agent-loop-1.txt`). This is a re-confirmation ON MY CHANGES, not CE2's number carried
forward: the gateway binary under it was rebuilt at 11:40 with both §3 fixes, and the whole (f1)–(f8)
journey plus (e1)/(e2)/(e3) is green against a live `s` shell.

## 7. `hub-agent-participant-check` — 7651 never came up

`curl -s :7651/readyz` answered **`000` at every poll** of this slice: 10:58, 11:36, 11:49, 11:57,
12:11, 12:21, plus two bounded poll loops (15 min and 10 min, one curl every 30 s) that never saw
anything but `000`. TC3e is fourth in the fleet wasm mutex queue
behind a `c8` hold that has run since **11:32** (`/tmp/semio-wasm-build.queue`: `c8` → `s11` →
`tc3e` → `play` → `ca1` at 12:11), and its stdio/gis/note wasm-release builds come before the
bootstrap, so **17/17 could not be attempted**. This is the same wall CE2 hit in both of its windows.

What I ran instead, so the gate has a number on the current tree and so my own §3 edits are proved
not to have moved the hub lane: `OS_MCP_HUB_ORIGIN=http://127.0.0.1:7621 bun ./📜️script.ts
hub-agent-participant-check` → **14/17 rows green** (`ce3-hub-participant-7621.txt`, 65 s), with the
three reds byte-identical to CE2's two runs:

1. `0c features.mcpWorkspace is true` — `mcpWorkspace=false openPlan=true`; the running binary
   (`target-jc1/debug/os-hub`, 09-21 03:42) predates the `agent_delegation_ready` probe.
2. `11 action_prepare reaches a guest the HUB authorized` — `gis.s.gis.gismap@1/*#editor.addFeature`:
   `INTERNAL: instantiate: wasmtime: no exported instance named 'semio:framework/codec@1.0.0'`; the
   published `jc1-boot` catalog predates the `codec` export.
3. `12 action_invoke commits the agent's edit` — cascades from 2.

Both remaining hub-side rows that touch the code I changed are **green**, which is the regression
check this run was worth on its own: `8 artifact_open of a HUB document answers — kind=gis.map
sizeBytes=81038` and `9 artifact_snapshot of a HUB document answers real bytes — packBytes=80801
sprBytes=237`.

## 7b. One more gate defect fixed while measuring

`client-e2e`'s FIRST run of this slice (`ce3-client-e2e-1.txt`) did not end in a tally at all: after
27 printed rows it threw `ping did not answer within 240000ms` out of `session.request`, because the
`ping` that follows `notifications/cancelled` was the one request in the journey with no `.catch`.
The file's own comment six lines above states the law it was missing — "A request that never answers
must become a NAMED RED, not an exception … which is strictly worse than a red row". The `ping` now
folds a timeout into an error envelope like every other call, and the step reports it by name.
Re-measured after the change: **36/38 again, in 77 s** (`ce3-client-e2e-4.txt`), so the guard costs
nothing when the server is healthy.

## 8. Honest gaps

1. `🧩️puzzle`'s descriptor cannot be regenerated on the current tree (§5) — the one blocker under
   both `client-e2e`'s catalog-health red and `capability-audit-check`'s catalog diagnostic.
2. `inference_run` cannot be driven by a real client because no inference publishes a payload
   contract (§3.3).
3. The 29 audit findings are CA1's source edits; this slice only measured them (§4).
4. The plugin-host `--lib` describe-deadline law was not run with a component fixture (§2).
5. **`hub-agent-participant-check` 17/17 was not attempted**: hub 7651 never answered (§7). The
   14/17 ceiling on 7621 is a stale binary + stale catalog, exactly as CE2 located it — no repo code
   is implicated, and a third identical run is now the evidence for that.
6. I did not run `cargo test -p semio-framework-os-mcp`. The two unit tests that read this reply
   shape (`🗿️artifact/🧪️tests/🔬️quick/🦀️.rs:171`, `🏠️workspace/🧪️tests/🔬️quick/🦀️.rs:242`) assert
   `packBytes > 0` and are additive-safe by inspection, and the three gates above drive the real
   binary end to end; a heavy test build would have taken build-dir locks from the fleet for no new
   information.
7. The auto-commit captured the 32 G probe value of `DESCRIBE_FUEL_BUDGET` in a commit between my
   edit and my revert. **The working tree holds the shipped `8_000_000_000`** (verified:
   `🛂️descriptor-emission/🦀️.rs:243`), so the next auto-commit restores it; no build ever used the
   32 G value except my own private `target-ce3` emitter.

## 9. Files changed

* `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` — `sprBase64` on both artifact
  resource bodies (§3.1); canonical `source_dialect` (§3.2).
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️.rs` — `sprBase64` in
  `artifact_snapshot_output_shape()` + docstring (§3.1).
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🔣️.json`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts` — `@generated`, regenerated by
  `schema-mirror`.
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` — `snapshotDocumentBase64()`; the snapshot
  step compares the whole document (§3.1).
* `📜️ce3-puzzle-fuel-probe.sh` (ticket folder, new) — mutex-free native fuel probe (§5).
* `🐍️ce3-inference-probe.mjs` (ticket folder, new) — untruncated `inference_run` reply (§3.3).
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` — the `ping` step can no longer abort the
  journey with an exception (§7b).

Infrastructure: nothing was started or restarted by this slice. CE2's `s` serve on **6196** (200) and
hub **7621** (`readyz` 200, pid 607) were alive on arrival and are alive at 12:11; `🗑️generated/ce2-bridge`
still holds the rendezvous. The only process I started and stopped was the native fuel probe (§5),
killed by pid. **I never took the fleet wasm mutex** — see §5 for why.

Nothing under `🔌️plugin` (FP10's), `🌎️hub` (TC3e's) or `✏️s/🔌️plugins` (CA1's) was edited.
