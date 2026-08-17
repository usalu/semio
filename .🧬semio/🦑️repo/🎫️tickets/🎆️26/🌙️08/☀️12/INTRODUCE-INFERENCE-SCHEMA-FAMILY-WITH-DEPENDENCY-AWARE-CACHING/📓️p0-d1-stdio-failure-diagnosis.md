# P0-D1 — stdio 5-test failure diagnosis

Read-only diagnosis. No repo files edited by this session. Full raw output also saved at
`scratch-p0-d1-inference-rerun.txt` in this ticket folder.

## Reproduction

**Note on the first attempt**: my very first `cargo test -p semio-s-plugin-stdio --lib -- inference`
run silently produced a 38-byte output file containing only `tail: stdout: No space left on device`
— the disk really was full during that window (peer coordinator message confirmed: repo-root
`target/` was 428G, since deleted, 442GiB now free). That result was discarded per the coordinator's
instruction and everything below is a **fresh rerun after the disk fix and after UCAS's
`semio-framework-plugin` rename finished propagating** (peer-confirmed: 0 errors,
`Finished dev profile in 4m 20s`).

Command (cold rebuild after target-dir deletion, took 22m 26s to compile — this is normal, not a
hang):

```
CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING/🎯️target" \
cargo test -p semio-s-plugin-stdio --lib -- inference
```

Real tail of output:

```
warning: `semio-s-plugin-stdio` (lib test) generated 763 warnings (run `cargo fix --lib -p semio-s-plugin-stdio --tests` to apply 434 suggestions)
    Finished `test` profile [unoptimized] target(s) in 22m 26s
     Running unittests 📦️glue.rs (.../🎯️target/debug/deps/semio_s_plugin_stdio-2a4f49cf00e30d00)

running 98 tests
...
test artifacts::csv::standards::v_rfc4180::subsets::any::schema::inferences::component::tests::inference_default_law ... ok
...

failures:

---- artifacts::json::standards::v_rfc8259::subsets::any::schema::inferences::component::tests::inference_default_law stdout ----

thread '...' panicked at .../🔣️json/.../💡️inferences/🦀️component.rs:82:9:
assertion `left == right` failed
  left: JsonInference { outline: JsonOutline { node_count: 1, max_depth: 1, root_kind: "null" } }
 right: JsonInference { outline: JsonOutline { node_count: 0, max_depth: 0, root_kind: "" } }

---- artifacts::html::standards::v5::subsets::any::schema::inferences::component::tests::inference_default_law stdout ----

thread '...' panicked at .../🌐️html/.../💡️inferences/🦀️component.rs:82:9:
assertion `left == right` failed
  left: HtmlInference { outline: HtmlOutline { element_count: 1, max_depth: 1, text_length: 0 } }
 right: HtmlInference { outline: HtmlOutline { element_count: 0, max_depth: 0, text_length: 0 } }

---- artifacts::pdf::standards::v1_4::subsets::any::schema::inferences::component::tests::inference_default_law stdout ----

thread '...' panicked at .../📄️pdf/🏅️standards/🔖️1.4/.../💡️inferences/🦀️component.rs:82:9:
assertion `left == right` failed
  left: PdfInference { outline: PdfOutline { page_count: 1, word_count: 0, char_count: 0 } }
 right: PdfInference { outline: PdfOutline { page_count: 0, word_count: 0, char_count: 0 } }

---- artifacts::md::standards::v_commonmark::subsets::any::schema::inferences::outline::component::tests::collects_headings_and_counts_words_and_blocks stdout ----

thread '...' panicked at .../📝️md/.../💡️inferences/🧾outline/🦀️component.rs:116:9:
assertion `left == right` failed
  left: 4
 right: 3

failures:
    artifacts::html::standards::v5::subsets::any::schema::inferences::component::tests::inference_default_law
    artifacts::json::standards::v_rfc8259::subsets::any::schema::inferences::component::tests::inference_default_law
    artifacts::md::standards::v_commonmark::subsets::any::schema::inferences::outline::component::tests::collects_headings_and_counts_words_and_blocks
    artifacts::pdf::standards::v1_4::subsets::any::schema::inferences::component::tests::inference_default_law

test result: FAILED. 94 passed; 4 failed; 0 ignored; 0 measured; 1976 filtered out; finished in 1.68s
```

**Reproduced 4 of the 5 originally-listed failures, not 5** — see below. `cargo test -p
semio-s-plugin-stdio --lib -- collects_headings` reproduces the 5th (md) independently the same
way (same panic, same file:line).

### Why csv no longer fails

