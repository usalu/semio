# Wave 15 — the five red scenarios, written up as specification defects

> These are findings for the repository owner, not bugs to be tuned away. Every one of them was
> produced by a SECOND IMPLEMENTATION refusing to guess. None of them is closable by widening a
> comparison profile, relaxing a tolerance, swapping a fixture or deleting a scenario, and none of
> them was.

Date 2026-08-26. HEAD `8d9b51f081f42b36722b54f80a5c502d6322f9ca` (2026-08-25 14:57:24 +0200), dirty
tree. Toolchain `rustc 1.99.0-nightly (c4af71034 2026-07-06)`. Raw logs in `w15-work/`.

All five were **re-run today and are still red, each for its stated reason**, verbatim from the
tool's own stdout (exit status read from the tool, never through a pipe):

```
[test] level=exhaustive cases=15 executed=799 passed=795 failed=4 errored=0 parity=0/0    (--owner 📕️norm)
[test] level=exhaustive cases=1  executed=25 passed=24  failed=1 errored=0 parity=0/0    (--case mutate-jack-1)
```

| # | case :: scenario | mode | the second implementation's own words |
|---|---|---|---|
| 1 | `mutate-en1990-1 :: mutate-insert-variable-action` | differential | *"the committed vector declares this mutation applied, yet this implementation refused it: `insert-variable-action` would seed the composed child slot `'qK'`, whose childId is content-addressed by a function no specification in this repository states"* |
| 2 | `mutate-en1990-1 :: inverse-insert-variable-action` | differential | *"the forward mutation could not be applied to its own committed before-snapshot: …"* (same cause, reached from the committed before-snapshot) |
| 3 | `mutate-iso16757-1 :: identity-round-trip` | round-trip | *"this artifact's carrier cannot be read by a second implementation. `'}'` is not a `key=value` field: the notation nests records and tables and flattens nested records into `key=key=value` runs with no delimiter, and this repository publishes no grammar for it"* |
| 4 | `mutate-vdi3805-1 :: identity-round-trip` | round-trip | same cause |
| 5 | `mutate-jack-1 :: spec-vector-create-node` | conformance | *"the committed vector declares a refusal, but the mutation applied"* |

**They are not five findings. They are two defects.** Numbers 1, 2 and 5 are the same defect seen
from two sides; 3 and 4 are the second defect. The audit that surfaced them read them as three; the
evidence below says two, and says the first one is considerably worse than it looked.

---

## Defect A — a composed child slot's `childId` is content-addressed by `std`'s deliberately unspecified default hasher

**Scenarios:** `mutate-en1990-1 :: mutate-insert-variable-action`,
`mutate-en1990-1 :: inverse-insert-variable-action`, `mutate-jack-1 :: spec-vector-create-node`.

### The clause

`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🦀️component.rs:87-93` —

```rust
fn en1990_qk_scene_id(entries: &[En1990QkEntry]) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = serde_json::to_string(entries).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("en1990-qk-{:016x}", hasher.finish())
}
```

`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs:271` is the same call in the same role,
and the jack vector's own header says so outright
(`…/🧬️mutations/🌱️create-node/🧪️tests/rejects-a-node-id-the-scene-already-holds/🦀️component.rs:8-12`):

> *"every one of this vocabulary's eight diff builders funnels its changed scene through
> `diff_replace_content`, which mints a fresh handle whose `child_id` is a
> `std::collections::hash_map::DefaultHasher` digest of the child content. Hand-authoring such an
> `➡️after` would mean hand-forging a value from `std`'s deliberately unspecified default hasher."*

