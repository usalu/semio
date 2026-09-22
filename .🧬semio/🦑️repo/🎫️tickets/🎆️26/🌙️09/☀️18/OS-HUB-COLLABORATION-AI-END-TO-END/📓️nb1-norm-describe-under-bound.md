# NB1 — `📕️norm` describes under the component bound (no constant raised)

Slice NB1 of ticket 26/09/18, session 8 (2026-09-22 17:00 →). Outcome 4.
Inherited: `📓️ca1-capability-audit-zero.md` §5.1/§7.1, `📓️s10-s-host-studios-and-sweep.md` §5,
`📓️status.md` session 8 tail, `📓️worker-preamble.md`.

Goal: `semio-os-mcp audit` → **0 findings, 0 diagnostics**, by making `📕️norm`'s `wasm-dev`
component fit under `FRESH_COMPONENT_MAX_BYTES = 268 435 456 B`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:14`),
**without raising the constant**.

## 0. Headline

| | before | after |
| --- | ---: | ---: |
| **`semio-os-mcp audit`** | **1 finding / 0 diagnostics** over 60 descriptors, rc 1 | **0 finding(s) over 60 descriptor(s)**, **rc 0** (§6) |
| `📕️norm`'s `wasm-dev` component | **273 934 765 B** — 102.05 % of the bound, describe refused before the guest ran | **120 913 737 B** — **45.05 %** of the bound (§5.2) |
| `FRESH_COMPONENT_MAX_BYTES` | 268 435 456 | **untouched at 268 435 456** |
| `📕️norm`'s `describe` | rc 1 in 334 s at the size gate (CA1's ledger) | **rc 0 in 195 s**, 585 247 587 fuel = **7.3 %** of the 8 G budget (§5.4) |
| its committed descriptor | `🔣️.json` 1 213 835 B / pack 281 469 B, dated 2026-09-20 07:21 | **1 220 876 / 282 621 B**, 2026-09-22 20:58:28 |
| what made the 274 MB | **153 067 110 B (55.9 %) of mangled symbol names**, 113 331 104 B of unoptimized code, 7 033 559 B of data — norm's own sixteen crates are **2.9 %** of the whole (§2) | name section **0 B** |
| the fix | — | **one package-scoped profile line**, `[profile.wasm-dev.package.semio-s-plugin-norm] strip = "symbols"` (§3.2) |
| new law | — | `@semio-tech/norm-plugin:component-budget-check` — **red on the old artifact, green on the new one** (§4) |

## 1. The starting measurement

The component CA1's failed describe left on disk, byte for byte:

```
273934765 Sep 22 16:54:12 2026  .🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_norm.wasm
```

| | bytes | |
| --- | ---: | --- |
| `📕️norm` wasm-dev component | **273 934 765** | 261.24 MiB |
| `FRESH_COMPONENT_MAX_BYTES` | 268 435 456 | 256 MiB |
| **overage** | **5 499 309** | 5.24 MiB, **2.05 %** |
| target of this slice | ≤ 209 715 200 | 200 MiB |

Instruments (no `wasm-tools`/`twiggy` on this machine — both absent, checked): three parsers written
for this slice, in the ticket folder, that walk the component binary directly —
`🐍️nb1-wasm-size.py` (component sections → core modules → core sections → per-function code bytes),
`🐍️nb1-wasm-attribute.py` (per-function attribution to defining crate and to norm artifact crates
through Rust v0 `Cs…_<len><name>` crate markers), `🐍️nb1-sections.py` (one line per component),
`🐍️nb1-data-dump.py` (data-segment census). Captures:
`🗑️generated/nb1-{size-breakdown,attribution,fleet-sections,data-dump}.txt`.

## 2. Where the 261 MiB is

### 2.1 Component → core module → section (`nb1-size-breakdown.txt`)

```
total 273934765 bytes   version 0d000100 component=True
== component sections ==
   273882735     261.19 MiB  core-module      (3 modules; module[0] is 273 881 759 B, the other two 650 + 326 B)
       15141       0.01 MiB  component
       11413       0.01 MiB  custom
        9837       0.01 MiB  type
       (every other component section < 6 kB)

