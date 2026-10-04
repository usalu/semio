# 🛟️ Report — Quiz End to End Fault Tolerance (2026-10-03)

## What was broken

The reported crashes (`QuizSession.rehint` reading `hints` of `undefined`, `RunScreen` → `keysHidden` reading `keys` of
`undefined`) came from a **stale proctor**: the dev stack on 6061/8791 had been started on 2026-10-02 22:55, before the
challenge levels landed. Its `GET /instance` listed no `quiz.open-task`, and every quiz kind at wire version 1 — the
version the client spoke as well, because the challenge contract had changed commands, events and views without raising
it. The client sent `start-run` with a challenge, the old proctor accepted it and answered run views whose sheet had no
`challenge`; `challengeRules(undefined)` is `undefined`.

Validated root causes and the holes behind them:

| Hole | Consequence |
|---|---|
| The wire version was never raised with the contract | proctor and client of different contracts took each other's views for their own |
| Proctor answers were adopted unchecked (`JSON.parse(...) as V`) | a view of another shape crashed the session and the run screen |
| The outbox settles every non-transient delivery failure as `refused` and drops the command | an envelope mismatch, an unknown command kind or an unreadable answer (a network's sign-in page) would have dropped queued answers |
| `PresenceRoom.stop()` closed sockets still connecting | Chromium logged "WebSocket is closed before the connection is established" on every room change |
| No error boundary | one throwing screen blanked the whole client |
| `dev` kept running the proctor it built while its Rust sources changed, and reused any proctor that answered | the stale proctor of the report; nothing told the developer |
| Home cards sized through `.quiz-home-cell > *`, which since 10-02 is a `display: contents` host | every overview card collapsed to its title (two `🥞️layered-home` specs red; every spec depending on the `desktop` project — presence, shortage, away — did not run) |
| Vite's WebSocket proxy calls `socket.destroySoon()` when its upstream ends; Bun 1.3's upgrade sockets have none (probe `upgrade_socket_probe.ts`: 1.3.13 `undefined`, 1.4.2 `function`) | the whole dev server died the moment the proctor went away while a socket was proxied — found live on the beside stack |

## What changed

**Contract.** `$defs/WireVersion` (`const 2`) in `🧬️schema/🔣️.json`, `WIRE_VERSION` in both twins. The proctor
(`🎭️actors`, `❓️queries`, `🧩️instance`) and the client use it; the proctor's own constant is gone. New language-agnostic
case `🧪️tests/🤝️wire-version` (Python oracle, TS and Rust subjects, fixture `🧫️fixtures/🤝️wire-version`): every core
projects the schema's version and FNV-1a 64 of the schema without its prose; the oracle fails unless that pair is the
committed one, so a contract change without a raise cannot pass (mutation-checked: a stale fingerprint → parity 1/3).

**Client agreement** (`🎯️targets/⚛️react/🔨️modules/🛂️proctor`). Before its first call and after any refusal the
`ProctorClient` reads `GET /instance` (one shared question for concurrent callers); `disagreement(definition)` says
`newer`/`older`/`foreign` unless every quiz kind the client sends (`QUIZ_KINDS`, exhaustive by type) is declared at
`WIRE_VERSION`. An incompatible proctor is sent nothing, is asked again after `AGREEMENT_RECHECK_MS` (30 s), and throws
`ProctorIncompatible` — a `ProctorUnavailable`, so the outbox keeps waiting and the deputy decides. Reachability gained
`incompatible`; `QuizConnection.contract` names it. Answers that cannot be read (no JSON, a view of another shape —
`isCatalogView`, `isLearnerView`, `isRunView`, now in `💾️persistence` and shared with the restore of stored views) are a
shortage after the agreement is checked again, never a crash and never a refusal. `quizInstance(version)` describes an
agreeing proctor (tests and fakes).

**UI.** Older/foreign proctor: the connection reads like an unreachable one (everything saved on the device). Newer:
"Page out of date – everything is kept on this device until you reload it" in the navbar and a banner with *Reload page*
(EN/DE, *du*). `ScreenBoundary` around every screen: a throwing screen shows *Something went wrong while showing this
screen. Everything you did is kept.* with *Try again now* and *Reload page*.

**Presence.** A socket left while connecting is closed once it opens (`leave`), and a join that is abandoned settles.

**Dev stack** (`🧱️stack/🟦️.ts`, `🛂️proctor/🏗️bootstrap/🟦️.ts`). `SupervisedProctor`: rebuilds and swaps the proctor
when a Rust source of a crate it is built from changes (`proctorSourceDirectories` from `cargo metadata --offline`;
`proctorSourceChanged`), relaunches it when the catalog or a quiz changes (`catalogFiles`), relaunches it after a crash
(`relaunchDelay` 1 s doubling to 30 s); a reused proctor of another contract is announced with `[WARN]`. Framework Vite
plugin `semioServeUpgradeVitePlugin` gives upgrade sockets Node's `destroySoon` (`destroySoon`, held to Node's own), used
by the site.

