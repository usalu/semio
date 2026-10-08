# S Parent and Plugin Deletability Audit 10

Read-only source audit on 2026-10-08. Read prior `BOUNDARY-AUDIT-9-S-DEV-GIS.md`, root AGENTS, S AGENTS, and available direct plugin AGENTS. No production, manifest, fixture, script, AGENTS, Git, or worktree changes. RepoMCP unavailable; no goal or ticket lifecycle asserted. No runtime, build, test, or physical deletion executed. This report advances the finite inventory beyond audit 9; it does not establish atomic repository acceptance.

## Finite inventory

Enumerated all 34 actual direct plugin directories, with no fixed shortened package catalogue. S has Cargo workspace (138 explicit member strings observed) and Bun workspace manifests, but no `✏️s/🟦️.ts` or `✏️s/🦀️.rs`. Direct plugin root source inventory is one TS root (Block), two Rust roots (Block/Stdio), 30 direct TS package manifests, and two direct Rust package manifests. Other defining implementation roots are child artifact/module/contract owners rather than an invented plugin lib. Table paths are relative to each `✏️s/🔌️plugins/<owner>`.

| Owner | Existing defining root sources/manifests |
| --- | --- |
| ✒️writer | `📦️packages/🟦️typescript/package.json` |
| ➗️mathematical | `📦️packages/🟦️typescript/package.json` |
| 🀄️wfc | `📦️packages/🟦️typescript/package.json` |
| 🌀️procedural | `📦️packages/🟦️typescript/package.json` |
| 🌊️flow | `📦️packages/🟦️typescript/package.json` |
| 🌍️gis | `📦️packages/🟦️typescript/package.json` |
| 🌿️vcs | `📦️packages/🟦️typescript/package.json` |
| 🎞️animate | `📦️packages/🟦️typescript/package.json` |
| 🎥️shooting | `📦️packages/🟦️typescript/package.json` |
| 🎪️demonstrator | `📦️packages/🟦️typescript/package.json` |
| 🎬️sequence | `📦️packages/🟦️typescript/package.json` |
| 🏗️fem | `📦️packages/🟦️typescript/package.json` |
| 🏛️architect | `📦️packages/🟦️typescript/package.json` |
| 🏭️process | `📦️packages/🟦️typescript/package.json` |
| 💠️lowpoly | `📦️packages/🟦️typescript/package.json` |
| 💡️reasoning | `📦️packages/🟦️typescript/package.json` |
| 📋️forms | Absent |
| 📏️layout | `📦️packages/🟦️typescript/package.json` |
| 📐️cad | Absent |
| 📕️norm | `📦️packages/🟦️typescript/package.json` |
| 📖️playbook | `📦️packages/🟦️typescript/package.json` |
| 📜️imperative | `📦️packages/🟦️typescript/package.json` |
| 📸️remodel | `📦️packages/🟦️typescript/package.json` |
| 🔋️energy | `📦️packages/🟦️typescript/package.json` |
| 🔱️trinity | `📦️packages/🟦️typescript/package.json` |
| 🕸️dag | `📦️packages/🟦️typescript/package.json` |
| 🖍️draw | `📦️packages/🟦️typescript/package.json` |
| 🖨️raster | `📦️packages/🟦️typescript/package.json` |
| 🗄️stdio | `🦀️.rs`; `📦️packages/🦀️rust/Cargo.toml` |
| 🗒️note | Absent |
| 🧩️puzzle | `📦️packages/🟦️typescript/package.json` |
| 🧱️block | `🟦️.ts`; `🦀️.rs`; `📦️packages/🟦️typescript/package.json`; `📦️packages/🦀️rust/Cargo.toml` |
| 🪐️space | `📦️packages/🟦️typescript/package.json` |
| 🪵️sourcing | `📦️packages/🟦️typescript/package.json` |

TypeScript compiler AST inspected Block root and S dev entry/worker. Block has exactly two export declarations to its own shared schema, one value and one type-only. S dev entry imports GIS presentation, framework boot, Puzzle board factories; worker imports framework host and concrete GIS worker. Rust roots and actual Cargo manifest text were independently inspected. Stdio imports its registry contract, framework plugin, and std; it accepts caller-owned `ContributionRegistry` and `AssemblyOwner` with no selected codec defaults. Block mounts only shared schema and retirement. These are owner-local contracts, not artifacts despite the misleading Stdio contract package name. Norm contract manifest likewise points to its own registry and framework contracts, not concrete norm standards.

## Exact cross-plugin manifest edges

All direct TS package dependencies were parsed as JSON and resolved by actual package names across S/framework package manifests. Below are all resolved sibling plugin dependency occurrences in those manifests; these are eager install/build edges in authored test infrastructure, not demonstrated runtime imports. All resolve to four CAD extension packages. Catalogue metadata cannot itself prove a consumer uses these APIs.

- `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-spatial-shape` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-energy` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/package.json`.
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/package.json` → `@semio-tech/cad-js-module-aec-building-structure` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/package.json`.

Total: 72 resolved sibling occurrences. The generic framework-3d, infinite-world-r3f, and presentation names resolve under `🧰️framework`, not sibling S plugins.

## Oracle bridge inventory

