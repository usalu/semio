# WP-O3b — Zero-touch hub + browser collaboration

Session 9 / 2026-09-23 (slice O3b).

## Status
- Policy: `authenticated` via schema `autoApply` → `set_authenticated_template` at compose (name-fallback removed). `/instance` shows `autoApply: true`.
- Framework `HubDocumentAuthority` wired in `compose_hub_server`; hub `/scopes/.../document/ws` route restored so session+open-plan grant still drives full `handle_ws` (presence leases + credits). Rebuild4 in flight.
- Mount prove (framework path, pre-restore): **PASS** `o3b-doc-ws-mount3.txt` — 101 + first frame on 7704.
- Collab o3c9k (framework path): sockets sustained, sync **Persisted**; presence empty; edits `action-owner-mismatch`. Evidence under `generated/o3c9k-*`.
- Zero-touch: `serve` already calls `ensureDevLocalHub` (reused 7704). Clean-state prove + `▶️start` still pending. Hand-staged gis2d registry not removed yet (activate-s still on wasm queue).

## Live ports
| hub | **7704** (hold shell 836079 / os-hub 69101) |
| gis2d | **6204** (serve shell 836082) |
| peer | leave 7681 |

## Evidence
- `o3b-hub-rebuild3.txt` exit 0 (document_authority wire)
- `o3b-doc-ws-mount3.txt` PASS
- `o3c9k-collab-scenario.txt` / screenshots / console
- `hub-7704.txt`, `serve-6204.txt`

## Next
1. Finish rebuild4 with hub route restored; restart 7704; remount + o3c9l collab.
2. Zero-touch clean-state prove; remove staged registry when activate works.
3. mv report to audit-collaboration rename path.
