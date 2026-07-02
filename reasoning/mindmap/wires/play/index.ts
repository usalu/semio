// #region 🧲Header
/** @emoji 🔗 WIRES play harness: WIRES domain UI on `@puzzle/2d` normal-graph rendering. */
// #endregion 🧲Header

export {
  Playground2d as PlaygroundWires,
  PUZZLE_2D_PLAY_APP_ID as WIRES_PLAY_APP_ID,
  PUZZLE_2D_PLAY_CONTROLLER_ID as WIRES_PLAY_CONTROLLER_ID,
  buildPuzzle2dPlayRuntime as buildWiresPlayRuntime,
  puzzle2dPlayCmd,
} from "@semio-tech/puzzle-2d-play";

export const WIRES_PLAY_HIERARCHY_TAB_ID = "wires-play-hierarchy";
export const WIRES_PLAY_KINDS_TAB_ID = "wires-play-kinds";

const WIRES_PLAY_HIERARCHY_IDENTITY_PREFIX = "wires-play-hierarchy.identity.";
const WIRES_PLAY_HIERARCHY_RELATIONSHIP_PREFIX = "wires-play-hierarchy.relationship.";

export {
  edgeKindIdToRelationshipKind,
  relationshipKindDisplayName,
  relationshipKindToEdgeKindId,
  wiresIdentityLabelForNodeId,
  wiresRelationshipHierarchyLabel,
  wiresRelationshipKindForEdgeId,
  type RelationshipKind,
  type WiresFixtureIdentityV1,
  type WiresFixtureRelationshipV1,
  type WiresFixtureV1,
} from "../react/index.ts";

import { Playground2d, type Puzzle2dPlayHierarchyBuildOptions } from "@semio-tech/puzzle-2d-play";
import type { KindCatalogBundle, Puzzle2dFixtureV1 } from "@semio-tech/puzzle-2d-react";
import { type UiNode, type UiTreeItemNode, type UiTreeNode, type UiTreeSectionNode } from "@semio-tech/framework-playground-core";
import { puzzle2dFixtureMergedKindCatalogs } from "@semio-tech/puzzle-2d-react";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
  METABOLISM_WIRES_FIXTURE,
  relationshipKindDisplayName,
  wiresFixtureBoard,
  wiresIdentityLabelForNodeId,
  wiresRelationshipHierarchyLabel,
  wiresRelationshipKindForEdgeId,
  type WiresFixtureKindCatalogsV1,
  type WiresFixtureV1,
} from "../react/index.ts";

export type WiresPlayHierarchyBuildOptions = Puzzle2dPlayHierarchyBuildOptions;

export const WIRES_PLAY_FIXTURE: WiresFixtureV1 = METABOLISM_WIRES_FIXTURE;
export const WIRES_PLAY_DEFAULT_FIXTURE: Puzzle2dFixtureV1 = wiresFixtureBoard(METABOLISM_WIRES_FIXTURE);

export const WIRES_PLAY_FIXTURE_METABOLISM_ID = "metabolism";

export const WIRES_PLAY_FIXTURE_OPTIONS = [{ id: WIRES_PLAY_FIXTURE_METABOLISM_ID, label: "Metabolism" }] as const;

/** @emoji 🕸️ WIRES play defaults: continuous force-graph redraw (no auto-stop). */
export const WIRES_PLAY_LIVE_FORCE_GRAPH_DEFAULTS = {
  forceLayoutGravity: 0,
  puzzle2dRedrawPlaying: true,
  puzzle2dRedrawProgressiveAutoStopMs: 0,
} as const;

export function wiresPlayRelationshipKindForEdgeId(edgeId: string): ReturnType<typeof wiresRelationshipKindForEdgeId> {
  return wiresRelationshipKindForEdgeId(WIRES_PLAY_FIXTURE, edgeId);
}

