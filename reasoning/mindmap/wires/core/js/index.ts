// #region 🧲Header
/** @emoji 🔗 `@semio-tech/reasoning-mindmap-wires-core` — WIRES app logic on `@puzzle/2d`. */
// #endregion 🧲Header

import {
  PUZZLE_2D_PLAY_APP_ID,
  PUZZLE_2D_PLAY_CONTROLLER_ID,
  buildPuzzle2dPlayRuntime,
  puzzle2dPlayCmd,
  type Puzzle2dPlayHierarchyBuildOptions,
} from "@semio-tech/puzzle-2d-core";

export {
  buildPuzzle2dPlayRuntime as buildWiresPlayRuntime,
};

export const WIRES_PLAY_APP_ID = PUZZLE_2D_PLAY_APP_ID;
export const WIRES_PLAY_CONTROLLER_ID = PUZZLE_2D_PLAY_CONTROLLER_ID;
export { puzzle2dPlayCmd };

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
  type WiresFixtureIdentity,
  type WiresFixtureRelationship,
  type WiresFixture,
} from "../../react/index.ts";

import type { KindCatalogBundle, Puzzle2dFixture } from "@semio-tech/puzzle-2d-react";
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
  type WiresFixtureKindCatalogs,
  type WiresFixture,
} from "../../react/index.ts";

export type WiresPlayHierarchyBuildOptions = Puzzle2dPlayHierarchyBuildOptions;

export const WIRES_PLAY_FIXTURE: WiresFixture = METABOLISM_WIRES_FIXTURE;
export const WIRES_PLAY_DEFAULT_FIXTURE: Puzzle2dFixture = wiresFixtureBoard(METABOLISM_WIRES_FIXTURE);

export const WIRES_PLAY_EXAMPLE_METABOLISM_ID = "metabolism";

export const WIRES_PLAY_EXAMPLE_OPTIONS = [{ id: WIRES_PLAY_EXAMPLE_METABOLISM_ID, label: "Metabolism" }] as const;

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
  wiresFixture: WiresFixture,
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
export function buildWiresPlayKindsTree(catalogs: WiresFixtureKindCatalogs | undefined): UiNode {
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
export function wiresPlayHierarchyTreeSelectedIds(fixture: Puzzle2dFixture, graphSelectionIds: readonly string[]): string[] {
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
export function wiresPlayHierarchyTreeHighlightedIds(fixture: Puzzle2dFixture, graphHoverId: string | null): readonly string[] {
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
  wiresFixture: WiresFixture,
  puzzleFixture: Puzzle2dFixture,
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

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("wires play fixture", () => {
    it("metabolism board has seven identities and nine relationships", async () => {
      const { METABOLISM_WIRES_FIXTURE, wiresRelationshipKindForEdgeId } = await import("../../react/index.ts");
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

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for wires. */
export function buildReasoningWiresProgramDefinition(): PlatformDefinition {
	return {
		id: "reasoning.wires",
		name: "Reasoning Wires",
		apiVersion: "1",
		apps: [{ id: "wires", label: "Wires", controllerId: WIRES_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖Play
import { createPlaygroundApp } from "@semio-tech/framework-playground-core";
import { registerPuzzle2dPlayDeclarativeBodies } from "@semio-tech/puzzle-2d-core";

export const wiresPlayAppDefinition = createPlaygroundApp({
	id: WIRES_PLAY_APP_ID,
	label: "Wires",
	controllerId: WIRES_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "wires",
		resolveDedupe: ["react", "react-dom", "three", "@semio-tech/puzzle-2d-react", "@semio-tech/reasoning-mindmap-wires-react"],
		watchIgnored: [
			"../../../../puzzle/2d/rs/lib.rs",
			"../../../../puzzle/2d/rs/target/**",
			"../../../../puzzle/2d/rs/Cargo.toml",
			"../../../../puzzle/2d/rs/script.ts",
		],
		optimizeDeps: {
			include: [
				"react",
				"react-dom",
				"react/jsx-runtime",
				"react/jsx-dev-runtime",
				"three",
				"@react-three/fiber",
				"@react-three/drei",
				"lucide-react",
				"@semio-tech/infinite-cavas-react-renderer",
				"@semio-tech/puzzle-2d-react",
				"@semio-tech/reasoning-mindmap-react",
			],
		},
	},
	createRuntime: () => buildPuzzle2dPlayRuntime(),
	registerBodies: () => registerPuzzle2dPlayDeclarativeBodies(),
	bootRenderer: async (pg) => {
		const { bootWiresPlay } = await import("@semio-tech/puzzle-2d-react/play");
		bootWiresPlay(pg);
	},
});
//#endregion 🔖Play