33 direct plugin-owned bridge workspaces exist. Their Cargo dependencies and Rust `AGGREGATES` eagerly name concrete artifacts. The caller is `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts:18-30`, `mutationBridgeFor`: it searches the requested owner then ancestors for `🏭️bridge/📜️script.ts`, before selecting explicit providers. `runProbe` invokes the selected bridge at line 60. Bridge scripts run sibling Rust inventory binaries. These are real linked executable dependencies, but their purpose is oracle coverage of concrete dispatch, not application installation. Parent location becomes a deletability concern if a surviving artifact's required oracle still compiles a removed sibling. Do not count them as plugin runtime reexports.

Exact concrete dependency rows in every observed bridge manifest (relative to plugin owner):

- `✏️s/🔌️plugins/✒️writer/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-writer-writer = { path = "../🗿️artifacts/✒️writer/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-mathematical-equation = { path = "../🗿️artifacts/➗️equation/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-wfc-2d = { path = "../🗿️artifacts/◻️2d/📦️packages/🦀️rust", features = ["component-app-assembly"] }`; line 22: `semio-s-artifact-wfc-3d = { path = "../🗿️artifacts/🧊️3d/📦️packages/🦀️rust", features = ["component-app-assembly"] }`; line 23: `semio-s-artifact-wfc-bitmap = { path = "../🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust", features = ["component-app-assembly"] }`; line 24: `semio-s-artifact-wfc-grid2d = { path = "../🗿️artifacts/🔲️grid2d/📦️packages/🦀️rust" }`; line 25: `semio-s-artifact-wfc-grid3d = { path = "../🗿️artifacts/🧱️grid3d/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-procedural-generation2d = { path = "../🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust", features = ["component-app-assembly"] }`; line 22: `semio-s-artifact-procedural-generation3d = { path = "../🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust", features = ["component-app-assembly"] }`.
- `✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-flow-flow = { path = "../🗿️artifacts/🌊️flow/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🌍️gis/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-gis-gismap = { path = "../🗿️artifacts/🗺️gismap/📦️packages/🦀️rust", features = ["component-app-assembly"] }`; line 22: `semio-s-artifact-gis-gisterrain = { path = "../🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust", features = ["component-app-assembly"] }`.
- `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-vcs-vcs = { path = "../🗿️artifacts/🌿️vcs/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🎞️animate/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-animate-presentation = { path = "../🗿️artifacts/🎬️presentation/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-shooting-shooting = { path = "../🗿️artifacts/🎥️shooting/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-demonstrator-playground = { path = "../🗿️artifacts/🎪️playground/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-sequence-sequence = { path = "../🗿️artifacts/🎬️sequence/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🏗️fem/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-fem-2d = { path = "../🗿️artifacts/◻️2d/📦️packages/🦀️rust", features = ["component-app-assembly"] }`; line 22: `semio-s-artifact-fem-3d = { path = "../🗿️artifacts/🧊️3d/📦️packages/🦀️rust", features = ["component-app-assembly"] }`.
- `✏️s/🔌️plugins/🏛️architect/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-architect-program = { path = "../🗿️artifacts/🏛️program/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🏭️process/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-process-process3d = { path = "../🗿️artifacts/🧊️process3d/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-lowpoly-lowpoly = { path = "../🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-reasoning-wires = { path = "../🗿️artifacts/🔌️wires/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/📋️forms/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-forms-forms = { path = "../🗿️artifacts/📋️forms/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-layout-layout = { path = "../🗿️artifacts/📏️layout/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/📐️cad/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-cad-cad = { path = "../🗿️artifacts/📐️cad/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-norm-din16798 = { path = "../🗿️artifacts/🌬️din16798/📦️packages/🦀️rust" }`; line 22: `semio-s-artifact-norm-din18599 = { path = "../🗿️artifacts/⚡️din18599/📦️packages/🦀️rust" }`; line 23: `semio-s-artifact-norm-din4108 = { path = "../🗿️artifacts/🧱️din4108/📦️packages/🦀️rust" }`; line 24: `semio-s-artifact-norm-en1990 = { path = "../🗿️artifacts/⚖️en1990/📦️packages/🦀️rust" }`; line 25: `semio-s-artifact-norm-en1991 = { path = "../🗿️artifacts/🏋️en1991/📦️packages/🦀️rust" }`; line 26: `semio-s-artifact-norm-en1992 = { path = "../🗿️artifacts/🏛️en1992/📦️packages/🦀️rust" }`; line 27: `semio-s-artifact-norm-en1993 = { path = "../🗿️artifacts/🔩️en1993/📦️packages/🦀️rust" }`; line 28: `semio-s-artifact-norm-en1994 = { path = "../🗿️artifacts/🧩️en1994/📦️packages/🦀️rust" }`; line 29: `semio-s-artifact-norm-en1995 = { path = "../🗿️artifacts/🪵️en1995/📦️packages/🦀️rust" }`; line 30: `semio-s-artifact-norm-en1996 = { path = "../🗿️artifacts/🪨️en1996/📦️packages/🦀️rust" }`; line 31: `semio-s-artifact-norm-en1997 = { path = "../🗿️artifacts/🌍️en1997/📦️packages/🦀️rust" }`; line 32: `semio-s-artifact-norm-en1998 = { path = "../🗿️artifacts/🫨️en1998/📦️packages/🦀️rust" }`; line 33: `semio-s-artifact-norm-en1999 = { path = "../🗿️artifacts/🪶️en1999/📦️packages/🦀️rust" }`; line 34: `semio-s-artifact-norm-iso16757 = { path = "../🗿️artifacts/📇️iso16757/📦️packages/🦀️rust" }`; line 35: `semio-s-artifact-norm-vdi3805 = { path = "../🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/📖️playbook/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-playbook-playbook = { path = "../🗿️artifacts/📖️playbook/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/📜️imperative/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-imperative-procedure = { path = "../🗿️artifacts/📜️procedure/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/📸️remodel/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-remodel-remodeling = { path = "../🗿️artifacts/📸️remodeling/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🔋️energy/🏭️bridge/Cargo.toml`: line 22: `semio-s-artifact-energy-model = { path = "../🗿️artifacts/🔋️model/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/Cargo.toml`: line 24: `semio-s-artifact-trinity-jack = { path = "../🗿️artifacts/🔌️jack/📦️packages/🦀️rust", features = ["component-app-assembly"] }`; line 25: `semio-s-artifact-trinity-rewriting = { path = "../🗿️artifacts/♻️rewriting/📦️packages/🦀️rust", features = ["component-app-assembly"] }`.
- `✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-dag-dag = { path = "../🗿️artifacts/🕸️dag/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🖍️draw/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-draw-drawing = { path = "../🗿️artifacts/🖍️drawing/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🖨️raster/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-raster-raster = { path = "../🗿️artifacts/🖨️raster/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🗒️note/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-note-note = { path = "../🗿️artifacts/🗒️note/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-puzzle-2d = { path = "../🗿️artifacts/◻️2d/📦️packages/🦀️rust" }`; line 22: `semio-s-artifact-puzzle-3d = { path = "../🗿️artifacts/🧊️3d/📦️packages/🦀️rust" }`; line 23: `semio-s-artifact-puzzle-5d = { path = "../🗿️artifacts/🖐️5d/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🧱️block/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-block-2d = { path = "../🗿️artifacts/◻️2d/📦️packages/🦀️rust" }`; line 22: `semio-s-artifact-block-3d = { path = "../🗿️artifacts/🧊️3d/📦️packages/🦀️rust", features = ["component-app-assembly"] }`; line 23: `semio-s-artifact-block-5d = { path = "../🗿️artifacts/🖐️5d/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🪐️space/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-space-home = { path = "../🗿️artifacts/🏠️home/📦️packages/🦀️rust" }`; line 22: `semio-s-artifact-space-space = { path = "../🗿️artifacts/🪐️space/📦️packages/🦀️rust" }`.
- `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/Cargo.toml`: line 21: `semio-s-artifact-sourcing-curation = { path = "../🗿️artifacts/🗂️curation/📦️packages/🦀️rust" }`.

