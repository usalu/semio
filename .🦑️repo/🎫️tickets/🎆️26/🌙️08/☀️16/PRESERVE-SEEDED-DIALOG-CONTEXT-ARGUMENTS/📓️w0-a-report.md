# Lane 0-A report — Directory schema + pure read model (contract C1)

## Changed files

**New (my lease):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️component.json` — JSON Schema
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️component.rs` — wire types (`DirectoryEvent`, `DirectoryEventBody`, `DirectoryCommand`, `DirectoryStreamMessage`, `DirectoryActor`, `Hlc`, and the six read DTOs)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️component.ts` — TypeScript twin of the above
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️component.rs` — `DirectoryReadModel`/`DirectorySpace`, pure `fold`/`fold_all`, Rust unit tests
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🟦️component.ts` — TypeScript twin of the fold + type guards (`isDirectoryEventBodyKind`, `isDirectoryCommandKind`, `isDirectoryStreamMessageKind`)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧾️events.json` — golden fixture, 16 events, dense `seq` 1..16

**Modified (my lease, purely additive):**
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` — one added mount: `#[path = "../../🔨️modules/📇️directory/🦀️component.rs"] pub mod os_directory;` (plus a 3-line doc comment), placed right after the existing `os_vcs` mount. `git diff --stat` confirms 6 insertions, 0 deletions.
- `🧰️framework/🛍️products/💻️os/🟦️component.ts` — one new `//#region 🔖️Directory` block appended after the file's last existing region (`StdioFormatKinds`), with explicit `export type {...}` / `export {...}` re-exports (no `export *`, per CLAUDE.md) and an `if (import.meta.vitest)` parity test block. `git diff --stat` confirms 80 insertions, 0 deletions — no existing region (including the peer-leased `AppChannelCodec`/`AppChannelClient`/`🧪️Tests`) was touched.

## Design notes / how the contract was implemented

- **Two-file split**, mirroring the existing `🔨️modules/🗣️dsl/🧬️schema/` + `🔨️modules/🗣️dsl/🦀️component.rs` shape: `🧬️schema/` holds pure wire types (no logic); the module-root `🦀️component.rs`/`🟦️component.ts` hold `DirectoryReadModel`/`fold`/`fold_all`, importing from `./🧬️schema/...`.
- `DirectorySpaceKind`/`DirectorySpaceVisibility`/`DirectorySpaceRole` are re-declared string-identically (atelier/studio/archive, private/public, author/spectator) rather than imported from `🪐️space`, because `🪐️space` is not mounted into `semio-framework-os-kernel`'s `📦️glue.rs` (confirmed by grep — it's absent, and the crate's own header comment documents other modules as "unwired pending dep-DAG cleanup"). Same convention `🌎️hub/📇️directory`'s `SpaceRole` already uses for the identical reason (verified by reading that file).
- `DirectoryReadModel` carries one field beyond contract-freeze.md's literal C1 prose: a `users: Map<string, UserView>` side-table (both twins). This is what makes the decider-law phrase "`user.created` … only feeds member display data, no space change" concrete and testable — `user.created` fold is a no-op on `spaces`, but backfills `MemberView.email`/`displayName` when a later `member.upserted`/`invite.redeemed` adds that user. `spaces`/`cursor` match the contract exactly.
- `fold`'s idempotency guard is `event.seq <= model.cursor` (early return, no field touched) — the golden fixture's second-pass replay test asserts `fold_all(once, events) == once` in both languages.
- The archive decider law ("`archive-space` first emits `member.upserted{role: spectator}` for every current author, then `space.archived`") is exercised by the fixture itself (events 14–16), not hard-coded in `fold` — `fold` only ever applies one event at a time, staying a pure per-event function as the law requires.

## sharedFileRequest

**File:** `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/📋️contract-freeze.md`
**Region:** C1, the `space.created` event body and the `create-space` command body
**Change:** both bodies' space-kind field should read `spaceKind`, not the current prose's bare `kind`.
**Why:** both bodies are internally tagged on a discriminator that is *also* named `kind`
(`DirectoryEventBody`/`DirectoryCommand` both use `#[serde(tag = "kind")]`). A same-named payload
field collides with the tag on the wire — serde would try to deserialize the tag string
(`"space.created"`) into the payload's `DirectorySpaceKind` enum using the same JSON key. I renamed
the field to `spaceKind` in both twins (documented at the top of every file that touches it) to make
the schema actually compile/round-trip; any other lane implementing the hub's command handler or a
client against the contract's literal prose should use `spaceKind`, not `kind`, for this field.

## Commands run + results