**Overview cards.** The two `🥞️layered-home` specs red since 2026-10-02 were a real bug: `LayeredOverview` (checkpoint
668) hosts every card in a `display: contents` element, so `.quiz-home-cell > * { width: 100% }` sized a box-less host
and every card — whose content measures its own room as a size container — collapsed to its title: tall, narrow cards
cut off at the bottom of their cells. The width now goes to the card (`[data-overview-card]`) in the grid and the list.

**E2E.** New spec `🧪️tests/🛟️proctor-faults` (project `faults`, after `away`): the proctor dies between every two tasks
of an expert run and in the moment of the submission; a proctor of an older contract is sent nothing while the device
decides a whole run; a newer one shows the banner; once they agree everything arrives and a fresh device finds it. The
driver no longer tolerates "WebSocket is closed before the connection is established".

## Evidence

| Gate | Result |
|---|---|
| quiz React suite | 839/839 (6 new contract tests, error boundary, presence teardown) |
| quiz core TS / Rust | 415/415 / 177 passed |
| parity, quiz owner | 132/132 (new case `🤝️wire-version` 3/3; its guard mutation-checked) |
| proctor Rust | 90 unit, 15 conformance, 21 end-to-end (now asserting the `/instance` versions) |
| site unit (incl. supervision, upgrade sockets) | 234/234; the upgrade test fails without the plugin with exactly the reported `TypeError` |
| typecheck | quiz React and site clean |
| e2e, both topologies | every project but `pets` green: dev 55 passed, rehearsal 54 passed, including `🥞️layered-home`, presence, shortage, away and the new `🛟️proctor-faults`. `🐕️pet-walk` (dev 2, rehearsal 3 failed) assumes the collapsed-card geometry; the owning session (QUIZ-PETS, active) accepted the card fix and moves its ladder/rope proofs to the quiz pages |
| taxonomy | quiz clean; teaching has 18 errors, all at the teaching workspace root (Cargo.toml, locks, `.config`), predating this ticket |

- Live, beside stack (Bun 1.3.13): proctor killed → `[stack] the proctor ended by itself with status 255; launching it
  again in 1 s` → ready after 246 ms, the site survived (before the plugin the dev server died with
  `socket.destroySoon is not a function`); a proctor source touched → `building it anew while the running one serves` →
  ready after 262 ms.
- Live, the developer's own stack (6061 with the stale proctor 8791 still running): a fresh device made exactly one
  `GET /instance` and no command or query, the header said *Quiz-Server nicht erreichbar – alles wird auf diesem Gerät
  gespeichert*, a run started on the device and its screen rendered three tasks — no crash, no console error.

## Open

- **Restart the running dev stack once.** The process on 6061/8791 predates the supervisor; its site already serves the
  new client (which keeps everything on the device against that proctor). Stopping it and starting the launch row again
  builds the current proctor; from then on `dev` rebuilds it by itself.
- **Bun 1.3.13 does not proxy WebSockets through Vite** (a socket to `ws://127.0.0.1:6063/scopes/…/presence/ws` stays
  connecting while the same socket to the proctor directly is welcomed). The Claude preview launcher resolves `bun` to
  `~/.bun/bin/bun.exe` 1.3.13; the developer's shell and the gate use 1.4.2, where presence works. Presence is decorative
  and its absence degrades nothing else.
- **A proctor that lost its data** (restored from an older backup, a dev data folder moved aside) still makes a device
  forget a learner it does not know (existing rule, needed for erasure on request). Re-enrolling such a learner would
  need the proctor to tell erasure from loss.
