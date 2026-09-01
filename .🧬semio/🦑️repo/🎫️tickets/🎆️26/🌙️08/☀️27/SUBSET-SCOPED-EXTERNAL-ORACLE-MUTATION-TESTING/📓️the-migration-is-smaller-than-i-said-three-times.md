# 📐️ The leaf migration is smaller than I said — three corrections, each in the same direction

I have now been wrong about this migration's cost three times, always by overstating it, and always by
asserting a constraint instead of measuring it. Recording all three together because the pattern is the
point: every "this is a big semantic migration" claim I made dissolved when checked.

## 1. "The wire format changes" — false

Claimed that newtype variants would move every fixture, diff codec and text codec. Measured with a
standalone serde crate: a newtype variant over a struct serialises **byte-identically** to a struct
variant, internally (`tag=`) and adjacently (`tag=`+`content=`) tagged.

```
internal struct : {"mutation":"setVersion","major":1,"minor":4}
internal newtype: {"mutation":"setVersion","major":1,"minor":4}          IDENTICAL: true
adjacent struct : {"mutation":"setCell","payload":{"row":2,"value":"x"}}
adjacent newtype: {"mutation":"setCell","payload":{"row":2,"value":"x"}} IDENTICAL: true
```

## 2. "`SetSnapshot` cannot be a leaf" — false, and this was the expensive one

Claimed that because `is_approved_verb("set-snapshot")` is false, `SetSnapshot` cannot become a leaf,
so all 60 aggregates must DELETE it — 574 call sites losing their whole-document restore semantics.

**I conflated two different descriptor fields.** The derive asserts:

```rust
assert!(str_eq(#kind::SEMANTICS.kind, #expected_kebab), "Mutations semantic kind must match its variant");
assert!(is_approved_verb(#kind::SEMANTICS.verb),        "Mutations requires an approved semantic verb");
```

`kind` must equal `kebab(variant)`. `verb` is a SEPARATE field and only it is checked against the
table. The migrated reference proves it — `✳️drawing`'s rotate leaf is
`{ verb: "rotate", entity: "node", kind: "rotate-node" }`; verb ≠ kind by construction.

So `SetSnapshot` migrates intact as `{ verb: "set", entity: "snapshot", kind: "set-snapshot" }`. `set`
is approved. **No semantics lost, no wire change, 574 sites get a syntax change only**
(`X::SetSnapshot { snapshot }` → `X::SetSnapshot(SetSnapshot { snapshot })`).

## 3. "Each artifact forces a semantics decision" — overstated

With `SetSnapshot` surviving, the only variant that genuinely cannot migrate is `NoMutation`: it is a
UNIT variant, and the derive requires every variant to wrap exactly one payload
("Mutations requires every variant to wrap exactly one direct MutationKind payload"). No approved verb
means "do nothing" either — the 41 are `add append apply bind change clear commit connect create
delete disconnect drag duplicate edit extract finish fix flatten group inline insert merge move remove
rename reorder replace resize restore rotate scale seal set split start switch toggle unbind unflatten
ungroup`.

That leaves ONE decision per artifact, with two honest answers:

* **delete it**, as `✳️drawing` did — its `inverse()` no-op arms become `vec![]`, which is
  semantically identical (nothing to undo); or
* **rename it** to an approved-verb kind (`apply` is available), preserving the variant.

Either way `#[derive(Default)]` must go — `✳️drawing` does not derive it — and anything calling
`::default()` on an aggregate needs a source.

## Corrected size

| | as I claimed | measured |
|---|---|---|
| aggregates | 60 | 60 |
| leaves to author | ~550 | ~550 |
| variants forced to change SEMANTICS | `NoMutation` + `SetSnapshot` | **`NoMutation` only** |
| call sites losing behaviour | ~1364 | **790** (`::NoMutation`), and zero if renamed rather than deleted |
| `::SetSnapshot` sites | rewritten semantically | **574, syntax only** |

## The worked example, and why it is now tractable

`📐️step/🔖️ap214/✳️cc1` is the smallest aggregate: 5 variants, and its module header already states
each kind's conformance rule and its inversion behaviour. Under the corrected constraints it needs four
leaves — `set-snapshot` (verb `set`), `set-file-schema` (verb `set`), `set-product-identity` (verb
`set`), `remove-shape-representation` (verb `remove`) — and one decision, `NoMutation`, whose
`inverse()` arm already returns "nothing to undo".

Critically, its documented inversion strategy SURVIVES: the header explains that
`remove-shape-representation` degrades to `SetSnapshot` because CC1 owns no verb for the restored
state. Under correction 2 that escape hatch is still available, so the migration costs cc1 nothing
semantically. My earlier reading — that migrating cc1 would strip its undo — was a consequence of the
mistake, not of the migration.
