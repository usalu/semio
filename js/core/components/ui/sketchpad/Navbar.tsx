// #region Header

// Navbar.tsx

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

import { ArrowLeft, ArrowRight, ArrowUp, Fullscreen, Home, Info, MessageCircle, Minus, Settings, Square, Terminal, Wrench, X } from "lucide-react";
import { createContext, FC, ReactNode, useCallback, useContext, useState } from "react";
import { useNavigate } from "react-router";
import { EditorType, SketchpadScope, useEditorCommands, useEditorType, useEditorPanelVisibility, useKits, useNavigation, useSketchpad, useSketchpadCommands, useSketchpadScope } from "../../../store";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "../Breadcrumb";
import { ButtonGroup, ButtonGroupItem } from "../ButtonGroup";
import { Command, CommandInput, CommandItem, CommandList, CommandShortcut } from "../Command";
import { Toggle } from "../Toggle";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";

export interface PanelSection {
  id: string;
  label: string;
  content: ReactNode;
  defaultOpen?: boolean;
  order?: number;
  actions?: Array<{
    icon: ReactNode;
    onClick: () => void;
    title: string;
  }>;
}

export type PanelKey = "details" | "workbench" | "console" | "chat" | "settings";

export interface PanelSections {
  details: PanelSection[];
  workbench: PanelSection[];
  console: PanelSection[];
  chat: PanelSection[];
  settings: PanelSection[];
}

interface PanelSectionContextValue {
  sections: PanelSections;
  addSection: (panelKey: PanelKey, section: PanelSection) => void;
  removeSection: (panelKey: PanelKey, sectionId: string) => void;
}

const PanelSectionContext = createContext<PanelSectionContextValue | null>(null);

export const PanelSectionProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [sections, setSections] = useState<PanelSections>({
    details: [],
    workbench: [],
    console: [],
    chat: [],
    settings: [],
  });

  const addSection = useCallback((panelKey: PanelKey, section: PanelSection) => {
    setSections((prev) => ({
      ...prev,
      [panelKey]: [...prev[panelKey].filter((s) => s.id !== section.id), section].sort((a, b) => (a.order || 0) - (b.order || 0)),
    }));
  }, []);

  const removeSection = useCallback((panelKey: PanelKey, sectionId: string) => {
    setSections((prev) => ({
      ...prev,
      [panelKey]: prev[panelKey].filter((s) => s.id !== sectionId),
    }));
  }, []);

  return <PanelSectionContext.Provider value={{ sections, addSection, removeSection }}>{children}</PanelSectionContext.Provider>;
};

export const usePanelSections = (panelKey: PanelKey): PanelSection[] => {
  const context = useContext(PanelSectionContext);
  if (!context) throw new Error("usePanelSections must be used within PanelSectionProvider");
  return context.sections[panelKey];
};

export const useAddPanelSection = () => {
  const context = useContext(PanelSectionContext);
  if (!context) throw new Error("useAddPanelSection must be used within PanelSectionProvider");
  return context.addSection;
};

export const useRemovePanelSection = () => {
  const context = useContext(PanelSectionContext);
  if (!context) throw new Error("useRemovePanelSection must be used within PanelSectionProvider");
  return context.removeSection;
};

export interface PanelDefinition {
  key: string;
  icon: React.ComponentType<{ size?: number }>;
  tooltip: string;
  hotkey: string;
}

export const PANEL_CONFIGS: Record<EditorType, PanelDefinition[]> = {
  [EditorType.HOME]: [
    { key: "chat", icon: MessageCircle, tooltip: "Click to toggle chat panel", hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: "Click to toggle settings panel", hotkey: "⌘," },
  ],
  [EditorType.KIT]: [
    { key: "chat", icon: MessageCircle, tooltip: "Click to toggle chat panel", hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: "Click to toggle settings panel", hotkey: "⌘," },
  ],
  [EditorType.DESIGN]: [
    { key: "workbench", icon: Wrench, tooltip: "Click to toggle workbench panel", hotkey: "⌘J" },
    { key: "details", icon: Info, tooltip: "Click to toggle details panel", hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: "Click to toggle chat panel", hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: "Click to toggle settings panel", hotkey: "⌘," },
  ],
  [EditorType.TYPE]: [
    { key: "workbench", icon: Wrench, tooltip: "Click to toggle workbench panel", hotkey: "⌘J" },
    { key: "details", icon: Info, tooltip: "Click to toggle details panel", hotkey: "⌘L" },
    { key: "console", icon: Terminal, tooltip: "Click to toggle console panel", hotkey: "⌘K" },
    { key: "settings", icon: Settings, tooltip: "Click to toggle settings panel", hotkey: "⌘," },
  ],
};

