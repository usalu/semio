# F6 Recon + Pilot Report — OpText/OpBinary/DiffCodec Derive-vs-Hand-Roll

**Scope**: recon step before Wave F6 (the final per-artifact wave). Located and read the real derive
machinery, piloted it for real on 3 representative standards (💾️binary, 🎞️gif 89a, 🎨️svg 1.1), verified
every claim by actually compiling and running `cargo test`, and swept all 31 standards for
classification. **All work is real and currently sitting in the live tree** (not reverted) — see
"Files touched" at the end. Full crate: **1019 passed, 0 failed** (baseline was 1013; +6 new law
tests). `bun ./📜️script.ts policy`'s `dsl-migration/diff-completeness` rule now shows exactly 28
stdio breaches (was 31 before this session) — binary/gif89a/svg confirmed dropped out by their new
`DiffCodec` impls, with zero edits to `📜️script.ts` itself.

## 0. Headline correction to the mission brief

The mission text says the derive "emits `OpText`/`OpBinary` automatically from a `RecordSpec`". **This
is stale.** The derive macro's own file header says otherwise:

> `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:5-6`
> "P6: `DslArtifact`/`DslOps` no longer emit `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` — those
> traits are handcrafted per artifact."

and at the `DslOps` derive itself (line 914): `// P6: DslOps emits DslVariants only — OpText/OpBinary
must be handcrafted per artifact.`

So: `#[derive(dsl::DslOps)]` on a Mutation enum gives you `dsl::DslVariants` (the record-spec-per-variant
machinery) for free, but you **always** hand-write a ~15-line `OpText` impl and an ~8-line `OpBinary`
impl on top of it. The good news: those two impls are **100% boilerplate, identical in every real
usage found in the repo** (`FlowMutationDsl`, `SpaceMutation`, and now `BinaryMutation`/`GifMutation`
below) — copy-paste, change the type name, done. See §2.

`#[derive(dsl::DslDiff)]` is unaffected by P6 — it still directly emits a full `protocol::DiffCodec`
impl. But it has its own, much more consequential restriction (§3).

## 1. Where the derive machinery lives

- Proc-macro source: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`
  - `#[proc_macro_derive(DslRecord, ...)]` (line 572) — plain struct → `DslField` (record shape). The
    field-classification engine (`classify_field`, line 224) is shared by every derive below.
  - `#[proc_macro_derive(DslArtifact, ...)]` (line 627) — struct → `DslField` + envelope constants
    (`__DSL_ENVELOPE_ID`/`__DSL_EXTENSION`); P6 no longer emits `ArtifactDsl`/`ArtifactPack` from this.
  - `#[proc_macro_derive(DslDiff, ...)]` (line 710) — **struct-only** (`Data::Struct` hard-checked at
    line 715-716, compile error otherwise) → real `protocol::DiffCodec` (`print_diff`/`parse_diff`/
    `encode_diff`/`decode_diff`), reusing `record_codegen`.
  - `#[proc_macro_derive(DslScalar, ...)]` (line 767) — **unit-variant-only** enum (line 778-779 hard
    error otherwise) → `DslField` (an `Shape::Enum` of kebab tags). This is how a plain enum like
    `GifDisposal` becomes usable as a struct field without becoming a "mutation-shaped" thing.
  - `#[proc_macro_derive(DslOps, ...)]` (line 905) — **enum-only** (`Data::Enum` hard-checked, line
    909-910) → `dsl::DslVariants` ONLY (line 914-915 comment). No `DslField` impl is emitted for the
    enum itself — you cannot embed a `DslOps`-derived enum as an ordinary struct field, only reach it
    via `#[dsl(statements)]` (Statements/sum-type fields) or the standalone `OpText`/`OpBinary` you
    write.
  - `#[proc_macro_derive(DslEnum, ...)]` (line 924) — same `DslVariants`-only shape as `DslOps` but for
    plain tagged-data enums that are NOT mutations (e.g. a recursive block-tree node kind).
- Runtime primitives the handcrafted `OpText`/`OpBinary` bodies call:
  `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`
  - `DslVariants` trait (line 233): `variants()`, `to_named_record()`, `from_named_record()`.
  - `dsl::print`/`dsl::parse` — the same text engine `DslRecord` uses.
  - `pub mod variants_binary` (line 296): `encode_op`/`decode_op`, generic over any `T: DslVariants`.
    Layout is **exactly** the mission's stated law: `format u8 (=1) | variant ordinal varint | record
    body` (line 294 doc comment, `OP_BINARY_FORMAT: u8 = 1` at line 301). Zero per-artifact code needed
    — literally `dsl::variants_binary::encode_op(self)` / `dsl::variants_binary::decode_op(bytes)`.
- `protocol::DiffCodec`/`OpText`/`OpBinary` trait definitions + their LAWS (verbatim, this is the
  normative contract every impl in this report satisfies):
  `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:126-165`.

### Real non-stdio usage found (proves the mechanism, used as templates)

- `FlowMutation`/`FlowMutationDsl` — `🧰️framework/…/🌊️flow/🌿️vcs/🦀️component.rs:685-826`. The canonical
  "domain enum ≠ Dsl enum" pattern: `FlowMutationDsl` is a separate, `DslOps`-derived mirror enum with
  flat/renamed fields (its variants use `#[dsl(key=...)]`/`#[dsl(block)]`), and
  `flow_mutation_to_dsl`/`flow_mutation_from_dsl` bridge to/from the real `FlowMutation`. Its
  `impl OpText for FlowMutationDsl` (line 770) is the literal template every hand-rolled `OpText`
  impl in this report copies. `impl OpBinary for FlowMutation` (line 816) forwards to
  `FlowMutationDsl`'s `OpBinary`, itself forwarding to `dsl::variants_binary`.
