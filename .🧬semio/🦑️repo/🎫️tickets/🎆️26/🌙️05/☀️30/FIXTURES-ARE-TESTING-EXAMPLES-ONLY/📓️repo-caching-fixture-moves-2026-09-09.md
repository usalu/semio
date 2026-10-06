# Caching and Preferences Fixture Relocation

Sixteen existing JSON files were inspected as example inputs/expected outputs, each consumed by its canonical TypeScript case. They moved byte-for-byte to the same semantic owner’s `🧫️fixtures/<case>` directory. Fixture schemas remain at their existing schema owners. Static fixture read/import paths were rebased, including shared CI environment/history consumers. Actual exported consumer tests ran through public Bun/Nx: 14 passed and 1 failed. The failing Trunk lockfile assertion expects the current metadata target to be uncached, but its project declares cache=true; the relocated JSON was found and parsed. Native inventory validation remains pending because it requires the full repository Nx graph. No claim that all 16 consumers passed.

## Preserved Moves

```json
[
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧪️tests/🔏️receipt/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧫️fixtures/🔏️receipt/🔣️.json",
    "sha256": "38fc25ce21a6ded7f99d9427fb04509c261925d2a558055e8fc6e6c6371956c7"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/🕸️wasm/🔣️.json",
    "sha256": "7692534f954bc702d37b2ce4c7dab664149f257e9dc02c7d9cafa809fdb0853a"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/🔒️trunk-lockfile/🔣️.json",
    "sha256": "72a3b32f22edc32294f9c6126f582c75b8e2e94db5a2cee5e0a86c5aff697de5"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🌿️environment/🧪️tests/🌿️workflow-context/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🌿️environment/🧫️fixtures/🌿️workflow-context/🔣️.json",
    "sha256": "d92058d11c6d126b1328efce80f8d9c2e1c1b74aba07dc3f65e5d63623bfa9e4"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🏃️resolve/🧪️tests/🧭️baseline-resolution/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🏃️resolve/🧫️fixtures/🧭️baseline-resolution/🔣️.json",
    "sha256": "18def55ed0583e33f41a954143739b18c474bce36ab0f8c97223bd544cf15052"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧫️fixtures/🧭️baseline-selection/🔣️.json",
    "sha256": "61f018843f9993371b380c0a404b027766a51696b615b2d2f45a33ba8b9a6308"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧪️tests/🚦️baseline-command/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧫️fixtures/🚦️baseline-command/🔣️.json",
    "sha256": "9b1af795776f6c9ff7059cc2556045437de7778bdf784bce90dc656c29ab8b70"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🐙️github/🧪️tests/🐙️workflow-history/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🐙️github/🧫️fixtures/🐙️workflow-history/🔣️.json",
    "sha256": "7bad4ec95dc240ee7d59d0f8dd6abf86127325c36535466dac4aa46ae68bba4e"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/🧪️tests/🕸️coverage/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/🧫️fixtures/🕸️coverage/🔣️.json",
    "sha256": "131352e1f54c05c0c365819c50a5aad31f3c3a2d3427e4edebd57a26baf2402f"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🔏️tool-fingerprint/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧫️fixtures/🔏️tool-fingerprint/🔣️.json",
    "sha256": "e3331388cf41b1b5e2e7ba4a1bdff840f933277b181c1ebcd9ce53ced884114b"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧫️fixtures/🛠️binaryen-toolchain/🔣️.json",
    "sha256": "90ec0e4d9ea226a4a5109670a090428a112f1742481e4d3c267cbf67856b9b0a"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧩️extension-attach/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🧩️extension-attach/🔣️.json",
    "sha256": "943cae8d9f9383440d91e0a9c361fa5f6d175bb3dc175805ea1f8911f2d94a8b"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🐳️devcontainer-context/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🐳️devcontainer-context/🔣️.json",
    "sha256": "6f1ceffed0678c324d1f13079c2dd320db06c31f275e08a754214eb23b191006"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔒️persistent-state/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🔒️persistent-state/🔣️.json",
    "sha256": "4c04f63f8ae13bf431ae444348de367eaf8a6683e6e03afc32b40ba488c416b7"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json",
    "sha256": "d8e16da4190e05053b15626865a75b47182984523f902838b4dda9f40b35d3fa"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧫️fixtures/🔒️playground-preferences/🔣️.json",
    "sha256": "af78af33c67d70f8dabc8d0231e4033ef7cca058833470f46cacbce0b94976b5"
  }
]
```

## Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️repo-caching-fixture-moves-2026-09-09.md",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/🧪️tests/🕸️coverage/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/🧪️tests/🕸️coverage/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/🧫️fixtures/🕸️coverage/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🐳️devcontainer-context/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🐳️devcontainer-context/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔒️persistent-state/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔒️persistent-state/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧩️extension-attach/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧩️extension-attach/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🐳️devcontainer-context/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🔒️persistent-state/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🧩️extension-attach/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧪️tests/🔏️receipt/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧪️tests/🔏️receipt/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧫️fixtures/🔏️receipt/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🔏️tool-fingerprint/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🔏️tool-fingerprint/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧫️fixtures/🔏️tool-fingerprint/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧫️fixtures/🛠️binaryen-toolchain/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🐙️github/🧪️tests/🐙️workflow-history/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🐙️github/🧪️tests/🐙️workflow-history/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🐙️github/🧫️fixtures/🐙️workflow-history/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧪️tests/🚦️baseline-command/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧪️tests/🚦️baseline-command/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧫️fixtures/🚦️baseline-command/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🌿️environment/🧪️tests/🌿️workflow-context/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🌿️environment/🧪️tests/🌿️workflow-context/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🌿️environment/🧫️fixtures/🌿️workflow-context/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🏃️resolve/🧪️tests/🧭️baseline-resolution/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🏃️resolve/🧪️tests/🧭️baseline-resolution/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🏃️resolve/🧫️fixtures/🧭️baseline-resolution/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧫️fixtures/🧭️baseline-selection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/🔒️trunk-lockfile/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/🕸️wasm/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧫️fixtures/🔒️playground-preferences/🔣️.json"
]
```

## Runtime Evidence

The passing cases exercised digest publication, native Binaryen WebAssembly execution, CI environment/resolution/selection/command, bounded GitHub transport, WASM tool identity and archive handling, native shell extension attachment, container context/state/bootstrap, and preferences. Independent oracles included Ajv, stable JSON, lodash, esbuild, YAML, minimatch, JSONC, smol-toml, Cargo/Git and native WebAssembly/Bash. Source-diff review corrected two overbroad basename substitutions (receipt output filename and Binaryen production manifest) before this run.
