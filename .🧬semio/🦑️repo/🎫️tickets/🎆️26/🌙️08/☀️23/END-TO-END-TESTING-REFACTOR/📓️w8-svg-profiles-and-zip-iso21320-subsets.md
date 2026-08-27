# Wave 8 — the three profile subsets that had no handcrafted mutations

Scope: `🎨️svg` 1.1 `✳️basic` and `✳️tiny`, and `🎒️zip` 2.0 `✳️iso21320`. All three previously had no
`🧬️schema/🧬️mutations` of their own: each subset's schema file opened with
`pub use …subsets::any::schema::*;`, its `ArtifactBuilder` declared `type Mutation = SvgMutation` /
`ZipMutation`, and `mutate()` called the parent subset's `apply_*_mutation`. Nothing about the
profile appeared in the vocabulary at all.

## Verified

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-svg-1-1-tiny        # exit 0
0 high-priority breach(es) across 0 rule(s)
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-svg-1-1-tiny
[test] level=exhaustive cases=1 executed=21 passed=21 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-svg-1-1-basic       # exit 0
0 high-priority breach(es) across 0 rule(s)
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-svg-1-1-basic
[test] level=exhaustive cases=1 executed=23 passed=23 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-zip-2-0-iso21320    # exit 0
0 high-priority breach(es) across 0 rule(s)
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-zip-2-0-iso21320
[test] level=exhaustive cases=1 executed=17 passed=17 failed=0 errored=0 parity=0/0
```

Repository-wide, nothing regressed:

```
$ bun ./📜️script.ts contract                                    # exit 0
0 high-priority breach(es) across 0 rule(s)
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio           # exit 0
[test] level=exhaustive cases=80 executed=1011 passed=1011 failed=0 errored=0 parity=0/0 not-exercised=20
$ cargo test --features oracles --lib                           # oracle crate
test result: ok. 208 passed; 0 failed; 1 ignored
```

## What genuinely distinguishes each subset, and what the vocabulary says about it

### `🎨️svg` 1.1 `✳️tiny` — 10 kinds

`no-mutation, set-snapshot, stamp-base-profile, insert-tiny-element, remove-element,
set-tiny-attribute, set-text, set-view-box, set-transform, strip-non-tiny`

SVG Tiny 1.1 excludes 16 elements plus every `fe*` filter primitive, and forbids 7 presentation
attributes everywhere. `✳️any`'s `insert-element`, `set-attribute` and `set-element-name` can each
leave the profile in a single step; `insert-tiny-element` and `set-tiny-attribute` reject an
excluded subtree or a forbidden attribute with a real diagnostic instead of writing it.
`stamp-base-profile` and `strip-non-tiny` have no counterpart in Full 1.1 at all. `✳️any`'s
`set-declaration`/`set-doctype` are deliberately absent — neither is a profile operation.

Fixture: the real committed `qr-code.svg`, which carries **335 real `style` attributes** and is
therefore a genuine Full-1.1 document that the profile rejects. `strip-non-tiny` has 335 real
attributes to remove.

### `🎨️svg` 1.1 `✳️basic` — 11 kinds

`no-mutation, set-snapshot, stamp-base-profile, insert-basic-element, remove-element,
set-basic-attribute, set-clip-path-reference, insert-clip-path-shape, set-text, set-view-box,
set-transform`

Basic is not Tiny with a longer allow-list. It KEEPS gradients, patterns, masks, opacity, `clipPath`
and the filter mechanism; it excludes nine expensive raster filter primitives and clipping to text.
So its profile-defining kinds are about filters and clip paths: `insert-basic-element` accepts a
`<filter>` carrying `feGaussianBlur` and refuses the same insert with `feTurbulence`, and the pair
`set-clip-path-reference`/`insert-clip-path-shape` address clip paths by `id` — the way a
`clip-path="url(#id)"` names them — and refuse anything that would clip to text. None of the three
exists in `✳️tiny`, whose profile has neither filters nor `clipPath`.

Fixture: `mouse.svg`, copied verbatim from
`🧰️framework/🔨️modules/🖼️assets/👋️introduction/🔣️mouse.svg` — a real production asset the framework's
own onboarding UI renders. It is only 1,463 bytes, and it was chosen for what it contains rather
than for its size: it is the **only real, committed SVG in this repository that declares a
`clipPath` at all**, and both clip-path kinds would have been vacuous against a synthetic one.

**Where the two vocabularies agree, and why that is not a copy.** Both declare a gated insert, a
gated attribute set, the profile stamp, and the geometry/text kinds. That is a real fact about the
two profiles — they are two restrictions of ONE schema, so they differ in what each GATE rejects and
in the kinds each adds, not in how a document is addressed. Where they genuinely differ they do:
Tiny declares `strip-non-tiny` and has no clip-path kinds; Basic declares two clip-path kinds and no
strip.

**Basic has no `strip-non-basic`, deliberately.** This repository holds no real SVG carrying an
excluded raster primitive, so such a kind would have been a no-op on every real fixture available. A
vacuous scenario is worse than an absent one; the absence is recorded here rather than papered over
with a synthetic input.

### `🎒️zip` 2.0 `✳️iso21320` — 8 kinds

`no-mutation, set-snapshot, set-archive-comment, add-stored-entry, add-deflated-entry, remove-entry,
rename-entry, set-entry-data`

ISO/IEC 21320-1:2015 §4.4 admits exactly two compression methods, Stored (0) and Deflate (8), out of
the twenty-odd APPNOTE defines; §4.1 forbids encryption. `✳️any`'s `add-entry` declares no method at
all. This subset splits it into `add-stored-entry`/`add-deflated-entry`, and the new
`ZipIso21320Method` type makes every non-admitted method unrepresentable — the restriction is the
profile, expressed in the type system.

Fixture: the real `🎒️zwischenbericht-projekte.zip` (20 real architecture photographs, ~1.53 MB). It
was checked, not assumed, to be a genuine ISO/IEC 21320-1 container: all 20 members Deflate, none
encrypted, one real 88-byte EOCD comment.

## Findings

**1. The ISO 21320-1 subset's own constraints are wire properties its snapshot cannot represent.**
`ZipSnapshot` models a member as `{name, data}` and nothing else — no compression method, no
general-purpose flag bits, no version-needed field. Every constraint
`check_iso21320_conformance` actually checks (encryption bit, Strong Encryption bit, trailing data
descriptor, version-needed ceiling) is therefore invisible to any snapshot mutation. The declared
method is authoritative for a writer that can honour it — the registered `zip` reference
implementation can, `ZipWriter::start_file` takes it explicitly — and advisory for this repository's
`encode_zip`, whose `canonical_compression_method` derives the method from the member's filename
extension (`.png/.jpg/.jpeg` → Stored, everything else → Deflate). Closing the gap means giving
`ZipEntry` a native-header facet; that is a schema change outside this vocabulary's scope and is
recorded in the mutations module's own header.

**2. `with_stored_entry` and `with_deflate_entry` were byte-identical no-op duplicates.** The
ISO 21320-1 builder already declared the two-method distinction in its public API, and both
functions called `✳️any`'s ungated `AddEntry` with exactly the same arguments. They now call
`AddStoredEntry`/`AddDeflatedEntry` respectively.

**3. `normalize_entry_for_iso21320` is still a declared no-op** (`fn normalize_entry_for_iso21320(entry: &mut ZipEntry) { let _ = entry; }`
in `✳️iso21320/🚪️io/🦀️component.rs`). Not fixed here — for the same reason as finding 1, there is
nothing in the snapshot for it to normalize. Left as it is, and named.

**4. Comparing the compression method would have compared our policy with itself.** The obvious
projection field for an ISO 21320-1 profile is the per-member compression method. It is the wrong
one: §4.4 admits BOTH methods, so which one a writer picks is writer freedom, and an oracle taught
to reproduce `canonical_compression_method` would be comparing this repository's policy against a
copy of that policy. `semantic-zip-iso21320-v1` therefore reports the ISO PREDICATES —
`isoCompressionAllowed` (method ∈ {Stored, Deflate}) and `encrypted` — which is what the standard
actually fixes, and keeps `compressionMethod` in `ignoreKeys` as the base archive profile does.

## Deliberate design decisions worth reviewing

**The three subsets' builders now take their own mutation type.** `SvgTinyBuilderConstruction::Mutation`
is `SvgTinyMutation`, not `SvgMutation`; likewise Basic and ISO 21320-1. `DerivedArtifactSpec` and
`ArtifactBuilder` bound the associated type only by `protocol::Mutation<Snapshot, Diff = Diff>`, and
`grep` confirms the three builders are referenced nowhere outside their own subsets' tests, so the
change is contained. Without it the vocabulary would exist but nothing would reach it.

**The mutations module is mounted from the subset's own schema file, not from `📦️glue.rs`.**
`#[path = "🧬️mutations/🦀️component.rs"] pub mod mutations;` sits at the top of each subset's
`🧬️schema/🦀️component.rs`, so the vocabulary is wired by the subset that owns it and the shared,
contended glue file is untouched. Two mechanisms this relies on were verified in a standalone rustc
build (`scratchpad/pathprobe`), not assumed:
- a `#[path]` on a module declared at a file's top level resolves relative to THAT file's directory
  (the `✳️image` precedent needed `#[path = "."]` only because its declarations sit inside an inline
  `mod` block);