export function wiresPlayIdentityLabelForNodeId(nodeId: string): string | undefined {
  return wiresIdentityLabelForNodeId(WIRES_PLAY_FIXTURE, nodeId);
}

export function wiresPlayRelationshipKindDisplayName(edgeId: string): string | undefined {
  const kind = wiresRelationshipKindForEdgeId(WIRES_PLAY_FIXTURE, edgeId);
  return kind != null ? relationshipKindDisplayName(kind) : undefined;
}

function wiresIdentityKindName(
  wiresFixture: WiresFixtureV1,
  catalogs: KindCatalogBundle,
  identityKindId: string | undefined,
): string | undefined {
  if (identityKindId == null || identityKindId.trim() === "") {
    return undefined;
  }
  return (
    wiresFixture.kindCatalogs?.identityKinds?.find((row) => row.id === identityKindId)?.name ??
    catalogs.nodes?.find((row) => row.id === identityKindId)?.name
  );
}

function wiresCatalogKindLabel(entry: { readonly id: string; readonly name?: string }): string {
  const display = entry.name?.trim();
  return display && display.length > 0 ? display : entry.id;
}

function wiresPlayKindCatalogSection(
  sectionId: string,
  label: string,
  entries: readonly { readonly id: string; readonly name?: string }[] | undefined,
): UiTreeSectionNode | null {
  if (!entries?.length) {
    return null;
  }
  const items: UiTreeItemNode[] = [...entries]
    .sort((a, b) => wiresCatalogKindLabel(a).localeCompare(wiresCatalogKindLabel(b)))
    .map((entry, index) => ({
      id: `${sectionId}.${index}.${entry.id}`,
      label: wiresCatalogKindLabel(entry),
      description: entry.id,
    }));
  return { id: sectionId, label, defaultOpen: false, items };
}

/** @emoji 🏷️ WIRES kinds tab: Identity kinds and Relationship kinds (not puzzle Nodes/Edges/Wires). */
export function buildWiresPlayKindsTree(catalogs: WiresFixtureKindCatalogsV1 | undefined): UiNode {
  const sections = [
    wiresPlayKindCatalogSection("wires-play-kinds.identity-kinds", "Identity kinds", catalogs?.identityKinds),
    wiresPlayKindCatalogSection("wires-play-kinds.relationship-kinds", "Relationship kinds", catalogs?.relationshipKinds),
  ].filter((section): section is UiTreeSectionNode => section !== null);
  if (!sections.length) {
    return {
      type: "tree",
      sections: [
        {
          id: "wires-play-kinds.empty",
          label: "Kinds",
          defaultOpen: false,
          items: [{ id: "wires-play-kinds.empty.msg", label: "No identity or relationship kind catalogs in this fixture" }],
        },
      ],
    };
  }
  return { type: "tree", sections };
}

/** @emoji 🌳 Maps graph selection ids to WIRES hierarchy tree row ids. */
export function wiresPlayHierarchyTreeSelectedIds(fixture: Puzzle2dFixtureV1, graphSelectionIds: readonly string[]): string[] {
  const out: string[] = [];
  for (const id of graphSelectionIds) {
    if (fixture.nodes.some((node) => node.id === id)) {
      out.push(`${WIRES_PLAY_HIERARCHY_IDENTITY_PREFIX}${id}`);
      continue;
    }
    if (fixture.edges.some((edge) => edge.id === id)) {
      out.push(`${WIRES_PLAY_HIERARCHY_RELATIONSHIP_PREFIX}${id}`);
    }
  }
  return out;
}

/** @emoji 🌳 Resolves a WIRES hierarchy tree row id back to a board graph id. */
export function wiresPlayHierarchyGraphIdFromTreeItemId(treeItemId: string): string | null {
  if (treeItemId.startsWith(WIRES_PLAY_HIERARCHY_IDENTITY_PREFIX)) {
    return treeItemId.slice(WIRES_PLAY_HIERARCHY_IDENTITY_PREFIX.length);
  }
  if (treeItemId.startsWith(WIRES_PLAY_HIERARCHY_RELATIONSHIP_PREFIX)) {
    return treeItemId.slice(WIRES_PLAY_HIERARCHY_RELATIONSHIP_PREFIX.length);
  }
  return null;
}