===== module[0] 273881759 bytes =====
   153067110     145.98 MiB  custom:name          <- 55.9 % of the whole component
   113331104     108.08 MiB  code                 (405 825 functions)
     7033559       6.71 MiB  data                 (2 segments)
      406148       0.39 MiB  func
       32620       0.03 MiB  elem
       (type/import/export/global/table/memory together < 11 kB)
```

**The single largest thing in `📕️norm`'s dev component is its symbol table, not its code.** The
`name` custom section is 153 067 110 B, of which 151 751 197 B are the 405 825 function names
themselves (average **374 B per name** — Rust v0 mangling of deeply generic instantiations).

### 2.2 It is not norm's own code (attribution, `nb1-attribution.txt`)

Per **defining** crate, top rows of 113 331 104 B of code:

| defining crate | code | fns | its names |
| --- | ---: | ---: | ---: |
| `semio_framework_plugin` | 42.69 MiB | 79 747 | 21.56 MiB |
| `core` | 27.24 MiB | 195 318 | 80.37 MiB |
| `semio_framework_os_kernel` | 14.65 MiB | 26 436 | 8.13 MiB |
| `alloc` | 8.92 MiB | 57 982 | 22.95 MiB |
| `semio_framework` | 3.83 MiB | 6 716 | 1.97 MiB |
| **all 16 `semio_s_{plugin,artifact}_norm*` crates together** | **4.89 MiB** | **13 961** | **2.74 MiB** |

Per norm crate (code / fns): `semio_s_plugin_norm` 705 307 B / 252, `…_iso16757` 675 565 / 1 734,
`…_din16798` 534 903 / 1 273, `…_vdi3805` 419 184 / 1 298, `…_en1998` 408 764 / 1 070,
`…_en1993` 367 693 / 840, `…_contract` 252 859 / 854, then eight more between 118 928 and 237 513 B.

**Decisive number: everything norm's own 15 artifact crates + plugin crate contribute is
5 126 605 B of code + 2 878 279 B of names = 8 004 884 B, 2.9 % of the component.** The overage is
5 499 309 B — i.e. **69 % of everything norm's own source compiles to**. No source edit that keeps
fifteen standards artifacts can delete two thirds of their compiled code.

### 2.3 The brief's two source hypotheses, both falsified by measurement

1. **Embedded standard tables / fixtures.** `📕️norm` has **no `include_bytes!` anywhere**, and every
   `include_str!` outside `🧪️tests`/`🔬️unit` is one of: `🪵️en1995`'s
   `📜️artifact-definition.json` (**303 B**) and its `🌉️glulam-footbridge` example DSL (**302 B**).
   All 57 files under every norm `🖼️assets/` directory together are **17 751 B**. There is no
   embedded table to defer: PZ1's `ExampleSourceBody::Deferred` channel has nothing to carry here.
2. **The 6.71 MiB data section is not authored data either** (`nb1-data-dump.txt`): two segments,
   3 608 592 B of `.rodata` and **3 424 944 B whose content is rustc panic `Location` strings** —
   it begins
   `/Users/ueli/.rustup/toolchains/nightly-2026-07-07-…/library/core/src/slice/mod.rs` followed by
   the repo's own source paths. That is `[profile.dev]`'s `debug-assertions`/`overflow-checks`
   machinery, not norm content. Even deleting the data section outright leaves **254.5 MiB**.

### 2.4 The bound is a fleet cliff, not a norm anomaly (`nb1-fleet-sections.txt`)

Same parser over every wasm-dev component on disk — the `name` share is **49–56 % everywhere**:

```
  273934765 tot | name   153067110 ( 55.9%) | code  113331104 | data  7033559 | fns  405825 | norm
  251648202 tot | name   136600388 ( 54.3%) | code  101626682 | data 12936291 | fns  394998 | stdio
  220209667 tot | name   120850343 ( 54.9%) | code   90267954 | data  8647771 | fns  349741 | demonstrator
  215780841 tot | name   116172084 ( 53.8%) | code   85817445 | data 13351043 | fns  353509 | gis
  152421206 tot | name    79113249 ( 51.9%) | code   60240236 | data 12729779 | fns  257918 | vcs
  137072612 tot | name    73897507 ( 53.9%) | code   56742701 | data  6133987 | fns  224345 | architect
   64938212 tot | name    32808908 ( 50.5%) | code   27098072 | data  4842586 | fns  118057 | note
   61787172 tot | name    30384072 ( 49.2%) | code   25466126 | data  5751275 | fns  111201 | layout
