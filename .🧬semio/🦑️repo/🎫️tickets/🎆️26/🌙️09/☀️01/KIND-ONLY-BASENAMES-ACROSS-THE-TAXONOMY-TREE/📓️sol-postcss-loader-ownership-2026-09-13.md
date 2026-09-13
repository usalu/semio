# Sol PostCSS Loader Ownership

## Outcome

The React PostCSS configuration now belongs to the semantic React build-styling concern:

    🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🎨️styling/🟦️.ts

The former package implementation at …/📦️packages/🟦️typescript/postcss.config.ts is absent. The package keeps its public ./postcss.config export through the anonymous explicit-ESM leaf …/📦️packages/🟦️typescript/🟦️.mts. Its complete body is:

    export { default } from "../../🛠️build-tooling/🎨️styling/🟦️.ts";

The owner exposes the named structural configuration uiPostcssConfiguration and the Vite-specific factory uiPostcssInlinePlugins. It does not export PostCSS or Tailwind types. The default export preserves the exact named plugin configuration {"plugins":{"@tailwindcss/postcss":{}}}.

The owner SHA-256 is 4ad1f69f95576dbd91893361b3adddd118ae0d8b7b476acfd70e4985eebf7a1a. The package leaf SHA-256 is eaabe0cc8d5967c890f29bbc09d3a3941fdeeb73f04dd4ddbc94e0ec9b71ecf8.

## Schema-first and first-red evidence

The existing language-neutral tool-configuration fixture and Draft 2020-12 schema were expanded before the source move. They require twelve semantic configuration owners, twenty-two current consumer rows, eight project bindings, six retired fixed-contract IDs, and an exact PostCSS interface section. That section distinguishes:

- postcss-load-config with the explicit 🟦️.mts search place;
- Vite with the uiPostcssInlinePlugins inline plugin array;
- the current semantic owner and package entry;
- the frozen historical fixture and its prior path; and
- two portable input/declaration vectors.

The first focused source check produced 0 pass / 1 fail / 47 assertions because the declared semantic owner did not yet exist:

    Expected true, Received false: …/⚛️react/🛠️build-tooling/🎨️styling/🟦️.ts

After the move, the first full direct route produced 8 pass / 1 fail / 148 assertions. It found an exact stale consumer left by the already-completed OS composition extraction: the tool-configuration fixture still expected the OS package router to carry config: "../../🏗️builder/🌐️vite/🟦️.ts", while the live owner was …/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts. The fixture and cached target input were rebased to that actual consumer.

The next full direct route produced 8 pass / 1 fail / 160 assertions because the concurrently retired …/🧩️vscode/README.md remained in the current consumer map. Its historical source token was removed from the current map and target inputs rather than recreated. The final current count is therefore twenty-two consumers: the PostCSS package leaf was added, the OS Vite selector moved, and the absent README ceased to be current authority.

## Taxonomy and cache closure

The live fixed contract and package disposition named postcss-config were removed. The TypeScript package rule no longer admits that conventional filename. The taxonomy now declares react-build-tooling-styling, binding build-tooling to member 🎨️styling. The frozen package-purity fixture itself remains byte-unchanged with decision state non-authoritative-concurrent-source-byte-drift; the ownership test explicitly proves its postcss-config prerequisite and historical path remain frozen input rather than current taxonomy authority.

The React Nx project names the semantic owner explicitly in namedInputs.default. The repo-library ownership target includes the semantic owner, anonymous package leaf, package manifest and project, historical fixture, ownership fixture/schema/test, taxonomy, current consumers, settings, and seed/derived launch authorities. Its package script and both launch files continue to select the existing registered target test-tool-configuration-ownership; no new command facade was added.

A focused current discovery check loaded the taxonomy, resolved 🎨️styling beneath build-tooling, and found no package-boundary problem for the new .mts leaf. It reported only the separately existing React package router implementation finding.

## Runtime evidence

| Control | Result |
| --- | --- |
| Retained ticket control + Draft-07 Ajv | Passed: schema version 2, two cases, frozen owner → current owner distinction |
| Focused portable taxonomy/registration | Passed: 2 tests, 34 assertions |
| Focused current package/search-place/Tailwind/Vite control | Passed: 1 test; package export, standalone loader, both vectors, Vite write:false, Bun and Node package resolution |
| Full direct package route | bun ./📜️script.ts test tool-configuration-ownership from repo-library TypeScript package: exit 0, 9 tests, 171 assertions, 6.72 s |
| Actual isolated Nx route | NX_DAEMON=false NX_ISOLATE_PLUGINS=false … bun nx run @semio-tech/repo-lib:test-tool-configuration-ownership --skip-nx-cache: exit 0, 9 tests, 171 assertions, test 18.46 s, Nx 19.3 s, cache skipped |
| Current Bun standalone loader | Bun 1.3.14 resolved the package 🟦️.mts through explicit searchPlaces; both vectors passed |
| Current Node standalone loader | Node 24.15.0 resolved the same package 🟦️.mts through explicit searchPlaces; both vectors passed |
| Cross-runtime projection | Configuration and complete CSS bytes match after removing only runtime/version labels; SHA-256 e02af85dbf7d53cb716f6fd43212728051ab01972383af868b6458ed8aa250fd |

The two exact current outputs under both runtimes are:

    /*! tailwindcss v4.3.3 | MIT License | https://tailwindcss.com */
    .probe {
      text-decoration-line: underline;
    }

    /*! tailwindcss v4.3.3 | MIT License | https://tailwindcss.com */
    .probe {
      color: #123456;
    }

Neither contains @apply.

The Vite control uses configFile:false, publicDir:false, write:false, and the explicit inline plugin array returned by the semantic owner. The coordinator’s retained pre-extraction negative control remains the authority that a filename string in css.postcss is a directory selector and can complete while leaving @apply unprocessed. This lane did not turn that failed boundary into a selector.

Terra’s final independent audit is accepted in 📓️terra-postcss-loader-audit-2026-09-13.md: semantic placement, minimal package glue, current 22-consumer map, historical separation, exact inputs, and both loader interfaces have no remaining blocker.

## Limits

No React, Storybook, browser, public-directory, or full product build ran. The Vite proof is a ticket-private in-memory compiler case. It proves the actual installed compiler consumes the inline owner interface, while the standalone proofs establish explicit search-place loading and package export resolution under current Bun and Node. It does not claim arbitrary PostCSS discovery or external consumer reachability.

The OS composition report now records the separate broad-quick integration boundary precisely: its former staging-root fixture selected the absent historical WGPU Rust command …/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts and stopped with ENOENT; the same fixture also named absent 🌐️.html and Trunk.toml siblings. The coordinator subsequently repaired that separate fixture and registered route. No WGPU source was changed here.

## Exact file attribution

| Action | Path |
| --- | --- |
| Created | 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🎨️styling/🟦️.ts |
| Created | 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.mts |
| Removed | 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/postcss.config.ts |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json |
| Updated | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json |
| Updated | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🎚️tool-configuration-ownership/🔣️.json |
| Updated | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎚️tool-configuration-ownership/🔣️.json |
| Updated | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️tool-configuration-ownership/🟦️.ts |
| Updated | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json |
| Updated authored control | 📋️postcss-loader-controls/🔣️.json |
| Updated authored schema | 📋️postcss-loader-controls/🧬️schema/🔣️.json |
| Created report | 📓️sol-postcss-loader-ownership-2026-09-13.md |

The two authored controls and this report are retained. Disposable logs, runtime JSON, Vite scratch, and private Nx cache/workspace data under 🗑️generated/sol-postcss-ownership are removed after report finalization.
