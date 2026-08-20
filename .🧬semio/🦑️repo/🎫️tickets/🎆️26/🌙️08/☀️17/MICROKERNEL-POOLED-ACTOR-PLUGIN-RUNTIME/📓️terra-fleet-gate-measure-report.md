# 📓 terra-fleet-gate-measure — independent check report

Packet: `fleet-gate-measure`. **Measurement only — no source file was edited.** Confirmed via
`git status --porcelain -- 🧰️framework ✏️s 🌎️hub` before writing this report: every modified path
belongs to a sibling/peer, none to this session.

All builds ran in the FOREGROUND with `timeout: 600000`,
`CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-fleetgate`,
exit codes captured with `command > file 2>&1; echo $?` (never through `| tail`).

---

## 1. Compile state — the 8 named crates, each `--lib`

| crate | exit | notes |
|---|---:|---|
| `semio-framework-graph` | **0** | matches gate-graph's claim |
| `semio-framework-3d` | **0** | matches gate-3d's claim |
| `semio-framework-os-services` | **0** | matches gate-services's claim |
| `semio-framework-number` | **0** | matches ticket's `number-green` baseline |
| `semio-framework-actor` | **0** | matches ticket's `actor-green` baseline |
| `semio-framework-os-kernel` | **0** | matches ticket's baseline, no regression |
| `semio-framework` | **0** | matches ticket's baseline, no regression |
| `semio-framework-plugin` | **101** — 7 errors | **not** in the brief's "all EXIT 0" list of 4, and it is not green. Matches the pre-existing, already-documented `sdk-final`/rule-27 baseline exactly: 3× E0277 `ArtifactStore: MemberFactory` + 4× E0308, all at `dispatch_group`/`MemberFactory`, all in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (line 11525/11533/11701/11709). This is the orphan-rule blocker rule 27 already recorded as confirmed-impossible from this crate; **not a regression**, just re-confirmed still open. |

**No regression on any of the four crates the brief flagged as "report immediately and loudly if any regressed."** All four (`os-kernel`, `semio-framework`, `actor`, `number`) are EXIT 0, as claimed.

Raw logs: `chk-graph.txt`, `chk-3d.txt`, `chk-services.txt`, `chk-number.txt`, `chk-actor.txt`, `chk-oskernel.txt`, `chk-framework.txt`, `chk-plugin.txt` in the session scratchpad (paths below).

---

## 2. THE HEADLINE — first real fleet compile

```
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=.../scratchpad/target-fleetgate \
  cargo check -p semio-s-plugin-note --lib --keep-going --message-format=short
EXIT: 101
```

**Result: the closure does NOT reach `semio-s-plugin-note`'s own source at all** — no
`Checking semio-s-plugin-note` line appears anywhere in the 523-line output (confirmed by grep).
It aborts two crates upstream of `note`, both inside `semio-framework-plugin`'s own dependency
chain:

| failing crate | errors | previously known? |
|---|---:|---|
| `semio-framework-plugin` (lib) | **7** | **Yes** — exactly rule 27's confirmed-impossible-from-this-crate `dispatch_group`/`MemberFactory` orphan-rule blocker. |
| `semio-framework-plugin-host` (lib) | **123** | **No — new finding.** Nothing in `important.md` documents a 123-error residue on `plugin-host --lib`. The prior `plugin-host` RED note (`terra-actor-green`, now marked STALE) was 6 **parse** errors in `🗣️dsl/🧬️schema` cascading through `os-kernel`; that is fixed (`os-kernel --lib` is EXIT 0 here). This is a **different, newly-surfaced** blocker: classic R10-residue "dropped/misplaced `.await`" shapes (see breakdown below), entirely within `plugin-host`'s own files. |

**Total: 130 errors** across the two failing crates in this closure walk. `--keep-going` let both
fail independently and it still could not proceed past them to reach `note`.

### `semio-framework-plugin-host` (123 errors) — bucketed

**By error code:**

| code | count |
|---|---:|
| E0308 (mismatched types) | 45 |
| E0277 (trait bound / not an iterator / Display / Try) | 35 |
| E0599 (no method found on opaque `impl Future`) | 24 |
| bare `error:` (no code — "method should be async or return a future") | 12 |
| E0107 (missing generics, `AsyncServices<T>`) | 3 |
| E0600 (unary `!` on a future) | 2 |
| E0282 (type annotation needed) | 1 |
| E0609 (field access on a future) | 1 |

