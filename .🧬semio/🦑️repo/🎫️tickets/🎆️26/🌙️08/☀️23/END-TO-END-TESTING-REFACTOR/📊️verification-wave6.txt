🧪️ Verification record — wave 6 (broaden the oracle matrix; first TypeScript subject differential)
Every line below is a command that was actually executed and its actual output.

────────────────────────────────────────────────────────────────────────────────
1. EIGHTEEN CASES, SIXTY-SIX EXECUTIONS, ALL PASSING
   $ bun ./📜️script.ts test parity quick
   [test] level=quick cases=18 executed=66 passed=66 failed=0 errored=0 parity=40/40
   $ bun ./📜️script.ts test metrics
   [metrics] scenario coverage      44/46 (95.7%)
   [metrics] oracle coverage        18/18 (100.0%)
   [metrics] dependency-clean       64/215 (29.8%)
   $ bun test 🧪️test/…/🧪️index.test.ts    52 pass  0 fail  779 expect() calls

2. FIVE MORE ARTIFACT FAMILIES, EACH AGAINST A REAL REFERENCE LIBRARY
   | artifact | oracle          | scenarios | note                                              |
   | -------- | --------------- | --------- | ------------------------------------------------- |
   | 🖼️bmp    | image 0.25      | 2         | BMP carries no alpha; reference is given RGB       |
   | 🖼️tiff   | image 0.25      | 2         | compression/strip/byte-order are writer freedom    |
   | 📷️jpg    | image 0.25      | 2         | LOSSY — geometry + luma histogram, not exact pixels |
   | 🧊️obj    | tobj 4          | 2         | no reference WRITER exists; tobj is the reader     |
   | 🟪️stl    | stl_io 0.8      | 2         | triangle soup — corners, not an index buffer       |
   One approved crate (`image`) serves three formats rather than three crates.
   $ bun ./📜️script.ts test oracle quick   cases=18 executed=34 passed=34

   Thirteen registered oracles now: pdf-writer, lopdf, png, gif, zip, flate2, hound, csv,
   image-bmp, image-tiff, image-jpeg, tobj, stl-io — plus clsx, cva, semver from the
   framework side. All classified `test-oracle`; production-reachable held at 151.

3. A THIRD FINDING FROM THE PURITY GATE
   Registering `image` failed immediately with six breaches:
     ✏️s/🔌️plugins/🎞️animate/…/⚙️engine/🎥️video/🦀️component.rs  imports image-{bmp,tiff,jpeg}
     🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs   imports image-{bmp,tiff,jpeg}
   Confirmed real: both crates declare `image = { version = "0.25" }` as a PRODUCTION dependency.
   Both call sites want raster decode/encode that ✏️s/🔌️plugins/🗄️stdio already implements, so both
   are Phase 7 targets. Recorded as `productionDebt` with the reachable paths, the owning module and
   the retirement plan — visible on every `dependency` run, never silenced.

4. FIRST TYPESCRIPT SUBJECT DIFFERENTIAL — IT RUNS TODAY
   🧰️framework/🔨️modules/🎠️kernel owns `versionSatisfies`, a deliberate SUBSET of the semver
   requirement grammar (`*`, `=`, `^`, `~`, `>=`) with the standard leading-zero caret tiers. That
   subset is a published contract, so `semver` 7.8.5 is a genuine oracle.
   $ bun ./📜️script.ts parity quick --owner 🎠️kernel
   [test] level=quick cases=2 executed=7 passed=7 failed=0 errored=0 parity=3/3
   27 vectors across three scenarios — exact/any, caret across the major, minor AND patch tiers
   (^1.2.3, ^0.2.3, ^0.0.3 — where reimplementations most often diverge), tilde and at-least.
   This repository agrees with `semver` on every one.

   A DESIGN GAP THE RUN EXPOSED: the case originally also carried a `@mode-error` scenario about
   this repository's own "unsatisfied, never a throw" law. A feature declares ONE oracle, so the
   coordinator ran the oracle role for that scenario too and reported it `errored`. The frozen
   contract is right and the feature was wrong: an error-law scenario is not the same capability as
   the differential one. Split into a sibling case `reject-malformed-version-input` under a recorded
   no-oracle decision — `semver` THROWS on some malformed input and COERCES other malformed input to
   a match, which is a different and equally valid contract, so comparing against it would measure a
   deliberate divergence rather than a defect.

────────────────────────────────────────────────────────────────────────────────
5. STILL BLOCKED, AND WHY I DID NOT TOUCH IT
   Every Rust SUBJECT still cannot compile. 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel
   now carries 60 `semio_framework::` references — `Fault`, `FaultCode`, `FaultOrigin` and the whole
   `kernel::` module — while `semio-framework` itself depends on `semio-framework-os-kernel`.
   Checked this wave: there is no standalone `semio-framework-diagnostic` crate to depend on
   instead, and `kernel::` lives inside `semio-framework` too. So the edge cannot be declared and the
   reference cannot be repointed; the fix is to extract those types into a lower crate or invert the
   dependency — a design decision belonging to the session doing that refactor.

   Everything on my side is ready for the moment it lands: 13 artifact subject handlers are written
   and gated behind the generated host's `sut` feature, and the metrics gate names the gap
   (`implementation rust 20/22 scenarios`) instead of hiding it.

6. WHAT THE ORACLE MATRIX COVERS NOW
   documents  PDF (create ×3, edit ×2)
   raster     PNG ×3 · GIF ×2 · BMP ×2 · TIFF ×2 · JPEG ×2
   mesh       OBJ ×2 · STL ×2
   archive    ZIP ×3 (incl. member removal) · zlib ×3 (round-trip mode)
   audio      WAVE ×3 (incl. sample-rate retune)
   ui         class-name flattening ×4 · utility merge ×3 · style variants ×3
   kernel     version requirements ×3 · malformed-input error law ×1
   platform   five-language host protocol ×3
