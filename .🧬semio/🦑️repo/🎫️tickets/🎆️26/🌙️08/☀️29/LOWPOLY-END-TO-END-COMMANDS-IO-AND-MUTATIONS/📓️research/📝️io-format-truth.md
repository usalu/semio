# Lowpoly IO Format Truth (9 formats x 2 directions)

VERDICT: the LATER report (5/9 real: obj, ply, json, png, txt; 4/9 honest stubs: dwg, gltf, las, stl)
is CORRECT. The EARLIER report ("8/9 fully implemented, only txt stubbed") was wrong — it mistook
code that *compiles and looks real* for code that *works*. No compiler run was available to me in this
session (build-lock deadlock across 3 concurrent agents on this ticket; coordinator took over the
compile pass separately) — this matrix is 100% source-reading + `git diff HEAD`, not test output.

## Why the earlier report was wrong (git diff HEAD proof)

At `HEAD`, obj/ply/png/dwg/stl/gltf/las export all shared one pattern (obj shown, others identical
modulo type names):
```rust
let bytes = <LowpolySnapshot as store::ArtifactPack>::encode_pack(snapshot);
<ObjSnapshot as store::ArtifactPack>::decode_pack(&bytes)...
```
This round-trips `LowpolySnapshot`'s own pack envelope (`lowpoly.lowpoly`) straight into a decoder that
unconditionally rejects any envelope id but its own — always throws `PackError::Schema("pack envelope
mismatch")` at runtime, despite compiling cleanly and reading as real logic. `txt` was the only leaf
with an explicit `Err("... not yet implemented")` stub at HEAD — hence the earlier report's "only txt
stubbed" claim, from static reading without execution. `json` alone was already correct at HEAD
(real `serde_json`, no diff in this fix).

## Architecture: is mesh geometry reachable in the io layer? YES, this claim is TRUE.

`LowpolyObject.mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>` — a content-addressed child
HANDLE (`🦀️component.rs:134`), not inline geometry. Confirmed independently via
`🧰️framework/🔨️modules/🚪️io/🦀️component.rs:768-802`: `ComposeSource`/`ErasedComposeSource` carry only
`{dialect, payload: Text|Binary}` — no store/session resolver, no way to follow the child handle. A
synchronous `&LowpolySnapshot -> Bytes` serializer genuinely cannot reach real mesh vertices.

## The 9x2 matrix (current working-tree state, all in files I own)

| Format | Export (lowpoly->fmt)                                                                 | Import (fmt->lowpoly) |
|--------|-----------------------------------------------------------------------------------------|------------------------|
| txt  | REAL — `📤️.../txt/.../🦀️component.rs:14-16` `TxtSnapshot::from_body(&print_dsl(s))` (DSL verbatim, CARRIER_TEXT law) | REAL — `📥️.../txt/.../🦀️component.rs:12-13` `parse_dsl(&from.to_body())` |
| json | REAL — `📤️.../json/.../🦀️component.rs:10-14` `serde_json::to_value(snapshot)` | REAL — `📥️.../json/.../🦀️component.rs:9-15` `serde_json::from_value(...)` |
| obj  | REAL — `📤️.../obj/.../🦀️component.rs:30-33` DSL hex-packed into `ObjUnknownStatement` (real per-line retention slot) | REAL — `📥️.../obj/.../🦀️component.rs:18-27` reads it back, `parse_dsl` |
| ply  | REAL — `📤️.../ply/.../🦀️component.rs:21-24` DSL hex-packed into `PlySnapshot.comments` | REAL — `📥️.../ply/.../🦀️component.rs:13-22` reads it back, `parse_dsl` |
| png  | REAL — `📤️.../png/.../🦀️component.rs:39-51` first paint layer's real RGBA pixels become the raster; DSL hex in a real `tEXt` chunk | REAL — `📥️.../png/.../🦀️component.rs:14-24` reads `tEXt` chunk, `parse_dsl` (raster not consulted) |
| stl  | STUB — `📤️.../stl/.../🦀️component.rs:23-25` `Err("...not implemented")` (no retention slot in `StlSnapshot`) | STUB — `📥️.../stl/.../🦀️component.rs:11-13` `Err(...)` |
| gltf | STUB — `📤️.../gltf/.../🦀️component.rs:14-16` `Err(...)` | STUB — `📥️.../gltf/.../🦀️component.rs:12-14` `Err(...)` |
| dwg  | STUB — `📤️.../dwg/.../🦀️component.rs:14-16` `Err(...)` | STUB — `📥️.../dwg/.../🦀️component.rs:12-14` `Err(...)` |
| las  | STUB — `📤️.../las/.../🦀️component.rs:14-16` `Err(...)` | STUB — `📥️.../las/.../🦀️component.rs:11-13` `Err(...)` |

All paths rooted at
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/`. Every stub is an
explicit typed `Err`, never a silent empty/wrong-but-valid file (`serialize_bytes` for the 4 stubs maps
`serialize()`'s `Err` through, never fabricates `Ok(vec![])`).

## Registry cross-check (`🚪️io/🦀️component.rs`)

- `import_stdio_kinds()`/`export_stdio_kinds()` (lines 3-8) advertise all 9 formats — unchanged, correct
  per ticket rule (never trim the list to tidy the matrix).
- `io_registry::entries()` (lines 254-271) registers export `ComposerEntry` rows for all 9 formats,
  including dwg/gltf/las/stl — registered but honestly erroring at call time, not silently missing.
- `derived_composition::compose()` (lines 36-131) tries all 9 import dialects; dwg/gltf/las/stl always
  fail their `if let Ok(...)` and fall through to the final `Err("no source in a known read dialect")`.
  No format is advertised-but-silently-wrong; broken formats are advertised-and-honestly-erroring.

## Changes vs HEAD (no `📝️io-implementation-result.md` existed — this is now that record)

`git diff HEAD --stat` on the owned `🚪️io/` tree: 18 files, +395/-112. Fixed for real: txt (was
explicit stub), obj/ply/png (were pack-envelope-mismatch bugs, now DSL-carrier trick reusing each
format's own real retention slot — never a second bespoke grammar). Converted from
silently-broken-looking-real to honest stubs: dwg/gltf/las/stl (both directions). Untouched, already
correct: json (both directions). `🚪️io/🟦️component.ts` diff (+34) mirrors the same 9-format list, not
independently investigated further (excluded from my ownership).

## Tests

`io_registry` test mod (`🚪️io/🦀️component.rs:276-360`, in a file I own) already pinned: round-trip
equality for txt/json/obj/png/ply, and an export-honesty assertion for stl/gltf/dwg/las. I added one
more test, `unimplemented_geometry_import_formats_error_honestly_instead_of_lying`, covering the same
4 formats' IMPORT direction (previously untested) — arbitrary bytes must error, never fabricate a
geometry-less snapshot silently.

**WRITTEN BUT NOT RUN THIS SESSION** — the shared build lock deadlocked 3 concurrent agents (me, a
verification agent, the coordinator) on this ticket; I stood down on `cargo` per the coordinator's
explicit instruction and the coordinator is running the compile/test pass separately in an isolated
`CARGO_TARGET_DIR` with `RUSTC_WRAPPER=""`. Do not treat this file as proof the tests pass — only as
proof of what they assert and why, from source.
