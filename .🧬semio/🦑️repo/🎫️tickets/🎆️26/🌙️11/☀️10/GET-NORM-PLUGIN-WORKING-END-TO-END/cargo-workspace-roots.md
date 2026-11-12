# Cargo workspace roots and the prototype profile

Decision: option (b). Nested workspaces are intended. 293 tracked or unignored manifests declare their own `[workspace]`. They are isolated probes, oracles, generators and codec crates (their sources say "own `[workspace]`, isolated"). Folding them into the enclosing workspace is wrong.

Cause: the profile policy (`renderCargoProfileConfigurationV1` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts`) renders `profile.<mode>.trim-paths="object"` for every root, including isolated ones. Cargo accepts that key only if the root declares `cargo-features = ["trim-paths"]`. 190 roots had it; 103 did not. No generator emits these manifests, so the roots are hand-authored and the policy has no single source to fix.

Fix:
- Added `cargo-features = ["trim-paths"]` as the first line of all 103 roots that lacked it. That includes the trinity graph-query root, the hub stdio oracle roots, `os/config/bridge` and equation/draw/fem/wfc/space/sourcing roots. No Cargo.lock change.
- Added the language-agnostic lint `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-workspace-root-features/🟦️.ts`. It asserts that the policy renders trim-paths for prototype and release. It lists manifests through `git ls-files` and parses them with `@iarna/toml` as the third-party oracle. It fails when any workspace root lacks `cargo-features` with `trim-paths`.
- The lint takes about 4 minutes when run through `repo tools exec bun -- test ./<file>`, most of it repo boot time. The test passes 2/2. It needs a longer bun timeout, because one early run hit the default 5 s test timeout.

Verification: `repo cargo --store=shared --configuration=prototype --parallelism=4 -- metadata --no-deps` exits 0 in the trinity graph-query root, the hub stdio oracle root and `os/config/bridge`. I did not run it in the other 100 fixed roots. The lint covers them. A one-off script confirmed 0 roots lacking after the fix.

Notes: the lint skips tickets, target and .cache paths. A root lacking features is a hand-authoring error. The lint is the guard, so new roots need no generator change.
