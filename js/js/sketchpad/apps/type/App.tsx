// #region Header

// TypeApp.tsx

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

import { FC, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { guid, Representation, Type } from "../../../semio";
import { Canvas, Window } from "../../Canvas";
import { useAddFooterItem, useRemoveFooterItem } from "../../Footer";
import { useKitCommands, useType } from "../../kits/store";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { ToolType, useAppType } from "../../store";
import { KitSection } from "../kit/panels/Details";
import TypeScene from "./canvas/Scene";
import { AttributesSection, AuthorsSection, PortSection, PortsListSection, PortsMultipleSection, RepresentationsSection, TypeDetails } from "./panels/Details";
import { RepresentationDropdown } from "./RepresentationDropdown";
import { useTypeApp, useTypeAppCommands, useTypeAppSelection } from "./store";
import { ToolsToggleGroup } from "./Tools";

const App: FC = () => {
  const { t } = useTranslation();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();
  const { setActiveTool } = useTypeAppCommands();
  const app = useTypeApp((s) => s);
  const activeTool = app?.activeTool ?? ToolType.SELECTION_NORMAL;
  const selection = useTypeAppSelection();
  const [isDragOver, setIsDragOver] = useState(false);

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
    if (appType !== "type") return;

    addSection("toolbar", {
      id: "type-tools",
      label: t("semio.sketchpad.app.type.tools"),
      order: 0,
      defaultOpen: true,
      content: () => <ToolsToggleGroup />,
    });

    addFooterItem({
      id: "type-representation-selector",
      content: <RepresentationDropdown />,
      order: 0,
    });

    return () => {
      removeSection("toolbar", "type-tools");
      removeFooterItem("type-representation-selector");
    };
  }, [addSection, removeSection, addFooterItem, removeFooterItem, appType]);

  // Dynamic details panel based on selection
  useEffect(() => {
    if (appType !== "type") return;

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
        label: t("semio.sketchpad.app.type.port.title"),
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
      label: t("semio.sketchpad.app.type.title"),
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
      label: t("semio.sketchpad.app.kit.title"),
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
  }, [addSection, removeSection, appType, selection, t]);

  const type = useType() as Type | undefined;
  const kitCommands = useKitCommands();
  const typeAppCommands = useTypeAppCommands();

  // Handle file drops
  useEffect(() => {
    if (appType !== "type") return;

    const handleDrop = async (event: DragEvent) => {
      event.preventDefault();
      event.stopPropagation();
      setIsDragOver(false);

      const files = event.dataTransfer?.files;
      if (!files || files.length === 0 || !type || !kitCommands || !typeAppCommands) return;

      for (let i = 0; i < files.length; i++) {
        const file = files[i];

        // Create File object
        const newFileGuid = guid();
        const newFile = {
          guid: newFileGuid,
          path: file.name,
          size: file.size,
          createdAt: new Date(),
          updatedAt: new Date(),
        };

        // Create Representation that references the file
        const newRepresentationGuid = guid();
        const newRepresentation: Representation = {
          guid: newRepresentationGuid,
          file: newFileGuid,
          description: file.name,
        };

        // Add file to kit with blob
        await kitCommands.addFile(newFile, file);

        // Add representation to type
        await kitCommands.updateType(type.guid, {
          representations: {
            added: [newRepresentation],
          },
        });

        // Select the new representation
        typeAppCommands.setSelectedRepresentation?.(newRepresentationGuid);
      }
    };

    const handleDragOver = (event: DragEvent) => {
      event.preventDefault();
      event.stopPropagation();
      if (event.dataTransfer?.types.includes("Files")) {
        setIsDragOver(true);
      }
    };

    const handleDragLeave = (event: DragEvent) => {
      event.preventDefault();
      event.stopPropagation();
      // Only set to false if we're leaving the document entirely
      if (event.relatedTarget === null) {
        setIsDragOver(false);
      }
    };

    document.addEventListener("drop", handleDrop);
    document.addEventListener("dragover", handleDragOver);
    document.addEventListener("dragleave", handleDragLeave);

    return () => {
      document.removeEventListener("drop", handleDrop);
      document.removeEventListener("dragover", handleDragOver);
      document.removeEventListener("dragleave", handleDragLeave);
    };
  }, [appType, type, kitCommands, typeAppCommands]);

  return (
    <Canvas>
      <Window id="type-scene">
        <TypeScene isDragOver={isDragOver} />
      </Window>
    </Canvas>
  );
};

export default App;
