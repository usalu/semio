# P2-FG2 Closer Report

Scope: close out wave FG2 (`gif` 87a+89a, `jpg` jfif-1.01, `bmp` v3, `tiff` 6.0, `deflate` rfc1950,
`las` 1.0, `dwg` ac1018+ac1024 — 9 standards, 7 fan-out agents). Read all 7 fan-out reports
(`p2-fg2-{bmp,deflate,dwg,gif,jpg,las,tiff}-report.md`) and the independent verification
(`p2-fg2-verify-report.md`) in full before acting. This closer is the sole agent in the wave
authorized to touch `📦️glue.rs`, `📜️script.ts`, and the framework's `🧪️fixture-sweep` graduation
list.

## 1. `glue_followup` items — none existed

None of the 7 fan-out reports requested a `glue_followup` item. `📦️glue.rs` appears in the dwg
report only, read-only: it documents (a) the pre-existing ac1018→ac1024 "default standard switched"
shim comment (used to explain why ac1018's own `⚙️engine` needed a fully-qualified-path fix, done
entirely inside dwg's own file, never in `glue.rs`), and (b) that ac1018's own `register()` is dead
code (never invoked from the real plugin bootstrap). Nothing to apply.

## 2. Full crate gate — `cargo test -p semio-s-plugin-stdio --lib`

Fresh run, no filter, run twice (before and after this closer's `🧪️fixture-sweep`/`📜️script.ts`
edits, to confirm no cross-crate side effect): **1773 passed, 0 failed, 1 ignored** both times.
Matches the independent verifier's own fresh count exactly, exceeds the recipe's "≥1714/0/1-ignored"
floor (13 prior standards + this wave's 9 = 22, zero failures anywhere).

## 3. A genuine multi-standard shared-fixture gap found and fixed (the load-bearing finding of this closer pass)

Before graduating anything, I traced exactly how the framework's own m5 auto-discovery test resolves
a facet's example fixture (`🧪️fixture-sweep/🦀️component.rs`, `m5_auto_discovery::derive_identity` +
`pilot_resolve::find_example_semio`): a discovered grammar/protocol facet's `artifact_rel` is always
the ARTIFACT directory (`✏️s/…/🗿️artifacts/<artifact>`) — the standard name is dropped entirely — and
`find_example_semio` resolves exactly ONE `.dsl.semio`/`.pack.semio` fixture per artifact, from
`<artifact_rel>/📚️examples/<any-slug>/🖼️assets/`. A multi-standard artifact therefore shares ONE
fixture slot across all its standards' facets.

For `dwg` (ac1018 + ac1024) this is harmless: both standards' grammars use the identical literal
envelope-mark `"stdio.dwg"` (confirmed by direct read of both `.grammar.semio` files), so whichever
standard's real bytes sit in the shared slot, both grammars recognize it. The dwg fan-out agent had
already regenerated the shared artifact-level fixture from real AC1024 output, and it correctly
serves both standards.

For `gif` (87a + 89a) this is NOT harmless: 87a's grammar uses the bare `envelope-mark = "stdio.gif"`
(matching the artifact's own `STDIO_GIF_DOCUMENT_SCHEMA` constant) while 89a's grammar requires the
literal `envelope-mark = "stdio.gif.89a"` — two different literal marks. Worse, the shared
artifact-level slot (`🎞️gif/📚️examples/🎬️demo/🖼️assets/`) was STILL the pre-Phase-2 placeholder: an
11-byte `68656c6c6f` ("hello" in hex) `🗣️example.dsl.semio` with no `semio …` preamble line at all,
and no `🎒️example.pack.semio` whatsoever. Neither fan-out agent touched it — each correctly scoped
to its own `🏅️standards/🔖️8Xa/📚️examples/` subtree per the wave's ownership rules (and both of those
per-standard fixtures ARE real: 87a's is 139/87 bytes, 89a's is a genuinely large real-fixture-backed
8.8MB/4.4MB pair) — but the framework's own shared-slot resolver never looks at either of those
per-standard locations.

Fix: repointed the shared artifact-level slot to real gif87a `print_dsl()`/`encode_pack()` output,
copied verbatim from gif87a's own already-real, already-tested standard-level fixture (not
hand-derived independently):
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` —
  overwritten (139 bytes, real).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new
  (87 bytes, real).

Verified this both fixes gif87a's own framework-level grammar/protocol check AND correctly leaves
gif89a's check soft-failing (not hard-failing) as long as gif89a itself stays off the graduation
list — confirmed live via `cargo test -p semio-framework-os-kernel --lib m5_handcrafted -- --nocapture`
before touching the graduation list at all: `[DEBUG] soft (stdio-exempt, pre-FG-wave) grammar
conformance failure for 🗄️stdio::🎞️gif::🔖️89a: grammar did not recognize shipped fixture DSL body` —
exactly the expected, harmless, still-soft outcome. This is why gif89a's `Grammar`/`ProtocolPack`
facets are deliberately NOT graduated (§5) — graduating them would hard-fail
`m5_handcrafted_grammar_conformance` for real against the now-gif87a-shaped shared fixture. This is a
`pilot_resolve` single-fixture-slot-per-artifact mechanism gap, not a gif89a content shortfall:
gif89a's own `⚙️engine::tests::conformance_laws::*` (6/6, using ITS OWN correct per-standard fixture)
independently pass for real, both per this closer's own crate-wide run and per the independent
`p2-fg2-verify-report.md`.

I did not attempt to fix `pilot_resolve` itself (a per-standard-aware fixture resolver) — that is a
framework-mechanism change to `🧪️fixture-sweep/🦀️component.rs` beyond a closer's append-only
graduation-list mandate for that file.

## 4. Policy gate — `bun run ./📜️script.ts policy`

**A measurement artifact found first**: the 5 PC-seeded rules' `kind` strings
(`stdio-artifacts/{grammar-parseability,protocol-parseability,fixture-honesty,
language-registration,json-transfer-ban}`) never appear anywhere in this CLI command's own printed
output, for any standard, regardless of allowlist state. Traced to
`runPolicyExit`/`formatBreachReport` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/
🟦️typescript/📦️index.ts`) — it prints ONLY `priority: "high"` breaches, and every finding
`policySchemaOverhaulPCBreaches` (`📜️script.ts`) produces for these 5 rules is unconditionally
`priority: "low"` (confirmed by reading each of the 5 sub-functions' `breaches.push({...})` call
sites). This means FG1's own closer-report claim of "0 breaches" for these rules via `bun run
./📜️script.ts policy` was a structurally tautological measurement — it would read exactly "0" no
matter what state the allowlists are in — not a real signal, and (confirmed below) FG1's own
allowlist entries were in fact never actually cleaned up.

Verified for real instead: wrote a temporary scratch script
(`.🦑️repo/🎫️tickets/…/generators/policy_pc_breach_check.ts`, kept per the ticket's "don't delete
scratch" rule) that imports `policySchemaOverhaulPCBreaches` directly (it is `export`ed) and prints
every finding, bypassing the CLI's high-priority-only filter. Before this closer's edits:
**126 real repo-wide low-priority breaches**, of which **69 were "allowlisted in
POLICY_*_ALLOWLIST but the file is now genuinely real" (stale) breaches touching exactly this wave's
9 standards** — 27 in `POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST` (9 standards × 3 facets:
diff/mutations/snapshot), 27 in `POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST` (same shape), 7 in
`POLICY_FIXTURE_HONESTY_ALLOWLIST` (artifact-level, one per artifact: bmp/deflate/dwg/gif/jpg/las/
tiff), 8 in `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST` (every FG2 standard except jpg — jpg's own
registration gap is real, not stale, correctly generated zero breach there). Zero breaches in
`POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` touching FG2 (none of the 9 were ever seeded there).

Also confirmed, as an aside (not fixed, out of this wave's scope): FG1's own 7 standards
(`md`/`xml`/`obj`/`stl`/`dxf`/`step`/`ifc`) are STILL present in all 5 `POLICY_*_ALLOWLIST` sets
right now, despite FG1's own closer report claiming "cleanly removed from (or never needed to be
added to) every allowlist." That claim was measured the same tautological way. Left untouched here —
retroactively fixing another wave's cleanup is outside this closer's own numbered task list — but
flagged plainly for whichever pass next touches these allowlists.

Removed all 69 stale FG2 entries from the 4 affected allowlists in `📜️script.ts` (jpg's
`POLICY_LANGUAGE_REGISTRATION_ALLOWLIST` entry deliberately kept, with an in-file comment explaining
why). Re-ran the same direct-import check: **57 breaches repo-wide, 0 touching FG2's 9 standards** —
shrink confirmed for this wave, zero growth for anyone else, jpg's real gap still correctly tracked
(no new "unallowlisted violation" breach appeared for it). Re-ran the actual `bun run
./📜️script.ts policy` CLI afterward too: byte-identical high-priority output before/after this
closer's edits (**21513 high-priority breach(es) across 25 rule(s)** both times — all pre-existing,
unrelated, repo-wide noise from a concurrent session's scaffolding of new stdio artifact types
under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/**` and similar, none of it caused by or fixed by
this closer's low-priority-only allowlist edits). Raw captures: `p2-fg2-closer-policy-full.txt`
(pre-edit high-priority CLI run), `p2-fg2-closer-pc-breach-check.txt` (post-edit direct low-priority
check, 0 FG2 hits), `p2-fg2-closer-policy-postfix.txt` (post-edit high-priority CLI run, unchanged).

