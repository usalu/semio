# P2-FG3 Closer Report

Scope: close out wave FG3 (`gltf` 2.0, `pdf` 1.4+1.7, `ply` 1.0, `svg` 1.1 — 5 standards, 4 fan-out
agents, pdf's two standards handled by one combined agent per FG1's step+ifc4 precedent). Read all 4
fan-out reports (`p2-fg3-{gltf,pdf,ply,svg}-report.md`) and the independent verification
(`p2-fg3-verify-report.md`) in full before acting. This closer is the sole agent in the wave
authorized to touch `📦️glue.rs`, `📜️script.ts`, and the framework's `🧪️fixture-sweep` graduation list.

## 1. `glue_followup` items — none existed

None of the 4 fan-out reports requested a `glue_followup` item, and none even mention `📦️glue.rs`
except to state it was never touched (not even read-only, unlike FG2's dwg report). Grepped all 4
reports plus the verify report for `glue_followup`/`glue.rs` — nothing to apply.

## 2. Full crate gate — `cargo test -p semio-s-plugin-stdio --lib`

Run three times across this closer pass (before any edits, after the `📜️script.ts` allowlist edits,
and once more after the `🧪️fixture-sweep` graduation-list edit): **1806 passed, 0 failed, 1 ignored**
every time, no variance. Matches the independent verifier's own fresh count exactly. Covers all 22
prior standards + this wave's 5 (27 total) with zero failures anywhere. Raw capture of the final run:
`p2-fg3-closer-stdio-full-crate-final.txt`.

## 3. Policy gate — `bun run ./📜️script.ts policy`

Confirmed the same measurement artifact FG2's closer first diagnosed: the 5 PC-seeded rules
(`stdio-artifacts/{grammar-parseability,protocol-parseability,fixture-honesty,language-registration,
json-transfer-ban}`) are unconditionally `priority: "low"` in every one of their `breaches.push({...})`
call sites, and `runPolicyExit`'s own CLI path prints ONLY `priority: "high"` breaches — so this
command's own printed output is structurally incapable of showing shrink/growth for these 5 rules,
regardless of allowlist state. Confirmed unaffected by this wave's edits either way: **21509
high-priority breach(es) across 25 rule(s)**, byte-identical before (`p2-fg3-closer-policy-full.txt`)
and after (`p2-fg3-closer-policy-after.txt`) this closer's allowlist edits.

Verified the real signal instead by writing a retargeted copy of FG2's own direct-import scratch
script (`.🦑️repo/🎫️tickets/…/generators/policy_pc_breach_check_fg3.ts`, imports the exported
`policySchemaOverhaulPCBreaches` directly, bypassing the CLI's high-priority-only filter; kept per
the ticket's "don't delete scratch" rule):

- **Before**: 98 real repo-wide low-priority breaches, of which **40 touched exactly this wave's 5
  standards**, every one of the "allowlisted but the file is now genuinely real" (stale) shape:
  - `POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST`: 3 gltf + 6 pdf (1.4×3 + 1.7×3) + 3 ply + 3 svg = 15
  - `POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST`: same shape, 15
  - `POLICY_FIXTURE_HONESTY_ALLOWLIST`: 1 gltf + 1 pdf + 1 ply + 1 svg = 4 (artifact-level, one per
    artifact — pdf's single artifact-level entry covers both its standards)
  - `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST`: 1 gltf + 2 pdf (1.4, 1.7 separately) + 1 ply + 1 svg = 5
  - `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST`: 1 (svg's snapshot facet only)
  - Total: 15+15+4+5+1 = 40, matching exactly.
- Removed all 40 stale entries from `📜️script.ts`'s 5 `POLICY_*_ALLOWLIST` sets, scoped precisely to
  each artifact's own lines (never a global find/replace — verified each edit's surrounding context
  before applying, since several of these allowlist arrays are 100+ entries long and share substrings
  across artifacts).
- **After**: **58 breaches repo-wide, 0 touching FG3's 5 standards** — shrink confirmed for this
  wave, zero growth for anyone else. Raw captures: `p2-fg3-closer-pc-breach-check-before.txt`,
  `-after.txt` (98→59, one entry missed on the first pass), `-after2.txt` (59→58, final, 0 FG3 hits).

**One deliberate non-removal, documented rather than silently left as an oversight**: gltf's own
`POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` entry
(`stdio/gltf/standards#2.0-subsets-any-io-import-deserializers-artifacts-json-rfc8259-any-component`)
was left in place. Investigated why it never appeared as "stale" despite the gltf fan-out report's
own claim of having removed the real `serde_json::to_vec(` call from that file's live code (confirmed
true — `grep -rn serde_json` on the file's live code shows zero call-site hits). Read the file
directly: its own NEW doc comment, added by the fan-out agent to explain the fix, quotes the OLD code
verbatim inside backticks — `` /// … was a literal `serde_json::to_vec(&from.value)` JSON … `` — and
`policyStdioJsonTransferBanBreaches`'s `ioHit` check is a naive `content.includes("serde_json::to_vec(")`
substring match with no comment-stripping, so it still fires on the comment text alone. This is a
SECOND, different measurement-artifact class from FG2's finding (that one was a printed-output
filtering bug; this one is a detection-rule precision gap) — removing the allowlist entry would
introduce a real NEW `medium`-priority breach for a file whose actual transfer-path content is
already correctly `serde_json`-free. Left in place, documented here and in STATUS.md, flagged as a
follow-up for whichever pass next touches `policyStdioJsonTransferBanBreaches`'s own detection logic
(outside this closer's allowlist-only mandate for that rule).

## 4. Graduation — 8 tuples appended to `STDIO_CONFORMANCE_GRADUATED`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`
(`//#region 🔖️StdioTransition`). Confirmed exact directory-name identifiers by listing disk
(`ls … 🏅️standards`) rather than guessing: `gltf/2.0`, `pdf/1.4`, `pdf/1.7`, `ply/1.0`, `svg/1.1`.
Verified every grammar/protocol file is non-trivial and real by size (1.3–9.1KB each, not
stub-sized) before graduating anything.

Appended, `Grammar`+`ProtocolPack` for 4 of the 5 standards:

```rust
("🧊️gltf", "🔖️2.0", ConformanceFacet::Grammar),
("🧊️gltf", "🔖️2.0", ConformanceFacet::ProtocolPack),
("📄️pdf", "🔖️1.4", ConformanceFacet::Grammar),
("📄️pdf", "🔖️1.4", ConformanceFacet::ProtocolPack),
("☁️ply", "🔖️1.0", ConformanceFacet::Grammar),
("☁️ply", "🔖️1.0", ConformanceFacet::ProtocolPack),
("🎨️svg", "🔖️1.1", ConformanceFacet::Grammar),
("🎨️svg", "🔖️1.1", ConformanceFacet::ProtocolPack),
```

**`pdf`/`1.7` deliberately NOT graduated** — before graduating anything, I traced `pilot_resolve`'s
own `find_example_semio`/`derive_identity` (this file's `ExampleAssetDiscovery`/`PilotResolve`
regions) and confirmed a facet's fixture is resolved via `artifact_rel` ALONE
(`components[..=artifacts_idx + 1].join("/")` — the `🗿️artifacts/<artifact>` directory, standard name
dropped entirely). This is the EXACT SAME single-fixture-slot-per-artifact mechanism gif/89a hit in
FG2. Confirmed live, not assumed: read both pdf standards' own snapshot grammar files —
`artifact-mark = "stdio.pdf"` (1.4) vs. `artifact-mark = "stdio.pdf.1.7"` (1.7), two different
literal marks — then read both fixture locations: the shared ARTIFACT-level slot
(`📄️pdf/📚️examples/🎬️demo/🖼️assets/`) holds `semio stdio.pdf.dsl v1`-prefixed real 1.4-format PDF
bytes (matches 1.4's mark), while 1.7's own real fixture instead sits at its per-standard
`🏅️standards/🔖️1.7/📚️examples/🎬️demo/🖼️assets/` location (`semio stdio.pdf.1.7.dsl v1` preamble) —
a location this framework resolver never looks at.

Verified live rather than trusting this reasoning alone: staged the 8-tuple append (pdf/1.4 IN,
pdf/1.7 OUT) and ran `cargo test -p semio-framework-os-kernel --lib m5_handcrafted -- --nocapture`
BEFORE finalizing anything else. Output confirms exactly the predicted, safe outcome:
`[DEBUG] soft (stdio-exempt, pre-FG-wave) grammar conformance failure for 🗄️stdio::📄️pdf::🔖️1.7:
grammar did not recognize shipped fixture DSL body` — pdf/1.7 correctly stays soft (would have
hard-failed `m5_handcrafted_grammar_conformance` for real had it been graduated here). pdf/1.7's own
`⚙️engine::tests::conformance_laws::*`, using ITS OWN correct per-standard fixture, remain its real,
trustworthy, independent verification (157/0 pdf-scoped in the stdio crate, both per this closer's
own run and `p2-fg3-verify-report.md`) — not a pdf/1.7 content shortfall, the identical
`pilot_resolve` mechanism gap FG2 already documented and declined to fix (a per-standard-aware
fixture resolver is out of a closer's append-only mandate for this file).

**`ProtocolSpr` withheld for all 5** — direct `find … -iname "*.spr.semio"` under all 4 artifact
trees returned zero hits; none of the 5 standards shipped a real `.spr.semio` fixture this wave
(every fan-out report explicitly deferred it as optional/non-blocking) — graduating it would be
graduation theater per the recipe's own "nothing to verify" rule, same as every prior wave.

A precise doc comment was added directly above the appended tuples explaining the pdf/1.7 exception
in full (mirroring FG2's own gif/89a comment's level of detail) — see the file itself,
`//#region 🔖️StdioTransition`.

## 5. Framework m5 harness — `cargo test -p semio-framework-os-kernel --lib`

Run in two stages: scoped to `m5_handcrafted` immediately after the graduation-list edit (to catch
any hard failure before touching anything else), then the full `--lib` suite at the end.

**Scoped run** (`p2-fg3-closer-m5-scoped-test.txt`):
- Grammar: **59 facet(s) found, 59 checked, 0 soft-skipped, 28 stdio-exempt soft failure(s), 3 hard
  failure(s)**. The 3 hard failures are the SAME pre-existing non-stdio pilots FG1's/FG2's own
  closers already found red (`🏗️fem::◻2d::🔖️1`, `📕️norm::📘️en1992::🔖️1`, `🕸️dag::🕸️dag::🔖️1`) —
  confirmed by name in the panic output, count unchanged, none of them stdio, none touched by this
  wave. `🗄️stdio::📄️pdf::🔖️1.7` correctly appears among the 28 soft failures (expected, §4) —
  28 = FG2's own post-graduation baseline of 32 minus exactly 4 (this wave's 4 newly-graduated
  `Grammar` facets), the arithmetically expected shrink.
- Protocol: **118 facet(s) found, 29 checked, 89 soft-skipped, 2 stdio-exempt-or-known-gap soft
  failure(s), 0 hard failure(s)**. The 2 soft failures are the SAME pre-existing
  `📕️norm::📘️en1992::🔖️1` (magic mismatch, already in `KNOWN_NON_STDIO_GAPS`) and `🏗️ifc::🔖️2x3`
  (unrelated pre-existing stdio gap, predates this wave) as before — independently confirmed by
  reading the actual failure messages, not just the count. Zero soft or hard failures mention any
  FG3 artifact, meaning all 4 newly-graduated `ProtocolPack` facets pass silently for real.

**Full-crate run** (`p2-fg3-closer-m5-full-crate-test.txt`): **762 passed, 2 failed** —
byte-identical to FG1's/FG2's own post-graduation baseline, confirming zero regression anywhere in
the framework crate from this wave's graduation-list append + allowlist edits. Both failures are the
same 2 test functions (`m5_handcrafted_grammar_conformance`, `m5_production_coverage`) failing for
the same 3 pre-existing non-stdio pilots, confirmed by name in the panic output for both tests.

## 6. `git check-ignore -v`

`git status --porcelain` scoped to the 4 FG3 artifact trees shows 5 new (`??`) paths: 4 new
`🎒️example.pack.semio` files (gltf, pdf-artifact-level, ply, svg — plain untracked additions inside
already-tracked directories) and one new directory, `📄️pdf/🏅️standards/🔖️1.7/📚️examples/` (pdf/1.7's
own per-standard fixture folder, brand new this wave since 1.7 previously had no examples dir at
all). Ran `git check-ignore -v` on all 5 individually:
- The 4 files: exit 1 each (not ignored, no matching pattern at all — correctly plain untracked
  content inside tracked parent directories).
- The new `📄️pdf/🏅️standards/🔖️1.7/📚️examples/` directory: matches `.gitignore:179`'s own
  `!**/🔖️*/**` un-ignore rule (exit 0 with the negation pattern printed) — the SAME rule that
  explicitly whitelists everything under a `🔖️<version>/` standards directory, identical to FG2's
  own gif/87a and gif/89a `📚️examples/` directories. Cross-checked against plain `git status`
  (which only lists genuinely trackable paths by default, no `--ignored` flag used) — the directory
  appears there too, confirming it is real, trackable content, not accidentally-ignored debris.

All 5 will be picked up correctly by git.

## 7. Final re-verification

Re-ran `cargo test -p semio-s-plugin-stdio --lib` one more time after all `📜️script.ts`/
`🧪️fixture-sweep` edits were in place: **1806 passed, 0 failed, 1 ignored**, unchanged from §2's
first run — confirms the framework- and tooling-level edits in this closer pass caused zero
Rust-side regression (expected, since neither `📜️script.ts`'s TypeScript allowlist arrays nor a
`.rs` graduation-list append inside `🧪️fixture-sweep` are in the stdio crate's own compilation unit
or fixture-resolution path for its own `#[cfg(test)]` suite).

## 8. STATUS.md

Appended a full `## FG3 (fan-out wave, …) — CLOSED 2026-08-11` entry mirroring FG1's/FG2's own
entries' structure and level of detail, including the §3 second-measurement-artifact finding and the
§4 pdf/1.7 mechanism-gap finding (explicitly cross-referenced as "the same wall gif/89a hit, now hit
twice"). Did not remove or edit any prior content.

## Program tally after FG3

27 of 31 official stdio standards now have real grammar/protocol files, real fixtures, and
`Grammar`+`ProtocolPack` graduated: 22 from before this wave (6 PC pilots + 7 FG1 + 9 FG2, with
gif/89a real-but-ungraduated) plus this wave's 4 newly graduated (gltf/2.0, pdf/1.4, ply/1.0,
svg/1.1). pdf/1.7 is real and independently tested but not framework-graduated (§4) — the SAME
`pilot_resolve` mechanism gap as gif/89a, now hit a second time. `ProtocolSpr` remains graduated for
only csv/txt (unchanged this wave). 4 standards remain for future FG-waves per the plan's roster.

## Known follow-up, not fixed this wave

1. **pdf/1.7's `Grammar`/`ProtocolPack` graduation is blocked by the `pilot_resolve`
   single-fixture-slot-per-artifact mechanism gap** (§4) — the real fix is a per-standard-aware
   fixture resolver in `🧪️fixture-sweep/🦀️component.rs` (teach `pilot_resolve` to prefer a fixture
   slug whose name matches the facet's own standard id when more than one candidate location
   exists), out of a closer's append-only mandate for that file. This is now the SECOND
   multi-standard artifact to hit this exact wall (`gif`, now `pdf`) — worth prioritizing the real
   fix once a third case appears, or sooner, rather than accumulating permanently-ungraduated
   standards that are otherwise fully real and tested.
2. **gltf's `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` entry stays in place**, masking a live
   false-positive `serde_json::to_vec(` substring hit inside its own doc comment (§3) — not a
   content gap (the transfer path is genuinely `serde_json`-free), a policy-rule precision gap (the
   rule's substring match doesn't strip comments first). The correct minimal fix is either
   rewording the doc comment to avoid the literal call-shaped substring, or teaching
   `policyStdioJsonTransferBanBreaches` to strip `///`/`/* */` comments before matching — outside
   this closer's allowlist-only mandate for that rule's own detection logic.
3. **FG1's own 7 standards remain present in all 5 `POLICY_*_ALLOWLIST` sets** (FG2's own closer
   already flagged this as a leftover from FG1's own closer pass, unchanged since) — still out of
   this wave's scope to retroactively fix.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs` — 8-tuple graduation
  append + explanatory doc comment (§4). Only edit in this file.
- `📜️script.ts` — 40 stale entries removed across `POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST` (15),
  `POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST` (15), `POLICY_FIXTURE_HONESTY_ALLOWLIST` (4),
  `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST` (5), `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` (1 — svg
  only; gltf's entry deliberately kept, documented in §3). No other part of this file touched.
- `.🦑️repo/🎫️tickets/…/STATUS.md` — appended FG3 closed-wave entry.
- `.🦑️repo/🎫️tickets/…/generators/policy_pc_breach_check_fg3.ts` (new, kept — ticket scratch script,
  §3, retargeted copy of FG2's own).
- `.🦑️repo/🎫️tickets/…/p2-fg3-closer-policy-full.txt`, `p2-fg3-closer-policy-after.txt`,
  `p2-fg3-closer-pc-breach-check-before.txt`, `-after.txt`, `-after2.txt`,
  `p2-fg3-closer-m5-scoped-test.txt`, `p2-fg3-closer-m5-full-crate-test.txt`,
  `p2-fg3-closer-stdio-full-crate-final.txt` (new — raw command captures).
- `.🦑️repo/🎫️tickets/…/p2-fg3-closer-report.md` (new — this file).

No `glue.rs`, and no artifact-owned `.rs`/`.grammar.semio`/`.protocol.semio` file was touched by this
closer. Ticket left open for the orchestrator/next wave.
