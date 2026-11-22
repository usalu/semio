# Findings E58
- Dependency generation retirement starves: it runs only in `tools setup all` and is skipped while any shared holder lives (`occupied`); the gate also forbids `cache gc` (original command must be exactly `tools setup all`). See result-E58.md section 2 for evidence and the two fix options. Needs a decision because it widens an authority gate.
- `dependencies.orphans` doctor detail promises a collector that does not exist.