## 5. Graduation — 16 tuples appended to `STDIO_CONFORMANCE_GRADUATED`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`
(`//#region 🔖️StdioTransition`). Confirmed exact directory-name identifiers by listing disk
(`ls … 🏅️standards`) rather than guessing: `gif/87a`, `gif/89a`, `jpg/jfif-1.01`, `bmp/v3`,
`tiff/6.0`, `deflate/rfc1950`, `las/1.0`, `dwg/ac1018`, `dwg/ac1024`. Verified every grammar/protocol
file is non-trivial and real by size (2-7KB each, not stub-sized) before graduating.

Appended, `Grammar`+`ProtocolPack` for 8 of the 9 standards:

```rust
("🎞️gif", "🔖️87a", ConformanceFacet::Grammar),
("🎞️gif", "🔖️87a", ConformanceFacet::ProtocolPack),
("📷️jpg", "🔖️jfif-1.01", ConformanceFacet::Grammar),
("📷️jpg", "🔖️jfif-1.01", ConformanceFacet::ProtocolPack),
("🖼️bmp", "🔖️v3", ConformanceFacet::Grammar),
("🖼️bmp", "🔖️v3", ConformanceFacet::ProtocolPack),
("🖼️tiff", "🔖️6.0", ConformanceFacet::Grammar),
("🖼️tiff", "🔖️6.0", ConformanceFacet::ProtocolPack),
("🗜️deflate", "🔖️rfc1950", ConformanceFacet::Grammar),
("🗜️deflate", "🔖️rfc1950", ConformanceFacet::ProtocolPack),
("☁️las", "🔖️1.0", ConformanceFacet::Grammar),
("☁️las", "🔖️1.0", ConformanceFacet::ProtocolPack),
("🖊️dwg", "🔖️ac1018", ConformanceFacet::Grammar),
("🖊️dwg", "🔖️ac1018", ConformanceFacet::ProtocolPack),
("🖊️dwg", "🔖️ac1024", ConformanceFacet::Grammar),
("🖊️dwg", "🔖️ac1024", ConformanceFacet::ProtocolPack),
```

