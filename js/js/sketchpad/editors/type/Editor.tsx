// #region Header

// TypeEditor.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import { FC, useEffect } from "react";
import { Canvas, Window } from "../../Canvas";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { EditorType, ToolType, useEditorType } from "../../store";
import TypeScene from "./canvas/Scene";
import { AttributesSection, AuthorsSection, PortSection, PortsListSection, PortsMultipleSection, RepresentationsSection, TypeDetails } from "./panels/Details";
import { useTypeEditor, useTypeEditorCommands, useTypeEditorSelection } from "./store";
import { ToolsToggleGroup } from "./Tools";

const Editor: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const editorType = useEditorType();
  const { setActiveTool } = useTypeEditorCommands();
  const editor = useTypeEditor((s) => s);
  const activeTool = editor?.activeTool ?? ToolType.SELECTION_NORMAL;
  const selection = useTypeEditorSelection();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (activeTool === ToolType.SELECTION_NORMAL) {
        if (e.shiftKey && !e.ctrlKey && !e.metaKey) {
          setActiveTool(ToolType.SELECTION_ADDITIVE);
        } else if ((e.ctrlKey || e.metaKey) && !e.shiftKey) {
          setActiveTool(ToolType.SELECTION_SUBTRACTIVE);
        }
      }
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (activeTool === ToolType.SELECTION_ADDITIVE && !e.shiftKey) {
        setActiveTool(ToolType.SELECTION_NORMAL);
      } else if (activeTool === ToolType.SELECTION_SUBTRACTIVE && !e.ctrlKey && !e.metaKey) {
        setActiveTool(ToolType.SELECTION_NORMAL);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [activeTool, setActiveTool]);

  useEffect(() => {
    if (editorType !== "type") return;

    addSection("toolbar", {
      id: "type-tools",
      label: "Tools",
      order: 0,
      defaultOpen: true,
      content: () => <ToolsToggleGroup />,
    });

    return () => {
      removeSection("toolbar", "type-tools");
    };
  }, [addSection, removeSection, editorType]);

  // Dynamic details panel based on selection
  useEffect(() => {
    if (editorType !== "type") return;

    const hasPorts = selection?.ports && selection.ports.length > 0;
    const hasMultiplePorts = selection?.ports && selection.ports.length > 1;
    const hasSinglePort = selection?.ports && selection.ports.length === 1;

    // Remove all previous sections
    removeSection("details", "type-details");
    removeSection("details", "type-representations");
    removeSection("details", "type-ports-list");
    removeSection("details", "type-authors");
    removeSection("details", "type-attributes");
    removeSection("details", "type-port");
    removeSection("details", "type-ports-multiple");

    if (!hasPorts) {
      // No selection: show Type section
      addSection("details", {
        id: "type-details",
        label: "Type",
        order: 0,
        defaultOpen: true,
        content: () => <TypeDetails />,
      });

      addSection("details", {
        id: "type-representations",
        label: "Representations",
        order: 1,
        defaultOpen: true,
        content: () => <RepresentationsSection />,
      });

      addSection("details", {
        id: "type-ports-list",
        label: "Ports",
        order: 2,
        defaultOpen: true,
        content: () => <PortsListSection />,
      });

      addSection("details", {
        id: "type-authors",
        label: "Authors",
        order: 3,
        defaultOpen: true,
        content: () => <AuthorsSection />,
      });

      addSection("details", {
        id: "type-attributes",
        label: "Attributes",
        order: 4,
        defaultOpen: true,
        content: () => <AttributesSection />,
      });
    } else if (hasSinglePort) {
      // Single port selected: show Port section
      addSection("details", {
        id: "type-port",
        label: "Port",
        order: 0,
        defaultOpen: true,
        content: () => <PortSection portGuid={selection.ports![0]} />,
      });
    } else if (hasMultiplePorts) {
      // Multiple ports selected: show Ports section
      addSection("details", {
        id: "type-ports-multiple",
        label: "Ports",
        order: 0,
        defaultOpen: true,
        content: () => <PortsMultipleSection portGuids={selection.ports!} />,
      });
    }

    return () => {
      removeSection("details", "type-details");
      removeSection("details", "type-representations");
      removeSection("details", "type-ports-list");
      removeSection("details", "type-authors");
      removeSection("details", "type-attributes");
      removeSection("details", "type-port");
      removeSection("details", "type-ports-multiple");
    };
  }, [addSection, removeSection, editorType, selection]);

  return (
    <Canvas>
      <Window id="type-scene">
        <TypeScene />
      </Window>
    </Canvas>
  );
};

export default Editor;
