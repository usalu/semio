# Details Capability Audit — 2026-09-27

Read-only source audit of the stdio snapshot Details surface. No source file
was changed and no Cargo/Nx job was started. The shared native validation lane
was already queued, so this document makes no runtime-green claim.

## Scope and evidence

- Details renderer and provider:
  `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs`
- Its unit tests:
  `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🧪️tests/🔬️unit/🦀️.rs`
- The retained snapshot action parser and validation boundary:
  `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`
- The schema validator supports cross-document `$ref`, `allOf`,
  `minProperties`, and `minLength`:
  `🧰️framework/🔨️modules/🧬️schema/✅️validator/🦀️.rs:172-185,458-484,867-944,1081-1088`.

The renderer correctly pages visible collection rows through
`tree_window_indexed_section`, preserves exact numeric JSON through
`valueEncoding: "json"`, uses RFC 6901 paths, hides the root `schema` edit,
hides fixed-key rename and required-field removal in the directly resolved
case, and applies `minItems`/`maxItems` plus `maxProperties`. The present
tagged-union switch implementation is deliberately not reported as absent:
work on that feature is active.

## P1 — The registered Semio envelope loses schema capabilities beneath its external-ref `allOf` variants

**Affected code**

- `🪟️details/🦀️.rs:112-119` resolves only `#/...` references.
- `🪟️details/🦀️.rs:246-280` descends only direct `properties`,
  `additionalProperties`, and `items`.
- `🪟️details/🦀️.rs:381-459` chooses the first `allOf` member for a template
  instead of composing the conjunction.
- `🪟️details/🦀️.rs:779-822` can only offer a variant when that incomplete
  template succeeds.

**Production schema shape**

The registered descriptor is `s.stdio.semio`:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🦀️.rs:40-65`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🔣️.json:7-280`

At `/subset`, the schema is a local `oneOf` with eighteen branches. Each
branch has a local `subset` const discriminator and an `allOf` whose first
member is an external sibling snapshot `$ref`. For example, the `model`
branch points to
`https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/model/snapshot.json`;
that source is the registered `s.stdio.semio.model` snapshot descriptor at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🦀️.rs:49-73`.

The renderer can identify the immediate branch from `/subset/subset`, but it
cannot resolve the external member nor merge its fields with the discriminator
constraints. Thus it cannot associate `/subset/schema`,
`/subset/spatial/0/id`, or any deeper member with their fixed schema. Those
rows fall back to generic edit/rename/remove controls; required-removal
blocking, enum controls, localized metadata, schema-backed missing-property
templates, and collection bounds do not reach them. It also cannot construct
a complete, valid replacement for `/subset`, so the new variant-switch surface
has no production-shape target to render here.

The existing tagged-union test is useful but only passes a fully local schema
to `from_value_and_schema` (`🧪️unit/🦀️.rs:365-461`). It cannot detect this
registered descriptor failure.

Resolve documents from `with_artifact_schema_registry(|registry|
registry.iter())`, index each `descriptor.snapshot.json_schema` by its `$id`,
and give Details a bounded resolver for `<$id>#/pointer` as well as `#/pointer`.
Compose `allOf` for property lookup, required/min/max constraints,
annotations, discriminator detection, and template construction. The public
validator API `OwnedJsonSchemaValidator::compile_with_documents` establishes
the same document model but cannot expose a resolved schema tree, so the
renderer needs its own small read-only resolver. Add a test with the envelope
and at least two external sibling documents: it must select by
`/subset/subset`, block required removal and fixed rename in the selected
document, render its enum/EN-DE metadata, and generate a validator-accepted
complete alternate variant.

## P1 — Paged large objects still trigger unbounded key scans while rendering controls

**Affected code**

- `🪟️details/🦀️.rs:681-690` builds `current` by reading every object key to
  discover missing fixed properties.
- `🪟️details/🦀️.rs:1264-1277` repeatedly scans every key while trying
  `new`, `new2`, … for an untyped dynamic property name.
- Both run from `collection_controls` at `:1279-1313`, before the indexed
  child page at `:1420-1433` is used.