`csv`'s `inference_default_law` passed in this run. `git status --short` on
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs`
shows `M` (modified, not yet auto-committed) and `git log -1 -- <that path>` still shows only
`a46ac1f883` (flag 491) — i.e. **a peer session has this exact file open right now and has already
applied, uncommitted, the identical fix this report was about to prescribe**: `Default` dropped from
the `#[derive(...)]` list and replaced with a hand-written `impl Default for CsvOutline` returning
`{ record_count: 0, column_count: 0, has_header: true }`. This is concurrent churn, not something
this (read-only) session did. See `## Concurrent-churn observations`.

## Root cause per failure

All four remaining failures (`html`, `json`, `pdf` 1.4, `md`) are **CODE bugs, not test bugs** — but
two different code bugs:

### 1–3. html / json / pdf(1.4) `inference_default_law` — same bug, `#[derive(Default)]` on the outline struct disagrees with what `compute()` legitimately returns for that artifact's `Snapshot::default()`

The law itself is real and intentional — not a copy-paste mistake. The framework spine's own
canonical exemplar states it explicitly:
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:1119-1125`:

```rust
// 🎯️ Hand-written (not derived) `Default`: it must equal `infer(&i64::default())` for the
// default law to hold, and `0.is_even() == true` disagrees with `bool::default() == false`.
impl Default for AddInference {
    fn default() -> Self {
        AddInference { is_even: true, abs_value: 0 }
    }
}
```

i.e. the framework's OWN documented pattern for this exact situation is: hand-write `Default` on the
inferred struct to equal `compute()`'s honest output for the snapshot's default, not derive it.
`puzzle3d`'s exemplar (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/.../💡️inferences/🦀️component.rs:19-25`)
happens to get this for free via `#[derive(Default)]` only because its default snapshot has
genuinely-empty `objects`/`attractions` vecs, so `flatten_snapshot` trivially returns an empty
`BTreeMap` — same value a derived `Default` gives. That coincidence does NOT generalize.

For html/json/pdf(1.4), the artifact's honest default document is **not empty in the collection
sense** — each has a real, deliberately-documented non-zero fact baked into `Snapshot::default()`:

- **html**: `HtmlSnapshot::default().root = HtmlNode::element("html")` (snapshot component.rs:132-140)
  — there is always a root element. `HtmlOutline::compute` walks it and (correctly) reports
  `element_count: 1, max_depth: 1`. This is independently already asserted as correct by an existing,
  passing, hand-written test in the same file:
  `outline/🦀️component.rs:70-75 default_snapshot_has_one_element_and_no_text`. The derived
  `HtmlOutline::default()` gives `element_count: 0`, contradicting that other test.
