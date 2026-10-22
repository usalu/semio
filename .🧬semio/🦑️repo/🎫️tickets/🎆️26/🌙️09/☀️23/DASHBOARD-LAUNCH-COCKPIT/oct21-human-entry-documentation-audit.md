# Human Entry Documentation Audit

This bounded read covered root README.md and .devcontainer/README.md. CONTRIBUTING.md and docs/README.md were absent at their literal paths. It is not a whole-repository documentation census.

The root developer guide names the dashboard as the only human control plane and provides dashboard task forms for file/history/Git/debugger/development services. Native repo commands are explicitly retained for agents and automation. No IDE entry route was found in these two documents.

The canonical library build router separately still advertised the repo-vscode build-vsix selection at inspection time. Controls is inspecting the owner before retiring the stale active route; this report does not claim its removal.

The devcontainer lifecycle table claims post-attach reinstalls repository hooks, conflicting with root README documentation that setup removes blocking hooks and does not install them. Controls is reconciling this bounded documentation finding with actual lifecycle source. A container account named vscode is not by itself an IDE configuration or extension. No runtime user change is justified by its name alone.

Read-only source observations do not establish runtime usability, zero-touch container execution, or exhaustive IDE-config absence.

Controls subsequently read actual lifecycle source: devcontainerAttach calls configureRepoHooks, which invokes raw bun ./script micro-commit install-hooks. Thus the table described current implementation accurately while the root policy conflicted. Root selected removal of the automatic lifecycle hook installation/helper, preserving unrelated micro-commit functionality and the existing canonical setup-git preparation; this is an authorized source change, not an executed modifying Git command or an actual container runtime result. Controls owns schema/test/docs reconciliation.


## Current Human Entry And Bootstrap Corrections

Fresh root README review identified three product quickstart build/publish examples still using the agent-only `repo run` form. They now use the existing `repo dashboard run` interface. The explicit agent/automation build reference remains valid. The human pre-push paragraph now names dashboard tasks. No dashboard command was executed by an agent.

The Windows bootstrap currently contains Test-ElevatedSession only for readiness reporting and no administrator refusal or Mount-DiskImage invocation. README now describes regular-terminal setup and separate machine-wide long-path guidance, preserving the initial fresh-clone bootstrap path. This is a source observation, not an actual Windows execution. The guide now matches the actual nightly-2026-07-20 toolchain and the three present workflow files rather than claiming an empty workflows directory. Hosted execution remains explicitly unverified.

Seven guarded exact replacements preserve other concurrent README content. This documentation correction adds no runtime behavior. Actual dashboard installation/journeys and cross-host execution remain pending.
