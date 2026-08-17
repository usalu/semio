# UI Hover Card Dependency Prune Queue

After the HoverCard source and package import were removed, `@radix-ui/react-hover-card` is expected to remain only as a direct UI React package dependency and root lock resolution. The package manifest and root lock are central registrar paths and must change atomically through Bun.

This prune is queued with Accordion because the externally owned animate plugin `package.json` remains dirty while the plugin wave is expanding. Running workspace lock regeneration now could absorb unrelated concurrent dependency drift. Do not hand-edit the lock or leave the manifest/lock pair inconsistent. After external package owners release their manifests, rehash the UI package manifest and root lock, remove both dead Radix direct dependencies in one registrar lease, regenerate with Bun, prove isolated lock drift, and rerun the UI gates.
