# F6d Closer — docx, md, xml, jpg, json, dxf, tiff (fourth and LAST F6 op-codec sub-wave)

**Role**: C6d closer. Only agent in this sub-wave permitted to touch `📦️glue.rs`/`📜️script.ts`
(neither was actually edited — both stayed read-only, see below). Read all 7 fan-out reports and the
independent verify report, applied any `glue_followup` items (none existed), ran the full crate gate
and the repo-wide policy check myself (not trusted from any report), updated `STATUS.md`'s ownership
ledger with a full F6-program summary, and wrote this report plus the separate
`f6-final-summary.md` program-wide consolidation for the orchestrator's next (gate/G) wave.

## 1. Reports read

- `f6-docx-ecma-376-report.md`, `f6-md-report.md`, `f6-xml-report.md`, `f6-jpg-report.md`,
  `f6-json-rfc8259-report.md`, `f6-dxf-r12-report.md`, `f6-tiff-report.md` (7 fan-out agents).
- `f6d-verify-report.md` (independent verify agent — re-ran every scoped test suite itself, grepped
  every diff/mutations file directly for `impl protocol::DiffCodec`/`OpText`/`OpBinary`, absence of
  live derive attributes, absence of `serde_json` stub calls; re-ran the whole-crate suite and the
  policy check itself rather than trusting the self-reports).

All 7 self-reports check out against the verify report; the verify report's own headline ("All 7
self-reports check out ... 1075 passed, 0 failed ... 1 stdio breach remaining, `ifc/2x3` only") was
independently reproduced by this closer's own fresh runs below, not just re-read.

## 2. `glue_followup` items

**None.** Grepped all 7 fan-out reports plus the verify report for `glue_followup`/`glue followup`:
zero hits in any of the 8 files. Every one of the 7 artifacts' op-codec work landed entirely inside
already-mounted `🧬️schema/{🔺️diff,🧬️mutations}/🦀️component.rs` files (the same leaves F1-F5 already
wired into `glue.rs`) — no new `glue.rs` mount or `script.ts` change was ever needed, same pattern
every F6 sub-wave closer (F6a/F6b/F6c) already documented. Nothing to apply.

## 3. Full crate gate (this closer's own fresh run)

```
cargo test -p semio-s-plugin-stdio --lib
```

**Result: 1075 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out**, finished in ~7.8-8.0s
(saved twice, consistent both times — `f6d-closer-full-crate-test.txt` in this folder). Matches the
independent F6d verify agent's own number exactly. Per-artifact scoped counts (cross-checked against
the verify report's own independently-run numbers): docx 47/47, md 26/26, xml 24/24, jpg 31/31,
json 60/60, dxf 15/15, tiff 31/31 — sum 234, all passing, all including both mandatory law tests
(`diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`).

No failures needed investigating — this closer's own gate ran clean on the first try. All 7 fan-out
reports independently documented (and cross-reading confirms internal consistency across all 7,
consistent with them observing the same real shared-tree phenomenon rather than fabricating it)
transient concurrent-session churn mid-session: a `docx`-internal tri-state test-coverage gap that
`docx`'s own fan-out agent caught via its own failing `cargo test` run and self-fixed before this
closer ever ran (not a cross-artifact issue); a stale `3d`-module Cargo-manifest glitch from an
unrelated concurrent workspace-relocation wave, self-resolving; and brief compile windows where `md`/
`docx` were mid-edit by their own sibling F6d fan-out sessions. None of these were ever caused by, or
required action from, any artifact outside its own ownership boundary, and none are present in this
closer's own final, clean run.

## 4. Policy check (this closer's own fresh run)

```
bun run ./📜️script.ts policy
```

Exit code 1 (breaches exist — expected, see below), full output saved to
`f6d-closer-policy-run.txt` (21591 lines).

**`dsl-migration/diff-completeness` rule, stdio-scoped: exactly 1 breach.**

```
dsl-migration/diff-completeness  ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs
```

Grepped the full breach listing for every one of this wave's 7 artifact/standard paths (docx, md,
xml, jpg, json, dxf, tiff): **zero matches for any of the 7** — every one's new hand-rolled
`DiffCodec` impl satisfies the check's literal-text grep
(`content.includes("dsl::DslDiff") || content.includes("DiffCodec for")`). The only remaining stdio
breach is `🏗️ifc/2x3`, the pre-existing 32nd standard the recon report's own §8 row 5 explicitly
flagged as out of scope for the entire F6 program from the very first pilot session (added by the
unrelated sibling ticket `ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`, never one of the official 31,
never rostered into F6a/F6b/F6c/F6d). This confirms the F6 program's own stated goal — "the goal is
for the live policy check to stop flagging your file ... zero stdio entries" — is achieved for the
full official scope: **31/31 official standards, 0 breaches.**

`POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) confirmed untouched: grepped the allowlist's
full contents for `stdio` — zero matches, unchanged from every prior sub-wave. Every one of the 28
F6a-d breaches (and the 3 recon-pilot breaches before them) was resolved by a real implementation,
none by allowlisting.

**No policy-checker quirk found.** Per the task brief's instruction to investigate whether a surviving
breach might indicate a policy-checker quirk worth documenting (like F1's facet-mirror-drift
false-positive) — I checked: `ifc/2x3`'s diff file genuinely has no `DiffCodec` impl (grepped its
`🔺️diff/🦀️component.rs` directly: no `impl protocol::DiffCodec`, no `dsl::DslDiff` derive, real
`serde_json`-shaped absence). The breach is real and correctly reflects unfinished, never-assigned
work on an explicitly out-of-scope 32nd standard — not a checker quirk, not a false positive, nothing
to document as a bug.

**Repo-wide (non-stdio) breach count**: 129 total `dsl-migration/diff-completeness` breaches exist
across the whole repo (other plugins entirely outside this program's scope — `✒️writer`,
`➗️mathematical`, and others — all deferred to a separate future wave per the allowlist's own "W6"
comments). Only 1 of the 129 is under `🗄️stdio`, and that 1 is the explicitly out-of-scope `ifc/2x3`.

**Other unrelated breach types observed** (not in scope for this ticket, not investigated further):
`taxonomy/emoji-prefix` fires 645 times across `🗄️stdio` paths (missing U+FE0F variation selectors on
emoji-prefixed directory names) — pre-existing, unrelated to diff-completeness, not part of this
program's mandate.

## 5. Shared-file check (`glue.rs`, `script.ts`)

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`: **zero diff** against tracked baseline as of
  this closer's session (`git status` shows it clean). The "MM" state visible in the task brief's
  initial `git status` snapshot had already resolved by the time this closer ran — another concurrent
  session's edit, not touched by this closer or any of the 7 F6d fan-out agents.
- `📜️script.ts`: small pending diff (2 insertions, 6 deletions), inspected directly via `git diff` —
  a `POLICY_STDIO_OWNER_TABLE_REL`/`POLICY_STDIO_OWNER_TABLE_LEGACY_REL` path-migration edit (moving
  the stdio owner-table SSOT path from a ticket-folder fallback to the stdio plugin's own registry,
  dropping the one-wave legacy-path fallback comment). Grepped for all 7 of this wave's artifact
  names and for `ALLOWLIST`: zero matches — unrelated to `POLICY_DIFF_COMPLETENESS_ALLOWLIST` and to
  any of this wave's artifacts. Same concurrent sibling-ticket-automation pattern every closer since
  F2 has documented and correctly left alone. **Not touched by this closer.**

Neither file was edited by this closer despite being the one agent in this sub-wave permitted to —
there was nothing to apply.

## 6. STATUS.md ownership ledger

Appended two new sections to `STATUS.md`:

1. **F6d sub-wave section** — same format as F6a/F6b/F6c's own sections: roster, per-artifact
   classification table (all 7 hand-roll/hand-roll, the highest hand-roll density of any F6 sub-wave),
   full-crate gate result, policy shrink result (8→1), `glue_followup`/shared-file check, and the
   ownership-ledger update marking all 7 rows op-codec-complete.
2. **"F6 program — CLOSED" section** — a short capstone marking the whole F6 program (recon pilot +
   F6a + F6b + F6c + F6d = 31/31 official standards) complete, pointing to `f6-final-summary.md` for
   the full program-wide consolidation.

## 7. Real bugs found across the whole F6 program (consolidated for the gate wave)

Collected from all 4 sub-wave closer reports plus this wave's own fan-out reports — full detail in
`f6-final-summary.md`:

