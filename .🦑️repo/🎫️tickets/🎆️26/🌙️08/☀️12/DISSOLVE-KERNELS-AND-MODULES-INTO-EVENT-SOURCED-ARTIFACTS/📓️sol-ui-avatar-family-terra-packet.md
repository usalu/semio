# Terra Packet UI-Avatar-01: Table Avatar Extraction and Dead Facet Removal

## Preconditions

- Read AGENTS/audit; apply_patch only, no modifying Git.
- Require all five clean source hashes recorded in the audit.
- Current shared React index hash will be announced by coordinator; never edit it.

## Terra Writable Closure

1. Old `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👤️Avatar/🟦️component.tsx` and `🧪️story.tsx` (delete after move).
2. New `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🟦️component.tsx` and `🧪️story.tsx` (create).
3. HistoryTable and VirtualFileSystem components (update only the direct import path).
4. `.storybook/stories/ui/🖐️DragAndDrop.stories.tsx` (remove DraggableAvatar import/reference and use concise local initials markup while preserving the reorder/DragHandle story).
5. Unique acceptance `📓️terra-ui-avatar-family-table-avatar-extraction-acceptance.md`.

## Required Result

- New TableAvatar file owns `TableAvatarProps` and `TableAvatar`.
- Base Radix Avatar/Image/Fallback helpers remain private within the same file and are not exported.
- DraggableAvatar and its props do not survive.
- New TableAvatar story covers only TableAvatar; no base/Draggable story remains.
- Both production direct imports resolve to the new path.
- Do not touch the shared index, manifests/locks, generated census, other UI components, protected renderer, or plugins.

Checkpoint and wait after source move. Coordinator removes the broad Avatar registrar and unused package-level AvatarPrimitive import, adding the specific TableAvatar import/export. After signal, run stale old-family/path scans, new-path consumer scans, diff checks, and UI React lint/typecheck/test-quick/build once without unrelated repair.
