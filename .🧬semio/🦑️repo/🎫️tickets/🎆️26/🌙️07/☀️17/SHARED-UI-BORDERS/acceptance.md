# Shared UI Borders Acceptance

- Preserve the silhouette and chip design, including continuous outlines on active windows.
- Merge coincident borders across sibling windows and across container/control hierarchy; never suppress unrelated edges.
- Resolve shared-edge highlighting deterministically: hover takes priority over active, focus, and neutral decoration, independent of sibling order.
- Join window-edge buttons and pane toggles to their window outline without redundant handle separators.
- Join navbar edges and adjacent name chips without changing chip geometry.
- Ensure open menus are not clipped by their window or hidden behind neighboring panes; preserve keyboard and pointer interaction.
- Cover shared-edge geometry and priority with language-neutral cases and executable tests, including comparison against an independent implementation where available.
- Validate all modified shared UI implementations and document any unavailable runtime/browser verification explicitly.
