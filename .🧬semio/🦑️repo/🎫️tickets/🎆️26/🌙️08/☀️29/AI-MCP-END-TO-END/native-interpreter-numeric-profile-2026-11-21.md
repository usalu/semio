# Native Interpreter Numeric Profile

The managed Norm descriptor emitter was sampled at 20:11 CET after 15 minutes of elapsed execution. Its main thread had 2,117 samples under `CoreInstance::step`, with 728 under the parent interpreter's `execute_numeric` call. The sample shows separate frames in the numeric leaf for `execute_numeric_fixed`, `dispatch_numeric` and operand popping. The managed descriptor emitter uses the same Core interpreter as the native MCP runtime. The sample describes Norm descriptor emission.

The benchmark compiled the dependency-free `semio-plugin-interpreter-numeric` crate with the pinned `nightly-2026-07-20` compiler, once at O0 and once at O3. Both builds used identical debug-assertion and overflow-check settings, matching the prototype policy. The same optimized harness called the crate's public entry point for six million i32 and i64 add, multiply, XOR and rotate operations per run. Nine runs alternated which binary went first. Each run matched the Bun WebAssembly oracle's exact final values of i32 `968880972` and i64 `-6724291103802427519`.

| Numeric leaf | Median for 6,000,000 operations |
| --- | ---: |
| O0 | 851.249 ms |
| O3 | 60.404 ms |

The O3 leaf took 92.9% less time in this workload, a 14.09× speedup. The numeric dispatch workload isolates the leaf, so the sample and timing do not establish the duration of a descriptor emission or an MCP operation.

The canonical policy now selects O3 for the numeric leaf beside its parent in the repository, runtime and streaming groups. The managed descriptor emitter and native MCP manifests both resolve to the root repository workspace, where their prototype role now selects O3 for both crates. Their release roles already establish an O3 target default. The ticket's direct build fallback reads both selectors from the same policy. The language-neutral inventory accounted for the new constraint across the 142 affected workspace entries.

The new native-role check reproduced numeric O0 before the change. Afterward, eight scoped checks passed, including the profile inventory, policy schema, independent TOML parsing, SQLite, effective selectors and artifact roles. The broader run failed the existing test `neutral consumed Cargo actor witnesses require the full actual compiler tuple` at `expect(actual).toContain("--message-format=json-render-diagnostics")`. The failing `actorArguments` fixture row is the release component `neutral-actor` in `🌎️hub/Cargo.toml`, whose current command emits `--message-format=json`. The numeric selector change applies only when rendering prototype overrides. The actor fixture and message-format path were left as found.

The reproducible benchmark inputs are `numeric-profile-witness.json`, its schema, `numeric-profile-witness.rs` and `numeric-profile-witness.ts`. Run the TypeScript entry with `repo tools exec bun`. It builds only the leaf and witness under the ticket's generated directory. No managed Cargo root was written to or rebuilt during this qualification.
