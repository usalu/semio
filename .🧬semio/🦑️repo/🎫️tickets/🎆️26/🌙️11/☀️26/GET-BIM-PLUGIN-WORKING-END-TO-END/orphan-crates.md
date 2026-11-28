# 🔍️ Orphan Crates Under the BIM Plugin

Read-only audit of every `Cargo.toml` under `✏️s/🔌️plugins/🏙️bim/` (257 manifests, 256 packages, 1 workspace-only manifest).

## Result

0 true orphans. Every bim package is a member of the bim model workspace root (`🗿️artifacts/🏢️model/Cargo.toml`) except the bridge, which is its own workspace root by design.

## Candidates Examined

| Path | Package | `#[test]` / `#[cfg(test)]` | `📋️build-owners.json` | `.cargo/config.toml` | Verdict |
|---|---|---|---|---|---|
| `✏️s/🔌️plugins/🏙️bim/🏭️bridge/Cargo.toml` | `semio-bim-mutation-bridge` | 0 / 0 | Yes (path and name) | No | Not an orphan: own `[workspace]` root (the manifest comment says this is deliberate, to avoid serialising behind the repo-root lease). Only package outside the bim root. |
| `…/🧬️schema/💡️inferences/🪑️components/📦️packages/🦀️rust` | `semio-s-artifact-bim-model-components` | n/a | Yes (path) | No | Not an orphan: member of bim root (`members`, line 52). |
| `…/✏️editor/🧩️entities/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `semio-s-artifact-bim-model-entities-tests` | n/a | Yes (path and name) | No | Not an orphan: member of bim root (line 21). |
| `…/✏️editor/🧩️entities/🧬️families/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `semio-s-artifact-bim-model-entities-families-tests` | n/a | Yes (path and name) | No | Not an orphan: member of bim root (line 22). |
| `…/✏️editor/🧩️entities/🏘️zoning` | none | n/a | No | No | Not a crate: directory exists but contains no `Cargo.toml`. No `zoning` entity crate exists anywhere under bim. |

## Notes

- Membership was checked against 293 live workspace roots in the repo (the ticket copies under `🧬semio/🎫️tickets/` were excluded). Only the bridge is outside the bim root, and it is its own root.
- If "self-rooted workspace" were not counted as membership, the bridge would be the single orphan. Its own manifest, however, declares it as a workspace root, so it is not an orphan under the stated rule.
- The `zoning` suspect is a missing crate rather than an orphan crate. The `components` and entity-test suspects are all members of the bim root.
- `📋️build-owners.json` and `.cargo/config.toml` were checked by package path and package name; neither references any `zoning` crate.
