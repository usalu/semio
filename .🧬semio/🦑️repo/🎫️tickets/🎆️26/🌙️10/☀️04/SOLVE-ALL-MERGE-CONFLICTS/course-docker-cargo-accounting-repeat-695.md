# Course Docker Cargo Metadata Accounting — Repeat 695

Disposition: the old Quiz Dockerfile.cargo.toml is intentionally retired by the incoming Course deployment architecture. It has no receiving Docker-local file. Native compiler configuration remains at the repository compiler owner; runtime image packaging now accepts the authenticated native executable. No source file was written in this investigation.

## Immutable Input Evidence

- H-only path: 🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile.cargo.toml. The whole-input census records HEAD blob f0e31314ee6a584f49ba1c845cf3b1731a368c8b. This file is absent in both B and S, so S cannot contain an explicit deletion of this H-only addition; retirement follows the receiving architecture and current authored absence contract.
- Immutable H Quiz Dockerfile copies this file into /usr/local/cargo/config.toml in a Rust builder stage. That stage excludes repository .cargo and invokes Cargo directly; the small config exists solely because the old container deliberately bypassed the launcher-only compiler ingress.
- Immutable S Course Dockerfile is a single distroless runtime stage. It copies proctor, course content, stack files and data; there is no Rust builder stage, Cargo invocation, Rust toolchain or Cargo configuration copy.
- Immutable S Course deploy image command first calls buildProctor with release mode, checks the executable against current course data, enters artifact.withExecutable, and supplies a finite tar context to Docker. The context roster contains Dockerfile, ignore rules, executable, content, stack files and data only.
- Current Course deploy test explicitly requires Dockerfile.cargo.toml absent at 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🟦️.ts:261. This is source evidence only; the test was not executed.

## Actual Receiving Owners

- 🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🟦️.ts:465–472: builds the native release through buildProctor and packages its authenticated executable through courseImageContext.
- 🎓️teaching/🏛️architecture/📚️course/🚀️deploy/📦️image/🟦️.ts:110: courseImageExecutable accepts only native Linux release executables and preserves the original custody/recheck.
- 🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts:270: buildProctor enters canonical buildCargoArtifacts with repositoryCargoArtifactBuildPolicyV1, captured native storage, current source boundary and caller control.
- .cargo/config.toml retains the actual compiler settings formerly copied into the container. Its original launcher/compiler ingress applies to the native builder; no alternate Docker Cargo owner is needed.

## Full Setting Ledger

| H Docker-local setting | Current disposition |
| --- | --- |
| build.build-dir = /cache/build | Retired container cache-mount path. Canonical native builder owns build storage; current .cargo/config.toml names repository shared native build storage and the admitted build recipe controls its actual directory. |
| build.rustflags = -Z threads=0 | Exact current .cargo/config.toml value retained. |
| unstable.embed-metadata = DoNotEmbed | Exact current .cargo/config.toml value retained. |
| unstable.fine-grain-locking = true | Exact current .cargo/config.toml value retained. |
| unstable.build-dir-new-layout = true | Exact current .cargo/config.toml value retained. |
| unstable.checksum-freshness = true | Exact current .cargo/config.toml value retained. |
| Linux GNU threads=0, linker-features=+lld, link-self-contained=+linker, unstable-options | Exact current target configuration retained. |
| profile.release opt-level=3, lto=thin, codegen-units=1, strip=debuginfo, incremental=false | Exact current .cargo/config.toml release profile retained. |

All non-container-path values were compared as parsed TOML structures and matched exactly. No obsolete Docker builder config is restored.

## Concurrent Launch Deletion Accounting

Parent reported that .vscode/🧩️launch.seed.jsonc was deleted externally after the source pass. This agent did not delete either launch file. The current dashboard authority source at 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts:350 requires both .vscode/launch.json and .vscode/🧩️launch.seed.jsonc absent. Current read-only snapshot: [{"path": ".vscode/launch.json", "exists": true}, {"path": ".vscode/🧩️launch.seed.jsonc", "exists": false}]. The external event and current authority requirement are recorded separately from this merge agent write ledger.

No build, compiler, typechecker, tests, runtime, resolver, generator, Git mutation or catalog write was performed.

Final frozen observation: launch.json is still present (SHA-256 88ab146ca1aadb292d00ea8b7624dcc21addf434448068cdc6eb56e8d4a8563b); launch.seed remains absent. The fresh authority assertion is now at line 352 and still expects both absent. No launch bytes changed during final verification. See catalog-final-verification-repeat-695.md.