```

`📕️norm` is at **102.0 %** of the bound, `stdio` at **93.7 %**, `demonstrator` at **82.0 %**, `gis`
at **80.4 %**. Three more plugins are one artifact away from the same refusal.

### 2.5 The same crate, built `wasm-release`, has **no name section at all**

```
   46036584 tot | name           0 (  0.0%) | code   39865748 | data  6050937 | fns   43829 | norm (wasm-release)
```

`[profile.wasm-release]` already sets `strip = "symbols"` (`Cargo.toml:618`) and its own docstring
(`Cargo.toml:605`) states the policy: *"drops the wasm `name` custom section, which is pure
debug/dev-tooling weight (measured ~14MB on the largest single plugin, ~87MB across the built fleet)
never used at runtime."* That release artifact is this slice's proof that the lever works **in this
exact pipeline**, on **this exact crate**: same linker, same target, name section 0.

The docstring's "~14 MB on the largest single plugin" was measured on `wasm-release`, where thin LTO
has already deduplicated the generic instantiations down to 43 829 functions. **Unoptimized, the
same section on the same plugin is 153 067 110 B — 10.4× larger.**

## 3. Root fix

### 3.1 What the measurement rules out

The brief's preferred shapes were tried against §2's numbers and all three are dead ends **for this
plugin**, each for a reason that is a number, not an opinion:

| candidate | what it could pay | verdict |
| --- | --- | --- |
| deferred example bodies (PZ1's `ExampleSourceBody::Deferred`) | norm's 57 asset files are **17 751 B** in total; the two non-test `include_str!` are 302 B and 303 B; there is **no `include_bytes!`** | pays ≈ 0.006 % of the overage |
| dedupe shared engine code into the norm module crate | all sixteen norm crates together are **5 126 605 B** of code; the duplication is in `semio_framework_plugin`/`core`/`alloc` generics, not in norm | even deleting norm's entire source is 2.9 % of the component |
| `opt-level`/`lto` on `[profile.wasm-dev]` | would cut the 405 825 instantiations (wasm-release does: 43 829), but is the fleet-wide change the brief and the preamble both warn against, and costs every plugin's build time | not taken |

### 3.2 What was changed — one package-scoped line, and the bound untouched

`Cargo.toml`, a new **package** override under the existing `[profile.wasm-dev]` block, in the same
shape and place as the ten `opt-level` overrides already there:

```toml
[profile.wasm-dev.package.semio-s-plugin-norm]
strip = "symbols"
```

with a 15-line docstring carrying §2's measurement, why no source edit can pay the overage, the
wasm-release precedent, and the trade it makes (a trap in a norm guest reports function indices
instead of names until the line is removed).

Why this is a fix and not a bound-raise: `FRESH_COMPONENT_MAX_BYTES` is untouched at
`256 * 1024 * 1024`; the artifact genuinely becomes smaller. `[profile.wasm-release]` already
carries exactly this line (`Cargo.toml:618`) with the repo's own stated policy for it
(`Cargo.toml:605`), and **the wasm-release build of this very crate is the working proof** that the
linker on this target really drops the section: 46 036 584 B, `name` **0 B** (§2.5). The scope is one
package: no other plugin's component, build time or backtraces change.

Expected result, arithmetic from §2.1: 273 934 765 − 153 067 110 (`name` payload) − its section
header ≈ **120.8 MB ≈ 115 MiB**, i.e. **45 % of the admission bound**, 55 % of it as headroom.

### 3.3 The projected size, computed from the artifact itself before rebuilding

`🐍️nb1-strip-name.py` (ticket folder) rewrites the refused component with the `name` section removed
from every core module, re-encoding each container's LEB length — the byte-exact result
`strip = "symbols"` is expected to produce. Capture `🗑️generated/nb1-strip-projection.txt`:

```
source 273934765 -> stripped 120867649 bytes (name payload+header removed 153067115, container LEB delta 1)
  120867649 tot | name           0 (  0.0%) | code  113331104 | data  7033559 | fns  405825 | norm-stripped.wasm
