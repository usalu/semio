@capability-forms-1-mutate
@oracle-forms-python-independent
@comparison-ordered-json-v1
@mutations-forms-1-any
Feature: Apply every typed form document mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.forms.form` document and all ten typed mutations, written in
  Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`, from rules 1, 2
  and 3 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`,
  and from the ten committed vectors together with the `scene` array each scenario carries in
  its own doc string. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to say that because this artifact is persisted through this subset's own
  codecs and no third party reads them, there is no reference to register. The fifteen `📕️norm` and
  nineteen `🧿️semio` references refuted that in this same wave over this same carrier. A third-party
  library was nonetheless declined and the reason is concrete: XForms, JSON Schema forms and ODK all carry the survey INLINE,
  none of them models a survey whose content is a child artifact addressed by content, and none of them
  reads this carrier.

  📌️ WHAT THIS CASE'S EVIDENCE ACTUALLY COVERS, stated plainly rather than left to be inferred from a
  green row. Nine of the ten committed vectors leave the snapshot BYTE-IDENTICAL,
  because Nine of the ten kinds address steps and blocks that live in the CHILD
  SCENE and not in this document — the snapshot carries only `schema`, `id`, `version`, a title and
  two composed child handles. What those vectors really pin is a DIAGNOSTIC, and the reference DERIVES
  it from the scene the doc string carries — status, code and path — rather than reading it off the
  committed outcome, which is the only way this comparison says anything at all. So this case's
  evidence is ONE applied mutation and Nine diagnostics: no committed vector in it
  exercises a create/delete/move/replace that SUCCEEDS. That is a real gap in the case's fixtures, and the reference
  states it rather than passing over it.

  📌️ A CROSS-CASE DIVERGENCE THE REFERENCE SURFACED, which neither case could see alone.
  `s.playbook.playbook` is the same shape with the same verbs, and the two subsets answer the same
  situation differently: a duplicate step id is a REJECTED `mutation.duplicate-id` here
  (`create-step`) and an APPLIED `mutation.no-op` there (`add-step`); a block added to a step that does
  not exist is `mutation.invariant` here (`create-block`) and `mutation.target-missing` there
  (`add-block`). Neither divergence is stated anywhere. Both are visible only
  because one reference was written against both surfaces, and both are reported rather than absorbed
  into a per-case table.

  📌️ A SIBLING NOTE, because the count of second implementations must not be overstated. This
  reference and `mutate-playbook-1`'s are ONE implementation instantiated twice, differing in the verb
  names, in the diagnostic each situation raises and in the handle members. That the two
  instantiations DISAGREE on two situations is the finding above.

  🚧️ TWO OF THE TWENTY-ONE SCENARIOS ARE REFUSED BY CLAUSE. `inverse-change-form-title`: the
  committed vector ADDS the `title` member to a snapshot that carried none, so undoing it requires
  REMOVING the member, and nothing committed says whether the verb accepts a null argument or what
  removing a title means — a gap the `📖️playbook` sibling does not have, because there `title` is
  always present and nullable. And `identity-round-trip`: the committed grammar
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` describes a
  DIFFERENT DOCUMENT — the generic `family-scene` canvas grammar, `layers { shape { … } }` with
  `id`/`x`/`y`/`fill`/`stroke`/`opacity` fields — while the committed artifact carries a `steps=[ … ]`
  list of nested `blocks=[ … ]`, `options=[ … ]`, `fields=[ … ]`, `params={ … }` and a bare
  `condition { }` block, none of which the grammar mentions. Four more subsets — `📖️playbook`,
  `📏️layout`, `🖍️draw` and `🖨️raster` — carry that same canvas grammar over four equally unrelated
  documents, differing from this one only in their `grammar`, `extension` and `artifact-mark` lines.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️component.json
    And the committed outcome vector asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️component.json
    When <id> is applied through apply_form_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>", "scene": <scene>}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id                      | vector                                                                               | scene                                                                                                                                                              |
      | create-step             | 🌱create-step/🧪️tests/rejects-a-duplicate-step-id                                     | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | delete-step             | 🗑️delete-step/🧪️tests/rejects-deleting-a-step-the-scene-does-not-hold                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | reorder-step            | 🔀reorder-step/🧪️tests/no-ops-when-the-step-already-sits-at-that-index                | [{"id":"step-basics","title":"Basics","blocks":[]},{"id":"step-photos","title":"Photos","blocks":[]},{"id":"step-summary","title":"Summary","blocks":[]}]          |
      | rename-step             | ✏️rename-step/🧪️tests/no-ops-when-the-step-already-carries-that-title                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | change-step-description | 📝change-step-description/🧪️tests/no-ops-when-clearing-an-already-absent-description  | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | create-block            | ➕create-block/🧪️tests/rejects-a-block-for-a-step-that-does-not-exist                 | []                                                                                                                                                                 |
      | delete-block            | ➖delete-block/🧪️tests/rejects-deleting-a-block-missing-from-an-existing-step         | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | move-block-to-step      | 📦move-block-to-step/🧪️tests/no-ops-when-the-block-stays-at-its-index-in-its-own-step | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text"},{"id":"q-visit-date","label":"Visit date","kind":"text"}]}] |
      | replace-block           | 🔁replace-block/🧪️tests/no-ops-when-the-replacement-block-is-identical                | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text","required":true}]}]                                          |
      | change-form-title       | 🏷️change-form-title/🧪️tests/titles-an-untitled-survey                                | []                                                                                                                                                                 |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    When <id> is applied and then its own computed inverse is applied through apply_form_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>", "scene": <scene>}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id                      | vector                                                                               | scene                                                                                                                                                              |
      | create-step             | 🌱create-step/🧪️tests/rejects-a-duplicate-step-id                                     | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | delete-step             | 🗑️delete-step/🧪️tests/rejects-deleting-a-step-the-scene-does-not-hold                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | reorder-step            | 🔀reorder-step/🧪️tests/no-ops-when-the-step-already-sits-at-that-index                | [{"id":"step-basics","title":"Basics","blocks":[]},{"id":"step-photos","title":"Photos","blocks":[]},{"id":"step-summary","title":"Summary","blocks":[]}]          |
      | rename-step             | ✏️rename-step/🧪️tests/no-ops-when-the-step-already-carries-that-title                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | change-step-description | 📝change-step-description/🧪️tests/no-ops-when-clearing-an-already-absent-description  | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | create-block            | ➕create-block/🧪️tests/rejects-a-block-for-a-step-that-does-not-exist                 | []                                                                                                                                                                 |
      | delete-block            | ➖delete-block/🧪️tests/rejects-deleting-a-block-missing-from-an-existing-step         | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | move-block-to-step      | 📦move-block-to-step/🧪️tests/no-ops-when-the-block-stays-at-its-index-in-its-own-step | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text"},{"id":"q-visit-date","label":"Visit date","kind":"text"}]}] |
      | replace-block           | 🔁replace-block/🧪️tests/no-ops-when-the-replacement-block-is-identical                | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text","required":true}]}]                                          |
      | change-form-title       | 🏷️change-form-title/🧪️tests/titles-an-untitled-survey                                | []                                                                                                                                                                 |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed to a FormsSnapshot, printed back to `.forms` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