- `SpaceMutation`/`SpaceDiff` — `🧰️framework/…/🪐️space/🦀️component.rs:147-364`. Simpler: `SpaceMutation`
  is `DslOps`-derived DIRECTLY (no separate Dsl mirror needed, because none of its variants carry an
  enum-shaped payload — see §3). `SpaceDiff` (line 238) is `DslDiff`-derived directly: a FLAT
  "one `Option<T>` per possible mutation" struct (mirrors `writer_op::WriterDiff`'s shape), explicitly
  documented at line 198-200 as "the pattern every `#[derive(dsl::DslDiff)]` struct uses (the derive
  only supports structs, never tagged enums)". **This is the only real precedent for `DslDiff`
  actually being used on a "collection-shaped" diff**, and its diff is NOT the stdio recipe's nested
  collection-triple shape (`removed`/`modified`/`added`) — it's flatter. gif 89a below is the FIRST
  real attempt at deriving the recipe's actual nested-collection diff shape, and it fails for an
  unrelated reason (§3).
- `WriterDiff` (`✏️s/🔌️plugins/✒️writer/…/🔺️diff/📝️text/🦀️component.rs:237-262`) — the repo's **only**
  other hand-rolled `DiffCodec` impl before this session. Important finding: **it uses
  `serde_json::to_string`/`from_str`** for `print_diff`/`parse_diff` — i.e. it satisfies the trait's
  LAWS (one line, round-trips, deterministic) but is NOT a genuinely handcrafted one-line grammar
  either. It is not a usable template for "how to hand-write a real grammar" — only for "the encode_diff
  = print_diff().into_bytes() binary-format shortcut is a legitimate, precedent-backed simplification."
  Do not copy its `print_diff`/`parse_diff` bodies.

## 2. The OpText/OpBinary handcrafted-wrapper pattern (identical every time)

Once a Mutation enum's whole payload-type closure implements `DslField` (see §3 for when it doesn't),
`#[derive(dsl::DslOps)]` gives you `DslVariants`, and this pair of impls is **always the same shape** —
verified working in this pilot (`BinaryMutation`, `GifMutation`) and matches the framework precedent
(`FlowMutationDsl`, `SpaceMutation`) exactly:

```rust
impl protocol::OpText for XMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}
impl protocol::OpBinary for XMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}
```

