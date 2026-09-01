# CAD Rust build/check + mutate-cad-1 oracle verification

Session context: `semio-s-plugin-stdio` was under active concurrent-session refactor the entire time
this ticket ran (791 modified/deleted paths at the start of this pass, 998 by the end — actively
growing, confirming an in-progress rewrite, not a stale/abandoned state). Per standing instruction,
`semio-s-plugin-stdio` was NOT touched or fixed. Everything below works around that dependency.

## 1. `cargo build -p semio-s-plugin-cad --keep-going`

Command run verbatim from repo root, output tail captured to `🗑️generated/build-cad-1.txt`.

**Result: never completed in this session.** The process sat at
`Blocking waiting for file lock on build directory` for the entire session (confirmed at multiple
checkpoints, minutes apart, with `git status` showing the stdio churn count still climbing each
time — 791 → 810 → 850 → 858 → 998 modified paths). This matches the documented repo pattern
(`project-concurrent-cargo-workspace-churn`): a concurrent session holds the cargo target-dir lock
for an extended, still-unfinished rewrite. No cargo output beyond the lock-wait line was ever
produced, so **zero errors were observed for `semio-s-plugin-cad` in this session** — but that is
because the build never got far enough to compile the crate's own sources at all, not because a
completed build was clean. This is an honest non-result, not a pass.

## 2. `cargo check -p semio-s-plugin-cad --keep-going`

Same story: launched from repo root, output to `🗑️generated/check-cad-1.txt`. Since
`semio-s-plugin-cad` depends on `semio-s-plugin-stdio` (`✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml:56`),
`cargo check` must also compile (or at minimum metadata-resolve through) the same dependency graph
and contends for the identical target-dir lock. **Result: also blocked on the same lock for the
entire session — no type-checking of the cad crate's own compilation units was ever observed.**
Reporting honestly: this session could NOT establish whether the cad crate's own code type-checks
cleanly; the dependency lock blocked even that. (The committed oracle registration
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` records an
EARLIER pass's claim that "this subset's own plugin crate compiles clean at `cargo check --lib`" —
that is a prior session's claim from its own rationale text, not something re-verified here.)

## 3. `semio-s-plugin-stdio` attribution + poll

Confirmed by direct build attempt (§1): the crate never got far enough to attribute any error to
`semio-s-plugin-cad` itself, because the blocking dependency never finished compiling. Polled at
four points across the session; `git status --porcelain | grep -c stdio` went 791 → 810 → 850 → 858
→ 998, and the build/check processes stayed on the identical `Blocking waiting for file lock on
build directory` line throughout — a live, still-growing in-progress refactor, not a stall. Per
instruction, `semio-s-plugin-stdio` was left untouched.

Because the lock never released in this session, task 3's fallback
(`bun nx run @semio-tech/cad-plugin:test-long`) was never reached — running it against a still-locked,
possibly half-renamed dependency tree would produce noise, not signal.

## 4. `mutate-cad-1` phases

Nx project: `test-s-plugins-cad-artifacts-cad-9be55e-mutate-cad-1`
(root: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🧪️tests/mutate-cad-1`).

### `contract` — ran to completion, exit 1 (repo-wide gate, not cad-scoped)

```
cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
bun ./📜️script.ts contract --owner "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad" --case mutate-cad-1
```

This phase is NOT actually scoped by `--owner`/`--case`: `capabilityManifestBreaches` and
`binaryProtocolDriftBreaches` (in the TS test-platform package) iterate `registry.contributions` for
every owner in the repo regardless of the CLI selector, so the reported breach count is repo-wide.
**Before my fixes: 1857 high-priority breaches. After: 1856** (net -1, matching exactly the one CAD
defect fixed below). The command still exits 1 — it will keep failing until the repo-wide gaps
(109 owners missing a v2 `mutationManifests` entry, near-universal `binaryProtocolDriftBreaches`
"missing" counts, etc.) are cleared elsewhere; none of that is cad-specific or in this ticket's scope.

Two real, CAD-attributed findings surfaced and were triaged:

