# Current World Held Grant Contract Readonly Review

Lexical complete-pair inspection against current producer declarations. This is a source-only review, not a Rust type checker or native admission. Full World8 pairs remain held and unchanged.

```json
{
  "declarations": [
    {
      "type": "RetainedCloneGrant",
      "currentDeclaration": "pub struct RetainedCloneGrant {\n    pub maximum_items: usize,\n    /// 🧮️ Bounds payload copying within one clone turn.\n    pub maximum_copy_bytes: usize,\n    pub maximum_capacity_bytes: usize,\n    /// ♻️ Bounds physical owner and scaffold release independently of copying.\n    pub maximum_release_bytes: usize,\n    pub maximum_depth: usize,\n}"
    },
    {
      "type": "RetainedCloneProgress",
      "currentDeclaration": "pub struct RetainedCloneProgress {\n    pub copied_items: usize,\n    pub copied_bytes: usize,\n    pub retained_capacity_bytes: usize,\n    pub released_bytes: usize,\n}"
    }
  ],
  "constructorCandidates": []
}
```
