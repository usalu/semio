// #region Header

// App.tsx

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

// #region Imports

import {
  AddIcon,
  AwardIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  CodeIcon,
  DocumentIcon,
  HandIcon,
  LocalKitIcon,
  MonitorIcon,
  MoonIcon,
  MousePointerIcon,
  RemoteKitIcon,
  SortAscendingIcon,
  SortDescendingIcon,
  SunIcon,
  TemporaryKitIcon,
  TutorialIcon,
  UserIcon,
} from "@semio/assets";
import { formatDistanceToNow } from "date-fns";
import { de, enUS } from "date-fns/locale";
import { FC, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useSearchParams } from "react-router";
import i18n, { useLabel } from "../i18n";
import { generateUniqueName, guid, Guid, importKit, Kit, KitShallow } from "../semio";
import { docsRegistry } from "./Docs";
import { Action, Band, Input, Scrollable, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Spinner, Table, TableAvatar, TableColumn, Textarea, Toggle, ToggleGroup, TreeContent, TreeItem } from "./elements";
import type { AppConfig, AppEdit, AppPlugin, PanelDefinition, PanelVisibility } from "./shared";
import { createPanelDefinition, Expertise, Mode, PanelKind, registerAppPlugin, registerEventHandler, registerRuntimeAction, Theme } from "./shared";
import {
  Canvas,
  ConceptFilter,
  useAddFooterItem,
  useAddPanelSection,
  useAppType,
  useExpertise,
  useFocus,
  useGetKitKind,
  useHomeApp,
  useHomeCommands,
  useHotkeys,
  useIsMobile,
  useKits,
  useKitShallows,
  useLanguage,
  useDevice,
  useMode,
  useNavigation,
  useRemoveFooterItem,
  useRemovePanelSection,
  useSketchpadActor,
  useSketchpadCommands,
  useTheme,
  useTooltip,
  Window,
} from "./Sketchpad";

// Alias for internal use
const useHome = useHomeApp;

// #endregion Imports

// #region Types

export interface HomeSelection {
  kits?: Guid[];
}

export interface HomeSelectionDiff {
  added?: Guid[];
  removed?: Guid[];
}

export type HomeSortColumn = "name" | "type" | "updatedAt" | "createdAt";
export type HomeSortDirection = "asc" | "desc";

export interface LoadingKit {
  tempGuid: Guid;
  name: string;
}

export interface HomeState {
  panelVisibility: PanelVisibility;
  selection?: HomeSelection;
  sortColumn?: HomeSortColumn;
  sortDirection?: HomeSortDirection;
  loadingKits?: LoadingKit[];
}

export interface HomeDiff {
  panelVisibility?: Partial<PanelVisibility>;
  selection?: HomeSelectionDiff;
  sortColumn?: HomeSortColumn;
  sortDirection?: HomeSortDirection;
}

export interface HomeEdit extends AppEdit<HomeSelectionDiff> {}

export interface HomeCommandContext {
  home: HomeState;
  origin?: string;
}

export interface HomeCommandResult {
  diff?: HomeDiff;
}

// #endregion Types

// #region Home App Plugin Registration

/**
 * Home app plugin for the sketchpad machine.
 * Provides HOME.* events, actions, and guards.
 */
const homeAppPlugin: AppPlugin = {
  id: "home",
  namespace: "HOME",
  machine: {
    // Actions are defined in Sketchpad.tsx for now
    // TODO: Move home-specific actions here when Sketchpad.tsx is refactored
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: (): HomeState => ({
      panelVisibility: { toolbar: true, workbench: false, details: false, chat: false, settings: false },
      selection: undefined,
      sortColumn: undefined,
      sortDirection: undefined,
      loadingKits: [],
    }),
  },
};

if (typeof window !== "undefined") {
  registerAppPlugin(homeAppPlugin);

  // Register event handlers using the new dynamic dispatch system
  // These handlers are called directly by the sketchpad machine's APP_EVENT action
  registerEventHandler("HOME.TOGGLE_PANEL", {
    action: (context: any, event: any) => ({
      homeApp: {
        ...context.homeApp,
        panelVisibility: {
          ...context.homeApp.panelVisibility,
          [event.panel]: !context.homeApp.panelVisibility[event.panel],
        },
      },
    }),
  });

  registerEventHandler("HOME.SET_PANEL_VISIBILITY", {
    action: (context: any, event: any) => ({
      homeApp: { ...context.homeApp, panelVisibility: event.panelVisibility },
    }),
  });

  registerEventHandler("HOME.SET_SORT", {
    action: (context: any, event: any) => ({
      homeApp: { ...context.homeApp, sortColumn: event.column, sortDirection: event.direction },
    }),
  });

  registerEventHandler("HOME.SELECT_KIT", {
    action: (context: any, event: any) => {
      const kits = context.homeApp.selection?.kits || [];
      if (kits.includes(event.guid)) return {};
      return { homeApp: { ...context.homeApp, selection: { kits: [...kits, event.guid] } } };
    },
  });

  registerEventHandler("HOME.DESELECT_KIT", {
    action: (context: any, event: any) => {
      const kits = context.homeApp.selection?.kits || [];
      return { homeApp: { ...context.homeApp, selection: { kits: kits.filter((k: Guid) => k !== event.guid) } } };
    },
  });

  registerEventHandler("HOME.CLEAR_SELECTION", {
    action: (context: any) => ({ homeApp: { ...context.homeApp, selection: undefined } }),
  });

  registerEventHandler("HOME.SET_HOVER", {
    action: (context: any, event: any) => ({
      homeApp: { ...context.homeApp, hover: { kits: event.kits } },
    }),
  });

  registerEventHandler("HOME.CLEAR_HOVER", {
    guard: (context: any) => {
      const hover = context.homeApp.hover;
      return hover !== undefined && (hover.kits?.length ?? 0) > 0;
    },
    action: (context: any) => ({ homeApp: { ...context.homeApp, hover: undefined } }),
  });

  // Keep legacy runtime actions for backwards compatibility during migration
  registerRuntimeAction("homeTogglePanel", (context: any, event: any) => {
    if (event.type !== "HOME.TOGGLE_PANEL") return {};
    return {
      homeApp: {
        ...context.homeApp,
        panelVisibility: {
          ...context.homeApp.panelVisibility,
          [event.panel]: !context.homeApp.panelVisibility[event.panel],
        },
      },
    };
  });
  registerRuntimeAction("homeSetPanelVisibility", (context: any, event: any) => {
    if (event.type !== "HOME.SET_PANEL_VISIBILITY") return {};
    return { homeApp: { ...context.homeApp, panelVisibility: event.panelVisibility } };
  });
  registerRuntimeAction("homeSetSort", (context: any, event: any) => {
    if (event.type !== "HOME.SET_SORT") return {};
    return { homeApp: { ...context.homeApp, sortColumn: event.column, sortDirection: event.direction } };
  });
  registerRuntimeAction("homeSelectKit", (context: any, event: any) => {
    if (event.type !== "HOME.SELECT_KIT") return {};
    const kits = context.homeApp.selection?.kits || [];
    if (kits.includes(event.guid)) return {};
    return { homeApp: { ...context.homeApp, selection: { kits: [...kits, event.guid] } } };
  });
  registerRuntimeAction("homeDeselectKit", (context: any, event: any) => {
    if (event.type !== "HOME.DESELECT_KIT") return {};
    const kits = context.homeApp.selection?.kits || [];
    return { homeApp: { ...context.homeApp, selection: { kits: kits.filter((k: Guid) => k !== event.guid) } } };
  });
  registerRuntimeAction("homeClearSelection", (context: any) => ({ homeApp: { ...context.homeApp, selection: undefined } }));
  registerRuntimeAction("homeSetHover", (context: any, event: any) => {
    if (event.type !== "HOME.SET_HOVER") return {};
    return { homeApp: { ...context.homeApp, hover: { kits: event.kits } } };
  });
  registerRuntimeAction("homeClearHover", (context: any) => ({ homeApp: { ...context.homeApp, hover: undefined } }));
}

