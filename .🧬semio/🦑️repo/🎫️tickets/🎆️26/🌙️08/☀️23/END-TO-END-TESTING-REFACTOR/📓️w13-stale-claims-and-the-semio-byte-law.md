# Wave 13 — clearing w12's five stale/dishonest claims, and what running the subject phase found

Successor to `📓️w12-final-audit.md` and `📓️w12-what-the-numbers-mean.md`. Every command quoted below
was actually run; exit codes were read from the tool's own exit status, never through a pipe.

The five items w12 left open were framed as small and precise. Four of them were. The fifth — giving
the 19 `mutate-semio-*` identity handlers a byte claim — required running the Rust subject phase for
those cases for the first time, and that run surfaced **five real defects in production code and in
test adapters**, all fixed at the cause. That is the part of this wave worth reading.

---

## What the subject phase found the moment it ran

| # | defect | where | fixed |
|---|---|---|---|
| 1 | Four `format!` string literals carried a stray `.await` INSIDE the quoted text, so the DSL/diff printers emitted `G[…,[].await]` and the printed document no longer re-parsed | `✳️drawing/🧬️schema/📸️snapshot`, `✳️drawing/🧬️schema/🔺️diff`, `✳️value/🧬️schema/🔺️diff` (×2) | ✅ production |
| 2 | `unbind-representation`'s inverse re-bound the escrowed link by APPENDING, so undoing a removal at index 0 returned the pool in the wrong order | `✳️kit/…/✂️unbind-representation/↩️inverse` | ✅ production |
| 3 | Every `mutate-semio-*` adapter treated ANY outcome message as a rejection, but the frozen contract makes `Info`/`Warning` advisory — `delete-vertex`/`delete-node` raise `mutation.cascade` at `Info` on every well-formed body | 19 adapters | ✅ one shared bridge |
| 4 | `mutate-semio-any`'s subject half had never compiled: `SemioSnapshot { subset, .. }` passed a `&str` where a `SemioSubsetSnapshot` was required | `mutate-semio-any/🦀️component.rs` | ✅ adapter |
| 5 | `mutate-semio-image` read `bitDepth`/`delayMs` at variant level, but serde's enum-level `rename_all` renames VARIANTS and not their fields, so the committed vectors spell them `bit_depth`/`delay_ms` | `mutate-semio-image/🦀️component.rs` | ✅ adapter |

Nothing was weakened to make any of these pass. #3 is the only change that makes a check *accept*
more, and it corrects a mis-stated law rather than relaxing a real one: the assertion that matters
(`current != expected`) is untouched, and `Info`/`Warning` messages ride along with a diff that WAS
applied in full, by the contract's own definition.

---

## The five w12 items, one by one

### 1. `mutate-svg-1-1/component.feature:55` — already corrected, verified rather than re-fixed

`grep -rn "OPEN, and left red\|FAILS on the ORACLE" ✏️s/` returns **nothing**. The paragraph now
reads *"A note on `inverse-remove-element`, which WAS red and is not any more — kept here because the
defect is instructive and the remedy is the one this feature named rather than a relaxed law"*, and
goes on to name the fix (apply the forward step and its inverse to ONE parsed tree) and to record
that `mutate-xml-1-0` shared the routing and never showed the defect because its minified fixture
carries no inter-element whitespace. `git log --date=iso -1` on that file dates the correction
`2026-08-25 01:06:10 +0200`, i.e. after the w12 audit snapshot was taken. **No edit was needed; the
audit item was stale, not the file.**

### 2. The "peer-blocked" claim — 19 adapters, not 15

The audit named 15 files (`mutate-zip-2-0`, `mutate-gif-89a`, `mutate-pptx-ecma-376`,
`mutate-txt-utf-8`, `mutate-docx-ecma-376`, `mutate-dxf-r12`, `mutate-tiff-6-0` and 8
`mutate-semio-*`). Seven of those had already been corrected by the time this wave started. What was
still there was **all 19 `mutate-semio-*` adapters**, each carrying some variant of *"the Rust
SUBJECT phase is blocked this wave by a concurrent os-kernel refactor, so it is written and gated but
not run."* Every one now states the real position, naming its own case:

> the Rust SUBJECT phase RUNS. The os-kernel blocker earlier waves recorded here was cleared on
> 2026-08-24 — `cargo check -p semio-framework-os-kernel --lib` exits 0 and `semio-s-plugin-stdio`
> builds — so `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case <case>` really executes
> every scenario below. The gate keeps the two BUILDS apart; it has never been a reason the subject
> half goes unmeasured, and for this recorded no-oracle case the subject phase is the only phase that
> runs at all.

One further stale sentence was found outside that pattern and corrected:
`mutate-semio-kit/🦀️component.rs:29` claimed a latent `protocol::Mutation`-unreachability gap in
`✳️text`'s adapter was "masked" by two causes, one of them the os-kernel block. It has one cause now,
and the paragraph says so.

