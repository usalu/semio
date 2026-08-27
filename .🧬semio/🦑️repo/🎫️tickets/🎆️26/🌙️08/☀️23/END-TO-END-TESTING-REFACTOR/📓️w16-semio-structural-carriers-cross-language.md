# Wave 16 — the four semio STRUCTURAL carriers, converted to cross-language differential oracles

Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Scope: the `🧿️semio` subsets **`✳️brep`, `✳️graph`,
`✳️kit`, `✳️object`**. Successor to `📓️w13-cross-language-recipe.md`, whose recipe this wave follows
and, in three places, extends.

All four `noOracleDecision`s are gone. All four cases are now `@mode-differential` against a second
implementation written in Python from the format's own committed specification documents, and all
four report **`parity=N/N`, exit 0** — 148 scenarios compared across two languages.

**The differential found a real defect on its first run.** `✳️brep`'s `BrepCurve` and `BrepSurface`
serialized their multi-word members as `radius_major`/`controlPoints`-style snake_case, contradicting
the subset's own committed JSON schema, because serde's `rename_all` renames an enum's VARIANTS and
not its struct-variant FIELDS. Four committed specification vectors had never caught it, because all
four use single-word geometry arms. Fixed at the cause (§4); the reference was not touched.

---

## 0. Headline

| case | kinds | scenarios | oracle | parity | real artifact |
|---|---|---|---|---|---|
| `mutate-semio-object` | 9 | 28 | `semio-object-python-independent` | **28/28** | committed `📦️crate` object |
| `mutate-semio-graph` | 11 | 34 | `semio-graph-python-independent` | **34/34** | committed `🕸️wires` graph |
| `mutate-semio-kit` | 15 | 46 | `semio-kit-python-independent` | **46/46** | committed `🪑️furniture` kit |
| `mutate-semio-brep` | 13 | 40 | `semio-brep-python-independent` | **40/40** | committed `🧊️solid` brep |

Scenario count per case is `3 × kinds + 1`: `mutate-<kind>`, `inverse-<kind>` and
`spec-vector-<kind>` for every declared kind, plus `identity-round-trip`.

---

## 1. Which second producer, and why it is genuinely independent

**Route taken: an independent second IMPLEMENTATION in Python, written from the committed
specification, for all four.** Not a third-party library — there is none.

### What was searched for first

`.dsl.semio` and `.pack.semio` are semio-native carriers. No third-party library in any ecosystem
reads or writes the semio envelope (`0x89 'S' 'E' 'M' 0D 0A 1A 0A` + LE-u32 token, or the
`semio <plugin>.<artifact>.<component> v<n>` text preamble), and none knows this repository's own
`SemioBrepMutation` / `SemioGraphMutation` / `SemioKitMutation` / `SemioObjectMutation` vocabularies —
those vocabularies ARE the specification, not a fact an external library could confirm or refute.

The one place a genuine third party could have entered is `✳️brep`'s **payload**: a b-rep topology
graph is the same subject matter OpenCascade / `ruststep` / `ifcopenshell` speak. It does not help
here, for a reason that is structural rather than incidental: the format under test is the semio
CARRIER, and every one of those libraries would have to be handed a STEP/IGES/IFC document this
repository produced by exporting the brep — which routes the comparison through
`🚪️io/📤️export/…/📐️step/🔖️ap214`, our own code, on the way in. The reference would then be judging our
exporter's output, not the brep codec, and any disagreement would be unattributable between the two.
A `three.js`/mesh library is worse still: `✳️brep` carries exact analytic surfaces (NURBS with
weights and knot vectors, torus, cone), and tessellating them to compare is a lossy projection that
would hide exactly the mutations this case exercises (`replace-surface` swapping a cone for a
sphere). Recorded as considered and rejected, not as unavailable.

### What makes the Python genuinely independent

Each `🐍️component.py` was written against the committed documents listed in its own module docstring
and in its oracle `rationale`, and imports nothing from the Rust:

