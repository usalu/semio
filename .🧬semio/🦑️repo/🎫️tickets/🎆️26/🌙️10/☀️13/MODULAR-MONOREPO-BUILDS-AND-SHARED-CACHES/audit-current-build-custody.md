# Current Build Custody Source Audit

Read-only Source audit, 2026-10-15. Existing ticket remains active. Inspected actual current Nx/Pages and profile/unit/compiler Sources and the two current handback reports. No tests, compiler, build, Native image, cleanup, SDK or physical operation ran; assertions below are static findings, not runtime qualifications. No Storage Native owner audit was duplicated.

## Actionable Finding

P2 — `requireCargoRlibArgumentsV1` validates an exact profile prefix but accepts arbitrary repetitions of every profile-owned flag after that prefix (profile owner line 12). For example, the valid projected `opt_level=3` prefix followed by `-Copt-level=0` satisfies both conditions: every expected prefix entry matches and every argument matches the broad allowlist. `readRustUnitV1` forwards this accepted vector; the actual compiler appends `unit.arguments` unchanged (compiler line 25). This allows accepted profile DATA and effective compiler options to disagree, contrary to the effective-profile closure contract. Arguments are included in semantic hashing, so this finding does not assert a digest collision. Restrict the suffix to the separately owned fixed flags (currently `-Zthreads=1`) and reject appended profile flags, duplicate profile switches, and edition switches. Add neutral refusal cases and independent validation for the suffix boundary.

## Nx Contract Gap

Nx `withOutputCustodyV1` line 47 adds all five member paths to the declared paths but does not add `cohort.root`. Aggregate declarations obtain aggregate plus five members as reported; member-only or descendant-only declarations obtain five members plus their declaration, without the aggregate artifact lease. This falls short of the requested unconditional aggregate-plus-five custody shape. The shared cohort group still serializes the inspected Pages producer/reader and Nx cohort operations, so this audit does not claim a demonstrated race from this omission. Include `cohort.root` explicitly if that exact contract is required; cover aggregate/member/descendant lock rosters independently of matcher tests.

## Inspected Positive Boundaries

Nx snapshots input paths before its first await; selects actual Original buildMode through `readNativeCallerAuthorityV1`; opens the actual products store; retains group SH for put (cache mode exclusive) and EX for restore (cache mode shared) before artifact acquisition; refuses nested SH-to-EX and new-group acquisition after held outputs. Private AsyncLocalStorage state is used rather than public lease data. Pages imports the same pure cohort owner and uses its root/resource for actual reader and publisher custody. Cohort data/member objects and arrays are frozen.

Profile reader copies full schema-owned DATA, freezes nested strip structures and the profile, and planning replaces the input profile reference with a copied/frozen profile before canonical digest calculation. Full normalized unit enters semantic digest, and receipts compare that digest. Observed off/false/thin/fat mapping matches the stated Source mapping. Projector refuses host/dual roles, wasm, unsupported kinds/operations/backend/defaults. Compiler appends incremental beneath checked funded temporary storage after reservation recheck, with no accepted descriptor path. These are Source observations only. Complete runtime acceptance listed in the handbacks remains pending.

