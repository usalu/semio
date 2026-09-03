# Test Fixture Prevalidation — mutate/io/io-png/command-lowpoly-1

**Per-case verdict**: all four cases intact and discovered with the right adapters after the sweep.
`mutate-lowpoly-1` [rust,python], `io-lowpoly-1` [rust], `io-lowpoly-png-1` [rust,python],
`command-lowpoly-1` [rust] — verified by reading every file in all four directories against current
source (structs, fields, module paths, wiring in `📦️packages/🦀️rust/🦀️.rs` and `✏️editor/🦀️.rs`).
`command-lowpoly-1`'s 13 representative payloads match `every_command()` byte-for-byte; `47` matches
both `LOWPOLY_MIGRATED_TOOL_IDS.len()` and `app_commands!`'s row count (0 `BatchOnlyPendingRewrite`).

**Drift found and fixed** (post-sweep stale references, all in owned files):
1. `mutate-lowpoly-1/🦀️.rs`: `#[path]` to the shared law helper was 5 levels up, needed 9
   (`../../../../../../../../../🗄️stdio/🧪️oracle/⚖️law/🦀️.rs`) — verified by `ls`. Also stale doc
   paths (`../../🏅️standards/…/🔣️oracle.json` → `../../🧪️oracle/🔣️.json`, mutations dir path
   simplified). **This same 5-vs-9 `#[path]` mistake exists in `mutate-cad-1`, `mutate-block-3d-1`,
   `mutate-block-5d-1`, and every other sibling using this shared law import — confirmed broken there
   too, but those files are outside this pass's ownership.**
2. `mutate-lowpoly-1/🥒️.feature`: `🐍️component.py`→`🐍️.py`, `🦀️component.rs`→`🦀️.rs`.
3. `🧪️oracle/🔣️.json`: 4 stale relative paths (`../../../../../🧪️tests/…` → `../🧪️tests/…`) and
   `🐍️component.py`→`🐍️.py` in the rationale prose; verified JSON still parses.
4. `command-lowpoly-1/🥒️.feature`: `✏️editor/🦀️component.rs`→`✏️editor/🦀️.rs`.
5. `io-lowpoly-1/🦀️.rs` + `🥒️.feature` — **real bug, not just renaming**: all 8 non-PNG formats
   asserted a successful round trip. Read `🚪️io/🦀️.rs`'s own `#[cfg(test)]` unit test
   `unimplemented_geometry_formats_error_honestly_instead_of_lying` and each stub leaf's doc comment:
   `dwg`/`gltf`/`las`/`stl` `serialize_bytes` unconditionally `Err`s (mesh is a content-addressed
   handle, unreachable at this layer). Split the adapter: `json`/`obj`/`ply`/`txt` keep the real
   round-trip law; the other four now assert the exact documented error string. Feature file rewritten
   into two `Scenario Outline`s (same `roundtrip-<format>` scenario ids, adapter registration
   unchanged) with the "expected to fail forever" framing replaced by "asserts today's real error."

**Discovery**: `bun ./📜️script.ts test discover` → **213 test case(s)** repo-wide. The four lowpoly
rows all present with correct adapters:
```
...-command-lowpoly-1  [rust]
...-io-lowpoly-1        [rust]
...-io-lowpoly-png-1    [rust,python]
...-mutate-lowpoly-1    [rust,python]
```

**Pillow fail-injection proof** (`.venv/bin/python3`, Pillow 12.2.0, matches the oracle's pinned
version): ran the real host (`🧰️framework/…/🧪️test/📦️packages/🐍️python/🐍️.py`) against
`io-lowpoly-png-1/🐍️.py` with a synthetic 1×1 RGBA PNG carrying a `semio-lowpoly-dsl` tEXt chunk as
`subjectRawInputs.rust`. Pass → `status: passed`, exit 0. Renamed the tEXt keyword to `wrong-keyword`
→ `status: failed`, exit 1, `AssertionError: roundtrip-png: Pillow found no 'semio-lowpoly-dsl' tEXt
chunk`. Restored the original file from a byte copy; `diff` reported no differences; reran → `status:
passed`, exit 0 again. The oracle can fail and does.

**Flagged, not fixed (outside ownership, `🚪️io/**` off limits)**: `🚪️io/📤️export/…/🔣️json/…/🦀️.rs`'s
`serialize()` calls `serde_json::to_value(snapshot: &LowpolySnapshot)` and
`🚪️io/📥️import/…/🔣️json/…/🦀️.rs`'s `deserialize()` calls `serde_json::from_value(...)`, both
unconditional production code — but `LowpolySnapshot`'s `Serialize`/`Deserialize` derive in
`🧬️schema/📸️snapshot/🦀️.rs` is `#[cfg_attr(test, …)]`-gated. This ticket's own
`📓️research/📝️serde-removal-schema.md` independently found and fixed the SAME defect class at two
other call sites (mutations text codec, `MediaConversion`) but did not touch `🚪️io/`; the json IO leaf
looks like a third, unfixed instance of it. My four owned test files (`io-lowpoly-1`'s `document()`
and non-stub round trip, `io-lowpoly-png-1`'s `document()`) call `serde_json` on `LowpolySnapshot`
the same way — consistent with the plugin's own already-passing `#[cfg(test)]` unit tests, so left
as-is rather than guessed at, but worth a direct look once `cargo test -p semio-s-plugin-lowpoly
--lib` runs.