| document | what it supplied |
|---|---|
| `…/✳️<subset>/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` | the DSL record grammar, its `hex` macro, and every tag alphabet (`i\|o\|x` ports, `Z\|B\|I\|F\|S\|Y\|L\|M\|R` values, `h\|c\|s` pins, `L\|C\|E\|N` curves, `P\|C\|O\|S\|T\|N` surfaces) |
| `…/📸️snapshot/💾️binary/📡️component.protocol.semio` | `format u8`, the varint-length-prefixed `schema`, and the prose sentence naming each subset's trailing payload |
| `…/📸️snapshot/🔣️component.json` | the JSON projection's member names (`startVertex`, `innerLoops`, `isVoid`, `radiusMajor`, `halfAngle`, `uCount`/`knotsU`, `childId`, …) |
| `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` | the verb list and each verb's positional argument list |
| the committed per-kind `(before, mutation, after)` vectors | the externally tagged JSON wire form of each verb, and the cascade behaviour of `delete-vertex` / `delete-node` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs` (`🔖️Envelope`/`🔖️Binary`/`🔖️Text`) | the semio envelope — framework CARRIER code, the same exception w13 named and for the same reason: the envelope has no separate prose document |
| `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️component.rs` (`ArtifactRef::to_uri`) | `<artifact_id>!<kind>@<standard>/<subset>`, documented there as "the only dialect-coordinate codec in the repo" — again framework carrier code, not a subject codec |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (`LinkPin`, `BlobRef`) | the three `✳️kit` pin shapes and `BlobRef { hash, size, mediaType }` |

No subset's `📸️snapshot/🦀️component.rs`, `🧬️mutations/🦀️component.rs` or `🔺️diff/🦀️component.rs` body
informed the Python. The Rust files were opened only for the SIGNATURES the Rust subject adapter
calls, exactly as w13 §3 permits.

### The derivation that is not a reading, and how it is pinned

Every one of the four `📡️component.protocol.semio` files stops at the same boundary and says so: it
frames `format u8` and the schema string, then declares the rest an opaque `payload` chain, while
NAMING in prose what is in there (`"10 fixed f64 LE plus three optional child-handle slots"`,
`"per-node … id/kind/label strings plus a fixed 16-byte position, nested ports/properties lists"`,
`"types/designs/objects/models/properties/representations"`, `"varint counts, per-field
length-prefixes, real f64 LE coordinates, a real per-variant tag byte for curve/surface"`).

The Python writes that named layout out in the order the grammar's own `document` production lists
the collections. That is a derivation, not a reading, and it is **pinned by byte-exact re-encoding of
each committed `🎒️example.pack.semio`**:

```
object    DSL 380 B  pack 267 B
graph     DSL 297 B  pack 183 B
kit       DSL 734 B  pack 498 B
brep      DSL 443 B  pack 537 B
```

All eight files are reproduced byte for byte by the Python from the grammar and the protocol alone.
A misreading of any tag ordinal, field order or varint width could not do that. Ordinals confirmed
directly in the committed bytes: `out → 0x01` / `in → 0x00` (graph ports), `int → 0x02` /
`str → 0x04` (graph values), `head → 0x00` (kit pins), `line → 0x00` / `circle → 0x01` /
`nurbs → 0x03` (brep curves), `nurbs → 0x05` (brep surfaces).

**Honest boundary, per subset.** Arms that no committed artifact carries are the natural extension of
the ones that are pinned, not themselves pinned, and no scenario round-trips them through the pack:
graph's `bytes`/`list`/`map`/`ref` binary value arms, and kit's `checkpoint`/`snapshot` binary pin
arms. Said in each adapter's docstring and each oracle `rationale`.

---

## 2. Which real artifact, and the honest limit

Each case reads the richest genuine committed document its subset has, in place through `asset://`,
never written to:

* **`✳️object`** — `…/✳️object/📚️examples/📦️crate/` — a non-identity translation with ALL THREE
  optional child slots occupied by real `brep`, `mesh` and `value` handles. The only committed
  document that exercises the `ArtifactChild` codec three slots at a time.
