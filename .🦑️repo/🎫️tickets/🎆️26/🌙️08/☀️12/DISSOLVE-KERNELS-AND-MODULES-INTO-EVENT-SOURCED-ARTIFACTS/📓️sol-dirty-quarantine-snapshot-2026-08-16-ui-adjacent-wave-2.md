# Dirty Quarantine Snapshot: UI Adjacent Wave 2

- HEAD remains `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.
- Total porcelain entries increased from `1924` to `3183` while the isolated UI leases ran.
- Stdio entries increased from `184` to `439`.
- The serialized framework React index remains stable at `6efa99283af7df14639d1f301456690d3d16860156ed8d2bf1087094a2bfc2fc` after accepted Accordion and DiagramNode registrar changes.

The increase proves the plugin wave is still advancing. Stdio/glTF, plugin package registrars, generated mirrors, dependency registries, root lock regeneration, and full census regeneration remain quarantined. The current UI execution queue may continue only on clean, hash-stable closures that do not cross into the moving plugin graph or protected renderer/platform owners.