## Three highest-value execution slices

1. **Generic S workspace ownership.** `✏️s/Cargo.toml` `[workspace].members` enumerates concrete plugin artifacts/extensions and `[workspace.dependencies]` carries their aliases; child manifests inherit `workspace = "../../../.."` or deeper. `✏️s/package.json` includes `**` and `../🧰️framework/**` rather than a concrete eager plugin roster. Original law: S survives deleting any plugin. Cargo's explicit catalogue obstructs physical omission independently of neutral runtime behavior. Cut a generic S workspace containing only actual S-neutral owners/contracts; child plugin/artifact workspaces own concrete members, dependencies, targets and feature forwarding. Put optional authored S deployments in independently owned composition workspaces. Do not make generic owner code rewrite a known roster on deletion, nor retain optional missing path dependencies. This is an ownership proposal, not an assertion a generic S root currently exists.
2. **S dev installed inventory.** `✏️s/🧑‍💻dev/🚀️entry/🟦️.ts:1,3-4` fixes GIS and Puzzle; worker `🧩️service-composition/👷️worker/🟦️.ts:2` fixes GIS; `💡️services/🦀️.rs:6` returns GIS contribution; native `💡️services/⌨️entrypoint/🦀️.rs` consumes it; MCP `💡️services/🌉️mcp/⌨️entrypoint/🦀️.rs:2` invokes fixed GIS protocol. Services Cargo manifest line 27 is unconditional GIS dependency and line 43 forwards its MCP feature. Original law: S survives plugin deletion and plugin survives artifact deletion. Existing comments call this outward application assembly: if intentionally specific, retain it under its specific deployment owner and add a distinct neutral S application consuming caller-owned framework inventories. Generic installation must accept empty inventories and preserve cancellation/disposal; child deployment owns concrete factory types. This repeats audit 9's unresolved finding with AST confirmation, not a new fix.
3. **Plugin oracle ownership.** WFC `🏭️bridge/Cargo.toml` links all five artifact crates, while `🏭️bridge/🦀️.rs` builds fixed typed `AGGREGATES`. Caller `mutationBridgeFor` ancestor lookup chooses this single plugin bridge even when auditing one surviving artifact. Original law: plugin survives artifact deletion. Move artifact-specific dispatch oracle composition to that artifact's owner, or have an explicit deployment/provider own a selected oracle bridge; surviving owner tests cannot compile absent artifacts. Keep plugin-neutral shared solver/contracts independently buildable. Distinguish this test-inventory deletion failure from a runtime plugin dependency; execute a surviving WFC artifact oracle after physically omitting one sibling before claiming acceptance.

