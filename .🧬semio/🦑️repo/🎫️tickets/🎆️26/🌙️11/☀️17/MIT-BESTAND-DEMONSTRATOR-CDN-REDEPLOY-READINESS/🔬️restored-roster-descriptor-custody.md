# Restored release roster descriptor custody

The current release roster contains 11 plugins and 21 extensions. For every root, the canonical descriptor reader validated its authored Pack and JSON, independently validated the staged Pack and JSON, and compared the staged semantic digest with the published browser receipt. All 32 staged pairs match their own receipts. Twenty-four pairs match the current authored Pack bytes and semantic digest, covering all 21 extension modules. The raw JSON bytes differ across all 32 pairs, so the comparison uses the canonical semantic reader.

| Guest | Authored digest prefix | Staged and receipt digest prefix | Example change |
| --- | --- | --- | --- |
| CAD | `b7a7f1f25b73` | `fee180a819a0` | Default layout uses explicit split axes |
| Energy | `1bc24050665a` | `847b72debdc9` | Default layout uses explicit split axes |
| FEM | `aa040286b787` | `0af30e1828a9` | Interaction action and icon surface |
| GIS | `fb68e4315d73` | `06b9070b990d` | Default layout contains a direct map window |
| Playbook | `753cbc43b969` | `0c2df8650dc2` | Hover target and granularity arguments |
| Process | `02f450590dc4` | `bc0dbb1fa330` | Editor action and argument declarations |
| Puzzle | `e116c3e5282c` | `3ac61866f928` | Asset digest and media declarations |
| Sourcing | `41987953f093` | `879b2af2ecba` | Preview subscribes to row interaction |

The eight stale staged roots require replacement for exact authored receipt acceptance. The Process component is a required runtime dependency even though the Bearbeiten surface is mounted by Demonstrator. Demonstrator and Procedural match current descriptor semantics and separately require refreshed executable bytes for their changed runtime sources. Descriptor equality establishes the interface and declared assets. Optimized-core custody and the browser interaction suites establish the executable release.

The locked joint WASI unit graph additionally includes Flow, whose standalone executable predates current read and cancellation-session code. The tail pairing of Flow and Playbook shares 1,023 units across a 1,060-unit union. Pairing Sourcing and Puzzle shares 942 units across a 1,392-unit union. Together these two pairs use the fewest total units among the three possible four-root matchings. Actual paired admission remains subject to compiler and product-store capacity.