`grep -rn "peer-blocked\|blocked this wave\|os-kernel refactor" ✏️s/` (excluding the ticket tree) now
returns only `mutate-zip-2-0`'s own line, which already reads *"…that blocker was cleared on
2026-08-24"*.

### 3. `create-and-round-trip-bmp` / `create-and-round-trip-tiff` — the 1.000 similarity pair

Both feature files were byte-identical apart from the format name. Each is now written against its
own format's real distinguishing behaviour, and each names its own fixture and says why that fixture
is the one it is:

* **BMP** describes the POSITIONAL layout — 14-byte `BITMAPFILEHEADER`, 40-byte `BITMAPINFOHEADER`,
  scanlines padded to a 4-byte boundary, rows stored bottom-up — and the fact that the reference
  encoder is handed RGB because a 24-bit `BI_RGB` bitmap carries no alpha at all.
* **TIFF** describes the opposite: an 8-byte header with a byte-order mark and an offset to a CHAIN
  of tag directories, geometry in `ImageWidth`/`ImageLength`, samples behind
  `StripOffsets`/`StripByteCounts`, every value over four bytes stored out of line — and the fact
  that the reference encoder is handed full RGBA, because TIFF carries four samples per pixel.

The BMP case's second fixture also changed, from `8×4` to `5×3`. Both of the old sizes gave a
scanline that was already a multiple of four bytes (`4×3 = 12`, `8×3 = 24`), so **neither exercised
the 4-byte row-padding rule that is BMP's most characteristic failure mode**. `5×3` gives a 15-byte
scanline that must be padded to 16 — one dead byte per row, three rows — so a stride computed as
`width * 3` reads the image skewed by one more byte on every row down, and the odd height leaves no
symmetry to hide a flipped image behind. That is a STRENGTHENED fixture, not a swapped one: the
comparison profile, the assertions and the TIFF fixtures are untouched.

### 4. The 19 `mutate-semio-*` identity handlers and the byte law

Before this wave the family asserted the semantic half of the identity law and made no byte claim in
either direction. Now every one of the 19 either asserts the byte law or states, in the handler and
in the feature, exactly why it cannot.

The choice between the two forms of the law was made from EVIDENCE, not from assumption. A
ticket-local probe (`w13-carrier-probe/`) parsed each subset's committed `📚️examples/…/🗣️example.dsl.semio`
and `🎒️example.pack.semio`, re-printed and re-encoded, and compared bytes:

```
animation: dsl_exact=true (278/278)     cad:   dsl_exact=true (479)  pack_exact=true (560)
audio:     dsl_exact=true (226/226)     document: dsl_exact=true (610) pack_exact=true (271)
video:     dsl_exact=true (172/172)     drawing:  dsl_exact=true (394) pack_exact=true (533)
flow:      dsl_exact=true (249) pack_exact=true (160)   graph: dsl=297 pack=183
kit:       dsl=734 pack=498    model: dsl=544 pack=476  object: dsl=380 pack=267
presentation: dsl=826 pack=516 table: dsl=240 pack=132  text:  dsl=203 pack=118
mesh:      dsl=340 pack=388    image: dsl=217 pack=110  envelope: dsl=286 pack=205
brep:      dsl=443 pack=537
```

Every committed artifact is reproduced byte for byte by the codec that wrote it, and every pack
decodes to the same snapshot its text twin does. So the correct law here is
`law::carrier_is_exact` — the same reading `mutate-dag-1` and `mutate-bmp-v3` already record for
their own carriers — and `law::reparsed_not_copied` would be exactly backwards. Each handler now
says which it is and why, so the exemption is a checked claim rather than an absence.

**What each of the 19 got:**

| case | byte claim |
|---|---|
| `cad` `document` `drawing` `flow` `graph` `kit` `mesh` `model` `object` `presentation` `text` | `carrier_is_exact` on the `.dsl.semio` text AND on the `.pack.semio` twin |
| `audio` `animation` `video` | `carrier_is_exact` on the `.dsl.semio` text; no pack bridge is exported for these three, and the handler says so rather than implying the twin was checked |
| `image` `any` | new `parse`/`print`/`decode`/`encode` bridges added to production (`✳️image`, `✳️any`), so their identity handlers now read the real `🖼️swatch` / `🌐️envelope` artifacts and assert both carriers — until now these two scenarios moved no artifact bytes at all |
| `brep` `table` | had **no `identity-round-trip` scenario at all**. Both now have one, reading `🧊️solid` / `📃️sheet`, with `carrier_is_exact` on both carriers. `✳️brep` gained the four bridges too |
| `value` | **documented absence.** `s.stdio.semio.value` is the only one of the eighteen subsets that commits no example artifact in either encoding, so there are no committed bytes to reproduce and no input bytes a codec could have copied. The handler and the feature both state this, name what closing it would take, and are explicit that what the scenario asserts instead is the typed completeness law — real, but not a byte law |

