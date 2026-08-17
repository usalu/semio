# Wave 0 R1-G Plugin Integration

## Scope

Updated only `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` and this ticket evidence. No stdio source, IO/store, nonstdio plugin, builder, host, or WIT source was edited.

The repository MCP was unavailable in this environment, so `repo://goals` and the ticket lifecycle tools could not be used. The parent assigned the already-active ticket; no ticket status transition was attempted.

## Delivered

- `AppBuilder` derives `AppDefinition::dialect` and `AppDefinition::role` by strictly parsing the authoritative canonical surface app ID. Invalid app IDs fail at the builder boundary; no fallback or default was added.
- `AppDefinition` is explicitly re-exported from the plugin app module for its existing builder consumer.
- The surface authoring documentation is retained as outer documentation on `ArtifactEditor`, resolving the inner-documentation placement error without deleting the documentation.
- `surface_builder_forward!` now expands each complete method set independently for each builder type, avoiding invalid nested macro repetition.
- `OpenArtifact`, `SetDefaultApp`, and `ClearDefaultApp` now validate canonical dialect, role, and app coordinates and relay exact command payloads through `ReplayShellCommand`. Invalid or mismatched coordinates produce typed OS faults; there are no wildcard/no-op branches.
- Added focused tests covering valid opening/default relays and invalid/mismatched coordinate rejection.
- `ArtifactDefinitionError::new` is public. This is the typed construction boundary used by stdio's registry when it maps catalog lookup failures into the public artifact-definition error. It resolves stdio's `E0624` without a stringly untyped escape hatch or compatibility layer.

## Verification

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo check -p semio-framework-plugin` | Passed (exit 0) | `🧪️wave-0-r1-g-plugin-check-final.log` |
| `cargo test -p semio-framework-plugin --lib opening_command_relay_tests` | Passed: 3/3 | `🧪️wave-0-r1-g-plugin-relay-tests.log` |
| stdio fundamental gate | Reached test execution after a successful compile, then the 15-second wrapper budget expired | `🧪️wave-0-r1-g-stdio-fundamental-final.log` |
| focused stdio registry parity, fundamental | Reached test execution after a successful compile, then the 15-second wrapper budget expired | `🧪️wave-0-r1-g-stdio-parity.log` |
| focused stdio registry parity, quick | Reached test execution after a successful compile, then the 30-second wrapper budget expired | `🧪️wave-0-r1-g-stdio-parity-quick.log` |
| focused stdio registry parity, long | Passed: `schema_runtime_capabilities_exactly_match_registered_declarations` (1 passed; 3461 skipped) | `🧪️wave-0-r1-g-stdio-parity-long.log` |

The original fundamental log, `🧪️wave-0-r1-g-stdio-fundamental.log`, records the observed `E0624`: stdio registry attempted to call the previously-private `ArtifactDefinitionError::new`. The final plugin check and long parity gate prove that the public typed constructor resolves it.

All successful gates emitted compiler warnings only; neither the final plugin check nor the long stdio parity gate reported an error.

## Files

- Updated: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- Added: this report and the `🧪️wave-0-r1-g-*` command logs in this ticket folder.
