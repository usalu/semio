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
import { useTranslation } from "react-i18next";
import { Canvas, Window } from "../../Canvas";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { ToolType, useEditorType } from "../../store";
import { KitSection } from "../kit/panels/Details";
import TypeScene from "./canvas/Scene";
import { AttributesSection, AuthorsSection, PortSection, PortsListSection, PortsMultipleSection, RepresentationsSection, TypeDetails } from "./panels/Details";
import { useTypeEditor, useTypeEditorCommands, useTypeEditorSelection } from "./store";
import { ToolsToggleGroup } from "./Tools";

const Editor: FC = () => {
  const { t } = useTranslation();
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
    removeSection("details", "type-port");
    removeSection("details", "type-ports-multiple");
    removeSection("details", "type-kit");

    if (hasSinglePort) {
      // Single port selected: show Port section then Type section
      addSection("details", {
        id: "type-port",
        label: t("port.title"),
        order: 0,
        defaultOpen: true,
        content: () => <PortSection portGuid={selection.ports![0]} />,
      });
    } else if (hasMultiplePorts) {
      // Multiple ports selected: show Ports section then Type section
      addSection("details", {
        id: "type-ports-multiple",
        label: t("ports.multipleTitle", { count: selection.ports!.length }),
        order: 0,
        defaultOpen: true,
        content: () => <PortsMultipleSection portGuids={selection.ports!} />,
      });
    }

    // Always show Type section (with all subsections)
    addSection("details", {
      id: "type-details",
      label: t("type.title"),
      order: 50,
      defaultOpen: true,
      content: () => (
        <>
          <TypeDetails />
          <RepresentationsSection />
          <PortsListSection />
          <AuthorsSection />
          <AttributesSection />
        </>
      ),
    });

    // Always add Kit section at the bottom
    addSection("details", {
      id: "type-kit",
      label: t("kit.title"),
      order: 100,
      defaultOpen: true,
      content: () => <KitSection />,
    });

    return () => {
      removeSection("details", "type-details");
      removeSection("details", "type-port");
      removeSection("details", "type-ports-multiple");
      removeSection("details", "type-kit");
    };
  }, [addSection, removeSection, editorType, selection, t]);

  return (
    <Canvas>
      <Window id="type-scene">
        <TypeScene />
      </Window>
    </Canvas>
  );
};

export default Editor;
