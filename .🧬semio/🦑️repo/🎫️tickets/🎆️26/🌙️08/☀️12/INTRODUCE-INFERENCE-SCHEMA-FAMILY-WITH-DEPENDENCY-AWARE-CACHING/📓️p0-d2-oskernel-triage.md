# P0-D2 — os-kernel lib-test-build ownership triage

Task: determine whether the ~144-error `semio-framework-os-kernel --lib` test-build failure
reported by SMO (#2545) is caused by our ticket's spine work (`💡️inference`, `#[state(inferred)]`,
`ArtifactInferrer`, the `📡️spr` Inference region) or is pre-existing/someone else's.

## Reproduction

**First attempt** (real command, real output — but NOT a valid reproduction of the reported bug):

```
CARGO_TARGET_DIR=".../🎯️target" cargo test -p semio-framework-os-kernel --lib --no-run
```

Ran ~16:18–16:24. Full contents of `scratch-p0-d2-testbuild-raw.txt` at that point:

```
    Blocking waiting for file lock on build directory
error: failed to link or copy `.../🎯️target/debug/build/proc-macro2-06b0ba31cf55c6cc/build_script_build-06b0ba31cf55c6cc` to `.../🎯️target/debug/build/proc-macro2-06b0ba31cf55c6cc/build-script-build`

Caused by:
  No space left on device (os error 28)
```

This was **not** a 144-error compile failure, it was disk exhaustion (`df -h` at the time: `/dev/disk3s5 926Gi 429Gi 433Gi 50%` — actually already showed room; the ENOSPC came from a since-deleted 428G repo-root `target/` that filled the disk mid-build). A peer coordinator session flagged this correctly mid-task (their message quoted my own scratch file verbatim, verified true) and reported the repo-root `target/` had been deleted (442 GiB freed) and that `semio-framework-plugin` (which had been red repo-wide) had been fixed by UCAS.

**Second attempt**, re-run after the disk fix, same command, full cold rebuild (`🎯️target` for this ticket was empty of build artifacts beyond `.rustc_info.json`/`CACHEDIR.TAG`):

```
CARGO_TARGET_DIR=".../🎯️target" cargo test -p semio-framework-os-kernel --lib --no-run
```

Ran 16:24 → 18:41 (waited on the ticket's own target-dir build lock — other concurrent
subagents in this same ticket were compiling `semio-s-plugin-stdio`/`semio-s-plugin-puzzle`
against the same `CARGO_TARGET_DIR`, confirmed via `ps aux`; this is normal per
`📌️important.md` rule 5), then compiled cold for ~15m34s. Final lines of
`scratch-p0-d2-testbuild-raw.txt`:

```
warning: `semio-framework-os-kernel` (lib test) generated 109 warnings (run `cargo fix --lib -p semio-framework-os-kernel --tests` to apply 96 suggestions)
    Finished `test` profile [unoptimized] target(s) in 15m 34s
  Executable unittests 📦️glue.rs (.../🎯️target/debug/deps/semio_framework_os_kernel-d64e7b0a7fea73c1)
```

`grep -c "^error"` → **0**. `grep -oE "E[0-9]{4}"` → **no matches**. The build now succeeds
with 0 errors, 109 warnings. **The 144 errors SMO reported do not reproduce right now.**

This is not because the report was wrong — git archaeology below shows both root causes were
real and long-predate this ticket — it is because **someone else fixed both of them between
SMO's report and this second build**, confirmed by:

1. A committed fix for cluster A: commit `fd01661f06` (flag 495, 18:08:12 today), ticket
   "Subset Conformance and Integrated Roundtrips" — see `## Cluster A`.
2. An **uncommitted** (`git status`: `M`) working-tree fix for cluster B, present in
   `🏪️store/🔄️sync/🦀️component.rs` as of this writing (mtime 18:30:03 today) — see `## Cluster B`.

I did not make either change. Both are read directly off the live tree as of the second build.

## Error clusters

Could not classify the original ~144 compiler errors directly (never captured — see above).
Classification instead done by static inspection of the exact fixtures/traits SMO named, cross-
checked against the now-present fix diffs, which target precisely the two symptom clusters SMO
described:

- **Cluster A — `tempfile` not a dev-dependency**: every one of the 7 `tempfile::tempdir()` call
  sites in `🏪️store/🔄️sync/🦀️component.rs` (lines 2371, 2752, 2832, 2855, 2886, 2921, 2966 as of
  the pre-fix tree) would each produce an unresolved-crate error, plus every call site downstream
  of a function that used the returned `TempDir` — consistent with a large single-digit-to-low-
  double-digit slice of the ~144.
