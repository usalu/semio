# Result pfx: semio `🫙️prefix` codec migration (18 domains)

Executor Sonnet 5.5, foreground, no sub-agents, no cargo, no modifying git command, no `package.json`/`bun.lock`/storage edit.

## Outcome

- Law `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🫙️prefix/🟦️.ts`: **25 pass / 22 fail -> 48 pass / 0 fail** (47 laws + 1 new shared-vector law).
- All 22 reds were production gaps (not law drift): 18 `🛬️native/🦀️.rs` without `binary_into`/`document_into`/`workspace::decode`, graph/table/value never mounted `🫙️projection`, base encoder without `EncodingPrefix`.
- Decision: the 18 domain codecs are hand-written (no generator exists), so the shared part lives once in the base native module and every domain reuses it (no repeated code).
- All 25 semio `🪶️sqlite` bun suites are green; `✏️s/🔌️plugins/🧪️tests/📁fixture-placement` stays at its law4 state (38 pass / 1 fail: the BIM blessed-output rows need a cargo run) and `🗂️native-ownership` 4/0.

## Design (schema-first retained, one shared helper layer)

Base `✉️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🦀️.rs` (shared by all domains):

- `Body` + `with_body(payload, id, control, native_control, read)`: size admission, SQL ledger, progress forwarding and envelope unwrapping, one place instead of 18 inline copies. Domain `decode` is now: `admit_layout` + `with_body(.., |body, decoder, limits| workspace::decode(decoder, empty, |output, decoder| match body {Binary => binary_into, Text => document_into}))`.
- In-place fillers: `reader_text_into`, `reader_bytes_into`, `hex_text_into`, `hex_bytes_into`, `hex_float` (stack word, 384 bytes like the encoder), `binary_list_into`, `text_list_into` (insert a zero-heap element, then fill it, so cancellation keeps every born buffer in the retained prefix), `empty_child`, `empty_reference`.
- Removed (dead after the migration, no compat layer): `decode`, `binary_list`, `text_list`, `text`, `hex`, `hex_text`. `Owned` stays only for the 18 `retire_sqlite_snapshot` overrides (Store replacement is outside this task).
- Each domain: `pub fn binary_into(output, body, control, limits)`, `pub fn document_into(...)`, `pub fn empty()` (made `pub` for the envelope); recursive/enum values select the real variant in the retained slot before any text/octet/vector allocation (document blocks, presentation shapes, drawing nodes, cad entities, brep curves/surfaces, model enums, kit pins/links, value, animation, object/kit child handles).
- Value native gained `value_text_into` / `value_binary_into` (table and graph reuse them); document native gained `blocks_text_into` / `block_text_into` (presentation reuses them).
- Envelope (`✉️base/…/📸️snapshot/🦀️.rs` `union_decoding`): one retained `SemioSnapshot` with a zero-heap placeholder subset, the selected subset is replaced by its `empty()` and filled by its own `*_into` (one `select!` macro replaces 36 arms).
- Base encoder (`🛫️native/🦀️.rs`): `EncodingPrefix { bytes, admission }` retained through `owned_workspace`; admission runs as `with_retirement_child(&mut prefix.admission, admit)`; the writer emits into `output:Some(&mut prefix.bytes)`; UTF-8 is validated before the payload is taken.
- graph/table/value roots: `🫙️projection` mounted (`projection::project/admit`), graph/table `💰️reconstruction` mounted (`reconstruction::reconstruct`), old inline reconstruction (`reconstruct_value_forest`, `value_push`, `restore_properties`, `ordered`, `identities`, …) deleted; value root now `pub mod projection` and `pub use reconstruction::{ValueForestStorage, reconstruct_value_forest_into}`; orphan files had monolith paths (`crate::base_foundation`, `crate::standards::v1`) and are re-pointed to the crate names; `TreeScratch`, `visit`, `References` made `pub` for the cross-crate use.
- Docstring emoji uniqueness (`docstring-and-debug-census`): zero reuse in every touched native/encoder file (module doc counted).

## Language-agnostic vectors

- Existing shared corpus kept: `🧫️fixtures/🫙️prefix/*` plus the existing `🧫️fixtures/🧵️hex-utf8/🔣️.json` (13 positive / 16 negative UTF-8 and hex cases) is now also consumed by the new helpers.
- New `🧪️tests/🧫️fixtures/🧵️hex-octets/🔣️.json` (octets, refused octets, numeric words incl. NaN word, 384-byte stack bound, cancellation position).
- TS law `shared native hexadecimal vectors agree with independent SQLite and platform decoders`: third-party outputs from `bun:sqlite` (`CAST(x'..' AS TEXT)`, `hex()`), the platform strict `TextDecoder`, `Buffer`, `DataView` NaN class.
- Rust twin `native_prefix_shared_hex_vectors_fill_original_slots_and_retain_cancelled_prefixes` (in the existing `native_prefix` target) runs the same JSON through `hex_text_into`, `hex_bytes_into`, `hex_float` and checks the cancelled prefix stays in the original slot.

## How verified (nothing was compiled by cargo)

