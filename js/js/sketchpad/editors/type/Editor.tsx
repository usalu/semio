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
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { EditorType, useEditorType } from "../../store";
import TypeScene from "./canvas/Scene";
import { AttributesSection, AuthorsSection, PortsSection, RepresentationsSection, TypeDetails } from "./panels/Details";
import { ToolsToggleGroup } from "./Tools";

const Editor: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const editorType = useEditorType();

  useEffect(() => {
    if (editorType !== EditorType.TYPE) return;

    addSection("toolbar", {
      id: "type-tools",
      label: "Tools",
      order: 0,
      defaultOpen: true,
      content: () => <ToolsToggleGroup />,
    });

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
      id: "type-ports",
      label: "Ports",
      order: 2,
      defaultOpen: true,
      content: () => <PortsSection />,
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

    return () => {
      removeSection("toolbar", "type-tools");
      removeSection("details", "type-details");
      removeSection("details", "type-representations");
      removeSection("details", "type-ports");
      removeSection("details", "type-authors");
      removeSection("details", "type-attributes");
    };
  }, [addSection, removeSection, editorType]);

  return <TypeScene />;
};

export default Editor;
