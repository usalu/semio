# Native Local Router Input Audit

## Current Actionable Finding

The completed `nx-root-gis-map-components-2/project-graph.json` declares the Map Rust test command as `bun ./📜️script.ts test` in the artifact package directory. Its effective native target inputs, local named groups and explicit dependency input groups contain 9,299 unique positive patterns. An independent minimatch check found zero matching patterns for either the local package router or its imported common Rust artifact router. Negative exclusions were ignored; they cannot supply missing ownership. This probe covers the execution target’s declared source groups, not unrelated named groups that it does not consume.

Affected source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`. `nativeCommandInputs` near line 339 seeds the generic Cargo command script; native normalization near line 619 replaces the target input declaration. The actual local router and its executable import closure need their own declared inputs. The newly added Map test-stack setting is one observable change currently omitted by this graph.

Raw independent receipt: `🗑️generated/native-local-router-input-audit.json`. The Nx executor owns a neutral regression, the scoped normalizer correction, and a fresh ordinary graph validation. Do not claim this finding repaired or use the current PDF input epoch as the final cache baseline until that evidence is available.

## Scoped Correction Evidence

The Nx owner reproduced the omission against a schema-validated fixture (`native-local-router-input-red.txt`). The normalizer now resolves the selected command relative to its declared working directory and includes the local script plus executable import closure, in addition to Cargo/toolchain inputs. The independent esbuild oracle reports three expected executable files and excludes the sibling fixture (`native-local-router-input-green.txt`). Ordinary Gismap/PDF project graph validation remains pending; the fresh Terra auditor is reviewing the final source and evidence.

## Ordinary Project Graph Proof

The ordinary current Gismap and PDF project checks both exited 0. Their build, check and test target inputs contain each exact local artifact router and the imported common Rust artifact router. The Gismap proof excludes the Terrain sibling router; the PDF proof excludes the JPG sibling router. Receipts: `gismap-project-native-router-current.json`, `gismap-native-router-input-proof.txt`, `pdf-project-native-router-current.json`, and `pdf-native-router-input-proof.txt`. The shared helper owner reports the correction settled. The coordinator is refreshing the separate literal PDF/JPG production-root closure oracle; the final native cache proof remains pending.

## Full Production Isolation Follow-Up

The coordinator refreshed the full production-root oracle against the ordinary post-router graph. The conservative closure covers 31 projects, 34 target invocations and 2,322 unique positive input patterns, with zero unresolved groups or targets. Independent minimatch accepts the actual PDF production root and rejects the actual JPG production root; both the local PDF router and shared Rust artifact router are included. All candidate paths were checked to exist. Graph SHA-256: `816ac778bc18db7d6f116c7026ebb23408c00365ff989060e03b2302d4e4c372`. Receipts: `pdf-native-production-isolation-post-router.json` and `pdf-native-production-isolation-post-router.txt`, exact exit 0. This remains structural evidence; cache-hit restoration is pending.