const Navigation: FC = ({}) => {
  let navigate = useNavigate();
  const navigation = useNavigation();
  const kits = useKits();
  return (
    <Breadcrumb>
      <BreadcrumbList>
        <BreadcrumbItem tooltip="Click to go home">
          <BreadcrumbLink href="/">
            <Home size={16} />
          </BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator
          items={[{ label: "Starter", href: "/metabolism/starter" }]}
          tooltip="Click to see kits"
          onNavigate={(href) => {
            navigate(href);
          }}
        />
        <BreadcrumbItem tooltip="Click to go to kit">
          <BreadcrumbLink href="/metabolism">Metabolism</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator
          items={[
            { label: "Types", href: "/designs/types" },
            { label: "Representations", href: "/designs/representations" },
          ]}
          tooltip="Click to see all artifacts kinds of the kit"
          onNavigate={(href) => {
            navigate(href);
          }}
        />
        <BreadcrumbItem tooltip="Click to go to see all designs of the kit">
          <BreadcrumbLink href="/designs">Designs</BreadcrumbLink>
        </BreadcrumbItem>
      </BreadcrumbList>
    </Breadcrumb>
  );
};

const Search: FC = ({}) => {
  return (
    <Command>
      <CommandInput />
      <CommandList>
        <CommandItem>
          <CommandShortcut>Ctrl+K</CommandShortcut>
        </CommandItem>
      </CommandList>
    </Command>
  );
};

const PanelToggles: FC = ({}) => {
  const editorType = useEditorType();
  const panelConfig = PANEL_CONFIGS[editorType];
  const visiblePanels = useEditorPanelVisibility();
  const { togglePanel } = useEditorCommands();

  if (panelConfig.length === 0) return null;

  const handleToggle = (panelKey: string) => {
    // Handle mutual exclusivity for chat, details, and settings
    const current = visiblePanels[panelKey];
    if (!current && (panelKey === "chat" || panelKey === "details" || panelKey === "settings")) {
      // Close the other exclusive panels
      const exclusivePanels = ["chat", "details", "settings"];
      exclusivePanels.forEach((p) => {
        if (p !== panelKey && visiblePanels[p]) {
          togglePanel(p as any);
        }
      });
    }
    togglePanel(panelKey as any);
  };

  return (
    <ToggleGroup
      type="multiple"
      value={Object.entries(visiblePanels)
        .filter(([_, isVisible]) => isVisible)
        .map(([key]) => key)}
      onValueChange={(values) => {
        // Iterate over all panel config keys, not just existing visiblePanels
        panelConfig.forEach(({ key }) => {
          const isCurrentlyVisible = visiblePanels[key] || false;
          const shouldBeVisible = values.includes(key);
          if (isCurrentlyVisible !== shouldBeVisible) {
            handleToggle(key);
          }
        });
      }}
    >
      {panelConfig.map(({ key, icon: Icon, tooltip, hotkey }) => (
        <ToggleGroupItem key={key} value={key} tooltip={tooltip} hotkey={hotkey}>
          <Icon />
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  );
};

interface NavbarProps {}

const Navbar: FC<NavbarProps> = ({}) => {
  const { onWindowEvents } = useSketchpadScope() as SketchpadScope;
  let navigate = useNavigate();

  return (
    <div id="navbar" className={`w-full h-12 bg-background border-b flex items-center justify-between px-4 [-webkit-app-region: drag]`} style={{ WebkitAppRegion: "drag" }}>
      <ButtonGroup>
        <ButtonGroupItem value="back" tooltip="Click to go back, hold to see history" onClick={() => navigate(-1)}>
          <ArrowLeft size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem value="forward" tooltip="Click to go forward, hold to see history" onClick={() => navigate(1)}>
          <ArrowRight size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem value="up" tooltip="Click to go up" onClick={() => navigate("/")}>
          <ArrowUp size={16} />
        </ButtonGroupItem>
      </ButtonGroup>

      <Navigation />
      {/* <Search /> */}

      <PanelToggles />
      <Toggle variant="outline" tooltip="Click to toggle fullscreen">
        <Fullscreen />
      </Toggle>
      {onWindowEvents && (
        <div className="flex items-center gap-2 ml-4">
          <ToggleGroup type="single">
            <ToggleGroupItem value="minimize" tooltip="Click to minimize" onClick={onWindowEvents.minimize}>
              <Minus size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="maximize" tooltip="Click to maximize" onClick={onWindowEvents.maximize}>
              <Square size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="close" tooltip="Click to close" onClick={onWindowEvents.close} className="hover:bg-danger">
              <X size={16} />
            </ToggleGroupItem>
          </ToggleGroup>
        </div>
      )}
    </div>
  );
};
export default Navbar;
