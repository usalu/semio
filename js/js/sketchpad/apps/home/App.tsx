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

import { AddIcon, AwardIcon, ChevronDownIcon, ChevronRightIcon, CodeIcon, DocumentIcon, HandIcon, LocalKitIcon, MonitorIcon, MoonIcon, MousePointerIcon, RemoteKitIcon, SmartphoneIcon, SortAscendingIcon, SortDescendingIcon, SunIcon, TabletIcon, TemporaryKitIcon, TutorialIcon, UserIcon } from "@semio/assets";
import { formatDistanceToNow } from "date-fns";
import { de, enUS } from "date-fns/locale";
import { FC, useEffect, useMemo, useRef, useState } from "react";
import { useNavigate, useSearchParams } from "react-router";
import * as Y from "yjs";
import i18n, { useLabel } from "../../../i18n";
import { generateUniqueName, guid, Guid, Kit, KitShallow } from "../../../semio";
import type { SketchpadStore } from "../../App";
import {
  AppStore,
  Canvas,
  ConceptFilter,
  identitySelector,
  registerHomeStoreFactory,
  useAddFooterItem,
  useAddPanelSection,
  useAppType,
  useExpertise,
  useFocus,
  useGetKitKind,
  useHotkeys,
  useIsMobile,
  useKits,
  useKitShallows,
  useLayout,
  useMode,
  useNavigation,
  useRemoveFooterItem,
  useRemovePanelSection,
  useSketchpadCommands,
  useSketchpadStore,
  useSyncDeep,
  useTheme,
  useTooltip,
  Window,
} from "../../App";
import { Action, Input, Scrollable, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Strip, Table, TableAvatar, TableColumn, Textarea, Toggle, ToggleGroup, TreeContent, TreeItem } from "../../elements";
import type { AppEdit, PanelDefinition, PanelVisibility } from "../../sketchpad";
import { createPanelDefinition, Expertise, Layout, Mode, PanelKind, Theme } from "../../sketchpad";
import { docsRegistry } from "../docs/App";
import { AppConfig } from "../index";

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

export interface HomeState {
  panelVisibility: PanelVisibility;
  selection?: HomeSelection;
  sortColumn?: HomeSortColumn;
  sortDirection?: HomeSortDirection;
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

// #region Store

export class HomeStore extends AppStore<HomeState, HomeDiff, HomeSelectionDiff, HomeEdit, HomeCommandContext, HomeCommandResult> {
  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: (fn: () => void) => void) {
    super(parent, yMap, transact);

    transact(() => {
      if (!yMap.has("panelVisibility")) {
        const yPanelVisibility = new Y.Map<boolean>();
        yPanelVisibility.set("toolbar", false);
        yPanelVisibility.set("workbench", false);
        yPanelVisibility.set("details", false);
        yPanelVisibility.set("chat", false);
        yPanelVisibility.set("settings", false);
        yMap.set("panelVisibility", yPanelVisibility);
      }
      if (!yMap.has("isTransactionActive")) {
        yMap.set("isTransactionActive", false);
      }
      if (!yMap.has("currentTransactionStack")) {
        yMap.set("currentTransactionStack", new Y.Array<any>());
      }
      if (!yMap.has("pastTransactionsStack")) {
        yMap.set("pastTransactionsStack", new Y.Array<any>());
      }
      if (!yMap.has("redoStack")) {
        yMap.set("redoStack", new Y.Array<any>());
      }
    });
  }

  get panelVisibility(): PanelVisibility {
    const yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
    if (!yPanelVisibility) {
      return {
        toolbar: false,
        workbench: false,
        details: false,
        chat: false,
        settings: false,
      };
    }
    return {
      toolbar: yPanelVisibility.get("toolbar") ?? false,
      workbench: yPanelVisibility.get("workbench") ?? false,
      details: yPanelVisibility.get("details") ?? false,
      chat: yPanelVisibility.get("chat") ?? false,
      settings: yPanelVisibility.get("settings") ?? false,
    };
  }

  get selection(): HomeSelection | undefined {
    const yKits = this.yMap.get("selectedKits") as Y.Array<string>;
    if (!yKits || yKits.length === 0) return undefined;
    return {
      kits: yKits.toArray(),
    };
  }

  get sortColumn(): HomeSortColumn | undefined {
    return this.yMap.get("sortColumn") as HomeSortColumn | undefined;
  }

  get sortDirection(): HomeSortDirection | undefined {
    return this.yMap.get("sortDirection") as HomeSortDirection | undefined;
  }

