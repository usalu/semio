# Plugin TestConfig Direct Leaf — Root Review 38

## Reviewed Corrections

The mounted TestConfig fixture owns one change-test-config-selection leaf with a required nullable selected payload. Its structural diff keeps Identity, Clear and Set distinct through serde. Text encoding now uses a JSON nullable string, distinguishing None from the literal string "null", the empty string and Unicode/newline content.

Root review repaired the nullable field deserializer to use the explicit serde Deserialize UFCS call, avoiding an unimported trait method. Required-nullable decoding rejects a missing field instead of treating it as None. The authored binary decoder uses checked u32-to-usize conversion and compares the remaining slice length without unchecked header-length arithmetic.

The leaf's TestConfig type path previously referred to a nonexistent crate::app reexport. Root added only a cfg(test) pub(crate) reexport from plugin_builder_contract_tests. Runtime Interaction/lifecycle regions were not changed.

## Actual Verification

The initial agent controller checked only fixture-container structure and source words. Root replaced it with an actual payload-schema Ajv gate, independent jsonc-parser versus JSON parser comparison, exact expected values, six malformed payload cases, source mount checks and complete input hashes.

The executed gate passed36of36 checks with stable inputs in 🧪️test-config-selection/🧫️run-fNS4fM. Its retained receipt is 🔣️results.json and the outer log is 🧪️test-config-selection/🧪️root-review-38.log. The receipt explicitly states rustExecuted:false.

Two real Rust tests are source-present but have not run: nullable_selection_serde_text_and_binary_round_trip and structural_config_diff_serde_preserves_identity_clear_and_set. Plugin compilation remains held by the independent runtime lifecycle work and remaining fixture adoptions.

## Exact Leaf Boundary

- Owner: framework OS Plugin/🧪️tests/🧬️test-app-mutations/🎚️config
- Document diff source: cdc7b99c722aa436d7772844f070c601624ca9361f0d7dbc8575e5ee63eb8ed5
- Transparent aggregate: 8d16376a3d97829596d6feabc7f79219b8e4e77d55e47eb95a8425a48a6b7be4
- Direct leaf source: 5a012ee961ad95bb79faccb0acf0a7e2950958bba4f02a90115b3eabbc75a28d
- Descriptor: 228cf88a52bd55121771f2f3218075748ce75a66eef6e86bc0542c2c4f3620ba
- Payload schema: 34be4f6a2423afb996092f008b66852a4d4843d11a23d8c36319d098b50b4ebc

TestMutation count/label staging remains unmounted and incomplete. This result is not a Plugin test pass or global mutation readiness.