- **FIXED** — `🧬️mutations/💾️binary/📡️component.protocol.semio` (the handcrafted binary wire
  protocol for `CadMutation`, embedded via `include_str!` into the production `cad.spr`/`cad.op`
  `LanguageSpec` registrations in `✏️s/🔌️plugins/📐️cad/🦀️component.rs`) still described the
  PRE-migration 14-verb object vocabulary (`add-object`, `remove-object`, `patch-object`,
  `translate-objects`, `rotate-objects`, `scale-objects`, `set-pane-objects`, `add-node`,
  `remove-node`, `patch-reference`, `set-references`, `set-active-model-definition`, `set-snapshot`)
  that ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3 explicitly retired (see the
  docstring at the top of `🧬️mutations/🦀️component.rs`). The contract check flagged this as
  "19 mutation kind(s) have no wire record and 13 record(s) name a kind that no longer exists" — the
  ONLY owner in the whole repo with a nonzero orphan count (every other owner's protocol file either
  has zero orphans or, in several cases, uses a record-name spelling/`tag=`-punctuation the checker's
  regex cannot even parse, e.g. `gismap`'s PascalCase `CreatePosition`/`tag 1` — which is why the CAD
  file is unusually well-instrumented for this exact gate). Rewrote the file to declare one
  `record <kind> tag=<1..20>` per current `CadMutation::KINDS` entry, with field lists taken directly
  from each mutation's own Rust payload struct (e.g. `create-shape-model`: `child-id utf8`,
  `target utf8`; `move-reference`: adds `new-origin array f64`; `replace-reference-media`'s three
  `Option<_>` fields encoded as `bytes`, consistent with how the file already treated nested/complex
  fields). Re-ran `contract` after the fix: the cad binary-protocol-drift line is now GONE entirely
  (0 missing, 0 orphaned) — confirmed by diffing the two full contract runs
  (1857 -> 1856 total repo-wide breaches, net -1, exactly this one line; raw run output was
  inspected and then deleted per ticket housekeeping). Note: this `.semio`
  text is descriptive/registered, not literally parsed at runtime — the real `encode_op`/`decode_op`
  come from `dsl::DslOps`'s derive-generated `OpBinary` on the Rust types directly
  (`🧬️mutations/💾️binary/🦀️component.rs`) — so this fix corrects the schema-first documentation
  surface the contract gate audits, not runtime behaviour (which was already correct and already
  covered by its own round-trip unit test).
- **FIXED (same defect, sibling surface, no lint covers it)** —
  `🧬️mutations/📝️text/📖️component.grammar.semio` (the `cad.op` text grammar, same `include_str!`
  wiring pattern, same retired 14-verb vocabulary, same production comment confirms the retirement:
  "The pinned pre-migration byte fixture retired with the generic `Patch*`/`Set*` variants it
  exercised (SEMANTIC-MUTATIONS-OVERHAUL, 26/08/12)" in `🧬️mutations/📝️text/🦀️component.rs`).
  Rewrote the `mutation`/`op-line` productions to the current 20 kinds with field shapes matching each
  payload struct (`node-block` reused verbatim — `CadNode{id,label,kind}` never changed;
  `reference-row` rewritten to include all nine `CadReference` fields, including `orientation`, which
  the old row omitted). No automated gate currently checks this file's freshness (unlike the binary
  file, the drift-detector regex only reads `💾️binary/📡️component.protocol.semio`), so this fix is
  schema-first hygiene, not a gate-driven fix — flagged for transparency in case a future gate is
  added here.
