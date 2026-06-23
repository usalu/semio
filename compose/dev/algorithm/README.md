# Summary

Compose Algorithms provides Storybook-based algorithm visualizations with separate copy/paste and replaceable-types stories.

### Specs

- The bundle exposes Storybook stories focused on the `compose/rs` single-source implementation as re-exported through `@compose/js`, `@compose/react`, and `@compose/ui`.
- Storybook setup mirrors `compose/ui` so addons, decorators, MDX handling, and development behavior stay consistent.
- Visual primitives for algorithm rendering come from `@compose/ui`, while the replaceable-types story renders the full Nakagin Capsule Tower source and computes its compatible tree directly from the live selection.
