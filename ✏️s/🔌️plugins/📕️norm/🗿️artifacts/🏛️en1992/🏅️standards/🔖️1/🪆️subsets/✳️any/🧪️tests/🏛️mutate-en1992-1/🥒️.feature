@capability-en1992-1-mutate
@oracle-en1992-1-python-independent
@comparison-ordered-json-v1
@mutations-en1992-1-any
Feature: Apply every typed EN 1992 mutation against an independent Python implementation
  `s.norm.en1992` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1992` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1992Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️.py` beside this file is
  it: all 35 kinds of this vocabulary, written in Python from the repository's own written
  specification of what a semantic mutation means — `📓️taxonomy.md`'s verb table, naming mechanics
  ("New-value fields are `new_<field>`") and addressing convention ("Inverse always computed from
  `base`", "Missing target ⇒ `inverse` returns `Vec::new()`"), and `📓️derivation-rules.md`'s shape
  rules — plus this subset's committed catalog for the closed list of kinds. It imports nothing from
  the Rust it judges and transliterates none of it: the document field a `new*` argument names is
  resolved by normalised spelling against the document's own keys, which is what the naming mechanic
  states, never from a table copied out of `🧬️mutations/**` — and the paragraph below names the
  spellings in THIS subset where that resolution can genuinely go wrong. The recorded no-oracle
  decision it replaces is gone from
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`, because there is now a
  reference to compare against.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path below is a declared `asset://` fixture, so neither side holds a transcription that could
  drift. All thirty-five kinds are flat `change-<field>` edits, and this subset's own difficulty is
  PREFIX FAMILIES: ten `change-anchor-*` keys, five `change-liquid-*` keys and two `change-bridge-*`
  keys sit in the same document as the bare section keys they shadow (`change-anchor-as-mm2` beside
  `change-as-mm2`, `change-anchor-d-mm` beside `change-d-mm`). Resolving `new_as_mm2` by normalised
  spelling has to land on the bare key and not on the anchor one, and that is a genuine way for an
  independent reading to go wrong. Each side then asserts the same three laws in role — the applied
  document must BE the committed after-snapshot; an `applied` vector must move the document and a
  `rejected` one must leave it bit-identical; and the mutation followed by its OWN computed inverse
  must restore the before-snapshot exactly. What `parity` adds on top is the only thing a single
  implementation can never provide: that two implementations, in two languages, written from one
  written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the
  restored document repeats the before-document on all thirty-five rows; only the mutated projection
  distinguishes `change-as-mm2` from `change-anchor-as-mm2`, which is exactly the confusion this
  case exists to catch.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/🛢️liquid-retaining-fem-anchor/🖼️assets/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio` — a
  named case that carries the liquid-retaining, FEM and anchor field families at once, which is why
  every prefix family above is present in one document instead of being split across fixtures. It is
  an authored case, not a submitted design. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so identity is
  compared at the envelope preamble, the ordered `key=value` fields and the digest and length of the
  re-emitted bytes, never at an inferred token-to-enum mapping.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                               | dir                               | fixture                                  |
      | change-annex                     | 🌍️change-annex                     | 🌍️switches-annex-to-en                     |
      | change-m-ed-knm                  | ⤴️change-m-ed-knm                  | ⤴️raises-m-ed-knm-to-187-5                 |
      | change-v-ed-kn                   | 🪚️change-v-ed-kn                   | 🪚️raises-v-ed-kn-to-96-5                   |
      | change-f-ck                      | 🪨️change-f-ck                      | 🪨️raises-f-ck-to-45-0                      |
      | change-b-mm                      | ↔️change-b-mm                      | ↔️raises-b-mm-to-375-0                     |
      | change-d-mm                      | ↕️change-d-mm                     | ↕️raises-d-mm-to-512-5                     |
      | change-as-mm2                    | 🪜️change-as-mm2                    | 🪜️raises-a-s-mm2-to-1608-5                 |
      | change-f-yk                      | 🔧️change-f-yk                      | 🔧️raises-f-yk-to-550-0                     |
      | change-rho-l                     | 🧮️change-rho-l                     | 🧮️raises-rho-l-to-0-015625                 |
      | change-n-ed-kn                   | 🏋️change-n-ed-kn                   | 🏋️raises-n-ed-kn-to-62-5                   |
      | change-p-kn                      | 🏹️change-p-kn                      | 🏹️raises-p-kn-to-45-5                      |
      | change-ac-mm2                    | 📐️change-ac-mm2                    | 📐️raises-a-c-mm2-to-168750-0               |
      | change-use-fem                   | 🕸️change-use-fem                   | 🕸️turns-use-fem-on                         |
      | change-span-m                    | 🌉️change-span-m                    | 🌉️raises-span-m-to-7-5                     |
      | change-udl-kn-m                  | ⬇️change-udl-kn-m                  | ⬇️raises-udl-kn-m-to-26-25                 |
      | change-fire-rating               | 🔥️change-fire-rating               | 🔥️switches-fire-rating-to-r120             |
      | change-provided-axis-distance-mm | 📏️change-provided-axis-distance-mm | 📏️raises-provided-axis-distance-mm-to-42-5 |
      | change-bridge-sigma-c-mpa        | 🛣️change-bridge-sigma-c-mpa        | 🌉️raises-bridge-sigma-c-mpa-to-15-75       |
      | change-bridge-delta-sigma-s-mpa  | 🔁️change-bridge-delta-sigma-s-mpa  | 🔁️raises-bridge-delta-sigma-s-mpa-to-132-5 |
      | change-tightness-class           | 💧️change-tightness-class           | 💧️switches-tightness-class-to-tc2          |
      | change-hd-over-h                 | 🌊️change-hd-over-h                 | 🌊️raises-hd-over-h-to-12-5                 |
      | change-liquid-sigma-s-mpa        | 🧲️change-liquid-sigma-s-mpa        | 🧲️raises-liquid-sigma-s-mpa-to-235-5       |
      | change-liquid-rho-p-eff          | 🧷️change-liquid-rho-p-eff          | 🧷️raises-liquid-rho-p-eff-to-0-0078125     |
      | change-liquid-f-ct-eff-mpa       | 🧱️change-liquid-f-ct-eff-mpa       | 🧱️raises-liquid-f-ct-eff-mpa-to-3-25       |
      | change-liquid-es-mpa             | 🌀️change-liquid-es-mpa             | 🌀️raises-liquid-e-s-mpa-to-205000-0        |
      | change-liquid-sr-max-mm          | 🕳️change-liquid-sr-max-mm          | 🕳️raises-liquid-s-r-max-mm-to-312-5        |
      | change-anchor-h-ef-mm            | ⚓️change-anchor-h-ef-mm            | ⚓️raises-anchor-h-ef-mm-to-105-0           |
      | change-anchor-cracked            | 💥️change-anchor-cracked            | 💥️turns-anchor-cracked-on                  |
      | change-anchor-f-uk-mpa           | 💪️change-anchor-f-uk-mpa           | 💪️raises-anchor-f-uk-mpa-to-900-0          |
      | change-anchor-f-yk-mpa           | 🛡️change-anchor-f-yk-mpa           | 🛡️raises-anchor-f-yk-mpa-to-720-0          |
      | change-anchor-as-mm2             | 🔩️change-anchor-as-mm2             | 🔩️raises-anchor-a-s-mm2-to-157-0           |
      | change-anchor-d-mm               | ⭕️change-anchor-d-mm               | ⭕️raises-anchor-d-mm-to-16-0               |
      | change-anchor-c1-mm              | 📍️change-anchor-c1-mm              | 📍️raises-anchor-c1-mm-to-137-5             |
      | change-anchor-n-ed-kn            | 🪝️change-anchor-n-ed-kn            | 🪝️raises-anchor-n-ed-kn-to-22-5            |
      | change-anchor-v-ed-kn            | ✂️change-anchor-v-ed-kn            | ✂️raises-anchor-v-ed-kn-to-11-25           |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                               | dir                               | fixture                                  |
      | change-annex                     | 🌍️change-annex                     | 🌍️switches-annex-to-en                     |
      | change-m-ed-knm                  | ⤴️change-m-ed-knm                  | ⤴️raises-m-ed-knm-to-187-5                 |
      | change-v-ed-kn                   | 🪚️change-v-ed-kn                   | 🪚️raises-v-ed-kn-to-96-5                   |
      | change-f-ck                      | 🪨️change-f-ck                      | 🪨️raises-f-ck-to-45-0                      |
      | change-b-mm                      | ↔️change-b-mm                      | ↔️raises-b-mm-to-375-0                     |
      | change-d-mm                      | ↕️change-d-mm                     | ↕️raises-d-mm-to-512-5                     |
      | change-as-mm2                    | 🪜️change-as-mm2                    | 🪜️raises-a-s-mm2-to-1608-5                 |
      | change-f-yk                      | 🔧️change-f-yk                      | 🔧️raises-f-yk-to-550-0                     |
      | change-rho-l                     | 🧮️change-rho-l                     | 🧮️raises-rho-l-to-0-015625                 |
      | change-n-ed-kn                   | 🏋️change-n-ed-kn                   | 🏋️raises-n-ed-kn-to-62-5                   |
      | change-p-kn                      | 🏹️change-p-kn                      | 🏹️raises-p-kn-to-45-5                      |
      | change-ac-mm2                    | 📐️change-ac-mm2                    | 📐️raises-a-c-mm2-to-168750-0               |
      | change-use-fem                   | 🕸️change-use-fem                   | 🕸️turns-use-fem-on                         |
      | change-span-m                    | 🌉️change-span-m                    | 🌉️raises-span-m-to-7-5                     |
      | change-udl-kn-m                  | ⬇️change-udl-kn-m                  | ⬇️raises-udl-kn-m-to-26-25                 |
      | change-fire-rating               | 🔥️change-fire-rating               | 🔥️switches-fire-rating-to-r120             |
      | change-provided-axis-distance-mm | 📏️change-provided-axis-distance-mm | 📏️raises-provided-axis-distance-mm-to-42-5 |
      | change-bridge-sigma-c-mpa        | 🛣️change-bridge-sigma-c-mpa        | 🌉️raises-bridge-sigma-c-mpa-to-15-75       |
      | change-bridge-delta-sigma-s-mpa  | 🔁️change-bridge-delta-sigma-s-mpa  | 🔁️raises-bridge-delta-sigma-s-mpa-to-132-5 |
      | change-tightness-class           | 💧️change-tightness-class           | 💧️switches-tightness-class-to-tc2          |
      | change-hd-over-h                 | 🌊️change-hd-over-h                 | 🌊️raises-hd-over-h-to-12-5                 |
      | change-liquid-sigma-s-mpa        | 🧲️change-liquid-sigma-s-mpa        | 🧲️raises-liquid-sigma-s-mpa-to-235-5       |
      | change-liquid-rho-p-eff          | 🧷️change-liquid-rho-p-eff          | 🧷️raises-liquid-rho-p-eff-to-0-0078125     |
      | change-liquid-f-ct-eff-mpa       | 🧱️change-liquid-f-ct-eff-mpa       | 🧱️raises-liquid-f-ct-eff-mpa-to-3-25       |
      | change-liquid-es-mpa             | 🌀️change-liquid-es-mpa             | 🌀️raises-liquid-e-s-mpa-to-205000-0        |
      | change-liquid-sr-max-mm          | 🕳️change-liquid-sr-max-mm          | 🕳️raises-liquid-s-r-max-mm-to-312-5        |
      | change-anchor-h-ef-mm            | ⚓️change-anchor-h-ef-mm            | ⚓️raises-anchor-h-ef-mm-to-105-0           |
      | change-anchor-cracked            | 💥️change-anchor-cracked            | 💥️turns-anchor-cracked-on                  |
      | change-anchor-f-uk-mpa           | 💪️change-anchor-f-uk-mpa           | 💪️raises-anchor-f-uk-mpa-to-900-0          |
      | change-anchor-f-yk-mpa           | 🛡️change-anchor-f-yk-mpa           | 🛡️raises-anchor-f-yk-mpa-to-720-0          |
      | change-anchor-as-mm2             | 🔩️change-anchor-as-mm2             | 🔩️raises-anchor-a-s-mm2-to-157-0           |
      | change-anchor-d-mm               | ⭕️change-anchor-d-mm               | ⭕️raises-anchor-d-mm-to-16-0               |
      | change-anchor-c1-mm              | 📍️change-anchor-c1-mm              | 📍️raises-anchor-c1-mm-to-137-5             |
      | change-anchor-n-ed-kn            | 🪝️change-anchor-n-ed-kn            | 🪝️raises-anchor-n-ed-kn-to-22-5            |
      | change-anchor-v-ed-kn            | ✂️change-anchor-v-ed-kn            | ✂️raises-anchor-v-ed-kn-to-11-25           |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1992 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/🛢️liquid-retaining-fem-anchor/🖼️assets/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
