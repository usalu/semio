# W7 Closer — os-run Fix + Cross-Plugin IoRouter Test

## Verdict inherited: FAIL (independently re-confirmed), now resolved to the extent cheap/safe

`w7-verify-report.md` returned **FAIL** on 5 points. Re-checked each directly against the working
tree before touching anything:

1. **No `w7-report.md` existed.** Confirmed true (file did not exist). **Fixed**: wrote
   `w7-report.md` this session, retroactively documenting the real, already-verified W7
   implementation (os-run `RunArtifact` codec fix + cross-plugin `IoRouter` test) from the code
   itself — no new claims, everything traceable to `w7-verify-report.md`'s own findings.
2. **The two gate commands named in the original task don't cover the new test** (it lives in
   `semio-framework-plugin-host`, not `semio-framework-os-run` or `semio-framework`). This is a
   process/documentation fact about which commands exercise which crate, not a code defect —
   nothing to fix. Noted here for the record; this closer's own gate (below) matches the task's
   literal command list, same limitation applies.
3. **cad's wasm component fails to build** (`E0599`, missing `ArtifactStore::projection_json`,
   ~10+ plugins affected). **Not fixed** — design-judgment, see Follow-ups.
4. **The cross-plugin test's real assertions are unverified in this environment** because of
   (3) — its guard is the same silent-skip convention already used by
   `wasm_plugin_runtime_loads_real_plugin_component_if_present` in the same file, so the guard
   itself is not a defect, just currently un-exercised. **Not fixed** — downstream of (3).
5. **Master plan's W7 gate ("wasm builds succeed") not met** — downstream of (3), **not fixed**.

## Cheap/safe fixes applied this session

Two dead-code compiler warnings on `semio-framework-os-run`'s own code (both flagged in the
verifier's `cargo check` output, §1) were fixed directly — neither is design-judgment, both are
confirmed-safe deletions/annotations with zero behavior change:

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/📦️glue.rs`** — removed the
   unused `extern crate semio_framework_os_kernel as dsl_core;` alias (line 7). Grepped the whole
   repo for `dsl_core`: it is a real, used alias in ~15 *other* crates' `glue.rs` files, but in
   this specific crate it had zero call sites (`store`/`dsl`/`protocol` cover everything this file
   actually needs) — confirmed dead by the crate's own W1-era comment, which explains `dsl_core`
   was superseded by `workflow` (`semio_framework`) for this crate specifically.
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:304`** — added `#[cfg(test)]` to
   `fn run_fault_bytes`. Grepped its only 5 call sites: the definition plus 4 uses, all inside
   `mod tests` (`FakeHost`, line 1240+). It has zero non-test callers, so it was only ever "dead"
   from a non-test build's point of view — `#[cfg(test)]` makes that explicit and removes the
   warning without touching behavior (still compiles and is called identically under `cargo test`).

Verified via re-run: `cargo check -p semio-framework-os-run` now produces **0 warnings
attributable to this crate's own code** (the two warnings still printed in the tail belong to its
dependencies `semio-framework-plugin-host` and `semio-framework-os`, out of scope here).

## Not fixed — design-judgment, documented as follow-ups

