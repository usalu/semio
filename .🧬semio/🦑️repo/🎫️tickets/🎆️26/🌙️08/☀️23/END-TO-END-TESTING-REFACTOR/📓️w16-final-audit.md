# Final audit of `26/08/23/END-TO-END-TESTING-REFACTOR` after the divergence-fixing wave

Run window 2026-08-25 22:45 → 2026-08-26 09:20 CEST. Head at start
`8d9b51f081f42b36722b54f80a5c502d6322f9ca` (2026-08-25 14:57:24 +0200), dirty tree — 2,610 modified
or untracked paths, and **the tree kept moving throughout**: a peer session broke and re-fixed
`semio-framework-plugin` between 04:20 and 07:32, and the case population grew from 164 to 166 cases
while this audit was running. Successor to `📓️w13-final-audit.md`.

Raw logs: `w16-audit/`. Every `[test]`, `test result:` and `[budget]` line below is copied verbatim
from the tool's own stdout; every exit code was read from the tool's own exit status, never through a
pipe.

---

## 0. The most misleading thing a reader would otherwise believe

**That this ticket still has a headline number.**

`📓️w13-final-audit.md` closed with `parity=1012/1277` and a remediation list whose item #10 was
*"make a per-case budget overrun fail that case, not the whole run."* That item was not done. In the
eleven hours of this audit **the three aggregate commands aborted four times between them, on four
different cases, across all three phases, and not one of them emitted a `[test] level=…` line**:

| command | first case to blow the 900 s budget | outcome |
|---|---|---|
| `parity exhaustive` (repo-wide) | `mutate-present-1` (`🎞️animate`) | exit 1, no summary, ~12 of 166 cases done |
| `parity exhaustive --owner 🗄️stdio` | `mutate-pdf-1-7-e` | exit 1, no summary, ~90 of 101 cases done |
| `oracle exhaustive` (repo-wide) | `mutate-ifc-2x3-sav` | exit 1, no summary — **and this command was exit 0 in every previous wave** |
| `parity exhaustive --owner 🗄️stdio` (retry, warm) | `mutate-avi-1-0` — **case 1** | exit 1, no summary |

And the cause is not the one the runner names. Its own hint text says *"Likely shared cargo
target-dir lock contention from another concurrent session."* The actual chain, measured:

1. `📜️script.ts:340` points **every** generated test host at one shared
   `CARGO_TARGET_DIR = ⚡️cache/agents/local/cargo-test-hosts`.
2. Nothing prunes it. `cleanTestOutputs` (`📦️index.ts:1428`) walks `testCacheRoot` /
   `taxonomy.testOutputChildDirs` — that is `⚡️cache/tests/{results,work,hosts,diffs,reports}`,
   **842 MB**. The shared target dir is not in its scope at all.
3. It reached **280 GB** and filled the volume: `df` reported `926Gi` total, `2.9Gi` available,
   **100 % capacity**, at 07:52.
4. Writes failed mid-build. `cargo test -p semio-s-plugin-stdio --lib` died with
   `error: failed to write file …/dep-graph.part.bin: No space left on device (os error 28)`, and the
   shared dir is now **corrupt** — a direct `cargo build` of one host at 08:56 returned
   `error: could not parse/generate dep info at: …/cargo-test-hosts/debug/deps/host-7c417a03471afe69.d`.
5. Every host build therefore recompiles from scratch, which alone exceeds 900 s — which is why the
   fourth attempt died on **case 1**.
