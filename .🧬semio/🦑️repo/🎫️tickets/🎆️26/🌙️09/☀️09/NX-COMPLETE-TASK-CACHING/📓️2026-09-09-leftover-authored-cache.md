# Leftover authored `cache: false`

Walked 413 `📋️project.json` files (excluded tickets, `node_modules`, `dist`, `target`, `.nx`).

Flipped **200** leftover deterministic targets to `cache: true` across **22** files:

| family token | flipped |
| --- | ---: |
| `check` | 179 |
| `test` | 15 |
| `stdio` | 4 |
| `verify` | 2 |

Continuous (`dev`/`serve`/`watch`/`start`) and mutating (`setup`/`deps`/`clean`/`publish`/`format`/`prepare`/`activate`/`generator-inputs`/`checkTarget` freshness / `write-baseline`) were not enabled.

No remaining `cache: false` target still matches those cacheable name families.

## Remaining `cache: false` (161)

| reason | count |
| --- | ---: |
| other (name outside cacheable families) | 53 |
| continuous:dev | 47 |
| mutating:deps | 15 |
| mutating:setup | 12 |
| mutating:clean | 7 |
| continuous:watch | 6 |
| mutating:publish | 6 |
| continuous:serve | 4 |
| mutating:prepare | 4 |
| continuous:start | 2 |
| mutating:write-baseline | 2 |
| mutating:activate | 1 |
| mutating:format | 1 |
| mutating:generator-inputs | 1 |
| mutating:checkTarget-freshness | 0 |

## Other (53)

Left uncached because the target name has none of `test`/`lint`/`build`/`generate`/`schema`/`verify`/`stdio`/`wasm`/`package`/`check`/`generator`:

- benches / smoke: `bench` (2), `bench-plugins`, `bench-plugins-native`, `bench-plugins-react`, `bench-plugins-wgpu`, `catalog-smoke`, `secure-local-smoke`, `collab-e2e`
- mutating-adjacent ops: `purge`, `update`, `new`, `new-taxonomy-mutation`, `ci-baseline`, `audit`, `doctor`, `disk-report`, `disk-prune`, `reset-document-ownership`
- live / native runners: `preview-generated` (3), `cpp`, `native`, `native-release`, `run`, `daemon`, `workflow`, `describe`
- oracles: `oracle-status`, `oracle-run`, `oracle-native`, `oracle-epjson`, `oracle-emit`, plus `*-oracle` helpers
- source / contract helpers: `*-source`, `*-contract`, `*-enforce`, `*-report`, ownership/map/value targets