**`gif`/`89a` deliberately NOT graduated** — §3's mechanism gap. **`ProtocolSpr` withheld for all 9**
— direct disk check (repeated per standard) confirms none of the 9 shipped a real `.spr.semio`
fixture this wave (every fan-out report explicitly deferred it as optional/non-blocking); graduating
it would be graduation theater per the recipe's own "nothing to verify" rule.

A long, precise doc comment was added directly above the appended tuples in `🦀️component.rs`
explaining the gif89a exception in full (so a future reader doesn't need to re-derive it from this
report) — see the file itself, `//#region 🔖️StdioTransition`.

## 6. Framework m5 harness — `cargo test -p semio-framework-os-kernel --lib`

Run in full (no filter), twice — once scoped to `m5_handcrafted` right after the graduation-list
edit (to catch any hard failure immediately, before touching anything else), once as the full
`--lib` suite at the end.

**Scoped run** (`m5_handcrafted_grammar_conformance` + `m5_handcrafted_protocol_conformance`):
- Grammar: **59 facet(s) found, 59 checked, 0 soft-skipped, 32 stdio-exempt soft failure(s), 3 hard
  failure(s)**. The 3 hard failures are the SAME pre-existing non-stdio pilots FG1's own closer
  already found red before M1 (`🏗️fem::◻2d::🔖️1`, `📕️norm::📘️en1992::🔖️1`, `🕸️dag::🕸️dag::🔖️1`) —
  confirmed by name in the panic output, count unchanged (3), none of them stdio, none touched by
  this wave. `🗄️stdio::🎞️gif::🔖️89a` correctly appears among the 32 soft failures (expected, §3) —
  32 = FG1's own post-graduation baseline of 40 minus exactly 8 (the 8 newly-graduated `Grammar`
  facets), which is the arithmetically expected shrink (9 standards graduated `Grammar` minus the 1
  that stays soft-exempt on purpose).