6. `runProbe` still `throw`s on `ETIMEDOUT`
   (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1708-1712`)
   and nothing catches it between there and `run`, so one slow case discards every result before it.

So the honest headline is not a ratio. It is: **the testing module has grown a 280 GB build
directory that its own `clean` command cannot see, filled the disk with it, corrupted it, and can no
longer produce any aggregate number at all.** Everything else in this report is measured around that.

The second most misleading thing, and it would survive a green run: **the oracle count nearly
doubled without a single new third-party reference.** §2.

---

## 1. The commands

All from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` unless stated.

### 1.1 `bun ./📜️script.ts contract` — **exit 1** (w13: exit 0)

```
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery

  testing/discovery  🧰️framework  42 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s  4 executable test file(s) outside the canonical owner-root test tree, baseline allows 1
```

**Breaches by rule id: `unmanaged-tests` (kind `testing/discovery`) × 2. All fourteen rule ids that
`📓️w13-final-audit.md` enumerated are still at zero.** `⚡️cache/breaches/testing.json` read straight
afterwards holds exactly those two records, nothing at a lower priority.

**Attribution — not this ticket's work.** The baseline (`📇️registry/🔒️migration.json`) has not moved
since `a2746cd371` (2026-08-23 20:01), so the counts grew. Exactly ten executable test files are new
since w13's head `9ed590cd87`, which is exactly the overage (7 + 3):

```
🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🟨️javascript/🧪️browser-host.test.js
🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🧊️webgpu/📦️packages/🟨️javascript/🧪️webgpu-surface.test.js
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/eventstore/eventstore_test.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️g1_contract_test.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️contract_test.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️g3_event_store_test.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️g3_filesystem_test.go
✏️s/🔌️plugins/🎬️sequence/… ×3 (untracked)
```

None touches a format, an oracle, a profile or a case. The ratchet is working; somebody else's work
grew the legacy backlog.

### 1.2 `bun ./📜️script.ts oracle exhaustive` (repo-wide) — **exit 1, NO SUMMARY**

```
[budget] cargo run --quiet --manifest-path …/test-s-plugins-stdio-artifacts-ifc-305581-mutate-ifc-2x3-sav-oracle-rust/Cargo.toml -- --plan … --out … exceeded 900000ms — killed. Likely shared cargo target-dir lock contention from another concurrent session — investigate before retrying.
error: spawnSync cargo ETIMEDOUT
      at runProbe (…/📚️library/📦️packages/🟦️typescript/📦️index.ts:1699:18)
      at executeOne (…/🧪️test/📜️script.ts:529:17)
      at runPhases (…/🧪️test/📜️script.ts:579:23)
EXIT=1
```

It ran 05:31 → 07:30 and got as far as `🖨️raster` / `✒️writer` before dying. **This is a regression:
`oracle exhaustive` was exit 0 with `executed=1331 failed=0` in wave 12 and wave 13.** Nothing about
the oracles regressed — the build environment did.

### 1.3 `bun ./📜️script.ts parity exhaustive` (repo-wide) — **exit 1, NO SUMMARY**

```
[budget] cargo run … test-s-plugins-animate-artifacts-present-9e140e-mutate-present-1-subject-rust … exceeded 900000ms — killed.
error: spawnSync cargo ETIMEDOUT
EXIT=1
```

That host links `semio-repo-test-host` + `semio-s-plugin-stdio-test-oracle` (lopdf, ruststep, image,
zip, quick-xml, gif, …) **plus** `semio-s-plugin-animate`. The budget is applied to `cargo run`,
which *includes compiling the host*, so a cold build over fifteen minutes is not an anomaly.

### 1.4 `bun ./📜️script.ts parity exhaustive --owner 🗄️stdio` — **exit 1 twice, NO SUMMARY**

First attempt (00:11 → 03:10) died on `mutate-pdf-1-7-e-subject-rust`; the retry (08:21 → 08:43) died
on `mutate-avi-1-0-subject-rust`, the first case.

**The stdio numbers in §3 are therefore taken from 24 per-case runs**
(`parity exhaustive --owner 🗄️stdio --case <case>`, one file per case in `w16-audit/percase/`),
because a per-case overrun costs one case instead of the run. Labelled as such everywhere.

### 1.5 `bun ./📜️script.ts dependency` — **exit 0**

```
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
[dependency] production-debt png (oracle png-png-1-2-mutate) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt zip (oracle zip) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt image (oracle image-tiff-6-0-mutate) reachable from ✏️s/🔌️plugins/🎞️animate/…/🎥️video/🦀️component.rs, 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs — owner 🧰️framework/🔨️modules/🗺️surface
```

Byte-for-byte the wave-11, wave-12 and wave-13 figures. **That stability is itself a finding — §4.**

### 1.6 `bun test 🧪️index.test.ts` in `📦️packages/🟦️typescript` — **exit 1** (w13: exit 0)

```
 66 pass
 3 fail
 2061 expect() calls
Ran 69 tests across 1 file. [153.21s]
```

`expect()` calls went **1,823 → 2,061** — the suite gained real assertions for the first time in
three waves. Two of the three failures are §1.1 surfacing twice:

```
(fail) 🔍️ discovery and contract > every committed case satisfies the frozen contract
  - []
  + [ "testing/discovery:🧰️framework:42 executable test file(s) …", "testing/discovery:✏️s:4 executable test file(s) …" ]
(fail) 🔍️ discovery and contract > the migration backlog is a shrink-only ratchet, never a growing allowlist
  Expected: <= 1   Received: 4
```

The third is load-induced and was running against the full-disk condition:

```
(fail) 🧹️ clean safety > removes marked outputs, never unmarked directories, and reports identically in dry mode [58547.59ms]
  ^ this test timed out after 5000ms.
```

### 1.7 `cargo test --features oracles --lib` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — **exit 0**

```
running 374 tests
test result: ok. 372 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 125.04s
```

374 (w13: 371), 372 passing (w13: 369), the same 2 `#[ignore]`d one-shot fixture-derivation helpers.
This crate does not link the framework, which is why it is the one Rust suite that ran cleanly all
night.

### 1.8 `cargo test -p semio-s-plugin-stdio --lib`, from the repo root — **exit 101, SIGABRT, NO RESULT LINE**

```
running 5807 tests
error: test failed, to rerun pass `-p semio-s-plugin-stdio --lib`
  process didn't exit successfully: `…/semio_s_plugin_stdio-2fb2227c8dda3b4b` (signal: 6, SIGABRT: process abort signal)
```

w13 reported this target as **913 compile errors and zero runnable tests**. It now compiles and
**5,807 tests are declared** — but the command as prescribed still yields no `test result:` line,
because one test aborts the process. Re-run with that one test skipped (labelled: not the prescribed
command):

```
$ cargo test -p semio-s-plugin-stdio --lib -- --skip a_semio_member_mints_and_reopens_a_real_child_envelope
running 5806 tests
test result: FAILED. 5667 passed; 136 failed; 3 ignored; 0 measured; 1 filtered out; finished in 117.34s
```

**136 failures — exactly the number `📓️w19-three-crosscutting-defects.md` reported. Not one of them
has been fixed since.** Detail in §6.3.

### 1.9 `cargo check -p semio-s-plugin-stdio --lib` — **exit 0**

```
warning: `semio-s-plugin-stdio` (lib) generated 109 warnings (run `cargo fix --lib -p semio-s-plugin-stdio` to apply 104 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 7m 07s
```

Measured at 07:40, after the peer's `semio-framework-plugin` refactor landed. Between 04:20 and 07:32
this same command was **exit 101** with 1–7 errors, all of them in `semio-framework-plugin`
(`E0432: unresolved import component::component_persistent_local`, four `E0599: no method named
dispatch_typed`), while `🔌️plugin/🦀️component.rs` was being rewritten under it — file mtimes 04:56,
05:29. Recorded because it is what stopped the subject phase for three hours, not because it is a
defect of this ticket.

---

## 2. The population moved under the ratio, and the new oracles are our own code

| | w13 (`9ed590cd87`) | 2026-08-25 23:20 | 2026-08-26 08:40 |
|---|---|---|---|
| cases | 164 | 164 | **166** |
| scenarios | 4,564 | 4,910 | **4,933** |
| `@oracle-` cases | 79 | 121 | **138** |
| `@oracle-` scenarios | 1,331 | 3,191 | **4,355** |
| `@no-oracle-` cases | 85 | 43 | **28** |
| oracle whose reference is a THIRD-PARTY package | 79 | 79 | **81** |
| oracle whose reference is an IN-REPO second implementation | 0 | 42 | **57** |

Read the last two rows together. Oracle-backed cases went 79 → 138, and **56 of the 59 new ones are
a `*-python-independent` / `*-typescript-three-independent` registration with `"package": ""`** — a
Python or TypeScript file written in this repository, in this ticket, by the same wave that wrote the
Rust it is compared against. That is a legitimate second producer under the standard, and several are
excellent (§5). It is not the same evidence as `lopdf`, `comrak` or `ruststep`, and a dashboard that
reports "oracle coverage 138/166" without that split overstates what changed.

**And most of them cannot yet produce a single comparison**, because their plugin's Rust subject does
not compile. `📓️w14-norm-no-oracle-conversion.md` says so about its own work: *"`parity` is still
0/0 and the reason is not this work — `semio-s-plugin-norm` does not compile"*, with its own log line
`[test] …/mutate-en1996-1: rust subject host exited 101 without emitting results`. The 15 `📕️norm`
cases carry 799 scenarios; `mutate-program-1` (`🏛️architect`) gained an oracle overnight and carries
533 on its own. A case in that state contributes `executed=0 passed=0 failed=0` to the summary line
(`📜️script.ts:523`, `:528`) — w13 remedy #2, still open.

---

## 3. Parity, measured per case

24 stdio cases completed a per-case `parity exhaustive --owner 🗄️stdio --case <case>` run. Verbatim
lines in `w16-audit/percase/`; aggregate:

```
cases=24 executed=510 passed=508 failed=2 errored=0 parity=242/244   (99.18 %)
```

**This is not comparable to w13's 1012/1277 — it is 24 of 101 stdio cases, not all of them.** What it
does establish, on the cases it covers, is that the fixing wave's claims hold:

| case | w13 | now | reading |
|---|---|---|---|
| `mutate-docx-ecma-376-transitional` | 8/13 | **13/13** | the `conformance` attribute fix works |
| `mutate-docx-ecma-376-strict` | subject never compiled | **21/21** | `vml_markup` added to the production vocabulary |
| `create-and-read-jpeg` | subject never compiled | **2/2** | `standards::v_jfif_1_01` |
| `create-and-round-trip-stl` | subject never compiled | **2/2** | `standards::v_ascii` |
| `mutate-bmp-v3` | 14/15 | **14/15** | the documented open divergence, unchanged |
| `mutate-gif-87a` | 24/25 | **24/25** | the documented open divergence, unchanged |
| `extract-text-pdf-1-4` | 0/2, 2 errored | **executed=2, parity=0/0** | see §7 |

Both remaining failures are the two paragraphs that forbid closing them by weakening anything:

```
[test] parity failed: …🖼️bmp::mutate-bmp-v3::mutate-set-pixel-data::rust::subject (1 differences)
[test] parity failed: …🎞️gif::mutate-gif-87a::mutate-set-global-color-table::rust::subject (1 differences)
```

Divergences seen in the aborted whole-owner runs, with attribution verified against the code rather
than the fixing agents' prose:

* `mutate-gif-89a :: mutate-set-screen-size` and `:: mutate-set-frame-geometry` — **under-specified
  verb, documented**, feature `component.feature:31-45`. The reference emits a frame that overhangs
  the canvas; `encode_gif` refuses. GIF89a permits both.
* `mutate-pdf-1-7 :: inverse-{remove-page, append-page-content, set-page-content}` — **ours**, the
  `PdfPage` snapshot carries a page's content as one extracted `text` field, so neither side can
  round-trip a 294-operator content stream. The feature says so and refuses to drop the axis.
* `mutate-md-commonmark :: mutate-set-snapshot` — **THEIRS, and `📓️w13-final-audit.md` §2.2(11) got
  this wrong.** That audit attributed it to "OURS (CommonMark parser)". The feature had already said
  the opposite since `18adc8cce3` (2026-08-25 01:06, *before* the audit's own head moved):
  `comrak`'s **writer** injects a literal `<!-- end list -->` between a list and a following code
  block, and its own reader then reports that as a sixth block. A `#[cfg(test)]` test added this wave
  (`📝️md/…/🚪️io/📥️import/🧩️deserializers/🦀️component.rs`) demonstrates that, fed the oracle's own
  output, this parser reports the `htmlBlock` in position 3 exactly as the oracle does. Left red on
  purpose; no profile widened.
* `mutate-semio-model :: {mutate-set-snapshot, inverse-set-snapshot}` — two diffs dated 22:03, i.e.
  written by a **peer session's** run before mine started, showing the Rust subject projecting `null`
  where the Python reference projects the document. **Not attributed here**: I did not reproduce it
  in a run of my own, and a diff file is a derived artifact. It is flagged for the next wave to
  confirm.

Repo-wide parity, per-owner, for the owners that completed:

| owner | comparisons | equal |
|---|---|---|
| `🧰️framework/🔨️modules/🖱️ui` | `cases=3 executed=17 passed=17 failed=0 errored=0 parity=7/7` | 7/7 |
| `🧰️framework/🔨️modules/🎠️kernel` | `cases=2 executed=7 passed=7 failed=0 errored=0 parity=3/3` | 3/3 |

`🦑️repo` (`host-protocol-parity`, 30/30 at w13) and `💻️os` were not re-run — every remaining slot
went to the stdio per-case loop.

---

## 4. Two third-party oracle libraries that no manifest declares

`dependency` prints exactly the wave-11 numbers. It cannot print anything else, because the two
libraries the cross-language wave started using are invisible to it.

**Pillow.** `mutate-semio-image`'s reference imports it (`🐍️component.py:520`, `from PIL import
Image`) and its rationale names *"Pillow (PIL 11.3, littleCMS)"*. Its registration
(`🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧪️oracle/🔣️.json`) declares
`"ecosystem": "python", "package": ""`. **Pillow is nowhere in `🔒️dependencies.json`** — 17 python
entries, `pypdf 6.14.2` and `simplejson 4.1.1` among them, no Pillow. The runner's venv is created
`--system-site-packages` (`📜️script.ts:381`), so:

```
$ ⚡️cache/tests/hosts/python-env-cacdfc3fbe53ad5f7f5baff78016644f/bin/python -c "import PIL; print(PIL.__version__, PIL.__file__)"
11.3.0 /Users/ueli/Library/Python/3.9/lib/python/site-packages/PIL/__init__.py
```

The oracle resolves out of **the developer's own user site-packages**. On a devcontainer, on CI, on
any second machine, that import raises `ModuleNotFoundError`. This is a zero-touch / cross-platform
violation as well as an undeclared dependency.

**three.js.** `mutate-semio-mesh`'s reference does `import * as THREE from "three"`
(`🟦️component.ts:48`); its registration declares `"ecosystem": "javascript", "package": ""`. `three`
**is** in `🔒️dependencies.json` — as a **production-reachable** js dependency of five `package.json`
files. The test platform's own `package.json` declares only `class-variance-authority`, `clsx`,
`semver`. So an oracle now reads a production library, which is exactly the `production-debt` pattern
the gate reports for `png`, `zip` and `image` — and it reports nothing, because no registration names
the package. The adapter's prose claims **three.js r185**; what resolves is
`node_modules/three@0.182.0`, r182.

57 of the 138 oracle registrations declare an ecosystem with `"package": ""`. For 55 that is the
honest "second implementation, no distribution" convention. For these two it hides a real third-party
reference from the only gate that would catch it, and there is no contract rule for *"an oracle
adapter imports a package no manifest declares."*

---

## 5. Did anyone weaken evidence? — checked eight ways, one removal, no weakening

Baseline for every diff: `9ed590cd8749af38dab141723300f9f91120cfad`.

1. **Comparison profiles.** `git diff … -- '*🔣️component.json'` filtered to lines that actually
   declare a knob (`^[+-]\s*"(ignoreKeys|tolerance|arrays|mode)"`) returns **nothing**. Repo-wide,
   every changed line mentioning `ignoreKeys` / `"tolerance"` / `"arrays"` is prose inside a
   `description` or `rationale`. `semantic-pdf-conformance-a-v1` still declares
   `"tolerance": 0, "ignoreKeys": []`.
2. **Exemption constants.** `git diff … -- '✏️s/🔌️plugins/🗄️stdio/**'` filtered to
   `const [A-Z_]*(TOLERANCE|WRITER_FREEDOM|UNOBSERVABLE|GUARD_VECTORS)` returns **nothing**.
3. **Scenarios.** 164 features before, 164 after (166 now, two added); no `D` in
   `git diff --name-status`. Per-case counts parsed with the repository's own `parseFeature`:
   **27 cases gained scenarios, zero lost.**
4. **Fixtures.** Over `*🧫️fixtures*` and `*📚️examples*`: 4 added, 69 modified, **0 deleted**; 66 of
   the 69 are `.rs` demo modules in `📕️norm`. The only non-`.rs` modifications anywhere are three
   PDF 1.4 `📚️examples/🎬️demo/🖼️assets/` files, regenerated after the 1.4 codec rewrite. **Every
   fixture a case actually reads is untouched** — `📄️bachelor-thesis.pdf`,
   `🖼️abbau-aufbau-masterarbeit-grundriss.tiff`, `🌐️zukunft-bau-entwerfen-mit-bestand.html`.
5. **Law call sites.** Over the 1,725 adapter files that existed at w13, excluding doc comments:
   **324 → 309**. Sixteen files decreased. Fifteen are `📕️norm` (−2 each) — their Rust adapters used
   to register `mutate_oracle_for` / `inverse_oracle_for`, i.e. *this repository's own answer on the
   oracle side*, and those were removed when a Python reference took the role. A strengthening.
   The sixteenth is §5c.
6. **Assertion clauses.** Every changed `Then`/`And` in a stdio feature was read. One scenario's
   claim genuinely changed (§5b). The 19 `🧿️semio` conversions moved *"the resulting snapshot matches
   the committed after-snapshot fixture"* to *"the independent implementation and the subject
   agree"* — but the committed-vector claim was **not dropped**: it moved into a new scenario per
   case, `Apply <id> to its committed specification vector`, whose `Then` is *"each reaches the
   committed after-snapshot and the two agree"*. That is why every one of those cases gained
   scenarios (`mutate-semio-brep` 27 → 40).
7. **Examples rows.** 343 removed / 574 added across stdio features. The only payload that changed
   size is PDF 1.4's `set-snapshot`, which went from a one-page `612×792` document to a **two-page**
   A4 + A5 page tree — a strengthened vector.
8. **Open-divergence paragraphs.** Three survive verbatim (`mutate-bmp-v3`, `mutate-gif-87a`,
   `mutate-gif-89a`), each still saying *"Do not weaken the profile, the row's parameters or the
   fixture to close it."* `mutate-tiff-6-0`'s now reads `✅️ CLOSED, AT THE CAUSE`, and it was:
   `TiffIfd` gained its own `pixels` field, threaded through the snapshot, the diff, both diff codecs
   and the proto/graphql/ts/json mirrors.

### 5a. The one deliberate PROJECTION change, and it is stricter

`font_program` (`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📄️document/🦀️component.rs:1299-1309`) now returns
`stream.get_plain_content()` — the **decoded** program — and reports its length **plus a digest**
(`:2127` projects `programBytes` and `programDigest`). w13 §2.2(1) asked for the decoded length; the
wave added the digest too, so a font program whose bytes change now fails even when its length does
not. All six conformance profile descriptions were rewritten to say so, including the sentence w13
called false. This is the correct shape of a deliberate projection change: stricter, and justified in
writing.

### 5b. The one scenario whose claim changed — examined hardest, and it is a strengthening

`mutate-tiff-6-0-baseline :: identity-round-trip` went from *"the re-encoded bytes differ from the
input"* to *"the re-encoded bytes reproduce the reference writer's own file exactly, flipping one
byte of the decoded raster changes them"*, and `law::reparsed_not_copied` was replaced by
`law::carrier_is_exact`. Three checks:

1. the fixture (17,473,180 bytes) is **unchanged** — last commit `3c7d3c108c` (2026-08-23 23:12),
   clean in the working tree. It was not regenerated to fit our writer.
2. its bytes are the **oracle's** own `write_tiff` output over IFDs the third-party `image` encoder
   produced from a real 2275×2560 JPEG scan and a real PNG floor plan
   (`🖼️tiff/…/✳️any/🧪️oracle/🦀️component.rs:823`, `#[ignore]`d). Byte equality against it is a
   genuine cross-writer claim.
3. the anti-passthrough tripwire is **replaced by a stronger one**, not dropped: the handler flips one
   byte of the DECODED raster, re-encodes, and requires the result to differ from the input
   (`🦀️component.rs:245-250`). A codec smuggling input bytes fails there; it would have passed
   `reparsed_not_copied` only by accident.

The old assertion could never have held — the feature's own text says `encode_tiff` regenerates
`CORE_STRIP_TAGS`, and the projection carries `stripOffsets` at tolerance 0. **Not a weakening.** One
artefact left behind: the module docstring at `:19-20` still names a law the handler no longer calls.

### 5c. The one genuine assertion removal

`mutate-block-2d-1`'s Rust adapter lost three `law::` calls. Two are restated by hand and by the new
Python reference; the third — the `.dsl.semio` **carrier** law — is genuinely no longer asserted, and
is stated as a gap in three places with its reason (the subset's `store::ArtifactDsl` impl is
handwritten `async`, the generated host is synchronous). In the same change the adapter went from
*not linking the plugin crate at all* to driving the real `block2d_mutation_report_json`. Net gain.
`semio-s-plugin-block` does not compile, so no phase of this case runs today regardless.

---

## 6. The four blockers `📓️w13-final-audit.md` named

### 6.1 Is the PDF 1.4 subset real, or removed? — **REAL, and its vectors were strengthened**

* `PdfSnapshot { schema, pages: Vec<PageDoc> }` (`…/📸️snapshot/🦀️component.rs:85-94`), replacing
  `{ schema, page: PageDoc }`.
* `decode_pdf` (`…/🚪️io/🦀️component.rs:456-478`) resolves `/Root → /Pages`, walks `/Kids`
  recursively with a `HashSet` cycle guard and real `/MediaBox` inheritance, refuses `/Encrypt`
  rather than guessing, and errors when the tree resolves to no page. `encode_pdf` (`:547`, `:559`,
  `:567`) allocates and writes **every** page.
* `pageCount` remains in both conformance projections (`✳️a/🧪️oracle/🦀️component.rs:136`,
  `✳️any:147`) — the anchor that fails again if a producer starts dropping pages.
* the honest limitation is preserved, not quietly dropped: `✳️a`/`✳️x` still report
  `stdio.pdf.{a,x}.schema-gap-unverifiable`, under a heading *"What this snapshot deliberately does
  not carry."*

**Runtime confirmation was not obtained** — `mutate-pdf-1-4-a` / `-x` are among the 77 stdio cases
the per-case loop had not reached. The code change is real; the 0/9 → n/9 number is not measured
here.

### 6.2 Does DWG read its own real fixture? — **code fixed, runtime unconfirmed**

`🔖️ac1024/…/🚪️io/🦀️component.rs:7935` now has one shared `decode_r2010_entity_common_fields`
documented with the two version-gated bits that were wrong: the R13–R2000-only `nolinks` bit must not
be consumed, and R2010 adds three visual-style presence bits after `shadow_flags` — exactly the
two-bit misalignment an underflow at entity `0x239` type `77` implies. The broken `include_str!` w13
found at `…/🧬️mutations/🦀️component.rs:163` now carries the fifth `../` and resolves.

`📓️w17-four-fixes-…`'s own second-round log shows `mutate-dwg-ac1018` at
`executed=7 passed=7 failed=0`. **In my runs both DWG cases returned
`executed=0 … not-exercised=1` with `rust subject host exited 101` — the peer's framework breakage,
not DWG.** Unconfirmed here.

### 6.3 Do the in-crate `#[test]`s run, and do the `KINDS` tests pass? — **yes; 102 of 104 KINDS pass**

They run. 5,806 executed, **5,667 pass, 136 fail, 3 ignored**.

* **`KINDS` conformance: 104 tests, 102 ok, 2 FAILED** — and the two are the `✳️baseline` subsets
  wave 12 created to drive `unregistered-mutation-vocabulary` to zero:
  ```
  artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::tests::kinds_match_enum_variants_in_declaration_order ... FAILED
  artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::tests::kinds_match_enum_variants_in_declaration_order ... FAILED
  ```
  Identical to what `📓️w19-…` measured. Neither has been fixed.
* **`fixture_honesty_law`: 50 tests, 37 ok, 13 FAILED.** This is the law that exists to stop a
  shipped example fixture drifting from the codec that claims to have produced it, and it is red for
  `bcf`, `docx`, `dwg`, `dxf`, `ifc` 2x3, `ifc` 4, `pdf` 1.7, `pptx`, `svg`, `tiff`, `xlsx`, `xml`,
  `zip`. The docx failure is a whitespace divergence inside the embedded XML parts between the
  shipped `.dsl.semio` and `demo_docx_snapshot()`. **Thirteen artifacts ship an example carrier their
  own codec does not reproduce, and nothing outside these in-crate tests looks.**
* **One test aborts the whole process** (§1.8): `a_semio_member_mints_and_reopens_a_real_child_envelope`,
  SIGABRT in a destructor — `MemberFactory` declares `create`/`open` and no wind-down, so a store it
  mints cannot satisfy `ArtifactStore::drop`'s own assertion. Diagnosed in `📓️w19-…`; unfixed.

Separately: **17 committed example assets are zero-byte files** (12 × `🎒️example.pack.semio`, plus
`🎞️example.gif`, `🎞️example.pptx`, `💬️example.bcf`, `📕️example.xlsx`, `📷️example.png`). I resolved
every `include_bytes!` / `include_str!` path in the repository: **none points at one of them**, so no
law is quietly passing on empty bytes. They are dead empty files shipped as examples.

### 6.4 The five stdio cases with no compiling subject half — **fixed**

All five adapter imports resolve, and three are confirmed by a run:
`mutate-docx-ecma-376-strict` **21/21**, `create-and-read-jpeg` **2/2**,
`create-and-round-trip-stl` **2/2**. `vml_markup` was added to both `✳️strict` production mutation
modules (`📜️docx/…:356`, `📕️xlsx/…:321`); `mutate-json-rfc8259-i-json`'s private `protocol_inverse`
was replaced by the subset's own `inverse_json_i_json_mutation`. `mutate-xlsx-ecma-376-strict` and
`mutate-json-rfc8259-i-json` were not reached by the loop.

---

## 7. Runner defects that hide evidence — two open, one new, one newly decisive

* **w13 remedy #2 — open.** A case whose host fails to build returns `{ results: [], problems }`
  (`📜️script.ts:523`, `:528`) and contributes `executed=0 passed=0 failed=0 errored=0`. During the
  peer breakage this produced fourteen stdio cases reporting a green-looking
  `parity=0/0 not-exercised=1` line while nothing had run.
* **w13 remedy #10 — open, and now decisive** (§0). It is no longer a nicety; it is why this ticket
  has no aggregate number.
* **New: `ownerShipsImplementation`** (`📜️script.ts`). A subject is dispatched only in a language the
  owner ships a `📦️packages/<lang>` directory in; otherwise the runner prints
  `[test] no-subject-implementation …` to **stderr**. The reasoning is right — a `🐍️component.py`
  under `🗄️stdio` is a reference host, and `🗄️stdio` ships only `🟦️typescript` and `🦀️rust`, so all
  43 python adapters correctly stop being asked for a subject. Measured effect: **exactly one case**,
  `extract-text-pdf-1-4`, and the live run printed exactly that line:
  ```
  [test] no-subject-implementation ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/extract-text-pdf-1-4 (adapters python host references only; this repository ships no implementation of the owner in any of those languages)
  ```
  Its two scenarios were `errored` at w13; they now report `executed=2 passed=2 parity=0/0` — the
  oracle half still counts, the missing subject appears only on stderr.
* **The guard has a hole `mutate-semio-mesh` will fall into.** Its TypeScript adapter registers
  **oracle handlers only** (`🟦️component.ts:962-968`), but `🗄️stdio` *does* ship a
  `📦️packages/🟦️typescript`, so `ownerShipsImplementation` returns true and the runner will ask that
  adapter for a subject — `validateRegistration` (`📦️index.ts:1750`) then emits
  `adapter has no subject registration for scenario …` for all **52** of its scenarios. The case was
  not reached by the loop, so this is a prediction from the code, not an observation.

---

## 8. Exemption lists — measured repo-wide for the first time

Over all 158 Rust case adapters, **151 declare no `UNOBSERVABLE` exemption at all.** Seven do:

```
100.0%   1 of 1    mutate-energy-model-1
 86.7%  13 of 15   mutate-mathematical-1
 43.8%   7 of 16   mutate-process3d-1
 16.7%   2 of 12   mutate-raster-1
 11.8%   2 of 17   mutate-png-1-2
  7.1%   1 of 14   mutate-draw-1
  2.9%   1 of 35   mutate-remodel-1
```

`mutate-energy-model-1` still exempts its **only** kind, so that plugin's entire mutation vocabulary
has no forward evidence. `mutate-process3d-1` at 43.8 % is a third one w13 did not name. All three
say so in place; no gate measures any of it. All 28 remaining `@no-oracle-` decisions declare
substitutes; none is empty.

---

## 9. Prose, similarity, stubs

* sentences over 70 characters shared by three or more features: **77, touching 109 of 164**
  (w13: 78 over 122).
* max feature-to-feature 5-gram Jaccard **0.839** (`mutate-pdf-1-4-a` / `-x`; w13 0.806); max
  case-adapter-to-case-adapter **0.939** (`mutate-procedural-2d-1` / `mutate-procedural-3d-1`) —
  **unchanged from w13, and neither header still mentions the other.**
* **The new Python oracles are 27 implementations and one 15-fold copy.** Excluding `📕️norm`,
  exactly one pair of the 43 committed `🐍️component.py` files exceeds 0.60 — `mutate-fem2d-1` /
  `mutate-fem3d-1` at 0.796, a relationship both features state. The 15 `📕️norm` oracles are
  **0.95–0.96 mutually similar**, the highest figure anywhere in this repository;
  `mutate-en1990-1/🐍️component.py` (870 lines) and `mutate-en1993-1/🐍️component.py` (884 lines)
  differ in **62 lines** once the standard's identifier is normalised away, and none of the 15
  headers mentions a sibling.
* `todo!` / `unimplemented!` in every `🧪️tests` and `🧪️oracle` tree: **zero**.

---

## 10. Coverage — what could and could not be measured

* scenarios executing in at least one phase: **not measurable this run.** It requires a completed
  `oracle` and `subject` sweep, and neither completed (§0).
* plugins that can run a subject phase: **not measurable** for the same reason.
* plugin libraries that compile: the sweep was still running at the end of this window;
  `semio-s-plugin-animate` is **57 errors**, identical to w13, and `semio-s-plugin-stdio` is
  **exit 0**. Partial results in `w16-audit/plugin-matrix.txt`.

Stating these as unmeasured is the point of the report, not a gap in it: `📓️w13-final-audit.md` was
able to quote 2,284 of 4,564 and 7 of 34 because its runs completed. Ten weeks of cache growth later,
they do not.

---

## 11. Housekeeping the ticket's own documents need

* `📋️status.md` has not been touched since `215e369d07` (2026-08-23 18:01). Its dashboard says
  **11 cases / 32 scenarios / 212 dependencies / 10 registered oracles**, and its owner table says
  *"every other non-`compose` owner: discovered / surveyed."* Live: **166 cases / 4,933 scenarios /
  232 dependencies / 138 oracle registrations across 37 owners.** A reader who starts there is off by
  two orders of magnitude. `📋️contract.md` and `📋️architecture.md` are likewise frozen at
  2026-08-23.
* `🎒️zip/🧪️tests/mutate-zip-2-0/🦀️component.rs:10-13` is the last "that blocker was cleared" line.
  It cites `cargo check -p semio-framework-os-kernel --lib` — the **default** feature set, which is
  true — but w13 §5's point was that the `sync` feature set was the broken one, and the line still
  does not say which it means.
* `mutate-tiff-6-0-baseline/🦀️component.rs:19-20` names a law the handler no longer calls (§5b).

---

## 12. What should happen next, in order of how much evidence it buys

1. **Prune the shared host target dir and stop it growing.**
   `⚡️cache/agents/local/cargo-test-hosts` is **280 GB**, corrupt, and outside `clean`'s scope. Until
   it is reclaimed this ticket cannot produce a single aggregate number. It is a generated cache
   (`CACHEDIR.TAG`, and the runner's own header calls its output "safe to delete, never commit") —
   but deleting 280 GB of build state while other sessions are live is the owner's call, not an
   auditor's, so it is recommended here and was **not** done.
2. **Bring the shared target dir into `cleanTestOutputs`' scope**, or give each host its own. One
   directory shared by ~200 generated crates is also what serialises every host build behind one
   cargo lock.
3. **Make a per-case budget overrun fail that case** (`runProbe`, `📦️index.ts:1708`) — w13 remedy
   #10. Four aborted runs in one night.
4. **Make a case whose host failed to build count as `errored`** (`📜️script.ts:523`, `:528`) — w13
   remedy #2. Fourteen cases reported a green-looking line while nothing ran.
5. **Declare Pillow, and register `three` as a test-oracle package** (§4); add a contract rule for an
   oracle adapter importing an undeclared package.
6. **Fix the 13 `fixture_honesty_law` failures** (§6.3) — thirteen artifacts ship an example carrier
   their own codec does not reproduce.
7. **Fix the two `✳️baseline` `KINDS` tests** and the SIGABRT in `MemberFactory`'s wind-down (§6.3).
8. **Re-measure `mutate-pdf-1-4-a` / `-x` and both DWG cases** once the runner can complete a sweep —
   the code fixes are real and unverified at runtime.
9. **Give `mutate-semio-mesh` a TypeScript subject or exclude TypeScript from its dispatch** (§7).
10. **Rewrite `📋️status.md`.**
