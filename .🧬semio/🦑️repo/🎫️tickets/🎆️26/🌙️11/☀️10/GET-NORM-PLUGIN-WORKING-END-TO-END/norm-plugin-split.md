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

## Revision: five plugins

The three-plugin build (run `w6Fy3xX`) still failed: the 10-app `norm` component exceeded the limit. Defined functions per linked rlib (pre-gc, `wasm-rlib-functions.py`) showed:

| Group | Functions |
|---|---|
| stdio-semio crates (pulled in only by din18599 `SemioMembers`) | 678,581 |
| shared framework base | 494,912 |
| `semio_hub_norm` monomorphizations (about 18k per app) | 182,262 |
| one norm family (model, diff, editor, viewer, …) | 70k–117k |

So din18599 is isolated, and no plugin carries more than four families. `norm-split-move.py` moved the families:

| Plugin | Families | Lane |
|---|---|---|
| `norm` | din4108, din16798, iso16757, vdi3805 | din4108 |
| `norm-energy` (`📒️`) | din18599 + stdio-semio | din18599 |
| `norm-eurocode` (`📗️`) | en1990, en1991, en1997, en1998 | en1990 |
| `norm-eurocode-structural` (`📓️`) | en1992, en1993, en1994 | en1992 |
| `norm-eurocode-materials` (`📙️`) | en1995, en1996, en1999 | en1995 |

Remaining risk: `norm-energy` carries about 1.27 M functions before gc (base, stdio-semio, one family). Whether gc brings it under the limit is unknown until it links. The fallback is to drop `SemioMembers` from din18599 (editor and viewer).

## Build-stage findings after the split (2026-11-18)

1. **All five components link.** None exceeds the function limit any more, including `norm-energy`.
2. **Native transpile tool not staged.** Built it with `NATIVE_EXEC_TARGET=<native-transpile project>:build SEMIO_BUILD_MODE=release`.
3. **Stale descriptor emitter.** The tool, built Nov 17 16:43, predated the owned-actor ABI change (Nov 17 19:46) and rejected every component ("no core module implementing the owned Semio actor ABI"). Rebuilt it via the `describe` phase.
4. **Unclassified window actions.** `describe()` trapped on `norm-<family>-viewport:norm.shortcut.*`. Fixed at the shared `view_action` helper (`norm-command-classification.md`).
5. **`norm-energy` was 599 MB, above the emitter's 512 MiB limit.** 312 MB of it is the `name` section, because prototype builds keep `strip = "none"` for every role (`🏗️build/🎚️mode/🦀️profile/🟦️.ts`). Fast fallback: din18599's editor and viewer no longer compose `SemioMembers`, so the stdio-semio closure is dropped. The din18599 apps run, but the climate semio-table child is not opened as a semio member.
   - **Follow-up:** strip symbols for the prototype `component` role, which roughly halves every component (`norm-eurocode-materials` is 458 MB and `norm-eurocode-structural` is 491 MB, close to the limit). Then restore `type Members = semio_s_artifact_stdio_semio::SemioMembers` on din18599. This needs one full unit rebuild.