  protected hash(state: HomeState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): HomeState {
    return {
      panelVisibility: this.panelVisibility,
      selection: this.selection,
      sortColumn: this.sortColumn,
      sortDirection: this.sortDirection,
    };
  }

  protected applySelectionDiff(selectionDiff: HomeSelectionDiff): void {
    let yKits = this.yMap.get("selectedKits") as Y.Array<string>;
    if (!yKits) {
      yKits = new Y.Array<string>();
      this.yMap.set("selectedKits", yKits);
    }
    if (selectionDiff.removed) {
      selectionDiff.removed.forEach((guid) => {
        const index = yKits.toArray().indexOf(guid);
        if (index !== -1) {
          yKits.delete(index, 1);
        }
      });
    }
    if (selectionDiff.added) {
      selectionDiff.added.forEach((guid) => {
        if (!yKits.toArray().includes(guid)) {
          yKits.push([guid]);
        }
      });
    }
  }

  protected inverseSelectionDiff(selection: HomeSelection, diff: HomeSelectionDiff): HomeSelectionDiff {
    const inverseDiff: HomeSelectionDiff = {};
    if (diff.added) {
      inverseDiff.removed = diff.added;
    }
    if (diff.removed) {
      inverseDiff.added = diff.removed;
    }
    return inverseDiff;
  }

  protected getSelection(): HomeSelection | undefined {
    return this.selection;
  }