This is what caught defects 1 and 2 in the table above: `carrier_is_exact` on `✳️drawing`'s text
carrier failed immediately with `G[…,[].await]`, and adding the assertions forced the subject phase to
run at all, which is how the `kit` inverse-ordering bug surfaced.

### 5. `discovery is idempotent` — explicit budget

The test performs two full repository discoveries and was measured at 5130.40 ms against bun's 5000 ms
default. It now carries an explicit `30_000` budget, the same one its repo-walking siblings in the
same `describe` block already had, with a comment giving the reason and stating why raising it hides
nothing (cost is proportional to the committed case count, and the contract phase is what fails if
that count is wrong).

---

## Verification — every line real output, exit codes read from the tool's own status

### `bun ./📜️script.ts contract` — exit 0

```
0 high-priority breach(es) across 0 rule(s):
```

### `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio` — exit 0

```
[test] level=exhaustive cases=101 executed=1321 passed=1321 failed=0 errored=0 parity=0/0 not-exercised=25
```

An earlier run of the same command exited 1 with `executed=1274 passed=1274 failed=0`, because three
cases (`mutate-html-5`, `mutate-ifc-2x3`, `mutate-ifc-2x3-cobie`) died on
`error: failed to get 'serde' as a dependency of package 'semio-s-plugin-stdio'` — a cargo-registry
hiccup under four concurrent sessions, not a code failure. Re-run individually they are
`21/21`, `11/11` and `15/15`, and the clean whole-owner run above is green.

### `bun test 🧪️index.test.ts` in `📦️packages/🟦️typescript` — exit 0

```
 69 pass
 0 fail
 1823 expect() calls
Ran 69 tests across 1 file. [80.22s]
```

`discovery is idempotent` no longer times out; an earlier run of the same suite, taken while three
cargo builds were saturating the machine, was also `69 pass / 0 fail` at 133.52 s total.

### The Rust SUBJECT phase, all 19 `mutate-semio-*` cases — 509 scenarios, all green

```
animation   27/27    any          43/43    audio   21/21    brep     27/27    cad     33/33
document    37/37    drawing      35/35    flow    27/27    graph    23/23    image   27/27
kit         31/31    mesh         35/35    model   23/23    object   19/19    presentation 31/31
table       17/17    text         15/15    value   19/19    video    19/19
```

Every one of those `executed = passed`, `failed=0 errored=0`. `brep` and `table` are 27 and 17 rather
than 26 and 16 because each gained the `identity-round-trip` scenario this wave added.

### `parity exhaustive --owner 🗄️stdio` on the two rewritten round-trip cases — both exit 0

```
create-and-round-trip-bmp:  [test] level=exhaustive cases=1 executed=4 passed=4 failed=0 errored=0 parity=2/2
create-and-round-trip-tiff: [test] level=exhaustive cases=1 executed=4 passed=4 failed=0 errored=0 parity=2/2
```

**`parity=2/2` on BMP is the first differential comparison this case has ever run**, and it found a
sixth defect on the way: the subject import read
`semio_s_plugin_stdio::artifacts::bmp::standards::v3::…` where the module is `v_v3`. The oracle-only
run never compiles the `sut` half and no parity run had ever been made, so that path was wrong and
green at the same time. Fixed, and the file now says so. With it fixed, the new `5×3` fixture — the
one that actually exercises BMP's 4-byte scanline padding — **passes against the reference decoder**,
so our stride arithmetic and row-order canonicalisation are right where nothing had checked them.

### What could NOT be verified, and why

`cargo test -p semio-s-plugin-stdio --lib` does not compile: **914 errors**, none of them in code
this wave touched. They are `#[cfg(test)]` blocks in OTHER artifacts (`📐️step`, `🧊️gltf`, `🖊️dwg`,
`🎞️pptx`, `🏗️ifc`, …) carrying the same automated-async-sweep damage as the four `.await`-in-a-format-
string defects fixed above — e.g. `exp.awaitected`, `logical_mismatcasync hes`, and an
`#[async_test]` on a non-`async fn`. The lib itself builds (`cargo check -p semio-s-plugin-stdio
--lib` is exit 0); it is only the test harness that is broken, which is why nothing else noticed.

Consequence for this wave: the in-crate fixture test I updated alongside the `✳️kit` inverse fix
(`the_undo_restores_the_captured_link_at_its_own_index`) **could not be run**. The law it states is
executed end to end anyway — `mutate-semio-kit`'s `inverse-unbind-representation` scenario is the
same assertion through the test platform, and it is green — but that is a substitute, not the thing
itself, and it is recorded here rather than glossed.

**That 914-error `cfg(test)` build is the biggest thing this audit found that is still open.**
