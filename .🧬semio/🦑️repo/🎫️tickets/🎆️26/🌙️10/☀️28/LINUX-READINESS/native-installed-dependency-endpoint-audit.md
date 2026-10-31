# Installed Native Verifier Dependency Endpoint Audit

Read-only audit using retained scripts/reports/logs and current configuration. No container, network, setup, compiler, graph or native query was run; no Source was edited or copied.

## Exact retained failure

native-installed-postboot.outcome.json records PID297424, status1/signalnull, ended2026-10-30T03:32:58.726Z. The registered container command invoked the existing semio-linux-readiness-source at /workspaces/semio, then /home/vscode/.local/bin/repo tools exec --light node -- <ticket>/native-layout-installed.cjs. Node24.15.0 failed MODULE_NOT_FOUND for ajv/dist/2020 at helper line1, with only that helper in the require stack. This occurred before schema, installed Receipt or executable bytes were read.

It proves neither receipt corruption nor a changed native binary, and it is not a successful receipt/native witness. Node did start through the registered tool route.

## Resolution assumption versus owned material

native-layout-installed.cjs is a CommonJS script below /workspaces/semio. Its bare require walks ancestor node_modules directories, including /workspaces/semio/node_modules. Root package.json declares Ajv8.20.0. Moving the require base to createRequire(root/package.json) would reach the same root endpoint, so no retained evidence justifies that helper change. Node's working directory alone is not the resolution mechanism; ancestry already includes the root.

The helper intentionally does not provision dependencies. The Node tool registry describes a runtime and an inherited environment, not ownership of every package a script may require. A working repo tools exec --light node therefore cannot establish an installed root package generation.

The observed blocker is unresolved/unqualified private JavaScript material. The saved error alone cannot distinguish an absent endpoint/package, stale binding, wrong installed major, or incomplete package body; there is no postboot endpoint target/version census in this receipt. It is consequently too strong to claim that the entire private dependency store is absent. It is also unsupported to blame an incorrect module-resolution base.

## Private container topology and retained proof

source-projection-audit.md and host-capabilities.md document replacement of the old node_modules volume after an actual same-filesystem preservation refusal. The retained private Podman Source:O view protects host Source writes and keeps only the private repository/native cache volumes. Source node_modules paths must be ordinary replaceable managed generation projections, not separate mountpoints or borrowed host-generation links.

The final private view was established against320endpoint identities from all six scope manifests/package members. Only inherited host-generation symlinks were removed inside the isolated overlay; all host identities were unchanged. Valid private generation links were retained if present. That proves historical isolation and endpoint cleanup, not their current postboot target or installed package completeness.

The old container-setup-endpoint-volume.log ended at Same-filesystem ordinary preservation Source required. The separate package-endpoints attempt was deliberately cancelled with an incomplete-sync report. These are historical failed/cancelled qualifications, not evidence that a current complete root generation exists. The temporary container-dependency-store.tar archives an earlier proven inactive/unbound namespace (dependency-budget-audit.md); restoring or extracting it is not a package provisioner or an authorized fix.

Recovery207458 restarted the exact existing stopped container. It did not create a fresh devcontainer, execute current create hooks, qualify the root dependency binding, or freeze all Source. Source:O visibility and cache persistence must not be conflated with a current endpoint receipt. No statement about overlay persistence after reboot is inferred without an actual current mount/binding observation.

## Safe next owned canonical action after the controlled gate

Preserve the current schema/fullgraph owner. Once Root schedules container work, use the existing run-container-command.cjs wrapper with unique labels. Keep Docker and dependency preparation normal; light belongs only to bounded Node observation.

First qualify the private root endpoint through a standard-library-only registered Node observation: lstat/readlink/realpath of /workspaces/semio/node_modules, the resolved Ajv package version/dist entry, and its relation to the private /workspaces/semio cache stores. Do not follow a borrowed host endpoint into a host dependency generation, copy packages, relink manually, mount node_modules, restore the tar, or add NODE_PATH. No dependency-importing verifier should be used to perform this initial qualification.

The existing public provisioner is exactly repo dependencies ensure. Current operation catalogue marks it requires:none, progress/cancellable, with no selection override. Its captured dependency kernel calls ensureBuildEnvironmentV1; SourceOwnerBootstrapProtocol scope is ".". A stale material receipt invokes capacityBuildEnvironmentInstallerV1 and the normal complete locked JavaScript producer under actual dependency-store custody. It is not necessary to launch workspace:setup or a general target graph just to request this dependency qualification.

The next provisioner command, if the current endpoint is unready, is:

~~~sh
SEMIO_LANGUAGE=en /home/deck/.local/bin/repo tools exec --light node -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/LINUX-READINESS/run-container-command.cjs' native-private-dependencies-current /home/vscode/.local/share/semio/launcher/bin/repo dependencies ensure
~~~

Use the existing installed stage-zero launcher, avoiding reliance on an as-yet-unverified native Receipt for provisioning. Do not rebuild or replace it merely for this observation. A real refusal remains a causal diagnostic; never disable the ownership, Source-generation, path or preservation guards.

After a genuine provisioner0 and requalified private endpoint, rerun the retained installed Receipt verifier through the same wrapper with a fresh label and /home/vscode/.local/bin/repo tools exec --light node -- <ticket>/native-layout-installed.cjs. Preserve all Ajv/closed-schema, single-link stable-file, executable mode, ELF and digest checks.

Installed Receipt0 proves the observed installed bytes and Receipt agree, not complete current Go/SDK provenance. Current native-query acceptance additionally requires Root's Source quiet-window/current contributor qualification, a canonical repo-mcp:install when the saved build closure is stale, and the separately owned native-layout-acceptance.cjs plus current MCP/scoped/cancellation controls. Preserve every manifest-listed shard/hash/physical identity, actual limits and protected caches. The earlier warm container remains a warm recovery context; cold devcontainer acceptance is still separate.

This report proposes the exact owned sequence; none of these actions was executed during the audit.
