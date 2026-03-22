// #region 🔖Header
// [👤semio📚3dm🖱️ui🗃️src💻rhinopanel](repo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/RhinoPanel.tsx)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Main panel component rendering the semio kit/type/model tree view for Rhino.

// #endregion 🔖Header

// #region 🔖Imports
// [👤semio📚3dm🖱️ui🗃️src💻rhinopanel🔖imports](repo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/RhinoPanel.tsx/s/Imports)
// Imports MUST include React, semio types, bridge API, and Lucide icons from assets.

import React, { useCallback, useEffect, useState } from "react";
import type { Kit, Type as SemioType, Design, Model } from "@semio/js";
import { importKit } from "@semio/js";
import { importApi } from "./bridge";
import { ChevronDownIcon, ChevronRightIcon, AddIcon, TypeIcon, LayoutIcon } from "@semio/assets";

// #endregion 🔖Imports

// #region 🔖TreeNodeKind
// [👤semio📚3dm🖱️ui🗃️src💻rhinopanel🔖treenodekind](repo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/RhinoPanel.tsx/s/TreeNodeKind)
// Tree node kind MUST distinguish between structural folders and selectable items.

type TreeNodeKind = "kits" | "kit" | "types" | "type" | "models" | "model" | "designs" | "design";

interface TreeNode {
  id: string;
  label: string;
  kind: TreeNodeKind;
  children?: TreeNode[];
  data?: unknown;
}

// #endregion 🔖TreeNodeKind

// #region 🔖BuildTree
// [👤semio📚3dm🖱️ui🗃️src💻rhinopanel🔖buildtree](repo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/RhinoPanel.tsx/s/BuildTree)
// Tree builder MUST convert Kit[] into the specified tree structure.

function buildTree(kits: Kit[]): TreeNode {
  return {
    id: "root-kits",
    label: "Kits",
    kind: "kits",
    children: kits.map((kit) => ({
      id: `kit-${kit.guid}`,
      label: kit.name,
      kind: "kit" as const,
      data: kit,
      children: [
        {
          id: `kit-${kit.guid}-types`,
          label: "Types",
          kind: "types" as const,
          children: (kit.types ?? []).map((type) => ({
            id: `type-${type.guid}`,
            label: type.name,
            kind: "type" as const,
            data: type,
            children: [
              {
                id: `type-${type.guid}-models`,
                label: "Models",
                kind: "models" as const,
                children: (type.models ?? []).map((model) => ({
                  id: `model-${model.guid}`,
                  label: model.name ?? model.guid.substring(0, 8),
                  kind: "model" as const,
                  data: { model, type, kit },
                })),
              },
            ],
          })),
        },
        {
          id: `kit-${kit.guid}-designs`,
          label: "Designs",
          kind: "designs" as const,
          children: (kit.designs ?? []).map((design) => ({
            id: `design-${design.guid}`,
            label: design.name,
            kind: "design" as const,
            data: design,
          })),
        },
      ],
    })),
  };
}

// #endregion 🔖BuildTree

// #region 🔖TreeNodeComponent
// [👤semio📚3dm🖱️ui🗃️src💻rhinopanel🔖treenodecomponent](repo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/RhinoPanel.tsx/s/TreeNodeComponent)
// Tree node component MUST render expandable/collapsible nodes with action buttons.

function TreeNodeIcon({ kind }: { kind: TreeNodeKind }) {
  switch (kind) {
    case "type":
      return <TypeIcon className="h-4 w-4 text-blue-500" />;
    case "design":
      return <LayoutIcon className="h-4 w-4 text-green-500" />;
    default:
      return null;
  }
}

function TreeNodeView({ node, depth, onImportKit, onImportModel }: { node: TreeNode; depth: number; onImportKit: () => void; onImportModel: (data: { model: Model; type: SemioType; kit: Kit }) => void }) {
  const [expanded, setExpanded] = useState(depth < 2);
  const hasChildren = node.children && node.children.length > 0;
  const paddingLeft = depth * 16;

  const handleToggle = useCallback(() => {
    if (hasChildren) setExpanded((prev) => !prev);
  }, [hasChildren]);

  const handleAction = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      if (node.kind === "kits") {
        onImportKit();
      } else if (node.kind === "model") {
        onImportModel(node.data as { model: Model; type: SemioType; kit: Kit });
      }
    },
    [node, onImportKit, onImportModel],
  );

  const showAction = node.kind === "kits" || node.kind === "model";

  return (
    <div>
      <div className="flex cursor-pointer items-center gap-1 rounded px-1 py-0.5 hover:bg-zinc-100 dark:hover:bg-zinc-800" style={{ paddingLeft }} onClick={handleToggle}>
        {hasChildren ? expanded ? <ChevronDownIcon className="h-4 w-4 shrink-0 text-zinc-400" /> : <ChevronRightIcon className="h-4 w-4 shrink-0 text-zinc-400" /> : <span className="inline-block h-4 w-4 shrink-0" />}
        <TreeNodeIcon kind={node.kind} />
        <span className="truncate text-sm">{node.label}</span>
        {showAction && (
          <button className="ml-auto shrink-0 rounded p-0.5 text-zinc-400 hover:bg-zinc-200 hover:text-zinc-700 dark:hover:bg-zinc-700 dark:hover:text-zinc-200" onClick={handleAction} title={node.kind === "kits" ? "Import Kit" : "Import Model"}>
            <AddIcon className="h-3.5 w-3.5" />
          </button>
        )}
      </div>
      {expanded && hasChildren && node.children!.map((child) => <TreeNodeView key={child.id} node={child} depth={depth + 1} onImportKit={onImportKit} onImportModel={onImportModel} />)}
    </div>
  );
}

