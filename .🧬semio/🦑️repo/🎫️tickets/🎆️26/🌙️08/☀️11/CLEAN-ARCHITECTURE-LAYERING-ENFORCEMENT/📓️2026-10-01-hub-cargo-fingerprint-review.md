# Hub Cargo Fingerprint Review

Read-only source and retained artifact audit on 2026-10-01. No Cargo/build/generation invoked; no jobs interrupted. Native High supplied exact current route/previous artifact authority. Current route uses live workspace manifests and CWD `/Users/ueli/Documents/semio`; the directory named parent-avi-removal is compiler storage, not proof of copied-source selection.

## Evidence From The Retained Last Attempt

`🗑️generated/goal-stdio/test-artifacts/exact-cargo-laws-iAGjYC/00/build.json` selects live Hub manifest, `cargo test -p semio-hub --lib --no-run --message-format=json`. `/01/build.json` differs only by `--bin os-hub`; library status0, bin status101. Both use the same ticket parent-avi-removal/target directory.

Reading every compiler-artifact JSON line in those two retained build.stdout files gives /00:633 fresh:true and10 fresh:false; /01:632 fresh:true and11 fresh:false. The /00 build.stderr names ten rebuilt owner packages (PDF/DXF/OBJ/Semio/GIS map+terrain and Hub stdio/VCS/GIS/Hub), then reports 8m47. Consequently the latest retained attempt does not establish a full dependency rebuild. Earlier fresh:false observations may be genuine, but need their own attempt artifact comparisons; do not describe elapsed build time alone as a full rebuild.

## Actual Runner Inputs

Repo library/🟦️.ts:1534–1548 composes admitted manifest paths and cargoTargetDirectory; its port prepares the selected Cargo workspace before spawning. Neutral process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts:156–179 preserves configured environment, sets stable CARGO_TARGET_DIR, and creates a unique receipt directory. That unique directory is used for stdout/stderr/receipts, not appended to Cargo build argv. Native test environment is separately composed; RUST_TEST_NOCAPTURE is removed there. Build argv at219 uses exact target plus declared cargoArgs, unchanged profile/features for the inspected Hub route.

Hub package script:5102–5140 uses exactCargoStageEnvironments. Hub staging-environment/🟦️.ts sets build RUST_MIN_STACK to the stable configured override or33554432 and native stack268435456. It does not introduce per-attempt IDs into compiler flags. Cargo metadata recordings currently retain argv/target/status but omit effective build-dir, full environment and toolchain identity; therefore they cannot prove inherited RUSTFLAGS/CARGO_ENCODED_RUSTFLAGS/config overrides remained equal across older attempts.

## Preparation And Generated Writes

Cargo workspace provider/🟦️.ts:181–193 runs authored preparation recipes for all members of reachable scopes, including dependency/dev/build groups, once per recipe per preparation call. prepareCargoWorkspaceInvocation at199–213 invokes it before each Cargo operation when repository workspace metadata authorizes preparation. This is broader than only the selected package's runtime closure. It uses ambient process.env, not the capture environment passed to the exact runner: a concrete environment-authority mismatch worth fixing independently, although no evidence here proves it caused recompilation.

Membership publication at150–171 compares parsed members and returns before writing when unchanged. Entity schema generator `schema/🏷️entity-kinds/🏃️execution/🟦️.ts:28–31` calls neutral writeGeneratedFileIfChanged; process/artifacts/files:6–14 compares bytes and avoids no-op writes. Hub stdio composition/🟦️.ts:184 likewise writes only when previous differs from next. These inspected producers do not explain byte-identical mtime churn. Their actual changed outputs can legitimately invalidate consumers; do not suppress such generation.

## Retained Fingerprint Facts

Intermediate value fingerprints are under target/intermediate/debug/build/semio-framework-value/<hash>/fingerprint/lib-semio_framework_value.json. The specifically reported bf6d39290bdb2b8a and f7373d3d060aa54f have the same features `[default]`, source path identity15620637521541782125, rustflags `[-Z,threads=8]`, compiler/config identities, and value-derive dependency fingerprint12830367260088683805. They differ in profile (16253270031121296954 versus7064039232684141894) and dependency profile fingerprints. They are distinct legitimate compilation units, not evidence of failed reuse of one unit. Older units also retain different path identities and one derive unit has duplicated threads flags; those demonstrate historical configuration/source variants, not necessarily current attempt instability.

Both target fingerprints use CheckDepInfo checksum:true. Repository .cargo/config.toml enables checksum-freshness and separate intermediate build-dir. Byte-identical source mtime rewriting is therefore not supported as the primary cause of these observed units. Native High reports the effective intermediate override as target/intermediate; do not infer it solely from CARGO_TARGET_DIR because default build-dir remains separate.

## Next Exact Diagnostic

Preserve current command/profile/features/budgets and compiler queue. Compare the SAME package/unit hash's retained JSON and dep-info across two corresponding attempts, together with compiler-artifact fresh flags. Future owned receipt metadata should capture effective target/build dirs, relevant explicit environment values (not arbitrary sensitive environment), rustc/toolchain identity, config authority hashes, and preparation changed-output receipts. Cargo fingerprint trace on an explicitly authorized future owned build can identify the exact dirty input; it must not become a parallel compiler probe or cache bypass. Current evidence identifies historical profile/path variants and a preparation environment mismatch, but does not prove a live global cache invalidation defect.
