@capability-layout-1-mutate
@no-oracle-layout-mutation-semantics
@comparison-ordered-json-v1
@mutations-layout-1-any
Feature: Apply every typed layout-document mutation to its committed specification vector
  `s.layout.layout` is a semio-NATIVE artifact: no third party reads or writes
  `.dsl.semio`/`.pack.semio`, so there is no reference implementation to register as an oracle. That
  is recorded as the `layout-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and its substitutes are the
  committed per-kind specification vectors plus the inverse law. This case re-exercises those SAME
  committed bytes end-to-end through `apply_layout_mutation_json`/`undo_layout_mutation_json`.

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
  was previously `null`, so their inverse has to clear it again rather than write an empty string.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion below lives in the subject handler, which compares against the committed after-document
  through the shared `⚖️law` module and fails with the first divergence named by JSON path.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_layout_mutation_json
    Then the resulting document is the committed after-document, and the mutation moved it
    Examples:
      | id                     |
      | rename-layout          |
      | change-print-target    |
      | change-data-fields     |
      | create-page            |
      | delete-page            |
      | rename-page            |
      | change-page-width      |
      | change-page-height     |
      | update-page-margins    |
      | update-page-columns    |
      | reorder-pages          |
      | create-story           |
      | delete-story           |
      | edit-story             |
      | create-link            |
      | delete-link            |
      | change-link-path       |
      | create-frame           |
      | delete-frame           |
      | move-frame             |
      | resize-frame           |
      | change-frame-fill      |
      | change-frame-stroke    |
      | change-frame-wrap-mode |
      | change-frame-columns   |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> and then every step of its own computed inverse are applied through undo_layout_mutation_json
    Then the document is the committed before-document again, member positions included
    Examples:
      | id                     |
      | rename-layout          |
      | change-print-target    |
      | change-data-fields     |
      | create-page            |
      | delete-page            |
      | rename-page            |
      | change-page-width      |
      | change-page-height     |
      | update-page-margins    |
      | update-page-columns    |
      | reorder-pages          |
      | create-story           |
      | delete-story           |
      | edit-story             |
      | create-link            |
      | delete-link            |
      | change-link-path       |
      | create-frame           |
      | delete-frame           |
      | move-frame             |
      | resize-frame           |
      | change-frame-fill      |
      | change-frame-stroke    |
      | change-frame-wrap-mode |
      | change-frame-columns   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When it is parsed, printed back to DSL and parsed again through round_trip_layout_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
