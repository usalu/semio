# Descriptor emitter build failure, 2026-11-27

## What broke
The base publication (`chain-base.log`, 01:44) ended with "Descriptor emitter prerequisite build failed: Cargo artifact build failed: ...describe/.../rust/Cargo.toml". The artifact build's own diagnostics file (`.cache/b/r/m/u/l/q7134ea/semio-cargo-logs/cargo-artifacts-OAr0sE.log`, 01:43) is 0 bytes: no rustc error was ever produced, so the build was killed or cancelled (memory warden or concurrent edit race), not a compile failure.

## Verification
- `repo run @semio-tech/os-plugin-describe-rs:build` could not give output: after a 20 min capacity wait the project-graph read died with the unrelated repo-tool error "Actual declared requested Source scope required (7566ad79)" (`🗑️generated/describe-build.log`).
- Instead: `repo capacity run --memory=2600 --cpu=1 -- repo cargo --store=shared --configuration=prototype --parallelism=1 -- check --offline --locked --manifest-path <describe Cargo.toml> --lib --bins`. Result: rc 0, `Finished prototype profile` in 5m 57s, zero `error` lines; `semio-framework-job-context` (facade) and its step crate compiled (`🗑️generated/describe-check.log`).
- The explicit facade re-export list is complete for this consumer; no source change was needed.

## Changes
- `publish-chain.sh`: an "Descriptor emitter prerequisite build failed" result is now retried up to 5 times per step (60 s apart, attempt log kept); a sixth failure is final, so a real compile error does not loop. Written atomically; the idle old runner (pid 55827, in the energy spare gate, no child) was stopped and a new one launched with nohup (`chain-runner.log`), starting again at base.

## Result
Descriptor emitter check rc 0; chain relaunched.
