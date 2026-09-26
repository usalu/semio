# WGPU VFS Localization and Accepted Accessibility

## Scope

This continuation aligns the WGPU virtual file system scene with the mounted React control contract without changing the shared scene payload.

## Implementation

- Added shell-owned English and German VFS chrome packs for `Name`, the empty-state message, and expand/collapse labels.
- Threaded the immutable presentation pack through `SceneEngineHosts` into VFS paint.
- Added candidate, sealed, and accepted semantic controls derived from the exact staged VFS row and chevron hits.
- Projected accepted rows and chevrons as focusable buttons. Chevron nodes carry `expanded`; row activation emits only `selectRows`.
- Added an accepted focus identity containing window generation, retained scene node, host, row, and control kind. Candidate ACK rebases the identity only when the same control survives.
- Chevron Enter and pressed Space toggle renderer-local expansion. Row Enter and Space remain unhandled because mounted React defines no row keyboard activation. Double-click navigation remains pointer-only.
- Added localized raw-tree fixture laws and mounted React coverage. React chevrons now expose `aria-expanded`.

## Validation

Focused React command:

```text
bun nx run '@semio-tech/framework-renderer-react:test' -- long --run '../../../../🧪️tests/🔬️engine-contract/🟦️.ts' --silent=false --reporter=verbose -t 'validates and flattens the shared raw virtual file system interaction law|mounts localized raw virtual file system controls'
```

Result: 2 passed, 699 skipped; Nx completed successfully in 32.1 seconds.

The VFS fixture and schema parse as JSON. `rustfmt --check` parsed the edited Rust sources and reported formatting differences in the concurrently edited files; `git diff --check` reported no whitespace faults.

Native laws awaiting the root-owned cargo session:

- `accepted_vfs_controls_match_the_shared_localized_keyboard_law`
- `virtual_file_system_scene_chrome_uses_the_shared_english_and_german_labels`
- `accepted_vfs_controls_publish_localized_buttons_and_reject_a_retired_row_action`
