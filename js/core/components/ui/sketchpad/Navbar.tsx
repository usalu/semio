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

import { ArrowLeft, ArrowRight, ArrowUp, Fullscreen, Home, Info, MessageCircle, Minus, Square, Terminal, Wrench, X } from "lucide-react";
import { FC, useState } from "react";
import { useNavigate } from "react-router";
import { SketchpadScope, useSketchpadScope } from "../../../store";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "../Breadcrumb";
import { ButtonGroup, ButtonGroupItem } from "../ButtonGroup";
import { Command, CommandInput, CommandItem, CommandList, CommandShortcut } from "../Command";
import { Toggle } from "../Toggle";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";

const Navigation: FC = ({}) => {
  return (
    <Breadcrumb>
      <BreadcrumbList>
        <BreadcrumbItem>
          <BreadcrumbLink href="/">
            <Home size={16} />
          </BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator
          items={[
            { label: "Starter", href: "/metabolism/starter" },
            { label: "Geometry", href: "/metabolism/geometry" },
          ]}
          onNavigate={(href) => {}}
        />
        <BreadcrumbItem>
          <BreadcrumbLink href="/metabolism">Metabolism</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator
          items={[
            { label: "Types", href: "/designs/types" },
            { label: "Representations", href: "/designs/representations" },
          ]}
          onNavigate={(href) => {}}
        />
        <BreadcrumbItem>
          <BreadcrumbLink href="/designs">Designs</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator
          items={[
            {
              label: "Capsule Dream",
              href: "/designs/nakagin/capsule-dream",
            },
          ]}
          onNavigate={(href) => {}}
        />
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
  const [visiblePanels, setVisiblePanels] = useState<VisiblePanels>({
    workbench: false,
    details: false,
    console: false,
    chat: false,
  });
  const togglePanel = (panel: keyof VisiblePanels) => {
    setVisiblePanels((prev) => {
      const newState = { ...prev };
      if (panel === "chat" && !prev.chat) {
        newState.details = false;
      }
      if (panel === "details" && !prev.details) {
        newState.chat = false;
      }
      newState[panel] = !prev[panel];
      return newState;
    });
  };
  return (
    <ToggleGroup
      type="multiple"
      value={Object.entries(visiblePanels)
        .filter(([_, isVisible]) => isVisible)
        .map(([key]) => key)}
      onValueChange={(values) => {
        Object.keys(visiblePanels).forEach((key) => {
          const isCurrentlyVisible = visiblePanels[key as keyof VisiblePanels];
          const shouldBeVisible = values.includes(key);
          if (isCurrentlyVisible !== shouldBeVisible) {
            togglePanel(key as keyof VisiblePanels);
          }
        });
      }}
    >
      <ToggleGroupItem value="workbench" tooltip="Workbench" hotkey="⌘J">
        <Wrench />
      </ToggleGroupItem>
      <ToggleGroupItem value="console" tooltip="Console" hotkey="⌘K">
        <Terminal />
      </ToggleGroupItem>
      <ToggleGroupItem value="details" tooltip="Details" hotkey="⌘L">
        <Info />
      </ToggleGroupItem>
      <ToggleGroupItem value="chat" tooltip="Chat" hotkey="⌘[">
        <MessageCircle />
      </ToggleGroupItem>
    </ToggleGroup>
  );
};

interface VisiblePanels {
  workbench: boolean;
  details: boolean;
  console: boolean;
  chat: boolean;
}

interface NavbarProps {}

const Navbar: FC<NavbarProps> = ({}) => {
  const { onWindowEvents } = useSketchpadScope() as SketchpadScope;
  let navigate = useNavigate();

  return (
    <div id="navbar" className={`w-full h-12 bg-background border-b flex items-center justify-between px-4 [-webkit-app-region: drag]`} style={{ WebkitAppRegion: "drag" }}>
      <ButtonGroup>
        <ButtonGroupItem value="back" onClick={() => navigate(-1)}>
          <ArrowLeft size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem value="forward" onClick={() => navigate(1)}>
          <ArrowRight size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem value="up" onClick={() => navigate("/")}>
          <ArrowUp size={16} />
        </ButtonGroupItem>
      </ButtonGroup>

      <Navigation />
      {/* <Search /> */}

      <PanelToggles />
      <Toggle variant="outline">
        <Fullscreen />
      </Toggle>
      {onWindowEvents && (
        <div className="flex items-center gap-2 ml-4">
          <ToggleGroup type="single">
            <ToggleGroupItem value="minimize" onClick={onWindowEvents.minimize}>
              <Minus size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="maximize" onClick={onWindowEvents.maximize}>
              <Square size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="close" onClick={onWindowEvents.close} className="hover:bg-danger">
              <X size={16} />
            </ToggleGroupItem>
          </ToggleGroup>
        </div>
      )}
    </div>
  );
};
export default Navbar;
