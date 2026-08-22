# P10s-a Dependency-Cruiser Semantics Alignment

## Scope

Implemented the narrow policy repair identified by `📓️p10s-global-dependency-cruiser-audit.md`:

- interpret the `renderer-hosts-only-ui` allowlist using dependency-cruiser's resolved-path semantics;
- exempt only approved bootstrap-tooling entry points from the cross-technology runtime rules;
- retain all rule families, generated sources, `node_modules`, Compose, OS/product paths, and cross-technology runtime sources in the graph.

The only implementation file changed by this packet is the root `.dependency-cruiser.cjs`.

## Boundary Design

`renderer-hosts-only-ui` now permits the canonical resolved forms of:

- the renderer React host itself;
- framework UI and styling;
- framework protocol packages;
- React and React DOM;
- Vite/Vitest test-tooling references;
- Node built-ins, whether dependency-cruiser reports `node:*` or the resolved bare built-in name.

It does not permit arbitrary `node_modules` targets. The focused regression corpus explicitly keeps Three, framework kernel, replication, OS glue, and the demonstrator product outside the allowlist.

The cross-technology rules now exempt only these source entry-point forms:

- `📜️script.ts`;
- `vite.config.{js,jsx,ts,tsx,mjs,mts,cjs,cts}` and its owned `⚙️`-prefixed form;
- `vitest.config.{js,jsx,ts,tsx,mjs,mts,cjs,cts}` and its owned `🧪️`-prefixed form.

Ordinary runtime source, generated source, and arbitrary `*.config.ts` files remain governed. All 20 directed cross-technology rules use the same exact entry-point boundary.

## Focused Regression Coverage

Configuration loading runs an owned fixture assertion before dependency-cruiser receives the rules. It proves:

- four approved script/config shapes match;
- runtime entry points, generated files, and arbitrary config names do not match;
- all 20 cross-technology rule objects carry the narrow boundary;
- renderer self/UI/styling/protocol/React/React DOM/Vitest/Node fixtures are allowed;
- Three, kernel, replication, OS glue, and demonstrator fixtures remain forbidden;
- the exported renderer rule is wired to the tested resolved-path allowlist.

Focused load command:

```text
node -e 'const config = require("./.dependency-cruiser.cjs"); ...'
[DEBUG] config-regression=pass cross-rules=20 bootstrap-patterns=1 renderer-allow-patterns=7
```

## Exact Narrow Census

Command used before and after:

```text
bunx dependency-cruiser compose 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config .dependency-cruiser.cjs --output-type json
```

The JSON result was grouped by rule name with `jq`. The pre-edit run completed in 19.21 seconds. The post-edit run completed in 18.17 seconds. The terminal rendered one post-edit `✏️s → 🧰️framework` group with a replacement glyph; its 1 finding is combined with the correctly rendered 23-findings group below because both are the same rule emitted from the same 20-rule configuration.

| Severity | Rule | Before | After | Delta |
| --- | --- | ---: | ---: | ---: |
| error | `no-circular` | 155 | 155 | 0 |
| error | `no-core-path` | 84 | 84 | 0 |
| warn | `no-cross-package-relative` | 228 | 228 | 0 |
| error | `no-cross-technology-compose-to-🧰️framework` | 32 | 8 | -24 |
| error | `no-cross-technology-♻️mit-bestand-to-✏️s` | 1 | 1 | 0 |
| error | `no-cross-technology-♻️mit-bestand-to-🧰️framework` | 21 | 9 | -12 |
| error | `no-cross-technology-✏️s-to-🧰️framework` | 160 | 24 | -136 |
| error | `no-cross-technology-🌎️hub-to-🧰️framework` | 22 | 18 | -4 |
| error | `no-cross-technology-🧰️framework-to-♻️mit-bestand` | 3 | 3 | 0 |
| warn | `no-impl-segment` | 2 | 2 | 0 |
| warn | `no-plugin-to-extension-📐️cad` | 4 | 4 | 0 |
| error | `no-state-outside-os` | 1 | 1 | 0 |
| error | `not-to-unlisted` | 89 | 89 | 0 |
| warn | `plugins-framework-sdk-only` | 11 | 11 | 0 |
| error | `renderer-hosts-only-ui` | 147 | 43 | -104 |
| error | `s-modules-no-plugins` | 9 | 9 | 0 |
| error | `ui-no-framework-packages` | 36 | 36 | 0 |
| **all** | **Total** | **1,005** | **725** | **-280** |
| **error** | **Total** | **760** | **480** | **-280** |
| **warn** | **Total** | **245** | **245** | **0** |

The literal audit gate was then run with its original output mode:

```text
bunx dependency-cruiser compose 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config .dependency-cruiser.cjs --output-type err
x 721 dependency violations (476 errors, 245 warnings). 11362 modules, 10002 dependencies cruised.
```

It completed in 16.96 seconds and exited non-zero as intended. The shared tree changed concurrently between the grouped JSON measurement and this literal gate: the owned style-variants packet removed four `class-variance-authority` imports from ActionGroup, ButtonGroup, ToggleGroup, and Button, eliminating four additional error findings while the warning total stayed exactly 245. The per-rule table is therefore the isolated immediate before/after measurement for this configuration packet; the 721/476/245 line is the later literal-gate state, not silently substituted into that table without another per-rule snapshot.

## Runtime Findings Preserved

The post-edit graph still reports 63 cross-technology runtime findings across the six populated directions. Both reverse directions remain unchanged (`♻️mit-bestand → ✏️s`: 1; `🧰️framework → ♻️mit-bestand`: 3). The renderer rule still reports 43 non-presentation dependencies, while the genuine companion ownership rules remain unchanged (`ui-no-framework-packages`: 36, `no-state-outside-os`: 1, `s-modules-no-plugins`: 9).

Representative renderer targets intentionally kept outside the tested allowlist include framework OS glue, kernel, replication, registry/product sources, Three, and the demonstrator. Generated code remains scanned, arbitrary non-approved config files remain scanned, and `options.doNotFollow` was not widened.

## Result

P10s-a removed 280 policy-shaped false positives without suppressing a warning, a rule family, vendor resolution generally, or the remaining runtime architecture backlog. The narrow command correctly remains non-green because the remaining genuine/declaration/taxonomy findings belong to the later P10s repair packets.
