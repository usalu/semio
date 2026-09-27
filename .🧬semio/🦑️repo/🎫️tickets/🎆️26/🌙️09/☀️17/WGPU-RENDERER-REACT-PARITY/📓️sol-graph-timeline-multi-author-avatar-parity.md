# GraphTimeline Multi-Author Avatar Parity

Astra verification: the mounted React oracle passed 1 test with 703 skipped in 35.08 seconds (`🗑️generated/astra-runtime/clock30/react-graph-authors-2.log`). The first attempt failed because this repository's custom `fireEvent` has no `load` method; the test now dispatches the actual image load event inside React `act`, preserving its image/fallback assertions. Native graph author laws remain pending.

## Scope

This packet closes the GraphTimeline author projection gap recorded in `📓️terra-consolidated-current-parity-audit.md`. React already renders every authored history author through `HistoryTable` and `TableAvatar`; the retained WGPU decoder previously retained only `name`, and its painter displayed initials for only the first author.

## Shared law

The language-neutral fixture is `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/👥️graph-timeline-authors/🔣️.json` with schema sibling `🧬️schema/👥️graph-timeline-authors/🔣️.json`. It fixes:

- the authored order `Ada Lovelace`, then `Grace Hopper`;
- stable author identifiers `ada` and `grace`;
- one valid exact-source inline avatar and one initials fallback;
- 20 logical-pixel circles, 8 logical-pixel overlap, and 12 logical-pixel origin advance;
- an invalid replacement source used to prove old raster pixels are not reused.

## Production behavior

`HistoryColumnAuthorJson` now decodes `id`, `name`, and a non-empty optional `avatar` while preserving source array order. Each author is painted in order. The image owner key is derived from the accepted scene host id, checkpoint id, and author id; when an id is absent, the authored row index is the bounded fallback. Avatar sources use the existing `current_ui_image_key` exact-source raster admission and `push_rounded_raster_quad` compositor. A source that has not decoded, has no image, or replaces a previously decoded source paints that author's initials instead.

The painter uses the same outer border and inset image/fallback structure as the existing retained VFS avatar implementation. Later authors paint after earlier authors, matching React's negative-space stack order.

## Oracles and native laws

The actual mounted React `GraphTimelineHost` oracle in `engine-contract/🟦️.ts` validates the fixture through AJV, asserts the two `data-slot=avatar` nodes in authored stable-id order, the `-space-x-2` overlap owner, exact image `alt` and source, fallback-until-load behavior, the initials-only second author, and fallback after source replacement.

The native scene laws are:

1. `shared_author_fixture_preserves_authored_identity_order_overlap_and_initials`
2. `graph_timeline_paints_every_author_and_never_reuses_a_replaced_avatar_source`

The second law verifies both outer avatar circles at 12 logical-pixel origin separation, two independent fallback circles before a valid source is admitted, only Ada's exact raster after admission, and no raster from Ada's former source after replacement.

## Validation

- Shared JSON Schema: passed with strict AJV via Bun.
- Rust source and native law file: parsed and formatted with Rust 2021 `rustfmt`.
- Focused mounted React oracle command:
  `SEMIO_TEST_LEVEL=long bun nx run @semio-tech/framework-renderer-react:test -- ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --run --silent=false --reporter=verbose -t 'renders every GraphTimeline author'`
- Native execution is owned by the root task and should use the two exact law names above.