- **Cluster B — `DemoSnapshot`/`DemoMutation` missing `ArtifactPack`/`OpText`/`OpBinary`**: 53
  occurrences of `DemoSnapshot`/`DemoMutation` across the test module, feeding into generic
  bounds on `SyncSession<P, Mutation>`, `ArtifactStore<P, Mutation>`, `create_document_envelope`,
  `ArtifactCommand::Apply`, etc. — each of the file's 11 `#[test]` functions (plus several shared
  helper fns) would fail with `E0277` trait-bound-not-satisfied at every generic instantiation
  site, easily accounting for the bulk of ~144 given monomorphization-triggered duplication of
  the same missing-impl error across call sites.

## Cluster A tempfile — git archaeology

`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` **never** had a
`[dev-dependencies]` (or target-gated dev-dependencies) section containing `tempfile` at any
point before today:

```
$ git log -S"tempfile" --oneline -- ".../Cargo.toml"
(no output — tempfile string never added/removed in this file's whole history until today)
```

The `tempfile::tempdir()` **usage** in the test file was introduced in commit `8baa5706ec`
(flag 450, embedded timestamp `🎆️26🌙️08☀️06⏰️13⌚️55⏱️33` = **Aug 6**, six days before this
ticket started), a large multi-domain commit ("Implement store module sync worker and persistent
storage" among six other unrelated bullets):

```
$ git log -S"tempfile" --oneline -- ".../🏪️store/🔄️sync/🦀️component.rs"
8baa5706ec 🐙️ueli🎆️26🌙️06☀️04🚩️450
```

So the test code using `tempfile` was **never buildable from the moment it landed** — this is
not a regression any session caused, it is code that landed unverified (unsurprising: `cargo
check` never compiles `#[cfg(test)]` code, so nothing short of an explicit `--tests`/`--lib
--no-run` build would ever have caught it, and evidently nothing did for 6 days).

**Fix, landed by someone else during this triage**: commit `fd01661f06` (flag 495, 18:08:12
today), from ticket **"Subset Conformance and Integrated Roundtrips"** (a session not listed in
this ticket's `📌️important.md` peer table — a fourth, newer concurrent session):

```diff
+# 🧪️ `🏪️store/🔄️sync`'s actor tests use `tempfile::tempdir()`; they are gated
+# `#[cfg(not(target_arch = "wasm32"))]`, so the dev-dep is target-gated to match.
+[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]
+tempfile = "3.20.0"
```

## Cluster B trait bounds — git archaeology

Traits: `ArtifactPack` (defined `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:818`),
`OpText` and `OpBinary` (both defined `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:340,354`).

Root cause is a **derive-macro design change ("P6")**, documented at the top of
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` (and its dual-mounted
compiled copy `.../✨️derive/📦️packages/🦀️rust/📦️glue.rs`, byte-identical, confirmed via `diff`):

> P6: `DslArtifact`/`DslOps` no longer emit `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` —
> those traits are handcrafted per artifact. `DslRecord` stays for field helpers only.

This landed in two commits, both **Aug 7** (embedded timestamps `⏰️14⌚️53⏱️59` /
`⏰️15⌚️28⏱️33`), both from ticket **RUNTIME-INSTALLABLE-EXTENSIONS**:

```
$ git log -S"P6:" --oneline -- ".../✨️derive/🦀️component.rs"
b92a614cad 🐙️ueli🎆️26🌙️06☀️04🚩️463
9391e1ed2b 🐙️ueli🎆️26🌙️06☀️04🚩️462
```

`9391e1ed2b`'s diff on this file removes the derive-generated `impl ::store::DocumentDsl for
#name { ... }` block entirely, replacing it with just two `pub const` envelope constants and a
doc note: `/// ✉️ Envelope constants for handcrafted DocumentDsl/DocumentPack wiring (P6: derive
no longer emits those traits).`

Confirming this was a deliberate, repo-wide migration (not an oversight): the **sibling**
`DemoMutation` fixture that lives in `🏪️store/🦀️component.rs`'s own test module (a
similarly-named but structurally separate fixture from the one in `🔄️sync/🦀️component.rs`) got
its handcrafted `impl OpText for DemoMutation` / `impl OpBinary for DemoMutation` added the very
next day, in the direct follow-up commit:

```
$ git log -S"impl OpText for DemoMutation" --oneline -- ".../🏪️store/🦀️component.rs"
daee507d43 🐙️ueli🎆️26🌙️06☀️04🚩️466
```

But `🏪️store/🔄️sync/🦀️component.rs`'s own separate `DemoSnapshot`/`DemoMutation` pair (added in
the same `8baa5706ec`, Aug 6, that introduced the `tempfile` usage) was **never given the
matching handcrafted impls** when the P6 migration swept the rest of the crate on Aug 7 — a gap
that has sat silent since Aug 7 for the same reason as cluster A (never exercised by `cargo
check`, only by an explicit test build).