* **`✳️graph`** — `…/✳️graph/📚️examples/🕸️wires/` — two nodes carrying between them an `out` port, an
  `in` port, an `int` property and a `str` property, an integral position and a fractional negative
  one, joined by one typed labelled edge.
* **`✳️kit`** — `…/✳️kit/📚️examples/🪑️furniture/` — the only committed document that carries all four
  composition shapes at once: repeated child pools (`objects`, `models`), a single child slot
  (`properties`), a link pool (`representations`, with a `LinkPin`) and a nested design of two pieces
  and a connection.
* **`✳️brep`** — `…/✳️any/📚️examples/🧊️solid/` — **`✳️brep` commits no example of its own.** This is
  the only genuine `stdio.semio.brep` document anywhere in the artifact, and it was found under the
  `✳️any` example set: three vertices, three edges carrying a LINE, a CIRCLE and a rational NURBS
  curve, a loop over all three, a face carrying a NURBS SURFACE with its own weights and two knot
  vectors, a shell and a solid. Four of the ten tagged geometry arms come from the artifact itself.
  No fixture was derived, copied or invented; the real one was located.

**The limit, stated rather than papered over.** These are real, committed, domain-authored documents
that between them touch every field their subsets have — but they are hundreds of bytes, not the
multi-megabyte documents THE STANDARD's word "complex" evokes. `asset://` resolves against the
artifact root with an explicit escape guard (w13 trap 2), so no larger `.dsl.semio` from another
plugin is reachable from a `🧿️semio` case, and copying one into `🧫️fixtures` to get around the guard
would be manufacturing the evidence rather than finding it. Producing a genuinely large real
`s.stdio.semio.*` example is separate work with its own provenance question.

What was done instead of pretending: the mutation PARAMETERS were chosen to reach the parts of each
vocabulary the artifact itself does not exercise, so the tested surface is wider than the artifact —
see §3.

---

## 3. Every declared kind, exercised three ways

For each of the 48 kinds across the four subsets:

* `mutate-<kind>` — applied to the real committed artifact, parameters from the feature's own doc
  string, projection = the whole resulting snapshot;
* `inverse-<kind>` — applied, then undone with each side's OWN computed inverse; the restored
  document is asserted equal to the input IN ROLE on both sides, and the projection is
  `{mutated, restored}` so the rows do not all project the same value and the differential cannot go
  vacuous;
* `spec-vector-<kind>` — the committed handcrafted `(before, mutation, after)` triple, applied by
  BOTH implementations and checked against the committed after-snapshot by each of them in role.
  **Added, not substituted** — the evidence the case rested on before is all still there;

plus one `identity-round-trip` per case that re-emits BOTH committed encodings byte for byte and
projects the digests, so the two languages' emitted BYTES are compared, not just their models.

### The `prepare` device — new in this wave

Three `✳️object` and one `✳️kit` verb are `create-<slot>` against a slot the real artifact already
occupies, where the grammar and the committed vectors define the verb only for an EMPTY slot. Rather
than swap in a poorer artifact or invent semantics, each scenario's doc string carries
`{"prepare": [...], "mutation": {...}}`; the `prepare` list is applied to the real document first and
the inverse law is asserted against the PREPARED document. Both implementations read the same list
from the plan, so neither can drift from what the other read. Used four times for that reason and
once more, in `✳️brep`, to widen coverage (below).

### Coverage the artifact alone would not give

* **`✳️graph`** — `create-node` appends a node carrying all THREE port kinds at once (`in`, `out`, and
  the `inOut` neither committed node uses) and a `float` property; `add-node-property` inserts a
  genuinely NESTED `SemioValue` — a `list` holding an `int` and a `map` — ahead of an existing
  property, so a value codec that only handles scalars, or one that appends instead of inserting,
  fails.
