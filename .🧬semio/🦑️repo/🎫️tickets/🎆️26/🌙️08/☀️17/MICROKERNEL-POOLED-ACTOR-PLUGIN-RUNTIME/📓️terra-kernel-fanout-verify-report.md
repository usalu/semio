# kernel-fanout-verify — independent measurement report

Packet `kernel-fanout-verify` on `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`. **Measurement only, no
repairs made, no source files edited.**

## Command run (foreground, timeout 600000, exactly as instructed)

```
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-fanout \
  cargo check -p semio-framework-os-kernel --lib --message-format=short \
  > .../terra-kernel-fanout-verify-check.txt 2>&1
echo "EXIT:$?"
```
**EXIT:101** (observed directly from `echo $?`, no pipe to `tail`).

A second run with `--message-format=json` was made (still within the "2-3 checks" budget) because the
short-format text visibly undercounted: rustc's own summary line said "49 previous errors" but only 46
distinct `error[E...]` lines existed in the short-format output. JSON, parsed structurally, confirms the
compiler's own tally: **49 error-level `compiler-message`s**, exactly matching `error: could not compile
semio-framework-os-kernel (lib) due to 49 previous errors`. The 3 "missing" lines in the short-format text
are diagnostics that rustc counts in its tally but suppresses from the rendered short output as
near-duplicates (same span/message shape). **Conclusion: trust the JSON count (49), not a naive line-grep
of the short-format text (46) — this is exactly the R21 "negative result from a query that cannot report
its own failure" trap, caught by cross-checking two differently-implemented tools.**

Files: `terra-kernel-fanout-verify-check.txt` (short format, human-readable), `terra-kernel-fanout-verify-check.json.txt` (raw json, authoritative for counts) — both in this ticket folder.

## Per-module error table (bucketed by `🔨️modules/<name>` path segment, python3 json parse — not shell grep)

| module | error count | top error codes |
|---|---:|---|
| `🏪️store` | 45 | E0308 ×25, E0599 ×12, E0277 ×8 |
| `📡️spr` | 2 | E0728 ×1, E0308 ×1 |
| `🚪️io` | 2 | E0600 ×1, E0277 ×1 |
| `🗣️dsl` | 0 | — |
| `🎒️pack` | 0 | — |
| `🌿️vcs` / `📇️directory` / `💡️inference` / `🧩️extension` | 0 / 0 / 0 / 0 | — |
| **total** | **49** | E0308 ×26, E0599 ×12, E0277 ×9, E0728 ×1, E0600 ×1 |

Every error's primary span resolved to a module path — **zero errors with no module attribution.**

## Starting-baseline comparison

| module | starting errors (handed) | self-reported ending | measured ending |
|---|---:|---:|---:|
| dsl | 316 | 0 | **0** ✅ matches |
| spr | 295 | 2 | **2** ✅ matches (same two lines: `📜️history/🦀️component.rs:623,629`) |
| store | 233 | 42 | **45** ❌ **does not match — see discrepancy below** |
| os-pack | 82 | 0 | **0** ✅ matches |
| long-tail (vcs 9 + directory 7 + inference 6 + extension 4 = 26) | 26 | 0 | **0** ✅ matches |
| io | 48 | *(no self-report handed — separate live packet)* | 2 (in progress, not evaluated against a claim) |
| **total** | **1000** | — | **49** |

## Overall result

**49 errors remain crate-wide. The crate is NOT green.** Exit code 101.

Because it is not green, step 5 of the brief (`cargo check -p semio-framework-plugin --lib` as the next
gate) was **not run** — the brief says to run it only if this crate is green, and running it now would
misreport the next gate's true starting state (it should be measured once this crate's blocker set is
actually closed).

## E0038 (dyn-compatibility) count

**0** `E0038` diagnostics anywhere in this `--lib` check output (JSON-verified, full scan). This is scoped
to `--lib` only, per the brief's exact command — `--all-targets`/test-cfg code was not compiled and is not
covered by this zero.

## Discrepancy found — store's self-report undercounts by 3, and mischaracterizes their nature

**store reported**: "Ending error count: 42, all attributable to exactly two cross-module blockers, zero
unexplained residue... both outside my owned paths."

**Measured**: 45 errors in `🏪️store/🦀️component.rs`, not 42. The extra 3 (from 4 distinct new-error sites,
since one function is hit twice) are **not** covered by store's own account of "two cross-module blockers
outside my owned paths" — they are a third, distinct cause, and it is **inside** store's own file:

```
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:642:38   E0308 (Shape::Record(artifact_child_spec))
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:689:38   E0308 (Shape::Record(owner_ref_spec))
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:771:38   E0308 (Shape::Record(link_pin_spec))
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:788:161  E0308 (Shape::Record(link_pin_spec), 2nd site)
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:818:38   E0308 (Shape::Record(artifact_link_spec))
```

All five errors are `expected fn pointer 'fn() -> os_dsl::schema::RecordSpec', found fn item 'fn() -> impl
Future<Output = RecordSpec> {...}'` — the exact same E4 fn-pointer-slot shape store correctly diagnosed for
the *dsl* `FieldSpec::new`/`RecordSpec::new` functions. But the fn items here are **not** dsl's — they are
defined locally inside store's own file:

```
🏪️store/🦀️component.rs:617: async fn artifact_child_spec() -> crate::os_dsl::RecordSpec { ... }
🏪️store/🦀️component.rs:655: async fn owner_ref_spec()      -> crate::os_dsl::RecordSpec { ... }
🏪️store/🦀️component.rs:702: async fn link_pin_spec()       -> crate::os_dsl::RecordSpec { ... }
🏪️store/🦀️component.rs:784: async fn artifact_link_spec()  -> crate::os_dsl::RecordSpec { ... }
```
(confirmed by reading the file directly — grep on `fn <name>` for each of the four names).

These four local functions feed `crate::os_dsl::Shape::Record(fn() -> RecordSpec)` — the same E4
fn-pointer slot as the dsl functions store already R9-reverted — but these four are **inside store's own
owned path**, not a cross-crate blocker, and they were simply left `async` and unfixed. Applying the exact
same R9 treatment store already applied elsewhere in this file (pure builder, no I/O, feeds a fn-pointer
slot, tag `// 🚫️async: E4 fn-pointer slot` — see R9/E4) would close all 5 of these sites without touching
any file outside store's `path_scope`.

Net effect: store's true residue is **40 cross-module-blocked errors (39 dsl-caused + 1 spr-caused) + 5
in-scope, fixable errors it missed**, not "42, zero unexplained residue, both outside my owned paths" as
reported. The dsl/spr attribution itself is otherwise sound — the 39 dsl-caused errors cluster exactly
where store's report says (lines 2226, 2255, 3488, 6364, each a `FieldSpec`/`RecordSpec`/`Shape`
mismatch), and the 1 spr-caused error is exactly at `3477:58` as described.

No other packet's self-report disagreed with measurement.

## Files written by this packet (measurement artifacts only, no source edits)

- `.🧬semio/.../MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-kernel-fanout-verify-check.txt` — raw `--message-format=short` output
- `.🧬semio/.../MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-kernel-fanout-verify-check.json.txt` — raw `--message-format=json` output (authoritative for counts)
- `.🧬semio/.../MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-kernel-fanout-verify-report.md` — this report

No source files under `🧰️framework/**` or anywhere else were read-and-edited; the four store function
definitions above were read only, to identify them for this report.