- Protocol: **118 facet(s) found, 24 checked, 94 soft-skipped, 2 stdio-exempt-or-known-gap soft
  failure(s), 0 hard failure(s)**. Zero of the 2 soft failures or 0 hard failures mention any FG2
  artifact by name (independently grepped the full `--nocapture` output for `🎞️gif`/`🖊️dwg` — zero
  hits, meaning every graduated `ProtocolPack` facet passed silently, no failure branch triggered).

**Full-crate run**: **762 passed, 2 failed** — byte-identical to FG1's own post-graduation baseline
(`p2-fg1-closer-report.md` §4 quotes the exact same "762 passed; 2 failed" numbers), confirming zero
regression anywhere in the framework crate from this wave's graduation-list append + allowlist edits.
Both failures are the same 2 test functions (`m5_handcrafted_grammar_conformance`,
`m5_production_coverage`) failing for the same 3 pre-existing non-stdio pilots. `m5_production_coverage`
also logs a soft `[DEBUG]` note for gif89a's own uncovered-production coverage — informational only,
not a failure (gif89a is not in that test's hard-failure list either).

## 7. `git check-ignore -v`

This closer's own edits created no new directories (the one new path,
`🎞️gif/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio`, sits inside an already-tracked directory).
Widened the check to every genuinely new (`??`, untracked) directory across the whole wave, per
`git status`, since this closer is the wave's own integration point: `🎞️gif/🏅️standards/🔖️87a/
📚️examples/`, `🎞️gif/🏅️standards/🔖️89a/📚️examples/` (both new — the gif fan-out agent's own
per-standard fixture directories, §3), `🖊️dwg/🏅️standards/🔖️ac1018/📚️examples/` (new — the dwg
fan-out agent's own ac1018-dedicated fixture directory), plus the one new file above. Ran
`git check-ignore -q` on each individually: **all four exit 1 (not ignored)** — confirmed via
`.gitignore:179`'s own `!**/🔖️*/**` un-ignore rule, which explicitly whitelists everything under a
`🔖️<version>/` standards directory (the exact fix STATUS.md's own top-of-file "🐛 Also fixed" entry
documents from earlier in this ticket). All four will be picked up correctly by git.

## 8. STATUS.md

Appended a full `## FG2 (fan-out wave, …) — CLOSED 2026-08-11` entry mirroring FG1's own entry's
structure and level of detail, including the §3 mechanism-gap finding and the §4 measurement-artifact
finding. Did not remove or edit any prior content.

## Program tally after FG2

22 of 31 official stdio standards now have real grammar/protocol files, real fixtures, and
`Grammar`+`ProtocolPack` graduated: 13 from before this wave (6 from PC's pilot ladder —
json/csv/zip/png/txt/binary — + 7 from FG1 — md/xml/obj/stl/dxf/step/ifc4) plus this wave's 8 newly
graduated (gif/87a, jpg, bmp, tiff, deflate, las, dwg/ac1018, dwg/ac1024). gif/89a is real and
independently tested but not framework-graduated (§3). `ProtocolSpr` remains graduated for only
csv/txt (unchanged this wave). 9 standards remain for future FG-waves per the plan's roster.

## Known follow-up, not fixed this wave

1. **jpg/jfif-1.01's missing 5-role `LanguageSpec` registration** — self-disclosed by jpg's own
   fan-out report, independently confirmed by this closer (zero `register_language`/
   `register_schema_spec` hits anywhere in jpg's tree) and by `p2-fg2-verify-report.md`. Small,
   isolated, copy-pasteable from any FG2 sibling's `register_pilot_languages()`. jpg's
   `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST` entry was deliberately kept (not stale).
2. **gif/89a's `Grammar`/`ProtocolPack` graduation is blocked by a real framework mechanism gap**
   (§3): `pilot_resolve` resolves one fixture per ARTIFACT, not per standard, so a multi-standard
   artifact whose standards use different literal grammar envelope-marks cannot have more than one
   standard framework-graduated at a time without a per-standard-aware fixture resolver. Not fixed
   here (would mean editing `🧪️fixture-sweep/🦀️component.rs` beyond the append-only graduation-list
   mandate) — recommend a small, dedicated framework-mechanism task: teach `pilot_resolve` to prefer
   a fixture slug whose name matches the facet's own standard id when more than one is available.
3. **FG1's own 7 standards are still present in all 5 `POLICY_*_ALLOWLIST` sets** despite FG1's own
   closer report claiming otherwise (§4) — a real, previously-undetected leftover from a measurement
   artifact in the CLI's own high-priority-only filtering, not something this wave introduced or is
   chartered to fix. Flagged for whichever pass next touches these allowlists.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs` — 16-tuple graduation
  append + explanatory doc comment (§5). Only edit in this file.
- `📜️script.ts` — 69 stale entries removed across `POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST`,
  `POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST`, `POLICY_FIXTURE_HONESTY_ALLOWLIST`,
  `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST` (jpg's entry in the last one deliberately kept, with an
  explanatory comment). No other part of this file touched.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` —
  overwritten with real gif87a `print_dsl()` output (§3).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new,
  real gif87a `encode_pack()` output (§3).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/STATUS.md`
  — appended FG2 closed-wave entry.
- `.🦑️repo/🎫️tickets/…/generators/policy_pc_breach_check.ts` (new, kept — ticket scratch script, §4).
- `.🦑️repo/🎫️tickets/…/p2-fg2-closer-policy-full.txt`, `p2-fg2-closer-pc-breach-check.txt`,
  `p2-fg2-closer-policy-postfix.txt` (new — raw command captures, §4).
- `.🦑️repo/🎫️tickets/…/p2-fg2-closer-report.md` (new — this file).

No `glue.rs`, and no artifact-owned `.rs`/`.grammar.semio`/`.protocol.semio` file (other than the
gif fixture pair above, which is a fixture asset, not codec/schema content) was touched by this
closer. Ticket left open for the orchestrator/next wave.
