# 📌️ Status — closed (2026-08-06)

Repo MCP unavailable; ticket closed by updating `🎫️ticket.json`.

## Done

All three sourcing extension crates live at Shape V2 `📦️packages/🦀️rust` with same-dir `📦️lib.rs`,
`[package.metadata.semio] role = "extension"`, frozen published names + component package ids, and
`sourcing_curate` path deps to the main sourcing package. Old `⚡️implementations` sandwiches under
`🧩️extensions/**` are gone. Domain typology (`BeamsModule`/`SlabsModule`/`WindowsModule`) stays in the
main plugin engine component (extensions are thin `PluginBundle` leaves per TEMPLATE-EXT / Rule B).

## Verification

`DEVELOPER_DIR=/Library/Developer/CommandLineTools` ticket overlays `verify-beams|slabs|windows` with
`verify-shims/` (ui-wgpu compat):

| Crate | check | test |
| --- | --- | --- |
| beams | pass | 1 passed |
| slabs | pass | 1 passed |
| windows | pass | 1 passed |

Root workspace check still blocked on concurrent UI-family core→ui-wgpu path breakage (out of scope).