Real captured output (binary, from the pilot's own test run):
- `NoMutation` → `"no-mutation"`
- `SetSnapshot { snapshot: BinarySnapshot { schema: "stdio.binary", bytes: [9,9] } }` → `"set-snapshot snapshot { schema=stdio.binary bytes="CQk=" }"`
- `Splice { offset: 1, remove_len: 2, insert: [170,187,204] }` → `"splice offset=1 remove-len=2 insert="qrvM""`
- `AppendBytes { data: [238,255] }` → `"append-bytes data="7v8=""`

Real captured output (gif 89a, ~20-variant enum, incl. a variant carrying a whole nested struct):
- `InsertFrame { index: 1, frame: GifFrame{..} }` prints via the same mechanism (verified round-trip in
  `op_text_binary_roundtrip_law`, 27/27 tests green) — no special-casing needed for a variant whose
  payload is an entire nested record.

## 3. The decision rule — WHY derive fails, precisely

There are **two independent, unrelated failure modes**, both confirmed by real `cargo check` errors
(not guessed). Either one alone forces hand-rolling for that side (OpText/OpBinary via `DslOps`, or
DiffCodec via `DslDiff`). A single artifact commonly hits both.

### 3a. Enum-in-tree (the documented one)

`DslField` — the trait every struct field's type must implement for `DslRecord`/`DslDiff`/`DslOps`
field codegen to compile — has exactly two derive sources: `#[derive(DslRecord)]` (plain structs) and
`#[derive(DslScalar)]` (**unit-variant-only** enums). A genuine **data-carrying enum** (any variant
with fields) has NO `DslField` impl, derivable or otherwise, and none can be added downstream (orphan
rule — `DslField` and the enum type are both foreign to a stdio artifact's own crate boundary is not
even the blocker here; the blocker is simpler: nobody, anywhere in this codebase, has an
`impl<T: DslVariants> DslField for T` bridge, and adding one is a framework-level design decision, not
a per-artifact one).

Confirmed via real compile error (`SvgDiff` with `#[derive(dsl::DslDiff)]` added):
```
error[E0277]: the trait bound `v1_1::...::SvgNodeDiff: DslField` is not satisfied
  --> …/🎨️svg/…/🔺️diff/🦀️component.rs:21:84
help: the trait `DslField` is not implemented for `SvgNodeDiff`
```
and the SAME rule on the Mutation side (`SvgMutation` with `#[derive(dsl::DslOps)]` added — this is a
**new finding beyond what the mission asked to check**, since the mission only asked about `SvgDiff`):
```
error[E0277]: the trait bound `...::XmlNode: DslField` is not satisfied
  --> …/🎨️svg/…/🧬️mutations/🦀️component.rs:44:15   (InsertElement { node: XmlNode })
error[E0277]: the trait bound `v1_1::...::SvgSnapshot: DslField` is not satisfied
  --> …/🎨️svg/…/🧬️mutations/🦀️component.rs:30:19   (SetSnapshot { snapshot: SvgSnapshot })
```
**`DslOps` fails for exactly the same structural reason as `DslDiff`** — a Mutation enum's `SetSnapshot`
variant ALWAYS carries the whole Snapshot type (every one of the 31 stdio artifacts does this by the
recipe's own design), so if the Snapshot tree contains a data-carrying enum ANYWHERE, `DslOps`
derivation fails too, independent of `DslDiff`. Some variants (svg's `InsertElement`/`SetTransform`)
also carry an enum-shaped payload DIRECTLY, which would still fail even if `SetSnapshot` didn't.

### 3b. Tri-state `Option<Option<T>>` (the UNDOCUMENTED, more consequential one)

This is the recon's most important finding. The top-level plan's own normative Diff design (line 34 of
`the-current-schemas-are-scalable-journal.md`) mandates tri-state for every nullable snapshot field:
`pub field_b: Option<Option<B>>, // nullable field tri-state: Some(None) = removed`. **The derive
cannot bind this at all — not "badly", not "verbosely", it is a hard compile error — even when there
is NOT a single enum anywhere in the type.**

Root cause, precisely: `dsl_derive::classify_field` (line 224) peels exactly ONE `Option<..>` layer via
`inner_of(ty, "Option")` (line 185) *before* checking any other attribute, then binds the REMAINING
type through `<remaining as DslField>::from_value`/`to_value` (the `OptionScalar` arm, line 399-413).
For a field typed `Option<Option<GifColorTable>>`, the remaining type after one peel is
`Option<GifColorTable>` itself — and there is **no `impl<T: DslField> DslField for Option<T>`
anywhere in the `dsl` crate** (confirmed by grep — only `Vec<T>`, `BTreeMap<String,T>`, `[T;N]` have
such blanket impls, `🧰️framework/…/🗣️dsl/🦀️component.rs:159-226`). So `Option<Option<T>>` is
structurally unbindable by this derive, period, regardless of what `T` is.

Confirmed via real compile error (`GifFrameDiff`/`GifDiff` with `#[derive(dsl::DslRecord)]`/
`#[derive(dsl::DslDiff)]` added):
```
error[E0277]: the trait bound `std::option::Option<GifColorTable>: DslField` is not satisfied
   --> …/🎞️gif/…/🔺️diff/🦀️component.rs:192:21   (pub lct: Option<Option<GifColorTable>>)
error[E0277]: the trait bound `std::option::Option<u8>: DslField` is not satisfied
   --> …/🔺️diff/🦀️component.rs:205:35   (pub transparent_index: Option<Option<u8>>)
```

**Consequence**: since tri-state is the plan's own mandated shape for every nullable field, `DslDiff`
is blocked for the overwhelming majority of the 31 diff types **even where there is zero enum in the
tree** — this is a bigger practical restriction than the documented "enum-only" one. `BinaryDiff`
(the one artifact whose diff has NO nullable field at all — just `splices: Vec<ByteSplice>`) is why it
was the one clean, complete derive success in this pilot.

### The unified decision rule (use this, not "does it have an enum")

For a given artifact's **Diff** type (governs whether `#[derive(dsl::DslDiff)]` works):
> Walk every field in the diff struct, recursively through every nested diff/value struct it embeds.
> If **any** field, anywhere in that walk, is typed `Option<Option<_>>` (or deeper) — hand-roll. Else
> if **any** field's (possibly-unwrapped-through-`Option`/`Vec`) type is a data-carrying enum (a `pub
> enum` with any non-unit variant) — hand-roll. Else — the derive works, but EVERY struct type
> anywhere in that walk (including ones declared in the SNAPSHOT module, e.g. `GifRgb`) needs
> `#[derive(dsl::DslRecord)]` added to it too (cascading requirement — the derive gives you nothing
> for a plain `serde`-only nested struct), and any nested `Vec<u8>` field wants `#[dsl(base64)]` for a
> compact grammar (see the caveat below — it does NOT work through `Option<Vec<u8>>`, only bare
> `Vec<u8>`).

For a given artifact's **Mutation** enum (governs whether `#[derive(dsl::DslOps)]` works cleanly, i.e.
whether you can skip the "separate Dsl-mirror-enum + conversion functions" step `FlowMutation` needed):
> Same walk, but over EVERY variant's fields (mutation payloads don't usually have tri-state — a
> mutation's `Option<T>` argument means "the new value", not "diff removed-vs-unchanged" — so 3a is
> almost always the only blocker here, 3b rarely applies). Since `SetSnapshot` always carries the WHOLE
> Snapshot, this reduces to: **does the Snapshot type (recursively) contain a data-carrying enum
> anywhere?** If yes, hand-roll (or introduce a `FlowMutationDsl`-style mirror enum + conversion
> functions if only `SetSnapshot`'s payload is the problem and every other variant is otherwise clean —
> not attempted in this pilot, flagged as a viable third option for a future agent with more budget).

### Known derive quirk found in passing (harmless, but worth knowing before debugging it blind)
`#[dsl(base64)]` on a field does **nothing** when the field is `Option<Vec<u8>>` — `classify_field`
peels the `Option` unconditionally before ever checking `attrs.base64`, so it silently falls back to
`Shape::List(UInt)` (a verbose bracketed list of decimal byte values) instead of compact base64. Not a
compile error, just a token-inefficiency trap. Confirmed by testing `GifFrameDiff::indices: Option<Vec<u8>>`
with `#[dsl(base64)]` attached — it compiles and round-trips fine, just verbosely. Only bare (non-Option)
`Vec<u8>` fields actually get the compact `Bytes64` shape.

## 4. Worked example — derive path (💾️binary, the clean case)

`BinarySnapshot`/`ByteSplice`/`BinaryDiff`/`BinaryMutation` have zero enums and zero tri-state fields.
Applied the derive directly:

```rust
// 📸️ snapshot/component.rs — DslRecord added so SetSnapshot{snapshot: BinarySnapshot} can embed it
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[artifact_schema(id = "s.stdio.binary")]
pub struct BinarySnapshot {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] #[serde(default)] #[dsl(base64)] pub bytes: Vec<u8>,
}

// 🔺️ diff/component.rs
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ByteSplice { pub offset: usize, pub remove_len: usize, #[dsl(base64)] pub insert: Vec<u8> }

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]
#[artifact_schema(id = "s.stdio.binary.diff")]
pub struct BinaryDiff {
    #[state(persistent)] #[serde(default, skip_serializing_if = "Vec::is_empty")] pub splices: Vec<ByteSplice>,
}
// protocol::DiffCodec for BinaryDiff is now FULLY GENERATED — no hand-written impl at all.

// 🧬️ mutations/component.rs
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum BinaryMutation {
    #[default] NoMutation,
    SetSnapshot { #[dsl(block)] snapshot: BinarySnapshot },
    Splice { offset: usize, remove_len: usize, #[dsl(base64)] insert: Vec<u8> },
    AppendBytes { #[dsl(base64)] data: Vec<u8> },
    TruncateAt { offset: usize },
}
// + the §2 handcrafted OpText/OpBinary wrapper (unavoidable even here, P6).
```

