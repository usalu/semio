# Summary

Reusable semio UI elements and Storybook source for shared interface primitives used by `semio/js`.

### Specs

- The `ui` bundle owns the shared element source formerly embedded in `semio/js/sketchpad`.
- Storybook configuration, stories, and static output for shared elements live in this bundle.
- `semio/js` consumes this bundle instead of defining shared element primitives locally.
