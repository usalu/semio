# Shared Map Mutation Audit

## Finding

`RewritingDiff` currently represents each changed map slot as
`Option<BTreeMap<String, Option<V>>>`. Its two local helpers in
`♻️rewriting/.../🔺️diff/📝️text/🦀️.rs` are generic: `apply_map_delta` gives
`None` strict-remove semantics and `merge_map_delta` replaces an earlier entry
with the later one. They serve both `parameter_bindings` and `rule_layout`, so
they do not belong to Rewriting.

The owner is `🧰️framework/🔨️modules/📡️replication/🎮️mutation`. Its
`MutationDiff::apply` and `MutationDiff::absorb` define the exact base-free,
sequential-composition law that these helpers implement. The shared map
primitive must be generic in its value `V`; it must not depend on Rewriting,
Graph's `PropertyValue`, or a product schema. A caller supplies `V` such as the
shared dynamic `DslValue`/native `PropertyValue` or `LayoutPoint`.

The two native map-ownership cases compiled, then failed as predicted (0/2):
transport decodes a `Set(PropertyValue::Null)` as removal, and last-key-wins
absorb turns a successful `Set` then strict `Remove` on an absent key into a
strict removal against the original base. The existing `fast-json-patch`
oracle correctly validates the source sequences, including their null values.

## Required Shared Representation

Use a tagged entry operation plus an explicit original-base presence
precondition. The canonical cross-language wire is a wrapper with a list of
entries; it avoids a Protobuf `optional map` and represents the same shape in
JSON Schema, TypeScript, GraphQL, and Protobuf. Decode it into a key-sorted,
duplicate-free map internally, or retain a sorted vector. Never use a JSON
object value of `null` as an operation signal.

```text
MapDelta<V> = { entries: MapEntryDelta<V>[] }

MapEntryDelta<V> =
  | { key: String, precondition: "any" | "present" | "absent",
      operation: { kind: "set", value: V } }
  | { key: String, precondition: "any" | "present" | "absent",
      operation: { kind: "remove" } }
  | { key: String, precondition: "never",
      operation: { kind: "reject" } }
```

All fields above are required. In particular, `value` is required for `set`,
so `value: null` means a present null-valued entry and an absent `value` field
is invalid. The `reject` case is the canonical result of composing incompatible
partial changes; it makes `absorb` total without inventing a successful apply
path. It must round trip even though normally produced local edit sequences do
not create it.

Direct change mutations emit `{ precondition: "any", operation: { kind:
"set", value } }`. Direct remove mutations emit `{ precondition: "present",
operation: { kind: "remove" } }`. An empty field diff is represented by an
absent outer `MapDelta`; an empty `entries` list is rejected or normalized away.

`apply_map_delta` must first validate every entry against the unmodified map,
then execute all entries. `present` on an absent key returns the existing
`mutation.apply.missing-target` error targeted at that key; `absent` on a
present key returns a distinct target-precondition error; `never` always
returns a stable unsatisfiable-precondition error. This preserves the current
all-or-nothing behavior for a map field and prevents a valid earlier entry from
being committed before a later entry rejects.

`absorb` combines entries by key, treating an entry as a partial transformer of
the original key state. It intersects the first entry's accepted presence set
with the states for which the second entry's precondition holds after the first
operation. No accepted state produces the `never/reject` entry. Otherwise the
result retains the accepted original states and the second operation. This is
ordinary composition of partial functions, so it is associative and exactly
matches sequential apply for every possible base, rather than only the bases
covered by a fixture.

The representation stores one entry per changed key. Superseded `set` payloads
are released when an entry is replaced; no edit history is retained. Its space
is `O(distinct changed keys)`, apply is `O(changed keys × map lookup)`, and
absorb is `O(changed keys × delta lookup)`. That supplies bounded coalescing
and retirement without an unbounded ordered operation log. A long-running
caller can still schedule these bounded per-entry loops through its existing
progress/cancellation path.

## Counterexamples To Last-Key-Wins

Let `A` mean absent and `P(v)` mean present with value `v`.