* **`✳️kit`** — `bind-representation` binds with a **checkpoint** pin where the committed link is
  pinned to head, so a pin codec that only knows `head` fails.
* **`✳️brep`** — `create-edge` builds an **ELLIPSE**, `create-face` a **TORUS**, and `replace-surface`
  is *prepared* with a **CONE** and then replaces it with a **SPHERE**, covering two arms in one row.
  With the committed vectors' PLANE (`create-face`) and CYLINDER (`replace-surface`), and the
  artifact's own LINE/CIRCLE/NURBS curve and NURBS surface, **all 4 curve arms and all 6 surface arms
  are exercised by both implementations.**
* **`✳️object`** — each `create-<slot>` lands a different artifact id and child id from the one it
  displaced, and each `delete-<slot>` must leave the other two handles whole.

### Ordered equality, not a weakened comparison

Every deletion is aimed at the LAST record of its collection, because the matching create/add/bind
verb appends. Undoing a removal from the MIDDLE of an ordered collection restores the set but not the
order — the old `noOracleDecision`s explicitly reserved the right to compare "order-insensitive over
the id-keyed sets". This wave does not use that escape: the comparison is `ordered-json-v1` with array
order significant, no `ignoreKeys` and no tolerance, and the scenarios are aimed so that ordered
equality genuinely holds. `delete-vertex` targets `v3`, whose two incident edges are the last two in
the edge list, so its inverse still has to undo a real two-edge cascade.

---

## 4. Divergences and attributions

### The one real divergence — and it is OUR defect, fixed at the cause

`mutate-semio-brep` came back **`parity=13/40`, 6 failed** the first time it ran. The full first
reading, verbatim:

```
[test] level=exhaustive cases=1 executed=80 passed=74 failed=6 errored=0 parity=13/40
[test] parity failed: …::mutate-semio-brep::mutate-create-vertex::rust::subject (16 differences)
  … 26 more parity-failed lines, one per non-vector scenario …
```

and the six red-in-role Rust scenarios named the cause exactly:

```
mutate-create-edge      failed | the planned mutation payload must decode: missing field `radius_major`
mutate-create-face      failed | the planned mutation payload must decode: missing field `major_radius`
mutate-replace-surface  failed | the planned mutation payload must decode: missing field `half_angle`
inverse-create-edge / inverse-create-face / inverse-replace-surface — the same three
```

**Diagnosis.** The two implementations agree on the BYTES — `identity-round-trip` projected the same
`dslDigest b067ddddd8382b7efb486e108f60564f`, the same `packDigest b35d729c525918c313f2d0198d2fdd03`
and the same 443/537 lengths from both sides. They disagreed only on the JSON PROJECTION's member
names, and only inside the two tagged geometry unions:

```
only-python : controlPoints  degreeU  degreeV  knotsU  knotsV  uCount  vCount   (+ radiusMajor, radiusMinor, majorRadius, minorRadius, halfAngle)
only-rust   : control_points degree_u degree_v knots_u knots_v u_count v_count  (+ radius_major, radius_minor, major_radius, minor_radius, half_angle)
```

`BrepCurve` and `BrepSurface` carried `#[serde(tag = "kind", rename_all = "camelCase")]`. On an
**enum**, serde's `rename_all` renames the VARIANTS only — renaming struct-variant MEMBERS needs
`rename_all_fields`. Every sibling record in the same subset is a plain struct, where `rename_all`
does reach the fields, which is why `startVertex` / `outerLoop` / `innerLoops` / `isVoid` were right
all along and only the geometry unions were wrong. The subset's own committed schema
`…/✳️brep/🧬️schema/📸️snapshot/🔣️component.json` declares every one of those members camelCase, so
**the Rust was non-conforming to its own specification document** and the Python — written from that
document — was right.

