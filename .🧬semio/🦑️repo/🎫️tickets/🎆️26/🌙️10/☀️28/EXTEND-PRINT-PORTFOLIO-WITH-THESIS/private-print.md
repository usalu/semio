# Private Print Store Boundary

## Actual Failure

The declared test Original was genuine, but its workspace was the live checkout. Bun's repository test preload fences the live cache/control/lease directories for the lifetime of the test process. Opening a Print document operation reaches those stores and is correctly rejected. Successful birth alone therefore did not establish that production storage effects could run directly against the checkout from a Bun test. The previous source-level feasibility report omitted this independent fence and must be read with this correction.

## Supported Private API

privateRepositoryWorkspaceV1 creates an owned isolated repository, validates that every partition/lease/control path remains inside it, issues genuine private-root invocations through invoke, pins process.cwd through within, provisions partitions through prepare, and retires all owned images after the final test. Its default managed short root is below checkout/.cache/test-sandbox. It does not copy or project checkout Source. The invoke helper creates a genuine private native caller; callers must consume its Original and issued controls, not spread or rewrite live checkout capabilities.

There is no existing Print source/store split. withPrintOperationV1 requires operation.workspace to equal the native caller repository root. Font preparation reads tracked fonts below that root, the compiler reads the shared library below that root, and stagePrintSources rejects any Source root outside the destination workspace. Source readers reject symbolic links and files whose link count differs from one. Symlink/hardlink projections cannot satisfy this contract.

## Location Constraint

The current ticket root is96 UTF16 path units. The private store profile's declared deepest path below a child root would be279 units, exceeding256. Therefore the supported private workspace allocator cannot place this profile below the ticket directory. A managed short private root outside the ticket is required by the current protocol. No private root or copied source was created by this review.

## Projection Implications

A private test that exercises unchanged production staging needs an exact physical snapshot of the registered documents, shared library and font source roster inside the managed private root. It must retain/check original Source hashes before and after compiler/publication consumption; otherwise a peer edit to the live Source after snapshotting would be invisible to the private staging recheck. This would be fixture projection, not a new production Source alias or weakened reader. It requires the parent task to resolve the managed-location constraint against ticket placement requirements. Checked Source must not be copied into the ticket, and the existing fence must remain enabled.

Token preparation also currently constructs its script path below the operation workspace. A minimal private snapshot would need a deliberately supported token-generation route or sufficient physical source closure; blindly copying only LaTeX/templates/fonts would leave that script absent. The script writes using its received genuine repoRoot, and its renderer imports broader design-token modules.

Private stores do not automatically inherit prepared live compiler/bundle/font artifacts or JavaScript dependency ownership. They need normal owned preparation and reader invariants; borrowing live store directories from the test is not sanctioned (apart from the API's separately documented immutable Nx-toolkit read, which does not apply to Print).

This review performed only source/path analysis and wrote this report. No fence was disabled, no authority was forged, no unrelated process or claim was changed, and no heavy compiler was launched.

## Accepted Managed Projection and Minimal Ownership

The parent explicitly selected the existing framework-managed short private repository as standard runtime test cache, with all authored scripts/logs/audits/exported proofs staying in the ticket. No checked Source is copied into the ticket and no production root containment rule changes. The private API automatically retires its owned runtime workspace.

Required consumed source projection is the union of the catalog librarySources, registered thesis/phd Source arrays, and four font-catalog leaves. Current read-only inventory contains 188 physical single-link files, 23277631bytes (22.20MiB). This includes both bibliographies, all shared LaTeX packages, logos, graph TeX and exact tracked fonts. Catalog and font-catalog JSON identities are separate provenance inputs. No TypeScript runtime closure is copied.

Token generation intentionally derives private semio-tokens.sty using the actual pure renderer. Record the live original token hash separately from derived consumed bytes; differing hashes would reflect normal production generation, not a silent snapshot mutation. Recheck originals and directory rosters after consumption to detect peer Source changes.

The dependency reader requires a genuine private ready generation. Minimal root fixture metadata is a named package with workspaces=[], semio.workspace={schemaVersion:1,members:[packages/*],owners:[]}, plus a matching Bun lockfileVersion1 root workspace and empty packages map. The positive member pattern is mandatory but may match zero packages. bunfig.toml is optional; MODULE.bazel, project.json and Print package metadata are not read by these Print/dependency interfaces. Create readiness using the owned withJavascriptDependencyStorageV1 plus prepareJavascriptDependencies sync API before opening document/probe execution. A manifest alone does not satisfy the read-only consumer. The cheap private-store smoke must establish these source-derived prerequisites before the next compiler run.

## Unchanged Production Script Child Integration

Reviewed the actual tectonic-template-compilation script and ScriptRouter/BundleScript implementation. The script creates its router at module load using new ScriptRouter(import.meta.dirname), without an explicit repository-root argument. The router defaults to findWorkspaceRoot(bundleRoot), whose provided physical start takes precedence over all environment and current-directory fallback sources. Its search therefore resolves the real source checkout from the real script path. BundleScript preserves the router's selected repoRoot.

receiveScriptProcessInvocation preserves the projected genuine Original but does not replace this router root. A private-root Original passed through scriptInvocationEnvironment would consequently reach BuildScript with the live checkout repoRoot and fail the Print workspace/native-authority equality check. Changing cwd or SEMIO_WORKSPACE_ROOT cannot fix an explicit physical router start. No existing script argument or supported root override is exposed.

Therefore unchanged production script-as-child integration is not supported for this private-root proof while runtime Source stays in the real checkout. The exact unchanged buildRegisteredPrintTemplate/publisher/compiler path remains available in the private test, but neither the script-router entry nor global CLI graph planning has thereby been executed. No root override, source extraction, capability rewrite, runtime copy, or infrastructure API change was introduced.