- **json**: `JsonSnapshot::default().value = JsonValue::Null` (snapshot component.rs:447-454) —
  `JsonOutline::compute` correctly reports `node_count: 1, max_depth: 1, root_kind: "null"` (a null
  IS a node, per the outline's own docstring: "`nodeCount` is a real recursive walk over every
  `JsonValue` node"). Derived `JsonOutline::default()` gives `node_count: 0, root_kind: ""`
  (`""` is not a real `JsonValue` kind — it would be dishonest/fabricated data).
- **pdf 1.4**: `PdfOutline::compute` hardcodes `page_count: 1` always, and says so explicitly in its
  own docstring: "`pageCount` is always `1` — this subset's `PageDoc` models exactly one page — not
  fabricated, it's the honest shape of the snapshot itself" (outline/🦀️component.rs:1-4). Derived
  `PdfOutline::default()` gives `page_count: 0`, which — per that same docstring — would be
  dishonest for this subset (there is no zero-page state).

  Note: pdf **1.7** (`🏅️standards/🔖️1.7`) does NOT have this bug and needs no fix — its
  `Pdf17Outline::page_count = pages.len()` verbatim, and `PdfSnapshot::default().pages` (1.7 snapshot
  component.rs:185-196) really is `Vec::new()`, so `compute(default)` and derived `Default()` already
  agree (both 0). Only 1.4 is broken.

**Verdict: CODE is wrong (outline structs should have hand-written `Default`), TEST is correct** (it
is enforcing the framework's own documented law).

### 4. md `collects_headings_and_counts_words_and_blocks` — independent bug, wrong expected value in a hand-written fixture, no relation to the Default-derivation issue

This is a different test, not `inference_default_law`, and md's own `inference_default_law` (in
`💡️inferences/🦀️component.rs`) already passes (`MdSnapshot::default().blocks` really is
`Vec::new()`, an honest empty vec, so this artifact doesn't hit bug class #1-3 at all).

`MdOutline::compute`'s `walk_block` (`🧾outline/🦀️component.rs:52-83`) increments `block_count` by 1
on **every** call, unconditionally, at the top of the function — including calls for container nodes
(`BlockQuote`, `List`) — and then separately recurses into the container's children, each of which
gets its own `walk_block` call (and its own +1). This is stated as intentional in the function's own
docstring (`🧾outline/🦀️component.rs:5-7`): "`blockCount` is a real recursive walk counting every
`MdBlock` node (list items and **block-quote contents included**)".

The failing fixture (`outline/🦀️component.rs:106-113`) has 3 top-level blocks — `Heading`,
`Paragraph`, `BlockQuote { blocks: [Heading] }` — plus 1 block nested inside the `BlockQuote`. Per
the documented semantics that's `MdBlock` node count = 4 (`Heading1`, `Paragraph`, `BlockQuote`
itself, nested `Heading2`), which is exactly what the actual panic shows (`left: 4`). The test's
hardcoded expectation `assert_eq!(outline.block_count, 3)` (line 116) undercounts by one — it counts
only the 3 top-level entries and forgets the block-quote's own nested heading is a `MdBlock` node
too, contradicting the function's own docstring.

`section_outline` and `word_count` assertions in the same test both pass (verified: they're not in
the panic — only `block_count` failed), confirming the walk/flatten logic itself is otherwise sound;
this is a narrow off-by-one in the fixture's expected value, not a walk-logic bug.

**Verdict: CODE is correct (per its own docstring), TEST is wrong** (expected value should be `4`).

## Shared root cause?

**Two unrelated root causes, not one:**

- Bugs #1-3 (html, json, pdf 1.4) share ONE root cause: `#[derive(Default)]` was used on the outline
  struct instead of a hand-written `Default` matching `compute()`'s honest output for that artifact's
  default snapshot — the exact anti-pattern the framework spine's own `AddInference` exemplar warns
  about and shows how to fix. csv had the same bug (now independently being fixed by a peer, in
  flight, see above) — so this is really a **4-artifact instance of one pattern** (csv/html/json/pdf),
  of which peer already covered csv.
- Bug #4 (md) is unrelated: a plain arithmetic mistake in one hand-written test fixture. It does not
  share any code or root cause with #1-3.

## Fix plan

All four fixes below are self-contained edits to a single file each (three, since csv is already
being fixed by a peer session — verify its final state before closing rather than re-doing it).

### Fix A — html: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs`

Current lines 12-18:
```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlOutline {
    pub element_count: u32,
    pub max_depth: u32,
    pub text_length: u32,
}
```
Replace with (drop `Default` from the derive list; add a hand-written impl right after the struct,
mirroring the pattern csv's peer fix already landed):
```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlOutline {
    pub element_count: u32,
    pub max_depth: u32,
    pub text_length: u32,
}

/// 🎯️ Hand-written (not derived): must equal `compute(&HtmlSnapshot::default())` for
/// `inference_default_law` to hold — a default html document always has one root element
/// (`HtmlSnapshot::default().root`), which disagrees with `u32::default() == 0`.
impl Default for HtmlOutline {
    fn default() -> Self {
        Self { element_count: 1, max_depth: 1, text_length: 0 }
    }
}
```
(values already proven correct by the existing passing test `default_snapshot_has_one_element_and_no_text`, lines 70-75 of the same file — no need to re-derive them.)

### Fix B — json: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs`

Current lines 13-19:
```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonOutline {
    pub node_count: u32,
    pub max_depth: u32,
    pub root_kind: String,
}
```
Replace with:
```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonOutline {
    pub node_count: u32,
    pub max_depth: u32,
    pub root_kind: String,
}

/// 🎯️ Hand-written (not derived): must equal `compute(&JsonSnapshot::default())` for
/// `inference_default_law` to hold — `JsonSnapshot::default().value` is `JsonValue::Null` (a real
/// node), which disagrees with `u32::default() == 0` / `String::default() == ""`.
impl Default for JsonOutline {
    fn default() -> Self {
        Self { node_count: 1, max_depth: 1, root_kind: "null".to_string() }
    }
}
```
(values confirmed by the actual panic output above: `left: JsonOutline { node_count: 1, max_depth: 1, root_kind: "null" }`.)

### Fix C — pdf 1.4: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs`

Current lines 11-17:
```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfOutline {
    pub page_count: u32,
    pub word_count: u32,
    pub char_count: u32,
}
```
Replace with:
```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfOutline {
    pub page_count: u32,
    pub word_count: u32,
    pub char_count: u32,
}

/// 🎯️ Hand-written (not derived): must equal `compute(&PdfSnapshot::default())` for
/// `inference_default_law` to hold — this subset's `PageDoc` always models exactly one page (see
/// `compute`'s own doc above), which disagrees with `u32::default() == 0`.
impl Default for PdfOutline {
    fn default() -> Self {
        Self { page_count: 1, word_count: 0, char_count: 0 }
    }
}
```
Do NOT touch pdf **1.7**'s `Pdf17Outline` (`.../🏅️standards/🔖️1.7/.../🧾outline/🦀️component.rs`) — it
has no bug; its derived `Default` already agrees with `compute(&PdfSnapshot::default())` (both all-zero/`None`).

### Fix D — md: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs`

Line 116, currently:
```rust
        assert_eq!(outline.block_count, 3);
```
Replace with:
```rust
        assert_eq!(outline.block_count, 4); // Heading1 + Paragraph + BlockQuote + nested Heading2
```
Nothing else in that test changes — `section_outline` (line 115) and `word_count` (line 117) already
pass and are untouched by this. Do not touch `walk_block` / `MdOutline::compute` — they are correct.

### Verification command for whoever applies these

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-s-plugin-stdio --lib -- inference
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-s-plugin-stdio --lib -- collects_headings
```
Expect `test result: ok` for all four family-root `inference_default_law`s and for
`collects_headings_and_counts_words_and_blocks`, restoring the stdio long-profile baseline to
2026 passed / 0 failed (of the 5 originally red) / rest unchanged.

## Blast radius

**Independent of UCAS's stdio subset-roster restructure.** UCAS's #2548 carve-out is about
`🔣️taxonomy.json`-driven subset rosters and the framework `🔌️plugin/🦀️component.rs` /
`📡️spr` / kernel `💡️inference` module surface (the generic `Inference`/`InferenceSpec` traits,
`InferenceCache`, `infer_field` driver — read in full above, at
`🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️component.rs` and
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`). None of that is touched by
this fix plan. All four fixes are scoped to a single leaf `🧾outline/🦀️component.rs` file each,
inside `✳️any/🧬️schema/💡️inferences/`, for artifacts (csv/html/json/pdf/md) that are **not**
`🧿️semio` subsets and are not part of any roster move. Also confirmed: `Default` is never consulted
by the real runtime cache path — `ArtifactInferrer::infer_cached`'s default passthrough
(`🧰️framework/.../🔌️plugin/🦀️component.rs:917-926`) just calls `infer()` directly, and none of
csv/html/json/pdf override `infer_cached`, so these `Default` impls are a pure trait-bound +
test-law concern, not a live-cache-semantics concern — zero runtime behavior change, compile-only +
test-only blast radius.

## Concurrent-churn observations

- First `cargo test` attempt (before the coordinator's disk-full notice) returned a truncated
  38-byte file containing only `tail: stdout: No space left on device` — discarded, not used for any
  conclusion in this report.
- `df -h /` confirmed disk recovered to 432-442GiB free before the real rerun.
- `ps aux` during the ~25 min the rerun was compiling showed a large number of concurrent
  `cargo check`/`cargo test` processes from multiple peer sessions/tickets (SUBSET-CONFORMANCE...,
  UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, DISSOLVE-KERNELS-AND-MODULES..., trinity, fem, gis, surface,
  etc.) — consistent with `📌️important.md`'s warning that a slow/red build can be system-wide
  contention, not a local bug. This session's own build finished cleanly in 22m 26s once it got CPU
  time.
- **csv's `🧾outline/🦀️component.rs` is mid-edit by a peer session right now** (`git status --short`
  shows `M`, uncommitted; `git log -1` for that path still only shows the original flag-491 fan-out
  commit). The in-flight diff is, byte-for-byte in structure, the same fix this report independently
  derived for html/json/pdf (drop `Default` from the derive list, hand-write
  `impl Default for CsvOutline { ... has_header: true ... }`). Strong independent corroboration of
  the root-cause diagnosis and the prescribed fix shape. Whoever applies fixes A-D should re-check
  csv's final committed state rather than re-touching it.

## Honest pass/fail

- Reproduced the failures with real, freshly-run (post-disk-fix, post-UCAS-propagation) command
  output: **yes**, pasted verbatim above.
- All 5 originally-listed failures accounted for: **yes** — 4 reproduced red in this run
  (html/json/pdf-1.4/md); the 5th (csv) is explained as already-being-fixed-in-flight by a peer, with
  evidence (`git status`/`git log`), not assumed.
- Root cause identified with evidence (framework exemplar, snapshot Default impls, docstrings, actual
  panic left/right values) for all 5: **yes**.
- Test-vs-code verdict stated with evidence for each: **yes** — code wrong (needs hand-written
  `Default`) for csv/html/json/pdf-1.4; test wrong (off-by-one expected value) for md.
- Fix plan precise enough to apply without re-deriving anything: **yes** — exact file, exact current
  lines, exact replacement text, exact expected values (all independently cross-checked against
  either an existing passing test or the actual panic output, never guessed).
- No repo files edited by this (read-only) session: confirmed — only this ticket-folder report and
  `scratch-p0-d1-inference-rerun.txt` were written.