`ArtifactPack`'s required-method set (`encode_pack_with`/`decode_pack_with`, defaulted
`encode_pack`/`decode_pack`/`record_spec`) and `OpText`/`OpBinary`'s signatures have **not**
changed since before this ticket's spine work landed:

```
$ git log -S"fn encode_pack_with" --oneline -- ".../🏪️store/🦀️component.rs"
b92a614cad 🐙️ueli🎆️26🌙️06☀️04🚩️463   (Aug 7 — pre-ticket)
9391e1ed2b 🐙️ueli🎆️26🌙️06☀️04🚩️462   (Aug 7 — pre-ticket)
8baa5706ec 🐙️ueli🎆️26🌙️06☀️04🚩️450   (Aug 6 — pre-ticket)
```

The only commits touching `🏪️store/🦀️component.rs` and `📡️spr/🎮️command/🦀️component.rs`
**after** our ticket's spine commit (`a714dbc6f1`, flag 489 — first commit containing the new
`💡️inference/🦀️component.rs` module) were `a445617cae` (flag 493, "Artifacts Only Plugin
Architecture" / "Dashboard Tui Workforce" — added unrelated `ArtifactChild`/`ChildRef`
composition primitives, confirmed via `grep` no touch to `trait ArtifactPack` itself) and
`1caac91709` (flag 492, "Unified Composable Artifact System" — confirmed via `grep` no touch to
`trait OpText`/`trait OpBinary`). **Neither our ticket nor any current peer ticket touched these
trait definitions or their required-method sets.**

**Fix, present in the live tree (uncommitted as of this writing)**: `🏪️store/🔄️sync/🦀️component.rs`
now has hand-written `impl ArtifactDsl for DemoSnapshot`, `impl ArtifactPack for DemoSnapshot`,
`impl OpText for DemoMutation`, `impl OpBinary for DemoMutation` blocks, added right after the
struct/enum declarations — the exact same shape as `🏪️store/🦀️component.rs`'s own
`DemoMutation` fix from Aug 7 (`daee507d43`). `git status --short` shows this file as `M`
(modified, not yet committed) — someone else's in-flight fix, not mine; I made no edits.

## VERDICT

**PRE-EXISTING / UNOWNED.**

Both symptom clusters SMO reported predate this ticket by 5–6 days and originate from two
separate, unrelated historical events:

- Cluster A (`tempfile`): commit `8baa5706ec` (Aug 6, flag 450) added test code using
  `tempfile::tempdir()` without ever adding `tempfile` to the crate's dev-dependencies — landed
  unverified, never buildable, undetected for 6 days because `cargo check` skips `#[cfg(test)]`.
- Cluster B (trait bounds): commits `9391e1ed2b`/`b92a614cad` (Aug 7, flags 462–463, ticket
  RUNTIME-INSTALLABLE-EXTENSIONS) made a deliberate "P6" design change removing derive-generated
  `ArtifactPack`/`OpText`/`OpBinary` impls in favor of handcrafted ones; the follow-up migration
  commit `daee507d43` (Aug 7, flag 466) handcrafted the impls for `🏪️store/🦀️component.rs`'s own
  demo fixtures but missed the separate demo fixtures in `🏪️store/🔄️sync/🦀️component.rs` —
  an incomplete migration, undetected for the same reason as cluster A.

This ticket's spine work (first landing at commit `a714dbc6f1`, flag 489, Aug 12) touched
`📡️spr/🎮️command/🦀️component.rs` (additively — new `//#region 🔖️Inference`, confirmed no
changes to the `OpText`/`OpBinary` trait definitions themselves), the derive macro's dual copy
(additively — new `#[state(inferred)]` match arm, confirmed no changes near the P6 removal from
5 days earlier), `🔌️plugin/🦀️component.rs`, `🧬️schema/🦀️component.rs`, and the new
`💡️inference/🦀️component.rs` module — never `🏪️store/🔄️sync/🦀️component.rs`, never that
crate's `Cargo.toml`, and never the `ArtifactPack`/`OpText`/`OpBinary` trait *definitions*.