**Why nothing had caught it.** Every committed specification vector for this subset uses a
single-word arm: `line` (origin/direction), `circle` (center/axis/radius), `plane` (origin/normal),
`cylinder` (origin/axis/radius). The spelling is identical in both conventions there, so all 13
`spec-vector-` scenarios passed — they are the 13 in `parity=13/40`. Two in-crate test docstrings had
even written the defect down as though it were the design: *"camelCase VARIANTS but snake_case FIELDS
(`radius_major`, `control_points`), because it declares `rename_all` without `rename_all_fields`"*.
It took a differential that deliberately reached `ellipse`, `torus`, `cone`, `sphere` and `nurbs` to
turn that observation into a failure.

**Fixed at the cause, not worked around.** `rename_all_fields = "camelCase"` added to both enums in
`…/✳️brep/🧬️schema/📸️snapshot/🦀️component.rs`, with a docstring on each saying why it is not
redundant beside `rename_all`; the two in-crate docstrings that recorded the old behaviour as
intended were corrected. **The Python was not touched, the comparison profile was not touched, and no
`ignoreKeys` was added** — the alternative of "teaching the reference our spelling" would have
deleted the finding. Blast radius measured before the edit: `grep` over every committed `*.json` and
`*.ts` in `✏️s` and `🧰️framework` for `radius_major`/`control_points`/`half_angle`/`major_radius`/
`u_count`/`knots_u`/… returns **nothing**, so no committed vector, fixture or twin spelled the old
form; `✳️cad`'s `major_axis_end` is a different enum in a different subset and is untouched.

**After the fix:**

```
$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-brep --implementation rust
[test] level=exhaustive cases=1 executed=80 passed=80 failed=0 errored=0 parity=40/40      # exit 0
```

and the two sides' `identity-round-trip` projections now agree member for member as well as byte for
byte:

```
oracle-python  "surface": {"kind":"nurbs","controlPoints":[…],"weights":[1.0,1.0],"uCount":2,"vCount":1,"degreeU":1,"degreeV":1,"knotsU":[…],"knotsV":[…]}
subject-rust   "surface": {"kind":"nurbs","controlPoints":[…],"weights":[1,1],    "uCount":2,"vCount":1,"degreeU":1,"degreeV":1,"knotsU":[…],"knotsV":[…]}
dslDigest b067ddddd8382b7efb486e108f60564f  packDigest b35d729c525918c313f2d0198d2fdd03  (both sides, 443/537 bytes)
```

### Two unspecified behaviours, recorded rather than papered over

1. **`✳️brep` `delete-vertex` cascade vs. loops.** The committed vector runs against a brep with NO
   loops, so neither the grammar nor any vector says whether the cascade also purges loop entries
   naming a severed edge. The real `🧊️solid` DOES carry such a loop. Both implementations leave the
   loop entry standing — consistent with the committed `delete-edge` vector, whose after-snapshot
   keeps `l1` naming the removed `e4` — and they agree, but that agreement is evidence about the two
   implementations, not about a specification that is silent. Stated in the feature description.
2. **`✳️kit` `remove-type` vs. a referenced type.** The committed vector removes a type no piece
   references. The committed kit's only type IS referenced by both pieces of its design, and nothing
   says what removing a referenced type does to them. `remove-type` is therefore aimed at a type
   appended by its own `prepare` list — the vector's exact situation. Stated in the feature
   description.
3. **`✳️object` `create-<slot>` over an OCCUPIED slot.** The grammar and all three committed
   `create-<slot>` vectors define the verb only for an EMPTY slot, and the Python implements exactly
   that: it refuses. The Rust carries an in-crate test named
   `create_brep_overwriting_an_existing_handle_restores_the_prior_one_on_undo`, so it deliberately
   OVERWRITES. No scenario reaches that case — the `prepare` list empties the slot first, which is
   why `parity=28/28` — so this is **not a measured divergence**; it is an unspecified extension of
   our codec beyond its own written vocabulary, recorded here rather than left implicit. Teaching the
   reference to overwrite would have meant deriving its behaviour from a Rust test name instead of
   from the specification, which is exactly what makes a reference worthless.

### Everything else agreed

