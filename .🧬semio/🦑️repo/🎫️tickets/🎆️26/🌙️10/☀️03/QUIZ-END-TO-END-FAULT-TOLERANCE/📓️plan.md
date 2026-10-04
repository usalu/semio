# 🛟️ Plan — Quiz End to End Fault Tolerance

## Diagnosis (validated 2026-10-03)

- The dev proctor on 8791 was started 2026-10-02 22:55 by `dev` (site 6061 from the same run) — before the challenge
  levels landed. `GET /instance` lists no `quiz.open-task`, and every quiz kind at version 1.
- The client also speaks version 1: the challenge contract changed commands, events and views without raising the wire
  version. The stale proctor accepted `start-run` and answered run views whose sheet has no `challenge`;
  `challengeRules(undefined)` is `undefined` → `QuizSession.rehint` and `keysHidden` (RunScreen) throw.
- Proctor answers are never checked against the contract (`JSON.parse(...) as V`).
- The outbox turns every non-transient delivery failure into a `refused` settlement and drops the command — an
  incompatible proctor (envelope mismatch, unknown kind) or an unreadable answer (a proxy's HTML page with 200) would
  lose queued answers.
- `PresenceRoom.stop()` closes sockets still in CONNECTING → Chrome logs "WebSocket is closed before the connection is
  established".
- No error boundary: one throwing screen blanks the whole site.
- `dev` keeps a proctor it started running while its Rust sources change, and reuses any proctor that answers.

## Work

1. Contract: `WireVersion` in `🧬️schema/🔣️.json` (TS + Rust twins), raised to 2; a language-agnostic case pins the
   contract fingerprint (schema without prose) to the wire version, so a contract change without a raise fails.
2. Proctor: speaks the schema's wire version (no own constant).
3. Client `ProctorClient`: agreement with the proctor through `GET /instance` before the first call and after any
   refusal; reachability `incompatible`; `ProctorIncompatible` (transient: outbox keeps waiting, the deputy decides);
   unreadable answers transient.
4. UI: connection status for an incompatible proctor (EN/DE: page out of date → reload; proctor older → kept on
   device); an error boundary per screen with retry.
5. Presence: a socket left while connecting closes once open.
6. `dev`: refuses to silently reuse an incompatible proctor (says so), rebuilds and restarts the proctor it owns when
   its sources change (off with `TEACHING_ARCHITECTURE_QUIZ_WATCH=off`).
7. Tests: unit + journey (FakeProctor incompatible, unreadable), e2e specs: proctor away mid-run and during submit,
   proctor restart while answering, incompatible proctor, expert clock while away; full gates.
