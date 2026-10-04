# Managed launch and neutral direction routes

[Full current source evidence](managed-launch-route-audit-inputs/full-current-route-sources-1.json) retains eight bodies/hashes and command presence counts.

The ordinary managed command is `bun nx run @semio-tech/plugin-registry:generate`, already registered in the launch seed. Its Nx target calls the registry's permanent `📜️script.ts generate`; that router selects GenerateScript in projection/🟦️.ts. The generator writes `.vscode/launch.json` using generateLaunchJson, which reads the authored seed skeleton and substitutes registry placeholders. Ordinary concrete consumer/direction rows therefore flow through the preserved skeleton. This is a broader registry generation command, not a launch-only command.

The three registered ingress consumer commands and the new framework-neutral product-direction command each occur exactly once in the seed. The retained input records current generated-file presence separately; no generator was run by this audit.

The real neutral direction target routes to `lint framework-neutral-product-direction`, calls verifyDependencyDirection with sourceRole framework-neutral, loads the actual present workspace package/taxonomy policy, inventories role-owned TypeScript/JavaScript sources, then invokes dependency-cruiser. It requires every declared strict rule, a framework area, and a nonempty followed source inventory. It rejects forbidden semantic edges and validates the reported graph against the expected inventory. It has no baseline or source-form exemption. It is a TypeScript/JavaScript graph gate, not a Rust/Cargo deletion or native proof. Existing owner roots are selected; absent owners are excluded, while an entirely empty neutral inventory refuses.

Root's actual route result is separate runtime evidence. This audit inspected sources only and neither regenerated outputs nor ran native tasks.