Real captured `print_diff` output: `"splices=[ offset=1 remove-len=2 insert="CQkJ" ]"` and
`"splices=[ offset=0 remove-len=0 insert="" offset=5 remove-len=1 insert="qrs=" ]"`.

Verified: `cargo test -p semio-s-plugin-stdio --lib "artifacts::binary"` → **18/18 passed**, including
new `op_text_binary_roundtrip_law` and `diff_codec_text_binary_roundtrip_law`.

## 5. Worked example — hand-rolled path (🎞️gif 89a AND 🎨️svg 1.1)

Both `GifDiff` (blocked by 3b, tri-state — `gct`/`loop_count` at top level, `lct`/`transparent_index`/
`plain_text` inside `GifFrameDiff`) and `SvgDiff` (blocked by BOTH 3a — `SvgNodeDiff` is a real enum —
AND 3b — `declaration`/`doctype`) needed a real hand-rolled `impl protocol::DiffCodec`. `SvgMutation`
also needed a hand-rolled `OpText`/`OpBinary` (3a, via `SetSnapshot`→`SvgSnapshot`→`XmlNode` and
`InsertElement.node: XmlNode` directly) — `GifMutation` did NOT (its Snapshot tree has no enum, so
`#[derive(dsl::DslOps)]` worked cleanly on it even though `GifDiff` needed hand-rolling — **the two
sides of one artifact can and do land on different paths**).

### The grammar template (reusable, not project-specific)

Both hand-rolled codecs in this pilot use the SAME small set of primitives (duplicated per-file today,
by design — see the "known duplication" note below):

```rust
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
fn hex_decode(s: &str) -> Result<Vec<u8>, String> { /* pairs of hex digits -> bytes */ }
// Bracket-depth-aware split — the whole grammar's parsing primitive. Tracks '[' / ']' only.
fn split_top_level(s: &str, sep: char) -> Vec<&str> { /* … */ }
fn strip_brackets(s: &str) -> Result<&str, String> { /* … */ }
// Uniform Option<T> tag: None -> "[0]", Some(v) -> "[1,<enc(v)>]"
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String { /* … */ }
fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> { /* … */ }
```
- Plain structs: positional `[f1,f2,f3]` tuples (order = declaration order, both sides must agree).
- Strings/byte arrays: hex (not base64 — no external dep, and this artifact family's own
  `ArtifactDsl::parse_dsl`/`print_dsl` already use hex for the same reason, so it's the established
  local idiom, not an invented one).
- Data-carrying enums (`XmlNode`, `SvgNodeDiff`, `TransformOp`): single-uppercase-letter tag prefix
  immediately followed by the bracketed positional payload — `E[...]`/`T[...]`/`R[...]` etc. Tag chosen
  distinct per variant, documented in a doc comment next to the `enc_*`/`dec_*` pair.