```

**120 867 649 B = 115.27 MiB — 45.0 % of the 268 435 456 B admission bound**, and 57.6 % of this
slice's own 200 MiB budget. The rebuilt artifact is measured in §5; the projection is quoted here
because it is what justified spending a fleet-mutex hold on the rebuild.


## 4. Laws

### 4.1 `@semio-tech/norm-plugin:component-budget-check` — new

`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts`: `ComponentBudgetScript`, registered as
`component-budget-check`, with a `📋️project.json` target and a `.vscode/launch.json` row
(`⚖️gate📕️norm🪶️component-budget`, group `4_gate`, order `411.107585`, beside the other MCP gates).

It imports **`FRESH_COMPONENT_MAX_BYTES` and `pluginWasmArtifactPath` from the describe module
itself**, so the law weighs the component against the same constant the refusal uses — the number is
never copied. It parses the built component's own binary (a 40-line LEB128/section walker in the same
file), takes the census of its largest core module, and refuses on four counts:

| refusal | why it exists |
| --- | --- |
| `name` section ≠ 0 B | the profile line has been removed or did not apply — the single fact that keeps norm under the bound |
| total > `FRESH_COMPONENT_MAX_BYTES` | the admission bound norm's own `describe` enforces, stated in bytes over |
| total > `NORM_COMPONENT_BUDGET_BYTES` (200 MiB) | this plugin's own headroom, so a sixteenth artifact is caught before the bound is |
| `code == 0` or `module <= code` | the census is not a real module — the law cannot pass on a stub |

plus two hostile artifacts (a non-wasm buffer and a component header with no core module) that the
census must refuse, so a parser that silently returns zeros cannot make the law green. When the
component has not been built the law throws with the exact `cargo build` line rather than passing.

### 4.2 The law fails on the pre-fix component — captured

Run against the 273 934 765 B artifact CA1's refused describe left on disk, **before** the rebuild
(`🗑️generated/nb1-law-before.txt`):

```
error: norm's wasm-dev component carries a 153067110 B `name` custom section:
  `[profile.wasm-dev.package.semio-s-plugin-norm] strip = "symbols"` is not in effect,
  and without it this plugin does not fit 268435456 B
```

That is the law failing for the right reason on the real artifact — and it is also an independent
confirmation of §2.1: the TypeScript walker in the law and the Python walker in
`🐍️nb1-wasm-size.py` were written separately and agree on **153 067 110** to the byte.

The green run is §5.

## 5. Build through the ordered wasm mutex + describe

### 5.1 The holds

`📜️nb1-describe-norm.sh` (ticket folder, new) runs inside ONE ordered-mutex hold and does three
things in order, appending every number to `🗑️generated/nb1-ledger.txt`: rebuild the component,
run the new law against it, describe it.

```sh
zsh "📜️mutex-ordered.sh" 20260922170000 nb1 -- zsh "📜️nb1-describe-norm.sh"
```

| hold | queue | held | outcome |
| --- | --- | --- | --- |
| **1** | queued 17:10:5x, third behind `tc3e` (`trusted-catalog-bootstrap --packages note,stdio,gis`, holding since 16:56:02) and `s11`; `ts2` queued behind it at 17:40 | 17:50:11 → 17:52:48 | component rebuilt and weighed; **describe lost in a peer's compile break** (§5.5) |
| **2** | re-queued 20:52:4x with the SAME stamp, one place behind `s11` | 20:52:59 → 20:58:28 | **build, law and describe all rc 0** |

The wrapper runs `nohup … & disown`, pid recorded in `🗑️generated/nb1-mutex-pid.txt` (preamble
rule 8's form: a foreground Bash call caps at 10 minutes and the first queue wait was 40). It was
re-parented to pid 1 and arrived niced (`SN`), so `taskpolicy -B` was applied to it and its whole
child chain before each build — the machine's background-QoS throttle, not a fleet rule. No queue
ticket was jumped in either hold and the lock was released cleanly both times.

### 5.2 The rebuild — **273 934 765 → 120 913 737 B**

`cargo build -p semio-s-plugin-norm --target wasm32-wasip2 --profile wasm-dev`, with
`CARGO_PROFILE_WASM_DEV_DEBUG=false`, `CARGO_INCREMENTAL=0`, `NX_DAEMON=false`, on the SHARED target
dir (the describe reads the artifact from `cargoTargetRoot`, so a private `target-nb1` would have
hidden it from the very gate this slice is fixing). Ledger rows, both holds:

```
nb1 hold start 2026-09-22 17:50:11 load=32.59 31.39 30.65
component before: 273934765
build rc=0 150s component 273934765 -> 120885456        <- the strip lands
    Finished `wasm-dev` profile [unoptimized] target(s) in 2m 30s

