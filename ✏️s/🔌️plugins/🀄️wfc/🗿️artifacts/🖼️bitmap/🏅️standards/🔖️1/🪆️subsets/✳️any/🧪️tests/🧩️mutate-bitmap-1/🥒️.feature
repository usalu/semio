@capability-wfc-bitmap-1-mutate
@oracle-wfc-bitmap-python-independent
@comparison-ordered-json-v1
@mutations-wfc-bitmap-1-any
Feature: Apply every typed bitmap mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second
  implementation of the `s.wfc.bitmap` document and all ten typed mutations, written from
  `../../🧬️schema/📸️snapshot/🔣️.json`, the ten per-kind payload schemas under
  `../../🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`, and RFC 4648 §4 for the pixel carrier. It
  imports nothing from this repository and transliterates none of its Rust — its base64 codec is
  written from the RFC, not ported.

  Why a second implementation rather than a third-party library. The published overlapping-model WFC
  implementations compute a COLLAPSE from a PNG and hand back another PNG. None of them carries the
  problem statement as a document, none reads a palette-indexed base64 carrier, and none exposes a
  mutation vocabulary — and the vocabulary is what these vectors specify. The SOLVE itself is
  deliberately NOT a vector here: it is an inference, it is covered by `semio-s-plugin-wfc-engine`'s
  own tests and by this subset's deterministic-seed and contradiction tests, and pinning a seeded
  collapse into a committed fixture would freeze a sampler rather than a specification.

  Four things only the committed vectors state, and both implementations take them from there. The
  pixel buffer is base64 of one palette index per pixel, row-major — never an inline integer array.
  This subset tags its mutations EXTERNALLY: a payload is `{"ChangeSeed": {…}}`, a PascalCase variant
  name as the single key. `add-palette-color` INSERTS at an index and renumbers every pixel and pin
  at or above it, `remove-palette-color` is its exact inverse, and removing a colour that is still
  painted or pinned is FATAL rather than a silent repaint. And `resize-output` CASCADES: a pin
  outside the new extent is removed, announced with an info-level `mutation.cascade`.

  📌️ A DEFECT THE REFERENCE FOUND, which the Rust half alone did not see. `resize-input`'s diff
  originally declared only the two extent fields and let `apply` re-derive the pad/crop layout. The
  reference's footprint law — every field where `after` differs from `before` must be declared by the
  committed diff — reported the pixel buffer as moved but undeclared. The diff now carries the
  re-laid-out buffer explicitly, so a reader can reconstruct `after` from the diff alone instead of
  re-implementing the layout rule.

  ✅️ ALL TEN KINDS ARE ADJUDICATED AND NONE IS REFUSED.

  📌️ ONE CEILING, stated rather than implied: the SUBJECT half does not link this subset's own
  codec — `🦀️.rs` beside this file replays the committed vectors — so today the comparison
  establishes that an independent implementation of the specification computes the committed
  after-snapshots, which is a real check of the vectors, but not yet our codec against a second
  producer.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: The committed <id> vector declares its own kind and moves the document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "shared://🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then the committed mutation payload declares the <id> kind
    And the after-snapshot differs from the before-snapshot, or the committed outcome declares the vector a no-op
    Examples:
      | id                   | vector                                                            |
      | change-seed          | 🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99                   |
      | resize-input         | 📐️resize-input/📐️grows-the-sample-to-6-by-4                      |
      | set-input-pixels     | 🖌️set-input-pixels/🖌️paints-a-2-by-2-block-of-colour-1            |
      | add-palette-color    | 🎨️add-palette-color/🎨️appends-a-third-colour                     |
      | change-palette-color | 🖍️change-palette-color/🖍️recolours-the-second-entry               |
      | remove-palette-color | 🧽️remove-palette-color/🧽️drops-the-unused-third-colour           |
      | resize-output        | 🖼️resize-output/🖼️shrinks-the-output-and-cascades-a-pin           |
      | change-model         | ⚙️change-model/⚙️widens-the-window-to-three                       |
      | pin-pixel            | 📌️pin-pixel/📌️pins-the-origin-cell-to-colour-1                   |
      | unpin-pixel          | 📍️unpin-pixel/📍️releases-the-pinned-origin-cell                  |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: The committed <id> vector changes only what its diff declares
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "shared://🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then every field where the after-snapshot differs from the before-snapshot is declared by the committed diff
    And every field the committed diff declares actually differs
    Examples:
      | id                   | vector                                                            |
      | change-seed          | 🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99                   |
      | resize-input         | 📐️resize-input/📐️grows-the-sample-to-6-by-4                      |
      | set-input-pixels     | 🖌️set-input-pixels/🖌️paints-a-2-by-2-block-of-colour-1            |
      | add-palette-color    | 🎨️add-palette-color/🎨️appends-a-third-colour                     |
      | change-palette-color | 🖍️change-palette-color/🖍️recolours-the-second-entry               |
      | remove-palette-color | 🧽️remove-palette-color/🧽️drops-the-unused-third-colour           |
      | resize-output        | 🖼️resize-output/🖼️shrinks-the-output-and-cascades-a-pin           |
      | change-model         | ⚙️change-model/⚙️widens-the-window-to-three                       |
      | pin-pixel            | 📌️pin-pixel/📌️pins-the-origin-cell-to-colour-1                   |
      | unpin-pixel          | 📍️unpin-pixel/📍️releases-the-pinned-origin-cell                  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the four-by-three two-colour sample
    Given the committed before-snapshot shared://🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/📸️snapshot/⬅️before/🔣️.json
    When its base64 pixel buffer is decoded and re-encoded by an implementation written from RFC 4648 §4
    Then the buffer is byte-identical and its length is exactly the input width times the input height
