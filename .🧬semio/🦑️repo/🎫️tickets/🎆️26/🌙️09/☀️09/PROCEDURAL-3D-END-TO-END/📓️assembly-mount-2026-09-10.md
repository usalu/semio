# Assembly — codecs, IO, examples and the plugin-root mount (2026-09-10)

Execution lane closing `📓️taxonomy-fix-2026-09-10.md` §5.1's measured packet: the five unsatisfied
codec bounds that keep `🧩️assembly`'s authored editor/viewer out of `ProceduralApps`, plus the
`🚪️io`/`📚️examples` facets the same subset lacks. Raw logs under `🗑️generated/asm-*.txt`.

**restage required: yes** — this lane never ran `activate` on the shared target. The descriptor
regenerated here is staged, not activated.

## 0. Baseline, re-measured today

`bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts check`, exit 1,
2 123 finding lines repo-wide, 22 of them `🌀️procedural:`
(`🗑️generated/asm-tax-before.txt`).

`📓️taxonomy-fix-2026-09-10.md` §5 attributes 18 of the 22 to assembly. Re-measured line by line the
attributable count is **17**, not 18, and the difference is a labelling ambiguity in the checker, not
drift: four of the six `surface "✳️any/…" is missing 🎚️config/🧬️schema/` lines carry no artifact in
their label (`assertAppSchemaOwner` labels a surface by its subset + role only), so they can only be
assigned by inspecting the tree. Two belong to `🌀️generation2d`'s viewer, four to assembly's two
surfaces.

| class | lines | owner |
|---|---|---|
| `is missing 🚪️io/` | 1 | assembly |
| `is missing 📚️examples/` | 1 | assembly |
| leaf not reachable from Cargo manifest — `✏️editor`/`👁️viewer` | 10 | assembly |
| leaf not reachable — `🧪️tests/🧩️mutate-assembly-1/🦀️.rs` | 1 | assembly |
| `surface … is missing 🎚️config/🧬️schema/` + `👥️presence/🧬️schema/` | 4 | assembly |
| **assembly total** | **17** | |
| the same three classes on generation2d/generation3d | 5 | not this lane |

<!-- SECTIONS APPENDED BELOW AS EACH STEP LANDS -->
