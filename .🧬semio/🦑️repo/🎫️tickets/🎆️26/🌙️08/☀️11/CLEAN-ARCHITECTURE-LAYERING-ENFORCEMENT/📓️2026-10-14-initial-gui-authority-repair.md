# Initial GUI Authority Repair

## Source Contract

Both initial authoring and issued caller schemas now require `launches`, matching the existing admission use of `.some`. Missing launches are refused by defining schema validation before that property is consumed.

Initial authoring admission now resolves the explicitly supplied CLI root to its canonical absolute path, resolves the declared workspace against that root, verifies exact root/tool/argument/selection/launch identity, resolves the declared artifact directory against the admitted root, and validates the resulting closed caller shape before the first envelope can be created. The transport, child, network and build profile declarations are preserved. No later quota/grant replacement or compatibility mode was introduced.

## GUI Inputs

Only these three rows were handcrafted in each of `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`:

- Record Original Receiving Source: child 180000 ms, existing parent policy retained.
- Record Original Receiving Native: child 180000 ms, existing parent policy retained.
- Verify Jack Native Composition: child 1200000 ms, existing parent policy retained.

Inside their JSON capability strings, `workspaceRoot` is now the literal `.` and `native.artifactDirectory` is the literal `.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture`. Plain `CARGO_TARGET_DIR` continues to use `${workspaceFolder}/...`. Actor's three absent-capability GUI rows were not changed. Original and edited capability inputs are retained in `initial-gui-authority-inputs.json`.

A read-only Python structural check decoded all six edited capability strings, confirmed that only the two path declarations changed, confirmed retained exact child bounds and ordinary path interpolation, and confirmed both defining schemas require launches. This is an executed source/edit verification, **not a passing runtime test or GUI/Windows acceptance**.

## Authored Test Coverage

The defining language-neutral graph corpus contains the two DSL 180000 ms cases and the composition 1200000 ms case with exact native transport/network/profile stock; it also contains POSIX and Windows path serialization expectations. The corpus also explicitly declares missing-launch omission, expected refusal validity and the defined admission error. Existing graph build tests now:

1. Compare defining authoring/caller validation to independent Ajv and refuse missing launches.
2. Admit relative declarations against a caller-owned fixture root different from process cwd.
3. Follow initial bootstrap → original finite envelope → actual orchestration help handoff → native task issuance, asserting unchanged policy/deadline and exact native stock.
4. Verify POSIX/Windows JSON serialization with their respective system path implementations.
5. Parse both actual GUI files with TypeScript's independent JSONC parser and compare these six rows to the portable corpus.

These tests are **authored and unrun**. Actual registered source execution remains blocked behind the live graph writer described in `📓️2026-10-14-build-discovery-repair.md`; no new capture was started after this source frontier changed.

The current writer-port inspection is separately retained in `📓️2026-10-14-current-graph-writer-port.md`; its current defining export already exists under the concurrent owner. No alias, separate singleton import, lease change or fresh runtime acceptance was introduced.


Current source roster follow-up: native ownership concurrently reauthored the original three capability artifact paths to `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/shared/target`; corresponding portable case native fields already match. The two DSL case `launchName` values contained literal Python `\U0001faf4` text and were corrected to the exact actual `🫴️ Record · Original Receiving Source` / `🫴️ Record · Original Receiving Native` row names. Read-only JSON comparison confirms all three current authored capability native objects, relative workspace roots and argument arrays agree with their portable fixture. This is current source/corpus agreement only; no GUI or test acceptance.