Across the other three cases — 108 compared scenarios — there were **zero divergences**; each
`parity=N/N` in §0 is a full match under `ordered-json-v1`.

Two places where the two implementations agreed in territory the SPECIFICATION does not cover are
recorded as findings about the specification, not about either codec:

1. **`✳️brep` `delete-vertex` cascade vs. loops.** The committed vector runs against a brep with NO
   loops, so neither the grammar nor any vector says whether the cascade also purges loop entries
   naming a severed edge. The real `🧊️solid` DOES carry such a loop. Both implementations leave the
   loop entry standing (consistent with the committed `delete-edge` vector, whose after-snapshot keeps
   `l1` naming the removed `e4`), and they agree — but that agreement is evidence about the two
   implementations, not about the specification, which is silent. Stated in the feature description.
2. **`✳️kit` `remove-type` vs. a referenced type.** The committed vector removes a type no piece
   references. The committed kit's only type IS referenced by both pieces of its design, and nothing
   specifies what removing a referenced type does to them. `remove-type` is therefore aimed at a type
   appended by its own `prepare` list — the vector's exact situation. Exercising the referenced case
   would measure a gap in the specification rather than the two implementations. Stated in the feature
   description.

**Nothing was weakened anywhere.** No `ignoreKeys`, no tolerance, no comparison-profile change, no
fixture removed, swapped or added, no `deferredKinds`, no assertion relaxed. All four cases stayed on
`ordered-json-v1`.

---

## 4b. In-crate unit tests — 4 red, attributed to a live peer refactor

`cargo test --lib brep` in `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust` reports
`441 passed; 4 failed` at the time of writing:

```
brep …::fixture_tests::…delete_vertex…::the_undo_recreates_the_vertex_then_both_severed_edges
     assertion failed: one create-vertex plus one create-edge per severed edge   left: 9  right: 3
brep …::tests::delete_vertex_cascades_to_dependent_edges_and_inverse_restores_both
     assertion failed: inverse must reconstruct the vertex AND the one cascade-deleted edge  left: 4  right: 2
object …::tests::create_brep_overwriting_an_existing_handle_restores_the_prior_one_on_undo
     assertion failed  left: "brep-01"  right: "brand-new"
object …::tests::create_delete_brep_round_trips
     assertion failed  left: "brep-01"  right: "brep-99"
```

**None of them is this wave's.** All four assert INVERSE-STEP COUNTS and CHILD-SLOT OVERWRITE
semantics, both owned by `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`'s
`apply_collection_mutation`/`inverse_collection_mutation` — a file another session was editing
throughout this work (mtime 19:24, then 19:26 while this was being written) and which, minutes later,
**did not compile at all**:

```
error: could not compile `semio-framework-os-kernel` (lib) due to 10 previous errors
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8359:13
```

The only production edit this wave made is `rename_all_fields` on two serde attributes, which cannot
change how many steps an inverse returns or whether a child slot overwrites; and the two `✳️object`
failures are in a module this wave never opened. The attribution was checked, not assumed: the serde
attribute was temporarily reverted to re-run the failing brep test in isolation, and that run could
not even build, because the peer's os-kernel was mid-edit — which is itself the evidence. The
attribute was restored immediately.

---

## 5. Framework gap this wave reproduces (pre-existing, w13 trap 1)

`parity` runs the ORACLE adapter in the SUBJECT role too, because the subject phase iterates every
adapter file the case has. An oracle-only Python adapter answers `adapter has no subject registration
for scenario …`. Reproduced exactly, verbatim, on `mutate-semio-object`:

```
$ bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-object      # exit 1
[test] level=exhaustive cases=1 executed=56 passed=28 failed=0 errored=28 parity=0/0
```

The 28 `passed` are the Rust subject — every scenario of the case, green. The 28 `errored` are the
Python adapter run as a subject; read straight out of
`…/⚡️cache/tests/results/…-mutate-semio-object-subject-python/📤️results.jsonl`:

