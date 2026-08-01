# Area emoji paths

Human-facing labels (launch.json, Storybook sidebar, docs headings) use **concatenated** `{areaEmoji}{name}{subEmoji}{subName}` segments. This is separate from repo entity IDs (`🏘️compose📚️js🗃️sketchpad…`).

| Path                                 | Label                  |
| ------------------------------------ | ---------------------- |
| `compose/client/lib/sketchpad`       | 🏘️compose✍️sketchpad   |
| `compose/client/lib/react`           | 🏘️compose⚛️react       |
| `ui/react`                           | 🖱️ui⚛️react            |
| `gis/map`                            | 🌐️gis📍️map             |
| `gis/terrain`                        | 🌐️gis⛰️terrain         |
| `puzzle/2d`                          | 🧩️puzzle🩻️2d           |
| `puzzle/3d`                          | 🧩️puzzle🏙️3d           |
| `puzzle/5d`                          | 🧩️puzzle👯️5d           |
| `infinite/canvas`                     | ♾️infinite✈️canvas      |
| `infinite/world`                     | ♾️infinite🏙️world      |
| `mathematical/graph`                 | 🧮️mathematical⭕️graphs |
| `framework/`                         | 🥅️framework            |
| `cad/`                               | 📐️cad                  |
| `reasoning/mindmap/wires`            | 🧠️reasoning🔗️wires     |
| Technology `compose` (launch prefix) | 🏘️compose              |
| Technology `ui`                      | 🖱️ui                   |
| Technology `repo`                    | 🧰️repo                 |
| Technology `coda`                    | 🔬️coda                 |
| Technology `puzzle`                  | 🧩️puzzle               |
| Technology `infinite`                | ♾️infinite             |
| Technology `gis`                     | 🌐️gis                  |
| Technology `mathematical`            | 🧮️mathematical         |
| Technology `reasoning`               | 🧠️reasoning            |
| `compose/client/ui/vscode`           | 🏘️compose🖱️vscode      |
| `reasoning/mindmap`                  | 🧠️reasoning🗺️mindmap   |

`AGENTS.md` frontmatter: technology roots use `emoji:`; area docs add `path:` plus optional `bundle.emoji` for sub-areas.
