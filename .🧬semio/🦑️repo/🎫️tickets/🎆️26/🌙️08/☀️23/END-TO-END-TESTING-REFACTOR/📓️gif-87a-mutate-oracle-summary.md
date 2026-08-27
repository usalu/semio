# GIF 87a mutation-oracle case — summary

Wave 7 subset: `🎞️gif` standard `🔖️87a` subset `✳️any` (12 `GifMutation` variants, no GCE-shaped
kinds — 87a genuinely has no frame delay, disposal, transparency, comment or application-extension
concept).

## Finding: 89a file mis-filed under the 87a example directory

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif`
(4,433,936 bytes) starts with the literal bytes `47 49 46 38 39 61` = `GIF89a`, confirmed with
`head -c6 | xxd`. It is a real, 54-frame, 800×800 animated GIF89a (per-frame local colour tables,
NETSCAPE2.0 loop extension), not a GIF87a — this repo's own 87a `decode_gif` rejects it outright
(bad magic, then would reject its Graphic Control Extensions even if the magic were patched). The
existing `📚️examples/💃️dancing/🦀️component.rs` in the 87a tree already documents this by decoding
it via `standards::v89a::engine::decode_gif`, so the discrepancy was already known/worked around,
just not corrected. This ticket does not fix the filing (out of scope — the example wiring belongs
to a different concern than the mutation-oracle case), only reports it per the fleet brief's
instruction.

## Fixture: derived once, real content throughout

No genuine GIF87a file existed anywhere in the repo. Derived one, real content only, no synthetic
gradients:

1. Scratch tool `gif87a-fixture-derive` (this ticket folder) reads the real `dancing.gif` with the
   pinned `gif` 0.13 reference crate, `ColorOutput::Indexed`, so palettes/indices come back exactly
   as stored on disk.
2. Cropped a genuine 16×16 rectangle of real, already-decoded palette indices out of three real
   frames (indices 0, 20, 40), each frame's own real local colour table preserved untouched.
3. Frame 0's real 256-colour local table was promoted to the derived file's own Global Color Table
   (the source file has no global table of its own — all real per-frame LCTs).
4. Wrote the result with `gif::Encoder` (GIF87a has no writer of its own in the reference crate —
   it always emits `GIF89a` and one Graphic Control Extension per frame), then patched the 6-byte
   header magic and walked the real GIF block grammar to strip every `0x21` extension block back
   out — GIF87a has no extension concept, and this repo's own 87a decoder hard-rejects any that
   remain.
5. Verified independently: the derived file re-decodes correctly through the `gif` crate itself
   (16×16 screen, 3× 16×16 frames, no extension blocks found).

Committed to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🧫️fixtures/🖼️dancing-87a.gif` (2,936 bytes),
referenced from the feature as `shared://🖼️dancing-87a.gif`.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS` (12 kebab-case variant names) and the self-test asserting it against
  both the enum and the manifest (additive only, +20 lines).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`
  — filled in: an independent GIF87a document model + JSON bridge + `gif`-crate codec, all 12
  mutation kinds, and inverse computation, none of it calling into this repo's own `decode_gif`/
  `encode_gif`/`GifMutation`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧪️oracle/🔣️.json`
  — new: oracle registration (`gif-87a-mutate`, capability `gif-87a-mutate`, `semantic-raster-v1`)
  and `mutationCatalogs` (`gif-87a-any`, 12 kinds).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🧫️fixtures/🖼️dancing-87a.gif` — new, the derived fixture.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🧪️tests/mutate-gif-87a/component.feature` — new: 25
  scenarios (12 `mutate-<kind>`, 12 `inverse-<kind>`, 1 `identity-round-trip`), one `Scenario
  Outline` + `Examples` table shared by mutate/inverse.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🧪️tests/mutate-gif-87a/🦀️component.rs` — new: oracle
  handlers (verified) + `#[cfg(feature = "sut")]` subject handlers (written, correct against the
  real, confirmed module paths in `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`, but NOT
  compiled this run — the plugin crate is blocked by a concurrent os-kernel refactor, exactly as
  the fleet brief said to expect).

Scratch (ticket folder, not source tree): `gif87a-fixture-derive/` (the derivation tool) and
`gif87a-oracle-checker/` (a standalone crate stubbing `semio_repo_test_host::Json` that
compile-checked and ran the real oracle module in isolation against the real fixture before the
full verification, confirming all 12 kinds mutate and invert correctly with the expected geometry
changes).

## Verification (real output)

From `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-gif-87a
4 high-priority breach(es) across 1 rule(s):
      4  testing/dependency
  testing/dependency  ✏️s/🔌️plugins/🎞️animate/…/🎥️video/🦀️component.rs  imports oracle image-bmp-3-mutate
  testing/dependency  ✏️s/🔌️plugins/🎞️animate/…/🎥️video/🦀️component.rs  imports oracle image-tiff-6-0-mutate
  testing/dependency  🧰️framework/…/🗺️tiled-map/🦀️component.rs        imports oracle image-bmp-3-mutate
  testing/dependency  🧰️framework/…/🗺️tiled-map/🦀️component.rs        imports oracle image-tiff-6-0-mutate
```
Zero breaches name `gif`, `gif-87a`, `mutate-gif-87a` or `gif-87a-any` anywhere — all 4 reported
breaches are pre-existing/concurrent, in unrelated plugins (bmp/tiff production imports), not this
case.

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-gif-87a
[test] level=exhaustive cases=1 executed=25 passed=25 failed=0 errored=0 parity=0/0
```
25/25 green, reproduced on a second run. (First attempt hit a concurrent peer's compile error in
`📷️png`'s 1.2 oracle module — the whole `semio-s-plugin-stdio-test-oracle` crate is one crate, so
any subset's broken file blocks every subset's run; that peer fixed their file mid-session and the
next run was clean, matching the fleet brief's "ignore unrelated concurrent breakage" guidance.)

Subject (SUT) phase intentionally not run — `semio-s-plugin-stdio` cannot compile right now (the
documented `📡️spr/🧵️channel` os-kernel refactor cycle), exactly as the fleet brief predicted.

## Deliberate deviations / notes for the coordinator

- Reused the existing `semantic-raster-v1` comparison profile (already registered for `gif` with
  capability `gif-raster`) rather than inventing a new one — `ComparisonProfile` is a closed enum
  in the framework schema, and this profile already exists and already fits GIF's own established
  precedent (`raster::project_gif`).
- Registered a NEW, distinct oracle id `gif-87a-mutate` (capability `gif-87a-mutate`) in my own
  subset's `🔣️component.json`, separate from the shared `gif` entry's `gif-raster` capability, per
  the fleet brief's explicit instruction not to reuse a capability-mismatched entry.
- `set-pixel-aspect-ratio` and `set-image-interlace` are real, applied, and round-tripped by both
  oracle and subject, but the reference `gif` crate's reader exposes neither pixel-aspect-ratio nor
  an interlace flag, and `raster::project_gif`'s existing shape (reused verbatim, not edited) does
  not report them either — both are honestly canonicalized away by the same established precedent
  the JPEG/GIF projections already use for what an independent reader cannot observe.
- No shared family-module edit was needed — GIF already has `raster::project_gif`, reused as-is.
