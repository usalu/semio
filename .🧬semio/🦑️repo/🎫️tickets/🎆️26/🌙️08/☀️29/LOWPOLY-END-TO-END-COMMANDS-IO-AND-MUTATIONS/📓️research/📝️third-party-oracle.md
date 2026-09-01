# Lowpoly PNG Third-Party Oracle

## 1. Pillow availability (verified, not assumed)

- Interpreter: `.venv/bin/python3` (uv-managed, `.venv/bin/python3` == the interpreter the repo's own
  non-compose Python test host (`🧰️framework/…/🧪️test/📦️packages/🐍️python/🐍️host.py`) runs under —
  root `pyproject.toml` scopes pytest to `compose/py`/`compose/engine` only, so this host is the
  actual harness for lowpoly's `🐍️.py` adapters).
- `cd /Users/ueli/Documents/semio && .venv/bin/python3 -c "import PIL; print(PIL.__version__)"` →
  `12.2.0`. `importlib.metadata` confirms `License-Expression = MIT-CMU`. Not in `pyproject.toml`
  (transitive via `matplotlib`, `dev` group) — no dependency change made.

## 2. What the oracle validates

Split into a NEW sibling case, `🧪️tests/io-lowpoly-png-oracle-1/`, because `@oracle-`/`@no-oracle-`
is a FEATURE-level tag (`tagValue(featureTags, …)` in the framework's `📦️index.ts:396-397`) and
cannot coexist with `io-lowpoly-1`'s own `@no-oracle-lowpoly-io-native-round-trip`. Files added:
`🥒️.feature`, `🦀️.rs`, `🐍️.py`, `🧫️fixtures/lowpoly-snapshot.json` (copy of `io-lowpoly-1`'s fixture).

- Rust subject reports `serialize()`'s pre-encode `PngSnapshot` (`width`, `height`, `pixels`, `tEXt`
  keyword) — never a decode of its own bytes.
- Python oracle independently re-derives width/height/pixels from the SAME fixture via the documented
  ~10-line selection rule (`primary_paint_raster`: first paint layer whose decoded length ==
  1024²×4, else `1x1` opaque white), builds a REAL PNG with `Pillow`, decodes it back with `Pillow`
  (`PIL.Image`/`PIL.PngImagePlugin`), and asserts recovered width/height/RGBA pixels/tEXt-hex match —
  genuine independent PNG-codec evidence, not a second run of our reader.
- This fixture's paint layer decodes to 8 bytes (≠ 4,194,304), so BOTH sides exercise the real,
  documented `1x1` fallback — not a fabricated corner case.
- `evaluateParity` (`📦️index.ts:2440`) pairs subject/oracle by scenario id and compares
  `output.projection` under `ordered-json-v1` — confirmed by reading the function, not assumed.

**Honest limit, stated in the feature file and `oracle.json`:** no cargo build was available this
pass (hard constraint), so the Python oracle's raw PNG comes from Pillow encoding the
independently-derived values, not from decoding Rust's actual `serialize_bytes()`/`encode_png()`
bytes. It proves parity between Rust's DECLARED intent and a Pillow-buildable equivalent, not yet
that today's live encoded stream is itself Pillow-readable. The DSL text's literal content is never
predicted (`print_dsl()`'s grammar is this subset's own spec) — only that its hex value round-trips
intact through Pillow is asserted.

## 3. Discovery proof

```
cd /Users/ueli/Documents/semio && bun ./📜️script.ts test discover
```
`[discover] 172 test case(s)` (was 171). Lowpoly rows:
```
…-io-lowpoly-1              …/🧪️tests/io-lowpoly-1               [rust]
…-io-lowpoly-png-oracle-1   …/🧪️tests/io-lowpoly-png-oracle-1     [rust,python]
…-mutate-lowpoly-1          …/🧪️tests/mutate-lowpoly-1            [rust,python]
```
`test discover --json` for the new case: `adapters: {rust: …🦀️.rs, python: …🐍️.py}`,
`localFixtureDir: …/io-lowpoly-png-oracle-1/🧫️fixtures` — both adapters and its own fixture dir
discovered correctly.

## 4. Execution + fail-injection proof (real harness, no cargo)

Built a `plan.json` (role `oracle`, scenario `png-oracle`, the committed fixture) and ran the REAL
host:
```
.venv/bin/python3 🧰️framework/…/🐍️host.py --plan plan.json --adapter …/🐍️.py --out results.jsonl
```
**Pass**: exit 0, `status:"passed"`, `projection: {"format":"png","width":1,"height":1,"pixels":[255,255,255,255],"textChunk":{"keyword":"semio-lowpoly-dsl","hexRoundTrips":true,"decodesToUtf8":true}}`.
Independently re-opened the emitted raw `.raw` file with a fresh `Image.open` — real PNG magic bytes
(`89504e47…`), real IHDR/tEXt/IDAT/IEND chunks, `size (1,1)`, `text` recovered, `pixel(0,0) ==
(255,255,255,255)`.

**Fail injection**: edited `🐍️.py`'s pixel-equality check to compare against a deliberately corrupted
expected buffer (`bytes([254]) + pixels[1:]`). Re-ran the same host command: exit **1**,
`status:"failed"`, real `AssertionError: png-oracle: Pillow decoded RGBA pixel data does not match
the composited paint layer bytes` in `diagnostics`.

**Restore**: reverted the edit; `diff` against a pre-corruption copy showed no differences; MD5 before
and after both `47eb832223725ab8995987a28279df9d`. Re-ran the host a third time: exit 0, `status:
"passed"`, identical `rawHash`/`projectionHash` to the first clean run.

## 5. oracle.json changes (honest coverage statement)

Added `oracles[].lowpoly-png-pillow` (`kind: third-party-library`, package `pillow` 12.2.0,
`MIT-CMU`, `productionReachable: false`). Rationale states plainly:
- Covers exactly the `png-oracle` scenario in the new case.
- Does NOT cover `io-lowpoly-1`'s other 8 `roundtrip-*` scenarios (still under
  `lowpoly-io-native-round-trip`'s `metamorphic-laws` substitute — appended a one-line UPDATE note
  there cross-referencing the new entry, main rationale untouched).
- Does NOT and CANNOT cover mesh geometry for `obj`/`ply`/`stl`/`gltf`/`dwg`/`las`:
  `LowpolyObject.mesh` is a content-addressed `store::ArtifactChild<SemioMeshSnapshot>` handle, never
  embedded geometry, and the io layer gets no store resolver to follow it — an architecture limit, not
  a tooling gap.
- Does NOT prove today's live `encode_png()` byte stream is Pillow-readable (no-cargo limit, §2).
- Does NOT compare the literal DSL text content (only its structural hex round-trip).

## 6. Handoff items

1. Wire the Rust side end-to-end once cargo is reachable: `bun ./📜️script.ts test contract --case
   io-lowpoly-png-oracle-1` then `test run … --implementation rust` and `--implementation python`,
   confirming real `evaluateParity` equality between subject and oracle projections.
2. Have the Python oracle decode the SUBJECT's actual raw artifact directly (once a cross-role
   artifact channel or a `cargo`-produced fixture exists), closing the "Pillow never touches Rust's
   own bytes" gap named above.
3. Same repo-wide `oracleHostPackages` ancestor-scope gap `io-lowpoly-1`'s own no-oracle decision
   already names for `tobj`/`ply-rs`/`stl_io` — unchanged by this pass.

## 7. Files touched (all within granted ownership)

- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️tests/io-lowpoly-png-oracle-1/🥒️.feature` (new)
- `…/io-lowpoly-png-oracle-1/🦀️.rs` (new)
- `…/io-lowpoly-png-oracle-1/🐍️.py` (new)
- `…/io-lowpoly-png-oracle-1/🧫️fixtures/lowpoly-snapshot.json` (new, copy of `io-lowpoly-1`'s fixture)
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`
  (edited — appended `lowpoly-png-pillow` to `oracles`, one-line addendum to
  `lowpoly-io-native-round-trip`'s rationale)

`io-lowpoly-1` and `mutate-lowpoly-1` were not modified.
