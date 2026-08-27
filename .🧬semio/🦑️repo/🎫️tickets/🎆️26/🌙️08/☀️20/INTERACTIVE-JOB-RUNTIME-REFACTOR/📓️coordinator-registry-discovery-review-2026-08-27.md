# Registry Discovery And Launch Verification

The canonical registry generation target intermittently disappears from Nx's graph even though its emoji project configuration declares it. Read-only checks established that the local emoji plugin directly returns all nine targets for the exact registry configuration, and Nx's native workspace glob returns that exact file among 181 configurations with no lossy paths. One shared graph snapshot contained emoji-config provenance; a later snapshot contained only package-json provenance and an empty target set.

The executor ran canonical generation successfully with `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false`, then compared the complete nine focused gate objects in the authoritative `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json`: exact equality passed, including the CAD ticket-local nextest artifact environment. However, a subsequent coordinator `nx show project` with the same flags still reported empty targets. The successful generation and exact output comparison are valid; the intermittent discovery defect is not claimed resolved.

No Nx implementation/configuration was changed, no caches or peer processes were deleted, and no target bypass was used. Executor evidence: `retained-launch-generation-attempt.txt`.