The existing sixteen-million-item test only uses an array
(`🧪️unit/🦀️.rs:93-98`), so it does not cover an object with a large dynamic
key set. A map containing `new` through `newN` performs an O(N²) sequence of
native `object_key` reads to discover `new(N+1)`. Any large object performs an
O(N) full key materialization merely to decide whether a handful of declared
properties are absent. This bypasses the Details paging guarantee and has no
progress or cancellation path.

Extend the provider contract with bounded targeted key presence/name allocation
operations, implemented natively without projecting all keys. For declared
fixed properties, probe each candidate path directly rather than enumerate the
map. For a dynamic key, make the controller allocate a conflict-free name or
provide an O(1)/bounded provider allocator. Add a lazy-object test that counts
key reads and proves rendering a small page does not scale with total keys.

## P2 — A derived insertion address can exceed the value-list budget after its parent has passed the bindability check

**Affected code**

- `detail_node` admits a collection when its current pointer is bindable
  (`🪟️details/🦀️.rs:1395-1419`).
- `collection_insert_path` then appends `/-` or `/new` (`:1264-1277`).
- `pointer_argument` emits every chunk without enforcing
  `UI_VALUE_MAX_ITEMS` (`:1007-1027`); only `path_is_bindable` applies that
  bound (`:1029-1050`).

For a dynamic object whose parent pointer consumes exactly the allowed chunk
budget, the parent renders as bindable. Adding `/new` makes the derived insert
address one chunk too long. The first add control calls `pointer_argument`,
`UiListBuilder::push` refuses the extra item, and the `?` propagates an
assembly failure for the complete Details panel instead of using the existing
source fallback. The same boundary exists for a move destination when its
decimal index has one more byte than the visible source index.

Check every derived address (`insert_path`, alternate move destination, and
missing-property path) with `path_is_bindable` before building controls. When
it is not bindable, omit that micro-control and expose the already-supported
whole-source draft. Add an exact maximum-chunk parent-key test; the rendered
tree must succeed and retain an editable source path.

## P2 — Object removals ignore `minProperties`

`item_capabilities` suppresses a key removal only when that key is in
`required` (`🪟️details/🦀️.rs:743-756`). It never compares the parent’s current
property count to `minProperties`, while insertion correctly observes
`maxProperties` at `:768-777`. The authoritative validator rejects the
resulting object once the count falls below `minProperties`
(`✅️validator/🦀️.rs:934-944`), so the UI can advertise an action that it
cannot complete.

When an object has `length <= minProperties`, hide removal for every key,
whether fixed or dynamic. Keep the required-key check as an independent
constraint. Add a three-property schema with `minProperties: 3` and one
optional key; the optional key must have no remove control, while adding is
still governed separately by `maxProperties`.

## P2 — “Create valid item” does not validate defaults or compose all object constraints

`template_from_schema` returns `default`, `const`, the first example, or the
first enum before it evaluates the type constraints
(`🪟️details/🦀️.rs:381-403`). JSON Schema defaults are annotations rather than
a guarantee of validity. For example, `{ "type": "string", "minLength": 2,
"default": "" }` produces `""`, which the shared validator rejects under
`minLength` (`✅️validator/🦀️.rs:1081-1088`).

Likewise, object synthesis only supplies direct `required` properties
(`🪟️details/🦀️.rs:420-435`). A valid schema with optional `title` and
`minProperties: 1` produces `{}` even though it is invalid; the same blind
spot is part of the external-`allOf` production failure above. The current
template test asserts that its JSON parses through `serde_json`
(`🧪️unit/🦀️.rs:171-242`), but does not feed every offered template to the
authoritative validator.

Compile the selected schema plus sibling documents once, validate every
prospective template before exposing it, and omit a control when no valid
template can be constructed. Add direct, nullable, `minProperties`, invalid
default, and composed-`allOf` cases, asserting both the native validator and
the existing third-party JSON oracle agree on the offered value’s JSON form.

## Coverage to preserve

Existing tests already validate the good local behavior for native lazy
reads, a requested array page, JSON-number transport, EN/DE annotations,
fixed versus dynamic keys, required fields, `minItems`/`maxItems`, long RFC
6901 paths, and an all-local tagged union. The additions above should extend
that suite rather than replace it.
