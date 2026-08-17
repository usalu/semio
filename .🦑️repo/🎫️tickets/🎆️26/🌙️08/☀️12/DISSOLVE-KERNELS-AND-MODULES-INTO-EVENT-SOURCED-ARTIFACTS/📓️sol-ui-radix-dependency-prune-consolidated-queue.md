# UI Radix Dependency Prune Consolidated Queue

The accepted UI deletions leave three direct React package dependencies without authored source consumers:

1. `@radix-ui/react-accordion`
2. `@radix-ui/react-hover-card`
3. `@radix-ui/react-tooltip`

Each remains only in the clean UI React package manifest and root `bun.lock`. Remove all three in one central registrar lease and regenerate the lock canonically with Bun after every externally owned workspace `package.json` is released. The animate plugin package manifest is currently dirty and the broader plugin wave is still expanding, so lock regeneration remains quarantined. Do not hand-edit the lock, absorb unrelated package drift, or leave manifest/lock state inconsistent.
