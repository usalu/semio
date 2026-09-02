@capability-writer-1-mutate
@oracle-writer-python-independent
@comparison-ordered-json-v1
@mutations-writer-1-any
Feature: Apply every typed writer document mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.writer.writer` document and its four typed mutations, written in
  Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rule 1 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the four committed vectors. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to say that because `s.writer.writer` is persisted through this subset's own
  codecs and no third party reads them, there is no reference to register. The fifteen `📕️norm` and
  nineteen `🧿️semio` references refuted that in this same wave over this same carrier. A third-party
  library was nonetheless declined and the reason is concrete: this document holds NO PROSE. It is a
  handle record — an id, a language id, a URI and a composed child handle into an
  `s.stdio.semio@v1/document` — and nothing outside this repository models an editor document whose
  body is a child artifact addressed by content.

  ✅️ WHAT THIS CASE'S EVIDENCE ACTUALLY COVERS. Three of the four kinds are document-level scalar
  setters and are fully adjudicated, with the reference additionally asserting in role what an
  after-snapshot comparison cannot: that each writes exactly ONE of the five members and never the
  composed child handle.

  🚧️ THREE OF THE NINE SCENARIOS ARE REFUSED BY CLAUSE, and reported rather than worked around.
  First, `edit-text` in both roles. It is the only kind that reaches the document's actual CONTENT,
  and the content is not here: the snapshot carries a child handle, not a body. The committed vector
  pins `{status: applied, messages: [{level: warn, code: mutation.no-op}]}` — the verb decided the new
  text was IDENTICAL to what the child already held — and neither the child's content nor the rule
  that compares them is stated anywhere a second implementation can read. Nor is the other branch: no
  committed vector shows what the handle becomes when the text really does change. Adding one vector
  that carries the child body — the `scene` array the siblings `mutate-playbook-1` and
  `mutate-forms-1` already put in their own doc strings — plus the child-addressing rule, closes it.
  Second, `identity-round-trip`. The committed grammar is the repository-wide PLACEHOLDER: its whole
  body is `payload = OCTET+` and its header production declares `"schema" SP "stdio.json"`, while the
  committed artifact's first line is `semio writer.writer.dsl v1` and its body is four HEX-ENCODED
  scalars plus a `[hex,hex]` child-handle pair. Nothing committed says the values are hex, that the
  pair is `(childId, target)`, or how the second element's
  `<artifactId>!<kind>@<standard>/<subset>` spelling is split. The sibling `mutate-note-1` shows a
  real grammar exists for this family; `📖️playbook`, `📋️forms`, `🌿️vcs` and `🔌️wires` report the
  same gap.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector asset://🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_writer_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id              | vector                                                                |
      | rename-writer   | 🏷️rename-writer/🧪️tests/renames-the-document-to-mission-brief          |
      | change-uri      | 🔗change-uri/🧪️tests/republishes-the-brief-under-a-new-uri            |
      | change-language | 🌐change-language/🧪️tests/switches-the-brief-from-plaintext-to-markdown |
      | edit-text       | ✏️edit-text/🧪️tests/warns-that-the-brief-body-is-unchanged             |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    When <id> is applied and then its own computed inverse is applied through apply_writer_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id              | vector                                                                |
      | rename-writer   | 🏷️rename-writer/🧪️tests/renames-the-document-to-mission-brief          |
      | change-uri      | 🔗change-uri/🧪️tests/republishes-the-brief-under-a-new-uri            |
      | change-language | 🌐change-language/🧪️tests/switches-the-brief-from-plaintext-to-markdown |
      | edit-text       | ✏️edit-text/🧪️tests/warns-that-the-brief-body-is-unchanged             |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed jack document and print it back without losing or copying anything
    Given the real committed artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed to a WriterSnapshot, printed back to `.writer` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