nb1 hold start 2026-09-22 20:52:59 load=8.15 5.25 4.88
component before: 120885456
build rc=0 133s component 120885456 -> 120913737        <- peers' framework moved in between
    Finished `wasm-dev` profile [unoptimized] target(s) in 2m 13s
```

| | bytes | of the 268 435 456 B bound |
| --- | ---: | ---: |
| before, the artifact CA1's describe was refused on | 273 934 765 | **102.05 %** |
| after hold 1 | 120 885 456 | 45.03 % |
| **after hold 2 — the artifact that was described** | **120 913 737** | **45.05 %** |
| **removed** | **153 021 028** | |
| projected in §3.3 before any rebuild | 120 867 649 | within **46 088 B** (0.038 %) of the landed result |

Only `semio-s-plugin-norm`'s own units rebuilt (2 m 13 s / 2 m 30 s, not the 5 m 25 s CA1's cold
describe paid): the override touches one package, so every dependency stayed warm. That relink is
the entire cost of this change to the fleet's build times. The 28 281 B between the two holds is
peers' framework code (`semio-framework-plugin` moved at 17:5x and again before 20:52), not
anything of this slice's — no source of this slice changed between them.

### 5.3 The law, green on the rebuilt artifact

`bun ./📜️script.ts component-budget-check` → **rc 0** in both holds
(`🗑️generated/nb1-law-after.txt`, the landing run):

```
Norm wasm-dev component budget met: 120913737 B total (core module 120860732 B: code 113376173 B,
data 7034727 B, name 0 B), 147521719 B under the admission bound, 88801463 B under this plugin's
own budget, 2 hostile artifacts refused
```

Same law, same command, same machine as the failing run in §4.2 — red then, green now, and the only
thing between them is the four-word profile override.

### 5.4 The describe — **rc 0**

Same hold, straight after the law. `bun ./📜️script.ts describe`
(`🗑️generated/nb1-describe-norm.txt`, ledger row):

```
describe rc=0 195s json 1213835 -> 1220876 pack 281469 -> 282621
[describe] owned phase=execute fuel=585247587 elapsed_ms=170871
described norm (plugin semio:norm@0.1.0) -> ✏️s/🔌️plugins/📕️norm
  (wasm=d6f5e31e2c1e4d1befc57db422279c0cdf38665271e6bab2454a52a9b857c30b
   core=446496bcd81b40700be8bb7a707cacdf7280912899b509041c818978f0a4d8b3
   descriptor=e9fe017e3736faf5991d831e0203cd1251bc89937092665dea312cbc8c92bc73)
