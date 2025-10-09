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

import { ArrowLeft, ArrowRight, ArrowUp, ChevronDown, ChevronUp, Fullscreen, Home, Info, MessageCircle, Minimize, Minus, Settings, Square, Wrench, X } from "lucide-react";
import { createContext, FC, ReactNode, useCallback, useContext, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import {
  EditorType,
  PanelVisibility,
  SketchpadScope,
  useEditorCommands,
  useEditorPanelVisibility,
  useEditorType,
  useIsFullscreen,
  useIsMobile,
  useIsNavbarExpanded,
  useKits,
  useNavigation,
  useNavigationHistory,
  useSketchpadCommands,
  useSketchpadScope,
} from "../../../store";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "../Breadcrumb";
import { ButtonGroup, ButtonGroupItem } from "../ButtonGroup";
import { Command, CommandInput, CommandItem, CommandList, CommandShortcut } from "../Command";
import { Toggle } from "../Toggle";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";

export interface PanelSection {
  id: string;
  label: string;
  content: ReactNode | (() => ReactNode);
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

export const getPanelConfigs = (t: (key: string) => string): Record<EditorType, PanelDefinition[]> => ({
  [EditorType.HOME]: [
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  [EditorType.KIT]: [
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  [EditorType.DESIGN]: [
    { key: "workbench", icon: Wrench, tooltip: t("panels.workbench"), hotkey: "⌘J" },
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  [EditorType.TYPE]: [
    { key: "workbench", icon: Wrench, tooltip: t("panels.workbench"), hotkey: "⌘J" },
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
});

const Navigation: FC = ({}) => {
  const { t } = useTranslation();
  let navigate = useNavigate();
  const navigation = useNavigation();
  const kits = useKits();
  return (
    <Breadcrumb>
      <BreadcrumbList>
        <BreadcrumbItem tooltip={t("navbar.home")}>
          <BreadcrumbLink href="/">
            <Home size={16} />
          </BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator
          items={[{ label: t("breadcrumb.starter"), href: "/metabolism/starter" }]}
          tooltip={t("navbar.kits")}
          onNavigate={(href) => {
            navigate(href);
          }}
        />
        <BreadcrumbItem tooltip={t("navbar.kit")}>
          <BreadcrumbLink href="/metabolism">{t("breadcrumb.metabolism")}</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator
          items={[
            { label: t("breadcrumb.types"), href: "/designs/types" },
            { label: t("breadcrumb.representations"), href: "/designs/representations" },
          ]}
          tooltip={t("navbar.artifacts")}
          onNavigate={(href) => {
            navigate(href);
          }}
        />
        <BreadcrumbItem tooltip={t("navbar.designs")}>
          <BreadcrumbLink href="/designs">{t("breadcrumb.designs")}</BreadcrumbLink>
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
  const { t } = useTranslation();
  const editorType = useEditorType();
  const panelConfig = getPanelConfigs(t)[editorType];
  const visiblePanels = useEditorPanelVisibility();
  const { togglePanel } = useEditorCommands();
  const isMobile = useIsMobile();

  if (panelConfig.length === 0) return null;

  // Exclusive panels: only one can be open at a time (details, chat, settings)
  const exclusivePanels = ["details", "chat", "settings"];
  const exclusiveConfigs = panelConfig.filter((p) => exclusivePanels.includes(p.key));
  const regularConfigs = panelConfig.filter((p) => !exclusivePanels.includes(p.key));

  // Find currently active exclusive panel
  const activeExclusivePanel = exclusiveConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key || exclusiveConfigs[0]?.key || "";
  const isAnyExclusivePanelOpen = exclusiveConfigs.some((p) => visiblePanels[p.key as keyof PanelVisibility]);

  const handleToggle = (panelKey: keyof PanelVisibility) => {
    const current = visiblePanels[panelKey];

    if (isMobile) {
      // On mobile, only one panel can be open at a time
      if (!current) {
        // Close all other panels
        (Object.keys(visiblePanels) as Array<keyof PanelVisibility>).forEach((p) => {
          if (p !== panelKey && visiblePanels[p]) {
            togglePanel(p);
          }
        });
      }
    } else {
      // Desktop behavior: Handle mutual exclusivity for details, chat, and settings
      if (!current && exclusivePanels.includes(panelKey)) {
        (exclusivePanels as Array<keyof PanelVisibility>).forEach((p) => {
          if (p !== panelKey && visiblePanels[p]) {
            togglePanel(p);
          }
        });
      }
    }
    togglePanel(panelKey);
  };

  const handleExclusivePressedChange = (pressed: boolean) => {
    if (pressed) {
      // Open the currently selected panel if not already open
      if (activeExclusivePanel && !visiblePanels[activeExclusivePanel as keyof PanelVisibility]) {
        handleToggle(activeExclusivePanel as keyof PanelVisibility);
      }
    } else {
      // Close the currently open exclusive panel
      const openPanel = exclusiveConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility]);
      if (openPanel) {
        togglePanel(openPanel.key as keyof PanelVisibility);
      }
    }
  };

  const handleExclusiveValueChange = (value: string | undefined) => {
    if (!value) return;

    // Close all exclusive panels and open the selected one
    (exclusivePanels as Array<keyof PanelVisibility>).forEach((p) => {
      const isOpen = visiblePanels[p];
      const shouldOpen = p === value;

      if (isOpen && !shouldOpen) {
        togglePanel(p);
      } else if (!isOpen && shouldOpen) {
        togglePanel(p);
      }
    });
  };

  return (
    <ToggleGroup type="multiple" value={[...regularConfigs.filter((p) => visiblePanels[p.key as keyof PanelVisibility]).map((p) => p.key), ...(isAnyExclusivePanelOpen ? [activeExclusivePanel] : [])]}>
      {/* Regular toggles (workbench) */}
      {regularConfigs.map(({ key, icon: Icon, tooltip, hotkey }) => (
        <ToggleGroupItem
          key={key}
          value={key}
          tooltip={tooltip}
          hotkey={hotkey}
          onClick={() => {
            handleToggle(key as keyof PanelVisibility);
          }}
        >
          <Icon />
        </ToggleGroupItem>
      ))}

      {/* Dropdown toggle for exclusive panels (details, chat, settings) */}
      {exclusiveConfigs.length > 0 && (
        <Toggle
          type="dropdown"
          pressed={isAnyExclusivePanelOpen}
          onPressedChange={handleExclusivePressedChange}
          value={activeExclusivePanel}
          onValueChange={handleExclusiveValueChange}
          tooltip={exclusiveConfigs.find((p) => p.key === activeExclusivePanel)?.tooltip}
          dropdownTooltip={t("navbar.changePanelType")}
          className={regularConfigs.length > 0 ? "border-0 border-l" : "border-0"}
          items={exclusiveConfigs.map(({ key, icon: Icon, tooltip, hotkey }) => ({
            value: key,
            label: <Icon />,
            tooltip,
            hotkey,
          }))}
        />
      )}
    </ToggleGroup>
  );
};

interface NavbarProps {}

const Navbar: FC<NavbarProps> = ({}) => {
  const { t } = useTranslation();
  const { onWindowEvents } = useSketchpadScope() as SketchpadScope;
  const isFullscreen = useIsFullscreen();
  const isNavbarExpanded = useIsNavbarExpanded();
  const isMobile = useIsMobile();
  const { toggleFullscreen, toggleNavbarExpanded, navigateBack, navigateForward, setIsMobile } = useSketchpadCommands();
  const [isVisible, setIsVisible] = useState(true);
  const navigate = useNavigate();
  const currentPath = useNavigation();
  const { canGoBack, canGoForward } = useNavigationHistory();

  const isAtRoot = currentPath === "/";

  useEffect(() => {
    const checkMobile = () => {
      setIsMobile(window.innerWidth < 768);
    };
    checkMobile();
    window.addEventListener("resize", checkMobile);
    return () => window.removeEventListener("resize", checkMobile);
  }, [setIsMobile]);

  useEffect(() => {
    if (!isFullscreen) {
      setIsVisible(true);
      return;
    }

    const handleMouseMove = (e: MouseEvent) => {
      setIsVisible(e.clientY < 50);
    };

    window.addEventListener("mousemove", handleMouseMove);
    return () => window.removeEventListener("mousemove", handleMouseMove);
  }, [isFullscreen]);

  if (isMobile) {
    return (
      <div
        id="navbar"
        className={`w-full bg-background border-b flex flex-col [-webkit-app-region: drag] transition-transform duration-200 ${isFullscreen && !isVisible ? "-translate-y-full" : "translate-y-0"} ${isNavbarExpanded ? "h-auto" : "h-12"}`}
        style={{ WebkitAppRegion: "drag" }}
      >
        <div className="h-12 flex items-center justify-between px-4 gap-2">
          <ButtonGroupItem value="back" tooltip={t("navbar.back")} onClick={navigateBack} disabled={!canGoBack}>
            <ArrowLeft size={16} />
          </ButtonGroupItem>

          <PanelToggles />

          <Toggle tooltip={isNavbarExpanded ? t("navbar.collapse") : t("navbar.expand")} pressed={isNavbarExpanded} onPressedChange={toggleNavbarExpanded}>
            {isNavbarExpanded ? <ChevronUp /> : <ChevronDown />}
          </Toggle>
        </div>

        {isNavbarExpanded && (
          <div className="flex flex-col gap-2 px-4 pb-4">
            <ButtonGroup>
              <ButtonGroupItem value="forward" tooltip={t("navbar.forward")} onClick={navigateForward} disabled={!canGoForward}>
                <ArrowRight size={16} />
              </ButtonGroupItem>
              <ButtonGroupItem value="up" tooltip={t("navbar.up")} onClick={() => navigate("/")} disabled={isAtRoot}>
                <ArrowUp size={16} />
              </ButtonGroupItem>
            </ButtonGroup>

            <Navigation />

            <div className="flex gap-2">
              <Toggle tooltip={isFullscreen ? t("navbar.exitFullscreen") : t("navbar.fullscreen")} pressed={isFullscreen} onPressedChange={toggleFullscreen}>
                {isFullscreen ? <Minimize /> : <Fullscreen />}
              </Toggle>

              {onWindowEvents && (
                <ToggleGroup type="single">
                  <ToggleGroupItem value="minimize" tooltip={t("navbar.minimize")} onClick={onWindowEvents.minimize}>
                    <Minus size={16} />
                  </ToggleGroupItem>
                  <ToggleGroupItem value="maximize" tooltip={t("navbar.maximize")} onClick={onWindowEvents.maximize}>
                    <Square size={16} />
                  </ToggleGroupItem>
                  <ToggleGroupItem value="close" tooltip={t("navbar.close")} onClick={onWindowEvents.close} className="hover:bg-danger">
                    <X size={16} />
                  </ToggleGroupItem>
                </ToggleGroup>
              )}
            </div>
          </div>
        )}
      </div>
    );
  }

  return (
    <div
      id="navbar"
      className={`w-full h-12 bg-background border-b flex items-center justify-between px-4 [-webkit-app-region: drag] transition-transform duration-200 ${isFullscreen && !isVisible ? "-translate-y-full" : "translate-y-0"}`}
      style={{ WebkitAppRegion: "drag" }}
    >
      <ButtonGroup>
        <ButtonGroupItem value="back" tooltip={t("navbar.back")} onClick={navigateBack} disabled={!canGoBack}>
          <ArrowLeft size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem value="forward" tooltip={t("navbar.forward")} onClick={navigateForward} disabled={!canGoForward}>
          <ArrowRight size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem value="up" tooltip={t("navbar.up")} onClick={() => navigate("/")} disabled={isAtRoot}>
          <ArrowUp size={16} />
        </ButtonGroupItem>
      </ButtonGroup>

      <Navigation />
      {/* <Search /> */}

      <PanelToggles />
      <Toggle variant="outline" tooltip={isFullscreen ? t("navbar.exitFullscreen") : t("navbar.fullscreen")} pressed={isFullscreen} onPressedChange={toggleFullscreen}>
        {isFullscreen ? <Minimize /> : <Fullscreen />}
      </Toggle>
      {onWindowEvents && (
        <div className="flex items-center gap-2 ml-4">
          <ToggleGroup type="single">
            <ToggleGroupItem value="minimize" tooltip={t("navbar.minimize")} onClick={onWindowEvents.minimize}>
              <Minus size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="maximize" tooltip={t("navbar.maximize")} onClick={onWindowEvents.maximize}>
              <Square size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="close" tooltip={t("navbar.close")} onClick={onWindowEvents.close} className="hover:bg-danger">
              <X size={16} />
            </ToggleGroupItem>
          </ToggleGroup>
        </div>
      )}
    </div>
  );
};
export default Navbar;
