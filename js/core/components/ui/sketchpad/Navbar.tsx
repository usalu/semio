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
import { useNavigate, useParams } from "react-router";
import { Design, Type } from "../../../semio";
import {
  EditorType,
  PanelVisibility,
  SketchpadScope,
  useDesignEditorCommands,
  useEditorPanelVisibility,
  useEditorType,
  useHomeCommands,
  useIsFullscreen,
  useIsMobile,
  useIsNavbarExpanded,
  useKitEditorCommands,
  useKits,
  useNavigation,
  useNavigationHistory,
  useSketchpadCommands,
  useSketchpadScope,
  useTypeEditorCommands,
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

const Navigation: FC = ({ }) => {
  const { t } = useTranslation();
  let navigate = useNavigate();
  const navigation = useNavigation();
  const kits = useKits();
  const pathMatch = navigation.match(/^\/([^/]+)(?:\/([dt])\/([^/]+))?/);
  const kitGuid = pathMatch?.[1];
  const editorTypeChar = pathMatch?.[2];
  const itemGuid = pathMatch?.[3];
  const kit = kits.find((k) => k.guid === kitGuid);
  const kitEditorCommands = useKitEditorCommands(kitGuid ? { kit: kitGuid } : undefined);
  const kitItems = kits.map((k) => ({ label: k.name, href: `/${k.guid}` }));
  const artifactKinds = [
    { label: t("breadcrumb.designs"), kind: "designs", href: kitGuid ? `/${kitGuid}?kind=designs` : "/?kind=designs" },
    { label: t("breadcrumb.types"), kind: "types", href: kitGuid ? `/${kitGuid}?kind=types` : "/?kind=types" },
    { label: t("breadcrumb.qualities"), kind: "qualities", href: kitGuid ? `/${kitGuid}?kind=qualities` : "/?kind=qualities" },
    { label: t("breadcrumb.files"), kind: "files", href: kitGuid ? `/${kitGuid}?kind=files` : "/?kind=files" },
    { label: t("breadcrumb.authors"), kind: "authors", href: kitGuid ? `/${kitGuid}?kind=authors` : "/?kind=authors" },
  ];

  // Find design or type for displaying hierarchy
  let design: Design | undefined;
  let type: Type | undefined;
  if (kit && itemGuid) {
    if (editorTypeChar === "d") {
      design = kit.designs?.find((d) => d.guid === itemGuid);
    } else if (editorTypeChar === "t") {
      type = kit.types?.find((t) => t.guid === itemGuid);
    }
  }

  // Build hierarchical display for design: NAME - VARIANT - VIEW
  const designLabel = design
    ? [design.name, design.variant, design.view].filter(Boolean).join(" - ")
    : itemGuid;

  // Build hierarchical display for type: NAME - VARIANT
  const typeLabel = type
    ? [type.name, type.variant].filter(Boolean).join(" - ")
    : itemGuid;

  // Create dropdown items for designs when in design editor
  const designItems = kit?.designs?.map((d) => ({
    label: [d.name, d.variant, d.view].filter(Boolean).join(" - "),
    href: `/${kitGuid}/d/${d.guid}`,
  })) || [];

  // Create dropdown items for types when in type editor
  const typeItems = kit?.types?.map((t) => ({
    label: [t.name, t.variant].filter(Boolean).join(" - "),
    href: `/${kitGuid}/t/${t.guid}`,
  })) || [];

  return (
    <Breadcrumb>
      <BreadcrumbList>
        <BreadcrumbItem tooltip={t("navbar.home")}>
          <BreadcrumbLink onClick={() => navigate("/")} style={{ cursor: "pointer" }}>
            <Home size={16} />
          </BreadcrumbLink>
        </BreadcrumbItem>
        {kitGuid && (
          <>
            <BreadcrumbSeparator items={kitItems} tooltip={t("navbar.kits")} onNavigate={(href) => navigate(href)} />
            <BreadcrumbItem tooltip={t("navbar.kit")}>
              <BreadcrumbLink onClick={() => navigate(`/${kitGuid}`)} style={{ cursor: "pointer" }}>
                {kit?.name || kitGuid}
              </BreadcrumbLink>
            </BreadcrumbItem>
          </>
        )}
        {kitGuid && !itemGuid && (
          <>
            <BreadcrumbSeparator
              items={artifactKinds}
              tooltip={t("navbar.artifacts")}
              onNavigate={(href) => {
                const kind = artifactKinds.find((a) => a.href === href)?.kind;
                const pathWithoutQuery = href.split('?')[0];
                navigate(pathWithoutQuery);
                if (kind && kitEditorCommands) {
                  kitEditorCommands.setFilterKinds([kind]);
                }
              }}
            />
          </>
        )}
        {editorTypeChar === "d" && itemGuid && (
          <>
            <BreadcrumbSeparator
              items={artifactKinds}
              tooltip={t("navbar.artifacts")}
              onNavigate={(href) => {
                const kind = artifactKinds.find((a) => a.href === href)?.kind;
                const pathWithoutQuery = href.split('?')[0];
                navigate(pathWithoutQuery);
                if (kind && kitEditorCommands) {
                  setTimeout(() => kitEditorCommands.setFilterKinds([kind]), 0);
                }
              }}
            />
            <BreadcrumbItem tooltip={t("breadcrumb.designs")}>
              <BreadcrumbLink onClick={() => {
                navigate(`/${kitGuid}`);
                if (kitEditorCommands) {
                  kitEditorCommands.setFilterKinds(["designs"]);
                }
              }} style={{ cursor: "pointer" }}>
                {t("breadcrumb.designs")}
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={designItems}
              tooltip={t("navbar.selectDesign")}
              onNavigate={(href) => navigate(href)}
            />
            <BreadcrumbItem tooltip={t("navbar.design")}>
              <BreadcrumbLink onClick={() => navigate(`/${kitGuid}/d/${itemGuid}`)} style={{ cursor: "pointer" }}>
                {designLabel}
              </BreadcrumbLink>
            </BreadcrumbItem>
          </>
        )}
        {editorTypeChar === "t" && itemGuid && (
          <>
            <BreadcrumbSeparator
              items={artifactKinds}
              tooltip={t("navbar.artifacts")}
              onNavigate={(href) => {
                const kind = artifactKinds.find((a) => a.href === href)?.kind;
                const pathWithoutQuery = href.split('?')[0];
                navigate(pathWithoutQuery);
                if (kind && kitEditorCommands) {
                  setTimeout(() => kitEditorCommands.setFilterKinds([kind]), 0);
                }
              }}
            />
            <BreadcrumbItem tooltip={t("breadcrumb.types")}>
              <BreadcrumbLink onClick={() => {
                navigate(`/${kitGuid}`);
                if (kitEditorCommands) {
                  kitEditorCommands.setFilterKinds(["types"]);
                }
              }} style={{ cursor: "pointer" }}>
                {t("breadcrumb.types")}
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={typeItems}
              tooltip={t("navbar.selectType")}
              onNavigate={(href) => navigate(href)}
            />
            <BreadcrumbItem tooltip={t("navbar.type")}>
              <BreadcrumbLink onClick={() => navigate(`/${kitGuid}/t/${itemGuid}`)} style={{ cursor: "pointer" }}>
                {typeLabel}
              </BreadcrumbLink>
            </BreadcrumbItem>
          </>
        )}
      </BreadcrumbList>
    </Breadcrumb>
  );
};

const Search: FC = ({ }) => {
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

const PanelToggles: FC = ({ }) => {
  const { t } = useTranslation();
  const { kit, design, type } = useParams();
  const editorType = useEditorType();
  const panelConfig = getPanelConfigs(t)[editorType];
  const visiblePanels = useEditorPanelVisibility();
  const homeCommands = useHomeCommands();
  const kitEditorCommands = useKitEditorCommands(kit ? { kit } : undefined);
  const designEditorCommands = useDesignEditorCommands(kit && design ? { kit, design } : undefined);
  const typeEditorCommands = useTypeEditorCommands(type ? { type } : undefined);
  console.log("[PanelToggles] editorType:", editorType);
  console.log("[PanelToggles] homeCommands:", homeCommands);
  console.log("[PanelToggles] kitEditorCommands:", kitEditorCommands);
  console.log("[PanelToggles] designEditorCommands:", designEditorCommands);
  console.log("[PanelToggles] typeEditorCommands:", typeEditorCommands);
  const commands = {
    [EditorType.HOME]: homeCommands,
    [EditorType.KIT]: kitEditorCommands,
    [EditorType.DESIGN]: designEditorCommands,
    [EditorType.TYPE]: typeEditorCommands,
  };
  console.log("[PanelToggles] commands:", commands);
  console.log("[PanelToggles] selected command:", commands[editorType]);
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
    console.log("[Navbar handleToggle] called with panelKey:", panelKey, "current state:", visiblePanels[panelKey]);
    console.log("[Navbar handleToggle] editorType:", editorType);
    const togglePanel = commands[editorType]?.togglePanel || (() => { console.log("[PanelToggles] fallback no-op called"); });
    console.log("[Navbar handleToggle] togglePanel function:", togglePanel);
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
    console.log("[Navbar handleToggle] calling togglePanel:", panelKey);
    togglePanel(panelKey);
  };

  const handleExclusivePressedChange = (pressed: boolean) => {
    const togglePanel = commands[editorType]?.togglePanel || (() => { console.log("[PanelToggles] fallback no-op called"); });
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
    const togglePanel = commands[editorType]?.togglePanel || (() => { console.log("[PanelToggles] fallback no-op called"); });
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

interface NavbarProps { }

const Navbar: FC<NavbarProps> = ({ }) => {
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
