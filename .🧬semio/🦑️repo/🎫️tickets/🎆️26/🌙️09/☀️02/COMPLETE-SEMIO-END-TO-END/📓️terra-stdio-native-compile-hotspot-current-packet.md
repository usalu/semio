# Stdio Native and Space Guest Compile Hotspot — Current Packet

Read-only audit on 2026-09-05. No process was stopped, no build was started, and no profile or cache setting was changed.

## Current execution evidence

The observed delay is active compiler work, not a Cargo target-lock wait. The process snapshot below was taken during this audit; the `R`/`RN` Rust children were consuming CPU while their Cargo parents slept awaiting them.

| PID | elapsed at snapshot | state | CPU | evidenced role |
| --- | ---: | --- | ---: | --- |
| 8838 | 1:15:29 | `U` | 4.9% | `rustc` compiling `semio_s_plugin_stdio` for the Home public-boundary target |
| 89433 | 9:16 | `R` | 10.3% | `rustc` compiling `semio_s_plugin_stdio` under Hub native qualification |
| 91819 | 7:10 | `RN` | 17.2% | third concurrent `semio_s_plugin_stdio` Rust compilation for the AI/Map target |
| 97490 | 1:21:46 | `Ss` | 0.0% | parent Cargo, `semio-hub` binary test with `sqlite`, `--no-run` |
| 85207 | 13:30 | `Ss` | 0.0% | second parent Cargo, `semio-hub` library test with `sqlite`, `--no-run` |

The full retained process arguments identify the Home child as a `cdylib` **and** `rlib` Stdio invocation in its dedicated ticket target. This is real code generation/link preparation, rather than metadata resolution or an idle compiler. It also means simultaneous isolated target roots duplicate Stdio work; that observation does **not** authorize terminating or merging any current owner target.

## Repository-owned hotspot, not a cache theory

`semio-s-plugin-stdio` is a deliberately monolithic taxonomy root:

- Its root has **4,690** textual `#[path]` inclusions, while its artifact tree currently contains **3,551** Rust sources: [Stdio root](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:124>). The source census is not a claim that all tree files become individual rustc units, but it accurately shows the breadth pulled through one root.
- The root itself records that `dsl::Mutations` validates the complete descriptor roster in a `DESCRIPTORS` constant and that glTF alone has 120 leaves exceeding rustc's normal long-const-eval budget: [Stdio root](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:13>). This is concrete source evidence of expensive type/const expansion, although no profiler was run to assign it a percentage of wall time.
- The library declares both `cdylib` and `rlib`: [Stdio manifest](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:17>). Its only feature is `plugin-root`; disabling it suppresses duplicate plugin installer symbols but does not gate the unconditional artifact roster: [Stdio manifest](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:25>).
- Hub has the direct non-default dependency [here](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/Cargo.toml:39>), and the WGPU/native paths inherit the same broad library through their current closure. Thus `default-features = false` currently solves installer collision, not compile-surface reduction.

This explains why an otherwise selected Hub `sqlite` receipt and a Space component receipt both spend time in Stdio despite not needing all file-format artifacts at their immediate call sites. It does not establish a global reduction for Hub or WGPU: their catalog/provider construction may legitimately require the full Stdio roster and must be audited separately before changing their feature closure.

## Smallest source-surface split with a demonstrated Space benefit

Space's only production references to Stdio are its Home import/export adapters. They require exactly four typed document families and JSON pretty writing:

| family | Home API used |
| --- | --- |
| CSV | `CsvSnapshot`, `STDIO_CSV_DOCUMENT_SCHEMA` |
| XLSX | `XlsxSnapshot`, `STDIO_XLSX_DOCUMENT_SCHEMA` |
| ZIP | `ZipSnapshot`, `STDIO_ZIP_DOCUMENT_SCHEMA` |
| JSON | `JsonSnapshot`, `STDIO_JSON_DOCUMENT_SCHEMA`, `schema::snapshot::write_json_pretty` |

The production import sites are under [Home IO](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io>) (nine direct `semio_s_plugin_stdio` uses). Its manifest already takes Stdio with `default-features = false`: [Space manifest](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:29>).

The narrow implementation candidate is a clean Stdio **Home-IO artifact surface**, containing only those four taxonomy subtrees and their required shared codecs, and a separate explicit full-catalog surface for the Stdio plugin/catalog hosts. It must change the root's unconditional module inclusion; adding a feature flag while retaining all current `#[path]` modules would not improve compilation. Space can then select `home-io` together with `default-features = false`; full Stdio materialization and any Hub/WGPU caller proved to need catalog-wide descriptors select `full-artifact-catalog` explicitly. This is a greenfield structural split, not a compatibility facade.

Do not apply this slice by removing Space's Stdio dependency: Home's real serializers/deserializers require the named snapshot/schema APIs. Do not loosen guest release LTO/codegen settings or reduce `rustc` threads as a substitute; this audit has no evidence either changes the root amount of checked generated taxonomy.

### First acceptance corpus for that split

1. A source/feature law asserts that the `home-io` root reaches only CSV, XLSX, ZIP, JSON and their declared common codec modules; it must fail if a broad artifact root is again included transitively.
2. Existing schema-first import/export fixtures for those four families run through Space under `home-io`, retaining exact snapshot/schema bytes and JSON pretty output.
3. The full Stdio plugin descriptor/catalog native law still emits its complete current artifact roster under `full-artifact-catalog`; this is the guard against silently dropping required plugins/artifacts.
4. The one-crate Space WASI producer materializes with the narrowed feature selection. Its result may be compared to the four-family fixtures, but no time target should be claimed before a controlled receipt.

The already-known independent native improvements remain narrower: keep Hub's selected runtime gate on its real `sqlite` closure rather than treating an all-feature qualification as its replacement, and make WGPU's declared `--scale` path select a genuinely scale-only closure before compilation. Neither is a substitute for the Stdio root split.

## Session 22205: budget/SIGTERM assessment

The Home report captures the exact interrupted invocation with `CARGO_BUILD_JOBS=1`, a private `home-space-component-sol-target`, `SEMIO_MATERIALIZE_CONCURRENCY=1`, and `SEMIO_BUILD_BUDGET_MS=86400000`: [accepted producer evidence](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-directory-home-browser-process-acceptance-p0.md:90>). The outer Bun/Nx owner exited 143 with `SIGTERM` while Cargo compiled Stdio; no `[budget]` line or compiler diagnostic was retained: [shell bootstrap frontier](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-actual-shell-bootstrap-producer-frontier.md:67>).

The nested Cargo call passes `buildBudgetMs()` directly: [plugin build stage](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:348>). The shared runner uses that timeout with `SIGKILL` and writes a `[budget]` diagnostic on `ETIMEDOUT`: [runner](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts:75>). Therefore a nested 24-hour budget expiration does not fit the observed early outer `SIGTERM`; neither does the later descriptor stage, which only begins after Cargo returns. The evidence supports external/orchestration interruption without identifying a sender. The resumed warm dedicated target is a separate attempt, not a completed receipt for session 22205.

## Scope and nonclaims

This packet makes no speedup estimate and no claim that the proposed feature layout alone fixes Hub or WGPU full-catalog compilation. It identifies a measured active Stdio compilation bottleneck and one source-supported, Space-specific compile-surface reduction. Any implementation must retain the full catalog closure where its descriptor/host admission needs it, retain all four Home IO families, and be qualified with the corpus above.