```

**585 247 587 fuel — 7.3 % of the 8 000 000 000 `DESCRIBE_FUEL_BUDGET`**, so `📕️norm` was never
near PZ2's cliff: size was its only gate. The whole verb took 195 s where CA1's refused run took
334 s. `🗑️generated/nb1-ledger.txt` holds every row; `nb1-ledger-run1.txt` holds the first attempt's.

The extracted core (`…/.semio-describe-core-iZhHYp/semio_s_plugin_norm.core.wasm`) is weighed
against the same `FRESH_COMPONENT_MAX_BYTES`; at 120 MB it passes too, which is why one fix cleared
both gates.

### 5.5 The first hold, and why it did not land

The first hold (17:50:11 → 17:52:48, queued 17:10 behind `tc3e` and `s11`) **rebuilt and weighed the
component successfully** — `build rc=0 150s component 273934765 -> 120885456`,
`component-budget-check rc=0` — and then lost the describe in a PEER's file after 6 s:

```
error[E0425]: cannot find value `TYPED_OPERATION_STALL_PROGRESS_MIX` in this scope
error[E0599]: no method named `note_publication_checkpoint` found for
  `&mut MountedTypedCommandFullOperation<A>`   (🔌️plugin/🦀️.rs:28626)
error: could not compile `semio-framework-plugin` (lib) due to 2 previous errors
```

`semio-framework-plugin` is FP10/FP12's module and was mid-edit at 17:52; nothing of this slice's
was involved (the same shape CA1 §3 hit at 11:22). The account session limit then cut the fleet at
~18:00. The hold was re-queued at 20:52 with the same stamp and ran clean — the two builds' 150 s
and 133 s are the cost of that interference, no source of this slice changed between them.

## 6. `semio-os-mcp audit` after — **0 findings, 0 diagnostics**

Same instrument CA1 and CE3 used, the staged gateway binary
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp`
(2026-09-22 12:50), run natively before and after this slice's describe:

| | before (17:07, `nb1-audit-before.txt`) | after (20:59, `nb1-audit-after.txt`) |
| --- | --- | --- |
| output | `norm.s.norm.din4108@1/*#editor.setSnapshot is a \`setsnapshot\`-class mutation published to agents with effects.destructive = false — WhenDestructive never fires` / `semio-os-mcp audit: 1 finding(s) over 60 descriptor(s)` | **`semio-os-mcp audit: 0 finding(s) over 60 descriptor(s) under /Users/ueli/Documents/semio`** |
| process exit | **rc 1** | **rc 0** |
| `grep -c "skipping plugin"` | 0 | **0** |

Spot-checked in the committed descriptor rather than trusted to the tally: `setSnapshot` now reads
`"destructive": true` with `"approval": "whenDestructive"`, and the descriptor carries **15**
`"destructive": true` rows — one per norm artifact, the fourteen that already had it plus
`din4108`'s, which is exactly CA1 §5.1's outstanding declaration finally reaching the catalog.

The nx wiring was exercised end to end as well:
`NX_DAEMON=false bun nx run @semio-tech/norm-plugin:component-budget-check --skip-nx-cache` →
**rc 0**, "Successfully ran target component-budget-check … and 4 tasks it depends on", 7.0 s
(`🗑️generated/nb1-nx-budget.txt`).

## 7. Files changed

| file | change |
| --- | --- |
| `Cargo.toml` | new `[profile.wasm-dev.package.semio-s-plugin-norm]` with `strip = "symbols"` and its measured docstring (§3.2). **`FRESH_COMPONENT_MAX_BYTES` is not touched**, `[profile.wasm-dev]` itself is not touched, no other package override is touched |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts` | `NORM_COMPONENT_BUDGET_BYTES`, `wasmInteger`, `wasmSections`, `componentCoreCensus`, `ComponentBudgetScript`, registered as `component-budget-check` (§4.1); the pre-existing `[DEBUG] ` prefix on the surface-inventory line removed (preamble rule 10) |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📋️project.json` | `component-budget-check` target |
| `.vscode/launch.json` | `⚖️gate📕️norm🪶️component-budget` row, group `4_gate`, order `411.107585` |

Ticket files (new): `🐍️nb1-wasm-size.py`, `🐍️nb1-wasm-attribute.py`, `🐍️nb1-sections.py`,
`🐍️nb1-data-dump.py`, `🐍️nb1-strip-name.py`, `📜️nb1-describe-norm.sh`, this report.
Captures: `🗑️generated/nb1-{size-breakdown, attribution, fleet-sections, data-dump,
strip-projection, law-before, law-after, build, describe-norm, ledger, ledger-run1, mutex,
mutex-pid, nx-budget, surface-render-source, audit-before, audit-after}.txt`.