## Source Census

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts`: 21991 bytes; SHA256 `b96c16e147b567db308964051386581aae220ec254f629a0b40e6cc814191806`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/📚️cohorts/🟦️.ts`: 3103 bytes; SHA256 `1af520370d82f6a73fb501f58b275e8242d6964b0424c03c431d1b0929f6a020`.
- `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/📦️publication/🟦️.ts`: 15237 bytes; SHA256 `98c2f67498dd65dbfea5cb271fa77cbe7ac81027465d46d1c0400e7c11dc5ee6`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🟦️.ts`: 4141 bytes; SHA256 `27b5e4599aa959985eb64720f36b717caf33b8df8ae9514afa3b63041df7ba8c`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🟦️.ts`: 9610 bytes; SHA256 `e3a5c3e7567c04609c159895b73dfc0b2f4a995de00793e81ed9439328dfc693`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🟦️.ts`: 8045 bytes; SHA256 `b5eb6a555e72a98aefc981a2240dafee2ab0b1239c89eb944569e8f1a11d1624`.

## Focused Final Source Rereview

2026-10-15. No new scoped actionable invalidation or custody bypass found in the inspected final paths. This supersedes the two earlier findings for these snapshots. No tests or physical actions executed; report evidence is static Source inspection.

The final cohort plan unions each declared product output with matching aggregate and all five members, freezes the roster, and the actual Nx wrapper unions that plan with the complete original paths. This retains Rust outputs as well as product declarations; group acquisition remains before exact artifact acquisition with actual Original mode and nested refusal checks. The member/descendant aggregate omission identified above is corrected.

The profile owner now schema-validates the entire suffix after its exact projected prefix. SchedulerSuffix requires exactly one `-Zthreads=1`; appended profile/edition switches and duplicate scheduler suffixes cannot satisfy that schema. Actual graph materialization and standalone constructor both use cargoRustUnitArgumentsV1; materialization preclassifies unsupported observed flags and retains custom cfg separately. Complete profile DATA still enters accepted units and semantic digest. The earlier profile override finding is corrected.

The extracted default configuration owner retains manifests, lock, ancestor config/toolchain and defining code in planningInputs while returning defining code only as compilerInputs. The actual selection callback merges complete planningInputs into outer freshness and returns compilerInputs to the materializer. Source inputs, including a genuinely consumed manifest, are independently merged by the materializer; no omission is inferred from compilerInputs alone. Actual selected producer uses the extracted transitive slice encoded bytes and digest to publish its descriptor and build its target, rather than a whole-selected snapshot. Duplicate/missing/cyclic slice predecessors refuse. Standalone constructor still includes broad ancestor configuration as expressly scoped in the handback; this is not claimed corrected.

No claim is made about filesystem races, physical cache restoration, compiler semantics, live Native hashing or runtime invalidation. Those need the separately authorized coherent capture and execution.

### Rereview Source Hashes

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts`: 21979 bytes; SHA256 `29dcb01dc40f49c87c92dd75c2147666d4c9988b27547621525b43aa671d45c7`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/📚️cohorts/🟦️.ts`: 3642 bytes; SHA256 `34d79ef21c0d11873105a698dd62d0825d934ce6f103956231d970b6164c6e53`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🟦️.ts`: 3782 bytes; SHA256 `8ee542e739c6bc79140a30b34cd9050dd677f455e3bdc74dc2f8d7b83a6464c3`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧬️schema/🔣️.json`: 2962 bytes; SHA256 `eb52b4f4d5a7bd0e50c86155c494010931f1ad7bd621a2c75fd8d3acb22e0f8c`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🟦️.ts`: 6617 bytes; SHA256 `95d82afff0fa59294e3f2c54bbefcaa11362f51bd201161deb040f8ef68e257d`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🟦️.ts`: 9251 bytes; SHA256 `910e222d741896bcfcdb27d9acdbf205fee0d9e0c7bb409b9c5752ee58fc71db`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/🟦️.ts`: 12485 bytes; SHA256 `ac06b7a4cb2c8053d7d74573f6f9ef9f85e1bf63240873cb64a23a4942180e4d`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🟦️.ts`: 2853 bytes; SHA256 `9a7e3ec56b0ca30c7580dea5ceaabae7514709dbe413fb4e833a2cf3cb494308`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📤️slice/🟦️.ts`: 1172 bytes; SHA256 `7910aa1a84a1095df8515184662c2e828a73856ba159216b6612095acb0ce7f2`.

## Shared Core Archive Source Audit

2026-10-15. No concrete new correctness/bound/availability regression established in this read-only Source pass. Inspected Core decoder, request/policy schema, neutral corpus/tests and actual SDK wrapper/policy. No test, compiler, tool preparation, download, Native, network or physical archive operation executed. Sources remain moving; the hashes below identify this inspection only.

The decoder validates/snapshots closed request and bytes before its first yield; compressed, inflated, per-file, header-count, path and per-entry metadata ceilings are present before entries return to materialization callers. Tar checks framing/checksum/footer/padding, regular nodes, bounded PAX/long names and normalized duplicate/ancestor paths. ZIP bounds its central roster, reconciles local framing/name/size/method/flags, requires a contiguous local layout, handles signed or unsigned descriptors with an unambiguous match, bounds inflation, checks exact size/CRC and rejects unsupported ZIP64/encryption/node classes. Actual SDK wrapper passes its own closed existing policy and explicit package prefix to the shared owner; it adds no materialization authority.

Inflate attaches abort-driven stream destruction, awaits finished in normal and finally paths and retains rejection handling. Header/CRC/inflate loops check carried control and publish/yield at bounded intervals. This is static drain-path evidence, not executed cancellation qualification.