- Collection triples (`removed`/`modified`/`added`, the recipe's own shape): `name{[removed];[modified];[added]}`
  — semicolon-separated sections, each itself a comma-separated list; `modified`/`added` entries are
  `idx:payload` (colon-separated, unambiguous because `idx` is always a bare decimal preceding the
  first colon in the whole entry, and nothing else at that parse depth uses `:`).
- Top-level Diff/Mutation line: space-separated `name=value` tokens (Diff) or `keyword arg=value ...`
  (Mutation), one token per changed field / per mutation — absent token = unchanged. No token, no
  value anywhere, ever contains a literal space (hex has none, all separators are `,`/`;`/`:`/`[`/`]`),
  so top-level tokenizing is a trivial `line.split(' ')`.
- `encode_diff`/`encode_op` = `print_diff()/print_op().into_bytes()` — same simplification `WriterDiff`
  (the repo's only pre-existing hand-rolled `DiffCodec`) uses. Satisfies every LAW in the trait
  contract (round-trips, deterministic, no `\n`). A real binary-optimized encoding is a valid future
  tightening, not required by the trait.

Real captured `print_diff` output (`GifDiff`, exercising both tri-states + all 3 collection triples at
once via a real `between()` result):
```
"width=20 height=16 gct=[0] loop=[1,5] frames{[];[0:[S:p,P:[0],Q:[1,[1,1,2,2,4,4,0,1,6869]]],1:[W:3,H:3,C:[1,[0,[[6,6,6],[6,6,6]]]],X:000000000000000000]];[2:[0,0,3,3,0,[1,[0,[[7,7,7],[7,7,7]]]],000000000000000000,10,d,[0],0,[0]]]} comments{[0];[];[]} appext{[0];[];[]}"
```

Real captured `print_diff` output (`SvgDiff`, exercising the recursive `SvgNodeDiff` enum tree +
attribute/child add/remove/modify + both tri-states):
```
"declaration=[1,[312e30,[1,5554462d38],[1,1]]] doctype=[1,3c21444f4354595045207376673e] root=E[[0],[1,[];[7769647468:3230];[1:686569676874:3330]],[1,[];[0:E[[1,636972636c65],[1,[78];[];[0:72:35]],[0]]];[1:T[6869]]]]"
```

Real captured `print_op` output (`SvgMutation::InsertElement`, the enum-payload-in-a-variant case):
```
"insert-element parent=[] index=1 node=E[636972636c65,[[72,31]],[]]"
```

Verified:
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::gif::standards::v89a"` → **27/27 passed**
  (incl. new `diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::svg"` → **60/60 passed** (incl. new
  `handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`).

### Known duplication (flag, don't fix here)
Every hand-rolled codec in this pilot re-declares its own `hex_encode`/`split_top_level`/
`encode_option`/etc. — `SvgDiff`'s copies were made `pub(crate)` so `SvgMutation` could reuse them
(same artifact, adjacent facet — legitimate intra-artifact reuse, not a new shared module), but
`GifDiff`'s copies are separate again. **No shared "hand-roll helpers" crate/module exists.** Once ≥3
artifacts hand-roll (this pilot is already 2), a future closer-level pass should consider extracting
these ~10 functions into a small internal helper module every hand-rolling F6 agent can depend on —
noted, not done here (would be a shared-file edit, out of scope for a single artifact's ownership
boundary, and this recon step is explicitly told not to touch shared files).

## 6. Schema-id convention

Two SEPARATE, non-conflicting id namespaces exist — do not confuse them:

1. **Already wired, already used by all 31 diffs since F1-F5**: `#[artifact_schema(id = "s.stdio.<artifact>[.<standard>].diff")]` on the Diff struct (the `ArtifactSchema`/`schema` crate's OWN descriptor-id convention, `"s."`-prefixed, dot-separated). Examples confirmed in this pilot's own files: `"s.stdio.binary.diff"`, `"s.stdio.gif.89a.diff"`, `"s.stdio.svg.diff"`. **F6 agents do nothing here — it's already correct on every artifact from prior waves.**
2. **The plan's B-R4 convention**, `"<doc-schema>#diff"` (hash-separated, built from the DOCUMENT schema constant, e.g. `STDIO_GIF89A_DOCUMENT_SCHEMA = "stdio.gif.89a"` → diff-schema-id would be `"stdio.gif.89a#diff"`; binary → `"stdio.binary#diff"`; svg → `"stdio.svg#diff"`). **This string is not consumed anywhere in stdio's code today.** It only appears in two doc comments (`protocol::DiffCodec`'s trait doc, `dsl_registry`'s module doc) and the one live registry that was meant to use it — `dsl_registry::full_resolver()` (`🧰️framework/…/🗣️dsl/📇️registry/🦀️component.rs:33-38`) — returns an **empty** `HashMap`, even for its own named "writer/note" W1 pilots. **No F6 agent needs to register a `"#diff"`-suffixed string anywhere** — just implement `DiffCodec`'s 4 methods; there is no call site waiting for that string in the stdio plugin today.

The doc-schema constant naming pattern (needed if you ever DO want to build the `#diff` string):
`pub const STDIO_<ARTIFACT>[<STANDARD>]_DOCUMENT_SCHEMA: &str = "stdio.<artifact>[.<standard>]";` —
e.g. `STDIO_GIF89A_DOCUMENT_SCHEMA`, `STDIO_PDF17_DOCUMENT_SCHEMA`, `STDIO_IFC2X3_DOCUMENT_SCHEMA`,
declared in the standard's `📸️snapshot/component.rs`, or at the artifact root
(`STDIO_BINARY_DOCUMENT_SCHEMA`, `STDIO_SVG_DOCUMENT_SCHEMA`) for single-standard artifacts.

## 7. Live policy state (observed, not edited — `📜️script.ts` untouched)

Ran `bun ./📜️script.ts policy` for real (full output: `f6-recon-policy-run.txt` in this folder, 21619
lines, includes ~158 total `diff-completeness` breaches repo-wide, most unrelated to stdio — matches
the "22,198 pre-existing unrelated breaches, not this program's scope" framing from S2's report for
the broader `policy` command).

**`dsl-migration/diff-completeness` rule, stdio-scoped**: before this session, all 31 stdio diff files
would have failed it (none had `dsl::DslDiff` or `DiffCodec for` in their own file text — the check is
literal-text, file-level, see `policyDiffCompletenessBreaches` at `📜️script.ts:3185-3205`). **After
this session's edits, exactly 28 remain** (verified via `grep` on the fresh policy output) — precisely
the 28 stdio diff files NOT touched in this pilot. `💾️binary`, `🎞️gif 89a`, `🎨️svg 1.1` are confirmed
absent from the breach list. **`POLICY_DIFF_COMPLETENESS_ALLOWLIST` (📜️script.ts:2304) currently
contains ZERO stdio paths** — meaning this policy was already silently tracking all 31 as breaches
without anyone having added them to the allowlist (the check just wasn't being run/reported against
stdio specifically before now, or was accepted as part of the general noise floor). **This means F6
doesn't need to ADD entries to the allowlist and then shrink them — it needs to make the 28 remaining
breaches disappear by giving those files real `DiffCodec` impls, exactly like this pilot did for the
first 3.** Do not add stdio paths to the allowlist as a shortcut — that would be the wrong direction
(the mission's own goal is "zero stdio entries", and there are already zero; adding some to "pass" the
check while deferring the real work would be exactly backwards).

**No equivalent live policy exists for `OpText`/`OpBinary` completeness on stdio.**
`policyDslCompletenessBreaches` (📜️script.ts:3071) exists but is scoped to `impl DocumentApp for X`
app usages (`🎛️apps` pattern) — stdio's artifacts are a "zero-app library" (own Cargo.toml package
description) and never implement `DocumentApp`, so this check structurally never sees stdio's Mutation
types. F6 agents doing the OpText/OpBinary side get no automated policy verification the DiffCodec
side gets — rely on the per-artifact `cargo test` law suite instead.

## 8. Classification of all 31 standards

Swept every standard's `🔺️diff/🦀️component.rs` for (a) `pub enum` declared IN that file (proxy for
3a — a diff-shaped enum node) and (b) `Option<Option<` occurrences (exact match for 3b — tri-state).
This catches the Diff-side question precisely. **The Mutation-side question (does `DslOps` also work)
is a SEPARATE check per artifact** — walk the Snapshot type tree the same way, or just add
`#[derive(dsl::DslOps)]` and read the compiler's `DslField is not implemented for X` errors, exactly as
this pilot did for svg. Do not assume Diff-side and Mutation-side classifications are the same for one
artifact (gif 89a is the proof they can differ: Diff hand-rolled, Mutation derived clean).

Legend: **DERIVE** = `#[derive(dsl::DslDiff)]` should compile once every nested struct also gets
`#[derive(dsl::DslRecord)]` (cascading — budget real time for this, it's not zero-effort even in the
"clean" case, see §4's `BinarySnapshot`/`ByteSplice` needing their own derives too). **HAND-ROLL** =
confirmed or near-certain blocker present, use §5's template. **CHECK-ENUM-ELSEWHERE** = this file's
own sweep found nothing, but the artifact's snapshot/diff pulls in a shared node model from ANOTHER
artifact's module (xml, gltf-style) that might itself contain an enum not visible to this
single-file grep — verify before trusting the DERIVE classification.

| # | Artifact | Standard | `pub enum` in diff file | `Option<Option<` count | Diff-side verdict | Notes |
|---|---|---|---|---|---|---|
| 1 | 💾️binary | raw | 0 | 0 | **PILOTED — DERIVE ✅ (done, real, tested)** | Both diff+mutation derived clean. |
| 2 | 🎞️gif | 89a | 0 | 7 | **PILOTED — HAND-ROLL ✅ (done, real, tested)** | Diff hand-rolled (3b only); Mutation `DslOps` DERIVED clean (no enum in GifSnapshot tree). |
| 3 | 🎨️svg | 1.1 | 1 (`SvgNodeDiff`) | 3 | **PILOTED — HAND-ROLL ✅ (done, real, tested)** | Both sides hand-rolled (3a AND 3b on diff; 3a on mutation via `XmlNode`). |
| 4 | ☁️ply | 1.0 | 0 | 0 | DERIVE (probable) | No signal either way in this file; verify no enum in `PlySnapshot`/element/property model before trusting. |
| 5 | 🏗️ifc | 2x3 | 0 | 0 | DERIVE (probable) | Not one of the official 31 (32nd, added by a sibling ticket, per STATUS.md — confirm scope before spending an agent on it). Shares model shape with ifc/4. |
| 6 | 🏗️ifc | 4 | 0 | 0 | DERIVE (probable) | `IfcValue`/`IfcEntity` own types (per F4 report) — verify `IfcValue` isn't itself a data enum reachable from the diff; W0 flagged ifc as having a severe pre-existing type-boundary defect, re-check. |
| 7 | 📄txt | utf-8 | 0 | 0 | DERIVE (probable) | Simple `lines: Vec<String>` shape. |
| 8 | 📄️pdf | 1.4 | 0 | 0 | DERIVE (probable) | Simpler than 1.7 (no `PdfValue` object-graph enum reachable here per this file's own sweep) — CHECK-ENUM-ELSEWHERE anyway since pdf 1.7's file (same family) has 2. |
| 9 | 📊️csv | rfc4180 | 0 | 0 | DERIVE (probable) | |
| 10 | 📐️step | ap214 | 0 | 0 | DERIVE (probable) | `StepValue` enum exists in the snapshot (per plan's completeness table: "own value enum ($,*,refs,enums,aggregates)") — CHECK-ENUM-ELSEWHERE, this is a strong candidate for actually being HAND-ROLL once `StepValue`'s reachability into the diff is checked; 0 enums in the diff FILE itself is not proof `StepEntityDiff`'s `args: Vec<StepValue>`-shaped fields (if any) don't reach it. |
| 11 | 📕️xlsx | ecma-376 | 0 | 0 | DERIVE (probable) | OPC model — CHECK-ENUM-ELSEWHERE (own copy per F5, `XlsxCell` may have a variant-shaped value type). |
| 12 | 🖊️dwg | ac1018 | 0 | 0 | DERIVE (probable) | Frozen/opaque-by-spec boundary per F5 — likely genuinely flat. |
| 13 | 🖊️dwg | ac1024 | 0 | 0 | DERIVE (probable) | Same family as ac1018. |
| 14 | 🖼️bmp | v3 | 0 | 0 | DERIVE (probable) | Flat header + palette + rows. |
| 15 | 🖼️tiff | 6.0 | 0 | 0 | DERIVE (probable) | Plan's table says `TiffValues` union for tag entries — CHECK-ENUM-ELSEWHERE, same caveat as step. |
| 16 | 🟪️stl | ascii | 0 | 0 | DERIVE (probable) | Simplest artifact in the set besides binary. |
| 17 | 🎞️gif | 87a | 0 | 2 | HAND-ROLL (3b) | Same family/pattern as 89a, simpler (no GCE). Copy gif 89a's template almost directly, smaller field set. |
| 18 | 🎒️zip | 2.0 | 0 | 1 | HAND-ROLL (3b) | Likely the `unix_mtime: Option<Option<i64>>` tri-state the plan's worked design calls out by name. Small, single tri-state field — cheap hand-roll. |
| 19 | 💬️bcf | 2.1 | 0 | 4 | HAND-ROLL (3b) | guid-keyed topics/comments/viewpoints — 4 tri-states, more surface than zip but still enum-free by this file's sweep. |
| 20 | 📷️png | 1.2 | 0 | 12 | HAND-ROLL (3b) | 12 tri-states (many optional ancillary chunks: gAMA/cHRM/sRGB/pHYs/tIME/bKGD each nullable) — largest enum-free tri-state surface in the set; budget real time. |
| 21 | 🗜️deflate | rfc1950 | 0 | 2 | HAND-ROLL (3b) | `dict_id: Option<Option<u32>>`-shaped, small. |
| 22 | 🧊️obj | 3.0 | 0 | 3 | HAND-ROLL (3b) | |
| 23 | 🧊️gltf | 2.0 | 0 | **42** | HAND-ROLL (3b) — LARGE | By far the biggest tri-state surface (42) — the fully-typed 2.0 model (materials/textures/samplers/animations/... each with many optional sub-fields) makes this the most expensive single hand-roll in the remaining 28. Consider splitting across the field groups (asset/scenes/nodes vs materials/textures vs animations) if agent-budget-constrained. Confirm 0 enums holds (glTF's own extras/extensions use a local `GltfJson` type per F4 — verify it isn't itself a diff-reachable enum). |
| 24 | 🎞️pptx | ecma-376 | 1 | 1 | HAND-ROLL (3a+3b) | Shape-tree reconstruction (F5 flagged defect fix) — almost certainly a node-kind enum analogous to `SvgNodeDiff`/`XmlNode`. Copy svg's template. |
| 25 | 📄️pdf | 1.7 | 2 | 2 | HAND-ROLL (3a+3b) | `PdfValue` object-graph enum (`Stream{dict,raw,decoded}` etc.) — 2 enums declared directly in the diff file. Real object-graph recursion, closer to svg/xml in shape than gif. |
| 26 | 📜️docx | ecma-376 | 1 | 2 | HAND-ROLL (3a+3b) | Body block tree (paragraphs/runs/tables) — same "recursive node-kind enum" family as svg/xml/md/pptx. OPC-diff-type duplication flagged in F5's report as a separate, unrelated cleanup item — don't conflate. |
| 27 | 📝️md | commonmark | 3 | 2 | HAND-ROLL (3a+3b) — LARGEST enum count | `MdBlock`/`MdInline` trees — 3 enums declared directly in the diff file, most of any standard. Budget accordingly; likely the 2nd-most-expensive hand-roll after gltf. |
| 28 | 📰xml | 1.0 | 1 (`XmlNodeDiff`) | 2 | HAND-ROLL (3a+3b) | **Do this one WITH or RIGHT AFTER svg** (svg embeds xml's node model but declares its OWN diff types per the spec-mandated-reuse rule — svg's `enc_xml_node`/`dec_xml_node` in this pilot are DIRECTLY reusable as a starting point for xml's OWN `XmlNodeDiff` hand-roll, since `XmlNode` itself is identical). Ordering win, same as the original plan's xml-before-svg snapshot-embedding rationale. |
| 29 | 📷️jpg | jfif-1.01 | 1 | 3 | HAND-ROLL (3a+3b) | `JpgOtherSegmentsDiff`/`JpgFrameFieldsDiff` types visible in F3b's own dead-code warnings (this session's `cargo check` output) — real collection-triple + tri-state shape, 1 enum. |
| 30 | 🔣️json | rfc8259 | 1 (`JsonValueDiff` presumably) | 0 | HAND-ROLL (3a only) | Own `JsonValue` enum (Null/Bool/Number/String/Array/Object) — the recipe's most "pure" enum-only case (zero tri-state per this sweep), structurally closest to svg's `SvgNodeDiff` in spirit but likely simpler (no separate attrs-triple concept). |
| 31 | 🖊️dxf | r12 | 1 | 0 | HAND-ROLL (3a only) | `DxfEntityDiff`-shaped (LINE/CIRCLE/ARC/POLYLINE/TEXT/SOLID/INSERT typed entity list) — enum-only like json, zero tri-state here. |

**Summary counts** (31 official standards): 1 piloted-derive (binary) + ~14 probable-derive (pending
per-artifact confirm, several flagged CHECK-ENUM-ELSEWHERE) + 2 piloted-hand-roll (gif89a, svg) + 7
tri-state-only hand-rolls (gif87a, zip, bcf, png, deflate, obj, gltf) + 7 enum-bearing hand-rolls
(pptx, pdf1.7, docx, md, xml, jpg, json, dxf — that's 8, recount: pptx/pdf1.7/docx/md/xml/jpg = 6
enum+tristate, json/dxf = 2 enum-only, total enum-bearing = 8). **Net: roughly 15 derive-eligible, 16
hand-roll-required** (once binary/gif89a/svg are subtracted as already-done) — hand-rolling is NOT the
rare exception the mission's framing implied; it's close to half the program by standard count, and
likely more than half by total effort (gltf's 42 tri-states and md's 3 enums are each substantial).

## 9. Copy-pasteable brief template for a future F6 fan-out agent

```
You are implementing OpText + OpBinary (for <ARTIFACT>Mutation) and DiffCodec (for <ARTIFACT>Diff)
for the <ARTIFACT> <STANDARD> stdio artifact. Read ticket folder file `f6-recon-report.md` FIRST — it
has the derive machinery's real capabilities/limits (§1-3), two full worked examples with real code
(§4-5), the schema-id convention (§6, you need do NOTHING there — it's already wired), and this
artifact's own row in the §8 classification table (verify it, don't just trust it — the sweep was a
grep heuristic, not exhaustive).

STEP 1 — classify for real (don't trust the table blindly):
  a. Diff side: add `dsl::DslDiff` to <ARTIFACT>Diff's derive list, `cargo check -p
     semio-s-plugin-stdio --lib`. If it compiles: you're on the DERIVE path for the diff — but you
     still need `#[derive(dsl::DslRecord)]` added to EVERY nested struct type the diff tree touches
     (compiler errors will tell you exactly which ones, one at a time). If it fails with `DslField is
     not implemented for <SomeEnum>` or `Option<...>: DslField is not satisfied` — you're on the
     HAND-ROLL path (§5's template), REMOVE the derive attempt and cite the real compiler error in a
     doc comment (see the F6-PILOT doc comments on GifFrameDiff/SvgDiff for the exact citation style).
  b. Mutation side: SEPARATELY add `dsl::DslOps` to <ARTIFACT>Mutation's derive list, `cargo check`
     again. Same triage. The two sides CAN and DO differ (gif 89a: diff hand-rolled, mutation
     derived) — do not assume one verdict from the other.

STEP 2a (DERIVE path, either or both sides): add `#[derive(dsl::DslRecord)]` to every nested struct
  (cascading — keep re-running `cargo check` until clean). Add `#[dsl(base64)]` to bare `Vec<u8>`
  fields (NOT `Option<Vec<u8>>` — doesn't work, see §3's derive-quirk note). Add `#[dsl(block)]` to
  struct-valued fields for readability (optional, matches `SpaceMutation`/gif89a precedent). For the
  Mutation side specifically, ALSO write the §2 handcrafted OpText/OpBinary wrapper — `DslOps` NEVER
  emits it (P6), even on full success.

STEP 2b (HAND-ROLL path, either or both sides): copy §5's primitive set (`hex_encode`/`hex_decode`/
  `split_top_level`/`strip_brackets`/`encode_option`/`decode_option`) into your artifact's
  🔺️diff/component.rs (mark the ones your Mutation side will also need as `pub(crate)`, reuse from
  🧬️mutations/component.rs — see how svg's mutations file imports from its own diff file). Write
  `enc_<T>`/`dec_<T>` pairs bottom-up for every value type your Diff/Mutation touches, following §5's
  conventions: hex for strings/bytes, `[f1,f2,...]` positional tuples for structs, single-letter tag
  prefix for enums, `name{[removed];[modified];[added]}` for collection triples, space-separated
  `name=value`/`keyword arg=value` for the top-level line. `encode_diff`/`encode_op` = the text bytes
  verbatim (don't invent a denser binary format unless you have spare budget — the trait laws don't
  require it).

STEP 3 — tests (mandatory, both paths): add `op_text_binary_roundtrip_law` (Mutation, every variant,
  incl. tri-state args and any collection-payload variant) and `diff_codec_text_binary_roundtrip_law`
  (Diff, using a real `between()` result exercising every collection triple + every tri-state
  `Some(None)`/`Some(Some(_))` transition — reuse/adapt the existing `field_sweep`/`between_roundtrip_law`
  test's fixtures if present, don't invent new ones from scratch). Both tests assert: `!printed.contains('\n')`,
  `parse(print(x)) == x`, `decode(encode(x)) == x`.

STEP 4 — verify for real: `cargo test -p semio-s-plugin-stdio --lib "artifacts::<artifact>"` — 0
  failures, INCLUDING the 2 new tests. Then `cargo test -p semio-s-plugin-stdio --lib` (whole crate) —
  must stay at 0 failures, count only goes up (never down) from whatever it was when you started.

STEP 5 — report: `.md` file in the ticket folder, cite the real `cargo test` pass counts (not
  estimated), note which path (derive/hand-roll) each side landed on and why (the exact compiler
  error if hand-rolled), any deviations from §5's grammar conventions and why.

Repo rules: no git-mutating commands, no worktrees, quote every emoji path, temp files in the ticket
folder as .txt, never close/reopen the ticket, `POLICY_DIFF_COMPLETENESS_ALLOWLIST`
(📜️script.ts:2304) — do NOT add your artifact to it; the goal is for the live `bun ./📜️script.ts
policy` diff-completeness check to stop flagging your file, not to allowlist around it. Do not touch
📜️script.ts, glue.rs, or any other artifact's files — your ownership is exactly
`🗿️artifacts/<artifact>/**` + your own report.
```

## 10. Files touched (real, live, not reverted)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `dsl::DslRecord` on `BinarySnapshot`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — `dsl::DslRecord` on `ByteSplice`, `dsl::DslDiff` on `BinaryDiff` (fully derived `DiffCodec`), + `diff_codec_text_binary_roundtrip_law` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `dsl::DslOps` on `BinaryMutation`, handcrafted `OpText`/`OpBinary` replacing `serde_json` stubs, + `op_text_binary_roundtrip_law` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `dsl::DslRecord` on `GifRgb`/`GifColorTable`/`GifPlainText`/`GifAppExtension`/`GifFrame`/`GifSnapshot`, `dsl::DslScalar` on `GifDisposal`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — hand-rolled `impl protocol::DiffCodec for GifDiff` (full grammar + helper functions), + `diff_codec_text_binary_roundtrip_law` test. `GifFrameDiff` left un-derived with an explanatory doc comment (3b blocker).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `dsl::DslOps` on `GifMutation` (derived clean), handcrafted `OpText`/`OpBinary` replacing `serde_json` stubs, + `op_text_binary_roundtrip_law` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — hand-rolled `impl protocol::DiffCodec for SvgDiff` (full grammar + helper functions, most made `pub(crate)` for mutations-side reuse), + `handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law` test. `SvgDiff` left un-derived with an explanatory doc comment (3a+3b blocker, both citations captured).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — hand-rolled `OpText`/`OpBinary` for `SvgMutation` (reusing the diff file's `pub(crate)` primitives) replacing `serde_json` stubs, + `op_text_binary_roundtrip_law` test. `SvgMutation` left un-derived with an explanatory doc comment (3a blocker, citation captured).
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-recon-binary-check1.txt`, `f6-recon-binary-test1.txt`, `f6-recon-gif-check1.txt` through `check6.txt`, `f6-recon-gif-test1.txt`/`test2.txt`, `f6-recon-svg-check1.txt`/`check2.txt`, `f6-recon-svg-mutation-check1.txt` through `check4.txt`, `f6-recon-svg-test1.txt`/`test2.txt`, `f6-recon-full-crate-test.txt`, `f6-recon-full-crate-test-final.txt`, `f6-recon-policy-run.txt`.

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework crates
were all read-only for this session. STATUS.md not updated (recon step, not a closer — leave the
ownership-ledger update to F6's actual closer once the fan-out wave runs).

## 11. Verification summary (all real, all this session)

| Check | Result |
|---|---|
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::binary"` | 18/18 passed |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::gif::standards::v89a"` | 27/27 passed |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::svg"` | 60/60 passed |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1019/0** (baseline 1013 + 6 new law tests) |
| `bun ./📜️script.ts policy` — `dsl-migration/diff-completeness`, stdio-scoped | 28 remaining (was 31; binary/gif89a/svg confirmed dropped) |
| `POLICY_DIFF_COMPLETENESS_ALLOWLIST` stdio entries | 0 (unchanged — correct, not a shortcut) |
