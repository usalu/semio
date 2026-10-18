## 3. Buckets and fixes (in progress, 2026-10-17 23:40)

Fixed (all re-run green with `bun test <file>`):

- Fixture moved into `🧪️tests/🧫️fixtures`, TS reader kept the old path: procedural examples x8 (`📚️examples/🧪️tests/🧩️geometry`), brep inference support `loadFixture` (+ new `loadModuleFixture` for brep-interchange), mesh-support oracle, generate-interactions, process3d timber-replay, norm oracle-source-ownership control path.
- Asset folder renamed, TS path stale: shooting `🌲️hexagonal-cut`, vdi3805 `🌶️nonconforming`.
- Trailing slash lost on a directory URL root (`new URL("<dir>", url)` then `new URL(rel, root)`, or string concatenation): shooting graphql/proto facets, trinity rewriting snapshot (5 roots, 5 concatenations, capture root), trinity binary64-transport.
- SQLite capability moved from `🧬️schema/📸️snapshot` to its `🚪️io/🪶️sqlite/📸️snapshot` owner: tests now import the sqlite owner (shooting, demonstrator, fem2d).
- Schema moved on, test stale: animate diff u32 page (source patch fields are `{value}` wrapped), equation sparse expression diff (`kinds`/`nextLabel`), architect program JSON delta codec (`removed/inserted/moved/modified`; codec `delta()` in `🚪️io/📝️text/🔺️diff/🔣️json` fixed, plus the contract test and the native-roles oracle), trinity typed-delta fixtures (null = unchanged; schema document list gained `artifact-reference` and the jack diff; graphql roots gained `artifact-reference`), stdio compact-patch jpg row (snapshot is now `{schema,image{...}}`).
- Project targets use executor `repo:script`: wfc bitmap (2 tests), energy capability.
- Rust source-text laws followed the carves: procedural geometry registry (crate names), flow tessellate (`fn invoke(&mut self`), draw interaction topology (`🖥️play-app`), layout media export (producer now in the editor root; schema + fixture + `🔣️schema-catalog.json` hash updated), stdio retained-native refusal (OneItemOwners delegation; fem3d store-preparation path), trinity member admission (ingress and composition files).
- Removed by design (evidence: commit 251fa724c4a deleted the `🩹️patch-snapshot` and `📸️set-snapshot` leaves of every stdio artifact): the schema/leaf/protocol half of `nativePilots` in the stdio compact-patch test (the patch algebra half stays).
- Removed by design: the layout `🧬️schema/🧪️tests/🔣️transport` doc-contract half. The Drawing child codec slot is bound in `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing` (owner module `🪪️document` deleted in 36fbe9da786); layout keeps an owner-only transport test and the composition test keeps the full oracle.
- Vendor vocabulary: `x-semio-collection-catalog` added to `🧰️framework/🔨️modules/🧬️schema/🧬️vendor-annotation-vocabulary/🔣️.json` (the architect snapshot schema carries it).

Environment, not fixable here: unlinked workspace packages (`@semio-tech/writer-writer`, `gis-gisterrain`, `gis-gismap-js` has no dist, `stdio-las`, `stdio-pdf`, `stdio-dwg`, `space-home`, the cad extension packages for vitest), undeclared `@xmldom/xmldom`, unbuilt `dist` (wfc unlinked public artifact), swap exhaustion (SIGKILL / rc 137 on heavy suites), brep fixtures without pinned meshes (need the cargo refresh), geometry-service fixture is `[]`.

