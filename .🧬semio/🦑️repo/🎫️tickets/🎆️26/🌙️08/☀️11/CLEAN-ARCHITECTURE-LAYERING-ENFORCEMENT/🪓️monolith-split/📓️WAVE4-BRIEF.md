# 🪚 Wave 4 Carving Brief

Shared instructions for every wave-4 carving executor. The goal is that everything compiles and runs end to end, and the compiler-ingress source boundary has to be passed crate by crate first.

## Read first

All files are in `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/` unless given as a repo path:

- `/Users/ueli/Documents/semio/AGENTS.md`
- `📓️EXECUTOR-RULES.md` (no modifying git, no compiling around the gate, re-read before every edit)
- `📓️CARVING-RULES.md`
- `📓️RESOLUTION-ORACLE.md`, including its sandbox patches
- `PRODUCER-CRATE-RECIPE.md`
- `CRATE-SOURCE-GATE-CENSUS.md`
- the RESULT-*.md file for any earlier split of your target
- `RESULT-CARVE-PETS.md` and `RESULT-CARVE-PRINT-SERVER.md`, as worked examples of a carve

Work in the foreground with no sub-agents. Do not end your turn until your RESULT file exists.

## Method

1. Measure first: `cd <monolith-split>/🗑️generated && ONLY=<crate> bun census.ts`.
   - Record files, bytes and active lines for the lib closure, and for the test closure (the cfg-`test` count matters too, since `cargo test` units are gated).
2. Embedded assets (`include_bytes!` / `include_str!` of data, icons, fonts or schemas) move to a runtime channel supplied through an explicit port or parameter.
   - Reuse existing mechanisms: the compiler `FontSet`/`CompilerFonts` port and print's `ChartSchemaAssets` are precedents.
   - Wire every host or caller in the same change. A carve must never leave a runtime regression; see RESULT-CARVE-COMPILER.md for how the hosts were wired.
3. Carve cohesive modules into producer crates. Each crate should land at or under 3500 active lines, 28 files and 230000 bytes, for both its lib and test closures.
   - Dependencies form a downward DAG; a child never depends on its former parent.
   - Each module file has exactly one owning crate: no `#[path]` re-mounting of the same file into two crates.
   - The former crate becomes either a thin facade with explicit `pub use` items, or disappears with its consumers re-pointed. Prefer the cleaner long-term option.
   - Follow the recipe exactly for each new crate: manifest, lib root placement, workspace membership (byte-sorted, in the right workspace), Nx project / `project.json`, the owner row in `📋️build-owners.json`, and six launch routes in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, matching what recent carves did for the seed.
4. Preserve every `//#region` marker. No compatibility aliases. Update all consumers, including Rust, TS/wasm bindings, scripts, fixtures and source-text readers (via `readModuleTreeSource` or child-specific reads).
5. Verify:
   - census rows for every resulting crate, lib and test;
   - the patched full-resolution differential per crate root (native, `--cfg test`, and the wasm targets the crate builds for);
   - a definition scan with a seeded sensitivity check;
   - rustfmt, path resolution, TOML parse, and parsing of both JSON files;
   - the related TS tests.
6. Real compile: try the crate's Nx producer route, `bun repo run @semio-tech/<project>-rs:check --configuration=prototype --parallel=1 -- --lib --offline`, or `bun repo cargo --store=shared --configuration=prototype -- check -p <crate>`. The build infra is churning; recent errors were "Finite original maintenance authority required" and a missing `📚️layers/🟦️.ts`. Try at start and at end. If the gate and launcher admit your crate, run check and test for real and fix every error. Report the actual output.
7. Peer-owned crates are not yours: `os-kernel`, `plugin`, `replication`, `ui-contract`, `ui` aggregate producers, `job`, `2d`. Touch them only with minimal consumer-side edits.

## Deliverable

`<monolith-split>/RESULT-W4-<SLUG>.md` covering: census before and after, the crate graph, the files changed, consumers re-pointed, verification results with real output, and residual risks.

## Rule added: god objects

An inherent `impl` cannot span crates. When a struct's impl is too large for one crate, decompose the struct into cohesive component structs. Each component owns its fields and its impl in its own crate. The original struct becomes a thin owner that composes the components. Methods live with their data, and callers use components directly where that is cleaner.

Do not use extension traits as a workaround for splitting one god impl. Use traits only for genuine polymorphism or inversion points.

Break cycles by passing narrow component references or explicit parameters instead of `&mut WholeState`. Big test files split along the same component lines.
