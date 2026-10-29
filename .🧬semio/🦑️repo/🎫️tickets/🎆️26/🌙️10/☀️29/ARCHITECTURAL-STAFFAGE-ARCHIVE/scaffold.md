# Staffage scaffold: projects, commands, taxonomy findings

S = `🎓️teaching/🏛️architecture/🧍️staffage`. All commands run from `/home/deck/semio`.

## Projects created

| Project | Path | Targets |
|---|---|---|
| `@teaching/staffage` | `S/📦️packages/🟦️typescript` | `test`, `typecheck` |
| `@teaching/staffage-react` | `S/🎯️targets/⚛️react/📦️packages/🟦️typescript` | `test`, `typecheck` |
| `@teaching/staffage-site` | `S/🌐️site/📦️packages/🟦️typescript` | `dev`, `dev-site`, `build`, `test`, `typecheck` |

Supporting files: vitest configs at `S/🧪️tests/🎚️config/🟦️.ts` (glob `../../🧪️tests/*/🟦️.ts`, excludes the config dir), `S/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (jsdom, React plugin, aliases) and `S/🌐️site/🧪️tests/🎚️config/🟦️.ts`; site `🌐️.html`, `🟦️.ts`, `🎨️.css`, `🏗️builder/🌐️vite/🟦️.ts`, `tsconfig.site.json`. Smoke cases: `S/🧪️tests/🧬️schema-twin`, `S/🎯️targets/⚛️react/🧪️tests/🚀️mount-archive`, `S/🌐️site/🧪️tests/📄️host-document`. `mountArchive(root)` stub is `S/🎯️targets/⚛️react/📦️packages/🟦️typescript/📚️app/🟦️.tsx` (website worker owns it now).

Workspaces: three paths appended to `🎓️teaching/package.json` `workspaces`. Dependency port 6071 (`semio.app.port.dev`, env `TEACHING_ARCHITECTURE_STAFFAGE_PORT`, hostKind `architecture-staffage`); grep over project/package.json found no other 6071.

Site facts: no `lang` on `<html>`; titles/noscript are en then de; Vite config = course config minus proctor, assignments, wasm and presence contract (plugins: serve close/upgrade, host html, emoji index html, referenced assets, Tailwind, React); aliases for `@teaching/staffage*`; `dedupe` react. `build` uses `buildViteArtifact` (owner `architecture-staffage:site`) exactly like the course: the bundle lands in the repository products store (receipt `.🧾️store/build.json`), not in `dist/`, because the course does the same. The course has no `preview` target, so none was added.

## Dependencies

New workspace members need a lock update: `repo show projects` refused with "lock file of dependency scope 🎓️teaching is out of date". Ran `repo dependencies lock --scope 🎓️teaching --lang=en` (32 s), which added the three members to `🎓️teaching/bun.lock` (diff reviewed: only the three workspace entries and their resolution rows). `repo dependencies ensure` was not needed for the fast loops (root `node_modules` already hoists react, vitest, jsdom, @testing-library/react).

## Fast inner loops (verified, seconds)

```
# library tests (0.5 s)
cd S/📦️packages/🟦️typescript && repo tools exec bun -- x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts
# react tests (jsdom, 0.6 s)
cd S/🎯️targets/⚛️react/📦️packages/🟦️typescript && repo tools exec bun -- x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts
# site tests: pass the config as an ABSOLUTE path (a relative path from the site package silently picked the library config)
cd S/🌐️site/📦️packages/🟦️typescript && repo tools exec bun -- x vitest run --config "$PWD/../../🧪️tests/🎚️config/🟦️.ts"
# typecheck per package (no cache), 5 to 60 s
cd <package> && repo tools exec bun -- x tsc -p tsconfig.json --noEmit --incremental false
# fast site bundle (6.7 s); the Vite config needs the bounded cache env
cd S/🌐️site/📦️packages/🟦️typescript
SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=/home/deck/semio/.cache/b/p/m/staffage-vite-fast \
  repo tools exec bun -- x vite build --mode prototype --config ../../🏗️builder/🌐️vite/🟦️.ts --configLoader native --outDir <abs-out>
# fast dev server on the fixed port (ready in 150 ms)
SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=/home/deck/semio/.cache/b/p/m/staffage-vite-fast \
  repo tools exec bun -- x vite --mode prototype --config ../../🏗️builder/🌐️vite/🟦️.ts --configLoader native --port 6071 --strictPort --host 127.0.0.1
