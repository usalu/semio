@capability-layout-1-mutate
@oracle-layout-1-python-independent
@comparison-ordered-json-v1
@mutations-layout-1-any
Feature: Apply every typed layout-document mutation to its committed specification vector and against an independent Python implementation
  `s.layout.layout` is a semio-NATIVE artifact: no third party reads or writes
  `.dsl.semio`/`.pack.semio`, so no reference LIBRARY is registered — confirmed again, from the
  carrier side, by this subset's own `layout-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`): none of the five export serializers
  this repository already links as third-party test oracles (dxf 0.6, png 0.18, svg, dwg, pdf) reads
  this artifact's own shape — each either coerces it into a permanently empty document, errors
  outright, or re-emits the artifact's own internal DSL text unparsed. The second producer a
  differential comparison needs is therefore a second IMPLEMENTATION, and `🐍️.py` beside
  this file is it: all 25 kinds of this vocabulary, written in Python from this subset's own
  committed `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` document shape and each
  mutation's own committed `(before, mutation, after)` specification vector, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  verb table and `📓️derivation-rules.md`'s per-collection shape rules. It imports nothing from the
  Rust it judges and transliterates none of it. The no-oracle decision this replaces
  (`layout-mutation-semantics`) is narrowed to an empty `capabilities` list rather than deleted (it
  already was, by a prior shard of this same ticket), because its own investigation — including the
  carrier-side serializer survey above — remains the honest record of what was checked; a dated note
  is appended recording that the `asset://` blocker it named is now resolved.

  Both implementations now read the SAME committed bytes: every `(before, mutation, after)` path is a
  declared `asset://` fixture rather than an `include_str!`-only literal, so the plan pins its digest
  and a Python reference can resolve it.

  What distinguishes this subset from every sibling is that a layout document is FOUR pools at TWO
  nesting depths, joined by reference. Three scalars sit at the document root (`name`, `printTarget`,
  `dataFieldsJson`); `pages`, `stories` and `links` are id-keyed root collections; and `frames` and
  `layers` live one level down, inside a page, so a frame is addressed by (page, frame) and never by
  id alone. A text frame names its `stories` member by id and an image frame names its `links` member
  by id, which is why `delete-story` and `delete-link` reach ACROSS pools into frames that point at
  them, and why `reorder-pages` moves a whole nested subtree rather than permuting a flat list.

  The Examples rows are chosen against exactly that: `delete-page` removes the MIDDLE page so undoing
  it has to restore an index and not append; `delete-frame` removes a text frame and its layer
  membership together, so an inverse that put the frame back without re-listing it in the layer's
  `objectIds` fails; and `change-print-target`/`change-data-fields` set an optional root scalar that
  was previously `null` (`printTarget`) or genuinely ABSENT (`dataFieldsJson`), so their inverse has
  to clear it again exactly as it was — the Python reference's first standalone run against these
  committed vectors caught exactly this distinction as a real bug (an inverse that wrote `dataFieldsJson:
  null` instead of omitting the key), fixed before registration.

  `mutate-<kind>`/`inverse-<kind>` now dispatch BOTH an oracle role (the Python implementation) and a
  subject role (this repository's own `apply_layout_mutation_json`/`undo_layout_mutation_json`,
  unaffected by this change, still reading the committed vectors through `include_str!`): each side
  answers in role, then the two are compared, and the subject additionally asserts the observability
  and inverse laws it always has through the shared `⚖️law` module.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed before-document asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-document asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When <id> is applied through apply_layout_mutation_json
    Then the resulting document is the committed after-document, the mutation moved it, and the two implementations agree
    Examples:
      | id                     | dir                     | fixture                                         |
      | rename-layout          | ✏️rename-layout          | 🏷️renames-the-document                            |
      | change-print-target    | 🖨️change-print-target    | 🖨️sets-a-cmyk-print-target                        |
      | change-data-fields     | 🧾change-data-fields     | 🧾️attaches-a-data-fields-payload                  |
      | create-page            | 🌱create-page            | ➕️appends-page-3                                  |
      | delete-page            | 🗑️delete-page            | 🚫️removes-page-2                                  |
      | rename-page            | 🏷️rename-page            | 🏷️renames-page-1                                  |
      | change-page-width      | ↔️change-page-width      | ↔️widens-page-1                                   |
      | change-page-height     | ↕️change-page-height     | ↕️lengthens-page-1                                |
      | update-page-margins    | 📐update-page-margins    | 📐️sets-asymmetric-margins-on-page-1               |
      | update-page-columns    | 🏛️update-page-columns    | 🏛️splits-page-1-into-three-columns                |
      | reorder-pages          | 🔀reorder-pages          | 🔀️moves-page-1-behind-page-2                      |
      | create-story           | 📖create-story           | 📖️appends-story-3                                 |
      | delete-story           | 📕delete-story           | 🚫️removes-story-2                                 |
      | edit-story             | ✍️edit-story             | 📝️rewrites-story-1-body                           |
      | create-link            | 🖇️create-link            | 🔗️appends-link-3                                  |
      | delete-link            | ✂️delete-link            | 🔗️removes-link-2                                  |
      | change-link-path       | 🛤️change-link-path       | 🔗️relinks-link-1-to-a-new-file                    |
      | create-frame           | ➕create-frame           | 🔲️inserts-a-rect-frame-at-index-1                 |
      | delete-frame           | ➖delete-frame           | 🚫️removes-the-text-frame-and-its-layer-membership |
      | move-frame             | 🕹️move-frame             | 📍️moves-the-rect-frame                            |
      | resize-frame           | 📏resize-frame           | 📐️resizes-the-rect-frame                          |
      | change-frame-fill      | 🎨change-frame-fill      | 🎨️repaints-the-rect-frame-fill                    |
      | change-frame-stroke    | 🖊️change-frame-stroke    | 🖊️adds-a-stroke-to-the-rect-frame                 |
      | change-frame-wrap-mode | 🔤change-frame-wrap-mode | 🔤️switches-the-text-frame-to-column-wrap          |
      | change-frame-columns   | 🔢change-frame-columns   | 🔤️splits-the-text-frame-into-two-columns          |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    When <id> and then every step of its own computed inverse are applied through undo_layout_mutation_json
    Then the document is the committed before-document again, member positions included, and the two implementations agree
    Examples:
      | id                     | dir                     | fixture                                         |
      | rename-layout          | ✏️rename-layout          | 🏷️renames-the-document                            |
      | change-print-target    | 🖨️change-print-target    | 🖨️sets-a-cmyk-print-target                        |
      | change-data-fields     | 🧾change-data-fields     | 🧾️attaches-a-data-fields-payload                  |
      | create-page            | 🌱create-page            | ➕️appends-page-3                                  |
      | delete-page            | 🗑️delete-page            | 🚫️removes-page-2                                  |
      | rename-page            | 🏷️rename-page            | 🏷️renames-page-1                                  |
      | change-page-width      | ↔️change-page-width      | ↔️widens-page-1                                   |
      | change-page-height     | ↕️change-page-height     | ↕️lengthens-page-1                                |
      | update-page-margins    | 📐update-page-margins    | 📐️sets-asymmetric-margins-on-page-1               |
      | update-page-columns    | 🏛️update-page-columns    | 🏛️splits-page-1-into-three-columns                |
      | reorder-pages          | 🔀reorder-pages          | 🔀️moves-page-1-behind-page-2                      |
      | create-story           | 📖create-story           | 📖️appends-story-3                                 |
      | delete-story           | 📕delete-story           | 🚫️removes-story-2                                 |
      | edit-story             | ✍️edit-story             | 📝️rewrites-story-1-body                           |
      | create-link            | 🖇️create-link            | 🔗️appends-link-3                                  |
      | delete-link            | ✂️delete-link            | 🔗️removes-link-2                                  |
      | change-link-path       | 🛤️change-link-path       | 🔗️relinks-link-1-to-a-new-file                    |
      | create-frame           | ➕create-frame           | 🔲️inserts-a-rect-frame-at-index-1                 |
      | delete-frame           | ➖delete-frame           | 🚫️removes-the-text-frame-and-its-layer-membership |
      | move-frame             | 🕹️move-frame             | 📍️moves-the-rect-frame                            |
      | resize-frame           | 📏resize-frame           | 📐️resizes-the-rect-frame                          |
      | change-frame-fill      | 🎨change-frame-fill      | 🎨️repaints-the-rect-frame-fill                    |
      | change-frame-stroke    | 🖊️change-frame-stroke    | 🖊️adds-a-stroke-to-the-rect-frame                 |
      | change-frame-wrap-mode | 🔤change-frame-wrap-mode | 🔤️switches-the-text-frame-to-column-wrap          |
      | change-frame-columns   | 🔢change-frame-columns   | 🔤️splits-the-text-frame-into-two-columns          |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When it is parsed, printed back to DSL and parsed again through round_trip_layout_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