1. **`csv` derive-macro hygiene bug** (F6a) — a `Mutation` variant field literally named `record`
   collides with the `dsl::DslOps` derive macro's own internal accumulator variable name, producing a
   confusing `E0308` (expected reference, found `RecordValue`) instead of the expected `DslField`
   trait-bound error. Real, reproduced, documented, not fixed (shared framework file, out of every F6
   agent's ownership boundary) — csv was hand-rolled around it.
2. **`xlsx` empty-string-key drop bug** (F6a) — every `dec_*` list-splitter chained a defensive
   `.filter(|s| !s.is_empty())` after `split_top_level`, silently dropping a legitimate empty-string
   OPC relationship-owner key. Caught by xlsx's own `diff_codec_text_binary_roundtrip_law` test,
   self-fixed in-flight by removing all 12 occurrences (within xlsx's own ownership boundary — not a
   shared-file bug).
3. **`stl` nested fixed-arity array print/parse bug** (F6b) — `[[f64;3];3]`-shaped fields compile
   clean under `#[derive(dsl::DslDiff)]` but are NOT round-trip-safe at runtime: the shared `dsl`
   crate's `Shape::Tuple` printer flattens every nesting level into one indistinguishable comma-join,
   and the parser never bounds a nested tuple's comma-consumption to its own arity. Real, reproduced
   runtime failure (`"tuple expects 3 elements, found 9"`), traced to
   `🧰️framework/…/🗣️dsl/🧬️schema/🦀️component.rs`'s `print_shape`/`parse_shape`. Not fixed (shared
   framework file, out of scope) — documented via doc-comment citation, flagged for whoever next
   works on the `dsl` crate.
4. **`las` bare-tuple missing-`DslField` gap** (F6b) — `(u16,u16,u16)`/`(f64,f64,f64)`-shaped fields
   have no blanket `impl DslField for (A,B,...)` anywhere in `dsl`, same root-cause family as the
   3b tri-state gap (a missing blanket impl) but a different type shape. Not fixed (shared framework
   file) — documented.
5. **`gltf`/`pptx`/`docx` generic-collection-diff derive gap** (F6c) — every generic collection-diff
   wrapper type (`GltfCollectionDiff<T,D>`, `IndexedTripleDiff<D,T>`, `NamedTripleDiff<K,D,T>`) has
   zero generics support in the `dsl::DslDiff`/`DslOps` derive macros at all — confirmed by literal
   malformed codegen (`E0107`), not just a missing-impl error. Not fixed (shared framework file) —
   documented, independently rediscovered as `bcf`'s own `NamedTripleDiff<K,D,T>` blocker in the same
   sub-wave.
6. **`jpg` tuple-arity `DslField` gap, second confirmation** (F6d, this wave) — `dsl` has no
   `DslField` impl for tuples of any arity, confirmed decisive and independently-fatal for
   `SetJfifHeader.version: (u8,u8)` even net of every cascading struct-derive fix. Same root-cause
   family as finding #4 (`las`) from F6b — this is a second, independent confirmation of a real,
   still-unfixed `dsl`-crate gap (tuples of any arity), not a new bug class. Not fixed (shared
   framework file, and fixing the Mutation shape itself was explicitly out of scope) — documented.
7. **`docx` tri-state test-coverage self-catch** (F6d, this wave) — a test-authoring gap, not a
   framework bug: the first `diff_codec_text_binary_roundtrip_law` fixture draft did not actually
   exercise the `based_on: Some(None)` tri-state transition; docx's own `cargo test` run caught this
   as a real failing assertion (not a passed-but-untested gap), fixed in-flight within docx's own
   ownership boundary before this closer ran.

**Net real, still-unfixed `dsl`-crate gaps carried forward to the gate wave**: 4 distinct classes —
derive-macro field-name hygiene (csv), nested fixed-arity array print/parse (stl), bare-tuple missing
blanket impl (las, confirmed independently a second time by jpg), and generic-collection-type derive
support (gltf/pptx/docx/bcf). All 4 are documented via doc-comment citations at their point of use in
the source and are out of scope for every F6 agent's own ownership boundary (shared `dsl` framework
crate edits forbidden by this ticket).

## 8. Files touched by this closer

- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/STATUS.md`
  — appended the F6d sub-wave section and the "F6 program — CLOSED" capstone section.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6d-closer-report.md`
  (this file).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6-final-summary.md`
  (program-wide consolidation for the gate (G) wave).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6d-closer-full-crate-test.txt`
  (this closer's own full-crate `cargo test` run, 1075/0).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6d-closer-policy-run.txt`
  (this closer's own full `bun ./📜️script.ts policy` run, 21591 lines).

**No files under `✏️s/**` were touched by this closer.** `📦️glue.rs` and `📜️script.ts` were both
read-only for this session — neither needed an edit. No `POLICY_DIFF_COMPLETENESS_ALLOWLIST` change.
No git-mutating commands were run.