### Focused Additional Neutral Vectors

These are coverage suggestions, not demonstrated bugs: (1) cancel genuine control during multi-chunk gzip and raw-deflate inflation, assert rejection and stream completion before returned callback exits; (2) abort during CRC of a stored ZIP member exceeding 1MiB, including a canceled publication/yield; (3) append a byte to an otherwise valid raw-deflate member, update compressed extents/central inventory but retain CRC/plain size, assert compressed trailing-byte refusal; (4) central count valid but physical local extents overlapping, gapped, or reordered, assert refusal; (5) checksum-valid local PAX path/size, stacked metadata, oversized metadata and dangling metadata at tar footer; (6) ambiguous descriptor signature/CRC boundary with CRC 0x08074b50, compare independent ZIP reader behavior and require exactly one framing interpretation. The current tests already cover positive signed/unsigned descriptors, mutation snapshot, many framing/path/size/collision refusals and independent Bun/fflate/SQLite observations; these suggestions target remaining cancellation/metadata ambiguity boundaries.

### Archive Inspection Hashes

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🟦️.ts`: 16437 bytes; SHA256 `066c7732737e38d10b7deb257860a47930c6c3f9dcfa4290c09ad1eb30254a54`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🧬️schema/🔣️.json`: 1429 bytes; SHA256 `12b1c9e307887c531a77d26bf3858f1d93fd04061f9ccd07b3f49910feea5832`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🧪️tests/🟦️.ts`: 9757 bytes; SHA256 `c9918f21ebaa2829805dc698cd2fd70d223411b838ccc3b2f6097fede271d2ec`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🧪️tests/🔣️.json`: 1240 bytes; SHA256 `6375263a034637513b9a4ae3ce6ed2a7604dfd7fb6cb92d35eb8449e32a89544`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🗃️archive/🟦️.ts`: 1299 bytes; SHA256 `df9165b17202f4e377bf6c46ea15d9d414da24b752d7d69b3e4ad5c74d2db3bb`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🗃️archive/📋️policy.json`: 203 bytes; SHA256 `1d624c5af534212f718ed988711ea733302730ce92733aef3e9f5869b18fa4a2`.

## Existing Mounted Borrower Whole-Operation Rereview

2026-10-16. No concrete whole-lifetime/control/deadlock issue found on the inspected existing-mounted macOS borrower route. Source-only independent review; no tests, physical probe, image, command or fixture execution. Read both Native cleanup handbacks. Fresh/inactive persistent design and physical acceptance are excluded from this conclusion; moving Source elsewhere is not implicitly approved.

Core acquire/recover/checkpoint/observe synchronously enter port.withOperation before asynchronous custody work. Reservations bind their later recheck/commit/retire to original options.control, while each journal transaction receives its current actual operation control. The returned custody transaction takes that control explicitly rather than retaining the completed acquire child. Retirement wrapper also retains the actual port operation.

The mounted provider authenticates its exact issued SH via the private borrower before returning a port. Observe/recheck/retire and public recheck enter that borrower. Private frames enroll accepted work synchronously, retain parent pending sets, close births and drain descendants before frame completion; failures including undefined remain represented. Closing from within its own accepted frame refuses. Close drains accepted work before exact SH release. A maintenance transition first drains/releases the SH, then obtains a distinct EX under its genuine maintenance child, accepts a separate owner, drains it before EX release, and is joined by image.close. The considered non-maintenance compiler-store context retains its image until its awaited workspace/body path exits and then awaits image.close in finally.

This inspection does not establish successful existing-mounted opening under current kernel inventory, actual compiler-store runtime, canceled child settlement on macOS, physical retirement, fresh creation or inactive activation. Those require separately authorized execution; the earlier 42/254 and strict results remain handback evidence rather than results rerun here.

### Existing Borrower Inspection Hashes

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`: 16972 bytes; SHA256 `ff1273e17909903bc8a46626bd3b066ebf3d3531f3951b86780fcf2dea47d86e`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts`: 6323 bytes; SHA256 `d17d831d310166ba42abc97a9167224f04008d35e8e7ef4d02c4f6d4a2f9a04b`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts`: 26991 bytes; SHA256 `80939af5ed9e766477fd6996e88c650e6df52b4b7ee892bf20c01870672c886b`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts`: 8841 bytes; SHA256 `4b7699c44d0809177226df4acd155ac7167d4eee61824c423d40b281e4aecde7`.