1. **`ArtifactStore<P, Mutation>::projection_json` has no definition anywhere**, despite ~10+
   plugins' wasm-binding files calling it (cad, jack, raster, process, writer, gis, shooting,
   puzzle, animate/present, trinity/rewrite — confirmed by `grep -rn "fn projection_json"` across
   the repo returning zero hits on `ArtifactStore` itself, only call sites). This blocks building
   a second real `.wasm` plugin component next to stdio, which is what the new cross-plugin test
   needs to actually exercise its routing assertions instead of hitting its silent-skip guard.
   Real fix requires deciding what `projection_json` should return per plugin (likely a
   `ArtifactStore` method backed by each snapshot's own JSON projection) — a cross-cutting
   framework design decision, not a local one-file patch, and out of this closer's cheap/safe
   scope.
2. **`bun nx run @semio-tech/framework-os-dev:build -- <plugin>` cannot build any plugin at all**
   right now, because the target's dependency graph pulls in `@semio-tech/assets:build`, which
   transitively runs a `storybook build` that fails on a broken, committed (not concurrent-churn)
   import: `.storybook/stories/ui/✅ValidationTree.stories.tsx` imports
   `@semio-tech/coda-desktop/renderer`, a package that does not exist anywhere in the repo. This is
   an nx-wiring/infra problem, orthogonal to W7 and to the artifact-format-retirement ticket
   entirely — flagging for whoever owns the storybook/nx build graph.
3. **Item (1) above is the actual reason item 4/5 of the inherited FAIL can't be closed out** —
   until `projection_json` is real, no second wasm component can be built next to stdio's, so the
   new cross-plugin `IoRouter` test's routed-compose assertions remain unverified in any
   environment that hits this gap (which, per the grep above, is every plugin with this common
   wasm-bindings shape — i.e. currently all of them except stdio).

## Final gate — re-run fresh this session, after the two fixes above

Raw logs in this folder: `w7-close-osrun-check.txt`, `w7-close-osrun-test.txt`,
`w7-close-stdio-check.txt`, `w7-close-stdio-test.txt`, `w7-close-policy.txt`.

1. **`cargo check -p semio-framework-os-run`** → **0 errors, 0 warnings attributable to this
   crate's own code** (up from 2 own-crate warnings before this session's fixes).
2. **`cargo test -p semio-framework-os-run --lib`** → **15 passed, 0 failed** — same 15 pre-existing
   `run_lib` graph/media-conversion tests the verifier saw; unaffected by the two warning fixes
   (confirms `run_fault_bytes`'s `#[cfg(test)]` didn't break its test-only call sites).
3. **`cargo check -p semio-s-plugin-stdio --lib`** → 0 errors, 493 pre-existing warnings (unrelated
   dead-code/style warnings scattered across the whole stdio plugin, none touched by W7 or this
   session — out of scope to clean up here).
4. **`cargo test -p semio-s-plugin-stdio --lib`** → **1930 passed, 0 failed, 3 ignored** — exact
   match to every prior wave's close report (w6-close, etc.), confirming stdio is untouched by
   W7's changes.
5. **`bun ./📜️script.ts policy`** → **21654 high-priority breach(es) across 26 rule(s)** —
   byte-for-byte the same breach count as `w6-close-report.md`'s final gate run. Confirms this
   session's two-line fix introduced **zero new breaches** and fixed none (neither line was a
   policy target; both were plain compiler-warning cleanups).

## Verdict

**W7 is now the best state it can reach without a design-scope framework fix.** The os-run
compile/test gate is fully green with no own-crate warnings. The cross-plugin `IoRouter` test is
real, well-constructed, and committed-ready code, but its routing assertions remain unverified in
any current environment until `ArtifactStore::projection_json` gets a real implementation (follow-up
#1 above) — this is a pre-existing, repo-wide gap discovered by this ticket's work, not created by
it, and is out of scope for a "cheap/safe" closer fix. Recommend opening it as its own ticket before
declaring the master plan's W7 gate ("wasm builds succeed") met.

## Files touched this session

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/📦️glue.rs` — removed unused
  `dsl_core` extern crate alias.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` — `#[cfg(test)]` on
  `run_fault_bytes`.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT/w7-report.md`
  — new, retroactive report for the unreported W7 implementation.
- `.../w7-close-report.md` — this file.
- `.../STATUS.md` — appended dated entry.
- `.../w7-close-osrun-check.txt`, `w7-close-osrun-test.txt`, `w7-close-stdio-check.txt`,
  `w7-close-stdio-test.txt`, `w7-close-policy.txt` — raw gate logs, this session.