```

## Results

- Library vitest: 1 file, 3 tests pass. React vitest: 1 file, 1 test pass. Site vitest: 1 file, 2 tests pass.
- tsc: the react and site packages are clean for their own files; the library typecheck currently fails only in other workers' files (`🔨️modules/📤️export/🖊️dwg/🟦️.ts` imports a non-exported `Text` and reads `.instance`; `🧪️tests/🫁️breath-plume/🟦️.ts` casts a readonly `Streamline`). Not mine; reported to the coordinator.
- Fast Vite build (prototype): 174 modules, built in 2.27 s (6.7 s wall); output 11 MB total (`index.html` 4.2 kB, CSS 309 kB, main chunk 2.46 MB plus 6 other chunks). Served statically with `python3 -m http.server`: `index.html` 200 (4210 B) and the entry chunk 200 (2.46 MB). Build warning from another worker's in-flight code: `writeDwg` is not exported by the library barrel but imported by `🎯️targets/⚛️react/🔨️modules/📤️downloads/🟦️.ts`.
- Fast Vite dev on 6071: `GET /` 200 (4378 B, contains `#root`), `GET /🟦️.ts` 200, ready in 147 ms, stopped by timeout.
- `repo show projects` / `repo run`: see the section below.

## Taxonomy findings (task 5)

- `pathEmojiPolicy` (single emoji grapheme with VS16, no generic folder glyphs, no sibling name collisions) and `areaEnforcement` (`🎓️teaching` is a declared `clean` area, undeclared areas enforced) are audited by the taxonomy inventory (`🧹️normalization/📏️fixed-contracts` `canonicalDirectory`), which reports `directory-kind-unresolved` for any directory that has no registered semantic kind.
- Registration in `🔣️taxonomy.json` is done by `semanticDirectoryKinds` (structural kinds with `parentKindIds`, e.g. `teaching-pets`, `teaching-course`) and `semanticDirectoryMemberKinds` (`members-of-teaching-architecture` lists `📚️course`, `⚡️energy`, ...; `members-of-teaching-pets` lists every pet; a member row may instead carry an anchored `memberSlugPattern` that admits any emoji plus matching slug). There are no per-directory manifests (`🔣️.json` with `x-semio`) for `🎓️teaching`; `🐾️pets` under architecture is not in the architecture member list but has a structural kind, so a structural kind suffices.
- Currently registered by name anywhere in the taxonomy (shared names such as `🧪️tests`, `🔨️modules`, `📦️packages`, `🧬️schema`, `🏗️builder`, `🎞️animation`, `🦴️rig`, `🗂️catalog`, `📤️export`, `🖊️dwg`, `📐️dxf`, `🌐️site`, `⚛️react`, `🎯️targets`, `🧫️fixtures`) resolve by contract; every name listed as NEW below does not occur in the taxonomy at all.
- No `🧍️staffage` kind exists: `teaching-staffage` (emoji `🧍️`, slug `^staffage$`, parent `teaching-architecture`) is missing, plus member rows for its descendants.
- I did not edit the 1.3 MB shared `🔣️taxonomy.json`: I could not run the inventory over the tree (my probe script `taxonomy-probe.ts` in this ticket was declined by the operator), so the exact set of enforced rows is unverified. Proposed registration, to be applied and then checked with the repository's taxonomy inventory: add `teaching-staffage` to `semanticDirectoryKinds` and a `members-of-teaching-staffage` row (ownerKindIds `teaching-staffage`) with memberNames for the direct children and `memberSlugPattern` rows for the case and category directories below.
- Names under S not found in the taxonomy (150 distinct directory names in total, `__pycache__` from the oracle script must be removed): 🐕️animals 🧱️components 🔗️contracts 🎨️depiction 🧮️evaluation 👤️humans 🪑️objects 📕️pdf 🖼️svg 🌡️thermal 📐️units 🌳️vegetation 🚜️vehicles and all test case directories (`🧬️schema-twin`, `🫁️breath-plume`, ...), app modules (`📚️app`, `🎛️viewer`, `🗃️browse`, ...) and fixture cases. Of the names the coordinator listed, `🗂️catalog`, `🦴️rig`, `🎞️animation`, `📤️export`, `🖊️dwg`, `📐️dxf`, `🏗️structure`, `🌐️site`, `🧪️tests`, `🧫️fixtures` do occur elsewhere in the taxonomy.
