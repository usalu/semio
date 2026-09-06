# Hub Catalog Compile Cohort

## Scope

The retained `exact-cargo-laws-ndUoQ8/00` compiler stream contained 35 errors from a coherent set of current-contract migrations. This packet records source repair only. Root owns the next native build and law verdict on its Hub cache.

## Repaired boundaries

- Inference WAL tests import the current `directory::Inference` trait and construct one typed `DurableFixtureRecord` per proof. Verifier targets and retained storage now receive that exact record.
- Chain fixtures publish Store-owned `WAL_EVENT` records, keep their independent CRC/BLAKE3 tampering, and perform every storage mutation through one acquired `WalWriterPermit` that is explicitly released.
- Hub tests enable the OS kernel `testkit` feature only through the Hub dev dependency so the real durable record constructor remains test-only.
- Directory tests use `DIRECTORY_EVENT_READ_MAX` and a test-only SQLite corruption seam instead of reaching through the backend's private lock.
- Native provider sibling tests use narrow catalog-owned accessors. The fixed window-kind container uses its infallible `first()` accessor directly.
- The directory event-page source oracle now scopes duplicate admission/process markers to the exact functions they protect; unrelated identical markers can no longer make a hostile mutation pass.

## Source evidence

- `bun nx format:check --files=<seven repaired Rust/TOML files>`: GREEN.
- `bun nx run os-hub:gis-map-proposal-source-check`: GREEN; AJV 1, hostile 7, lifecycle 9, visibility 7, errors 11, approval rejections 11.
- `bun nx run os-hub:directory-event-page-v1-source-check`: GREEN; checks 37, AJV 1, vectors 5, queries 12, hostiles 5, source hostiles 9, process hostiles 4, SHA-256 1.
- The plugin-registry native-catalog oracle completed its source corpus first: 23 cases, 4 positive and 19 denied. The enclosing Hub target then correctly refused to enter its native phase without a ticket-local artifact directory, so this packet makes no native claim.

## Remaining verdict

The next root-owned exact Hub native build must confirm the repaired cohort against the current shared source snapshot. No test law or executable is claimed green by this source-only packet.
