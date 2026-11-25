# 🔗️ Native test-executable link memory on macOS arm64 (2026-11-25)

Result in one line: **no link configuration brings the curation test executable under the 4096 MB final ceiling, let alone 2 GB; the linker (Apple `ld`, and `lld` just the same) needs about 6 GB for this one executable, because the executable itself carries 424 MB of unoptimised code from 714 linked crates. Nothing was changed in the profile schema or the cargo configuration, because no lever measured better than 5 percent. The `curation_unit|host` carving entry therefore stays, with its numbers re-measured (7179 / 7600).**

Benchmark: `cargo test --test curation_unit` of `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation` through `native-cargo.sh` (`repo cargo --store=shared`), 108 tests pass. Toolchain nightly-2026-07-20, Apple `ld` 27106.1, SDK 27.2, host 16 GB with peers building. Command line of the unit: `-C debuginfo=1 -C split-debuginfo=unpacked -C codegen-units=256 -C lto=off -Z threads=5`, 89 direct and 714 linked rlibs plus 258 codegen objects of the test crate.

## 1. Which process peaks

| phase of the unit (one `rustc` tree) | process | resident | footprint |
|---|---|---|---|
| codegen of the test crate (`-Z time-passes`: `codegen_crate` +2086 MB, `LLVM_passes`) | `rustc` | 3.2 to 3.5 GB | about 3.8 GB |
| link (rustc keeps its state: 2.3 GB resident while it waits) | `rustc` | 2.3 GB | |
| link | `clang` driver | 11 MB | |
| link | `/Library/Developer/CommandLineTools/usr/bin/ld` | 5.3 GB (sampler, 1 Hz), 6.06 GB (link-only replay) | 6.05 GB |
| whole unit as the warden measures it (`footprint` of the subtree, includes compressed and swapped pages) | `local-unit` | | 7114 (cancelled at its 7100 budget), 7179 (reported), 8216 and 8324 (cancelled, two replays under host pressure) |
| whole unit as the unit cache measures it (`maxRSS`, `peakRssBytes`) | | 5757 MB | |

So the linker child is the peak; `rustc`'s codegen is second and fits. `unit-cache` rusage and the warden disagree by 1.4 GB because only the warden counts what the OS compressed or swapped out of the tree: the warden number is the one that matters for the host.

Output of the link: `curation_unit-<hash>` 1.74 GB: `__TEXT` 543 MB (`__text` 424 MB, `__eh_frame` 85 MB, `__unwind_info` 11 MB), `__LINKEDIT` 1.27 GB, no DWARF in the file. Inputs: 916 rlibs in the lane, 9.8 GiB, of which 714 are linked.

## 2. Does the warden see the linker (kind and identity root)

Yes, no fix needed, three independent proofs:

1. By the law: the compiler is one subject (`local-unit`, pattern on `rustc`, root = the pinned Bun of the unit-cache wrapper above it, or identity = the pinned toolchain path); its memory is the footprint of its **subtree** (`observeTreeMegabytesV1`: `footprint -p` of every descendant on macOS, `smaps_rollup` on Linux), and a cancel signals the **tree** parent first. `cc`/`clang` and `ld` are descendants, match no kind of their own and so are never double counted.
2. By test: the existing classification fixture "megabytes: a subject holds itself and every process whose nearest subject it is" (rustc 200 + cc 100 + ld 50 = 350 MB) and the new test `the linker of a compile unit belongs to its compiler` in `🛡️warden/🧪️tests/🟦️.ts` (real macOS command lines: `rustc ... --test`, `cc`, `…/CommandLineTools/usr/bin/ld`; one subject, megabytes include the linker, the tree signal plan is `[rustc, cc, ld]`, the ceiling of a `--test` unit is the final one).
3. Live (`🔗️warden-tree-probe.ts`): chain `rustc 41685 → clang 42246 → ld 42249`; at the first sample of the linker (ld 321 MB footprint, 727 MB resident) the warden's own tree observation of the compiler reported 2589 MB, i.e. compiler plus linker; at its peak the unit reached the 7114 to 8324 MB above, which no `rustc` alone can reach.

One gap, not a defect of the path the build takes: a linker started by hand (as the link-only experiments here did) has no `rustc` or Bun ancestor and is not a subject; the experiments therefore carried their own footprint guard (kill above 7000 MB).

A stale oracle was found and fixed on the way: the admission ceiling (`🏔️ceiling`) treats `bin`, `dylib`, `staticlib` and a `--test` executable as `final` (4096), the warden's Python oracle still said only `cdylib`; `🛡️warden/🧪️tests/🐍️.py` now implements the same law and the stored classification expectation was rewritten by it (2 lines: `build_script_build|host` kind final, 4096).

## 3. Levers measured (before: 6052 MB footprint of the linker tree, link-only replay of the very same objects and rlibs, 41 s)