// #endregion 🔖TreeNodeComponent

// #region 🔖RhinoPanel
// [👤semio📚3dm🖱️ui🗃️src💻rhinopanel🔖rhinopanel](repo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/RhinoPanel.tsx/s/RhinoPanel)
// RhinoPanel MUST manage loaded kits and dispatch import actions.

export function RhinoPanel() {
  const [kits, setKits] = useState<Kit[]>([]);
  const [importUrl, setImportUrl] = useState("");
  const [isImporting, setIsImporting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const tree = buildTree(kits);

  const handleImportKit = useCallback(async () => {
    if (!importUrl.trim()) return;
    setIsImporting(true);
    setError(null);
    try {
      const response = await fetch(importUrl.trim());
      const blob = await response.blob();
      const result = await importKit(blob);
      if (result.kit) {
        setKits((prev) => {
          const existing = prev.findIndex((k) => k.guid === result.kit!.guid);
          if (existing >= 0) {
            const next = [...prev];
            next[existing] = result.kit!;
            return next;
          }
          return [...prev, result.kit!];
        });
        setImportUrl("");
      } else {
        setError("Failed to import kit.");
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Import failed.");
    } finally {
      setIsImporting(false);
    }
  }, [importUrl]);

  const handleImportModel = useCallback(async (data: { model: Model; type: SemioType; kit: Kit }) => {
    const { model, type, kit } = data;
    try {
      const file = kit.files?.find((f) => f.guid === model.file.guid);
      const fileUrl = file?.url ?? "";
      const tagNames = (model.tags ?? []).map((t) => {
        const tag = kit.tags?.find((kt) => kt.guid === t.guid);
        return tag?.value ?? "";
      });
      await importApi.importModel({
        kitName: kit.name,
        typeName: type.name,
        modelGuid: model.guid,
        fileUrl,
        tags: tagNames,
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Model import failed.");
    }
  }, []);

  return (
    <div className="flex h-full flex-col bg-white text-zinc-900 dark:bg-zinc-900 dark:text-zinc-100">
      {/* Header */}
      <div className="border-b border-zinc-200 px-3 py-2 dark:border-zinc-700">
        <h1 className="text-sm font-semibold">semio</h1>
      </div>

      {/* Import URL */}
      <div className="border-b border-zinc-200 px-3 py-2 dark:border-zinc-700">
        <div className="flex gap-1">
          <input
            type="text"
            className="min-w-0 flex-1 rounded border border-zinc-300 bg-transparent px-2 py-1 text-xs focus:border-blue-500 focus:outline-none dark:border-zinc-600"
            placeholder="Kit URL (.zip)"
            value={importUrl}
            onChange={(e) => setImportUrl(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleImportKit()}
          />
          <button className="rounded bg-blue-600 px-2 py-1 text-xs text-white hover:bg-blue-700 disabled:opacity-50" onClick={handleImportKit} disabled={isImporting || !importUrl.trim()}>
            {isImporting ? "..." : "Import"}
          </button>
        </div>
        {error && <p className="mt-1 text-xs text-red-500">{error}</p>}
      </div>

      {/* Tree View */}
      <div className="flex-1 overflow-y-auto px-1 py-1">
        {kits.length === 0 ? (
          <div className="px-3 py-4 text-center text-xs text-zinc-400">No kits loaded. Import a kit to get started.</div>
        ) : (
          <TreeNodeView
            node={tree}
            depth={0}
            onImportKit={() => {
              /* Focus the URL input */
            }}
            onImportModel={handleImportModel}
          />
        )}
      </div>

      {/* Footer */}
      <div className="border-t border-zinc-200 px-3 py-1 dark:border-zinc-700">
        <span className="text-xs text-zinc-400">
          {kits.length} kit{kits.length !== 1 ? "s" : ""} loaded
        </span>
      </div>
    </div>
  );
}

// #endregion 🔖RhinoPanel
