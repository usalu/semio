# Production Rust Adapter Contract

Read-only implementation inventory after the 21 package route publication; the compiler Source is frozen for the Nx qualification run.

## Existing Artifact Model

`rust-unit-v1` already enumerates library, binary, proc-macro, build-script, test and link target kinds, and exact output paths. The Cargo unit projector currently qualifies only a build operation with a single lib/rlib crate type. The descriptor materializer then rejects every non-workspace provenance and propagates that rejection to dependent units. It constructs all supported targets as libraries and declares exactly `lib<crate>.rlib`. The compiler dispatcher calls only `rustLibraryCompilerPortV1` and rechecks the compiler identity whose linker digest is the literal no-linker identity.

This means adding a binary compiler alone cannot qualify ordinary Repo builds. For example, Repo YAML already selects registry `serde` and `serde_json`; their procedural macros/build scripts remain required compiler predecessors. Other Repo products also use foreign dependencies. No foreign-root descriptor/materialization adapter is authored yet.

## Binary and Linker Facts Needed

A genuine binary adapter must retain the Cargo binary target name, source entry, platform/host role, exact effective profile flags, package environment, feature cfg, direct dependency edge names and receipt digests. It must emit the target executable basename, platform suffix and any required accompanying artifacts. It must model actual linker program bytes, effective target linker configuration, its SDK/sysroot/link inputs and flags as immutable facts; the library no-linker sentinel cannot qualify a linked executable. Arbitrary `-C linker` or link-argument paths cannot silently pass through the library flag allowlist.

The output receipt currently records path, content digest and byte size, not executable mode. A executable artifact publication and genuine Nx restoration must preserve and verify runnable permissions. A binary receipt must prove that the restored output can run; content equality alone cannot establish this. Every dependent unit's receipt remains acquired through current reader leases during compilation/linking, and all scratch, linker temp output and published artifacts remain funded inside the fixed Rust image.

Coordinator default package builds require both its library and `semio-repo-coordinator`. CLI default builds require its library, `repo` and `semio-repo`; the repo/mcp entrypoints explicitly request `semio-repo`. Dashboard builds explicitly request `semio` and install it through the dashboard installation owner. These selections must retain their actual binaries; narrowing to `--lib` would change deliverables.

## Foreign and Generated Predecessor Facts Needed

The foreign adapter must bind the actual prepared dependency store provenance/checksum, physical source census and raw Cargo manifest/configuration observations. Its authored source closure must remain admitted under declared source bounds, with portable source identity independent of mutable storage spelling. Absolute downloaded-source paths cannot be relabelled as authored workspace sources or omitted from semantic inputs.

Proc-macro compilation requires the host target triple, actual dynamic-library output and host loader/linker identity, exact feature/cfg/package environment and dependency receipts. Cargo custom-build compilation requires its host executable. Its run unit requires the actual build-script argument/environment contract, bounded generated-output inventory, parsed Cargo directives (cfg, env, link libraries/search paths and rerun Source declarations), and dependent generated/configuration inputs. Dynamic directives must be observed, bounded and incorporated into semantic compiler units before graph publication. A custom-build run cannot be absorbed into a cached aggregate Cargo tree.

## Qualification Sequence

1. Extend the schema/oracle portable fixtures with a small dependency-free binary and actual Cargo/rustc comparison, including prototype and release outputs.
2. Extend direct binary projection/materialization/compiler dispatch with closed linker and executable receipt facts; prove genuine Nx cold/warm/deleted-restore runtime and execute restored binaries.
3. Add independently receipted foreign library, proc-macro and custom-build units with bounded provenance and generated-output Source admission.
4. Run each real package project/default build through its normal editor/Nx route and enumerate every selected compiler task and restored deliverable. Repeat relevant changes and both modes before claiming 21-route production reuse.

Until those adapters qualify, the intent integration correctly makes these frontiers visible and refuses unsupported selected graph adapters. That refusal is a temporary gap and does not fulfill the user's requirement for working ordinary builds. Existing CLI/dashboard compiler construction still has finite compiler/dependency custody but bypasses per-unit Nx; it must not be labelled fully cached.
