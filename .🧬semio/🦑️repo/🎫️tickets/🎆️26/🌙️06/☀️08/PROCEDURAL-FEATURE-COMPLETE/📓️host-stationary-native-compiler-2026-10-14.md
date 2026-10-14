# Stationary Render Native Replay

Exact registered original-browser-output native PID 69402 ended naturally with Nx 1. Strict TypeScript and the nine source laws passed with zero failures and 142 assertions. Selected native assertions were not reached. Cargo stopped in semio-framework-job library code generation because an incremental cached CGU object was missing. No cache reset, deletion, authority widening or unchanged retry was issued. The defining original BuiltTree closer remains authored and native-unqualified.

```text
   |                ^^^^^

error: cached cgu 8hxgf8wmrhr8wc6spyphnbx1a should have an object file, but doesn't

warning: `semio-framework-job` (lib) generated 11 warnings
error: could not compile `semio-framework-job` (lib) due to 1 previous error; 11 warnings emitted
warning: build failed, waiting for other jobs to finish...
[cargo:build] running elapsedMs=10004
warning: method `original` is never used
 --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../⚔️conflict/♻️retirement/🦀️.rs:8:8

warning: `semio-framework-job` (lib) generated 11 warnings
error: could not compile `semio-framework-job` (lib) due to 1 previous error; 11 warnings emitted
warning: build failed, waiting for other jobs to finish...
[cargo:build] running elapsedMs=10004
warning: method `original` is never used
 --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../⚔️conflict/♻️retirement/🦀️.rs:8:8
  |
6 | impl<T:RetireOwned> OriginalProtocolOwner<T> {
  | -------------------------------------------- method in this implementation
[native:owner-command] running elapsedMs=120042
warning: `semio-framework-replication` (lib) generated 1 warning
error: command `/Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/cargo test --no-run --message-format json-render-diagnostics --package semio-framework-os-renderer-wgpu --lib --profile prototype --offline` exited with code 101
38 |       child.once("close", (code, signal) => accept({ code, signal }));
39 |     });
40 |     const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
41 |     if (!failure) return;
42 |     if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}`);
43 |     if (options.throwOnFailure) throw Error(failure);
                                           ^
43 |     if (options.throwOnFailure) throw Error(failure);
                                           ^
error: cargo exited with status 101
      at runBudgetedTestCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts:43:39)
      at async runCargoTestsV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts:172:343)
      at async runRepositoryCargoTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1379:11)
      at async run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:123:69)

Bun v1.3.14 (macOS arm64)
142 |     stopProgress();
        @semio-tech/framework-renderer-wgpu:test-original-browser-output-native:prototype    2m 9s
[script] {"owner":"procedural:Host:original-browser-output-native","command":"nx","stage":"complete"}
error: script "nx" exited with code 1
```
