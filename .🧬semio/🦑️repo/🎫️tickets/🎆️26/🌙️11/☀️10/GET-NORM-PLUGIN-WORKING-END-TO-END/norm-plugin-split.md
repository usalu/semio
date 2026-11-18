# Norm Plugin Split

## Why

After the hub compile fixes (`norm-hub-compile-fix.md`), the norm component (run `wNiaat6`) compiled every unit. The cdylib link then failed in `wasm-component-ld`:

```
failed to encode component: failed to decode world from module: module was not valid:
functions count exceeds limit of 1000000
```

stdio hit the same validator limit (tickets `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS`, `26/09/02/COMPLETE-SEMIO-END-TO-END`) and was resolved by splitting it into extension plugins (`stdio-office`, `stdio-pdf`, …). Norm packed 30 apps (15 standards × editor/viewer) plus `SemioMembers` (stdio-semio) into one component. That is not a flag problem; the component has to be split.

The earlier SIGKILL (exit 137) on `semio_hub_norm` was macOS memory pressure (8.9 GiB swap on a 16 GiB host with concurrent builds), not this limit.

## Split

| Plugin | Package | Crate | Families | Apps | Lane |
|---|---|---|---|---|---|
| `norm` | `semio:norm` | `semio-hub-norm` | din4108, din16798, din18599, iso16757, vdi3805 | 10 | din4108 |
| `norm-eurocode` | `semio:norm-eurocode` | `semio-hub-norm-eurocode` | en1990, en1991, en1997, en1998 (basis, actions, geotechnical, seismic) | 8 | en1990 |
| `norm-eurocode-materials` | `semio:norm-eurocode-materials` | `semio-hub-norm-eurocode-materials` | en1992–en1996, en1999 (concrete, steel, composite, timber, masonry, aluminium) | 12 | en1992 |

Each plugin is standalone: it declares its own artifacts, editor/viewer pairs, activations, `NormShellHooks` and `artifacts.write` request. No `depends_on` is needed, unlike stdio, which shares one host-owned kind registry. Every component stays at 12 apps or fewer, below `stdio-office`, which links with 20.

## Files

- `🌎️hub/🧩️compositions/📕️norm/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`, `🧩️component/📦️packages/🦀️rust/Cargo.toml`, `🧩️component/🔣️.json`
- `🌎️hub/🧩️compositions/📕️norm/🧩️extensions/📗️eurocode/**` (new: hub crate, component crate, project.json, script.ts, seeded descriptor)
- `🌎️hub/🧩️compositions/📕️norm/🧩️extensions/📙️eurocode-materials/**` (new, same)
- `🌎️hub/Cargo.toml` (workspace members)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (deployed module members `📗️norm-eurocode`, `📙️norm-eurocode-materials`; `🏛️` and `🧱️` were already taken)
- `🏢️semio-tech/🎡️play/📋️project.json` (lanes en1990 and en1992 added to the full play, `serve-standards` and `prepare-standards-e2e`)
- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/**` (regenerated: 5 / 4 / 6 playground rows)

The descriptors are seeded from the former whole-norm descriptor (`norm-split-descriptors.py`) because catalog generation requires a present `🔣️.json`. Each component build regenerates its own.

Generators: `norm-split.py`, `norm-split-descriptors.py`. The test and fixture fallout is tracked in `norm-split-fallout.md`.
