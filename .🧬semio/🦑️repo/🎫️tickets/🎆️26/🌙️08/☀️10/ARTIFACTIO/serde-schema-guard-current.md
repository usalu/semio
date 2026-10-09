# Native JSON Type Dependency Regression

The independent current audit found production Puzzle semantic mutation/diff APIs parameterized by third-party JSON Value, plus an unused Note JSON patch helper. New neutral Rust source vectors and an independent tree-sitter oracle require native JSON dependencies to be refused from semantic schema. Explicit test-only schema oracles and literal examples remain admitted. The production policy is intentionally unchanged until this new regression reaches an actual RED.
