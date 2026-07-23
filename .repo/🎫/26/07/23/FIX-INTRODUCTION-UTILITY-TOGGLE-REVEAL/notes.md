# Fix Introduction Utility Toggle Reveal

## Bug
Last Aggregator intro step (`transform-utility`) could not be completed: utility toggles never appeared.

## Root cause
`utilityBarFoldedFor` compared `elementIdSegment(windowId)` to the kind id segment. Live panes are instances (`puzzle3d-main-top`, `puzzle3d-main-perspective`), so `puzzle3dMainTop !== puzzle3dMain` and force-unfold never ran. Same bug for Actions-rail unfold.

## Fix
- `introductionTargetsWindow(windowId, windowKindId, targetKindId, targetSegment?)` matches kind **or** instance.
- `utilityBarFoldedFor` / `actionsFoldedFor` take `windowKindId` and use that helper.
- Transform step also `show`s the window kind so panes elevate while the utility mounts.