| Sequential entries | Resulting compact entry | Why replacement is unsound |
| --- | --- | --- |
| `set(null)` | `any / set(null)` | JSON `null` is the value, not remove. |
| `set("value")`; strict `remove` | `any / remove` | From `A`, the source sequence succeeds and returns `A`; a strict final remove rejects `A`. |
| strict `remove`; `set(null)` | `present / set(null)` | From `A`, both sequence and compact diff reject; a total `set(null)` would incorrectly succeed. |
| strict `remove`; strict `remove` | `never / reject` | From `P(v)`, the first removal succeeds and the second rejects. There is no successful original base. |
| strict `remove`; `set("x")`; strict `remove` | `present / remove` | From `P(v)` succeeds to `A`; from `A` preserves the first missing-target rejection. |

`any / remove` is a compact *erase* action, whereas `present / remove` is a
direct strict remove. They have the same final map for a present key and
different behavior for an absent key. That distinction is the minimum data
last-key-wins discards.

## Language-Neutral Vectors

These should replace the three Rewriting-only cases with a framework-owned
fixture and run against both the generic implementation and the existing JSON
Patch oracle. The oracle replays `source` in order; the framework test applies
`compact` directly. Each decoder must also round-trip `compact`.

```json
{
  "cases": [
    {
      "name": "null is a set value",
      "before": {},
      "source": [{"key":"nullable","precondition":"any","operation":{"kind":"set","value":null}}],
      "compact": {"entries":[{"key":"nullable","precondition":"any","operation":{"kind":"set","value":null}}]},
      "after": {"nullable":null}
    },
    {
      "name": "strict removal keeps missing-target rejection",
      "before": {},
      "source": [{"key":"missing","precondition":"present","operation":{"kind":"remove"}}],
      "compact": {"entries":[{"key":"missing","precondition":"present","operation":{"kind":"remove"}}]},
      "error": "mutation.apply.missing-target"
    },
    {
      "name": "set then remove is total erase",
      "before": {},
      "source": [
        {"key":"temporary","precondition":"any","operation":{"kind":"set","value":"value"}},
        {"key":"temporary","precondition":"present","operation":{"kind":"remove"}}
      ],
      "compact": {"entries":[{"key":"temporary","precondition":"any","operation":{"kind":"remove"}}]},
      "after": {}
    },
    {
      "name": "remove then set retains initial presence requirement",
      "before": {},
      "source": [
        {"key":"nullable","precondition":"present","operation":{"kind":"remove"}},
        {"key":"nullable","precondition":"any","operation":{"kind":"set","value":null}}
      ],
      "compact": {"entries":[{"key":"nullable","precondition":"present","operation":{"kind":"set","value":null}}]},
      "error": "mutation.apply.missing-target"
    },
    {
      "name": "two strict removals cannot become success",
      "before": {"item":"value"},
      "source": [
        {"key":"item","precondition":"present","operation":{"kind":"remove"}},
        {"key":"item","precondition":"present","operation":{"kind":"remove"}}
      ],
      "compact": {"entries":[{"key":"item","precondition":"never","operation":{"kind":"reject"}}]},
      "error": "mutation.apply.unsatisfiable-precondition"
    }
  ]
}
```

Add a two-key atomicity vector: one valid `set` plus one missing strict remove
must return the remove error and leave the valid key unchanged. Also test each
composition case against both absent and present bases; that establishes the
partial-function law instead of only checking the observed sequential base.

## Existing Shared Maps

No existing shared map abstraction can be reused for this contract.
`🌱️value/🗂️ordered/OrderedMap` is an immutable retained runtime container:
it silently leaves a missing removal unchanged and requires explicit root
retirement. It carries neither wire encoding nor entry preconditions, so using
it would both change strict-remove semantics and add an inappropriate lifetime
model to persisted diffs. `BTreeMap` remains an appropriate private,
deterministically ordered implementation store after validation. The existing
replication mutation contract has the correct owner and error/law vocabulary,
but no keyed-map delta abstraction yet.

## Migration Boundary

Add the generic `MapDelta`/entry algebra, codec, apply, compose, and
language-neutral tests below framework Replication's `🎮️mutation` owner.
Migrate both Rewriting map fields together and delete the local helpers. The
Rewriting artifact owns only the two field names, their values, and direct
domain mutation leaves. It should reference the shared map-delta schema in all
five language representations, with `DslValue` as the parameter value
projection and `LayoutPoint` as the layout value projection. Do not add a
legacy decoder for the ambiguous null-or-remove representation.