| lever | where | footprint of the linker tree | verdict |
|---|---|---|---|
| `-C debuginfo=line-tables-only` for the test crate only | rustc flag | ld unchanged (5353 vs 5343 MB RSS), unit maxRSS 4915 vs 5757 | rustc side only |
| `-C debuginfo=line-tables-only` for **all** 763 units (schema `prototype-native.debug`, implemented, tested, then reverted) | profile | unit maxRSS 5815 vs 5757, rlibs 9.78 vs 9.84 GiB | no effect, reverted |
| `-Wl,-S` (no debug map) | link arg | 5831 | -4 percent, **breaks file:line backtraces** |
| `-Wl,-no_deduplicate` | link arg | 5929 (full replay), 5021 RSS | -2 to -6 percent |
| `-Wl,-no_function_starts` / `-no_fixup_chains` / `-no_uuid` / `-no_dead_strip_inits_and_terms` | link arg | 6028 / 6045 / 6046 / 6043 | none |
| `-Wl,-ld_classic` | link arg | 6053 | accepted, same linker |
| `-C link-dead-code=yes` (no `-dead_strip`) | rustc flag | link fails | unusable |
| `split-debuginfo=unpacked` (already the default) | profile | | no DWARF in the output, nothing to gain |
| `lld` through the pinned toolchain (`-C linker-features=+lld -C link-self-contained=+linker`, same as the Linux entry) | rustflags | **does not link on this host**: `rust-lld: could not load TAPI file .../usr/lib/libSystem.tbd: unknown architecture arm64e.x1-macos` (SDK 27.2 is newer than the pinned LLVM) | blocked |
| `lld` with a copy of the SDK's `.tbd` files without the `arm64e.x1-*` targets | experiment only | 5725; `--threads=1` 5767; `--threads=1 -S` 5581 | links, but **no memory gain** |
| `codegen-units`, incremental, `lto` | profile | not measurable without rebuilding the whole tree (about 40 min per run); only about 10 percent of the symbols are duplicated across codegen units (`llvm-nm` of the six largest rlibs), so at most that | not pursued |

Conclusion: the linker memory follows the amount of code that is linked (424 MB of text after dead stripping, from 714 crates), not the linker, the debug information or the flags. Both linkers land at 5.6 to 6.0 GB. A configuration that gives 2 GB does not exist on this host.

## 4. What changed

- `🛡️warden/🧪️tests/🟦️.ts`: new test (linker belongs to its compiler); `🐍️.py` and `🧫️fixtures/🔣️.json`: oracle law for linked units, expectation rewritten by the oracle. Warden suite 46 pass, 0 fail.
- `🎟️admission/🧬️schema/🔣️.json`: `curation_unit|host` re-measured, `measuredMegabytes` 7179, `budgetMegabytes` 7600 (+5.9 percent, below the 60 percent cap share). Law test `🧢️unit-ceiling` 11 pass. **The entry is not removed**: the unit is far above 4096.
- Profile schema, profile TS and `.cargo/config.toml`: **unchanged** (the debug-level change was made, measured, and reverted). Profile tests 11 pass, consumption tests 26 pass.
- Tools of this investigation (kept, ticket folder): `🔗️link-sampler.sh` (1 Hz process memory), `🔗️link-hold.sh` (captures the compiler argv and keeps its build lane alive with a marker), `🔗️link-variant.sh`, `🔗️link-run.sh` (full replays through the capacity gate), `🔗️link-only.py`, `🔗️link-only-run.sh` (replays only the linker command with a footprint guard), `🔗️warden-tree-probe.ts`.

## 5. What would work (not done, needs a decision)

1. Make the executable smaller, which is carving by dependency, not by test count: the test target links the whole stack (`semio_plugin_*`, `semio_os_*`, stdio members). Splitting the 108 tests into five targets made it worse (five links, 5.9 to 6.8 GB each); the lever is a test target whose closure is a fraction of it (tests of the domain/operations crates against fakes instead of the full app), because link memory is about 14 bytes per byte of linked code.
2. Link the dependency stack once as a dynamic library for tests (`-C prefer-dynamic` for workspace crates): large change, unmeasured.
3. `opt-level=1` for the dependency crates (`[profile.*.package."*"]`) would shrink code size, at the compile-time cost the prototype profile deliberately avoids: unmeasured.
4. When the pinned LLVM parses SDK 27 `.tbd` files `lld` becomes selectable, but it measured no better than `ld`.

## 6. Incident notes

- The disk (25 GB free) was filled to zero once by a `cp -cR` of a 20 GB build lane (the copy ran out of space instead of cloning); removed immediately, nothing else was lost. Do not copy lanes; keep one alive with a marker `<lane>/v/<pid>-<name>` (as `🔗️link-hold.sh` does) and delete it after use.
- A peer's temporary `dbg` module (`#[path=…/GET-PROCESS-WORKING-END-TO-END/🗑️generated/debug-ledger.rs]` in three framework sources) broke every build for about 25 minutes after that ticket deleted its generated folder; it was removed by its owner meanwhile.