## Concrete next validation

Schema-first language-neutral deletion cases should define generic empty S, one synthetic plugin, two independent plugins, removed artifact, duplicate contribution identity, disposal and cancellation. TypeScript/Rust must consume identical cases with third-party schema validation behind test-only infrastructure. An isolated ticket-generated projection (no worktrees, no shared source deletion) must omit one GIS plugin then one WFC artifact separately, preserve neutral bytes and build neutral S plus surviving child-owned oracle through existing bun/nx tasks. Observe runtime with temporary `[DEBUG] ` logs before claiming shell or service survival. No legacy loaders, default catalogues, feature aliases, adapters or compatibility paths should be introduced.

## Focal source hashes

SHA-256 snapshot identifies this read-only observation, not a lock against peer advancement. Reinspect changed files before execution. Hashes include every finite direct-root defining file and every bridge source/manifest/script plus S/deployment callers below.

- `✏️s/Cargo.toml`: `dd551e7777b9c328f63d405c5514f56e15813930f8f8e4431213c4c68a31ab5b`.
- `✏️s/package.json`: `6e25a763521b3df1f5821fcec1357fa2a18839f79bd3490e005c1290c6e3c41f`.
- `✏️s/🔌️plugins/✒️writer/🏭️bridge/Cargo.toml`: `28c996314d3fd18572913eb476e9790aed27bc3fe5caaebad61147f8f58a11da`.
- `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`: `eb15144e0ae221dbb1b06d09023b05684e01743e2eee96f152528daf4438757e`.
- `✏️s/🔌️plugins/✒️writer/🏭️bridge/🦀️.rs`: `49edc8dff56beb1c868702a9fc7b340ba293da69f099bf1307378cb50b4252c4`.
- `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/package.json`: `3fe3de75e275230da097f6af07105d120fe264b93c4eff71b9e1fbf7ccf30d7a`.
- `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/Cargo.toml`: `184d0c794132b4a2efcbf21993ab9caeefbb244847bfc5699695f14f92a6f2d5`.
- `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`: `92edb1c904c5eb48d3a82406727e6dec56a008fb50d33c9a7a592989c170b7f8`.
- `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/🦀️.rs`: `8c3bf1bbaa59aa4c677acc2d5fd136b7998d6ce8b54f9b9c8973fc55d0c7471d`.
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript/package.json`: `b51496217957f01b54507e3c5605cdec9117d29102d1c9118f0fc488aadda4d2`.
- `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/Cargo.toml`: `e090bf87aef7a1e354d9daf2cfa9a3313aa7cd2b4b35906e6f6e5d60bbf6a9d7`.
- `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`: `96d0f8063b733a9b387a5a9d4e6d10e3e84cbaa078ca299f49a80746dea8eb46`.
- `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/🦀️.rs`: `4b36d20753787128808e09f8471722d87eb97c1cc053780ec144ae611da04e7f`.
- `✏️s/🔌️plugins/🀄️wfc/📦️packages/🟦️typescript/package.json`: `2de744178c21140c420bc1f4f3ee52cb904023a10fe7b8e8a561b3538a29f205`.
- `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.toml`: `6689abb594f12dd97fcba0dedc1649da40908b004c37e52d6cbaa11b0b38325a`.
- `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`: `ba74d0137f9307de66f2246c05e647cde74a876a6e8c4ee2382d7796f25417a4`.
- `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/🦀️.rs`: `41468d49510371462279897dc9c0c24ead796c4e906fca249edd3f095e672712`.
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/package.json`: `e9ca3f75ddd36bc8dec57d4bf93d3f720bc68f2d9398758853d48e87bc52a9dd`.
- `✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.toml`: `6ee3587f0a91c0be63bfcbc8dcb1af7cf857f3ae3fb2af53a2f09842ab734638`.
- `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`: `bbcb9e7f3bffc149939d572a526fa30f427d5bee4891e7afc989e8ec1a3f93bd`.
- `✏️s/🔌️plugins/🌊️flow/🏭️bridge/🦀️.rs`: `8a2ac60d78c8be08d6013770d0e1f660a1d066187e476657d851785608b271f3`.
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/package.json`: `8a7f698b5a0b486e19b8f671e5ffd38660d6f3fdba40d01e750fe53ace3df3c2`.
- `✏️s/🔌️plugins/🌍️gis/🏭️bridge/Cargo.toml`: `7c422bd9935ed51baa20b71cc5bb5bd48535ac683440147565f57af0b12c60e9`.
- `✏️s/🔌️plugins/🌍️gis/🏭️bridge/📜️script.ts`: `fdf1597b12c8da2f3a822d6e8a79616b6922bff32b3500ccb28ce9713e579ec4`.
- `✏️s/🔌️plugins/🌍️gis/🏭️bridge/🦀️.rs`: `483e3ce849dd3a92094e919e45280e800337f52998cd9bfe2782ff5210e462fb`.
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/package.json`: `64d59ee03a46ae522467ae03a57b96f1fd7c15838d1129127227df100ab2bbb0`.
- `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/Cargo.toml`: `6cc070c4ee5adc9af9f069fea0bd1a052990d7599f9fb48a21e97f5604a04f2c`.
- `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/📜️script.ts`: `942970ac385440db6f5b5e528fbf1358a95e3e0dec9795938ca3aa177ff88d36`.
- `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/🦀️.rs`: `11a7bb6bd9d31692babb346450f64e2decd6d4996b50aaee1e064eea446f1ee4`.
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript/package.json`: `a91a1c5ad8770152dc8f8875690c1c276edfb3f5d44ef9485c9a3c8e108f78ce`.
- `✏️s/🔌️plugins/🎞️animate/🏭️bridge/Cargo.toml`: `b652d584b72e80061936ada5b67532df86944e8deb55c548fe689393218abca7`.
- `✏️s/🔌️plugins/🎞️animate/🏭️bridge/📜️script.ts`: `8b0c9127206eec2d38b8dcb647dfee0794bf37a36df818abea8cd437724f5487`.
- `✏️s/🔌️plugins/🎞️animate/🏭️bridge/🦀️.rs`: `bafd3fbe19c72306bdcfe4a79b00697c2857dcb7330818c47f3d6ecf291c3c46`.
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json`: `4d171bb53e37b002454c3705773909de3a1a5cc1cf2d5a8b24bcc9e2953863a6`.
- `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/Cargo.toml`: `6eb496b9015ed9b27d11ac361dcdaf2f38e23e6cfdf6f715822022ff0cddde85`.
- `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/📜️script.ts`: `28415b551036fb2a012544cfbffaffc649d8c7033b7a58ebceb9a8cab0a2eded`.
- `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/🦀️.rs`: `c35d45fc47cf27c042f39d1a819645e526d214db663c9a7a0ba75757bedbf7ab`.
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript/package.json`: `5b0ebd78222787cac9d03dcf949935b766dfb3b1ec4ac18e7d382a6a211f615a`.
- `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/Cargo.toml`: `53e1caeea01b229e7145a0483a5b3a77d07ea2f063684605f85c43fdb4baa51f`.
- `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/📜️script.ts`: `ddd4e174be2eda158313db9522e951e885b0099d9bd839b22417d8b84624a3f4`.
- `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/🦀️.rs`: `86501427269193a424ec6aa0a7f372e58528e89d6192afbce3651463b912e17c`.
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript/package.json`: `25d498a74081078e67116fda078b395ed853e52f466a01ccc61d6e22a01b640c`.
- `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.toml`: `477ed7dfa768f660257c19f88a72fadb485fea8593e505d8f9ea3a281d41e85d`.
- `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/📜️script.ts`: `bae46f543427f35d129a487999bbb0f9f5ae7b97a8975986e8f950120fb6cd4d`.
- `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/🦀️.rs`: `18d20e9b0f1b292b40e4278652fe55c51593523b60973d8ee27201f1347f2807`.
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/package.json`: `78c8db132a9fcf57b35dfe4e7522ee52eeea86d3e9d193ce05c1a6af75f470d7`.
- `✏️s/🔌️plugins/🏗️fem/🏭️bridge/Cargo.toml`: `28d13fb1bb19423479cb2b5e3cff89431215ab2a949928f1be22f7cc60ac5de7`.
- `✏️s/🔌️plugins/🏗️fem/🏭️bridge/📜️script.ts`: `95cca6ae41b10fe1966d5453f3928768dde1285d86169e3857c9d400285ed77b`.
- `✏️s/🔌️plugins/🏗️fem/🏭️bridge/🦀️.rs`: `2be6ee47697550792c44fcba0f4ea07b3276498c7fe107629feb495b35225e29`.
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/package.json`: `1fa2805bceab82e4859598062fd41f1130c5790d5731430cf6afcc762112abda`.
- `✏️s/🔌️plugins/🏛️architect/🏭️bridge/Cargo.toml`: `80bdd0074bc2dbdae12e490e9f94ea69fc688d148f7e3198ad8c302b044ffb79`.
- `✏️s/🔌️plugins/🏛️architect/🏭️bridge/📜️script.ts`: `0f0a60c22618215c8caff428f5fd2db01126824d1bf9b84da9b36caf5d687056`.
- `✏️s/🔌️plugins/🏛️architect/🏭️bridge/🦀️.rs`: `b3839b8c89cbba3c801fe9e7f834592586424f9191b1d1fb3fe5255d2ac16200`.
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/package.json`: `3ed91e3632da98f9ebc8a8c51e1b728e0f5f1c68cb23425d0f7e6303b22b622e`.
- `✏️s/🔌️plugins/🏭️process/🏭️bridge/Cargo.toml`: `a459080d03963b999f6562265b3fbdf6f4ca824dc5f135be53beccde9c091587`.
- `✏️s/🔌️plugins/🏭️process/🏭️bridge/📜️script.ts`: `339de809026ddaff97953f768df41ce2f94fb9539a5c1d64063bb5098808d758`.
- `✏️s/🔌️plugins/🏭️process/🏭️bridge/🦀️.rs`: `122ae446fbeb741c4213240b7a52449f13ef274b7fff87b4417d3e8b3d2e2212`.
- `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/package.json`: `5f75b5bafe21be301f8079d507091b4609f1db9808914ba77529e87db71ad4cc`.
- `✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/Cargo.toml`: `04d895b2065f6c753c98db9fb2bfe90f23576087017f5b6c7dc6dea2175333f0`.
- `✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/📜️script.ts`: `cf9f1f305ada70863f726556defd9205bab0f5a86d8636f35e9fe5c3d60ca94f`.
- `✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/🦀️.rs`: `5b61f795eefbbe10b4b3092cd4d134ad9015b6379026aae109f31eab35eed73a`.
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/package.json`: `664072808d8e9ae128c1b84f26dddbba42f64c9d0227d92064bf105888a19b27`.
- `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/Cargo.toml`: `6b147ed02d9a11d2ed553763d5c29dd871f6067b110460bba6f26d0c68fb357a`.
- `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/📜️script.ts`: `301d6a0b57468500cebf963a8d0302e7b2f3a4fc28dca59a72fb1773a6d6c2ba`.
- `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/🦀️.rs`: `3f325644b7db0206742f682b2923f43309ccdd1bc56774a6e826cd72f66e9798`.
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/package.json`: `e61b6f824b588f627fb1b0f0a04c277b9dcad0cd2d1951acb724ffdfe07cf4ef`.
- `✏️s/🔌️plugins/📋️forms/🏭️bridge/Cargo.toml`: `60ea1c95739e704bf23b4487a394fe58bc476a1a7f247bfd6da64c59e1904838`.
- `✏️s/🔌️plugins/📋️forms/🏭️bridge/📜️script.ts`: `bb52463c751eebd0b284d857eadf36495952eaee75323f185296f265ae06b124`.
- `✏️s/🔌️plugins/📋️forms/🏭️bridge/🦀️.rs`: `34a1d01f12c2b3cbf99ed8401d79788d071fa8ae95204cf5f68cdb4690390c3f`.
- `✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.toml`: `462ee9d352856ccded59ca70a58796f9fa189f31cd2f18907d771ad6d8ca3cb8`.
- `✏️s/🔌️plugins/📏️layout/🏭️bridge/📜️script.ts`: `0653127622e6aa34435de6432cd7000ed821ee4fa8f9c4a75f2030f876c976cc`.
- `✏️s/🔌️plugins/📏️layout/🏭️bridge/🦀️.rs`: `8d2465bf2c5d814b809a9b6091caefdde36c9d095080c2706289036639e543fa`.
- `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/package.json`: `0f87f559763fa03029a34fefad27be5cba8640264f16434c3981524fa53f3210`.
- `✏️s/🔌️plugins/📐️cad/🏭️bridge/Cargo.toml`: `618aaecadcb53de335753bd673aedfce02080093a0b84c479d959867beb2c988`.
- `✏️s/🔌️plugins/📐️cad/🏭️bridge/📜️script.ts`: `47e0a9ebee95014b2f990f4b3f6373279981abe7a07909e470eac76bc17a3672`.
- `✏️s/🔌️plugins/📐️cad/🏭️bridge/🦀️.rs`: `7de40814246c852a683bb499501c452e26d32ca0432e31f04d3fb8ce5fa31d8d`.
- `✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.toml`: `6720ac5918f1f59000ca2c90b9e4f1b924e77f30b4db2f2866e04fa74b57fff3`.
- `✏️s/🔌️plugins/📕️norm/🏭️bridge/📜️script.ts`: `e84aeefc9f3b96884ee6468649414bc3e726547a48ed4169c4cbfd96a59780c5`.
- `✏️s/🔌️plugins/📕️norm/🏭️bridge/🦀️.rs`: `ad0ce9e0137092cc11e61cc6f5d5cc6a997ee39057d6bc79334ef330fe51d108`.
- `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/package.json`: `f059428614ed61ab3cb2a171d96f2e25a5b63a868a22a1d7cdc728c622870c9d`.
- `✏️s/🔌️plugins/📖️playbook/🏭️bridge/Cargo.toml`: `0c7c1bee2ca18ebf43e7ef1c6a43ad458fdcbdc6b35ef74bec35a49bdc6ec856`.
- `✏️s/🔌️plugins/📖️playbook/🏭️bridge/📜️script.ts`: `17b0d6ecadfc2c73d0c0f8415ae1125c153b7890bd6aefd84d91edf12241db4d`.
- `✏️s/🔌️plugins/📖️playbook/🏭️bridge/🦀️.rs`: `8d6191c8d38489d4c84e30018bbc652670d25992963bdbca8a3d716a33188cf9`.
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/package.json`: `7426cbab3bd51b1df06bb7307fd3862d5226ae9906818e6582ab82d7f298bb22`.
- `✏️s/🔌️plugins/📜️imperative/🏭️bridge/Cargo.toml`: `e64439ae6343ea84d5efb9d53e79c16804d1638d0563837949ea1fc2b349a844`.
- `✏️s/🔌️plugins/📜️imperative/🏭️bridge/📜️script.ts`: `af15585670b7157e9cadd33d33a7b3f37965d322eeedd5e6538d5f6d983a739d`.
- `✏️s/🔌️plugins/📜️imperative/🏭️bridge/🦀️.rs`: `88f7582195ff58841e00133b6bacdc0a822fbd72ecbc29a669453a43d4dc8852`.
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/package.json`: `195ca17f25d7de7a28243179558097cf8e3bf3761a367d866e894e10bef34a61`.
- `✏️s/🔌️plugins/📸️remodel/🏭️bridge/Cargo.toml`: `fb896276cd70ee84dd20d37bb14c6d6c075cd130e40839831fab2fed1b35348e`.
- `✏️s/🔌️plugins/📸️remodel/🏭️bridge/📜️script.ts`: `d391f6b368edf7b1d27b2b18dd8638c75b105ce81a2f9db2d336391e20fde015`.
- `✏️s/🔌️plugins/📸️remodel/🏭️bridge/🦀️.rs`: `57d41a8618aafbe75fa336036a1c7693beda15674bf2cb210fb4e9d027857013`.
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/package.json`: `7ceb6932cde3a487e1bdfbe81ca4e0fb7d14e6626ebe07acfd728fff9de67a7b`.
- `✏️s/🔌️plugins/🔋️energy/🏭️bridge/Cargo.toml`: `007283e47a6ce32443a6fa1a405e73dc312a8c16dff99d62b423b40d5834936f`.
- `✏️s/🔌️plugins/🔋️energy/🏭️bridge/📜️script.ts`: `5c36e364b20713bc3e1370d794239df868da671d1206cdc2cdccfa699fd64d18`.
- `✏️s/🔌️plugins/🔋️energy/🏭️bridge/🦀️.rs`: `6e3845b37f4141f03d0f49a5a562774c1d91d77a2a93badc650cf52d6d01d54b`.
- `✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript/package.json`: `bed5f0eeb4fe1c5a2ded4e0c1a17a1b4c02ec87b1f9e9417bd5efda6b2e8bb7d`.
- `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/Cargo.toml`: `6c3b58fbe2cb8fb230d19c398b6bf5c7817e3f7bf8690bce19fd2c153da3066b`.
- `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/📜️script.ts`: `2b8760117a5020b08debd0d69626198e7d16628a502cbc23ac5ebbc9cae0e38a`.
- `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/🦀️.rs`: `66c981bf13f4ee44685574ee35245e88757fe15cee2577b11b4cf51ecbf2b05a`.
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/package.json`: `6358e3ac97c7638e07349b0418aa9e7f58283c5d34a3893c3f76543d2ed5f862`.
- `✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.toml`: `b03375424590735c2001344aa49064fc06353bb4b5bfacef2f2e0187debd49d4`.
- `✏️s/🔌️plugins/🕸️dag/🏭️bridge/📜️script.ts`: `64faaa0493b1b038359ffd363b9fd465815499f7dacc1738651fba147e585ed5`.
- `✏️s/🔌️plugins/🕸️dag/🏭️bridge/🦀️.rs`: `4638cfcbdb83a427341f222eb0596684dade110b1f4dca7182a2feb56220a3f8`.
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/package.json`: `1bb2b207e5e0a5bb262a67f86c2daa5862fe733b9632aaac2e188bb3b37f2440`.
- `✏️s/🔌️plugins/🖍️draw/🏭️bridge/Cargo.toml`: `0052525887131290443cdc56382cf3a1e672309b86a0d3b0778c963969c59e02`.
- `✏️s/🔌️plugins/🖍️draw/🏭️bridge/📜️script.ts`: `3b2c7d85c5d25ce9eab081ba45e21c4f6f5507a4b6d59f07e7f4aa9aa5548b1f`.
- `✏️s/🔌️plugins/🖍️draw/🏭️bridge/🦀️.rs`: `1a882af0b2563770bf7e4369f675293f71001517ce89c8d5b2b300c26e50e3d5`.
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/package.json`: `c061aab9c151c5350657b495e6d8b7bafc5ff5ce965cdacaa880f41c217bdf14`.
- `✏️s/🔌️plugins/🖨️raster/🏭️bridge/Cargo.toml`: `6078d2e7a7665c71a2f5afafa4d892d1b556c571ce582fdf5e05d042d5ded769`.
- `✏️s/🔌️plugins/🖨️raster/🏭️bridge/📜️script.ts`: `cc49f7224962ba109bb7d401c262de4bbb1b3b6e29866f5a17104a102e969791`.
- `✏️s/🔌️plugins/🖨️raster/🏭️bridge/🦀️.rs`: `713917e7797eba95a1324242691c88dd664871e687d77e918e6bd31ffc09efa3`.
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/package.json`: `6f91abd4a14aef8e89eb0c55c303745f2f05042ae5162d469169cc55961f3533`.
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`: `c8bb49f316771b9d567662a7abd6fb5ce7037e34a8258f46ba2220ab4bffd604`.
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs`: `bfc5d967fa1a626b0042c6b6b9df5d98a850ad7dde0bd90fe4c34e618dd0d069`.
- `✏️s/🔌️plugins/🗒️note/🏭️bridge/Cargo.toml`: `0617ecdff80b8cede11c0a59132d3529f4645f598844297a65773574d60f2ea7`.
- `✏️s/🔌️plugins/🗒️note/🏭️bridge/📜️script.ts`: `885d2a962133b4f52c711dd2b9acd1f07880f7e29499172a1b37cc5ac94cae62`.
- `✏️s/🔌️plugins/🗒️note/🏭️bridge/🦀️.rs`: `ad0a0523df82fc0fbfe1ceb2c024f04e5e0453de7ee0e7f4c9b0f91a106a1faf`.
- `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.toml`: `16cc9de2788f0336a0c16f341a2635483766033e6fe2e78900ec943e2b72d59c`.
- `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/📜️script.ts`: `d56e78c3f6fcea576d2779dda764a89a70385b7e5613fb67b4f7ac8c980c309d`.
- `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/🦀️.rs`: `24959150eafd3981bdc7ccfd1a1f3fbdc5367bd8d0c58ff1c02d1c84fd3a5fc5`.
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/package.json`: `aa55531c99db5d60d11cc5c30cbb3f2c1041d7b16819ddce904c0fd35631ad24`.
- `✏️s/🔌️plugins/🧱️block/🏭️bridge/Cargo.toml`: `6cf66e5245aee87c395f9ebe16abc7d4a9a49bd9d81e3286ce4f033d9611aec6`.
- `✏️s/🔌️plugins/🧱️block/🏭️bridge/📜️script.ts`: `c2c186f8f9fcebbdb15929cc77b9bfe4352ba3204600c81c5a9d39964118dff1`.
- `✏️s/🔌️plugins/🧱️block/🏭️bridge/🦀️.rs`: `dbd8828d05a63ba1c3d52763bff94ddc6f408a4ec04d52188c5310781daec430`.
- `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/package.json`: `84bbdf8a890c7cee7919af14d440cfc9dbe0fbedf64c63e88e27fc05268ba5bc`.
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml`: `7297b1991a6bf14611bf3f9e33def853e33152a5a6fc2d06e6482ce36e7c8708`.
- `✏️s/🔌️plugins/🧱️block/🟦️.ts`: `d9655b511408fcd7b24a261579c482aedbada359a1fa0bac0bd20f5c236c9d54`.
- `✏️s/🔌️plugins/🧱️block/🦀️.rs`: `73b4cc8cd6a27cf42b0264452bb04dfc78d149e762266297f491f8ba6f51a032`.
- `✏️s/🔌️plugins/🪐️space/🏭️bridge/Cargo.toml`: `069af07e8cbfba0a3cbead92936a037d4644f487f9484eebcd4c2dcae5ba3425`.
- `✏️s/🔌️plugins/🪐️space/🏭️bridge/📜️script.ts`: `c0ee02791daac39542345585397f868a07d71d6b2e90a9981ad3e51a7b4557a0`.
- `✏️s/🔌️plugins/🪐️space/🏭️bridge/🦀️.rs`: `453c24d07fa54bb15f343e7648eaf92c965c2ecb41748463c1a21824f850e87b`.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/package.json`: `04b9a3c048c0522d916c5a615876180547c4ac65bcf2c329d4fadb2e7ffb8682`.
- `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/Cargo.toml`: `ca7d3b858498dc40e6a153366cfbff5ab86811a2d3f4988fa5667ea334db9c61`.
- `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/📜️script.ts`: `ed61786a06a03fea57d9903bad3106f157b72f0f6f44c405061ced6911dcf850`.
- `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/🦀️.rs`: `e4043f02894dd211cc97660cc81913a5554894de1719a8d27edfb1495c8e78f5`.
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/package.json`: `33a63301dfa3c5f2b3a9eb77e8a5f90ebab51c9d0c1dbcf5947c9437a9209637`.
- `✏️s/🧑‍💻dev/💡️services/🌉️mcp/⌨️entrypoint/🦀️.rs`: `1f3070d37144f7fb6924e1a75d9ce81630df0e51661946daeb6004fd0fc9e179`.
- `✏️s/🧑‍💻dev/💡️services/📦️packages/🦀️rust/Cargo.toml`: `5e7527bfb66bbd351e0b82723a98afd3be57f447f3113a30fb4eb020aeec804d`.
- `✏️s/🧑‍💻dev/💡️services/🦀️.rs`: `ddcbab83ecc94860e5a113a5d994a1afd10b762f9003d458bfc1d388e16d76f3`.
- `✏️s/🧑‍💻dev/🚀️entry/🟦️.ts`: `d4d757324e1a3b8704b92fbf420a5ae3df562cf7d0d048a148af0b35150ccf3f`.
- `✏️s/🧑‍💻dev/🧩️service-composition/👷️worker/🟦️.ts`: `273455e9f7af3572dd5657d678f536754b527eecc6bed0cbb578d0045da5898e`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts`: `b804a9c7eafcc03c0b3aaaf8384dffe4944f0ca32f5362d5aff72a23b1c72b8e`.
