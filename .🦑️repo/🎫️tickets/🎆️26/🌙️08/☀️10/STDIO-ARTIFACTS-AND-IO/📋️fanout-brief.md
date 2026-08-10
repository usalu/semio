# Fan-out Brief — Stdio Artifacts and Io

Read this before any wave work. Normative detail: `📜️normative-spec.md`. Tokens: `🧪tokens.json`. Matrix: `🧪owner-table.json`.

## Ticket
`26/08/10/STDIO-ARTIFACTS-AND-IO` · Goal `AI-OPTIMIZED-REPO`

## Exact facet dirs (copy-paste)
- builder: `🏗️builder`
- decomposer: `🪓️decomposer`
- text: `📝️text`
- binary: `💾️binary`
- deserializers: `🧩️deserializers`
- serializers: `🧵️serializers`
- stdio plugin: `🗄️stdio`

## Text leaves (8)
`📖️component.grammar.semio`, `🔤️component.ebnf`, `🅰️component.g4`, `🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto`, `🦀️component.rs`, `🟦️component.ts`

## Binary leaves (6)
`📡️component.protocol.semio`, `🔠️component.abnf`, `🥋️component.ksy`, `🌶️component.spicy`, `🦀️component.rs`, `🟦️component.ts`

## Ownership rules
- W1/W3/W7 ONLY touch taxonomy.json, root script.ts, root Cargo.toml, launch.seed.jsonc, framework os/host
- W4/W6 agents ONLY touch their assigned plugin subtree + its glue.rs
- NEVER use git modifying commands
- NEVER create files outside ticket folder OR the owned plugin/framework paths above
- Put all temp scripts/logs in this ticket folder
- Models: `cursor-grok-4.5-high` for design/hard codecs/heavy artifacts; `composer-2.5` for mechanical fan-out
- Do NOT use fast model variants

## Wave checklist
1. W0 — this brief + normative + owner table (done when files exist)
2. W1 — vocabulary + four twins + 7 new policy rules + delete 5 old + launch gates
3. W2 — `🗄️stdio` skeleton + binary/txt/json reference artifacts end-to-end
4. W3 — ArtifactBuilder/Decomposer traits + PluginBuilder::artifact_kind + registry collapse + MediaFormat deletion
5. W4a — deps: binary,txt,json,xml,deflate,zip,csv,md (Grok: deflate,zip)
6. W4b — 21 leaf formats (Grok: png,jpg,tiff,pdf,dwg,step,ifc)
7. W5 — note + cad pilots full absorb
8. W6 — 32 plugin agents migrate remaining 52
9. W7 — host/UI/mimes.csv
10. W8 — aggregate gate + close ticket

## Per-artifact migration recipe (W5/W6)
1. Move facets per path map in normative §5
2. Handcraft all missing text/binary spec leaves
3. Add `🏗️builder` and `🪓️decomposer` (rs+ts) implementing SDK traits
4. Rewrite `🚪️io` to deserializer/serializer shape using curated matrix row
5. Patch glue.rs + TS barrel
6. Delete old root facets
7. `cargo check` the plugin crate

## Stdio artifact recipe (W2/W4)
1. Create full artifact facet tree under `🗄️stdio/🗿️artifacts/<emoji><id>/`
2. Real codec (no stubs) — round-trip test + third-party test decode where possible
3. Builder + decomposer
4. IO: depend on DAG parents via serializers/deserializers
5. Example assets as conformance fixtures
6. Register kind on plugin via `.artifact_kind(...)`

## Gate commands
```
bun ./📜️script.ts policy
bun nx run @semio-tech/plugin-registry:check
bun nx run @semio-tech/plugin-registry:generate
cargo test -p semio-s-plugin-stdio
cargo check -p semio-s-plugin-<id>
```
