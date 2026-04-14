# Summary

Semio Algorithms provides Storybook-based algorithm visualizations with separate copy/paste and replaceable-types stories.

### Specs

- The bundle exposes Storybook stories focused on algorithm behavior comparison across multiple implementations, inputs, and selection-driven compatibility trees.
- Storybook setup mirrors `semio/ui` so addons, decorators, MDX handling, and development behavior stay consistent.
- Visual primitives for algorithm rendering come from `@semio/ui`, while the replaceable-types story renders the full Nakagin Capsule Tower source and computes its compatible tree directly from the live selection.
