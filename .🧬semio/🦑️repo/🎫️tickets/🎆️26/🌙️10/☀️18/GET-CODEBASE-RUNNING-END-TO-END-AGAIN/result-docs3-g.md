# result-docs3-g (wave 1026g, executor docs3-g): the build guide as it will be after tonight's landings, staged per code batch

FOR gate-g (21:03): 22 doc batches are ready in `C/docs3-g-tools/staged/<name>/` (index and landing order: `C/docs3-g-tools/staged/INDEX.md`). Land each WITH the code batch it names and revert it WITH that batch. `python3 -I stage.py check|apply|revert [--root DIR]` in each directory; idempotent, anchored, exit 2 on drift and exit 3 on a missing code prerequisite (both write nothing). The live guide was NEVER written by this lane: it still has the staged base sha256 `38412f96781d2985b7d09a375ced44a5318be8cea76592a588294d45f79b2a4c` (checked at the start, before the last build and at the end).
Three things you must know before landing:
1. **nxfix-g `core` replaces the WHOLE guide file** (manifest entry `📓️build-guide.md`, base sha256 = the current live file). It refuses after ANY other guide edit. Either land `core` before every other doc batch, or take the guide entry out of core's manifest and land the doc batch `nxfix-core` instead (its two "Nx path" / "Nx-Pfad" bullets are byte-identical to core's, so both ways end in the same guide).
2. **Code batches that carry their own guide edit** (kept, not duplicated): quietfc (`edit_code.py`, anchored on the `SEMIO_HOST_CAPACITY=off` line), obs-g (`apply_guide.py`; my `fleet` batch has the same text byte for byte, so either may land first), census2 / testmem2 (whole "Test runners" bullet; land exactly one), fdleak `guard` (prefix of the same bullet, after census2), walk-g footprint (`estimates.json` sentence). None of my edits touches those anchors: `foreign.py` re-checks all 14 foreign anchors on a guide with every doc batch applied (0 broken).
3. **`quietfc` doc batch needs the quietfc code batch first** (its own guide text says "staged, quietfc-g"; my batch turns that into "live", closes the 4b status, adds the waiting row): exit 3 before.

## What each batch adds (EN + DE, same structure; every batch is self-contained and says only what its code batch does)
| doc batch | code batch | what a developer or agent now reads |
|---|---|---|
| `quietfc` | quietfc-g-tools | "live" for the quiet follow-up / envelopes / declared machine bullets of the code batch; 4b closed in the status; capacity-wait row (follow-up forecast, soft stops, `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES` + `SEMIO_HOST_CAPACITY_CORES` only shrink) |
| `journal-fix` | verify2-g journal-fix | change-journal service: 128 MiB cap (it died with `memory` within seconds on the live tree), `root-gone` / `directory-gone`, bounded stop, leaked `reuse-*` pairs; "cannot vouch" row |
| `vouch` | verify2-g vouch | dependency verification asks the journal: `dependencies.vouch` = journal / cursor / walk:<reason> / off, what a vouch may skip, measured CPU 2.2 s to 0.65 s, real-run proof still owed |
| `probes` | toolsetup-g probes (+ twin probes-after-conv) | why the planner skip never hit (4 host probes), the closed `hostProbes` table, expected tool-setup 32-48 s to 3-6 s (not measured), "cannot be reused later" row |
| `plan-once` | graphlat-g plan-once | a scoped refresh plans once; the open double inference/setup (about 230 of 345 s) |
| `dashlock` | dashlock-g dashlock | a running dashboard never blocks a build (same digest reused under a shared lease, new digest beside it, try-lock only, keep current + 2), `Refusing to wait` row with `SEMIO_LEASE_ORDER` / `SEMIO_LEASE_ORDER_POLICY` |
| `purge` | purge-g purge | the one purge law: only `🗑️generated` of CLOSED tickets, tracked / young / link / not-closed never, `repo ticket purge-artifacts [<ticket>] [--apply]`, `repo clean --dry` first |
| `fleet` | obs-g fleet | `repo fleet status [--json] [--limit=<n>]`, MCP `fleet_status`, troubleshooting row, status line |
| `census2` | census2-g staged (xor testmem2) | per-file memory ceiling message (EN/DE), what to declare, `SEMIO_TEST_FILE_CEILING=off`; memory-table row |
| `fdleak-guard` | fdleak-g guard (after census2) | the `bun test ./` rule and the ENFILE row (bare operand = filter = 122,880 descriptors), the three guard messages, `SEMIO_TEST_OPERAND_GUARD`, `SEMIO_TEST_ORPHAN_GUARD`, `SEMIO_TEST_OPEN_FILES_CEILING` |
| `jobsrv` | jobsrv-g staged | Cargo's job pool handed to build scripts and wrapper (16 to 4, 6 to 3, 9 to 3 concurrent compilers), empty-pool fallback + notice row |
| `nxlock` | nxlock-g nxlock (after nxfix core) | Nx daemon start order, inert classes, `[lease-order] Lock order violated` row (observe mode, the `bazel-slot-gate` diagnostic every hot run prints) |
| `setup` | setup-g staged | new section "First run on a new machine": `repo preflight`, `repo tools setup all`, the Node native core, compiler helpers, per-platform prerequisites |
| `oracle2` | oracle2-g staged | read oracle on huge keys (split profiles, `--max-profiles`, mirror hand-over), log flush marker, no local time stamp |
| `xplat3` | xplat3-g staged | new section "Windows, Linux and macOS": rename retry, installer mutex, symlink probe, native core on Windows, unknown free space; what was NOT run on Windows |
| `platforms` | xplat4-g platforms | 220 suite verdicts (147 Windows-capable, 73 POSIX-only), six causes, CI rows block, `tag.py`, the 8 suites still to decide |
| `freespace` | xplat4-g freespace (after xplat3) | `freeSpaceSyncV1` and what "unknown" means per site |
| `conv` | conv-g staged (after oracle2) | numbers live in schema JSON, bilingual refusals (incl. "cannot vouch"), audit rules |
| `nxfix-core` | nxfix-g core | the guide part of core as an anchored batch (see 1) |
| `bench2` | none (measurement) | first honest baseline replaces the "Not met" status line (hot no-op 60/77/83 s, edit 506 s, footprints) |
| `mirror` | none (tool exists) | verify staged batches only on `mirror-tools/mirror.py` mirrors, never via a symlink farm or the live guide |
| `guard-go` | none (guard-g is live) | Go test sandbox: identity guard, `RelocateScratch`, `SEMIO_SANDBOX_GUARD`, `SEMIO_SANDBOX_SCRATCH`, "tests touched the real repository root" row |
Not given a batch: `corpus` (xplat4-g, test-only, nothing a reader acts on), `disk-g` (live since 10-26 09:00 and already the disk section), `testmem2` (superseded by census2), `bench3` observer (harness).

