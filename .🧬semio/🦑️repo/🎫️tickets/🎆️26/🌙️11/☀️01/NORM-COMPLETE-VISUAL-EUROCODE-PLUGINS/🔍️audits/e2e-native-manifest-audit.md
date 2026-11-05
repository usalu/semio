# Native Norm Manifest Selection

Read-only source review, 2026-11-04. Root Cargo.toml excludes the norm shared workspace and each artifact workspace. The repository cargo launcher cargoManifestV1 selects an explicit --manifest-path or cwd/Cargo.toml; it does not infer a workspace from -p package names. Dependency admission/configuration retains this exact manifest and cwd.

The selection regression belongs to the shared norm workspace. From repository root the correct command is:

```sh
repo --service=off cargo --store=shared --configuration=prototype -- test --manifest-path ✏️s/🔌️plugins/📕️norm/Cargo.toml -p semio-s-plugin-norm-app-surface-panels-inspection --lib interaction_picks_and_stored_checks_share_one_consistent_focus
```

Supply the fleet current session registry/address and shared capacity environment as usual. Source admission/storage/capacity behavior remains unchanged. The earlier root-cwd command without manifest would select the root workspace, where this package is excluded. No build was launched by this audit.

## Artifact Project Targets

| Artifact | Build | Check | Test |
| --- | --- | --- | --- |
| ⚖️en1990 | `repo run @semio-tech/norm-en1990-rs:build` | `repo run @semio-tech/norm-en1990-rs:check` | `repo run @semio-tech/norm-en1990-rs:test` |
| 🏋️en1991 | `repo run @semio-tech/norm-en1991-rs:build` | `repo run @semio-tech/norm-en1991-rs:check` | `repo run @semio-tech/norm-en1991-rs:test` |
| 🏛️en1992 | `repo run @semio-tech/norm-en1992-rs:build` | `repo run @semio-tech/norm-en1992-rs:check` | `repo run @semio-tech/norm-en1992-rs:test` |
| 🔩️en1993 | `repo run @semio-tech/norm-en1993-rs:build` | `repo run @semio-tech/norm-en1993-rs:check` | `repo run @semio-tech/norm-en1993-rs:test` |
| 🧩️en1994 | `repo run @semio-tech/norm-en1994-rs:build` | `repo run @semio-tech/norm-en1994-rs:check` | `repo run @semio-tech/norm-en1994-rs:test` |
| 🪵️en1995 | `repo run @semio-tech/norm-en1995-rs:build` | `repo run @semio-tech/norm-en1995-rs:check` | `repo run @semio-tech/norm-en1995-rs:test` |
| 🪨️en1996 | `repo run @semio-tech/norm-en1996-rs:build` | `repo run @semio-tech/norm-en1996-rs:check` | `repo run @semio-tech/norm-en1996-rs:test` |
| 🌍️en1997 | `repo run @semio-tech/norm-en1997-rs:build` | `repo run @semio-tech/norm-en1997-rs:check` | `repo run @semio-tech/norm-en1997-rs:test` |
| 🫨️en1998 | `repo run @semio-tech/norm-en1998-rs:build` | `repo run @semio-tech/norm-en1998-rs:check` | `repo run @semio-tech/norm-en1998-rs:test` |
| 🪶️en1999 | `repo run @semio-tech/norm-en1999-rs:build` | `repo run @semio-tech/norm-en1999-rs:check` | `repo run @semio-tech/norm-en1999-rs:test` |

Each ID and three targets was read from that artifact package project.json. Its script delegates to runArtifactRustPackageMain with the actual artifact package and explicit package inventory; prefer these registered targets for full artifact verification rather than root-workspace -p selection. Exact manifest ownership remains significant for ad hoc repo cargo commands.
