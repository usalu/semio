# CW9 — Final campaign verification and close

This is the closing wave of the entire db+protocol+vcs-slimming campaign. CW0-CW8 are all complete
and independently verified: the rename (protocol→playbook), the full 12-crate `protocol` family
(core/command/causal/crdt/format/history/materialize/io/wire, facade, testkit, cli), the kernel
cut-over (vcs slimmed, framework/core extracted, dsl_derive flipped), the full 24-crate `db` server
family, wire v2 client integration in framework/sync, both hub rebuilds (os-hub + compose-hub) on
`db`, the full app fan-out (CollectionOperation shape migration + broader Operation/OpText import
sweep across ~50 crates + kernel wave + vcs-plugin + preview-law staging for 5 gesture apps), and
shim removal + 3 new policy lints.

Read `/Users/ueli/.claude/plans/introduce-a-new-technology-cuddly-rabbit.md`'s "End-to-end
verification" section (near the bottom) and the CW9 row in its wave table for the full checklist.

## Checklist

1. **`cargo build --workspace`** — must be clean. If it isn't, investigate before anything else;
   this is a hard blocker (a green tree is the whole point of every prior wave).
2. **`cargo clippy` on the campaign's touched surface** — the full `protocol/*` family (12 crates),
   the full `db/*` family (24 crates), `vcs`, `semio-framework-core`, `semio-framework-sync`,
   `semio-framework-plugin`, `os-hub`, `compose-hub`, and a representative sample of the ~50
   app crates touched in CW7 (don't need literally all ~50, but cover at least one from each of
   the CW7 sweep groups — check the ticket's `cw7-sweep-{a,b,c}.txt`/`cw7-kernel-wave.txt` reports
   for the exact crate lists). Report warnings; only real errors (`-D warnings` violations if the
   workspace lints are configured that strictly — check root `Cargo.toml`'s `[workspace.lints]`)
   are blocking, pre-existing style warnings are not.
3. **Leveled test tiers**: run `test-quick` for the whole touched surface (should already be
   covered by plain `cargo test`, which defaults to the "fundamental" unscoped tier per the repo's
   test-level convention). Then run the `long` and `exhaustive` tiers specifically for
   `protocol_testkit`, `pack_testkit`, `db_testkit` (these are the crates with the crash-harness/
   corruption-fuzz/permutation-exhaustive test suites — check each for `mod long`/`mod exhaustive`
   submodules and run with the repo's leveled-test mechanism, e.g. `SEMIO_TEST_LEVEL=exhaustive
   cargo test -p protocol_testkit -p db_testkit` or whatever `runCargoTestBudgeted`'s actual env
   var/invocation convention is — check `repo/lib/js/index.ts` if unsure).
4. **`bun ./📜️script.ts lint`** (or the specific `verify`/`gate` target if one exists — check root
   `package.json`/`script.ts` for the exact command name) — confirm the three new CW8 policy lints
   fire correctly (zero breaches for `policyProtocolMigrationBreaches`/`policyDbServerOnlyBreaches`,
   a real allowlist for `policyCommandEnvelopeCompletenessBreaches`) and that no OTHER policy
   regressed because of this campaign's changes.
5. **e2e / hub integration tests**: `os-hub` and `compose-hub` each have their own test suites
   (already verified passing per CW6). If a true end-to-end smoke test is feasible (start a hub,
   connect via the wire-v2 protocol, submit a command, observe it land) without needing external
   infra (postgres/neo4j/docker), run one and report the result. If it needs infra you don't have
   locally, say so plainly rather than skipping silently.
6. **Storybook/dev builds** (if quickly checkable): the plan mentions "storybook/dev builds
   unbroken" — if there's a fast typecheck/build target for the TS/frontend side, run it; note if
   it's too slow/out of scope for this wave.

## Human todo list to compile (do not attempt to fix these yourself — just collect them)

Gather every deferred/follow-up item flagged across all prior waves' reports in this ticket folder
(read every `cw*-report.txt`/`cw*.txt` file present) plus the earlier rename ticket's flag. At
minimum, these are already known:
- `playbook/AGENTS.md` content still says `technology: protocol` and needs a human rewrite
  (AGENTS.md files cannot be edited by agents).
- A new `protocol/AGENTS.md` and `db/AGENTS.md` are owner-authored (don't exist yet, agents can't
  create technology AGENTS.md files per repo convention — verify this assumption by checking if
  other technologies' AGENTS.md were in fact created by a prior agent wave; if agents CAN create
  a NEW AGENTS.md (as opposed to editing an existing one), note that distinction and either create
  minimal ones or flag clearly why not).
- The real preview-law wire-up needs a `vcs::BackboneMessage::Preview` variant + WIT
  `host_backbone_send`/`host_backbone_poll` changes + a host-side relay — CW7's preview-law agent
  staged the exact seam in all 5 gesture apps but correctly stopped short since this touches
  critical files (`vcs/rs/lib.rs`, WIT, `framework/plugin/host/rs`) outside its scope.
- compose's real client-side wire-v2 network wiring (`WebsocketBackbone`/`login(hub_url)`) — CW6
  found it was already a pre-existing stub with no working runtime caller; deferred to a dedicated
  future ticket once compose actually needs a live client↔hub connection.
- The pre-existing icon-catalogue gaps (missing SVGs, `ui_wgpu` `IconName` typegen derive) flagged
  during the original rename wave — unrelated to this campaign, already filed separately.
- Any others you find in the wave reports.

## Close

Once the checklist is green (or every non-green item is a clearly-understood, already-known
pre-existing/deferred issue, not a regression from this campaign), close the ticket:

`ticket_close` with explicit path `26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING`,
a summary covering the full campaign (reference CW0-CW9 briefly), and the `files` list — this will
be large; a reasonable approach is one representative path per crate family plus every shared
critical file touched (root Cargo.toml, vcs/rs/lib.rs, framework/core/rs/lib.rs,
dsl/derive/rs/lib.rs, framework/plugin/rs/lib.rs, framework/sync/rs/lib.rs, root script.ts,
.vscode/launch.json) rather than literally every one of the ~100+ files touched across 9 waves.

Also, since the earlier `PROTOCOL-BINARY-OP-LOG-LAYER` ticket was closed with a note that it was
merged into this one, no action needed there — just reference it in this ticket's closing summary
for continuity.

## Report

Write `.repo/🎫️/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw9-final-report.txt`
with the full checklist results and the compiled human-todo list, before calling `ticket_close`.
