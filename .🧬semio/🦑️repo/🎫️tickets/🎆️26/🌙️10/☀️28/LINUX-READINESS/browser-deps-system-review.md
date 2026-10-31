# Browser Dependency System Port Review

Static current Source review, 2026-10-29. No tests, native commands, container, process or lease operations ran. The JSON array comparisons below read metadata only; they do not invoke the implementation.

## Findings

The first read found offline refusal inside the Linux branch. Current Source has corrected it: abort precedes an all-platform offline refusal, before host observation or dispatch. Root added non-Linux offline and abort-plus-successful-return neutral cases. Static reinspection confirms the correction; no test execution was performed here.

Current tests include a real temporary-filesystem system-port case in caller-owned artifacts. It exercises relative PATH exclusion, non-executable first candidate versus executable second candidate, missing PATH/name, platform/root facts, CLI file/directory distinction, and Windows permission semantics. Symlink executable identity and races remain ordinary OS execution semantics rather than an immutable/no-follow guarantee. Earlier mock-only coverage concern is resolved in Source; no fresh test pass is claimed.

## Executable And Sudo Selection

Production browserSystemPortV1 reads only the supplied PATH, ignores nonabsolute/empty elements, requires X_OK and stat().isFile(), and continues on ENOENT/ENOTDIR/EACCES. Other errors propagate. It follows symlinks through stat/access, as ordinary system command aliases may require; this is executable selection, not a no-follow authority proof or immutable executable identity. CLI presence checks regular-file status, not executable permission, which is appropriate for a runtime-loaded JS file.

Manager precedence is apt-get, dnf, pacman, zypper. Non-apt package managers use the resolved absolute executable. Unprivileged commands use the resolved absolute sudo, `-n --`, and the absolute manager; apt uses absolute env after sudo with `DEBIAN_FRONTEND=noninteractive`, followed by runtime and CLI arguments. This avoids depending on sudo preserving DEBIAN_FRONTEND or resolving the runtime through its secure PATH. The canonical runtimeExecutableV1 caller supplies the runtime; the public helper itself does not validate its absolute/executable identity.

For apt, resolved apt-get selects the branch but Playwright install-deps owns actual apt invocation. Do not claim the exact selected apt-get path is passed through: it is not. The Playwright CLI and runtime are executed as root through sudo for dependency installation; Chromium download runs afterward through the ordinary caller runtime/environment. Noninteractive sudo failures propagate and stop the download.

## Cancellation And Original Phase

The helper checks abort before observation, again before system execution, after awaiting the system child, and after browser download. It never starts download after a rejected/aborted system operation. A cancellation arising during a successful system await is caught by the post-await check. Each execute call goes through the native branch's existing run closure, preserving workspace, exact signal, parsed storage and NativeCallerPhaseV1. That runner advances the same phase, checks budget and delegates to runOwnedCommand with phase.options(environment); no new Original, phase or lease is issued by this helper.

Current fixtures cover both abort-and-throw and abort-plus-successful-return. Cancellation during browser download and native branch exact phase identity are not independently exercised by those helper mocks. Production wiring statically preserves those identities; neutral mocks alone do not prove owned-child termination timing.

## Exact Package Fixture Parity

Metadata comparison found complete ordered equality between policy arguments+packages and both root/sudo expected commands for DNF (20 packages), pacman (19) and zypper (21). Sudo comparison removes only the exact `-n -- /tools/<manager>` prefix. Thus no missing, extra or reordered package was found in these fixture expectations. This proves policy/fixture agreement, not distro package availability or actual Chromium library completeness. No network lookup or package-manager run occurred.

The policy has a closed Ajv/owned-schema neutral validation envelope, with new rejection cases for duplicate manager tuples and wrong argument arrays. New tests also ensure the native browser branch awaits prepareBrowserDependenciesV1 and the old shell browser-library helpers are absent. Current caching/🗂️suites/🔣️.json explicitly declares browser-dependencies: budget15000, typecheck none, exact system test file, platforms darwin/linux/win32, capabilities {}. The earlier broad bootstrap-dependencies route could discover it, but the new explicit row makes the canonical leaf clear. No focused/canonical test success is inferred here.

## SDK Map Check Status

Independently read fs.realpathSync('.nx/installation') + '/.semio-authorizer-source.json', checking exact sources[].path equality. Native branch 🟦️.ts, system/🟦️.ts and system/🔣️.json each have zero matching entries. caching/🗂️suites/🔣️.json has one entry, recorded SHA256 27973c99c42f7b47de72107bb4d3c89e1605be318a3c00366452a2edf167dee0. Therefore the existing SDK snapshot does not include this changed native/browser implementation closure; Source-map authentication must use a legitimate refreshed generation where required. No SDK compiler, module load, map regeneration or native probe was launched.

## Bash Completion Status

The actual native bootstrap Bash file has `set -euo pipefail`. Its final main-body invocation is standalone `repo_bootstrap`, followed by the success log. Inside repo_bootstrap, native install, native help, workspace:setup and debugger setup each use `|| return 1`. Because the top-level function call is not in an if/and/or status-suppression context, a nonzero function return triggers errexit before the completion log; failures propagate as nonzero rather than printing success. SKIP_REPO_BOOTSTRAP=1 explicitly returns zero and is the existing deliberate skip. Static ending review found no exit-propagation defect. No shell execution was used to validate this.
