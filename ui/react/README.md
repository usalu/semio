# Summary

Reusable compose UI elements and Storybook source for shared interface primitives used by `compose/js`.

### Specs

- The `ui` bundle owns the shared element source formerly embedded in `compose/js/sketchpad`.
- Storybook configuration, stories, and static output for shared elements live in this bundle.
- `compose/js` consumes this bundle instead of defining shared element primitives locally.
