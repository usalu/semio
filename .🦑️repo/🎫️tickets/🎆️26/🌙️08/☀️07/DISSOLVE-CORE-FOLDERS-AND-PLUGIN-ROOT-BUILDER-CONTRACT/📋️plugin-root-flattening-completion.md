# Plugin Root Flattening Completion

## Outcome

- Hoisted the contract leaf and `🛂️manifest`, `🎟️capabilities`, `🔧️setup`, and `🎛️apps` facet leaves into all 33 plugin roots.
- Removed all 33 redundant `🔌️plugin/` directories.
- Preserved the existing `🧱️block` and `🪐️space` root components and merged registration into dedicated `🔌️Registration` regions.
- Updated every remaining active Rust glue reference from the nested contract path to the plugin-root path. One declaration was already changed concurrently; the applied edit covered the other 39 across 32 glue files. The final diff contains all 40 requested old/new path pairs, and the active-source scan contains zero `../../🔌️plugin/` references across all 33 glue files.
- Removed `pluginDirName` from the taxonomy and public `Taxonomy` contract. `pluginChildDirs` now declares the four direct-root facets.
- Updated the workspace policy, registry validator, Rust taxonomy assertion, SDK documentation, and existing repo-library tests to enforce the direct-root shape and reject `🔌️plugin/`.
- Added no compatibility paths, migration scripts, permanent scripts, launch configurations, or new tests files.

## Structural Assertions

Command:

```sh
bun -e 'import { readdir } from "node:fs/promises"; import { existsSync } from "node:fs"; import { join } from "node:path"; const root="✏️s/🔌️plugins"; const owners=(await readdir(root,{withFileTypes:true})).filter(x=>x.isDirectory()).map(x=>x.name).sort(); const facets=["🛂️manifest","🎟️capabilities","🔧️setup","🎛️apps"]; const missing=[]; const nested=[]; for(const owner of owners){const base=join(root,owner); if(!existsSync(join(base,"🦀️component.rs"))) missing.push(join(base,"🦀️component.rs")); for(const facet of facets) if(!existsSync(join(base,facet,"🦀️component.rs"))) missing.push(join(base,facet,"🦀️component.rs")); if(existsSync(join(base,"🔌️plugin"))) nested.push(join(base,"🔌️plugin"));} const glue=await Array.fromAsync(new Bun.Glob("*/📦️packages/🦀️rust/📦️glue.rs").scan({cwd:root})); const stale=[]; for(const path of glue){const text=await Bun.file(join(root,path)).text(); if(text.includes("../../🔌️plugin/"))stale.push(path);} console.log(JSON.stringify({owners:owners.length,rootContracts:owners.length-missing.filter(x=>x.endsWith("/🦀️component.rs")&&!facets.some(f=>x.includes("/"+f+"/"))).length,requiredFacetLeaves:owners.length*facets.length-missing.filter(x=>facets.some(f=>x.includes("/"+f+"/"))).length,nested,missing,glueFiles:glue.length,stale},null,2)); if(owners.length!==33||missing.length||nested.length||stale.length)process.exit(1);'
```

Result: exit 0; 33 owners, 33 root contracts, 132 required facet leaves, 33 glue files, zero nested directories, zero missing leaves, and zero stale references.

## Tests and Checks

```sh
bun nx run @semio-tech/repo-lib:test-quick
```

Result: exit 1; 131 passed and 20 failed. Failures are existing/concurrent repository drift outside this follow-up, including dependency-boundary expectations, a removed CSS path, micro-commit history behavior, playground configuration, markerless package discovery, and workspace catalog expectations.

```sh
bun nx run @semio-tech/repo-lib:test-quick -- --test-name-pattern plugin-root
```

Result: exit 0; both new plugin-root taxonomy tests passed, with 150 unrelated tests filtered out.

```sh
bun nx run @semio-tech/repo-lib:test-quick -- --test-name-pattern validateTaxonomy
```

Result: exit 1; 14 passed and one pre-existing completeness-set expectation failed. Both new `pluginChildDirs` rejection tests passed.

```sh
bun nx run workspace:verify-gate
```

Result: exit 1 after dependency-cruiser passed. The existing plugin registry taxonomy walk reported broad artifact/glue drift. The new direct-root checks reported no nested `🔌️plugin/` directories and no missing root contracts or facets.

```sh
bun nx run @semio-tech/framework-os-dev:plugin
```

Result: exit 1 after compiling several plugin crates; the build stopped at `animate` with existing unresolved `engine::animate`, `engine::animate_video`, and snapshot schema imports.

```sh
bun nx run @semio-tech/framework-os-dev:plugin -- block
```

Result: exit 0; the collision-merged block root component compiled and produced its wasm component.

```sh
SEMIO_PLUGIN_ONLY=s bun nx run @semio-tech/framework-os-dev:plugin -- s
```

Result: exit 1 before compiling the space crate because the current Tokio feature set is unsupported on wasm.

```sh
bun nx run @semio-tech/repo-lib:lint
```

Result: exit 1 before checking the edited library because the existing generated manifest TypeScript has a syntax error at line 154.

```sh
git diff --check
```

Result: exit 0.

## Changed-File Summary

- Moved 165 contract/facet leaves out of `✏️s/🔌️plugins/*/🔌️plugin/` and created their direct-root equivalents.
- Completed all 40 Rust glue path replacements; 39 were applied in this follow-up after preserving one concurrent replacement already present.
- Updated `🧱️block/🦀️component.rs` and `🪐️space/🦀️component.rs` with registration regions.
- Updated the root workspace policy, repo taxonomy JSON, taxonomy discovery interface/validator, existing repo-library test file, plugin registry validator, and Rust plugin SDK component.
- Added this follow-up research and completion documentation inside the reopened ticket.
