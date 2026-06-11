/** Reference implementation — merged into playground renderer. */

function buildPuzzle2dPlayInspectorTree(fixture: Puzzle2dFixtureV1, selectionIds: ReadonlySet<string>): UiTreeNode {
  const kindCatalogs = puzzle2dFixtureMergedKindCatalogs(fixture);
  const { nodeIds, handleIds, edgeIds, unknownIds } = classifyPuzzle2dPlayInspectorSelection(fixture, selectionIds);
  const sections: UiSectionNode[] = [];
  if (nodeIds.length === 0 && handleIds.length === 0 && edgeIds.length === 0 && unknownIds.length === 0) {
    sections.push({
      type: "section",
      id: "puzzle-2d-play-inspector.empty",
      label: "Detail",
      children: [{
        type: "text",
        value: PUZZLE_2D_PLAY_IS_WIRES
          ? "No selection. Click the graph or pick an identity or relationship in the hierarchy."
          : "No selection. Click the graph or pick a row in the hierarchy.",
      }],
    });
    return uiDeclarativeSectionsToTree(sections);
  }
  // ... node/handle/edge sections with field + control items
  return uiDeclarativeSectionsToTree(sections);
}