/** @emoji 🌳 Maps graph hover ids to WIRES hierarchy tree row ids. */
export function wiresPlayHierarchyTreeHighlightedIds(fixture: Puzzle2dFixtureV1, graphHoverId: string | null): readonly string[] {
  if (!graphHoverId) {
    return [];
  }
  return wiresPlayHierarchyTreeSelectedIds(fixture, [graphHoverId]);
}

function wiresHierarchyHoverHandlers(
  onHover: ((id: string | null) => void) | undefined,
  graphId: string,
): Pick<UiTreeItemNode, "onPointerEnter" | "onPointerLeave"> {
  if (!onHover) {
    return {};
  }
  return {
    onPointerEnter: () => onHover(graphId),
    onPointerLeave: () => onHover(null),
  };
}

/** @emoji 🌳 WIRES workbench hierarchy: Identities and Relationships (not puzzle Nodes/Edges). */
export function buildWiresPlayHierarchySections(
  wiresFixture: WiresFixtureV1,
  puzzleFixture: Puzzle2dFixtureV1,
  selectionIds: readonly string[],
  options?: WiresPlayHierarchyBuildOptions,
): UiTreeNode {
  const omitItemSelection = options?.omitItemSelection === true;
  const onHover = options?.onHover;
  const selectedIds = omitItemSelection ? new Set<string>() : new Set(selectionIds);
  const catalogs = puzzle2dFixtureMergedKindCatalogs(puzzleFixture);
  const identityItems: UiTreeItemNode[] = wiresFixture.identities.map((identity) => {
    const kindName = wiresIdentityKindName(wiresFixture, catalogs, identity.identityKind);
    const description = kindName != null && kindName !== identity.label ? kindName : undefined;
    return {
      id: `${WIRES_PLAY_HIERARCHY_IDENTITY_PREFIX}${identity.nodeId}`,
      label: identity.label,
      ...(description !== undefined ? { description } : {}),
      ...(omitItemSelection ? {} : { isSelected: selectedIds.has(identity.nodeId) }),
      command: puzzle2dPlayCmd("hierarchySelect", { id: identity.nodeId }),
      ...wiresHierarchyHoverHandlers(onHover, identity.nodeId),
    };
  });
  const relationshipItems: UiTreeItemNode[] = puzzleFixture.edges.map((edge) => ({
    id: `${WIRES_PLAY_HIERARCHY_RELATIONSHIP_PREFIX}${edge.id}`,
    label: wiresRelationshipHierarchyLabel(wiresFixture, edge.id) ?? edge.id,
    ...(omitItemSelection ? {} : { isSelected: selectedIds.has(edge.id) }),
    command: puzzle2dPlayCmd("hierarchySelect", { id: edge.id }),
    ...wiresHierarchyHoverHandlers(onHover, edge.id),
  }));
  return {
    type: "tree",
    sections: [
      {
        id: "wires-play-hierarchy.identities",
        label: "Identities",
        defaultOpen: false,
        items: identityItems.length ? identityItems : [{ id: "wires-play-hierarchy.identities.empty", label: "(none)" }],
      },
      {
        id: "wires-play-hierarchy.relationships",
        label: "Relationships",
        defaultOpen: false,
        items: relationshipItems.length ? relationshipItems : [{ id: "wires-play-hierarchy.relationships.empty", label: "(none)" }],
      },
    ],
  } as UiTreeNode;
}

