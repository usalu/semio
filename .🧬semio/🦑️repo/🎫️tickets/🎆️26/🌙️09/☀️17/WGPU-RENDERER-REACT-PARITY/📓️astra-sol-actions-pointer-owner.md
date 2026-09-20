# Actions retained-pointer ownership

## Runtime evidence

Checkpoint 16's `pane-actions-scroll` step succeeds in React and fails in WGPU. React exposes the scrolled `engagementAbort` row as a 24 px row and dispatches the action from a physical press/release. WGPU's published hit at the same gesture is:

```text
TreeItem tree.label.puzzle3d-main-top/framework.section.engagements/action.engagementAbort
```

The WGPU console records the normalized pointer down but no Shell pointer-down line. It records only the later Shell pointer-up and retained release. No `engagementAbort` action is published.

Evidence:

- `🗑️generated/astra-runtime/checkpoint-16-full/steps.json`
- `🗑️generated/astra-runtime/checkpoint-16-full/wgpu/console.txt:6997-7010`

## Ownership trace

`AppInteractionState::handle_pointer_button` captures `ShellState::pointer_owner_at` on pointer down. `pointer_owner_at` currently recognizes modal layers, open anchored-panel geometry, and the static `pointer_hit_owner` kind/namespace classifier. A retained Actions row is a `TreeItem`, so the static classifier returns `Surface`. The world below the pane consumes pointer down. Release reaches the Shell fallback and resolves the retained row, but the retained event router has no matching press owner and correctly refuses activation.

The complete chrome walk already records every retained control id in `retained_hit_windows` together with its retained body and promotes that owner map atomically with the hit registry. That map is the authoritative DOM-layer equivalent. Scene-native generic and World3d hits do not occur in it.

## Fail-first law

`the_mounted_actions_tree_keeps_fixed_rows_clips_the_terminal_row_and_scrolls_to_it` now drives the actual dense Actions document through the normal chrome walk, scrolls to the terminal row, resolves its retained body, and applies the same `PointerCapture` press/repaint/release sequence as the renderer. It requires:

1. the retained row owns pointer down as `Chrome`;
2. capture keeps the release in `Chrome` across repaint;
3. the release publishes exactly one `engagementAbort`.

Native 50 restored only the pre-repair ownership branch and ran the strengthened law. It failed at the ownership assertion with actual `Surface` and expected `Chrome`. Native 49, with the repair present, ran the same law green. The full native and fresh browser gates remain root-owned.

## Production repair

`pointer_owner_at` now classifies the current topmost hit as `Chrome` when that exact hit id exists in the atomically published `retained_hit_windows` map, before falling back to the static classifier. This keeps raw engine-surface hits as `Surface`, while routing every retained document control through the Shell ingress that owns its event router.
