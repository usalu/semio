# Remaining Complete Semantic Owner Backlog

Read-only actual Bun SQLite schema census executed authored SQL for all six remaining subsets, queried actual table declarations/column counts and summed SQLite master SQL bytes plus table names. No owner projections or provider edits occurred. Width and schema values below are actual schema measurements, not full populated semantic value counts.

| Owner | Domain Tables | Maximum Columns | Canonical Schema Bytes | Canonical Fixture Scope |
| --- | ---: | ---: | ---: | --- |
| CAD | 14 | 25 | 5607 | Two layers, one block, nine entity variants plus a block line and polyline vertices |
| Document | 18 | 13 | 3580 | Eight block variants, recursive list/table/quote content, style/image identities and nullable run fields |
| Drawing | 16 | 34 | 5913 | Canvas/styles, recursive groups, six path commands, text/image nodes, Binary32 and Binary64 |
| Presentation | 26 | 18 | 5669 | Masters/layouts/slides, seven placeholder kinds, picture bytes and embedded Document content |
| BRep | 25 | 54 | 9552 | Four 3D/four 2D curve kinds, six surface kinds, NURBS payloads and reciprocal topology |
| Base Header Only | 1 | 21 | 1272 | Selected union header plus composition of all eighteen dedicated owners |

Choose CAD next. Its fourteen tables are the smallest complete dedicated domain among this backlog, with finite nonrecursive geometry variants and simple layer/block identity lookup. Document and Drawing require recursive content ownership; Presentation composes Document before its own shapes; BRep combines many geometry streams and topology references. Base is not a one-table completion: actual Source tests require all eighteen selected branches and aggregate schema/header accounting, so it must follow dedicated owner closure.

CAD canonical sources are actual `📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json`, `🧪️tests/🦀️.rs` fixture(), `🧪️tests/🟦️.ts` input and `🗄️.sql` under the current Semio v1 subsets path. Current Source positive input is deliberately different from Native constructor: ordinary Source handles are numeric strings while Native uses entity-N/block-line; Source ellipse end parameter2 versus Native5; Source polyline has two vertices versus Native three; Source text/insert/dimension values differ from Native. Therefore exact matched demand must handcraft the actual Native full constructor in Source carrier form, preserving all nine variants and block line; existing Source input cannot be relabeled as a matched Native fixture. Every float must capture Binary64 word identity before hydration. Metadata retaining empty should clear layers/blocks/entities and preserve schema.

The next complete populated CAD census should independently insert actual Native semantic rows into actual SQL and then validate foreign-key joins plus literal text/polyline role queries; root must not derive expected rows/bytes from owner projection. This backlog report does not claim that unperformed populated CAD census or future Native/Source provider laws passed.
