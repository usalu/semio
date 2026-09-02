# ⚠️ `#[value(…)]` gaps vs `#[serde(…)]` — found by conversion agents hitting them for real

| serde feature | `#[value(…)]` status | consequence | found in |
|---|---|---|---|
| `alias = "…"` | **NOT SUPPORTED** | decoder rejects the alias serde accepts — a real read-compat gap | `UiSectionNode.label` (`alias = "title"`) |
| `skip_serializing_if` on an ENUM VARIANT field | **PARSED THEN SILENTLY DROPPED** | field always emitted; encoding silently differs from serde | `CapabilityOwner`, `ContentBlock` (🌉️mcp) |
| `rename_all = "SCREAMING_SNAKE_CASE"` | not supported | must spell renames per-variant | 🌉️mcp |
| `untagged` | no equivalent | must hand-write the impl | `JsonRpcId/Incoming/Outcome`, `WindowLayoutChild/Root` |
| emitted crate path | hardcoded `::semio_framework_os_kernel::…` | **unusable by any crate BELOW os-kernel in the DAG** (Cargo cycle) | 🎭️actor (57 types blocked) |

## Why the silent ones are the dangerous ones
`skip_serializing_if` on an enum-variant field compiles, passes tests, and produces a DIFFERENT wire
shape than serde. Nothing fails. The same class of silence is why this ticket now requires a
differential oracle test (`*_matches_serde_json_byte_for_byte`) rather than a round-trip test alone.

## The DAG constraint is architectural, not cosmetic
`semio-framework-os-kernel` depends on `semio-framework-actor`, so actor cannot depend back to reach
the derive. Two crates already paid for this by hand-writing impls against `protocol::value::`:
`semio-framework-ui-scene` (45 impls / 26 types) and `🔺️mesh-engine` (`MeshData`). A
`#[value(crate = "…")]` override is dispatched to fix it generally.

## Precedent for machine-generated / "do not edit" files
Put the impl in a SIBLING file rather than overriding the marker. Done for `IconName`,
`Locale`/`Terminology`, `🕸️graph`'s 21 generated enums, and `♾️infinite`'s `IconName` (249 variants)
+ `MetabolismIconName` (29).
