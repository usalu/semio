# Setup Orchestration Ownership

The public Root command alone is not equivalent to the existing workspace:setup target. No graph was cancelled and no prerequisite was skipped during this investigation. Current full setup session7334 remains authoritative.

The target runs repo ./📜️script.ts setup as a side-effecting repo:script executor with cache:false and14 declared prerequisites. SetupScript.runFull only logs that the graph supplied dependencies and invokes ensureMcpBinary; it does not provision those prerequisites. The command registry marks setup requires:none and explicitly promises operation in a half-installed workspace. This exposes split orchestration ownership, not an authorized shorter equivalent route.

| Target | Declared command | Additional direct prerequisites |
| --- | --- | --- |
| deps-build-tools | repo ./📜️script.ts repo tools setup all | none declared locally |
| deps-js-all | repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts" sync --all | none declared locally |
| deps-python | repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync python | @semio-tech/ui-styling-py:deps, @semio-tech/energy-oracle-py:deps |
| deps-cargo | repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync cargo | none declared locally |
| deps-go | repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync go | none declared locally |
| deps-dotnet | repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync dotnet | @semio-tech/repo-test-dotnet:deps |
| deps-cpp | repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync cpp | cpp-setup |
| deps-browsers | repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync browsers |  |
| deps-wasm | repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync wasm | deps-wasm-opt, deps-trunk |
| deps-tools | repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync tools | none declared locally |
| setup-git | repo ./📜️script.ts setup git | none declared locally |
| prepare | aggregate | @semio-tech/framework-schema:generate, @semio-tech/ui-styling-tokens:generate, @semio-tech/assets:build, @semio-tech/framework-graph:generate, @semio-tech/plugin-registry:generate |
| repo-mcp:install | external project target | none declared locally |
| @semio-tech/framework-os-mcp-rs:build | external project target | none declared locally |

The closure also includes cpp-setup, deps-wasm-opt and deps-trunk. Python brings ui-styling-py:deps and energy-oracle-py:deps. .NET brings repo-test-dotnet:deps. Prepare orders framework-schema:generate, ui-styling-tokens:generate, assets:build, framework-graph:generate and plugin-registry:generate. Every external target own dependency closure and source execution class must also be retained before replacing the graph route. The framework-os-mcp-rs:build prerequisite must remain; ensuring a binary after silently dropping it is not sufficient equivalence proof.

The public Root script is a legitimate issuer: runWorkspaceScriptMain uses the existing received Original when supplied and otherwise creates the command policy with workspaceRoot. The private native dependency leaf only receives an existing process invocation, then requires the real Native caller phase and preparation storage. A raw leaf invocation or invented Native capability object would bypass the intended boundary and is not an alternative.

A complete graph-free design would put a closed provisioning workflow in an owned schema, make the public setup command execute that workflow under an appropriately issued Original, and derive the setup target orchestration from the same schema. It must include every current prerequisite and transitive generation/build stage, preserve Native phase/storage/budget/refinement checks, and avoid running the general project-discovery graph merely to find known provisioning stages. The Root birth must issue the authored Native workflow authority before work starts; child commands must not widen a workspaceRoot-only parent or start a fresh deadline.

Test-first plan: language-neutral workflow fixtures cover all14 prerequisites, each transitive closure, ordering, deduplication, completion, failure at each stage and cancellation. Ajv validates fixtures and Python graphlib.TopologicalSorter independently proves dependency order. Actual issued controls prove the same Original, nonincreasing deadline, closed phase capabilities, no later stages after failure, lease drainage and no private-leaf fallback. All three native platforms and devcontainer must select the same semantic workflow. A fresh owned setup integration must provision tools/dependencies and stage the actual MCP binary while a graph-discovery sentinel records zero global graph invocations. Existing full setup scope is retained until this equivalence is implemented and verified.