- **NOT FIXED, explicitly out of scope** — `cad-1-any`'s oracle registration carries a v1
  `mutationCatalogs` entry but no v2 `mutationManifests` entry, so
  `capabilityManifestBreaches` fires ("Catalog cad-1-any declares capability cad-1-mutate (20
  kind(s)) and no mutation manifest owns it"). Checked: this affects **109 owners repo-wide**
  (`writer-1-any`, `procedural-2d-1-any`, `gismap-1-any`, `vcs-1-any`, … — grep count against the
  full contract output), i.e. this is a repository-wide, in-progress protocol-v2 migration gap, not
  a CAD-specific regression. Per the ticket's own scope ("fix only what is genuinely broken inside
  the CAD plugin's own... assets") and because fixing one of 109 owners moves no release-gate
  denominator (per the check's own doc comment), this was left alone and is reported, not patched.
- **NOT FIXED, explicitly out of scope** — the same repo-wide `contract` run also flags CAD's
  `🚪️io/📤️export/🧵️serializers/🗿️artifacts/{stl,gltf,dwg,png,step,obj,ifc}/…/🦀️component.rs` as each
  emitting "the artifact's internal DSL text, not `<format>`" — i.e. every real-format export
  serializer is a stub. This is unrelated to `mutate-cad-1` (a separate `🚪️io` capability), a much
  larger implementation task (seven real file-format writers), and outside this ticket's remit;
  reported for visibility only.

### `oracle` (pure Python, independent of the cargo lock) — ran to completion, PASSED

Default level is `fundamental`, which this case's tags (`@level-exhaustive`/`@level-long`) don't
match — `bun nx run ...:test-oracle` alone reports `not-exercised` (see
`🗑️generated/mutate-cad-1-oracle.txt`), which is a real but misleading-if-quoted-alone result. Ran
explicitly at the case's own level:

```
cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
bun ./📜️script.ts oracle exhaustive --owner "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad" --case mutate-cad-1
```

Output (`🗑️generated/mutate-cad-1-oracle-exhaustive.txt`):

```
[test] level=exhaustive cases=1 executed=41 passed=41 failed=0 errored=0 parity=0/0
EXIT:0
```

41 = 20×`mutate-<kind>` + 20×`inverse-<kind>` + 1×`identity-round-trip`. **All 41 pass.** This is the
Python second-implementation oracle (`🐍️.py`) applying and inverting all 20 mutation kinds against
the 20 committed vectors and independently re-deriving the same after/before snapshots — genuine
signal, unaffected by the stdio lock since it never touches Rust/cargo.

### `subject` — did not complete in this session (blocked on the same cargo lock)

```
cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
bun ./📜️script.ts subject exhaustive --owner "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad" --case mutate-cad-1
```

Launched and left running; CPU time stopped advancing (frozen at ~7s) consistent with the process
blocking on the shared cargo target-dir lock while compiling the generated Rust test host (the
`🦀️.rs` adapter here explicitly does not link the cad plugin crate, but the generated host itself
still needs to compile, and shares the lock). This matches the oracle registration's own prior-pass
note: a `parity` probe was previously killed by the runner's 900s per-case budget for exactly this
reason (`spawnSync cargo ETIMEDOUT`). Did not obtain a subject or parity result in this session.

### `parity` — not attempted

Would require the same blocked compile step as `subject`; not run, per the above.

## Test-vector / schema audit (no cargo needed)

Verified by direct inspection, no drift found:

- **20 kinds, consistently spelled and ordered**, agree across all four places that declare them:
  Rust `CadMutation::KINDS` (`🧬️mutations/🦀️component.rs`), the Python oracle's `KINDS` tuple
  (`🧪️tests/mutate-cad-1/🐍️.py`), the feature file's two `Examples` tables (20 rows each,
  `🧪️tests/mutate-cad-1/🥒️.feature`), and the oracle registration's `mutationCatalogs[0].kinds`
  (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`).
- **"100 handcrafted vectors" reconciled**: there are 20 vectors (one per kind), not 100 — the "100"
  in the feature file's own docstring ("all 100 of its fixtures are handcrafted specification
  vectors") counts FILES, not vectors: `find … -path '*/🧪️tests/*' -type f` under
  `🧬️mutations/*/🧪️tests/*/` returns 120 files (20 vectors × 6 files each: `⬅️before`, `🦠️mutation`,
  `🔺️diff`, `🎯️outcome`, `➡️after`, plus one per-vector production `🦀️component.rs` unit test not
  counted in the "100 fixtures" claim, which covers only the five specification JSON files). No
  discrepancy — just a file-vs-vector counting distinction worth stating plainly.
- The two non-kind directories under `🧬️mutations/` (`💾️binary`, `📝️text`) are shared wire-format
  schema components, not mutation kinds — correctly excluded from the 20-kind count.

## Files touched

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio`
  — rewritten to the current 20-kind vocabulary (fixes a real, gate-flagged defect).
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
  — rewritten to match (same defect class, no gate currently covers it).

No Rust source was changed; no test was weakened or skipped.