1. Bun laws (runner `pfx-tools/one.py`, emulated hoist like law2/law4): prefix suite 48/0; `pfx-tools/suite.py` over every semio `🪶️sqlite` test suite (document, value, model, brep, graph, video, image, audio, base x6, kit, mesh, object, presentation, flow, drawing, cad, animation, table, text x2): all `0 fail`.
2. `rustfmt --emit stdout --config skip_children=true` parse of the 103 snapshot `.rs` files of the 19 crates (natives, roots, projection/reconstruction, tests): 0 failures.
3. Name-resolution differential oracle (`pfx-tools/oracle.py`, `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null --error-format=short`, HEAD copy vs working copy, `extern crate` stripped, external globs neutralised): 19 crate roots, **0 new errors** (base shows only the expected unresolved-extern deltas in the two native files).
4. Typed stub harness (`pfx-tools/stub/`, plain `rustc --emit=metadata` of hand-written API stubs of `semio_framework_value`, `store`, artifact reference; sanitized real schema types; the REAL native sources included by path; output deleted): base crate (native decoder + encoder), all 18 domain native modules, the envelope `union_decoding` module and the new Rust vector test **type-check and borrow-check clean**. Stub signatures were copied from the framework sources (`NativeDecodeControl`, `ByteReader`, `owned_workspace`, `with_retirement_child`, `allocation_stage_native`, `ArtifactChild`); they are not a substitute for the compile executor but they closed every generic/closure/borrow risk of the new helpers.
5. Source-boundary closure counts of the three roots that mount extra files (`readRustSourceClosureV1`): graph 2 files/397 lines, table 15/1122, value 15/1192, base 15/1913 (limits 32/4000/262144 B).

## Rust verification to run (compile executor)

Packages (no new Cargo target, no manifest edit): `semio-s-artifact-stdio-semio-base`, `-brep`, `-mesh`, `-model`, `-value`, `-document`, `-cad`, `-drawing`, `-image`, `-video`, `-audio`, `-animation`, `-presentation`, `-flow`, `-text`, `-table`, `-graph`, `-object`, `-kit` and the aggregate `semio-s-artifact-stdio-semio`.

```
bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-stdio-semio-value -p semio-s-artifact-stdio-semio-document -p semio-s-artifact-stdio-semio-base
bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-stdio-semio-table -p semio-s-artifact-stdio-semio-graph -p semio-s-artifact-stdio-semio-presentation   # new projection/reconstruction mounts, highest residual risk
bun repo cargo --store=shared --configuration=prototype -- check --all-targets -p semio-s-artifact-stdio-semio
bun repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-semio --test native_prefix
bun repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-semio-graph   # hex-float tests now call workspace::decode with a leaked recipient
bun repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-semio-base
```

Residual risk, in order: (1) `graph/table/value` `🫙️projection` + `💰️reconstruction` bodies were authored by c44e9645 and never compiled; I only fixed their paths/visibility and mounted them (APIs `PagedList::{has_reserved_slot,next_allocation_bytes,reserve_one,push_reserved}`, `RowIndex::*`, `RowWriter::{census,new_into,finish_*}` exist by grep, not by compile); (2) framework signatures not covered by the stubs (`workspace::decode` bound `T::controlled_retirement_supported()` for each snapshot type, `RetireOwned` derive of `SemioSnapshot`); (3) runtime physical-balance assertions of the `native_prefix` Rust laws (heap witness) can only be judged by a run.

## Findings handed over

- `native_prefix` Rust laws 1 and 2 (`native_prefix_original_flow_success_and_cancelled_interior`, `native_prefix_missing_recipient_refuses_before_owned_materialization`) installed the caller recipient on the SQLite control and called `contract::testing::with_decoding`, which installs its own recipient on the native control and closes it inside the observed closure: that can never satisfy `released_bytes == 0`. The decode path (and the other fillers in the repo, e.g. part21 input) uses the **native control's** recipient through `workspace::decode`, so both tests now build a `NativeDecodeControl` with the test's recipient and a `NativeSnapshotDecodeOwner` explicitly (missing-recipient test: no recipient on the native control). Judge these two first when the compile executor runs `native_prefix`.
- `retire_sqlite_snapshot` overrides in the 18 roots (and base) still use `Owned::new` (ungranted drop). Replacement belongs to the Store erased-conversion owner, unchanged here.
- CAD `🫙️projection` is still an unmounted orphan (import path fixed only); Kit/Model/Document/Presentation/Drawing/Base projection callers remain the documented pending frontier.
- `🧬️schema/🫙️prefix/` is an empty directory since 0c3e7894 deleted the five corpus schemas together with the Ajv checks; not restored (current fixture-placement decision), no schema file added for the new vectors.

## Files

Production: `✉️base/🚪️io/🪶️sqlite/📸️snapshot/{🛬️native/🦀️.rs, 🛫️native/🦀️.rs, 🦀️.rs}`; `🛬️native/🦀️.rs` of the 18 domains (brep, mesh, model, value, document, cad, drawing, image, video, audio, animation, presentation, flow, text, table, graph, object, kit); `🕸️graph/…/{🦀️.rs, 🫙️projection/🦀️.rs, 💰️reconstruction/🦀️.rs}`; `📊️table/…/{🦀️.rs, 🫙️projection/🦀️.rs, 💰️reconstruction/🦀️.rs}`; `🔢️value/…/{🦀️.rs, 🫙️projection/🦀️.rs, 🫙️projection/🌳️tree/🦀️.rs}`; `📐️cad/…/🫙️projection/🦀️.rs` (import path).

Tests/fixtures: `✉️base/…/🧪️tests/🦀️.rs` (`with_body` call), `✉️base/…/🧪️tests/🫙️prefix/{🟦️.ts, 🦀️.rs}`, new `✉️base/…/🧪️tests/🧫️fixtures/🧵️hex-octets/🔣️.json`, `🕸️graph/…/🧪️tests/🦀️.rs` (`workspace::decode` helper).

Tools (inputs, kept): `pfx-tools/` (`one.py`, `suite.py`, `rs.py`, `dom.py`, `d_*.py` per-domain rewrites, `docs.py`, `prune.py`, `unused.py`, `oracle.py`, `stub/`). Generated output (`🗑️generated/pfx-out`, `🗑️generated/pfx-stub`, scratch sandboxes) deleted.