### `cargo check -p semio-framework-os-kernel`
Clean — compiles with 9 pre-existing warnings, none from the new `os_directory`/`os_directory::schema`
modules (verified by grepping the second run's output for `📇️directory`/`error`: no matches). Full
tail:
```
warning: `semio-framework-os-kernel` (lib) generated 9 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 6 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 4.26s
```

### `cargo test -p semio-framework-os-kernel --lib directory`
First attempt hit a real bug (`DirectorySpaceRole` needed `Ord` for the test's `roles.sort()`) —
fixed by adding `PartialOrd, Ord` to its derive list in `🧬️schema/🦀️component.rs`. Second attempt hit
a transient `failed to move dependency graph … No such file or directory` — a concurrent-build
incremental-cache race (other lanes/sessions are building the same `target/` tree live, matches this
repo's known "concurrent cargo workspace churn" pattern), not a real error; retried and it passed
clean:
```
running 6 tests
test os_directory::schema::tests::command_kind_is_kebab_case ... ok
test os_directory::schema::tests::event_body_kind_is_the_dotted_wire_string ... ok
test os_directory::schema::tests::stream_message_kinds_round_trip ... ok
test os_directory::tests::folds_the_golden_fixture_into_the_expected_projection ... ok
test os_directory::tests::folding_is_idempotent_on_replay ... ok
test os_dsl::grammar::tests::backward_scan_and_jump_to_resolve_zip_eocd_and_central_directory_offset ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 954 filtered out; finished in 0.04s
```
(The 6th test, `os_dsl::grammar::…`, matched the `directory` substring filter incidentally — pre-
existing, unrelated, passing.)

### `bun nx run @semio-tech/framework-os:test`
Did **not** complete — not a code problem, an infrastructure one. `ps aux` showed the process count
for exactly this target climbing to **410+ concurrent `nx run @semio-tech/framework-os:test`
processes** while I waited (over 15 minutes). My own `tee`'d log (`🧪️0-a-nx-test.txt`) filled with 200+
repeats of nx's `> nx run …` / `$ bun nx run …` header and never once reached real vitest output —
consistent with nx's shared background daemon multiplexing *other* sessions' repeated invocations of
this same target into my client's terminal UI (this is a live repo with many concurrent lanes actively
editing files under `💻️os/`, per this ticket's own multi-lane structure). Proof it wasn't my own
process looping: after I terminated my own now-redundant client (`kill` on the PIDs my Bash call
actually spawned — not a shared/foreign process), **410 other `nx run` processes for this exact
target remained**, unaffected. I did not touch any of those (not mine to kill). Flagging this as an
infrastructure observation for the coordinator; `🧪️0-a-nx-test.txt` is left in the ticket folder as
evidence.

**Substitute verification** — I read `runVitest`/`runTestBudgeted` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` to see exactly
what `nx run @semio-tech/framework-os:test` shells out to (`node node_modules/vitest/vitest.mjs run
--config 🧪️vitest.config.ts --passWithNoTests …`, run once, not a retry loop), and ran that command
directly from `📦️packages/🟦️typescript`, bypassing the contended nx layer:

```
$ node node_modules/vitest/vitest.mjs run --config 🧪️vitest.config.ts --passWithNoTests
 ✓ |@semio-tech/framework-os| ../../🟦️component.ts > @semio-tech/framework-os directory > folds the golden fixture into the expected projection (parity with the Rust twin) 5ms
 ✓ |@semio-tech/framework-os| ../../🟦️component.ts > @semio-tech/framework-os directory > is idempotent on replay 0ms
 ❯ |@semio-tech/framework-os| ../../🟦️component.ts (157 tests | 1 failed) 69ms
     × matches the Rust plan_workflow across shared fixtures decoded via wasm 15ms
 FAIL … Error: Cannot find module 'file:///…/🖥️host/📦️packages/🦀️rust/pkg/semio_framework_os.js' imported from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️component.ts

 Test Files  2 failed | 2 passed (4)
      Tests  2 failed | 320 passed (322)
```

Both new directory tests pass (byte-identical Rust/TS fold parity over the golden fixture, plus the
idempotent-replay check). The only failure (×2, once per duplicated report line vitest prints) is a
**pre-existing, unrelated** test — `matches the Rust plan_workflow across shared fixtures decoded via
wasm` — failing because a wasm build artifact (`🖥️host/📦️packages/🦀️rust/pkg/semio_framework_os.js`)
does not exist in this checkout; that test's own file header says it needs `🖥️host`'s own `wasm`
target built first, which is unrelated to and outside this lease. All other 320 pre-existing tests in
this file (`AppChannelCodec`, `AppChannelClient`, `PluginGraph`, `InstanceDirectory`, …) still pass —
confirming my purely-additive `🔖️Directory` region did not regress anything.
`🧪️0-a-vitest-full.txt`/`🧪️0-a-vitest-direct-verbose.txt` in the ticket folder hold the full output.

## Blockers

- **Infrastructure, not code**: `bun nx run @semio-tech/framework-os:test` could not be run to
  completion due to 410+ concurrent processes on the same target saturating the machine (see above).
  Re-run it once that clears to get the "official" nx-wrapped result; the direct `vitest run` above is
  the real substitute evidence in the meantime.
- Nothing else. The Rust half is fully green via the exact specified commands.

## What is NOT done

- `bun ./📜️script.ts policy` was not run (not in this lane's verify list); the new schema dir was
  shaped to match `🎚️config/🧬️schema/` and `🗣️dsl/🧬️schema/`'s house style by inspection, and the
  taxonomy walk that checks leaf completeness (`policyComponentFileBreaches`) only recurses under
  each owner's `🗿️artifacts/` tree (confirmed by reading `📜️script.ts`), so it does not reach
  `🔨️modules/📇️directory/` at all — low risk, but unverified against the live policy command.
- The hub HTTP/WS surface (C2), client identity (C3), `s.space` artifact (C4), save/check-in (C5),
  and the command-flow wiring (C6) are explicitly other lanes' work — nothing here wires this schema
  into any live host/store, matching the same "unwired, pending" convention every other `🔨️modules/`
  schema in this crate currently uses.
- The `sharedFileRequest` above (renaming `kind`→`spaceKind` in contract-freeze.md's C1 prose) is
  unresolved; I did not edit contract-freeze.md myself (coordinator-only per the worker brief).
