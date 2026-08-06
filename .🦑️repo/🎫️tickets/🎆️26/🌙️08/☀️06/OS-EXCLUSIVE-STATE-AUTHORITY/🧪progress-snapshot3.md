# Progress 2026-08-06 (continued)

## Green crates
- semio-framework-os-kernel
- semio-framework-os (default; os-host-full gated)
- semio-framework-plugin (+ PureCommand Emit path)
- semio-framework-plugin-host
- semio-s-plugin-draw, architect, norm, fem, shooting, imperative (+ semio-s-imperative)

## Blocked
- semio-framework-os-infinite (~186) → layout/flow/space/sequence plugins
- Full host apply of Emit ops; guest INSTANCES/typed stores still present

## Done this stretch
- OS host default-feature green (media export path)
- CollectionOperation unified on VCS shape; command reexports VCS
- PureCommand guest hydrate → AppFrame::Emit
