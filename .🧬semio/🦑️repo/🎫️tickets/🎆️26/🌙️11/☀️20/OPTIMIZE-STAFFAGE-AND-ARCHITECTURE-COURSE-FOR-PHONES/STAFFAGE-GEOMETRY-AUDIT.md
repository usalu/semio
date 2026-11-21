# Staffage Geometry Audit

The earlier full catalog test reported 136 ready and 16 invalid authored assets; the current count was not re-run because repository bootstrap stalled. Other agents are editing the same records.

The shared feature rule explicitly sets 0.15 mm on paper and computes a 75 mm world-space minimum at 1:500, limited by the thinnest drawn filled shape on the same bone. Thin roles, textures, qualifying slender solids and thermal layer shells have explicit exemptions. The rule is in `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts` lines 108–109 and 234–252. This is not a units mismatch.

The reported multi-stem-birch 1:500 body loft stem-l-2 has a maximum rx of 0.03615 m, hence the computed dimension is 72.3 mm, below the threshold. A slender declaration requires the independent 20:1 check; it should not be added without satisfying that geometry. The same rules file requires every bone used by an authored 2D shape to occur in a 3D body solid (lines 173–184), so ivy's missing crown bones are record-specific failures under that rule. Correct authored solids and coverage, or apply a justified existing exemption; do not globally weaken these invariants.
