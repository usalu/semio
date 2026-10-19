# Private Builder Tool And Daemon OS Inputs

## Scope And Completed Source Alignment

Course preflight alignment is implemented and its scoped source test passed first on Bun, then on actual Node. The detailed seven-file change and red/green evidence is in `2026-10-19-course-preflight-alignment.md`. Runtime's broad Source capture worker was notified of the final helper edit and retains the full Source hash fence; no capture result is relabeled after a later change.

The prepared initial OS image contained cc/pkg-config/Docker CLI/Compose but lacked Git, make, g++ and curl. This was checked from the authenticated Debian base layer's executable roster and the exact installed package records, without launching another OS container during source alignment. The existing deployment `sourceRevision` uses genuine Git revision/time/status, and the authored devcontainer recipe includes build-essential and curl. These tools are legitimate OS prerequisites; their presence does not supply original project compiler authority or repository dependencies.

## Tools Snapshot: Terminal Success

Root authorized a new source-free private OS snapshot based on immutable image `sha256:04ed9880ecae29f4b9954f72f498b2da9b3b58e4fc4b482d193b64e4a38118e0`, preserving that old image for runtime Source transport. Authenticated Debian metadata selected and pinned Git `1:2.39.5-0+deb12u3`, build-essential `12.9` and curl `7.88.1-10+deb12u15`, with make/g++/libc development dependencies. All 46 exact package versions, SHA-256 hashes, archive sizes and installed-size estimates are retained as `2026-10-19-builder-os-tools-packages.json`.

Before download, the plan bounded 41,727,916 archive bytes / 197,585,920 installed-size bytes, 20,019,339 metadata bytes, 459,114,728 total conservative input bytes and 1,329,000,943 conservative temporary bytes. This fit the then-current 1.25 GiB / 512 MiB caps. The setup completed exit 0, with actual package-query console evidence and read-only Docker/Compose version queries. It invoked no repository dependency installation, project compiler, Store, image build or inner daemon.

After removing only the private guest's apt cache/lists and transferred Bun, actual new writable bytes were 187,793,408 and actual expanded root bytes 527,831,040. The source-free snapshot completed at `2026-10-19T12:46:27.581Z`, tag `semio-course-preflight-os:sol-tools-20261019`, immutable image ID `sha256:3b79ef2921a597b6e0906f5627fd0b283273748f79235eac11464cdfe5fe43cf`. Docker's compressed image accounting is 181,663,837 bytes. Its input is `2026-10-19-builder-os-tools-input.json`; the private container was removed and the tools setup session is terminal. Earlier immutable OS inputs remain unchanged.

## Actual Operator Topology And Daemon Plan

The actual authored image check publishes a container port and fetches 127.0.0.1 from its caller; Compose also resolves Caddy/certificate binds inside the private workspace. A Docker CLI inside a builder connected to the external Desktop daemon therefore does not establish those operator checks: daemon/client localhost and workspace paths differ. The external daemon remains valid for transporting this task's images and volumes. Root will own a genuine daemon inside the eventual privileged Debian builder, with the real Source volume and caller-local paths, so the unchanged project checks can run.

The tool guest simulated (without installing) authenticated pinned Docker CE `5:29.9.1-1~debian.12~bookworm`, containerd.io `2.4.2-2~debian.12~bookworm` and iptables `1.8.9-2`. Fifteen packages add 48,960,064 archive bytes and 210,170,880 installed-size bytes. Including tools, the signed-metadata plan projected 1,798,302,767 conservative temporary bytes and 508,074,792 input bytes. The former exceeded the old OS cap, so daemon packages were not downloaded or started in the tools step. Full simulated records are labeled separately from installed tool phases in the retained packages input.

The authenticated Debian base already provides `/usr/sbin/policy-rc.d` ending in exit 101. The final OS step explicitly checks that existing service-start refusal before installing daemon packages, then checks its actual process namespace for dockerd/containerd and refuses unexpected startup. No substitute service script or product workaround is created.

## Official Node And Final OS Budget

Root additionally authorized actual Node 24.15.0 before the managed Nx authorizer is compiled/loaded. The existing authored binary recipe is `.devcontainer/Dockerfile`: it downloads official Node and checks an architecture-specific SHA-256, then extracts only the Node binary. The private OS recipe follows that input contract; no suitable source-only managed Node owner was found before dependencies.

