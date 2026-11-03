# Build-fixer v9 (en1994 run 17, 30 E0308 in diff)
Root cause: the hand-written diff Rust (`🧬️schema/🔺️diff/{🏗️beam,🏛️column,🧱️slab}-rows/🦀️.rs`) still declared enum-typed row fields as `String`. Diff schema JSON already `$ref`s the snapshot properties (no JSON/twin change needed; twins untouched).
- en1994: patch fields typed (ActionKind, ActionStage, SupportCondition, ConstructionMethod, ReinforcementProduct, SheetingProfile, ColumnKind, BucklingCurve). `category` stays String (model String). Top-level scalar setters were already typed. Mutation diff leaves already pass enums.
- Pre-fix (same pattern, found by a snapshot-vs-patch type comparison over all norm artifacts): en1990 (PermanentKind, DistributionKind x2, TestDistribution), en1993 (MaterialKind, SectionKind, ActionKind, JointKind, JointCategory, BuiltUpKind, ParametricShape), en1998 (PierStiffnessBasis). All other artifacts clean.
- `schema twins check --strict` run via heavy.sh (result appended below if finished).