## Verification (all on `C/mirror-tools/mirror.py` mirrors `d3g-a`, `d3g-b`; nothing ran against the live tree; no `repo` command, no official run)
- `docs3-g-tools/verify.py` (stdlib; 21 batches without the prerequisite one): pristine guide pending for each; in seeded random orders (seeds 1, 2, 3 with the first engine; 11, 12, 21 and the final seed with the final engine) every batch applies, re-apply is a byte no-op, all report applied, the applied guide has identical lines for every order, reverting in another random order restores the ORIGINAL BYTES; apply-half / revert-half restores them; a moved anchor makes dashlock, mirror and bench2 refuse with exit 2 and leaves the file untouched; quietfc refuses with exit 3; EN and DE edit lists are parallel and carry the same `SEMIO_` names; every table row has exactly three pipes. FAILURES 0 in the final run (seed 31).
- First run found a real engine defect (an insert whose anchor was moved to mid-line would have been spliced into the line): the engine now requires the anchor at a line edge, drift test changed to a real mutation. 
- `docs3-g-tools/verify_codebatch.py`: applies quietfc's own guide edit pairs (read with `ast` from `edit_code.py`) to a mirror guide, then the quietfc doc batch (pending, applies, "staged" words gone in EN and DE, revert gives the post-code bytes), then every doc batch on top, then `foreign.py`: 14 foreign anchors (fdleak guard 2, census2 2, testmem2 2, walk-g 2, quietfc 2, obs-g 4), 0 broken.
- `docs3-g-tools/claims.py`: 80 needles (every `SEMIO_` name in a block plus named symbols, schema keys and message fragments), searched with `rg -F -o | sort -u` over the live tree and over every `*-g-tools` dir (plus the two unpacked `new-files.tgz`): 0 missing; 36 live and staged, 44 only in a staged batch (they belong to the code batch of the same lane, e.g. `SEMIO_HOST_CAPACITY_CORES`, `SEMIO_TEST_FILE_CEILING`, `SEMIO_TEST_OPERAND_GUARD`, `fleet_status`, `dependencies.vouch`, `hostProbes`, `freeSpaceSyncV1`). `SEMIO_SANDBOX_GUARD` and `SEMIO_SANDBOX_SCRATCH` are live only (guard-g).
- Rendered review: all batches applied to a scratch copy (293 to 383 lines), headings, table continuity and the pipe count checked by script; the German half was read, not machine-checked.

## Not verified / honest limits
- Markdown rendering in a viewer; the German text beyond reading; `repo verify taxonomy` for the changed `.md` (graph-loading, not run).
- Numbers are quoted from the lanes' results (bench2-g, graphlat-g, verify2-g, jobsrv-g, toolsetup-g ...); where a result says "not measured" or "simulated" the batch says so too (vouch wall time, probes tool-setup, plan-once, dashlock official-route proof, Windows items).
- census2-g's result still has `INTERIM_ROWS` / `INTERIM_TABLE` placeholders: its census numbers are NOT quoted, only the ceiling mechanism of its batch.
- `quietfc` assumes the code batch's three bullets are present; `fdleak-guard` assumes census2 landed (its code batch says so).
- Re-run when a batch changes: `python3 -I C/docs3-g-tools/build.py` rebuilds every staged batch from the guide as it is then (anchors are read from the guide, base hash recorded), then `verify.py <mirror>`, `verify_codebatch.py <mirror2>`, `claims.py`.