**By file (all under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/`):**

| file | errors |
|---|---:|
| `../../🦀️component.rs` (host's main component) | 40 |
| `../../⏳️imports.rs` | 23 |
| `../../⚡️effects/🦀️component.rs` | 21 |
| `../../🧵️shard/🦀️component.rs` | 10 |
| `.../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🦠️mutation/🦀️component.rs` | 8 |
| `.../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🦠️mutation/🦀️component.rs` | 8 |
| `.../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🔺️diff/🦀️component.rs` | 4 |
| `.../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🔺️diff/🦀️component.rs` | 4 |
| `.../🎚️config/🧬️schema/🧬️mutations/🦀️component.rs` | 2 |
| `.../🎚️config/🧬️schema/🦀️component.rs` | 2 |
| `../../🧵️shard/🏃️executor.rs` | 1 |

**Shape, by inspection of the messages:** this is textbook R10 residue, dominant shapes #6
("constructor left un-awaited, every use written `x.await.method()` / `x.method()` on the raw
future") and #1 (sync-context misuse) — `expected bool/String/Vec<..>/ActorId/CancelToken/Budget,
found future`, `no method named 'clone'/'map_err'/'ok'/'warn'/'saturating_add' found for opaque
type impl Future<...>`, `impl Future<...> doesn't implement Display`, `impl Future<...> is not an
iterator`, and 3 sites of `missing generics for struct AsyncServices: expected 1 generic argument`
(a signature drift, not an await issue). This was **not fixed by any packet on this ticket** —
it needs its own dedicated packet. **`plugin-host` is not among the peer-owned live-edit paths**
(`💻️os/🔨️modules/🏪️store`, `🗣️dsl/🧬️schema`, `💡️inference`) — none of the 11 files above touch
those trees, so this is a legitimate open target, not another session's in-flight work.

### Task 3 (bucket `note`'s own errors) — N/A, reported honestly

`note` never reaches its own source in this closure walk, so there is nothing of `note`'s own to
bucket. The bucketing above is of the two crates actually blocking it. Re-run task 3 once
`plugin-host` and `plugin` are both green.

Raw log: `chk-note-headline.txt` (full, 523 lines) in scratchpad.

---

## 4. `semio-s-plugin-stdio --lib`

```
EXIT: 101
```

Aborts at the **exact same point** as `note` — `semio-framework-plugin`, same 7 errors, same
locations (`🦀️component.rs:11525/11533/11701/11709`). (Run without `--keep-going`, so it does not
show the downstream `plugin-host` failure, but since `plugin-host` depends on `plugin` and `plugin`
fails first, `plugin-host` would necessarily also block `stdio`.) Confirms gate-graph/3d/services's
completion did unblock the fleet from the crates those three packets owned — the remaining
blockers are squarely `semio-framework-plugin` (known) and `semio-framework-plugin-host` (new).

Raw log: `chk-stdio-headline.txt` in scratchpad.

---

## 5. Censuses

Reused the prior packet's script (`terra-sdk-gate-census-dyn_census.py`, comment/string-stripped,
path-explicit, R10-safe) unmodified in logic — copied to
`.../scratchpad/terra-fleetgate-dyn_census.py`, only the output path patched. Cross-checked the two
numbers that matter most with a **differently-implemented tool** (`ripgrep`, raw, no
comment-stripping) per rule 21.

### First-party `dyn` — **57** (was 84 → **−27**)

| area | count |
|---|---:|
| `🧰️framework` | 52 |
| `✏️s` | 5 |
| `🌎️hub` | 0 |

Top offenders by trait: `HttpBody` 7, `HttpTransport` 5, `RouterEffectHandler` 5, `Operator` 4,
`OsBackbonePort` 4 (all pre-existing, documented exceptions — `HttpTransport`/`HttpBody` per R8's
own note, `OsBackbonePort` is `✏️s`-side). std/lang `dyn` (permitted, R1) totals 187
(`Fn`/`FnMut`/`FnOnce`/`Any`/`Error`/`Future`), unaffected by this ticket's zero-dyn target.

Consistent with the three gate packets' work (`gate-services` alone removed a generic-Send `dyn`
via R11's "exactly one impl ⇒ delete the trait object" on `BlockingHttpTransport<R>`) plus earlier
sessions' dedyn packets. **No discrepancy** — the brief cites "was 84" as prior-session context, not
a claim from today's three siblings, and the direction (down) is exactly what their summaries
implied without any of them re-running the repo-wide census themselves.

### `async fn` vs plain `fn` ratio — **85.62%** (was 86.7%)

| area | async fn | plain fn |
|---|---:|---:|
| `🧰️framework` | 9,798 | 10,109 |
| `✏️s` (fleet plugins) | 56,447 | 966 |
| `🌎️hub` | 251 | 90 |
| **total** | **66,496** | **11,165** |

`66496 / (66496+11165) = 85.62%`. This is a **slight regression from the cited 86.7%**, worth
flagging honestly rather than smoothing over: `🧰️framework` itself is only ~49.2% async
(9798/19907) — roughly half its `fn` items are still plain. That is not itself alarming on its own
(a large fraction of `🧰️framework` plain fns are legitimately E1/E2/E3 — trait impls, `const fn`,
derive/proc-macro internals — the tag census below shows 505 tagged exceptions, and untagged E4/E5
would show up as compile errors, not silently), but it means the 85.62% headline ratio is being
carried almost entirely by the fleet's 56,447-strong `✏️s` async count (98.3% async there), and a
regression in `🧰️framework`'s own ratio would be invisible in the blended number. **Recommend the
next census report `🧰️framework` and `✏️s` ratios separately, not just blended**, since they are
telling two very different stories right now.

Only 23 `.rs` files exist under any on-disk `🤖️generated/**` dir in the scanned areas (16 dirs) —
not enough volume to be the source of the 49% framework figure; this is real production/test code.

### `🚫️async:` tag count — **505**, cross-checked exact match via ripgrep

| class | count |
|---|---:|
| E1 | 257 |
| E4 | 143 |
| E5 | 30 |
| E3 | 62 |
| E6 | 13 |
| E2 | 0 |

`ripgrep -o '🚫️async: E\d' … | wc -l` → **505** — exact match with the python census's `tag_total`.
No E2 (`const fn`) tags found anywhere — either no `const fn` needs the marker in this codebase's
convention, or `const fn` sites aren't being tagged; not investigated further (out of this packet's
measurement scope).

Raw outputs: `terra-fleetgate-dyn_census_stdout.json`, `terra-fleetgate-dyn_census_stderr.txt`,
`terra-fleetgate-dyn_census.py` (script) in scratchpad.

---

## 6. Discrepancy summary — every sibling's claim vs my measurement

| claim | source | measured | verdict |
|---|---|---|---|
| `graph`/`3d`/`services` all green (`--lib` EXIT 0) | gate-graph/gate-3d/gate-services | all EXIT 0 | ✅ confirmed |
| `os-kernel`, `semio-framework`, `actor`, `number` all EXIT 0 | packet brief | all EXIT 0 | ✅ confirmed, no regression |
| SDK/`note`/`stdio` closure previously blocked on `semio-framework-3d` / `semio-framework-number` | `important.md` cross-packet findings | both now green, closure moved on | ✅ consistent — the blocker moved as expected once those two went green |
| the SDK crate itself was "assumed to be the last gate" | packet brief | **false, confirmed again** — `semio-framework-plugin` (7, known) and `semio-framework-plugin-host` (123, **new**) both still block `note`/`stdio` before the SDK crate is even reached | 🚨 **naming `semio-framework-plugin-host` is the actionable new discovery this packet produced** |
| dyn was 84 | prior baseline | **57** | net improvement, not a discrepancy |
| async ratio was 86.7% | prior baseline | **85.62%**, blended; `🧰️framework` alone only ~49.2% | not a red flag on its own, but flagged as worth separating framework-only vs fleet-only in the next census |

---

## Bottom line for the coordinator

1. **No regressions.** Every crate the brief demanded stay green is still green.
2. **`semio-s-plugin-note` and `semio-s-plugin-stdio` still never reach the SDK crate or their own
   source.** Two blockers, both inside `semio-framework-plugin`'s own tree:
   - `semio-framework-plugin --lib`: 7 errors, **already fully diagnosed and confirmed impossible to
     close from this crate** (rule 27 — orphan rule, needs a `🏪️store` change, `lease-request`
     already open).
   - **`semio-framework-plugin-host --lib`: 123 errors, newly discovered by this packet, not
     previously documented anywhere in the ticket.** Textbook R10 await-insertion residue
     (dropped/misplaced `.await`, shapes #1 and #6), concentrated in `⏳️imports.rs` (23),
     `⚡️effects/🦀️component.rs` (21), `🧵️shard/**` (11), the host's main `🦀️component.rs` (40), and
     the `🎚️config/🧬️schema/🧬️mutations/**` tree (26). None of it touches the three peer-owned live
     paths (`🏪️store`, `🗣️dsl/🧬️schema`, `💡️inference`) — it is a clean, unclaimed target. **This
     needs its own dedicated packet before the fleet-readiness question can be re-measured.**
3. Censuses show continued progress (dyn 84→57) and one soft signal worth a follow-up (blended
   async ratio dipped 86.7%→85.62%, driven by `🧰️framework` itself sitting at only ~49% async —
   recommend reporting the two areas separately going forward).

## Files

Ticket-folder scratch (load-bearing, `terra-fleetgate-*` prefix), same directory as this report:
`terra-fleetgate-chk-graph.txt`, `terra-fleetgate-chk-3d.txt`, `terra-fleetgate-chk-services.txt`,
`terra-fleetgate-chk-number.txt`, `terra-fleetgate-chk-actor.txt`, `terra-fleetgate-chk-oskernel.txt`,
`terra-fleetgate-chk-framework.txt`, `terra-fleetgate-chk-plugin.txt`,
`terra-fleetgate-chk-note-headline.txt`, `terra-fleetgate-chk-stdio-headline.txt`,
`terra-fleetgate-terra-fleetgate-dyn_census.py` (census script, copied),
`terra-fleetgate-terra-fleetgate-dyn_census_stdout.json`,
`terra-fleetgate-terra-fleetgate-dyn_census_stderr.txt`.

Session scratchpad (build cache only, per rule 24 — not load-bearing):
`/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-fleetgate/`.

No files under the repo's source tree (`🧰️framework`, `✏️s`, `🌎️hub`, etc.) were created, edited, or
removed by this packet — confirmed via `git status --porcelain` at the start of §1.
