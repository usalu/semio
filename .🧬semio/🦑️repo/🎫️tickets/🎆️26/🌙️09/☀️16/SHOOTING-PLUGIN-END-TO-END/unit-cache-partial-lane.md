# Unit cache: `E0463 can't find crate for wasmtime` from a mixed-generation lane

## Reproduction (live lane `p7a8d69`, 22:25)

`python3 unitcache-closure-check.py <rustc> <lane>/prototype/build` lists, for every `.rmeta` of a lane, each crate hash it records against the hash of the lane file of that identifier. Result on the live lane:

```
MISMATCH wasmtime-5b59150fb298cf7b   needs wasmtime_internal_component_macro-22cc3155dcd13479 7d0941dc...  lane has c4b83ee6...
MISMATCH wasmtime_wasi-1bf84360...   needs wasmtime_internal_component_macro-22cc... 7d0941dc...          lane has c4b83ee6...
MISMATCH wasmtime_wasi-...           needs wiggle_macro-7d87dc17635c9804 9881658a...                       lane has ef90c2eb...
MISMATCH wasmtime_wasi_io, wiggle    (same two macros)
checked 1019 units; 6 inconsistencies
```

A bare compiler call (`use wasmtime::Engine;`, `--extern wasmtime=.../libwasmtime-5b59150fb298cf7b.rmeta`, `-L dependency=` every unit directory of the lane, nothing else) fails with exactly the cargo message, and without the hint of which dependency is wrong:

```
error[E0463]: can't find crate for `wasmtime`
```

Every file is present (rmeta and rlib of wasmtime, environ and all dependencies); the lane is complete but of mixed generations. Earlier `wasmtime_environ` observations (two units 9055 and 7e45) were a red herring: 9055 (normal) and 7e45 (proc-macro side, no `compile` feature) are two legitimate units and wasmtime's recorded environ hash matches 9055.

## Root cause

1. The crate hash (SVH) of one unit is not a function of its inputs alone. Compiling the same proc-macro source twice with the same arguments gives the same SVH only while the compile-time path of the source is the same (`587b4a79` twice, `33a7ffc2` when compiled from another working directory). A unit that is compiled again after its entry was evicted, on another lane, or on another machine therefore gets another SVH (component_macro `7d0941dc` before, `c4b83ee6` now; wiggle_macro `9881658a` before, `ef90c2eb` now).
2. A proc-macro library enters the key of each dependant by `externIdentityV1`: the digest of the portable key of the unit that built it, deliberately not its bytes (linked files differ between machines). That identity did not contain the SVH. So `wasmtime` compiled against macro generation A and `wasmtime` compiled against macro generation B have the same key, the same entry, and a hit serves whichever variant is newest.
3. The metadata of every dependant records the SVH of every crate it was compiled against. `rebaseRestoredUnitV1` repairs this for metadata-only (check) units by patching hashes, but only for libraries found as `lib<id>.rmeta`; proc-macro dylibs were never looked at, and code units (those that emit an rlib) were never verified at all. A restored code unit was admitted without checking its dependency closure.
4. Cargo then keeps the restored `wasmtime` (fresh by its fingerprint) next to a macro that another hit or compile replaced, and the next dependant (`semio-plugin-host-engine`) fails to load it. A reused lane makes this likely (several builds replace dependencies one by one), a fresh lane hides it. `SEMIO_UNIT_CACHE_REMOTE=off` only lowered the odds (fewer foreign generations); the local store holds the same mixed variants, which is why the failure survived it.

No garbage collection is involved: the lane and the store keep complete files; lane trimming only touches released lanes (the live marker is already protected by the existing GC law), and the failing files all existed.

## Fix

- `🪪️extern/🟦️.ts`: the identity file of a dynamic library (version 2) carries the compiler's crate hash and the identity handed to dependants is `unit:<unit identity>:<crate hash>`; a library whose hash is unknown gets no identity and counts by its bytes. Two generations of one unit are now two keys, so a dependant is only hit against the generation it was built for.
- `🔭️interface/🟦️.ts`: `dependencyLibrariesV1` indexes the library files of the dependency directories once per unit (metadata files and proc-macro dylibs, `.dylib`/`.so`/`.dll`); `rebaseRestoredUnitV1` now rebases against proc-macro libraries too; new `restoredClosureViolationV1` admits a restored code unit only when every crate hash it records equals the current hash of the library of that identifier (sidecar that travels with the unit, else the compiler is asked).
- `📜️script.ts`: a restored unit that is not interface-keyed is checked with `restoredClosureViolationV1` before it is served, otherwise it is reported as `incomplete` (`closure: <id> has the crate hash X here, <unit> was compiled against Y`) and compiled; identity files get the crate hash; the dependency sidecar (crate hashes) is now published with every unit that has metadata, not only interface producers, so the check is a file read instead of a compiler call.
- `💾️store/🟦️.ts`: `restoreUnitV1` is all-or-nothing: every output is staged aside first and renamed only when all of them were staged; a fault leaves the lane untouched.

## Tests

- `🪪️extern/🧪️tests/🟦️.ts`: identity includes the crate hash, changes with it, is absent without it.
- `🔭️interface/🧪️tests/🟦️.ts`, "compiler law (closure)": real compiler as the independent oracle. Two generations of a proc-macro, a dependant of generation one; the same closure admits it, a mixed lane (macro two, dependant one) is refused by `restoredClosureViolationV1` and the compiler itself refuses to load the mixed lane (`E0460`/`E0463`); a sidecar-less unit is checked through the compiler.
- `unitcache-closure-check.py` (this folder): independent, library-free check of a whole lane through `rustc -Z ls=root`.

Suites run through `repo`: `repo tools exec bun -- test` on the extern and interface test files (1 and 29 passed), `repo run repo:test-cargo-units` and `repo run repo:test-cargo-unit-cache-processes` (see the end of this file for the outcome).