**Deciding commits**: `8baa5706ec` (cluster A root cause, Aug 6) and `9391e1ed2b` /
`b92a614cad` (cluster B root cause, Aug 7) — all three predate and are unrelated to
`INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`.

**Current status**: both root causes now appear fixed on the live tree — cluster A committed
(`fd01661f06`, flag 495, ticket "Subset Conformance and Integrated Roundtrips"), cluster B fixed
in an uncommitted working-tree edit to `🏪️store/🔄️sync/🦀️component.rs` by an unidentified
concurrent session. The second (post-fix) `cargo test -p semio-framework-os-kernel --lib
--no-run` reproduction in this report shows **0 errors**. I made neither fix; both were observed
already-applied on the shared tree during this triage (confirmed via `git log`/`git status`, not
assumed).

## Handoff paragraph for peers

*(Written for completeness per the task's instructions, though both root causes now appear
already fixed by another concurrent session as of this triage — see VERDICT.)* The
`semio-framework-os-kernel` lib-test-build failure SMO reported (#2545) is pre-existing and
unowned by any of the four tickets currently in this tree (ours, SMO, UCAS, APA). It has two
independent root causes, both dating to Aug 6–7, days before any of the four current tickets
started: (1) `🏪️store/🔄️sync/🦀️component.rs`'s tests use `tempfile::tempdir()` but
`tempfile` was never added to the os-kernel crate's dev-dependencies (commit `8baa5706ec`, Aug
6) — fix is a target-gated `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]
tempfile = "3.20.0"` block in `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`; (2)
an Aug 7 "P6" derive-macro migration (commits `9391e1ed2b`/`b92a614cad`, ticket
RUNTIME-INSTALLABLE-EXTENSIONS) stopped `#[derive(DslArtifact)]`/`#[derive(DslOps)]` from
auto-generating `ArtifactPack`/`OpText`/`OpBinary` impls, requiring them handcrafted per type;
the follow-up migration commit (`daee507d43`) handcrafted them for `🏪️store/🦀️component.rs`'s
own `DemoMutation` fixture but missed the separate, similarly-named `DemoSnapshot`/`DemoMutation`
fixture pair in `🏪️store/🔄️sync/🦀️component.rs` — fix is the same handcrafted
`ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` impl pattern already used next door. We are not
doing either fix (read-only ownership triage per our task). As of this triage both fixes appear
already applied on the live tree by another concurrent session (cluster A committed as
`fd01661f06`; cluster B as an uncommitted edit) — a second build now shows 0 errors — so this
paragraph may already be moot; worth a quick `git status`/rebuild check before anyone spends time
on it.

## Concurrent-churn observations

- The ticket's own `🎯️target` build directory was shared by multiple concurrent subagents within
  this same ticket (`cargo test -p semio-s-plugin-stdio --lib -- inference`, `cargo check -p
  semio-s-plugin-puzzle --all-targets`, `cargo test -p semio-s-plugin-puzzle --lib`, all
  confirmed via `ps aux` pointing at this ticket's `CARGO_TARGET_DIR`) — this is why the first
  and second `cargo test -p semio-framework-os-kernel` invocations both opened with "Blocking
  waiting for file lock on build directory"; per `📌️important.md` rule 5 this is normal, not
  contention with another ticket.
- Mid-task, a peer coordinator session sent a message correctly identifying that my first
  reproduction attempt had captured a disk-exhaustion error (repo-root `target/`, 428G, since
  deleted), not the reported 144-error compile failure, and reported that `semio-framework-plugin`
  (previously red repo-wide) had been fixed by UCAS. I independently verified the disk-full claim
  by re-reading my own scratch file and checking `df -h` before acting on it.
- A **fifth** concurrent session, ticket "Subset Conformance and Integrated Roundtrips"
  (commit `fd01661f06`, flag 495, 18:08:12 today — not listed in this ticket's
  `📌️important.md` peer table), landed the `tempfile` dev-dependency fix on `Cargo.toml` while
  this triage was in progress.
- An unidentified session made an uncommitted edit to `🏪️store/🔄️sync/🦀️component.rs` (mtime
  18:30:03 today) adding the handcrafted `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` impls
  for `DemoSnapshot`/`DemoMutation` — present in the working tree, not yet auto-committed at
  time of writing. I did not make this edit (this triage is read-only per task instructions).
- Neither change was made by this session. Both were discovered already-present via `git log`
  (cluster A) and `git status`/`stat` (cluster B), not inferred or assumed.
