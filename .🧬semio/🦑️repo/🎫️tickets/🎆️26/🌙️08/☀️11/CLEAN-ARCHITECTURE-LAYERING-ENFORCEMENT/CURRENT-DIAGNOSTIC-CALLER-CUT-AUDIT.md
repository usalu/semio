# Diagnostic Controlled Caller Cut Audit

Captured before the later caller/fault-byte cut: 1657 complete actual Rust sources containing Diagnostic imports or named conversions; 21 sources mention fault-byte APIs. Exact sources, bytes and SHA-256 are in `🗑️generated/diagnostic-controlled/direct-caller-before.json`. This is a lexical finite source inventory, not a generic implementation closure or assertion of native reachability. Existing source authorship is unknown.

The real Value traits still require uncontrolled `to_value` / `from_value`; these must be retired in a coordinated global trait cut, never satisfied by a panic, compatibility method or controlled-default fallback. The Diagnostic controlled behavior proof is a first source epoch only.

Owned JSON already exists in `semio-framework-pack-json`, which depends only on Value and has no Diagnostic dependency: no cycle would be introduced by a Diagnostic-owned dependency on that package. Its explicit `from_json_str_controlled` / `to_json_string_controlled` functions bind actual Value controls, and JsonMemberPolicy declares duplicate authority. Normal Diagnostic Serde remains on TextSpan and fault bytes, and must be removed after binding owned byte/caller APIs.

## Fault-Byte Direct Sources

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/💾️document/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs`
- `🧰️framework/🔨️modules/⚠️diagnostic/🧪️tests/🔬️fault-describe/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🔀️migrate/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️extension-continuation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️standalone/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs`

## Exact Named Call Sites and Required Refusal Boundary

The captured finite named-call inventory records 73 fault-byte mentions across 21 sources and 20 named conversion mentions across 3 sources; mentions include docs/tests/reexports and are not asserted executable. Complete exact source lines plus hashes: `🗑️generated/diagnostic-controlled/closed-named-call-sites.json`.

Controlled JSON currently returns ValueError(String) and prefixes nested failures with path prose. The original malformed-byte OS fallback must be separated from cancellation, quota refusal and allocation failure by an owned typed error kind, not string matching. Required schema-first coordinated change: Value error kind distinguishes malformed owned input, cancellation, ownership limit, allocation failure and work overflow; `.under` retains the kind; the JSON controlled interfaces preserve it. The actual caller decides whether a transport can return that refusal or requires a separately admitted emergency terminal error. Byte control must be supplied at real transport/job/native owners and obey existing capacity/progress/cancellation, never fabricated through an unlimited, default or always-accepting runtime control.

No caller/byte production cut is mounted before the controlled owner RED/GREEN epochs. All current unchecked Diagnostic ToValue/FromValue implementations and original five native laws remain unchanged during the controlled first cut except test-only registration. This audit declares staging and required next work; it does not claim final API removal.

## Unchecked Named Conversion Precision

The earlier prefix query also matched the controlled suffix. Exact word-boundary filtering leaves 16 unchecked named conversion mentions across two source files: Diagnostic main and original fault-describe tests. The new controlled test accounts for four other named controlled mentions. No additional external unchecked named conversion was found by this finite lexical query; this says nothing about generic FromValue/ToValue calls, derived field obligations, method calls on inferred types, or external consumers. Exact revised original-source lines/hashes: `🗑️generated/diagnostic-controlled/closed-unchecked-named-call-sites.json`. Raster integration tests occur in the byte caller roster only.

## Transport Binding Constraints

Guest host decoding already checks `GUEST_FAULT_MAXIMUM_BYTES = 65_536` before creating a TurnFault. That is the real transport bound; a later control must charge syntax construction, typed construction and lossy fallback within a declared caller admission, rather than only count the borrowed wire length. Guest SDK/plugin and reactor job APIs currently return Fault byte vectors in existing WIT outcomes; their public signatures have no codec-refusal arm or explicit allocation/progress control. A clean coordinated caller cut must thread controls from those actual job/turn owners and define terminal refusal at their schema boundaries. Replacing every call with a local always-accepting closure would hide cancellation and is outside the authorized clean contract. No such adapter is authored.

Public TextSpan Serde derive remains a normal runtime foreign boundary in this first cut. Its removal requires pinned actual containing serializers as well as direct span methods. Normal Diagnostic Cargo Serde/SerdeJson must become dev-only only after the owned byte path and containing callers move.
