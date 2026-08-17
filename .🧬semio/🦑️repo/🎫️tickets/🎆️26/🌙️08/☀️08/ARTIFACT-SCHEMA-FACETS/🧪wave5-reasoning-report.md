# Wave 5 Report — Reasoning Wires (`semio-s-plugin-reasoning-mindmap`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/💡️reasoning/**` plus this ticket folder.
Key `wires`, prefix `Wires`, schema id `s.reasoning.wires`. Former snapshot type `MindmapWiresDocument` → `WiresSnapshot`.

## 1. Field inventory (final)

| Field | State | Notes |
| --- | --- | --- |
| `wiresFixture` | persistent | opaque `DslValue` (identities, relationships, …) |
| `boardFixture` | persistent | opaque `DslValue` (board nodes/edges layout) |
| `selectedIds` | shared-ui | from `WiresConfig` |
| `dragNodeId` | preview | optional in-flight drag target |
| `dragLastX` / `dragLastY` | preview | last pointer during drag |
| `locale` | local-ui | BCP-47 tag |
| effect | — | none |

Snapshot facet = `wiresFixture` + `boardFixture` only. `DocumentApp` uses `Snapshot` / `initial_snapshot` / `doc.snapshot` / `cfg.snapshot`.

## 2. Diff-delta shape

`WiresDiff` sparse field delta:

- `artifact: Option<Box<WiresArtifact>>` — whole replacement wins
- persistent: `wiresFixture: Option<DslValue>`, `boardFixture: Option<DslValue>`
- shared-ui: `selectedIds: Option<WiresStringList>`
- preview: `dragNodeId: Option<Option<String>>`, `dragLastX/Y: Option<f64>`
- local-ui: `locale: Option<String>`

`WiresStringList` wraps `values: Vec<String>` for list patches. `MutationDiff<WiresSnapshot>` applies persistent entries; `apply_to_artifact` applies all classes.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]`. Nested:

- `artifacts::wires::schema`
- `artifacts::wires::snapshot::{schema, pack}`
- `artifacts::wires::diff::{component, schema}`

TypeScript `📦️index.ts` exports pack under snapshot plus schema/diff/dsl/spr/op facades.

Normative JSON Schema leaves use `title: "DslValue"` on fixture fields (scalar parity with Rust `DslValue`; not `additionalProperties` maps). Proto/graphql use `DslValue` message/scalar; diff proto embeds nested `WiresArtifact` like draw.

## 4. Other structural changes

- Pack moved: `🎒️pack/` → `📸️snapshot/🎒️pack/`
- `MindmapWires*` types → `WiresArtifact` / `WiresSnapshot` / `WiresDiff` / `WiresEngine`
- `ReplaceDocument` / `set-document` → `SetSnapshot` / `🖼️set-snapshot`
- `WiresConfig` envelope id `wires.config` (extension still `reasoning.wirescfg`)
- `wires_artifact_schema_descriptor()` registered from engine `register()`
- `metabolism_wires_example_snapshot()` falls back to handcrafted metabolism when bundled `🗣️example.dsl.semio` is still the stub envelope

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-reasoning-mindmap

```
    Finished `dev` profile [unoptimized] target(s) in 0.60s
```

(Full check also reports framework warnings and future-incompat notes for `block` / this crate; no errors.)

### cargo test -p semio-s-plugin-reasoning-mindmap --lib

```
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'wires|reasoning'

```
```

(empty — no policy lines matching plugin/artifact names)

Direct `policyArtifactSchemaBreaches(root)` filter on `scope` containing `reasoning`: **0** artifact-schema breaches after leaf parity fixes.

## 6. Shared-surface blockers

None for this plugin after aligning fifteen leaves on `DslValue` cardinality (policy treats `object` + `additionalProperties` as `map`, which broke parity with Rust `DslValue` scalar).

Repo MCP `ticket_close` was not available in this agent session (no `repo` namespace).

## 7. Not validated / follow-ups

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` remains the stub `semio demo.dsl v1` envelope; metabolism demo uses `handcrafted_metabolism_snapshot()` at runtime. DSL round-trip tests exercise printed metabolism from that path, not the asset file alone.
