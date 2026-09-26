# Virtual File System Expansion and Navigation

## Contract

The shared `virtual-file-system-interaction` fixture and schema define one raw parent-linked row tree. The scene does not carry renderer-produced `level` or `isExpanded` fields. Visibility cases cover the elided synthetic root, expanded and collapsed branch order, and derived levels. Navigation cases cover instance opening, media export, space navigation through path and studio URI forms, and refusal of an external URI.

## React implementation

`VirtualFileSystemHost` now parses `rowsJson` as raw `VirtualFileSystemNode` records. The domain-neutral `buildVirtualFileSystemSceneRows` helper derives parent buckets, visible DFS order, levels, child presence, and expansion state. The host initially expands rows authored with `hasChildren`, then owns chevron toggles locally. The underlying `VirtualFileSystem` continues to receive only visible flattened rows.

A row double-click resolves its `navigateUri` into the same actions as WGPU:

- `os://instance/{id}` -> `openInstance`
- `os://export/{id}/{format}` -> `exportMedia`
- `/spaces/{id}` and `studio:{id}` -> `navigateVirtualFileSystemNode`

Every action retains the host `surfaceId`. Unknown and malformed URIs dispatch nothing. Selection and context-menu behavior remain on the same visible row order.

## WGPU parity

The existing WGPU painter and hit resolver already own expansion and double-click routing. The shared native law now consumes the neutral fixture for both visible DFS output and every navigation route. The export parser was corrected to read the instance id and format after the `os://export/` prefix; the previous split selected the literal host segment `export` as the instance id.

Native law: `raw_tree_expansion_and_navigation_match_the_shared_react_law`.

## Validation

The fixture and schema parse with independent JSON parsers. Focused production React result: 2 passed, 695 skipped, exit 0. The native law is queued for the root-owned cargo session.
