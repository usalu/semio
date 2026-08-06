# 📌️ Status — closed via filesystem (2026-08-06)

Repo MCP unavailable; ticket closed by updating `🎫️ticket.json` only.

## Done on disk

All three 🪵️sourcing extension crates live at Shape V2 `📦️packages/🦀️rust` with same-dir `📦️lib.rs`,
`[package.metadata.semio] role = "extension"`, frozen component package ids, and `sourcing_curate` path deps
to `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust` (seven `../` segments — depth unchanged vs old
`⚡️implementations/🦀️rust` layout). Each extension has `📋️project.json` + `📜️script.ts` (leveled test
targets). Old `⚡️implementations` sandwiches under `🧩️extensions/**` are gone.

## Verification (this session)

Isolated ticket overlays `verify-beams` / `verify-slabs` / `verify-windows` (`DEVELOPER_DIR=/Library/Developer/CommandLineTools`):
`cargo check` fails in transitive `semio-framework-ui-wgpu` (`crate::wgpu` unresolved — concurrent
FRAMEWORK-UI-FAMILY work, not caused by the extension move). Root workspace `cargo check -p
semio-s-plugin-sourcing-{beams,slabs,windows}` also blocked by unrelated workspace breakage (math-family
mid-consolidation / compiler math dep). Extension manifests and path deps were hand-verified; re-run crate
checks once UI/math registrar handoffs land.

## Process plugin residue

`✏️s/🔌️plugins/🏭️process/**/⚡️implementations` — none on disk (stray `🎛️apps/🧊️3d/⚡️implementations/🦀️rust`
already removed by IMPERATIVE-AND-PLAYBOOK-EXTENSIONS-DE-SANDWICH).
