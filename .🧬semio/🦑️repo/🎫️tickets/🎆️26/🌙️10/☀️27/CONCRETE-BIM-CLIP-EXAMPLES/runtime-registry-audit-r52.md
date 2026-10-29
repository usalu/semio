# Runtime Registry Gate Audit R52

Read-only diagnostic,2026-10-29. Used Node filesystem APIs only; no alternate command, installer, binary, infrastructure edit or source copy. This audit does not claim the gate is fixed.

## Exact Registry and Parser Match

Current launcher registry at 🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json contains contract1 and restart.policy pointing to the build format-stamp schema. This is valid current source: Go runtime-registry file76–78 explicitly defines Restart.Policy; line165 requires it nonempty. JSONschema8 requires restart and19–23 defines its policy path. Removing restart would break the current implementation.

The referenced policy exists. Its $defs.Protocol.const has maximumReexecutions6, backoffMs[250,1000,4000,12000,30000,30000] and both busy texts, matching resident.LoadRestartPolicy448–476. No wrong intended restart value was found. It is a documented shared-policy input, not a runtime name or Boolean.

Current contract is1 in Go core11, registry and schema. Reproduced the registry-shape test's sorted canonical JSON excluding description/$comment with Node crypto: SHA2567ac903b3dab98d3172a40fbdc9a21f15a7391ccd15b7d72d8fde47e86f72df4a, exactly fixture.registryShape.schemaDigest. Thus the current files are internally consistent; this read-only snapshot cannot establish which concurrent author changed their earlier history. README67 and registry-contract188–206 require schema-shape changes to advance the contract, but a new bump cannot be justified merely from a digest mismatch because none exists now.

Given the reported installed executable error malformed restart, its decoder does not match the current struct. A stale or different installed launcher is the supported inference; no runtime download/pin defect follows. Default installer destination C:/Users/Ueli/AppData/Local/semio/launcher/bin/repo.exe was absent when checked. This does not identify the actual PATH executable; no environment/PATH enumeration or executable was invoked in this audit.

## No Supported Pre-Parse Self-Update Found

Launcher command.Main135–156 recognizes only --version/-V before PlanRoute; Version86 immediately loads the registry inside a repository. Build/workspace/script routes call Execute386–390, which first loads the registry. Domain/mcp routes also LoadRegistry atMain147. PlanRoute's discovery of build verbs already calls LoadRegistry atentry86. Therefore help, version inside this repository, scripts, tools/build targets and native installation commands all encounter the same decode gate before implementation dispatch. No launcher self-update/reinstall/bootstrap command or registry-schema bypass flag exists in the inspected route.

Root resolution is independent of executable location: StartDirectory39–46 prefers SEMIO_WORKSPACE_ROOT, then REPO_ROOT, then current working directory; ResolveRoot69–91 searches upward for paired project/package markers and package workspaces. Runtime overrides SEMIO_RUNTIME and cache/mirror options select provisioning after successful registry decode; they cannot fix an unrecognized registry field. --cwd is a script runtime option, not a pre-parse registry replacement. Changing root to evade the real repository would not validate or repair its command interface.

The supported source recovery path is the platform installer rebuilding/reinstalling the current native launcher; InstallCommand298 constructs that documented script command. The Windows installer itself is an external PowerShell bootstrap that provisions pinned Go and builds the launcher. Under this task's strict all-commands-through-repo constraint, invoking it directly would be an unauthorized bypass. No legitimate repo-only pre-parse self-rebuild path was established. Parent should retain this explicit gate rather than delete valid restart or claim native clip renders completed. An already rebuilt correct repo executable, if supplied by the concurrent infrastructure owner, could be tested through the authorized repo interface.

Current evidence cannot prove that reinstall alone resolves every downstream native-render dependency; it resolves the specific source/installed-decoder mismatch in principle. No executable probe was made here.