```json
{"scenario": "mutate-move-object", "status": "errored",
 "diagnostics": [{"message": "adapter has no subject registration for scenario mutate-move-object"}]}
```

This is **pre-existing** — w13 recorded it against `extract-text-pdf-1-4` and against its own
converted case — and the workaround is `--implementation rust`, which narrows the SUBJECT phase only
while the oracle phase still dispatches to Python. Registering the Python handlers as subjects would
"fix" it by manufacturing a self-comparison and must not be done. **No framework fix was made here;
it is not a case's business.**

---

## 6. Verification — real output

Every command run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`; exit codes are the tool's
own, never read through a pipe.

### Before

All four features carried `@no-oracle-semio-<subset>-mutation-semantics` and no `@oracle-` tag, so
the runner dispatched NOTHING for them in the oracle phase — they were four of the 85
`not-exercised` cases `📓️w12-final-audit.md` §2 counted. The four `noOracleDecisions` entries are
recoverable from git (`git show HEAD:…/✳️<subset>/🧪️oracle/🔣️.json`); each claimed its
evidence was "discharged by the subject phase". They are gone.

### `contract` — exit 1, and zero breaches name these four cases

```
$ bun ./📜️script.ts contract --owner 🗄️stdio                                          # exit 1
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery

  testing/discovery  🧰️framework  42 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s  4 executable test file(s) outside the canonical owner-root test tree, baseline allows 1
```

`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` read directly: **2 records, `unmanaged-tests` ×2,
neither of which names `mutate-semio-brep`, `mutate-semio-graph`, `mutate-semio-kit` or
`mutate-semio-object`.** Both are the pre-existing `🎞️animate`/`🎬️sequence` `.test.ts`/`.test.js`
counts w13 already recorded. `testing/contract`, `testing/oracle`, `testing/fixture` and
`testing/taxonomy` are all at zero. (An earlier reading of the same command also carried a
`testing/oracle  …/mutate-semio-model  Unknown oracle id @oracle-semio-model-python-independent`
record — a concurrent session's case mid-flight, which that session has since completed.)

### `oracle exhaustive` — all four green, exit 0 each

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-object      # exit 0
[test] level=exhaustive cases=1 executed=28 passed=28 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-graph       # exit 0
[test] level=exhaustive cases=1 executed=34 passed=34 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-kit         # exit 0
[test] level=exhaustive cases=1 executed=46 passed=46 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-brep        # exit 0
[test] level=exhaustive cases=1 executed=40 passed=40 failed=0 errored=0 parity=0/0
```

### `parity exhaustive` — the number that matters

```
$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-object --implementation rust
[test] level=exhaustive cases=1 executed=56 passed=56 failed=0 errored=0 parity=28/28      # exit 0

$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-graph  --implementation rust
[test] level=exhaustive cases=1 executed=68 passed=68 failed=0 errored=0 parity=34/34      # exit 0

$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-kit    --implementation rust
[test] level=exhaustive cases=1 executed=92 passed=92 failed=0 errored=0 parity=46/46      # exit 0

$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-brep   --implementation rust
[test] level=exhaustive cases=1 executed=80 passed=80 failed=0 errored=0 parity=40/40      # exit 0
```

### `dependency` — unchanged, exit 0

```
$ bun ./📜️script.ts dependency                                                        # exit 0
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
```

Identical to w13's reading. `package: ""` in all four oracle entries keeps them out of
`🔒️dependencies.json` and out of the import-purity probe, exactly as w13 §2 documents.

### The identity projections, across languages

Read directly out of `…/⚡️cache/tests/results/…-{oracle-python,subject-rust}/📤️results.jsonl` for
`mutate-semio-object`:

```
oracle-python  {"dslDigest":"cb2647575ef7dd228053db1f1577eb14","packDigest":"bcd783e6fe73c9be36b29a0187d068e4","dslLength":380,"packLength":267}
subject-rust   {"dslDigest":"cb2647575ef7dd228053db1f1577eb14","packDigest":"bcd783e6fe73c9be36b29a0187d068e4","dslLength":380,"packLength":267}
```