The framework type that carries the value specifies nothing about how it is minted.
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:2380-2400` documents `ArtifactChild`
as an *"Ownership handle a parent SNAPSHOT embeds for one owned child slot"* and says only what
`child_id`'s type is. `📓️derivation-rules.md`'s eight-rule recipe has no rule for a composed child
slot at all: rule 3 covers *"index-keyed ordered collection (no stable id)"*, which is exactly what
`q_k` is in the domain, but the artifact implements it as a child handle, and no rule says how a
mutation on such a slot addresses it or what its identity is a function of.

### The evidence

The committed vector for `insert-variable-action`
(`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴insert-variable-action/🧪️tests/seeds-the-first-variable-action-q-snow-at-12-5-kn/`)
is these four files. The mutation is

```json
{ "mutation": "insertVariableAction", "index": 0, "category": "Q_snow", "value": 12.5 }
```

and the declared outcome is `{"status": "applied"}`. The **entire** difference between the committed
before- and after-snapshots is one field:

```
 "qK": {
-  "childId": "en1990-qk-7904dd65836c8ff4",
+  "childId": "en1990-qk-69c0017661d2372c",
```

Nothing else moves. `Q_snow` and `12.5` never appear in either snapshot — as the fixture's own
docstring puts it, *"the twelve-and-a-half kilonewtons themselves never appear in the diff, only
their address does."* The entry list they belong to is not in the document; it lives in a
process-local `EN1990_QK_SCRATCH: RefCell<HashMap<String, Vec<En1990QkEntry>>>` thread-local
(`…/📘️en1990/🦀️component.rs:84`).

So an implementation that holds the committed before-snapshot and the committed mutation holds
**everything the specification gives it** and still cannot compute the after-snapshot, because the
after-snapshot's only changed field is a hash of a value the specification never handed it. The
Python reference says exactly that and stops. That is correct behaviour, and it is why the scenario
is red.

`mutate-jack-1 :: spec-vector-create-node` is the same defect wearing the opposite result. Its
committed before-snapshot's `content` is the opaque handle
`jack-content-create-node-collision-fixture`; the node `shaft` that the `createNode` payload is
supposed to collide with **is not in any committed file**. It is injected at test time by the Rust
fixture helper —

```rust
cache_jack_content(&snapshot.content.child_id, vec![payload.node.clone()], Vec::new());
```

— so the declared `{"status": "rejected", "code": "mutation.duplicate-id", "path": ["shaft"]}` is
reachable only from inside that Rust process. A second implementation reading the committed quartet
sees an empty scene, and `create-node` legitimately applies. **The vector is not wrong about the
verb; it is not self-contained.** Its declared outcome depends on out-of-band state.

And the reason jack has only this one vector is the first half of the defect: the same header
explains that a *state-changing* branch cannot be pinned at all, because its `➡️after` would have to
carry a hand-forged `DefaultHasher` digest.

### Why this is worse than "unpublished"

`std::collections::hash_map::DefaultHasher` is documented by the standard library as an
implementation detail: *"The internal algorithm is not specified, and so it and its hashes should
not be relied upon over releases."* `DefaultHasher::new()` is deterministic within one toolchain
(it is seeded with zero keys, unlike `RandomState`), which is why the committed hex digits are
stable today under `rustc 1.99.0-nightly`. They are not stable across a Rust upgrade, and they are
not computable in Python, TypeScript, Go or C# at all.

That means the committed after-snapshot is not merely un-reproducible by a second implementation —
**it is a fixture pinned to a value the Rust project explicitly refuses to guarantee.** A toolchain
bump can turn `mutate-en1990-1` red without a line of this repository changing.

### Scale

`DefaultHasher::new()` has **58 non-test call sites** across `✏️s` and `🧰️framework`
(`grep -rn "DefaultHasher::new()" --include="🦀️component.rs" ✏️s 🧰️framework | grep -v 🧪️tests | wc -l`),
spread over ~30 artifacts including `🔌️jack`, `📘️en1990`, `📙️din18599`, `🧊️process3d`, `🧊️3d`,
`🖨️raster`, `🗺️gismap`, `🏔️gisterrain`, `🏛️program`, `🎬️present`, `📐️cad`, `📋️forms`, `🕸️dag`,
`🖐️5d` and `📸️remodel`. Every artifact in that list whose snapshot embeds a composed child has the
same ceiling: **no second implementation can ever reproduce a mutation that touches the child slot**,
so those mutations can never meet the raised bar as things stand.

### What has to be decided to resolve it

Three questions, in order — none of them a test change:

1. **Is a composed child's `childId` content-addressed or allocated?** If content-addressed, the
   addressing function must be published as a specification (a named hash over a named canonical
   encoding of the child), the way `📖️component.grammar.semio` publishes a carrier. If allocated,
   the vectors must stop asserting a particular value.
2. **If content-addressed, by what?** `DefaultHasher` cannot be the answer — it is unspecified by
   its own vendor. A specified digest over a specified canonicalisation (the repository already has
   `digest` = sha256-truncated in the test protocol) would make the value computable by anyone.
3. **Must a specification vector be self-contained?** `mutate-jack-1`'s vector says no today: it
   needs a Rust helper to seed a cache. If the answer is yes, the child's content belongs in the
   committed `⬅️before` (as a fifth file beside the four), and the contract should have a rule that
   says so — there is none today.

Answering (1) and (2) closes scenarios 1 and 2 at the cause. Answering (3) closes scenario 5 and
unblocks the seven jack verbs that currently have no vector at all.

---

## Defect B — the `.dsl.semio` text carrier writes a nesting notation for which this repository publishes no grammar

**Scenarios:** `mutate-iso16757-1 :: identity-round-trip`, `mutate-vdi3805-1 :: identity-round-trip`.

### The clause

`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
in full — the `📔️vdi3805` file is byte-identical:

```
dialect grammar
grammar iso16757.snapshot
extension iso16757
start document

document = header body
header = "schema" SP "stdio.json" NL
body = payload NL?
payload = OCTET+
```

### The evidence

The committed artifact this grammar is supposed to describe
(`…/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`,
4,128 bytes) begins:

```
semio norm.iso16757.dsl v1
exchange-process=determineProduct catalogue=id=cat.demo metadata=edition-profile=fullPublished names=short-name=Demo preferred=locale=en text="Demo HVAC catalogue"
alternatives [locale:TEXT text:TEXT] {
}
lifecycle=revision="1" status=published manufacturer=id=mfg.demo names=preferred=locale=en text="Demo Manufacturer"
alternatives [locale:TEXT text:TEXT] {
  de "Demo Hersteller"
}
…
product-classes [id:TEXT group-id:TEXT parent-id:TEXT names:REC required-property-ids:LIST optional-property-ids:LIST] {
  class.valve group.valves _ {preferred=locale=en text="Control valve"
  alternatives [locale:TEXT text:TEXT] {
  }
  } [ prop.dn ] [ ]
}
```

Four things in those bytes that `payload = OCTET+` does not describe, and that no other committed
document in this repository describes either:

1. **Nested records flattened into `key=key=value` runs with no delimiter.**
   `catalogue=id=cat.demo`, `names=preferred=locale=en text="Demo Manufacturer"`. A reader cannot
   tell how deep `names=preferred=locale=en` goes, or where the record ends and the next sibling
   field begins, without being told.
2. **Typed table blocks** — `name [col:TYPE …] { rows }` with a declared type vocabulary
   (`TEXT`, `UINT`, `REC`, `TABLE`, `LIST`, `MAP`) whose members are nowhere enumerated.
3. **A table cell that contains a record which contains another table block**, closing with a bare
   `}` on its own line inside a row — which is the exact token the Python reference reports:
   `'}' is not a key=value field`.
4. **A sentinel `_` for an absent optional field**, and `[ … ]` / `{ }` for empty list and empty
   record — three more spellings with no document.

The grammar file is not merely thin. Its own `header` production is **factually wrong about the
artifact**: it says the file opens `schema stdio.json`, and the file opens `semio norm.iso16757.dsl v1`.

### Scale

`payload = OCTET+` is the committed body of **46 of the 112** text-snapshot grammars under `✏️s`
(`find ✏️s -name 📖️component.grammar.semio -path "*📸️snapshot/📝️text*"`), including **all fifteen**
`📕️norm` subsets. The other thirteen norm cases' `identity-round-trip` passes only because their
documents happen to be flat `key=value` lines that a reader can guess; the grammar is exactly as
absent there. `iso16757` and `vdi3805` are simply where the absence became load-bearing.

### What has to be decided to resolve it

1. **Where does the `.dsl.semio` notation's grammar live?** It is one notation shared by every
   subset that emits `ArtifactDsl`, not fifteen notations. Either the framework's DSL notation module
   publishes one grammar document and each subset's `📖️component.grammar.semio` references it, or
   each subset publishes its own — but 46 placeholders that all say `OCTET+` is neither.
2. **Is the nesting spelling normative or incidental?** `key=key=value` with no delimiter is
   ambiguous on its face (`a=b=c` could be `a.b = c` or `a = "b=c"`). If it is normative, the
   grammar must state the disambiguation rule. If it is incidental to the current printer, the
   printer is writing a format nobody has agreed to.
3. **What is the type vocabulary of a table header?** `TEXT UINT REC TABLE LIST MAP` appear in
   committed artifacts and in no committed document.
4. **Should a placeholder grammar be a contract breach?** Today `payload = OCTET+` satisfies every
   rule the testing domain has: 30 rule ids, none of which is *"a committed grammar must describe its
   committed examples."* A `grammar-is-placeholder` rule over the 46 would turn a silent gap into a
   counted one.

Answering (1) and (2) closes scenarios 3 and 4 at the cause and moves no comparison profile.

---

## What was NOT done to any of them

Verified for all five, against the working tree:

* **No comparison profile knob changed anywhere.**
  `git diff HEAD -- '*🔣️component.json' | grep -E '^[+-]\s*"(ignoreKeys|tolerance|arrays|mode)"'`
  returns nothing.
* **No fixture involved in any of the five was swapped, normalised or deleted.** The four `en1990`
  vector files, the two `.dsl.semio` example artifacts and the four `jack` vector files are all
  unmodified in the working tree.
* **No scenario was deleted and no assertion was removed.** The two `📕️norm` carrier scenarios keep
  their `Then the Rust side reproduces the committed file byte for byte and the Python side refuses,
  because this carrier's notation is specified nowhere` — the refusal is *asserted*, not tolerated.
* **The Python reference was refactored under them and they did not move.** All fifteen `📕️norm`
  adapters were rewritten this wave to import one shared engine instead of holding fifteen
  byte-identical copies of it (see `📓️w15-oracle-duplication-and-modes.md`); an offline harness
  replayed all **799** planned scenarios through the old and the new adapters and every one produced
  a byte-identical projection, a byte-identical raw digest and — for the four red ones — the
  identical exception type and message:

  ```
  [equivalence] scenarios=799 identical=799 mismatched=0
  ```

## One sentence

The five red scenarios are two specification holes: a composed child's identity is a hash from a
library that refuses to specify it, and the text carrier fifteen subsets write has no grammar — and
both are repository-scale (58 hasher call sites, 46 placeholder grammars), not local to the five
cases where a second implementation happened to walk into them.