  async executeCommand<T>(command: string, ...args: any[]): Promise<T> {
    let origin: string | undefined;
    let rest: any[];

    if (typeof args[0] === "string" && args[0].startsWith("semio.sketchpad.")) {
      origin = args[0];
      rest = args.slice(1);
    } else {
      origin = undefined;
      rest = args;
    }

    console.group(`[${origin || "unknown"}] Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) {
      console.groupEnd();
      throw new Error(`Command "${command}" not found in home store`);
    }
    const state = this.snapshot();
    const context: HomeCommandContext = { home: state, origin };
    const result = await callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
      this.recordEdit(result);
    }
    console.groupEnd();
    return result as T;
  }

  change(diff: HomeDiff): void {
    super.change({
      panelVisibility: diff.panelVisibility,
      selection: diff.selection,
    });
    this.transact(() => {
      if (diff.sortColumn !== undefined) {
        this.yMap.set("sortColumn", diff.sortColumn);
      }
      if (diff.sortDirection !== undefined) {
        this.yMap.set("sortDirection", diff.sortDirection);
      }
    });
  }
}

registerHomeStoreFactory((parent, yMap, transact) => new HomeStore(parent, yMap, transact));

function useHomeStore<T>(selector?: (store: HomeStore) => T): T | HomeStore {
  const store = useSketchpadStore();
  const homeStore = store.home();
  return selector ? selector(homeStore) : homeStore;
}

export function useHome<T>(selector?: (state: HomeState) => T): T | HomeState {
  const store = useHomeStore(identitySelector) as HomeStore;
  if (selector) {
    return useSyncDeep<HomeState>(store, selector as (value: HomeState) => HomeState) as T;
  }
  return useSyncDeep<HomeState>(store, identitySelector);
}

export function useHomePanelVisibility(): PanelVisibility {
  return useHome((s) => s.panelVisibility) as PanelVisibility;
}

export function useHomeCommands() {
  const store = useHomeStore() as HomeStore;
  return {
    togglePanel: (origin: string, panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    selectKit: (origin: string, Guid: Guid) => {
      const current = store.snapshot();
      store.change({
        selection: {
          removed: current.selection?.kits ?? [],
          added: [Guid],
        },
      });
    },
    addKitToSelection: (origin: string, Guid: Guid) => {
      store.change({
        selection: {
          added: [Guid],
        },
      });
    },
    removeKitFromSelection: (origin: string, Guid: Guid) => {
      store.change({
        selection: {
          removed: [Guid],
        },
      });
    },
    selectKits: (origin: string, kitIds: Guid[]) => {
      const current = store.snapshot();
      store.change({
        selection: {
          removed: current.selection?.kits ?? [],
          added: kitIds,
        },
      });
    },
    deselectAll: (origin: string) => {
      const current = store.snapshot();
      store.change({
        selection: {
          removed: current.selection?.kits ?? [],
        },
      });
    },
    setSortColumn: (origin: string, column: HomeSortColumn) => {
      store.change({
        sortColumn: column,
      });
    },
    setSortDirection: (origin: string, direction: HomeSortDirection) => {
      store.change({
        sortDirection: direction,
      });
    },
    toggleSort: (origin: string, column: HomeSortColumn) => {
      const current = store.snapshot();
      if (current.sortColumn === column) {
        store.change({
          sortDirection: current.sortDirection === "asc" ? "desc" : "asc",
        });
      } else {
        store.change({
          sortColumn: column,
          sortDirection: "asc",
        });
      }
    },
    execute: (origin: string, command: string, ...args: any[]) => store.executeCommand(command, origin, ...args),
  };
}

// #endregion Home Store

// #endregion Store

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
          <Input id="semio.sketchpad.app.home.panel.details.kit.version" value={kitShallow.version || ""} placeholder={useLabel("semio.sketchpad.app.kit.versionPlaceholder")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.home.panel.details.kit.description" value={kitShallow.description || ""} placeholder={useLabel("semio.sketchpad.app.kit.descriptionPlaceholder")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.home.panel.details.kit.icon" value={kitShallow.icon || ""} placeholder={useLabel("semio.sketchpad.app.kit.iconPlaceholder")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.home.panel.details.kit.image" value={kitShallow.image || ""} placeholder={useLabel("semio.sketchpad.app.kit.imagePlaceholder")} readOnly showLabel />
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
            placeholder={commonVersion === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.app.kit.versionPlaceholder")}
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
            placeholder={commonDescription === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.app.kit.descriptionPlaceholder")}
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
            placeholder={commonIcon === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.app.kit.iconPlaceholder")}
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
            placeholder={commonImage === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.app.kit.imagePlaceholder")}
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

const SettingsContent: FC<{ setTheme: (origin: string, theme: Theme) => void; setLayout: (origin: string, layout: Layout) => void; setExpertise: (origin: string, expertise: Expertise) => void; setMode: (origin: string, mode: Mode) => void }> = ({ setTheme, setLayout, setExpertise, setMode }) => {
  const theme = useTheme();
  const layout = useLayout();
  const expertise = useExpertise();
  const mode = useMode();

  return (
    <>
      <TreeItem>
        <TreeContent>
          <ToggleGroup
            id="semio.sketchpad.app.home.settings.theme"
            value={theme}
            onValueChange={(value: string) => setTheme("semio.sketchpad.app.home.settings.theme", value as Theme)}
            showLabel
            kind="single"
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
          <ToggleGroup
            id="semio.sketchpad.app.home.settings.layout"
            value={typeof layout === "object" ? "desktop" : layout}
            onValueChange={(value: string) => setLayout("semio.sketchpad.app.home.settings.layout", value as "desktop" | "tablet")}
            showLabel
            kind="single"
            items={[
              { value: "desktop", id: "semio.sketchpad.settings.layout.desktop", icon: <MousePointerIcon className="size-small" /> },
              { value: "tablet", id: "semio.sketchpad.settings.layout.tablet", icon: <HandIcon className="size-small" /> },
            ]}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <ToggleGroup
            id="semio.sketchpad.app.home.settings.expertise"
            value={expertise}
            onValueChange={(value: string) => setExpertise("semio.sketchpad.app.home.settings.expertise", value as Expertise)}
            showLabel
            kind="single"
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
            onValueChange={(value: string) => setMode("semio.sketchpad.app.home.settings.mode", value as Mode)}
            showLabel
            kind="single"
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

// #region App

type KitStoreKind = "temporary" | "local" | "remote";

type TableRow = {
  id: string;
  name: string;
  level: number;
  parentId?: string;
  hasChildren: boolean;
  isExpanded: boolean;
  type: KitStoreKind | "docs";
  updatedAt: string;
  createdAt: string;
  kit?: KitShallow;
  docsPath?: string;
  icon?: string;
};

const Home: FC = ({}) => {
  const navigate = useNavigate();
  const navigation = useNavigation();
  const [searchParams, setSearchParams] = useSearchParams();
  const kits = useKits();
  const getKitKind = useGetKitKind();
  const { createKit, navigateToKit, setTheme, setLayout, setExpertise, setMode } = useSketchpadCommands();

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
    removeSection("details", "semio.sketchpad.app.kit.title");
    removeSection("details", "semio.sketchpad.app.home.kits.multiple");

    // Only show section if something is selected
    if (hasKits) {
      const sectionId = hasSingleKit ? "semio.sketchpad.app.kit.title" : "semio.sketchpad.app.home.kits.multiple";
      addSection("details", {
        id: sectionId,
        order: 0,
        content: () => {
          return <KitSection />;
        },
      });
    }

    return () => {
      removeSection("details", "semio.sketchpad.app.kit.title");
      removeSection("details", "semio.sketchpad.app.home.kits.multiple");
    };
  }, [appType, addSection, removeSection, selection.length]);

  // Add chat panel section
  useEffect(() => {
    if (appType !== "home") return;

    addSection("chat", {
      id: "semio.sketchpad.app.home.chat",
      order: 0,
      content: () => {
        return <ChatPlaceholder />;
      },
    });

    return () => {
      removeSection("chat", "semio.sketchpad.app.home.chat");
    };
  }, [appType, addSection, removeSection]);

  // Add settings panel section
  useEffect(() => {
    if (appType !== "home") return;

    addSection("settings", {
      id: "semio.sketchpad.app.home.settings",
      order: 0,
      content: () => {
        return <SettingsContent setTheme={setTheme} setLayout={setLayout} setExpertise={setExpertise} setMode={setMode} />;
      },
    });

    return () => {
      removeSection("settings", "semio.sketchpad.app.home.settings");
    };
  }, [appType, addSection, removeSection, setTheme, setLayout, setExpertise, setMode]);

  // Get filters from search params (?kind=&name=&version=)
  const selectedKind = searchParams.get("kind") as KitStoreKind | null;
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
  const allConcepts = useMemo(() => {
    const conceptSet = new Set<string>();
    kits.forEach((kit) => {
      kit.concepts?.forEach((concept) => conceptSet.add(concept));
    });
    return Array.from(conceptSet).sort();
  }, [kits]);

  // Get selected concepts from search params
  const selectedConcepts = useMemo(() => {
    const conceptsParam = searchParams.get("concepts");
    return conceptsParam ? conceptsParam.split(",").filter(Boolean) : [];
  }, [searchParams]);

  const rows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const locale = i18n.language === "de" ? de : enUS;
    const formatDate = (date?: Date) => {
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

    const kitGroups = new Map<string, KitShallow[]>();

    kits.forEach((kit) => {
      const type = getKitKind(kit.guid) || "temporary";

      if (selectedKind && selectedKind !== type) return;
      if (searchQuery && !kit.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
      if (selectedName && kit.name !== selectedName) return;
      if (selectedVersion && (kit.version || "") !== selectedVersion) return;
      if (selectedConcepts.length > 0 && !kit.concepts?.some((c) => selectedConcepts.includes(c))) return;

      const key = kit.name;
      if (!kitGroups.has(key)) kitGroups.set(key, []);
      kitGroups.get(key)!.push(kit);
    });

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
  }, [kits, getKitKind, selectedKind, searchQuery, selectedName, selectedVersion, expandedRows, sortColumn, sortDirection]);

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

  const handleCreateKit = (type: KitStoreKind) => {
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

  const handleCreateVersion = (kitName: string, type: KitStoreKind) => {
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

  const toggleKind = (type: KitStoreKind) => {
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

  const toggleRow = (rowId: string) => {
    const newParams = new URLSearchParams(searchParams);
    const currentRows = newParams.getAll("e");

    if (currentRows.includes(rowId)) {
      // Remove row
      newParams.delete("e");
      currentRows.filter((r) => r !== rowId).forEach((r) => newParams.append("e", r));
    } else {
      // Add row
      newParams.append("e", rowId);
    }

    setSearchParams(newParams);
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
      <div
        className="flex flex-col h-full"
        onClick={(e: React.MouseEvent) => {
          if (e.target === e.currentTarget) {
            homeCommands.deselectAll("semio.sketchpad.app.home.canvas.table.deselect");
          }
        }}
      >
        <Strip
          direction="horizontal"
          id="semio.sketchpad.app.home.filter.strip"
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
            ...(selectedName ? [<Toggle pressed={true} onPressedChange={() => toggleName(selectedName)} id="semio.sketchpad.app.home.filter.name" icon={selectedName} />] : []),
            ...(selectedVersion !== null
              ? [<Toggle pressed={true} onPressedChange={() => toggleVersion(selectedVersion)} id="semio.sketchpad.app.home.filter.version" icon={selectedVersion || defaultVersionLabel} />]
              : []),
            ...(selectedKind && !selectedName && uniqueNames.length > 0
              ? uniqueNames.map((name) => (
                  <Toggle key={name} id={`semio.sketchpad.app.home.filter.name.${name}`} pressed={false} onPressedChange={() => toggleName(name)} icon={name} />
                ))
              : []),
            ...(selectedKind && selectedName && selectedVersion === null && uniqueVersions.length > 0
              ? uniqueVersions.map((version) => (
                  <Toggle key={version} id={`semio.sketchpad.app.home.filter.version.${version}`} pressed={false} onPressedChange={() => toggleVersion(version)} icon={version || <span className="italic opacity-50">{defaultVersionLabel}</span>} />
                ))
              : []),
            <Input key="search" id="semio.sketchpad.app.home.search" className="flex-1 min-w-[160px]" placeholder={useLabel("semio.sketchpad.app.home.searchPlaceholder")} value={searchQuery} onChange={(e) => handleSearchChange(e.target.value)} />,
          ]}
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
              return (
                <div
                  key={row.id}
                  className={`border-b p-single cursor-selectable h-large ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`}
                  role="button"
                  tabIndex={0}
                  onClick={(e) => {
                    if (isDocsRow && row.docsPath) {
                      navigate(`/${row.docsPath}`);
                    } else if (row.kit) {
                      handleRowClick(row.kit.guid, e);
                    }
                  }}
                  onDoubleClick={() => {
                    if (isDocsRow && row.docsPath) {
                      navigate(`/${row.docsPath}`);
                    } else if (row.kit) {
                      navigateToKit(row.kit.guid);
                    }
                  }}
                >
                  <div className="flex items-center gap-single justify-between" style={{ paddingLeft: `calc(${row.level} * 16 * var(--spacing))` }} onClick={(e) => e.stopPropagation()}>
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
                    </div>
                    <div className="flex items-center gap-single shrink-0">
                      {row.level === 0 && row.type !== "docs" && (
                        <Action
                          level="base"
                          onClick={(e) => {
                            e.stopPropagation();
                            handleCreateVersion(row.name, row.type as KitStoreKind);
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
    );
  }

  return (
    <>
      <HomeAppFooter />
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
            <Strip
              direction="horizontal"
              id="semio.sketchpad.app.home.filter.strip"
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
                ...(selectedName ? [<Toggle id={`semio.sketchpad.app.home.filter.name.${selectedName}`} pressed={true} onPressedChange={() => toggleName(selectedName)} icon={selectedName} />] : []),
                ...(selectedVersion !== null
                  ? [<Toggle id={`semio.sketchpad.app.home.filter.version.${selectedVersion}`} pressed={true} onPressedChange={() => toggleVersion(selectedVersion)} icon={selectedVersion || defaultVersionLabel} />]
                  : []),
                ...(selectedKind && !selectedName && uniqueNames.length > 0
                  ? uniqueNames.map((name) => <Toggle key={name} id={`semio.sketchpad.app.home.filter.name.${name}`} pressed={false} onPressedChange={() => toggleName(name)} icon={name} />)
                  : []),
                ...(selectedKind && selectedName && selectedVersion === null && uniqueVersions.length > 0
                  ? uniqueVersions.map((version) => (
                      <Toggle key={version} id={`semio.sketchpad.app.home.filter.version.${version}`} pressed={false} onPressedChange={() => toggleVersion(version)} icon={version || defaultVersionLabel} />
                    ))
                  : []),
                <Input
                  key="search"
                  id="semio.sketchpad.app.home.search"
                  className="flex-1 min-w-[200px]"
                  placeholder={useLabel("semio.sketchpad.app.home.searchPlaceholder")}
                  value={searchQuery}
                  onChange={(e) => handleSearchChange(e.target.value)}
                />,
              ]}
            />
            {/* Concept Filter */}
            <ConceptFilter allConcepts={allConcepts} />
            <Table<TableRow>
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
                    <div className="flex items-center gap-single justify-between" style={{ paddingLeft: `calc(${row.level} * 24 * var(--spacing))` }} onClick={(e) => e.stopPropagation()}>
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
                          <span className="size-tiny shrink-0" />
                        )}
                        <TableAvatar name={row.name} icon={row.type === "docs" ? row.icon : row.kit?.icon} />
                        <span className="text-left flex-1 min-w-0 truncate">{row.name}</span>
                      </div>
                      <div className="flex items-center gap-single shrink-0">
                        {row.level === 0 && row.type !== "docs" && (
                          <Action
                            level="base"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleCreateVersion(row.name, row.type as KitStoreKind);
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
              headerClassName="sticky top-0 border-b h-large"
              hierarchical={true}
            />
          </div>
        </Window>
      </Canvas>
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
