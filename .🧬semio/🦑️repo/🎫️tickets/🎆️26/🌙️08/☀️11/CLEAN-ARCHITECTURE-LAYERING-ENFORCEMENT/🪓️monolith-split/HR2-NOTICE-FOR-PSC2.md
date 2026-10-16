# Notice for the psc2 and rhu2 executors from hr2 (HYGIENE-RULES-2)

State at 18:04: the stdio `🧱️base` and `✉️base` renames were applied at 17:48 and fully reverted at 17:56 (the taxonomy validator requires a subset directory to be `<emoji><subset id>`, so `base` cannot be renamed without renaming the subset id). Nothing under `✏️s/🔌️plugins/🗄️stdio` is renamed. Plan files that hold `🧱️base` or `✉️base` paths are valid again.

Directories that stay renamed (paths changed, references rewritten in the same step):

| Old folder name | New folder name | Where |
| --- | --- | --- |
| `🧱️shared` | `🧱️document-fields` | `✏️s/🔌️plugins/🧱️block/🧬️schema/🧱️shared` and `…/🧪️tests/🧫️fixtures/🧱️shared` |
| `🫀️core` | `🫀️foundation` | `✏️s/🔌️plugins/🌀️procedural/🫀️core` |
| `🫀️core` | `🫀️kernel` | `✏️s/🔌️plugins/📐️cad/⚙️engine/🫀️core` |
| `🔨️geometry-core` | `🔨️geometry-kernel` | gltf 2.0 `…/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-core` |
| `🎵️path-helpers` | `🎵️path-navigation` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎵️path-helpers` |

Also changed in place: 26 `@/lib/*` imports in `🖥️server/🎛️coordinator/**` became relative imports; dependency rows in 10 `package.json` files and the six bun lockfiles; `🔒️dependencies.json` got six new oracle packages.