// #endregion Home App Plugin Registration

// #region Hooks (XState-based)

// Re-export hooks from Sketchpad for backwards compatibility
export { useHomeApp as useHomeAppExported, useHomeLoadingKits as useHomeLoadingKitsExported, useHomePanelVisibility as useHomePanelVisibilityExported, useHomeSelection as useHomeSelectionExported } from "./Sketchpad";
// Re-export the local alias
export { useHome };

// #endregion Hooks

// #region Commands

// #endregion Commands

// #region Navbar

// #endregion Navbar

// #region Canvas

// #region Windows

// #region Table

export {};

// #endregion Table

// #endregion Windows

// #region Panels

// #region Right

// #region Details

export const KitSection: FC = () => {
  const home = useHome() as HomeState;
  const selection = home?.selection;
  const selectedKits = selection?.kits || [];
  if (selectedKits.length === 0) return null;
  if (selectedKits.length === 1) return <SingleKitSection kitId={selectedKits[0]} />;
  return <MultipleKitsSection kitIds={selectedKits} />;
};

const SingleKitSection: FC<{ kitId: string }> = ({ kitId }) => {
  const kitShallows = useKitShallows();
  const kitShallow = kitShallows.find((k) => k.guid === kitId);
  if (!kitShallow) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.kit.notFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.home.panel.details.kit.name" value={kitShallow.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.home.panel.details.kit.version" value={kitShallow.version || ""} placeholder={useLabel("semio.sketchpad.app.kit.versionPlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.home.panel.details.kit.description" value={kitShallow.description || ""} placeholder={useLabel("semio.sketchpad.app.kit.descriptionPlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.home.panel.details.kit.icon" value={kitShallow.icon || ""} placeholder={useLabel("semio.sketchpad.app.kit.iconPlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.home.panel.details.kit.image" value={kitShallow.image || ""} placeholder={useLabel("semio.sketchpad.app.kit.imagePlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const MultipleKitsSection: FC<{ kitIds: string[] }> = ({ kitIds }) => {
  const kitShallows = useKitShallows();
  const kits = kitIds.map((id) => kitShallows.find((k) => k.guid === id)).filter((k) => k !== undefined) as KitShallow[];

  // Helper function to get common value or undefined if different
  const getCommonValue = <T,>(getter: (kit: KitShallow) => T): T | undefined => {
    if (kits.length === 0) return undefined;
    const firstValue = getter(kits[0]);
    const allSame = kits.every((kit) => getter(kit) === firstValue);
    return allSame ? firstValue : undefined;
  };

  const commonName = getCommonValue((k) => k.name);
  const commonVersion = getCommonValue((k) => k.version);
  const commonDescription = getCommonValue((k) => k.description);
  const commonIcon = getCommonValue((k) => k.icon);
  const commonImage = getCommonValue((k) => k.image);

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.home.panel.details.kits.name" value={commonName || ""} placeholder={commonName === undefined ? useLabel("semio.sketchpad.common.mixedValues") : undefined} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            id="semio.sketchpad.app.home.panel.details.kits.version"
            value={commonVersion || ""}
            placeholder={commonVersion === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.app.kit.versionPlaceholder.label")}
            readOnly
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            id="semio.sketchpad.app.home.panel.details.kits.description"
            value={commonDescription || ""}
            placeholder={commonDescription === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.app.kit.descriptionPlaceholder.label")}
            readOnly
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            id="semio.sketchpad.app.home.panel.details.kits.icon"
            value={commonIcon || ""}
            placeholder={commonIcon === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.app.kit.iconPlaceholder.label")}
            readOnly
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            id="semio.sketchpad.app.home.panel.details.kits.image"
            value={commonImage || ""}
            placeholder={commonImage === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.app.kit.imagePlaceholder.label")}
            readOnly
            showLabel
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

// #endregion Details

// #region Chat

const ChatPlaceholder: FC = () => {
  return (
    <TreeItem>
      <TreeContent>
        <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.panel.chat.placeholder")}</p>
      </TreeContent>
    </TreeItem>
  );
};

// #endregion Chat

// #region Settings

const SettingsContent: FC = () => {
  const [theme, setTheme, canSetTheme] = useTheme();
  const [language, setLanguage, canSetLanguage] = useLanguage();
  const [device, setDevice, canSetDevice] = useDevice();
  const [expertise, setExpertise, canSetExpertise] = useExpertise();
  const [mode, setMode, canSetMode] = useMode();

  const languageEnLabel = useLabel("semio.sketchpad.settings.language.en");
  const languageDeLabel = useLabel("semio.sketchpad.settings.language.de");
  const languagePlaceholder = useLabel("semio.sketchpad.app.home.settings.language.placeholder");

  return (
    <>
      <TreeItem>
        <TreeContent>
          <ToggleGroup
            id="semio.sketchpad.app.home.settings.theme"
            value={theme}
            onValueChange={(value: string) => setTheme?.(value as Theme)}
            showLabel
            kind="single"
            disabled={!canSetTheme}
            items={[
              { value: Theme.SYSTEM, id: "semio.sketchpad.settings.theme.system", icon: <MonitorIcon className="size-small" /> },
              { value: Theme.LIGHT, id: "semio.sketchpad.settings.theme.light", icon: <SunIcon className="size-small" /> },
              { value: Theme.DARK, id: "semio.sketchpad.settings.theme.dark", icon: <MoonIcon className="size-small" /> },
            ]}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Select id="semio.sketchpad.app.home.settings.language" value={language || "en"} onValueChange={(value: string) => setLanguage?.(value)} showLabel disabled={!canSetLanguage}>
            <SelectTrigger>
              <SelectValue placeholder={languagePlaceholder} />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="en">{languageEnLabel}</SelectItem>
              <SelectItem value="de">{languageDeLabel}</SelectItem>
            </SelectContent>
          </Select>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <ToggleGroup
            id="semio.sketchpad.app.home.settings.device"
            value={typeof device === "object" ? "desktop" : device}
            onValueChange={(value: string) => setDevice?.(value as "desktop" | "tablet")}
            showLabel
            kind="single"
            disabled={!canSetDevice}
            items={[
              { value: "desktop", id: "semio.sketchpad.settings.device.desktop", icon: <MousePointerIcon className="size-small" /> },
              { value: "tablet", id: "semio.sketchpad.settings.device.tablet", icon: <HandIcon className="size-small" /> },
            ]}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <ToggleGroup
            id="semio.sketchpad.app.home.settings.expertise"
            value={expertise}
            onValueChange={(value: string) => setExpertise?.(value as Expertise)}
            showLabel
            kind="single"
            disabled={!canSetExpertise}
            items={[
              { value: Expertise.BEGINNER, id: "semio.sketchpad.settings.expertise.beginner", icon: <TutorialIcon className="size-small" /> },
              { value: Expertise.NORMAL, id: "semio.sketchpad.settings.expertise.normal", icon: <UserIcon className="size-small" /> },
              { value: Expertise.EXPERT, id: "semio.sketchpad.settings.expertise.expert", icon: <AwardIcon className="size-small" /> },
            ]}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <ToggleGroup
            id="semio.sketchpad.app.home.settings.mode"
            value={mode}
            onValueChange={(value: string) => setMode?.(value as Mode)}
            showLabel
            kind="single"
            disabled={!canSetMode}
            items={[
              { value: Mode.USER, id: "semio.sketchpad.settings.mode.user", icon: <UserIcon className="size-small" /> },
              { value: Mode.DEV, id: "semio.sketchpad.settings.mode.dev", icon: <CodeIcon className="size-small" /> },
            ]}
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

// #endregion Settings

// #endregion Right

// #endregion Panels

// #region Tools

// #endregion Tools

// #endregion Canvas

// #region Footer

const HomeAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();

  useEffect(() => {
    if (appType !== "home") return;

    // TODO: Add home-specific footer items here

    return () => {
      // Cleanup
    };
  }, [appType, addFooterItem, removeFooterItem]);

  return null;
};

// #endregion Footer

// #region DropZone

const HomeDropZone: FC<{ children: React.ReactNode }> = ({ children }) => {
  const [isDragging, setIsDragging] = useState(false);
  const { t } = useTranslation();
  const { createKit, navigateToKit, storeKitFileBlobs } = useSketchpadCommands();
  const actor = useSketchpadActor();

  // Use XState background operations for tracking kit imports
  // This ensures imports continue even when navigating away from Home
  const startKitImport = (operationId: string, kitName: string) => {
    actor.send({ type: "BACKGROUND.START", operationId, operationType: `kit-import:${kitName}` });
  };

  const completeKitImport = (operationId: string) => {
    actor.send({ type: "BACKGROUND.COMPLETE", operationId });
  };

  const failKitImport = (operationId: string, error: string) => {
    actor.send({ type: "BACKGROUND.FAIL", operationId, error });
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.dataTransfer.items && e.dataTransfer.items.length > 0) {
      const hasZip = Array.from(e.dataTransfer.items).some((item) => item.kind === "file" && (item.type === "application/zip" || item.type === "application/x-zip-compressed"));
      if (hasZip) {
        setIsDragging(true);
      }
    }
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.currentTarget === e.target) {
      setIsDragging(false);
    }
  };

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    const files = Array.from(e.dataTransfer.files);
    const zipFile = files.find((f) => f.name.endsWith(".zip") || f.name.endsWith(".semio.zip"));

    if (zipFile) {
      const operationId = `kit-import-${guid()}`;
      const kitName = zipFile.name.replace(/\.(semio\.)?zip$/, "");
      startKitImport(operationId, kitName);
      // Allow React to render loading state before starting CPU-intensive import
      // Double rAF ensures the browser has painted the loading row
      await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
      try {
        const { kit, files: importedFiles } = await importKit(zipFile);
        await createKit("semio.sketchpad.app.home.dropzone", kit, false, false);
        // Store blobs for existing kit files BEFORE navigating (kit already has file definitions from SQLite)
        await storeKitFileBlobs(kit.guid, importedFiles);
        completeKitImport(operationId);
        // Don't auto-navigate - let user click the now-enabled row
      } catch (error) {
        console.error("[Home] Failed to import kit:", error);
        failKitImport(operationId, error instanceof Error ? error.message : String(error));
      }
    }
  };

  const handleFileInputChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    if (file.name.endsWith(".zip") || file.name.endsWith(".semio.zip")) {
      const operationId = `kit-import-${guid()}`;
      const kitName = file.name.replace(/\.(semio\.)?zip$/, "");
      startKitImport(operationId, kitName);
      // Allow React to render loading state before starting CPU-intensive import
      // Double rAF ensures the browser has painted the loading row
      await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
      try {
        const { kit, files: importedFiles } = await importKit(file);
        await createKit("semio.sketchpad.app.home.fileInput", kit, false, false);
        // Store blobs for existing kit files BEFORE navigating (kit already has file definitions from SQLite)
        await storeKitFileBlobs(kit.guid, importedFiles);
        completeKitImport(operationId);
        // Don't auto-navigate - let user click the now-enabled row
      } catch (error) {
        console.error("[Home] Failed to import kit:", error);
        failKitImport(operationId, error instanceof Error ? error.message : String(error));
      }
    }
    e.target.value = "";
  };

  return (
    <div className="relative h-full w-full" onDragOver={handleDragOver} onDragLeave={handleDragLeave} onDrop={handleDrop}>
      <input type="file" id="semio.sketchpad.app.home.importKit" accept=".zip" className="hidden" onChange={handleFileInputChange} />
      {children}
      {isDragging && (
        <div className="absolute inset-0 z-50 flex items-center justify-center bg-base/80 backdrop-blur-sm">
          <div className="flex flex-col items-center gap-2 text-center">
            <DocumentIcon className="h-12 w-12 text-muted-foreground" />
            <p className="text-lg font-medium">{t("semio.sketchpad.app.home.dropzone.label.normal")}</p>
            <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.home.dropzone.description.normal")}</p>
          </div>
        </div>
      )}
    </div>
  );
};

// #endregion DropZone

// #region App

type KitKind = "temporary" | "local" | "remote";

type TableRow = {
  id: string;
  name: string;
  level: number;
  parentId?: string;
  hasChildren: boolean;
  isExpanded: boolean;
  type: KitKind | "docs" | "loading";
  updatedAt: string;
  createdAt: string;
  kit?: KitShallow;
  docsPath?: string;
  icon?: string;
  isLoading?: boolean;
  concepts?: string[];
};

const Home: FC = ({}) => {
  const navigate = useNavigate();
  const navigation = useNavigation();
  const [searchParams, setSearchParams] = useSearchParams();
  const kits = useKits();
  const getKitKind = useGetKitKind();
  const { createKit, navigateToKit, getKitSnapshot } = useSketchpadCommands();

  const homeState = useHome() as any;
  const homeCommands = useHomeCommands();
  const isMobile = useIsMobile();
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const tooltip = useTooltip();

  const selection = homeState?.selection?.kits || [];

  const defaultKitName = useLabel("semio.sketchpad.app.kit.defaultName");
  const newVersionLabel = useLabel("semio.sketchpad.app.kit.newVersion");
  const defaultVersionLabel = useLabel("semio.sketchpad.app.kit.defaultVersion");

  // Dynamic details panel based on selection
  useEffect(() => {
    if (appType !== "home") return;

    const hasKits = selection.length > 0;
    const hasSingleKit = selection.length === 1;
    const hasMultipleKits = selection.length > 1;

    // Remove previous section
    removeSection("details", "semio.sketchpad.app.kit.properties");
    removeSection("details", "semio.sketchpad.app.home.kits.multiple");

    // Only show section if something is selected
    if (hasKits) {
      const sectionId = hasSingleKit ? "semio.sketchpad.app.kit.properties" : "semio.sketchpad.app.home.kits.multiple";
      addSection("details", {
        id: sectionId,
        specificity: 0,
        order: 0,
        content: () => {
          return <KitSection />;
        },
      });
    }

    return () => {
      removeSection("details", "semio.sketchpad.app.kit.properties");
      removeSection("details", "semio.sketchpad.app.home.kits.multiple");
    };
  }, [appType, addSection, removeSection, selection.length]);

  // Add chat panel section
  useEffect(() => {
    if (appType !== "home") return;

    addSection("chat", {
      id: "semio.sketchpad.app.home.chat",
      specificity: 0,
      order: 0,
      content: () => {
        return <ChatPlaceholder />;
      },
    });

    return () => {
      removeSection("chat", "semio.sketchpad.app.home.chat");
    };
  }, [appType, addSection, removeSection]);

  // Add settings panel sections
  useEffect(() => {
    if (appType !== "home") {
      return;
    }

    // Add Home-specific settings (most specific)
    addSection("settings", {
      id: "semio.sketchpad.app.home.settings",
      specificity: 20,
      order: 0,
      content: () => {
        return <SettingsContent />;
      },
    });

    // Add global Sketchpad settings (least specific)
    addSection("settings", {
      id: "semio.sketchpad.settings",
      specificity: 0,
      order: 0,
      content: () => {
        return <SettingsContent />;
      },
    });

    return () => {
      removeSection("settings", "semio.sketchpad.app.home.settings");
      removeSection("settings", "semio.sketchpad.settings");
    };
  }, [appType, addSection, removeSection]);

  // Get filters from search params (?kind=&name=&version=)
  const selectedKind = searchParams.get("kind") as KitKind | null;
  const selectedName = searchParams.get("name");
  const selectedVersion = searchParams.get("version");

  // Get search query from URL search params
  const searchQuery = searchParams.get("q") || "";

  // Get expanded rows from search params
  const expandedRowsParam = searchParams.getAll("e");
  const expandedRows = new Set(expandedRowsParam);

  const sortColumn = homeState?.sortColumn;
  const sortDirection = homeState?.sortDirection || "asc";

  // Collect unique names
  const uniqueNames = useMemo(() => {
    const nameSet = new Set<string>();
    kits.forEach((kit) => {
      const type = getKitKind(kit.guid) || "temporary";

      if (selectedKind && selectedKind !== type) return;
      nameSet.add(kit.name);
    });
    return Array.from(nameSet).sort();
  }, [kits, getKitKind, selectedKind]);

  // Collect unique versions for the selected name
  const uniqueVersions = useMemo(() => {
    if (!selectedName) return [];
    const versionSet = new Set<string>();
    kits.forEach((kit) => {
      if (kit.name === selectedName) {
        versionSet.add(kit.version || "");
      }
    });
    return Array.from(versionSet).sort();
  }, [kits, selectedName]);

  // Collect all unique concepts from kits
  // Note: In KitShallow, concepts are string[] (GUIDs), so we need to resolve them to names
  const allConcepts = useMemo(() => {
    const conceptSet = new Set<string>();
    kits.forEach((kitShallow) => {
      // Get full kit data to access Concept objects with names
      const fullKit = getKitSnapshot(kitShallow.guid);
      if (!fullKit) return;

      // Map concept GUIDs to their names
      kitShallow.concepts?.forEach((conceptEntry) => {
        const conceptGuid = typeof conceptEntry === "string" ? conceptEntry : (conceptEntry as any).guid;
        const concept = fullKit.concepts?.find((c) => c.guid === conceptGuid);
        if (concept?.name) conceptSet.add(concept.name);
      });
    });
    return Array.from(conceptSet).sort();
  }, [kits, getKitSnapshot]);

  // Get selected concepts from search params
  const selectedConcepts = useMemo(() => {
    const conceptsParam = searchParams.get("concepts");
    return conceptsParam ? conceptsParam.split(",").filter(Boolean) : [];
  }, [searchParams]);

  const rows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const locale = i18n.language === "de" ? de : enUS;
    const formatDate = (date?: Date | string) => {
      if (!date) return "";
      const parsedDate = date instanceof Date ? date : new Date(date);
      if (isNaN(parsedDate.getTime())) return "";
      return formatDistanceToNow(parsedDate, { addSuffix: true, locale });
    };

    // Add Docs section at the top
    const allDocsPages = docsRegistry.getAllPages();
    const allDocsSections = docsRegistry.getAllSections();

    const docsParentId = "docs-root";
    result.push({
      id: docsParentId,
      name: "Documentation",
      level: 0,
      hasChildren: true,
      isExpanded: expandedRows.has(docsParentId),
      type: "docs",
      updatedAt: "",
      createdAt: "",
      kit: undefined,
      docsPath: undefined,
    });

    if (expandedRows.has(docsParentId)) {
      allDocsSections.forEach((section) => {
        const sectionId = `docs-section-${section.id}`;
        const sectionPages = docsRegistry.getPagesBySection(section.id);

        result.push({
          id: sectionId,
          name: section.label,
          level: 1,
          parentId: docsParentId,
          hasChildren: sectionPages.length > 0,
          isExpanded: expandedRows.has(sectionId),
          type: "docs",
          updatedAt: "",
          createdAt: "",
          kit: undefined,
          docsPath: undefined,
          icon: section.icon,
        });

        if (expandedRows.has(sectionId)) {
          sectionPages.forEach((page) => {
            result.push({
              id: `docs-page-${page.path}`,
              name: page.title,
              level: 2,
              parentId: sectionId,
              hasChildren: false,
              isExpanded: false,
              type: "docs",
              updatedAt: "",
              createdAt: "",
              kit: undefined,
              docsPath: page.path,
              icon: page.icon,
            });
          });
        }
      });
    }

    // Add loading kits at the top (after docs)
    const loadingKits = homeState?.loadingKits || [];
    loadingKits.forEach((loadingKit: LoadingKit) => {
      result.push({
        id: `loading-${loadingKit.tempGuid}`,
        name: loadingKit.name,
        level: 0,
        hasChildren: false,
        isExpanded: false,
        type: "loading",
        updatedAt: "",
        createdAt: "",
        kit: undefined,
        isLoading: true,
      });
    });

    const kitGroups = new Map<string, KitShallow[]>();

    kits.forEach((kit) => {
      const type = getKitKind(kit.guid) || "temporary";

      if (selectedKind && selectedKind !== type) return;
      if (searchQuery && !kit.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
      if (selectedName && kit.name !== selectedName) return;
      if (selectedVersion && (kit.version || "") !== selectedVersion) return;

      // Filter by concepts: resolve GUIDs to names first
      if (selectedConcepts.length > 0) {
        const fullKit = getKitSnapshot(kit.guid);
        if (!fullKit) return;
        const kitConceptNames =
          kit.concepts
            ?.map((conceptEntry) => {
              const conceptGuid = typeof conceptEntry === "string" ? conceptEntry : (conceptEntry as any).guid;
              return fullKit.concepts?.find((c) => c.guid === conceptGuid)?.name;
            })
            .filter((name): name is string => name !== undefined) || [];

        if (!kitConceptNames.some((name) => selectedConcepts.includes(name))) return;
      }

      const key = kit.name;
      if (!kitGroups.has(key)) kitGroups.set(key, []);
      kitGroups.get(key)!.push(kit);
    });

    // Helper to resolve concept GUIDs to names
    const resolveKitConcepts = (kit: KitShallow): string[] => {
      const fullKit = getKitSnapshot(kit.guid);
      if (!fullKit) return [];
      return (
        kit.concepts
          ?.map((conceptEntry) => {
            const conceptGuid = typeof conceptEntry === "string" ? conceptEntry : (conceptEntry as any).guid;
            return fullKit.concepts?.find((c) => c.guid === conceptGuid)?.name;
          })
          .filter((name): name is string => name !== undefined) || []
      );
    };

    kitGroups.forEach((groupKits, name) => {
      const parentId = `kit-${name}`;
      const defaultKit = groupKits.find((k) => !k.version);
      const parentKit = defaultKit || groupKits[0];
      const hasChildren = groupKits.some((k) => k.guid !== parentKit.guid);

      const type = getKitKind(parentKit.guid) || "temporary";

      result.push({
        id: parentId,
        name: name,
        level: 0,
        hasChildren,
        isExpanded: expandedRows.has(parentId),
        type,
        updatedAt: formatDate(parentKit.updatedAt),
        createdAt: formatDate(parentKit.createdAt),
        kit: parentKit,
        concepts: resolveKitConcepts(parentKit),
      });

      if (expandedRows.has(parentId) && hasChildren) {
        groupKits.forEach((kit) => {
          if (kit.guid === parentKit.guid) return;
          const kitKind = getKitKind(kit.guid) || "temporary";

          const versionId = `${parentId}-${kit.version || "default"}`;
          result.push({
            id: versionId,
            name: kit.version || "(default)",
            level: 1,
            parentId,
            hasChildren: false,
            isExpanded: false,
            type: kitKind,
            updatedAt: formatDate(kit.updatedAt),
            createdAt: formatDate(kit.createdAt),
            kit: kit,
            concepts: resolveKitConcepts(kit),
          });
        });
      }
    });

    if (sortColumn) {
      const topLevelRows = result.filter((r) => r.level === 0);
      const childRows = result.filter((r) => r.level > 0);
      topLevelRows.sort((a, b) => {
        let comparison = 0;
        switch (sortColumn) {
          case "name":
            comparison = a.name.localeCompare(b.name);
            break;
          case "type":
            comparison = a.type.localeCompare(b.type);
            break;
          case "updatedAt":
            comparison = a.updatedAt.localeCompare(b.updatedAt);
            break;
          case "createdAt":
            comparison = a.createdAt.localeCompare(b.createdAt);
            break;
        }
        return sortDirection === "asc" ? comparison : -comparison;
      });
      const sortedResult: TableRow[] = [];
      topLevelRows.forEach((parent) => {
        sortedResult.push(parent);
        const children = childRows.filter((c) => c.parentId === parent.id);
        children.sort((a, b) => {
          let comparison = 0;
          switch (sortColumn) {
            case "name":
              comparison = a.name.localeCompare(b.name);
              break;
            case "type":
              comparison = a.type.localeCompare(b.type);
              break;
            case "updatedAt":
              comparison = a.updatedAt.localeCompare(b.updatedAt);
              break;
            case "createdAt":
              comparison = a.createdAt.localeCompare(b.createdAt);
              break;
          }
          return sortDirection === "asc" ? comparison : -comparison;
        });
        sortedResult.push(...children);
      });
      return sortedResult;
    }

    return result;
  }, [kits, getKitKind, selectedKind, searchQuery, selectedName, selectedVersion, expandedRows, sortColumn, sortDirection, homeState?.loadingKits]);

  const { setFocusItems, setOnFocusItem } = useFocus();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const prevRowsRef = useRef<string>("");

  useEffect(() => {
    const items = rows.map((row) => ({
      id: row.id,
      label: row.name,
      category: row.level === 0 ? "Kits" : "Versions",
    }));
    // Only update if the items have actually changed
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevRowsRef.current !== itemsKey) {
      prevRowsRef.current = itemsKey;
      setFocusItems(items);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rows]);

  useEffect(() => {
    const handleFocus = (itemId: string) => {
      setFocusedItemId(itemId);
    };
    setOnFocusItem(handleFocus);
    return () => setOnFocusItem(undefined);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleCreateKit = (type: KitKind) => {
    const existingNames = kits.map((k) => k.name);
    const uniqueName = generateUniqueName(defaultKitName, existingNames);
    const newKit: Kit = {
      guid: guid(),
      name: uniqueName,
      version: "",
      types: [],
      designs: [],
    };
    const local = type === "local" || type === "remote";
    const remote = type === "remote";
    createKit("semio.sketchpad.app.home.canvas.table.createKit", newKit, local, remote);
    navigateToKit(newKit.guid);
  };

  const handleCreateVersion = (kitName: string, type: KitKind) => {
    const existingVersions = kits.filter((k) => k.name === kitName).map((k) => k.version || "");
    const uniqueVersion = generateUniqueName(newVersionLabel, existingVersions);
    const newKit: Kit = {
      guid: guid(),
      name: kitName,
      version: uniqueVersion,
      types: [],
      designs: [],
    };
    const local = type === "local" || type === "remote";
    const remote = type === "remote";
    createKit("semio.sketchpad.app.home.canvas.table.createVersion", newKit, local, remote);
    navigateToKit(newKit.guid);
  };

  const toggleKind = (type: KitKind) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedKind === type) {
      newParams.delete("kind");
      newParams.delete("name");
      newParams.delete("version");
    } else {
      newParams.set("kind", type);
      newParams.delete("name");
      newParams.delete("version");
    }
    setSearchParams(newParams);
  };

  // Register hotkeys for filter toggles
  useHotkeys("semio.sketchpad.app.home.filter.kind.temporary", () => toggleKind("temporary"));
  useHotkeys("semio.sketchpad.app.home.filter.kind.local", () => toggleKind("local"));
  useHotkeys("semio.sketchpad.app.home.filter.kind.remote", () => toggleKind("remote"));

  const toggleName = (name: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedName === name) {
      newParams.delete("name");
      newParams.delete("version");
    } else {
      newParams.set("name", name);
      newParams.delete("version");
    }
    setSearchParams(newParams);
  };

  const toggleVersion = (version: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedVersion === version) {
      newParams.delete("version");
    } else {
      newParams.set("version", version);
    }
    setSearchParams(newParams);
  };

  const toggleConcept = (concept: string) => {
    const newParams = new URLSearchParams(searchParams);
    const currentConcepts = newParams.get("concepts")?.split(",").filter(Boolean) || [];
    if (currentConcepts.includes(concept)) {
      const updated = currentConcepts.filter((c) => c !== concept);
      if (updated.length > 0) {
        newParams.set("concepts", updated.join(","));
      } else {
        newParams.delete("concepts");
      }
    } else {
      newParams.set("concepts", [...currentConcepts, concept].join(","));
    }
    setSearchParams(newParams);
  };

  const toggleRow = (rowId: string) => {
    const currentUrl = new URL(window.location.href);
    const currentRows = currentUrl.searchParams.getAll("e");

    if (currentRows.includes(rowId)) {
      // Remove row
      currentUrl.searchParams.delete("e");
      currentRows.filter((r) => r !== rowId).forEach((r) => currentUrl.searchParams.append("e", r));
    } else {
      // Add row
      currentUrl.searchParams.append("e", rowId);
    }

    // Use native History API to preserve forward/back navigation
    // Preserve the existing history state to avoid breaking React Router
    window.history.replaceState(window.history.state, "", currentUrl);
    // Trigger popstate to notify React Router of the URL change
    window.dispatchEvent(new PopStateEvent("popstate", { state: window.history.state }));
  };

  const handleSearchChange = (value: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (value) {
      newParams.set("q", value);
    } else {
      newParams.delete("q");
    }
    setSearchParams(newParams);
  };

  const handleRowClick = (kitId: string, e: React.MouseEvent) => {
    if (e.shiftKey) {
      const currentIndex = rows.findIndex((r) => r.kit?.guid === kitId);
      if (selection.length > 0) {
        const lastSelectedId = selection[selection.length - 1];
        const lastIndex = rows.findIndex((r) => r.kit?.guid === lastSelectedId);
        if (lastIndex !== -1 && currentIndex !== -1) {
          const start = Math.min(lastIndex, currentIndex);
          const end = Math.max(lastIndex, currentIndex);
          const rangeIds = rows
            .slice(start, end + 1)
            .map((r) => r.kit?.guid)
            .filter((id): id is string => id !== undefined);
          homeCommands.selectKits("semio.sketchpad.app.home.canvas.table.selectKitsRange", rangeIds);
        }
      } else {
        homeCommands.selectKit("semio.sketchpad.app.home.canvas.table.selectKitShift", kitId);
      }
    } else if (e.metaKey || e.ctrlKey) {
      if (selection.includes(kitId)) {
        homeCommands.removeKitFromSelection("semio.sketchpad.app.home.canvas.table.removeKitCtrl", kitId);
      } else {
        homeCommands.addKitToSelection("semio.sketchpad.app.home.canvas.table.addKitCtrl", kitId);
      }
    } else {
      homeCommands.selectKit("semio.sketchpad.app.home.canvas.table.selectKit", kitId);
    }
  };

  const handleSortClick = (column: "name" | "type" | "updatedAt" | "createdAt") => {
    homeCommands.toggleSort("semio.sketchpad.app.home.canvas.table.toggleSort", column);
  };

  if (isMobile) {
    return (
      <HomeDropZone>
        <div
          className="flex flex-col h-full"
          onClick={(e: React.MouseEvent) => {
            if (e.target === e.currentTarget) {
              homeCommands.deselectAll("semio.sketchpad.app.home.canvas.table.deselect");
            }
          }}
        >
          <Band
            id="semio.sketchpad.app.home.filter.band"
            items={[
              ...(selectedKind
                ? [
                    <Toggle
                      kind="withAction"
                      pressed={true}
                      onPressedChange={() => toggleKind(selectedKind)}
                      actionIcon={<AddIcon />}
                      onActionClick={() => handleCreateKit(selectedKind)}
                      id="semio.sketchpad.app.home.filter.kind.show"
                      actionId="semio.sketchpad.app.home.filter.kind.create"
                      icon={
                        <>
                          {selectedKind === "temporary" && <TemporaryKitIcon />}
                          {selectedKind === "local" && <LocalKitIcon />}
                          {selectedKind === "remote" && <RemoteKitIcon />}
                        </>
                      }
                    />,
                  ]
                : [
                    <Toggle
                      kind="withAction"
                      pressed={false}
                      onPressedChange={() => toggleKind("temporary")}
                      actionIcon={<AddIcon />}
                      onActionClick={() => handleCreateKit("temporary")}
                      id="semio.sketchpad.app.home.filter.kind.temporary"
                      actionId="semio.sketchpad.app.home.filter.kind.createTemporary"
                      icon={<TemporaryKitIcon />}
                    />,
                    <Toggle
                      kind="withAction"
                      pressed={false}
                      onPressedChange={() => toggleKind("local")}
                      actionIcon={<AddIcon />}
                      onActionClick={() => handleCreateKit("local")}
                      id="semio.sketchpad.app.home.filter.kind.local"
                      actionId="semio.sketchpad.app.home.filter.kind.createLocal"
                      icon={<LocalKitIcon />}
                    />,
                    <Toggle
                      kind="withAction"
                      pressed={false}
                      onPressedChange={() => toggleKind("remote")}
                      actionIcon={<AddIcon />}
                      onActionClick={() => handleCreateKit("remote")}
                      id="semio.sketchpad.app.home.filter.kind.remote"
                      actionId="semio.sketchpad.app.home.filter.kind.createRemote"
                      icon={<RemoteKitIcon />}
                    />,
                  ]),
              ...(selectedName ? [<Toggle pressed={true} onPressedChange={() => toggleName(selectedName)} id="semio.sketchpad.app.home.filter.name" icon={<span className="size-small">N</span>} text={selectedName} />] : []),
              ...(selectedVersion !== null ? [<Toggle pressed={true} onPressedChange={() => toggleVersion(selectedVersion)} id="semio.sketchpad.app.home.filter.version" icon={selectedVersion || defaultVersionLabel} />] : []),
              ...(selectedKind && !selectedName && uniqueNames.length > 0
                ? uniqueNames.map((name) => <Toggle key={name} id={`semio.sketchpad.app.home.filter.name.${name}`} pressed={false} onPressedChange={() => toggleName(name)} icon={<span className="size-small">N</span>} text={name} />)
                : []),
              ...(selectedKind && selectedName && selectedVersion === null && uniqueVersions.length > 0
                ? uniqueVersions.map((version) => (
                    <Toggle key={version} id={`semio.sketchpad.app.home.filter.version.${version}`} pressed={false} onPressedChange={() => toggleVersion(version)} icon={version || <span className="italic opacity-50">{defaultVersionLabel}</span>} />
                  ))
                : []),
              <Input key="search" id="semio.sketchpad.app.home.search" className="flex-1 min-w-[160px]" placeholder={useLabel("semio.sketchpad.app.home.searchPlaceholder")} value={searchQuery} onChange={(e) => handleSearchChange(e.target.value)} />,
            ].map((content) => ({ content }))}
          />
          <ConceptFilter allConcepts={allConcepts} />
          <div className="flex items-center justify-between border-b px-single h-large">
            <span className="font-medium">{useLabel("semio.sketchpad.app.home.name")}</span>
            <Toggle
              kind="dropdown"
              pressed={sortColumn === "name"}
              value={sortColumn === "name" ? sortDirection : "asc"}
              onValueChange={(value) => {
                homeCommands.setSortColumn("semio.sketchpad.app.home.filter.name.sortColumn", "name");
                homeCommands.setSortDirection("semio.sketchpad.app.home.header.name.sortDirection", value as "asc" | "desc");
              }}
              items={[
                { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
              ]}
              id={"semio.sketchpad.app.home.sortByName"}
            />
          </div>
          <Scrollable className="flex-1">
            <div className="flex flex-col">
              {rows.map((row) => {
                const isSelected = row.kit ? selection.includes(row.kit.guid) : false;
                const isDocsRow = row.type === "docs";
                const isLoadingRow = row.isLoading;
                return (
                  <div
                    key={row.id}
                    className={`border-b p-single cursor-selectable h-medium ${isLoadingRow ? "opacity-50 pointer-events-none" : ""} ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`}
                    role="button"
                    tabIndex={isLoadingRow ? -1 : 0}
                    onClick={(e) => {
                      if (isLoadingRow) return;
                      if (isDocsRow && row.docsPath) {
                        navigate(`/${row.docsPath}`);
                      } else if (row.kit) {
                        handleRowClick(row.kit.guid, e);
                      }
                    }}
                    onDoubleClick={() => {
                      if (isLoadingRow) return;
                      if (isDocsRow && row.docsPath) {
                        navigate(`/${row.docsPath}`);
                      } else if (row.kit) {
                        navigateToKit(row.kit.guid);
                      }
                    }}
                  >
                    <div className="flex items-center gap-single justify-between" style={{ paddingLeft: `calc(${row.level} * var(--size-small))` }} onClick={(e) => e.stopPropagation()}>
                      <div className="flex items-center gap-single flex-1 min-w-0">
                        {row.hasChildren ? (
                          <Action
                            level="base"
                            id={"semio.sketchpad.app.home.toggleRow"}
                            onClick={(e) => {
                              e.stopPropagation();
                              toggleRow(row.id);
                            }}
                            icon={row.isExpanded ? <ChevronDownIcon className="size-small" /> : <ChevronRightIcon className="size-small" />}
                          />
                        ) : (
                          <span className="size-small shrink-0" />
                        )}
                        <TableAvatar name={row.name} icon={row.kit?.icon} />
                        <span className="text-left flex-1 min-w-0 truncate">{row.name}</span>
                        {isLoadingRow && <Spinner size="small" />}
                      </div>
                      <div className="flex items-center gap-single shrink-0">
                        {row.level === 0 && row.type !== "docs" && row.type !== "loading" && (
                          <Action
                            level="base"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleCreateVersion(row.name, row.type as KitKind);
                            }}
                            id={"semio.sketchpad.app.home.createVersion"}
                            icon={<AddIcon />}
                          />
                        )}
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          </Scrollable>
        </div>
      </HomeDropZone>
    );
  }

  return (
    <>
      <HomeAppFooter />
      <HomeDropZone>
        <Canvas>
          <Window id="home-table">
            <div
              className="flex flex-col h-full"
              onClick={(e: React.MouseEvent) => {
                if (e.target === e.currentTarget) {
                  homeCommands.deselectAll("semio.sketchpad.app.home.canvas.table.deselect");
                }
              }}
            >
              <Band
                id="semio.sketchpad.app.home.filter.band"
                items={[
                  ...(selectedKind
                    ? [
                        <Toggle
                          kind="withAction"
                          pressed={true}
                          onPressedChange={() => toggleKind(selectedKind)}
                          actionIcon={<AddIcon />}
                          onActionClick={() => handleCreateKit(selectedKind)}
                          id={"semio.sketchpad.app.home.hideKind"}
                          actionId={"semio.sketchpad.app.home.createKit"}
                          icon={
                            <>
                              {selectedKind === "temporary" && <TemporaryKitIcon />}
                              {selectedKind === "local" && <LocalKitIcon />}
                              {selectedKind === "remote" && <RemoteKitIcon />}
                            </>
                          }
                        />,
                      ]
                    : [
                        <Toggle
                          kind="withAction"
                          pressed={false}
                          onPressedChange={() => toggleKind("temporary")}
                          actionIcon={<AddIcon />}
                          onActionClick={() => handleCreateKit("temporary")}
                          id={"semio.sketchpad.app.home.showTemporary"}
                          actionId={"semio.sketchpad.app.home.createTemporary"}
                          icon={<TemporaryKitIcon />}
                        />,
                        <Toggle
                          kind="withAction"
                          pressed={false}
                          onPressedChange={() => toggleKind("local")}
                          actionIcon={<AddIcon />}
                          onActionClick={() => handleCreateKit("local")}
                          id={"semio.sketchpad.app.home.showLocal"}
                          actionId={"semio.sketchpad.app.home.createLocal"}
                          icon={<LocalKitIcon />}
                        />,
                        <Toggle
                          kind="withAction"
                          pressed={false}
                          onPressedChange={() => toggleKind("remote")}
                          actionIcon={<AddIcon />}
                          onActionClick={() => handleCreateKit("remote")}
                          id={"semio.sketchpad.app.home.showRemote"}
                          actionId={"semio.sketchpad.app.home.createRemote"}
                          icon={<RemoteKitIcon />}
                        />,
                      ]),
                  ...(selectedName ? [<Toggle id={`semio.sketchpad.app.home.filter.name.${selectedName}`} pressed={true} onPressedChange={() => toggleName(selectedName)} icon={<span className="size-small">N</span>} text={selectedName} />] : []),
                  ...(selectedVersion !== null ? [<Toggle id={`semio.sketchpad.app.home.filter.version.${selectedVersion}`} pressed={true} onPressedChange={() => toggleVersion(selectedVersion)} icon={selectedVersion || defaultVersionLabel} />] : []),
                  ...(selectedKind && !selectedName && uniqueNames.length > 0
                    ? uniqueNames.map((name) => <Toggle key={name} id={`semio.sketchpad.app.home.filter.name.${name}`} pressed={false} onPressedChange={() => toggleName(name)} icon={<span className="size-small">N</span>} text={name} />)
                    : []),
                  ...(selectedKind && selectedName && selectedVersion === null && uniqueVersions.length > 0
                    ? uniqueVersions.map((version) => <Toggle key={version} id={`semio.sketchpad.app.home.filter.version.${version}`} pressed={false} onPressedChange={() => toggleVersion(version)} icon={version || defaultVersionLabel} />)
                    : []),
                  <Input
                    key="search"
                    id="semio.sketchpad.app.home.search"
                    className="flex-1 min-w-[200px]"
                    placeholder={useLabel("semio.sketchpad.app.home.searchPlaceholder")}
                    value={searchQuery}
                    onChange={(e) => handleSearchChange(e.target.value)}
                  />,
                ].map((content) => ({ content }))}
              />
              {/* Concept Filter */}
              <ConceptFilter allConcepts={allConcepts} />
              <Table<TableRow>
                className="flex-1 min-h-0"
                columns={[
                  ...(!selectedKind
                    ? [
                        {
                          id: "type",
                          header: (
                            <div className="inline-flex items-center gap-single">
                              <span>{useLabel("semio.sketchpad.app.home.kind")}</span>
                              <Toggle
                                kind="dropdown"
                                pressed={sortColumn === "type"}
                                value={sortColumn === "type" ? sortDirection : "asc"}
                                onValueChange={(value) => {
                                  homeCommands.setSortColumn("semio.sketchpad.app.home.header.type.sortColumn", "type");
                                  homeCommands.setSortDirection("semio.sketchpad.app.home.header.type.sortDirection", value as "asc" | "desc");
                                }}
                                items={[
                                  { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                                  { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
                                ]}
                                id={"semio.sketchpad.app.home.sortByType"}
                              />
                            </div>
                          ),
                          accessor: (row) => (
                            <>
                              {row.type === "temporary" && <TemporaryKitIcon />}
                              {row.type === "local" && <LocalKitIcon />}
                              {row.type === "remote" && <RemoteKitIcon />}
                              {row.type === "docs" && <DocumentIcon className="size-small" />}
                            </>
                          ),
                          width: "w-0 whitespace-nowrap",
                          headerClassName: "relative group w-0 whitespace-nowrap",
                        } as TableColumn<TableRow>,
                      ]
                    : []),
                  {
                    id: "name",
                    header: (
                      <div className="flex items-center justify-between w-full">
                        <span>{useLabel("semio.sketchpad.app.home.name")}</span>
                        <Toggle
                          kind="dropdown"
                          pressed={sortColumn === "name"}
                          value={sortColumn === "name" ? sortDirection : "asc"}
                          onValueChange={(value) => {
                            homeCommands.setSortColumn("semio.sketchpad.app.home.header.name.sortColumn", "name");
                            homeCommands.setSortDirection("semio.sketchpad.app.home.header.name.sortDirection", value as "asc" | "desc");
                          }}
                          items={[
                            { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                            { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
                          ]}
                          id={"semio.sketchpad.app.home.sortByName"}
                        />
                      </div>
                    ),
                    accessor: (row) => (
                      <div className={`flex items-center gap-single justify-between ${row.isLoading ? "opacity-50 pointer-events-none" : ""}`} style={{ paddingLeft: `calc(${row.level} * var(--size-small))` }} onClick={(e) => e.stopPropagation()}>
                        <div className="flex items-center gap-single flex-1 min-w-0">
                          {row.hasChildren ? (
                            <Action
                              level="base"
                              id={"semio.sketchpad.app.home.toggleRow"}
                              onClick={(e) => {
                                e.stopPropagation();
                                toggleRow(row.id);
                              }}
                              icon={row.isExpanded ? <ChevronDownIcon className="size-small" /> : <ChevronRightIcon className="size-small" />}
                            />
                          ) : (
                            <span className="size-small shrink-0" />
                          )}
                          <TableAvatar name={row.name} icon={row.type === "docs" ? row.icon : row.kit?.icon} />
                          <span className="text-left min-w-0 truncate">{row.name}</span>
                          {row.isLoading && <Spinner size="small" />}
                        </div>
                        {row.concepts && row.concepts.length > 0 && (
                          <Scrollable orientation="horizontal" className="flex-1 min-w-0 max-w-[200px]">
                            <div className="flex items-center gap-single px-single h-medium w-fit">
                              {row.concepts.map((concept) => (
                                <Action key={concept} onClick={() => toggleConcept(concept)} id={`semio.sketchpad.app.home.row.concept.${concept}`} text={concept} className={selectedConcepts.includes(concept) ? "bg-active-base" : ""} />
                              ))}
                            </div>
                          </Scrollable>
                        )}
                        <div className="flex items-center gap-single shrink-0">
                          {row.level === 0 && row.type !== "docs" && row.type !== "loading" && (
                            <Action
                              level="base"
                              onClick={(e) => {
                                e.stopPropagation();
                                handleCreateVersion(row.name, row.type as KitKind);
                              }}
                              id={"semio.sketchpad.app.home.createVersion"}
                              icon={<AddIcon />}
                            />
                          )}
                        </div>
                      </div>
                    ),
                    headerClassName: "relative group",
                  },
                  {
                    id: "updatedAt",
                    header: (
                      <div className="flex items-center justify-between w-full">
                        <span>{useLabel("semio.sketchpad.app.home.lastUpdated")}</span>
                        <Toggle
                          kind="dropdown"
                          pressed={sortColumn === "updatedAt"}
                          value={sortColumn === "updatedAt" ? sortDirection : "asc"}
                          onValueChange={(value) => {
                            homeCommands.setSortColumn("semio.sketchpad.app.home.header.updatedAt.sortColumn", "updatedAt");
                            homeCommands.setSortDirection("semio.sketchpad.app.home.header.updatedAt.sortDirection", value as "asc" | "desc");
                          }}
                          items={[
                            { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                            { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
                          ]}
                          id={"semio.sketchpad.app.home.sortByUpdatedAt"}
                        />
                      </div>
                    ),
                    accessor: (row) => row.updatedAt,
                    headerClassName: "relative group",
                  },
                  {
                    id: "createdAt",
                    header: (
                      <div className="flex items-center justify-between w-full">
                        <span>{useLabel("semio.sketchpad.app.home.created")}</span>
                        <Toggle
                          kind="dropdown"
                          pressed={sortColumn === "createdAt"}
                          value={sortColumn === "createdAt" ? sortDirection : "asc"}
                          onValueChange={(value) => {
                            homeCommands.setSortColumn("semio.sketchpad.app.home.header.createdAt.sortColumn", "createdAt");
                            homeCommands.setSortDirection("semio.sketchpad.app.home.header.createdAt.sortDirection", value as "asc" | "desc");
                          }}
                          items={[
                            { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                            { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
                          ]}
                          id={"semio.sketchpad.app.home.sortByCreatedAt"}
                        />
                      </div>
                    ),
                    accessor: (row) => row.createdAt,
                    headerClassName: "relative group",
                  },
                ]}
                data={rows}
                onRowClick={(row, _, e) => {
                  const isDocsRow = row.type === "docs";
                  if (isDocsRow && row.docsPath) {
                    navigate(`/${row.docsPath}`);
                  } else if (row.kit) {
                    handleRowClick(row.kit.guid, e);
                  }
                }}
                onRowDoubleClick={(row) => {
                  const isDocsRow = row.type === "docs";
                  if (isDocsRow && row.docsPath) {
                    navigate(`/${row.docsPath}`);
                  } else if (row.kit) {
                    navigateToKit(row.kit.guid);
                  }
                }}
                rowKey={(row) => row.id}
                getRowId={(row) => row.kit?.guid || row.id}
                selectedRows={new Set(selection)}
                focusedItemId={focusedItemId}
                onFocusComplete={() => setFocusedItemId(undefined)}
                emptyMessage={useLabel("semio.sketchpad.app.home.noKits")}
                stickyHeader={true}
                headerClassName="sticky top-0 border-b"
                hierarchical={true}
              />
            </div>
          </Window>
        </Canvas>
      </HomeDropZone>
    </>
  );
};

export default Home;

// #endregion App

// #region Config

export const config: AppConfig = {
  id: "home",
  component: Home,
  routeSegments: [],
  additionalPaths: ["kits"],
  getPanels: (): PanelDefinition[] => [
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
    createPanelDefinition(PanelKind.CHAT, "semio.sketchpad.navbar.panelToggle.chat.show"),
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show"),
  ],
  matchesPath: (pathParts) => pathParts.length === 0 || (pathParts.length === 1 && pathParts[0] === "kits"),
  order: 0,
};

// #endregion Config