//#region 🔖Boot
if (
  typeof document !== "undefined" &&
  document.getElementById("root") != null &&
  !import.meta.vitest &&
  import.meta.env.PUZZLE_PLAY_ENTRY === "wires"
) {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootWiresPlay } = await import("@semio-tech/framework-playground-renderer-react/reasoning/wires");
    bootWiresPlay(new Playground2d());
  })();
}
//#endregion 🔖Boot

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("wires play fixture", () => {
    it("metabolism board has seven identities and nine relationships", async () => {
      const { METABOLISM_WIRES_FIXTURE, wiresRelationshipKindForEdgeId } = await import("../react/index.ts");
      expect(METABOLISM_WIRES_FIXTURE.board.nodes.length).toBe(7);
      expect(METABOLISM_WIRES_FIXTURE.board.edges.length).toBe(9);
      expect(wiresRelationshipKindForEdgeId(METABOLISM_WIRES_FIXTURE, "wires-rel-owns-capsule")).toBe("owns");
    });

    it("buildWiresPlayHierarchySections uses Identities and Relationships groups", () => {
      const tree = buildWiresPlayHierarchySections(WIRES_PLAY_FIXTURE, WIRES_PLAY_DEFAULT_FIXTURE, []);
      const groups = tree.sections.map((section) => section.label);
      expect(groups).toEqual(["Identities", "Relationships"]);
      const relationshipsSection = tree.sections.find((section) => section.label === "Relationships");
      const rel = relationshipsSection?.items?.find((row) => row.id === "wires-play-hierarchy.relationship.wires-rel-is-capital");
      expect(rel?.label).toBe("Is: Bridge → Capital");
    });

    it("wiresPlayHierarchyTreeSelectedIds uses identity and relationship row ids", () => {
      const fixture = WIRES_PLAY_DEFAULT_FIXTURE;
      expect(wiresPlayHierarchyTreeSelectedIds(fixture, ["wires-rel-is-capital"])).toEqual([
        "wires-play-hierarchy.relationship.wires-rel-is-capital",
      ]);
      expect(wiresPlayHierarchyGraphIdFromTreeItemId("wires-play-hierarchy.identity.wires-identity-tower")).toBe("wires-identity-tower");
    });

    it("buildWiresPlayKindsTree lists identity and relationship kind sections", () => {
      const tree = buildWiresPlayKindsTree(WIRES_PLAY_FIXTURE.kindCatalogs);
      expect(tree.type).toBe("tree");
      if (tree.type !== "tree") return;
      expect(tree.sections.map((section) => section.label)).toEqual(["Identity kinds", "Relationship kinds"]);
    });

    it("re-exports relationshipKindDisplayName for playground renderer", async () => {
      const play = await import("./index.ts");
      expect(play.relationshipKindDisplayName("owns")).toBe("Owns");
      expect(play.wiresPlayRelationshipKindDisplayName("wires-rel-has-capsule")).toBe("Has");
    });

    it("WIRES_PLAY_LIVE_FORCE_GRAPH_DEFAULTS enables continuous force-graph redraw", () => {
      expect(WIRES_PLAY_LIVE_FORCE_GRAPH_DEFAULTS.puzzle2dRedrawPlaying).toBe(true);
      expect(WIRES_PLAY_LIVE_FORCE_GRAPH_DEFAULTS.puzzle2dRedrawProgressiveAutoStopMs).toBe(0);
      expect(WIRES_PLAY_LIVE_FORCE_GRAPH_DEFAULTS.forceLayoutGravity).toBe(0);
    });

    it("metabolism kind catalogs use design-token color references", () => {
      const catalogs = WIRES_PLAY_FIXTURE.board.meta?.kindCatalogs;
      const owns = catalogs?.relationshipKinds?.find((row) => row.id === "wires.owns");
      const isKind = catalogs?.relationshipKinds?.find((row) => row.id === "wires.is");
      expect(owns?.color).toBe("var(--color-muted-foreground)");
      expect(isKind?.color).toBe("var(--color-secondary)");
      expect(catalogs?.identityKinds?.[0]?.color).toMatch(/^var\(--color-/u);
    });
  });
}
// #endregion 🧪Tests
