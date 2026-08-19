# 🔒️ lease-request — `🧰️framework/🔨️modules/🌱️value/🔀️serde/🦀️component.rs`

**From:** terra, packet `db-dedyn` (`semio-framework-os-kernel-db`).
**Blocking:** every `cargo check -p semio-framework-os-kernel-db` run — `semio-framework-replication`
(a transitive dependency of my crate) fails to compile because ITS dependency `🌱️value`'s serde
adapter is currently broken in the working tree.

## Finding

`🧰️framework/🔨️modules/🌱️value/🔀️serde/🦀️component.rs` has **uncommitted** working-tree changes
(`git diff --stat HEAD` shows 92 lines changed, `git log` shows no commit) that added the literal
`async` keyword to `impl Serializer for ...`/`impl Deserializer for ...` methods — classic **E1**
damage (externally-declared trait, signature fixed outside this repo) from the blind fleet codemod,
the exact class `deasyncify-external-impls.py` exists to undo.

Confirmed via a **read-only** `--scan` (nothing written):

```
python3 <ticket>/deasyncify-external-impls.py --scan 🧰️framework/🔨️modules/🌱️value
local traits known: 238 | files scanned: 2 | files touched: 1
{"reverted": 34, "kept_local": 19, "scanned_fns": 53}
reverted by trait:
  Serializer   28
  Deserializer  6
```

`cargo check -p semio-framework-os-kernel-db --lib` (CARGO_TARGET_DIR=scratchpad/target-db) fails
with 68 errors, all originating in `semio-framework-replication`'s build of this same file
(E0728 "await only allowed inside async fn", E0053 "incompatible type for trait: found future",
E0277 on `Serializer`/`Deserializer`/`SeqAccess`/`MapAccess` bounds) — none of them are in my crate,
all of them are upstream of it.

## Why I'm not fixing it myself

`🌱️value/**` is outside `db-dedyn`'s owned writable paths (`🛢️db/**` and this ticket folder only).
The `deasyncify-external-impls.py --apply` fix is mechanical and low-risk (removes only the wrongly
added `async`, touches nothing else, already validated by sol per `📌️important.md`), but applying it
to a file I don't own — possibly while whatever packet is mid-flight on that file — risks the exact
R6 "interrupted atomic" failure mode this ticket already paid for once (`db-trait-flip`, 84 errors).

## Ask

Either: (a) sol or the packet that owns `🌱️value` runs
`python3 <ticket>/deasyncify-external-impls.py --apply 🧰️framework/🔨️modules/🌱️value --report <ticket>/terra-dbdedyn-value-e1.json`,
or (b) confirm it's mid-flight and will self-resolve, so I know whether to keep polling or park this
packet's acceptance run until it clears.

## What I'm doing meanwhile

Continuing the `db-dedyn` structural work (enum de-dyn, `HostAsyncRuntime` genericization, block_on
classification) from source-level review and re-reading files before every edit, and will retry
`cargo check` periodically without polling in a tight loop.

## Update — 🌱️value cleared, a SECOND upstream blocker found: `🎒️pack`

The `🌱️value` blocker above is gone (working-tree diff dropped from 92 lines to 1 between two of my
retries) — no action needed there anymore.

`cargo check -p semio-framework-os-kernel-db --lib` now fails one crate further upstream:
`semio-framework-pack` (67 errors), all in `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️component.rs`
— `?` applied to a value that doesn't impl `Try` (an `impl Future` where a `Result` is expected),
`u32 == impl Future<Output = u32>`, etc. — the same "async added somewhere it now breaks a caller"
shape, but this time it looks like a **genuine, currently-mid-flight** async conversion, not E1
damage: `git diff --stat HEAD` shows **uncommitted** changes across `🎒️pack/⏳️async`,
`🎒️pack/🌐️http`, and `🎒️pack/📐️format` (234 lines), all three touched together, consistent with one
packet actively converting `pack`'s trait family and not yet finished landing every caller.

`🎒️pack` is explicitly in this ticket's "NOT yours" list, so I'm not touching it. Re-checked three
times over the session (several minutes apart each) — still uncommitted, error count went 67 → 73
between the last two checks, so it is actively being edited, not close to landing, and not
self-resolving on this session's timescale.

**Final status at report time: still blocked.** I completed all of `db-dedyn`'s structural work
(enum de-dyn, `HostAsyncRuntime` genericization, `DbFuture` removal, block_on classification) via
careful source review and re-reading every file before editing, but could not get a single
`cargo check -p semio-framework-os-kernel-db --lib` to finish end-to-end this session — every
attempt failed one crate short of mine, first on `🌱️value` (cleared), then on `🎒️pack` (still
broken at time of writing). **Please re-run acceptance once `🎒️pack` lands** — full detail in
`📓️terra-db-dedyn-report.md`.
