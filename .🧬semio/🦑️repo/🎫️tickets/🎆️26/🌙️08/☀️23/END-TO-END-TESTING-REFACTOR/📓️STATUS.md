# STATUS — the one authoritative record

Every other note in this folder is a dated wave report and describes only its own slice. Several read
as complete and are not. **This file supersedes all of them for totals.** If a number appears here
and in a wave note, this one is current.

The failure that made this file necessary: `📓️w14-final-audit.md` and `📓️w15-specification-defects.md`
both say "the five red scenarios". There are **at least 31**. Neither note was wrong about what it
measured; neither stated the total, and each read as whole.

## Red scenarios: 31, not 5 — and 31 is a floor

A per-owner sweep of the 36 non-`🗄️stdio` owners: `cases=63 executed=2319 passed=2288 failed=31
not-exercised=21`. **Nine red owners, not two.** `🗄️stdio` — 2,177 scenarios, 44% of the repo —
could not be measured at all that day, so the true count is higher.

They are **four causes, not 31 findings**, and every one is a refusal by clause, never a crash:

| n | cause |
|---|---|
| 16 | A composed child's `childId` is a **content address no document states** — 58 `DefaultHasher::new()` call sites across 46 files, and `std` explicitly refuses to specify that hasher's output. Affects `program-1`, `block-3d-1`, `note-1`, `writer-1`, `en1990-1`. |
| 7 | The `.dsl.semio` carrier has **no usable grammar**. Three are worse than the known `payload = OCTET+` placeholder: `forms`/`playbook` commit a grammar describing a *different document*, shared verbatim by `layout`/`draw`/`raster`; `note` commits a real grammar covering three of its six block kinds. |
| 7 | Committed vectors **under-determine their own verb**. |
| 1 | A vector is **not self-contained**. |

These are specification defects for the repository owner to adjudicate, not bugs to tune away.

## Parity — the number that answers the bar

**Unmeasurable repo-wide.** Measured so far: 10/10 on the two TypeScript-subject owners, and 244/246
across 23 of 166 cases by a concurrent session — **5.0% of scenarios**, and 11 of those 23 are
`create-*` synthesis cases rather than mutation cases.

The peer's `component_persistent_local!` break **has expired**; a Rust subject host now compiles and
emitted all 15 of `mutate-zip-2-0`'s scenarios. The constraint is now **our own harness**: five runs
died on `spawnSync cargo ETIMEDOUT` under one shared cargo target dir, each discarding everything
with no summary line. A repo-wide sweep had to be replaced by 36 per-owner invocations.

Earlier reports of `parity=1012/1277` (79.2%) were real when measured and are **not currently
reproducible**.

## The owner's bar is half unmet, and that was uncounted

**56 of 145 `mutate-*` cases — 2,592 of 4,862 scenarios, 53% — run on no artifact, on under 4 KiB, or
on a generic `🎬️demo` placeholder.** The flagship conversion `mutate-program-1` (533 scenarios, the
largest case in the repo) runs on a **28,538-byte synthetic demo**: `"Sample Clinic"`, `CLN-001`,
epoch timestamps. Ten of the eleven artifact-less cases gained a second implementation and none
gained a document.

## Second implementations

- `@no-oracle-`: **43 → 28 cases**, 1,719 → 578 scenarios. 11 argue from a clause; **17 still inherit
  the argument their siblings falsified**, though all 17 now self-label `THIS DECISION IS A DEBT, NOT
  A JUDGEMENT`.
- **The 12 `🏗️ifc`/`📐️step` cases still declare zero differential scenarios.** They were side-stepped,
  not fixed: two *new* IfcOpenShell cases were added instead, on the real 2,496,437-byte Nakagin IFC4.
  `model.to_string()` is a genuine writer, so the capability is proven — it just was not applied where
  the gap is.
- The 15 `📕️norm` oracles were **near-copies — all fifteen hashed to one hash** — now honestly
  refactored into a single 874-line shared module.

## What has held across five consecutive audits

**Zero weakening.** No comparison-profile knob changed, no `@comparison-` swap, no `ignoreKeys`
added, no widened tolerance, zero `@mode-differential` removed, no fixture shrank or was swapped, the
migration ratchet untouched, and the `law::` call-site delta unchanged. Independence is clean: no
oracle imports, wraps, shells out to, or transliterates our Rust.

## Confirmed production defects found by differential testing

`encode_pdf` serialized the retained COS graph alone, so every mutation in the authored `pages`/`info`
lane applied to the snapshot and vanished on export — **ten of thirteen PDF failures were this one
defect**, fixed at the cause. `🧊️obj`'s `RemoveFace` inverse lost `g`/`o` membership. `📄txt`'s
`(lines, trailing_newline)` was not injective. Plus, found earlier and unverified until parity ran:
`encode_tiff` dropping IFDs, `encode_bmp` discarding palettes, `decode_avi` rejecting real ffmpeg
output, `AviSnapshot` dropping nested chunks, `xml_escape_attr` losing control-character escapes.
