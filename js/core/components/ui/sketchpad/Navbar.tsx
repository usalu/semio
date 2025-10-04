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

import { ArrowLeft, ArrowRight, ArrowUp, Fullscreen, Home, Minus, Square, X } from "lucide-react";
import { FC } from "react";
import { useNavigate } from "react-router";
import { SketchpadScope, useEditorType, useKits, useNavigation, useSketchpad, useSketchpadCommands, useSketchpadScope } from "../../../store";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "../Breadcrumb";
import { ButtonGroup, ButtonGroupItem } from "../ButtonGroup";
import { Command, CommandInput, CommandItem, CommandList, CommandShortcut } from "../Command";
import { Toggle } from "../Toggle";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";
import { PANEL_CONFIGS } from "./panelConfigs";

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
  const visiblePanels = useSketchpad((s) => s.panelVisibility[editorType]) || {};
  const { togglePanel } = useSketchpadCommands();

  if (panelConfig.length === 0) return null;

  const handleToggle = (panelKey: string) => {
    // Handle mutual exclusivity for chat, details, and settings
    const current = visiblePanels[panelKey];
    if (!current && (panelKey === "chat" || panelKey === "details" || panelKey === "settings")) {
      // Close the other exclusive panels
      const exclusivePanels = ["chat", "details", "settings"];
      exclusivePanels.forEach((p) => {
        if (p !== panelKey && visiblePanels[p]) {
          togglePanel(editorType, p);
        }
      });
    }
    togglePanel(editorType, panelKey);
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
