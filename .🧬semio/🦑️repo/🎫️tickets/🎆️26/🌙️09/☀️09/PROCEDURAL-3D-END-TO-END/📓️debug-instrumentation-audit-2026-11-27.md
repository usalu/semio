# 🔎️ `[DEBUG]` instrumentation audit (2026-11-27)

Read-only audit (Haiku) of temporary `[DEBUG]` logging in the generation3d-related trees.

| Subtree | Hits |
|---|---|
| `✏️s/🔌️plugins/🌀️procedural` | 0 |
| `✏️s/🔌️plugins/🌊️flow` | 2 (BRep mesh-workbench test, lines 1080 and 1190) |
| `✏️s/🧑‍💻dev/🧩️composition` | 0 |
| `🧰️framework/🔨️modules/🧵️job` | 2 (`⏱️context/🦀️.rs:730` `dbg_post_take`, `📬️outcome/👷️worker/🦀️.rs:30` worker page reservation) |
| `🧰️framework/🔨️modules/🌱️value` | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer` | 2 (`ShellHost/🛫️spawned-session/🔶️document-services/🟦️.ts:74,77`) |
| `🌎️hub` | 1 (`🧩️compositions/🖍️draw/🧪️tests/🌐️guest/🦀️.rs:317`) |

- The job, renderer and hub hits were modified on 2026-11-26/27 by other agents' in-flight work. They are left for their owners to remove.
- `[DEBUG] archive/replacement wait` and `[DEBUG] preview progress` are not literal strings in source; the latter comes from this ticket's `react-journey-probe.mjs`.
- `[DEBUG] Verified local procedural module` comes from this ticket's `install-local-procedural.ts`.