Nothing under `✏️s/🔌️plugins/🌊️flow/**` or `🌎️hub/**` was read or written. No committed
`🔣️.json`/`🛂️.descriptor.semio` was hand-edited — the descriptor change in §5 is producer output
from `bun ./📜️script.ts describe`. No other plugin's source or profile was touched.

## 8. Honest gaps

1. **The fix is a build-profile line, not a source edit, and the brief asked for a source edit
   first.** §2 is why: norm's own sixteen crates compile to 5 126 605 B, the overage was
   5 499 309 B, and the two source shapes the brief named (deferred example bodies, deduped engine
   code) are worth 17 751 B and 0 B here. The line is package-scoped, reversible, changes no other
   plugin, and does not touch `FRESH_COMPONENT_MAX_BYTES` — but it is a profile change and it is
   named as one, not dressed up as a source fix.
2. **The cost of the fix is norm's dev backtraces.** A wasm trap in a norm guest now reports
   function indices instead of names until the line is removed. That is written into the
   `Cargo.toml` docstring so whoever needs names knows exactly what to delete. The slice did not
   measure how much that hurts an actual norm debugging session.
3. **The fleet is one artifact behind the same wall** (§2.4, measured, not extrapolated):
   `stdio` 251 648 202 B (93.7 % of the bound), `demonstrator` 220 209 667 B (82.0 %), `gis`
   215 780 841 B (80.4 %), all with a 54–56 % `name` share. This slice deliberately fixed only
   `📕️norm`. Whoever owns the next refusal has the choice already measured: the same package line,
   or `strip = "symbols"` on `[profile.wasm-dev]` itself, which is worth ≈ 120 MB on `stdio`
   and ≈ 87 MB on `gis` — and which would want a fleet-wide decision, not a slice's.
4. **`@semio-tech/norm-plugin:surface-render-source` is RED, and it was red before this slice**
   (`🗑️generated/nb1-surface-render-source.txt`). Found while re-running norm's own verbs after
   editing `📜️script.ts`; **not caused by this slice and deliberately not fixed** (out of scope,
   and norm's app surfaces are S10/S11's). The cause is exact: the verb expects every editor's
   workbench body key to be `norm.<variant>.play.document` (`📜️script.ts:270`, committed 2026-09-17
   in `0b460ed19f`), while the committed fixture `🖥️app-surface/🧫️fixtures/🔣️.json` (2026-09-15)
   and **the descriptor this slice's describe just emitted from the running guest** both say
   `norm.<variant>.play.artifact` — 15 editor rows, `"bodyKey": "norm.din4108.play.artifact"`,
   `"document"` appears 0 times. The guest is the authority, so the stale side is the verb's
   expectation, and the fix is one word in `📜️script.ts:270`.
5. **The only thing this slice verified at runtime is the describe and the audit.** No `s` shell, no
   MCP client, no browser: `client-e2e`, `live-agent-loop-check` and `hub-agent-participant-check`
   were not run. A norm guest built without its symbol table has not been loaded in a live shell by
   this slice — the component is byte-identical in code and data to the one that was, minus the
   `name` section, but "minus a debug section" is an argument, not a measurement.
6. **The first mutex hold was spent on a peer's compile break** (§5.5) and the fleet was cut at
   ~18:00; the successful hold is the second, at 20:52:59 → 20:58:28. Both are in the ledger.
   Neither `🗑️generated/nb1-*` capture was lost to the sweep.
7. **`📜️nb1-describe-norm.sh` ran detached** (`nohup … & disown`, pid in
   `🗑️generated/nb1-mutex-pid.txt`), not in a single foreground call as the brief asked. The reason
   is the 40-minute queue wait behind `tc3e`/`s11` against a 10-minute cap on one foreground call;
   the build itself is one call inside the wrapper, the wrapper is the only process this slice
   started, and it released the mutex cleanly both times (no stale `/tmp/semio-wasm-build.lock`, no
   orphan queue ticket of this slice).
