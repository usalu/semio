# Original Unicode And OpenType Work Authority

All Unicode and OpenType child runtime owners now accept the original FontWorkLedger. Native advance/step append work: &mut semio_framework_2d_text_font::work::FontWorkLedger; portable advance/step append work: FontWorkLedger. There is no default grant or compatibility route. Pure result readers and borrowed font table reads remain source based.

## Original Admission And Payload Effects

Unicode admission, property/mapping probes, normalization, grapheme clustering, bidi and line breaking share the parent ledger. Native paged output reservations use work.slot and committed work.push; portable retained rows use the owned FontBuffer and the corresponding ledger holds. Normalization ordering/composition, bidi cell/run/paragraph/order/target rewrites, and reverse line links explicitly price retained writes before mutation. Retry order preserves a completed mapping/property probe until the actual commit has authority. Native errors are borrowed static literals, avoiding error String allocation during refused original turns.

OpenType feature planning, canonical scalar mapping, font fallback, classes, mark/ligature/substitute/context matching, nested lookup application and cluster layout forward the same ledger to every child. Native contiguous buffers use first-allocation contiguous admission and vec_slot/vec_push commits. Original glyph gap admission returns pending after a separately funded allocation, then admits its uninitialized slots without reading them. Each insert/move quotes and pays its payload copy before modifying either glyphs or gap positions. The original compacted allocation transfers with its existing capacity; it is not cloned into an alternate output.

Portable glyph workspaces no longer allocate maximum glyph backing in their constructor. admit(work), insert(value,work), moveStep(to,work) and compactStep(work) require original authority. Portable archive publication advances one retained archive row per turn. Source strings/font bytes stay in their original owners. This work adds no runtime library dependency or build manifest change.

## Test-First Fixtures And Independent Oracles

New shared fixtures live under Unicode/🧪️tests/🎟️work and shape/🧪️tests/🎟️work. The initial Unicode witness reproduced progress with zero original items. The initial OpenType witness failed strict verification because the ledger parameter did not exist. Matching native laws consume the same fixtures and include HeapWitness comparisons of physical allocations/releases against receipts. Existing native/portable semantic fixture callers now supply an explicitly authored original test ledger; retirement calls were not accidentally changed into mutation/admission calls.

Portable source execution used V8 normalization, ICU Intl.Segmenter, bidi-js, the existing integrity-pinned @cto.af/linebreak oracle interface, and opentype.js actual feature/lookup/ligature tables. OpenType stringToGlyphs does not apply the required DFLT emoji feature policy, so emoji observations use its actual independent substitution.getLigatures records for the selected script/features. No production output is used as its expected result.

Actual executed evidence before Core's separately owned retirement migration:

- Complete Unicode original-authority law:254-source strict closure, zero diagnostics;3 NFC/grapheme cases,12 bidi cases and4 line-break cases;63,842 funded turns and100,645 denied item/copy/capacity/depth retries, beginning with zero authority and advancing only with caller-provided demands.
- Existing Unicode semantic corpus:9 test groups pass, including the768 mixed bidi direction combinations, category/composition/mapping witnesses, normalization/grapheme ICU comparisons, malformed records and interruption/retirement laws.
- Official Unicode16 line-break corpus:16,672 cases,3 grants and14 refusals pass through the original ledger against the pinned independent oracle.
- OpenType feature law:20 real shipped font faces,931 funded feature turns and931 zero-item denials pass against independently parsed actual feature/lookup indices.
- Complete shaping pipeline:4 portable text cases /8 grapheme clusters,58,214 funded turns and58,214 denials from zero copy/capacity/depth authority. Latin, canonical marks, DE/US flags and profession ZWJ output match the independent OpenType tables after normalization, clustering, font fallback and default GSUB composition.
- Existing OpenType semantic corpus:11 groups pass, covering real font source selection, single/multiple substitutions, GDEF classes, composed defaults, mark anchors, repeated contextual gap edits, compacted array transfer, contexts, lookup plans, cancellation and real ligatures.
- Combined owned runtime and new-law TypeScript closure:268 actual sources, zero diagnostics.
- Durable native source audit:16 runtime source files,36 funded advance/step functions,48 forwarded child calls, zero audit problems. It rejects obsolete unpriced reserve helpers and direct try_reserve_exact/push_reserved calls inside those turns.21 runtime/test Rust files are parser-clean. Maximum native path106 UTF-16 code units.

Ticket runners: unicode-work-check.mjs, unicode-corpus-check.mjs, shape-work-check.mjs, shape-corpus-check.mjs, font-native-work-check.mjs. Existing line-break-check.mjs was updated to use the new original caller signature. Generated transpilation outputs are removed by runners; no source copies were placed in the ticket.

## Ownership And Verification Limits

Core owns font base/catalogue/run/paragraph/outline and Layout shared admission. Core has now reserved font-only portable retirement, including field-local intoRetirement callback edits in these Unicode/shape sources; this report does not claim original release accounting for the previous UnitRetirement API or host VM GC. Core will rerun composed callers after that change. My allocation APIs are stable and no retirement blocks are reserved by me.

Native compile, native semantic/physical laws and native runtime execution remain UNVERIFIED: the cached repo runner does not compile current source. Rust grammar/source audits are not compile evidence. No raw cargo, build refactor, modifying git command, worktree or AGENTS edit was used. The independent native ttf-parser/rustybuzz laws are mounted for real execution when the current native runner becomes available.

## Narrow Glyph Workspace Close Follow-Up

At Core's request the original portable workspace now exposes pure nextCloseDemand() and closeStep(work), delegating row retirement and actual contiguous backing release to FontBuffer.retireOne(work). Compaction's one tail-slot pop also pays its copy before mutation. The new neutral workspaceRetirement fixture was authored before the API; initial strict verification reproduced the missing demand/ledger. Actual isolated execution then passes9 close turns,27 below-demand refusals,512 copied bytes and512 released bytes. The three targeted workspace/test/ledger sources have zero diagnostics; the full imported closure had one remaining Core-owned in-progress retirement caller diagnostic at that instant. This is portable modeled backing accounting, not a claim about host GC bytes. workspace-close-check.mjs executes this law. Core owns the context/apply closeStep(work) caller and the remaining font retirement migration.

## Completed Retirement Cohort Recheck

After Core completed the font retirement migration, fresh-cache execution again passes Unicode authority253 sources/0 diagnostics with the same63,842 turns/100,645 refusals, all9 existing Unicode groups, and all11 OpenType groups. Shape authority261 sources/0 diagnostics now passes74,594 complete-pipeline turns/denials,20 faces/931 feature turns/denials, plus9 workspace close turns/27 denials/512 copied and512 released bytes. The increased pipeline work reflects the newly funded compaction tail-slot writes. The narrow line-break fixture now imports its actual admission/source helpers, supplies original work to Unicode admission, and declares its private test backend through the owned LineBreakReference observation interface. Its253-source strict closure is0 diagnostics; the actual16,672-case independent official corpus still passes.
