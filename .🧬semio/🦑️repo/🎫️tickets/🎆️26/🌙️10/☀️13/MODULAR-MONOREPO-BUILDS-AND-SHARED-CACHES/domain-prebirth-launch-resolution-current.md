# Domain Prebirth Launch Resolution

Read-only inventory during Nx's fresh SDK/Native/ordinary Source hold. No imported process, lease, provider, graph or compiler implementation changed. This is the concrete handoff for Root's two domain projection sites; canonical registry/cleanup implementation remains Storage's next Source window.

## Actual Current Flow

`build/📜️script.ts:createBuildBootstrapCapabilitiesV1` parses the selected tool, resolves the portable build selection/mode, and calls `createNativeBuildCallerCapabilitiesV1` or `readBuildAuthoringCapabilitiesV1` before issuing the process envelope. The current default caller stores `arguments` and `selection` as `["repo", ...args]`, but declares only that single repo launch. Later `runOwnedBuildInLeasesV1` line 80 reconstructs capabilities and spreads them into a copied invocation. That is the first private-issuance break.

`runOwnedNxInvocationV1` in bootstrap line 598 reconstructs an Nx caller and spreads it into another copied invocation. Its helper `createNxBootstrapCapabilitiesV1` already resolves the exact Nx invocation and the JavaScript/watch child launches, but it currently runs after the parent birth for this nested route. This is the second private-issuance break. The replacement must retain the actual parent invocation object and controller; declaring downstream work before birth removes the need to brand either copy.

The existing immutable caller schema's `launches` is an array of literal string arrays. It can represent the finite union of the root repo request and its downstream Nx launches without inventing wildcard grants. Keep the root `arguments` and `selection` unchanged. Resolve child data from that declaration at the actual spawn boundary; do not replace the live parent's capability object merely to change a child envelope's selection.

## Exact Prebirth Resolution

1. Use the supplied environment argument, not a later ambient `process.env`, for `parseBuildInvocationV1`, `resolveBuildInvocationV1` and mode/config resolution. Root mode, canonical repository root, network, transport, child duration and artifact grant must be settled before the root envelope. Preserve explicitly authored resource ceilings; do not silently replace them with default policy on a nested call.
2. For the Nx tool, apply `nxBuildArgumentsV1` to the already resolved portable arguments, then `resolveNxInvocation` once with that same effective environment. This must preserve the literal opaque tail after `--`; aliases such as `--skip-cache`, `--output` and `--include-global-files` belong to the existing owned translator.
3. Capture the exact finite child set below. Prefix every actual Nx argument vector with the literal `"nx"`, as `createNativeNxCallerCapabilitiesV1` currently does. Avoid adding another prefix to already prefixed vectors. Repeated retry of the same declared command is permitted; a different command is not inferred from retry.
4. Preserve the root request in the repo capability's `arguments`/`selection` and include it in `launches`. Its downstream Nx commands must also be declared before birth. A closed domain child-envelope builder may select only those declared commands while retaining the exact root deadline/resources. Data serialization for a genuine child process is distinct from registering a new in-process invocation object.

| Actual Path | Required Literal Downstream Vector |
| --- | --- |
| Normal selected Nx invocation | `["nx", ...resolvedNx.args]` |
| Conditional JavaScript environment install | `["nx", "run", "workspace:deps-javascript", "--output-style=stream"]` |
| `repo tools setup nx` or `all` | `["nx", "--version"]` |
| `repo reset` with Nx | `["nx", "reset"]` |
| Configured watch registration | `["nx", "watch", "--all", "--includeGlobalWorkspaceFiles", "--verbose", "--", "bun", "nx", "run", watchTarget, "--output-style=stream"]` |
| Configured watch activation/restart | `["nx", "run", watchTarget, "--output-style=stream"]` |
| Actual watch daemon startup | `["nx", "daemon", "--start"]` |

The last row is a concrete current declaration gap: `NxScript.ensureDaemon` executes it at line 520 through direct owned capture, while `createNxBootstrapCapabilitiesV1` currently declares the main/JavaScript/watch vectors but not daemon startup. Declare it only for the actual watch path and retain the separately authenticated startup-to-service handoff. This does not make daemon lifetime a request loan or allow a latest request to mint a service.

`graph`/`show` use the first-party graph path and do not ordinarily launch Nx here; dependencies and direct Cargo also have distinct owners. Do not authorize unrelated Nx commands for those routes solely because the selected default tool is Nx. The JavaScript fallback is a finite declared contingency on graph-consuming Nx invocations, not a launch performed by every request.

## Child Role And Helper Continuity

The current `nativeNxChildEnvironmentV1` requires an Nx role with `arguments[0] === "nx"`, a selection exactly equal to the actual child, unchanged repository root/mode/network/artifact grant and schema-valid resources. `issueSelectedNxCallerEnvelopeV1` can select only an already declared launch. Root's new domain builder must satisfy this existing consumed boundary; simply passing a repo-shaped capability object into it will fail.

Keep the actual parent invocation in `NxScript` and the build helpers. At spawn, a domain-owned serializer can produce the child's Nx-shaped envelope from the predeclared launch set and original resource/deadline record. This is not a new parent birth or a public generic capability-registration API. In-process store/snapshot/publisher helpers must use their actual privately live parent rather than receive that projected child environment as a replacement controller. Storage owns those private reuse/derivation changes with the canonical registry.

Nested incoming callers may already be Nx-shaped. Validate their exact declared launches and original mode/root before using them; do not recompute a broader grant. `readBuildGraphAncestorSelectionV1` currently understands both repo and Nx namespaces and enforces declared selection. Preserve its actual ancestor graph checks while removing structural invocation copies.

## Additional Literal Copies To Coordinate

Two nearby copies also matter to the canonical private registry and must not be missed while fixing the named sites:

- `build/📥️caller/🟦️.ts:issueBuildTaskEnvironmentV1` calls `scriptInvocationEnvironment({...invocation, capabilities:selected}, ...)`. This constructs child-native-command data from a graph-owned execution. It should serialize that validated child envelope from the actual parent, not pretend the copied object is an issued in-process invocation. Storage's serializer/private-root integration and Root's domain grant owner must agree here.
- The Bazel watch branch in build script constructs `combined={...invocation, control:{...invocation.control, signal:AbortSignal.any(...)}}`. This needs an owned additional-signal derivation from the same private parent. It is not a capability projection and does not authorize another root.

## Required Laws Before Integration Is Claimed

Use neutral vectors for portable aliases/opaque tails, prototype/release configuration, tools/version, reset, the JavaScript contingency and both watch vectors plus daemon startup. Check the current child consumer's actual admission and run genuine Node child receipt/argv tests; calling the same resolver twice is not an independent oracle. Assert unchanged parent object/controller, immutable root request/resource grants, exact child mode/root/network/deadline, refusal of an undeclared launch, and no extra authority from a forged copied invocation. Actual in-process helpers must remain on the private original scope. No canonical namespace enrollment or global lease routing follows from these data declarations alone.
