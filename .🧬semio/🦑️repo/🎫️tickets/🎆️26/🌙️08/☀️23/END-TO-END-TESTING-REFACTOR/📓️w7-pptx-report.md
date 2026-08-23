# Wave 7 — 🎞️pptx ECMA-376 ✳️any mutation oracle report

Executor subset: `pptx` standard `ecma-376` subset `any`. Follows `📓️w7-fleet-brief.md` exactly;
shape copied from the green cases in `📓️w7-results.md` (closest precedents: 🎒zip's archive half,
📰xml's `quick-xml` half — this subset composes both, as the brief instructed).

## Files

- **Vocabulary** (read-only, confirmed): `.../🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — `PptxMutation`, 9 variants (`NoMutation`, `SetSnapshot`, `InsertSlide`, `RemoveSlide`,
  `MoveSlide`, `InsertShape`, `RemoveShape`, `SetShapeText`, `SetShapePosition`). Added `pub const
  KINDS: &[&str]`, `pub fn kind_of(&PptxMutation) -> &'static str`, and test
  `kinds_matches_enum_variants_and_manifest` (mirrors the 🎒zip precedent's own test), keeping the
  enum, the const, and the catalog manifest honest against each other.
- **Oracle** (new): `.../🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` →
  `semio_s_plugin_stdio_test_oracle::artifacts::pptx::standards::v_ecma_376::subsets::any::{oracle_apply_mutation,
  oracle_apply_mutation_inverse, oracle_round_trip, project_pptx_mutation}`.
- **Catalog + registration** (new): `.../🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`
  — oracle id `pptx-ecma-376-mutate`, `"ecosystem": "rust"`, capability `pptx-ecma-376-mutate`,
  catalog `pptx-ecma-376-any` (9 kinds), comparison profile `semantic-pptx-mutate-v1`.
- **Test case** (new): `.../🎞️pptx/🧪️tests/mutate-pptx-ecma-376/component.feature` + `🦀️component.rs`
  (oracle handlers top-level, subject handlers in `#[cfg(feature = "sut")] mod subject`).
- **Real fixture** (new): `.../🎞️pptx/🧫️fixtures/🎞️semio-talk.pptx` (112,429 bytes), referenced as
  `shared://🎞️semio-talk.pptx`.
- **Derivation script** (ticket-folder scratch, kept per the brief's "derivation scripts are in
  this ticket folder" convention): `w7-pptx-ecma-376-mutate/derive-semio-talk-subset.py`.

## Design — why `zip` + `quick-xml` composed, no standalone `pptx` crate

The only `pptx` crate on crates.io is a 0.1.0 first release. Per the brief's explicit instruction,
registering it would let a first-release crate judge our implementation, which is worse than no
oracle at all. `zip` and `quick-xml` are both already linked (for 🎒zip and 📰xml/🎨svg
respectively) and both genuinely independent of this repository's own PPTX codec. The oracle:

1. Unzips the PPTX with `zip`.
2. Parses `ppt/presentation.xml`, `ppt/_rels/presentation.xml.rels`, `[Content_Types].xml`, and
   every `ppt/slides/slideN.xml` with `quick-xml`, into an independent typed `Vec<PSlide>`
   (element/text tree parser written fresh in this file — not shared with, and not copied from,
   the 📰xml/🎨svg oracles' own trees, since PPTX has no shared family module either subset
   participates in).
3. Applies the mutation as a pure operation on that typed `Vec<PSlide>` (insert/remove/move a
   slide; insert/remove a shape; set a shape's text/position).
4. **Regenerates every slide-related OPC part from scratch** on every call — `ppt/slides/*.xml`,
   each slide's own `.rels`, `presentation.xml`'s `p:sldIdLst`, `presentation.xml.rels`'s slide
   relationships, `[Content_Types].xml`'s slide `Override`s — while every other part (layouts,
   master, themes, media, docProps, root rels) is carried forward byte-for-byte. This sidesteps
   incremental rId/id bookkeeping entirely and means `no-mutation` and `identity-round-trip` are
   genuine re-serializations too, never a byte pass-through.
5. Rezips with `zip`.

`MoveSlide` — the mutation the brief flagged as the point of this whole wave — relocates the real
title slide ("SemIO", index 0) to the end of the real 7-slide deck; verified directly against the
real fixture (see Verification below).

The projection (`semantic-pptx-mutate-v1`) is scoped to exactly what
`crate::artifacts::pptx::schema::snapshot::PptxShape` itself models: ordered slides, each an
ordered shape list, each shape's kind/text/position. A shape with no explicit `a:xfrm` (inheriting
position from its slide layout — true of every real placeholder in the chosen fixture except the
picture and two of slide 5's textboxes) projects as `x=y=cx=cy=0` on both sides, mirroring
`crate::artifacts::pptx`'s own importer's documented default for a missing `a:xfrm`
(`position_from_xml`, confirmed by direct reading before this default was chosen). OPC-level
plumbing a rebuilt slide mints fresh — part paths, relationship ids, `p:sldId`'s numeric id,
`p:cNvPr`'s shape id/name, which slideLayout a rebuilt slide targets — is absent from the
projection because the production typed model doesn't track it either.

## The real fixture — 0-byte placeholder finding, and the derived subset

**Finding, as the brief asked to report**: the committed
`📚️examples/🎬️demo/🖼️assets/🎞️example.pptx` for this artifact is 0 bytes — a placeholder, not a
fixture. It is not `include_bytes!`'d anywhere (the demo example's `PRIMARY_TEXT` comes from a
`.dsl.semio` file instead), so it is inert rather than a landmine; left untouched, out of this
ticket's scope.

The real 16 MB, 62-slide, 2020 conference deck at
`temp/domai-specific-programmaning-language-for-architects.pptx` ("Eine domänenspezifische
Programmiersprache für Architekten", 27.11.2020) was reduced to a smaller but genuine subset
**without corrupting the package's relationship graph**, per the brief's preference:

- Kept the first 6 real slides in presentation order (all real German/English text: title,
  agenda, "Warum?", "Informationstechnologien", a dense two-quote Latour slide, "Mit welchen
  Informationstechnologien...") plus real slide 23 ("Diagrammnotation") — the first slide in
  presentation order carrying a real embedded picture (`image3.png`, a real photo).
- Closed the OPC relationship graph around them: all 11 real `slideLayout`s (the one real
  `slideMaster` declares relationships to every one of them; trimming any would leave a dangling
  relationship in the master's own `.rels`), the real `slideMaster`, both real `theme`s, the real
  `notesMaster`, `presProps`/`viewProps`/`tableStyles`, and only the 3 real media images the kept
  parts actually reference (`image1.png`/`image2.png`, the master's own backgrounds;
  `image3.png`, slide 23's real photo).
- `docProps/app.xml`'s descriptive `<Slides>` count and `TitlesOfParts` vector were updated from
  62 to the 7 kept real slide titles (cosmetic/descriptive metadata, not structural, but left
  honest rather than stale).
- The PowerPoint-only `<p:extLst>` ("sections" + slide guides, a `p14`/`p15` extension outside
  ECMA-376's own `p:sldIdLst` structure and not parsed by this subset's decoder) was removed
  rather than left referencing numeric slide ids this derivation drops.
- Result: 112,429 bytes, 55 real ZIP members, every internal XML part verified well-formed
  (`xml.etree.ElementTree`) and the archive verified with `unzip -t` (no errors) before being
  wired into any test.

`temp/` is gitignored; `git check-ignore -v` on the copy committed under this artifact's
`🧫️fixtures/` confirms it is trackable:
```
.gitignore:624:!**/🧫️fixtures/**	✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🧫️fixtures/🎞️semio-talk.pptx
```
(exit code 0 — the file matched the negation rule that re-includes `🧫️fixtures/**`, so it is not
ignored.)

The full derivation script (stdlib-only Python — zipfile/re, no new dependency, a one-time
ticket-folder tool, not part of the repository's runtime) is at
`w7-pptx-ecma-376-mutate/derive-semio-talk-subset.py` in this ticket's own folder.

## Verification

Sanity-checked first with a standalone local Cargo binary (outside the source tree, in this
session's scratchpad — not committed) that `#[path]`-includes the oracle file verbatim against a
copy of `semio_repo_test_host::Json` and drives all 9 kinds + their inverses + the identity round
trip directly against the real fixture bytes; every kind mutated correctly (verified by reading
back the real projected slide/shape content — e.g. `move-slide` genuinely reordered slide 0 to the
end; `set-shape-position` genuinely moved the real picture shape's `a:xfrm`) and every inverse
restored the original projection exactly. This caught two real bugs before the official run: a
regex path-prefix mismatch in the Python derivation script that silently dropped every slide
relationship, and this oracle's own `quick-xml` document parser rejecting the `\r\n` real PowerPoint
writes between the XML declaration and the root element (fixed by tolerating whitespace-only text
there).

Official verification, from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-pptx-ecma-376
0 high-priority breach(es) across 0 rule(s):

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-pptx-ecma-376
[test] level=exhaustive cases=1 executed=19 passed=19 failed=0 errored=0 parity=0/0
```

19 = 9 `mutate-<kind>` + 9 `inverse-<kind>` + 1 `identity-round-trip`, all green. Confirmed no
breach in the full breach set (`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`) names `pptx`
(`grep -ic pptx` → 0).

`cargo check --features oracles` in the standalone oracle crate
(`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`) compiles this subset's oracle file with zero
warnings or errors of its own; a transient error in the concurrently-being-written 🏗️ifc peer file
was observed once and self-resolved on retry, exactly as the brief warned.

**Rust SUBJECT phase**: confirmed genuinely blocked repo-wide, not by anything in this subset —
`cargo check` at `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust` fails to even load the workspace manifest
(`✒️writer` plugin's `Cargo.toml` inherits `js-sys` from a workspace root that no longer declares
it), matching the brief's own "peer os-kernel refactor" note. The subject half
(`#[cfg(feature = "sut")] mod subject` in the case adapter) is written in full against the real
`PptxMutation`/`decode_pptx`/`encode_pptx`/`apply_pptx_mutation` API, transcribing
`PptxMutation::inverse`'s exact closed-form logic (confirmed by direct reading of the production
mutations file before transcribing), but is unverified by a compiler until that repo-wide breakage
clears. No subject or parity result is claimed.

## Hard-rule compliance

Stayed inside `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/`. Did not touch `📕️xlsx`/`📜️docx`, the
framework, `Cargo.toml`/`📦️lib.rs`, `.gitignore`, `project.json`/`launch.json`, or any other
subset's oracle file. No shared family module was created or edited — PPTX's `zip`+`quick-xml`
composition is written fresh in its own oracle file, matching the brief's note that PPTX has no
shared family module either the 🎒zip or 📰xml oracle already participates in. No modifying git
command was run. Ticket not closed or reopened (coordinator-owned).