The official [Node 24.15.0 checksum list](https://nodejs.org/dist/v24.15.0/SHASUMS256.txt), fetched over verified HTTPS, agrees with the authored x64 archive pin: `472655581fb851559730c48763e0c9d3bc25975c59d518003fc0849d3e4ba0f6`. Official archive HEAD reported 31,164,460 bytes. The recipe checks that exact length and SHA-256, bounds and reads the actual tar member before extracting, verifies Linux ELF64 machine 62, and executes only Node runtime introspection to prove actual version/platform/architecture. It records binary size/digest. This is checksum verification against authored and official inputs; no detached-signature verification is claimed.

Root explicitly raised this OS-only cap to 4 GiB temporary / 1 GiB compressed input and fifteen minutes, separate from project native storage. The fresh signed daemon plan, prior retained OS images/inputs, actual metadata, two-copy snapshot allowance and a 512 MiB Node archive/extraction reservation produce 619,316,608 conservative input bytes and 2,267,731,168 temporary bytes, both within those revised caps. Exact tool/daemon/Node plans were sent to root before this final OS provisioning step. The final guest remains source-free; no inner daemon, project compiler, dependencies, Store or build is authorized in this step.

## Final OS Runtime Outcome

The final OS provisioning command is in progress; its generated log is `🗑️generated/2026-10-19-builder-os-builder.log`. No final snapshot or Node/daemon success is claimed before terminal evidence.

That Node/daemon OS step completed exit 0. Actual package queries confirmed the three exact daemon packages. The existing Debian service policy denied start, and the actual process-namespace scan recorded no dockerd/containerd processes. Node's official archive length and authored/official SHA-256 both matched; the extracted ELF64 x64 binary measured 122,889,056 bytes with SHA-256 `d1de76d8edf2fededf6f8b30d244e2c0529ac607923a018283b77e9c74bd932c`. Actual Node introspection returned 24.15.0/linux/x64. Exact installed package/Node receipts are retained in `2026-10-19-builder-os-builder-packages.json`, separately from the earlier daemon simulation.

The intermediate source-free builder OS image was `sha256:7bafa7e94d9bdf474fe7dac822dcc027bd938fe0bd08d54a59f0bef5c28e31ea`, completed `2026-10-19T12:51:13.443Z`, with actual new writable 334,708,736 bytes, expanded root 860,987,392 bytes and Docker compressed accounting 295,694,427 bytes. The package/Node step's container was removed. Read-only Docker version queries during provisioning still refer to the external Desktop transport daemon; they are not an inner-daemon readiness result.

The earlier tight-cap recipe removed transported Bun before snapshotting. Under the revised 4 GiB cap, parent was notified before a final reversible seal added that already verified official Bun binary to a new derived image, with no package reinstallation or new network input. Actual Node/Bun runtime checks, Git version, Compose version, executable-path checks for all required tools and an actual empty inner-daemon process roster passed in this source-free image. Make/g++/cc were inspected as executable tools; no project compiler was executed.

The final image is `sha256:8f5852254c08a5941feea6ceb0610dbc5f66063702cfeb7f9a4edb76997e9a9b`, local tag `semio-course-preflight-os:sol-ready-20261019`, completed at `2026-10-19T12:54:00.975Z`. Its actual expanded root is 939,573,248 bytes, the last Bun-only writable layer is 79,527,936 bytes, and Docker's compressed image accounting is 333,151,468 bytes. Conservative whole-OS input accounting is 698,817,248 bytes and temporary accounting 2,560,950,176 bytes, below 1 GiB / 4 GiB. The final retained input is `2026-10-19-builder-os-ready-input.json`; generated evidence is `🗑️generated/2026-10-19-builder-os-ready.log`.

Final actual runtime/tool observations: Node 24.15.0 linux/x64, Bun 1.4.2 linux/x64, Git 2.39.5, Compose v5.6.0; executable paths exist for Git, make, g++, curl, cc, pkg-config, docker, dockerd, containerd, iptables, Node and Bun. Inner daemon process roster remained empty. All worker execution sessions are terminal and all owned OS containers were removed. Previous immutable OS image IDs are preserved; only this worker's explicitly authorized local OS images remain for parent reuse.

Root and independent reviewer were given the final immutable ID and receipts. Root owns the eventual privileged builder, real Source volume/Git metadata, inner-daemon startup, actual same-workspace operator checks and any original managed compiler/Store/build invocation. None of those outcomes follows from this OS snapshot or its runtime/version evidence. Course publication and full learner/image gates remain incomplete.

## Inner Daemon And Runner Follow-Up

The separately authorized source-free genuine inner-daemon smoke subsequently passed and cleaned its container; exact outcomes and limits are in `2026-10-19-builder-daemon-smoke.md`. The temporary full-runner readiness owner is prepared and source/pure assertions passed, with no actual full-builder creation; details and final handoff digest are in `2026-10-19-builder-runner.md`. These later checks do not change the original OS provisioning scope or claim native/course release completion.
