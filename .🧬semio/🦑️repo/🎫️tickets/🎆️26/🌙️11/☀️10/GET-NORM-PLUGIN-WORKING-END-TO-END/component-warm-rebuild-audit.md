# Warm Standards component rebuilds

## Rebuild and publish a component

Let the existing joint driver finish and release its compiler lane. The component driver requests the same pooled lane class for focused and joint component builds. A following driver can acquire an idle warm lane and reuse compiler units with matching source, features, target and inputs. Starting another driver before that release can allocate a second lane and spend memory and storage on duplicate work.

Run the ticket's native driver with the smallest component selection that needs fresh browser bytes. For the DIN 4108 archive checks this is `norm`.

```sh
SEMIO_UNIT_CACHE_REMOTE=off SEMIO_BUILD_MODE=prototype \
  repo "$ticket/native-exec.ts" . "$ticket/build-play-components.ts" build norm
```

Use the full affected selection when publishing a shared fix to every Standards app. A different component subset can change Cargo's unified features. Check the next invocation's `compiled` and `reused` counts for the observed reuse. Source admission has its own memo for unchanged compilation units. The driver's `warm` generation preserves the shared stage cache.

The normal driver links the selected components, describes and transpiles those bytes, optimizes the cores, probes the result, and publishes the browser modules and receipts. Record `wasmSha256`, `sourceTreeDigest` and `provenanceSha256` from the fresh `🧾️receipt.json`. Confirm the original component digest against its binary. Confirm the browser core and bridge are from the new publication, restore the relevant lane in the Play session and exercise the behavior that distinguishes the change. For the archive ladder, the diagnostic must show the expected grant and the `Replay` failure detail, or the archive must finish if the mismatch was fixed. A newer file timestamp is useful for waiting and does not establish the diagnostic result.

## Determine the affected compiler units

The joint driver's `l/unit-graph-prototype-00.json` lists Cargo units with indexed dependencies, platforms, features and profiles. Its sibling `l/cargo-prototype-00-b0.jsonl` lists actual compiler output paths. For each chosen component, walk dependencies from its root and intersect those units with the reverse dependency graph of the modified crates. Count distinct unit indices. The counts are from the running joint build's `wOltbvh` graph and include its selected roots.

| Changed unit | Affected units for `norm` | Affected units for `norm-eurocode` |
| --- | ---: | ---: |
| `semio-plugin-instance-ladders` | 108 of 812 | 313 of 1,056 |
| `semio-plugin-instance-docs` and `semio-plugin-instance-ladders` | 116 of 812 | 321 of 1,056 |
| Those two crates and `semio-os-store-document` | 197 of 812 | 407 of 1,056 |
| `semio-s-artifact-norm-en1997-examples` | 0 of 812 | 5 of 1,056 |

An EN 1997 example text changes the `include_str!` input of the examples library. The five affected units are far fewer than the archive replay closure. Repeating the component driver preserves the build-script outputs, procedural-macro resources and provenance needed to publish the archive fix.

## Replay a small Rust unit for a local WASI proof

1. Choose the compilation unit in the Cargo graph by package, target kind, platform and features. Read its `src_path`, dependency `index` and `extern_crate_name` entries. Match each dependency against `compiler-artifact` events for that package, target, platform and feature set. Resolve the event's files in the same lane and check that the files still exist.
2. Put a Rust response file and all compiler outputs in the ticket's `🗑️generated` directory. Use the exact repository-selected `rustc` toolchain. Add `-Ldependency` for the matching WASI and host artifact directories. The wasm libraries from this lane place link archives and metadata in separate files. Pass the matching `.rlib` and `.rmeta` for each named WASI library and the host proc macro binary for that platform. Use the unit's source, crate type and features. Keep the trial link and its output separate from the lane.
3. Compile the target library and link the small WASI entry point against the rebuilt library. For a full component the rebuild continues in dependency order across every affected unit. It must retain each unit's exact compiler arguments and build-script output paths and then link the `cdylib` with the same WASI target and settings. Crates that depend on build authority or procedural-macro source reads should go through the repository driver. Advanced experiments can capture the compiler invocation at the actual Rust launch boundary, with Cargo variables explicitly allowlisted, or use Cargo's verbose command output before the invocation. This preserves argument boundaries, configuration and toolchain instead of guessing them from a human-readable process listing.
4. Execute the fresh guest in scratch and assert the new behavior. The EN 1997 proof read all three example source files and compared their bytes to the `PRIMARY_TEXT` constants embedded in the rebuilt examples library. It also checked `direct-shear` and `free-earth` in each embedded source. Compilation took 0.47 seconds for the example library and 5.49 seconds for the WASI proof. This demonstrates the warmed narrow-unit path. The complete browser component still needs its compiler and publication recipe.

The same response-file method built the current EN 1992, EN 1993 and EN 1994 models and their 13 static schema leaves against the joint lane. The three model compiles took 32.3, 90.78 and 11.91 seconds, and the Pack audit linked in 5.61 seconds. That guest verified the six structural binary fixtures without a second component build.

A second WASI experiment changed only a dependency function body and kept the same public signature and `-Cmetadata`. The old consumer with the fresh dependency failed with `E0460`. Recompiling the consumer and linking the root then exercised the new behavior successfully. A root relink uses Rust crate identities and the bodies already incorporated in downstream units. Rebuild the reverse dependency set before claiming updated archive behavior.
