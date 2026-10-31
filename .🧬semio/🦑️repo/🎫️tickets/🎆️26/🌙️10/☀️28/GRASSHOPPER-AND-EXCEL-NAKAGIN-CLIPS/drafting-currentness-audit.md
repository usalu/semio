# Drafting Dependency Currentness Audit

The audit uses the same teaching Python3.12 interpreter as the original recorders. Python AST fingerprints must be compared within that interpreter version; the initial lightPython3.14 comparison was discarded because its AST representation differs. Finite producer/query/oracle declared raw-byte inputs are unchanged, but finite producer maps omit shared Drafting and are insufficient for accepting its changed dependency. Complete correct-interpreter old/current deltas and current external raw identities follow.

```json
{
  "renderer": [
    {
      "path": "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\🐍️.py",
      "old": "101ca0e3fd7974d51378d210d0ba8f645d097b123cb9e22d56eb358741fa1af0",
      "current": "8d2b80cf296ef2460e39759f391db4bdfee157b8e35350eaab06892c555afb92"
    }
  ],
  "gh": [
    {
      "path": "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\🐍️.py",
      "old": "101ca0e3fd7974d51378d210d0ba8f645d097b123cb9e22d56eb358741fa1af0",
      "current": "8d2b80cf296ef2460e39759f391db4bdfee157b8e35350eaab06892c555afb92"
    }
  ],
  "excel": [
    {
      "path": "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\🐍️.py",
      "old": "101ca0e3fd7974d51378d210d0ba8f645d097b123cb9e22d56eb358741fa1af0",
      "current": "8d2b80cf296ef2460e39759f391db4bdfee157b8e35350eaab06892c555afb92"
    }
  ],
  "external_dependency_raw": {
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\🐍️.py": "57a4c95f334ac09b8c854e007bc841ebe120e35983b8c2e2fc82aad47630b9c5",
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\🧩️surfaces\\🎨️coverage.py": "bd8acb4f55ba48fa5811175f7d14c180d0aa2260fa5e132ba54958f3d1ecb305"
  },
  "audit_python": "3.12.11 (main, Aug 18 2025, 19:17:54) [MSC v.1944 64 bit (AMD64)]"
}
```
