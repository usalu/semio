# Late SPR Test Layout

The 37-finding full scan identified 14 nested Rust fixture sources. Three mutation support trees now live under command/testkit `🧫️fixtures` owners. The original case-root module names remain unchanged. Ten direct canonical Rust implementations preserve all 13 inline test functions; each leaf mounts the same `tests` child through an explicit relative path. Descriptor owner paths and include paths follow the moved files. No assertion or mutation behavior was changed.

## Extracted Cases

```json
[
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-four-times/🦀️.rs",
    "count": 2,
    "names": [
      "direct_counter_leaf_contract",
      "plan_nests_two_twice_plans"
    ],
    "bodySha256": "74f944f2185bc65805034f232a709d819fdcc64548438c13faee8a9b3fc51310"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-twice/🦀️.rs",
    "count": 2,
    "names": [
      "direct_counter_leaf_contract",
      "plan_has_two_local_adds"
    ],
    "bodySha256": "e27835a03a37731bbc6e184a5e8e864415a5cd5054dd415104a00864986cf8a3"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-sequence/🦀️.rs",
    "count": 1,
    "names": [
      "direct_counter_leaf_contract"
    ],
    "bodySha256": "103aacb70e255cfcd11f33a5251888c8af2774cca3bda1e728c30fc2bc1adb35"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-then-notify-foreign/🦀️.rs",
    "count": 2,
    "names": [
      "direct_counter_leaf_contract",
      "plan_keeps_local_add_before_foreign_steps"
    ],
    "bodySha256": "2a5ae99d024b1a62e79f66869cb1ff028d1d2259fc6a38d13e02ed6e6a812989"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs",
    "count": 1,
    "names": [
      "direct_counter_leaf_contract"
    ],
    "bodySha256": "69643e85be0e243b87c206180d4a4a7e554c560548d95bcc82a6c3dcf6a0c82d"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-unchecked-counter/🦀️.rs",
    "count": 1,
    "names": [
      "direct_fixture_contract"
    ],
    "bodySha256": "ba30090fdf3feaccf99cf8862e9d67a597a10b8a3f400a34c7c55452b7a1b61c"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-observed-counter/🦀️.rs",
    "count": 1,
    "names": [
      "direct_fixture_contract"
    ],
    "bodySha256": "bf175752b0e490e8f44899328bcff4decb02ec6ebf8c4a7bf1664b323b715f8a"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-missing-counter/🦀️.rs",
    "count": 1,
    "names": [
      "direct_fixture_contract"
    ],
    "bodySha256": "4b095c52d6591917c60b69718326ce912d2524aa4fe0304af83054b9dce2bd65"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs",
    "count": 1,
    "names": [
      "direct_fixture_contract"
    ],
    "bodySha256": "9f2a712afd0b733ba6f9a565c20d0dd2ffebe29b25fb6d7b3f6aab0fdc0b5009"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🦀️.rs",
    "test": "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-rejected-counter/🦀️.rs",
    "count": 1,
    "names": [
      "direct_fixture_contract"
    ],
    "bodySha256": "8f69ac38be8af7dc7a885e448cece8f4ebe7b1eab8289714ebdca1c09af8871e"
  }
]
```

## Exact Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️spr-preservation-preimages-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-late-spr-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻spr-preservation/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🧬️mutations/📛️rename-mini/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🧬️mutations/📛️rename-mini/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🧬️mutations/📛️rename-mini/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-four-times/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-sequence/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-then-notify-foreign/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-twice/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/📔️registry/🧬️mutations/📛️rename-mini/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/📔️registry/🧬️mutations/📛️rename-mini/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/📔️registry/🧬️mutations/📛️rename-mini/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/📔️registry/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-missing-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-observed-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-rejected-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-unchecked-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🦀️.rs"
]
```

The retained Bun/Nx preservation check passed: all 13 test bodies match the captured originals after only include-path relocation; non-test Rust prefixes/tails are unchanged, all 24 descriptor/schema fixtures differ only in owner paths, and 31 affected literal file references resolve. The first actual kernel compilation found one further existing unit-suite include of the moved registry descriptor; that include was repaired without changing its assertion. The repeated actual kernel Cargo test compilation exited 0. Public Bun/Nx then executed all 13 extracted laws from the resulting test binary with exact filters; every law passed. The retained preservation/runtime input completed both source comparison and real execution. A later concurrent edit added one incorrect extra parent segment to the registry include; the coordinator restored its physically validated path and verified that registry consumer directly through public Bun/Nx and the real Cargo kernel test target: `os_spr::command::tests::derive_mutations_wires_complete_leaf_and_atomic_registration` passed, with 1,037 other tests filtered.