- an explicit `pub mod mutations;` shadows the `mutations` brought in by the file's own
  `pub use …any::schema::*;`, while everything else the glob supplies still resolves.

**A new shared oracle family module, `🧪️oracle/📰markup/🦀️component.rs`.** Tiny and Basic are two
restrictions of one schema, so their oracles genuinely share every parse, write, address and
projection step — the family-module rule, not a copy in each subset. It holds the quick-xml element
tree, the SVG `viewBox`/`transform` grammars, the JSON spec codec and the semantic projection. The
`✳️any` SVG oracle keeps its own private copy of the same machinery; refactoring it onto the shared
module belongs to its owner, not here.

**`inverse_svg_tiny_mutation` / `inverse_svg_basic_mutation` / `inverse_zip_iso21320_mutation`.**
Thin free functions over `Mutation::inverse`, added for the same reason `kit` added its four
wrappers in wave 7: a test adapter compiled as an external crate cannot name the `protocol::Mutation`
trait. With them the case's subject exercises the implementation's OWN inverse algebra instead of a
transcription of it in the adapter — which is what the wave-7 SVG case had to settle for.

**The inverse law is checked on the oracle side, not deferred to parity.** The oracle phase records
outcomes; a scenario passes unless its handler returns `Err`. So an `inverse-<kind>` scenario would
otherwise prove only that the inverse ran. All three adapters' `inverse_oracle` handlers now compare
the restored artifact's independent projection against the REAL original's and fail the scenario on
a mismatch. Negative control, run to prove the check bites rather than assuming it:

```
# condition inverted from != to == in mutate-svg-1-1-tiny's inverse_oracle
[test] level=exhaustive cases=1 executed=21 passed=11 failed=10 errored=0 parity=0/0
# restored
[test] level=exhaustive cases=1 executed=21 passed=21 failed=0 errored=0 parity=0/0
```

Exactly the 10 inverse scenarios flip, which is what the check covers.

**A real property of a profile-closed vocabulary, stated rather than hidden.** `remove-element`'s
inverse is a GATED insert, so in a Tiny document you cannot undo the removal of a node the profile
itself would refuse. The Tiny case therefore removes `<defs id="defs663"/>` (profile-clean) rather
than a `style`-carrying `<g>`. The feature file says so in the description instead of quietly
choosing a convenient target.

## Honest limits

**The production Rust could not be compiled.** `cargo check -p semio-s-plugin-stdio` never reaches
the stdio crate: `semio-framework-job` fails first with 5 errors (four `ManuallyDrop<Option<…>>` vs
`Option<…>` mismatches and one double mutable borrow in
`🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️component.rs`), the `RetainedJobPayload`
refactor the brief names. So the three mutation modules, the three schema-file edits and the three
adapters' `#[cfg(feature = "sut")]` subject halves are **unverified by the compiler**. What was
verified instead:
- every new and changed Rust file parses, including through its `#[path]` submodule, checked with
  `rustfmt --emit stdout` against the real tree (read-only, nothing reformatted);
- the two module-system mechanisms the mounting relies on, in a standalone rustc build;
- the whole oracle half, which compiles and runs — 208 oracle-crate unit tests pass, including 14
  new ones covering the three profile gates and the ISO inverse algebra.

The `KINDS`-versus-enum conformance tests (`kinds_matches_enum_variants_and_manifest`, one per
subset, plain `#[test]`) live in the blocked production crate and have therefore **not run**. Their
manifest half was checked by hand: all three catalogs match their `KINDS` const exactly, in
declaration order.

**`parity=0/0` is not a pass.** No subject ran for any of the three cases. The differential claim in
`@mode-differential` is not yet discharged; what IS discharged today is that the registered reference
implementation performs every declared kind on a real artifact, that every kind's inverse restores
that artifact's independent projection exactly, and that a full decode/re-encode is possible without
passing bytes through.

## Files

New:
- `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📰markup/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧪️oracle/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧪️oracle/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧪️oracle/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧪️tests/mutate-svg-1-1-tiny/{component.feature,🦀️component.rs}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧪️tests/mutate-svg-1-1-basic/{component.feature,🦀️component.rs}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧪️tests/mutate-zip-2-0-iso21320/{component.feature,🦀️component.rs}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧫️fixtures/mouse.svg`

Changed:
- `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/📦️lib.rs` — additive: the `📰markup` family
  module and three subset oracle modules mounted. No existing line altered.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧬️schema/🦀️component.rs`

`📦️glue.rs`, `Cargo.toml`, the framework, the taxonomy, the shared stdio oracle manifest, `.gitignore`,
`project.json` and `launch.json` are untouched. `quick-xml` 0.42 and `zip` 6 were already linked in
the oracle crate; no dependency was added.