Two implementations, two languages, from one written specification, the same bytes.

### Negative control

`apply_mutation` in `mutate-semio-object/🐍️component.py` was temporarily changed so that every
`Delete<slot>` verb also cleared the `mesh` handle — precisely the slot confusion the feature claims
to catch — and the oracle phase run again:

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-object      # exit 1
[test] level=exhaustive cases=1 executed=28 passed=24 failed=4 errored=0 parity=0/0
```

Read straight out of the result stream:

```
inverse-delete-brep         failed | undoing inverse-delete-brep did not restore the crate object
inverse-delete-properties   failed | undoing inverse-delete-properties did not restore the crate object
spec-vector-delete-brep     failed | the applied snapshot does not match the committed after-snapshot
spec-vector-delete-properties failed | the applied snapshot does not match the committed after-snapshot
```

Four scenarios red **in the oracle's own role** — the inverse law and the committed after-snapshot
both caught it without the subject being consulted at all. The `mutate-delete-*` rows carry no
in-role assertion by design and are the ones parity exists to catch. The edit was reverted and the
same command returns `executed=28 passed=28 failed=0 errored=0`, exit 0.

---

## 7. Files this wave touched

Nothing outside these nineteen files — sixteen case/manifest files plus the three that carry the
codec fix §4 found. No framework file, no shared manifest, no `Cargo.toml`, no
`📦️lib.rs`, no `🔒️dependencies.json`, no fixture, no comparison profile, no `ignoreKeys`, no
`project.json`, no `launch.json`, no example artifact.

| file | change |
|---|---|
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-object/🐍️component.py` | **new** — independent implementation + oracle adapter |
| `…/mutate-semio-object/component.feature` | rewritten — `@oracle-…`, 28 scenarios |
| `…/mutate-semio-object/🦀️component.rs` | rewritten — subject only, reads its inputs from the plan |
| `…/✳️object/🧪️oracle/🔣️.json` | `noOracleDecisions` removed, `oracles[]` gains the entry |
| `…/mutate-semio-graph/🐍️component.py` | **new** |
| `…/mutate-semio-graph/component.feature` | rewritten — 34 scenarios |
| `…/mutate-semio-graph/🦀️component.rs` | rewritten — subject only |
| `…/✳️graph/🧪️oracle/🔣️.json` | decision removed, oracle registered |
| `…/mutate-semio-kit/🐍️component.py` | **new** |
| `…/mutate-semio-kit/component.feature` | rewritten — 46 scenarios |
| `…/mutate-semio-kit/🦀️component.rs` | rewritten — subject only |
| `…/✳️kit/🧪️oracle/🔣️.json` | decision removed, oracle registered |
| `…/mutate-semio-brep/🐍️component.py` | **new** |
| `…/mutate-semio-brep/component.feature` | rewritten — 40 scenarios |
| `…/mutate-semio-brep/🦀️component.rs` | rewritten — subject only |
| `…/✳️brep/🧪️oracle/🔣️.json` | decision removed, oracle registered |
| `…/✳️brep/🧬️schema/📸️snapshot/🦀️component.rs` | **codec fix** — `rename_all_fields = "camelCase"` on `BrepCurve` and `BrepSurface`, with a docstring on each saying why it is not redundant beside `rename_all` |
| `…/✳️brep/🧬️schema/🧬️mutations/➰replace-curve/🧪️tests/…/🦀️component.rs` | docstring corrected — it recorded the snake_case fields as intended |
| `…/✳️brep/🧬️schema/🧬️mutations/🗺️replace-surface/🧪️tests/…/🦀️component.rs` | docstring corrected — same |

Scratch (this ticket folder only): `w16-structural-carriers/🐍️smoke.py` — a host-free driver that
loads one case's `🐍️component.py` behind a stub `semio_repo_test` module — plus one
`mutate-semio-<subset>.json` parameter file per case and the verbatim run logs.
