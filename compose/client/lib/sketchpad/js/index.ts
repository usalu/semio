// #region 🧲Header
// 2024-2026 Ueli Saluz <ueli@semio-tech.com>
// Render-agnostic sketchpad product: {@link Platform} apps, {@link Component} snapshots, controller-owned {@link Store}s.
// #endregion 🧲Header

//#region 🔌Adapters
import type { Design, Kit, Piece, Session, SetResult, Type } from "@semio-tech/compose-js";
import { Design as JsDesign, Kit as JsKitEntity, Piece as JsPiece, Session as ComposeSession } from "@semio-tech/compose-js";
import {
	fetchComposeFileSystemChildren,
	type ComposeFileSystemChildRef,
	type ComposeFileSystemParentRef,
} from "@semio-tech/compose-react";
import { gunzipSync } from "fflate";
import type { Store as JsKitStore } from "@semio-tech/compose-js";
import {
	CommandBus,
	Component,
	Controller,
	ObservableCell,
	Panel,
	Platform,
	PluginHost,
	Store,
	PlatformTopologyStore,
	PlatformTopologyPayload,
	platformTopologyStoreId,
	PLATFORM_TOPOLOGY_STORE_PREFIX,
	VirtualFileSystemController,
	buildPanelWindowBody,
	buildPuzzle2dWindowBody,
	buildPuzzle5dWindowBody,
	buildVirtualFileSystemWindowBody,
	virtualFileSystemSurfaceId,
	virtualFileSystemScopeKey,
	virtualFileSystemDescriptorValues,
	type VirtualFileSystemDescriptorValueModel,
	type VirtualFileSystemModel,
	type VirtualFileSystemNodeRecord,
	type VirtualFileSystemScope,
	type VirtualFileSystemSchemaModel,
	type VirtualFileSystemVisibleNode,
	createDefaultLayout,
	createTabStackLayout,
	registerPlatformComponent,
	registerSidePanelBody,
	registerWindowBody,
	type ComponentKind,
	type PanelModel,
	type PlatformSpec,
	type PluginManifest,
	type PluginModule,
	type Puzzle2dModel,
	type Puzzle5dModel,
	type SideTabSpec,
	type UiNode,
	type UiSectionNode,
	type UiTreeItemNode,
	type UiTreeNode,
	type WindowBodyViewContext,
	uiDeclarativeSectionsToTree,
	getPlatformControllerById,
	WindowKindRuntime,
	AppRuntime,
} from "@semio-tech/framework-platform-core";
import {
	FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
	FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
	FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
	FRAMEWORK_PANEL_TAB_INSPECTION_ID,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY,
	type CommandDescriptor,
	type NavigationDestination,
	type NavigationLevel,
	type SearchItemSpec,
} from "@semio-tech/framework-core";
import {
	type RelationshipKind,
	type WiresFixtureV1,
	wiresFixtureBoard,
	wiresKitInstanceId,
} from "@semio-tech/reasoning-mindmap-wires-react";
//#endregion 🔌Adapters

//#region 🪁ComposeUiI18n
import {
	decodeIcon,
	encodeIcon,
	iconSvgMarkup,
	registerUiTranslationBundles,
	setControlLabelIdResolver,
	type UiLabelValue,
	type UiLocale,
	type UiToolbarParentCategory,
} from "@semio-tech/ui-react";

/** @emoji 🪁 Sketchpad toolbar parent labels keyed by {@link UiToolbarParentCategory}. */
type ComposeSketchpadToolbarParentEntries = { readonly [K in UiToolbarParentCategory]: UiLabelValue };

export const composeSketchpadToolbarParentDe: ComposeSketchpadToolbarParentEntries = {
	history: { label: { normal: "Verlauf", beginner: "Verlauf" } },
	hand: { label: { normal: "Hand", beginner: "Hand" } },
	selection: { label: { normal: "Auswahl", beginner: "Auswahl" } },
	lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
	filter: { label: { normal: "Filter", beginner: "Filter" } },
	open: { label: { normal: "Öffnen", beginner: "Öffnen" } },
	save: { label: { normal: "Speichern", beginner: "Speichern" } },
	transfer: { label: { normal: "Transfer", beginner: "Transfer" } },
	transform: { label: { normal: "Transformieren", beginner: "Transformieren" } },
	create: { label: { normal: "Erstellen", beginner: "Erstellen" } },
	view: { label: { normal: "Ansicht", beginner: "Ansicht" } },
	actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
	settings: { label: { normal: "Einstellungen", beginner: "Einstellungen" } },
};

export const composeSketchpadToolbarParentEn: ComposeSketchpadToolbarParentEntries = {
	history: { label: { normal: "History", beginner: "History" } },
	hand: { label: { normal: "Hand", beginner: "Hand" } },
	selection: { label: { normal: "Selection", beginner: "Selection" } },
	lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
	filter: { label: { normal: "Filter", beginner: "Filter" } },
	open: { label: { normal: "Open", beginner: "Open" } },
	save: { label: { normal: "Save", beginner: "Save" } },
	transfer: { label: { normal: "Transfer", beginner: "Transfer" } },
	transform: { label: { normal: "Transform", beginner: "Transform" } },
	create: { label: { normal: "Create", beginner: "Create" } },
	view: { label: { normal: "View", beginner: "View" } },
	actions: { label: { normal: "Actions", beginner: "Actions" } },
	settings: { label: { normal: "Settings", beginner: "Settings" } },
};

/** @emoji 🪁 Compose sketchpad i18n keys resolved from shell control ids. */
export type ComposeSketchpadControlTranslationKey =
	| "compose.sketchpad.navbar.back"
	| "compose.sketchpad.navbar.forward"
	| "compose.sketchpad.navbar.up"
	| "compose.sketchpad.navbar.search.open"
	| "compose.sketchpad.navbar.find.open"
	| `compose.sketchpad.navbar.panelToggle.${string}`
	| `compose.sketchpad.toolbar.parent.${UiToolbarParentCategory}`;

type ComposeSketchpadTranslationTree = {
	readonly compose: {
		readonly sketchpad: {
			readonly toolbar: {
				readonly parent: ComposeSketchpadToolbarParentEntries;
			};
		};
	};
};

function applyComposeSketchpadToolbarParentEntries(
	bundles: Record<UiLocale, { translation: ComposeSketchpadTranslationTree }>,
): void {
	bundles.de.translation.compose.sketchpad.toolbar.parent = composeSketchpadToolbarParentDe;
	bundles.en.translation.compose.sketchpad.toolbar.parent = composeSketchpadToolbarParentEn;
}

const composeSketchpadTranslationBundles = {
  de: {
    translation: JSON.parse(String.raw`{
  "compose": {
    "label": {
      "normal": "",
      "beginner": ""
    },
    "file": {
      "name": "Name",
      "size": "Groesse",
      "created": "Erstellt",
      "updated": "Aktualisiert"
    },
    "folder": {
      "created": "Erstellt",
      "updated": "Aktualisiert"
    },
    "sketchpad": {
      "label": {
        "normal": "",
        "beginner": ""
      },
      "navbar": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "back": {
          "label": {
            "normal": "Zurueck",
            "beginner": "Klicken, um zurueckzugehen, halten um Historie zu sehen"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Links"
        },
        "forward": {
          "label": {
            "normal": "Vorwaerts",
            "beginner": "Klicken, um vorwaerts zu gehen, halten um Historie zu sehen"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Rechts"
        },
        "up": {
          "label": {
            "normal": "Eine Ebene hoch",
            "beginner": "Klicken, um eine Ebene hoeher in der Navigationshierarchie zu gehen"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Oben"
        },
        "kits": {
          "label": {
            "normal": "Kits",
            "beginner": "Klicken, um alle Kits zu sehen"
          }
        },
        "navigationButtons": {
          "label": {
            "normal": "Navigation",
            "beginner": "Navigationsknoepfe"
          }
        },
        "docs": {
          "label": {
            "normal": "Dokumentation",
            "beginner": "Klicken, um Dokumentation anzuzeigen"
          },
          "hotkey": "Ctrl+Shift+D"
        },
        "search": {
          "label": {
            "normal": "Suche",
            "beginner": "Nach Inhalten suchen"
          },
          "open": {
            "label": {
              "normal": "Suche",
              "beginner": "Klicken, um Suche zu oeffnen und schnell zu jedem Element zu navigieren"
            },
            "manual": "navigation#search",
            "tutorial": "getting-started/intro#search",
            "hotkey": "Ctrl+K"
          },
          "close": {
            "label": {
              "normal": "Suche schliessen",
              "beginner": "Klicken, um den Suchdialog zu schliessen"
            },
            "manual": "navigation#search",
            "tutorial": "getting-started/intro#search",
            "hotkey": "Escape"
          },
          "title": {
            "label": {
              "normal": "Suche",
              "beginner": "Suche"
            }
          },
          "description": {
            "label": {
              "normal": "Nach Kits, Entwuerfen, Typen und mehr suchen",
              "beginner": "Nach Kits, Entwuerfen, Typen und mehr suchen"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Suchen...",
              "beginner": "Suchen..."
            }
          },
          "noResults": {
            "label": {
              "normal": "Keine Ergebnisse gefunden",
              "beginner": "Keine Ergebnisse gefunden"
            }
          }
        },
        "find": {
          "label": {
            "normal": "Finden",
            "beginner": "Im aktuellen Kontext finden"
          },
          "open": {
            "label": {
              "normal": "Finden",
              "beginner": "Klicken, um Elemente in der aktuellen Ansicht zu finden"
            },
            "hotkey": "Ctrl+F"
          },
          "close": {
            "label": {
              "normal": "Finden schliessen",
              "beginner": "Klicken, um den Finden-Dialog zu schliessen"
            },
            "hotkey": "Escape"
          },
          "title": {
            "label": {
              "normal": "Finden",
              "beginner": "Finden"
            }
          },
          "description": {
            "label": {
              "normal": "Elemente in dieser Ansicht finden",
              "beginner": "Elemente in dieser Ansicht finden"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Finden...",
              "beginner": "Finden..."
            }
          },
          "noResults": {
            "label": {
              "normal": "Keine Ergebnisse gefunden",
              "beginner": "Keine Ergebnisse gefunden"
            }
          }
        },
        "focus": {
          "label": {
            "normal": "Fokusmodus",
            "beginner": "Fokusmodus umschalten um Ablenkungen auszublenden"
          },
          "open": {
            "label": {
              "normal": "Fokus",
              "beginner": "Klicken, um in den Fokusmodus zu wechseln und Ablenkungen auszublenden"
            },
            "manual": "navigation#focus",
            "tutorial": "getting-started/intro#focus",
            "hotkey": "Ctrl+Shift+F"
          },
          "close": {
            "label": {
              "normal": "Fokus verlassen",
              "beginner": "Klicken, um den Fokusmodus zu verlassen und alle UI-Elemente anzuzeigen"
            },
            "manual": "navigation#focus",
            "tutorial": "getting-started/intro#focus",
            "hotkey": "Escape"
          },
          "input": {
            "label": {
              "normal": "Fokus-Eingabe",
              "beginner": "Tippen, um nach einem Element zum Fokussieren zu suchen"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Nach einem Element suchen...",
              "beginner": "Nach einem Element suchen..."
            }
          },
          "title": {
            "label": {
              "normal": "Fokus",
              "beginner": "Fokus"
            }
          },
          "description": {
            "label": {
              "normal": "Auf ein Element in der aktuellen Ansicht fokussieren",
              "beginner": "Auf ein Element in der aktuellen Ansicht fokussieren"
            }
          },
          "other": {
            "label": {
              "normal": "Andere",
              "beginner": "Andere"
            }
          }
        },
        "copyJsonToClipboard": {
          "label": {
            "normal": "JSON kopieren",
            "beginner": "Den aktuellen Sketchpad-JSON-Status in die Zwischenablage kopieren"
          },
          "hotkey": "Ctrl+Shift+J"
        },
        "breadcrumb": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "designs": {
            "label": {
              "normal": "Entwuerfe",
              "beginner": "Entwuerfe"
            }
          },
          "types": {
            "label": {
              "normal": "Typen",
              "beginner": "Typen"
            }
          },
          "qualities": {
            "label": {
              "normal": "Qualitaeten",
              "beginner": "Qualitaeten"
            }
          },
          "temporary": {
            "label": {
              "normal": "Temporär",
              "beginner": "Temporär"
            }
          },
          "local": {
            "label": {
              "normal": "Lokal",
              "beginner": "Lokal"
            }
          },
          "remote": {
            "label": {
              "normal": "Remote",
              "beginner": "Remote"
            }
          },
          "files": {
            "label": {
              "normal": "Files",
              "beginner": "Files"
            }
          },
          "authors": {
            "label": {
              "normal": "Autoren",
              "beginner": "Autoren"
            }
          }
        },
        "tutorials": {
          "label": {
            "normal": "Tutorials",
            "beginner": "Tutorials"
          }
        },
        "tutorial": {
          "controls": {
            "stop": {
              "label": {
                "normal": "Tutorial beenden",
                "beginner": "Klicken, um das aktuelle Tutorial zu beenden"
              }
            },
            "previous": {
              "label": {
                "normal": "Vorheriger Schritt",
                "beginner": "Zum vorherigen Schritt im Tutorial gehen"
              }
            },
            "playPause": {
              "label": {
                "normal": "Abspielen/Pause",
                "beginner": "Tutorial abspielen oder pausieren"
              }
            },
            "next": {
              "label": {
                "normal": "Nächster Schritt",
                "beginner": "Zum nächsten Schritt im Tutorial gehen"
              }
            }
          }
        },
        "recording": {
          "controls": {
            "playPause": {
              "label": {
                "normal": "Aufnahme abspielen/pausieren",
                "beginner": "Aufnahme abspielen oder pausieren"
              }
            },
            "stop": {
              "label": {
                "normal": "Aufnahme beenden",
                "beginner": "Aufnahme beenden und speichern"
              }
            }
          }
        },
        "createKit": {
          "label": {
            "normal": "Kit erstellen",
            "beginner": "Klicken, um ein neues Kit zu erstellen"
          }
        },
        "createDesign": {
          "label": {
            "normal": "Entwurf erstellen",
            "beginner": "Klicken, um einen neuen Entwurf zu erstellen"
          }
        },
        "createChild": {
          "label": {
            "normal": "Kind erstellen",
            "beginner": "Klicken, um ein Kind-Artefakt zu erstellen"
          }
        },
        "createType": {
          "label": {
            "normal": "Typ erstellen",
            "beginner": "Klicken, um einen neuen Typ zu erstellen"
          }
        },
        "createVersion": {
          "label": {
            "normal": "Version erstellen",
            "beginner": "Klicken, um eine neue Version zu erstellen"
          }
        },
        "searchInput": {
          "label": {
            "normal": "Such-Eingabe",
            "beginner": "Tippen, um nach Elementen zu suchen"
          }
        },
        "panelToggle": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "workbench": {
            "label": {
              "normal": "Arbeitsbereich",
              "beginner": "Das Workbench-Panel links ein- oder ausblenden"
            },
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "hud": {
            "label": {
              "normal": "HUD umschalten",
              "beginner": "Das HUD-Panel in der Mitte ein- oder ausblenden"
            },
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "right": {
            "label": {
              "normal": "Rechtes Panel umschalten",
              "beginner": "Das rechte Panel fuer Details und Einstellungen ein- oder ausblenden"
            }
          },
          "tools": {
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "toolbar": {
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "stats": {
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "details": {
            "label": {
              "normal": "Details",
              "beginner": "Das Details-Panel rechts ein- oder ausblenden"
            },
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "chat": {
            "label": {
              "normal": "Chat umschalten",
              "beginner": "Chat-Panel umschalten"
            },
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen umschalten",
              "beginner": "Einstellungen-Panel umschalten"
            },
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "leftSidePanel": {
            "label": {
              "normal": "Linkes Panel umschalten",
              "beginner": "Das linke Seitenfeld mit Workbench-Tabs umschalten"
            }
          },
          "rightSidePanel": {
            "label": {
              "normal": "Rechtes Panel umschalten",
              "beginner": "Das rechte Seitenfeld mit Detail-Tabs umschalten"
            }
          },
          "hudPanel": {
            "label": {
              "normal": "HUD-Panel umschalten",
              "beginner": "Das zentrale HUD-Panel umschalten"
            }
          }
        },
        "home": {
          "label": {
            "normal": "Home",
            "beginner": "Home"
          }
        },
        "kitName": {
          "label": {
            "normal": "Kit Name",
            "beginner": "Kit Name"
          }
        },
        "kitVersion": {
          "label": {
            "normal": "Kit Version",
            "beginner": "Kit Version"
          }
        },
        "name": {
          "label": {
            "normal": "Name",
            "beginner": "Name"
          }
        },
        "design": {
          "label": {
            "normal": "Design",
            "beginner": "Design"
          }
        },
        "type": {
          "label": {
            "normal": "Typ",
            "beginner": "Typ"
          }
        },
        "quality": {
          "label": {
            "normal": "Quality",
            "beginner": "Quality"
          }
        },
        "navigation": {
          "label": {
            "normal": "Navigation",
            "beginner": "Navigation"
          }
        },
        "panelToggles": {
          "label": {
            "normal": "Panel-Umschalter",
            "beginner": "Panel-Umschalter"
          }
        },
        "fullscreenToggle": {
          "label": {
            "normal": "Fullscreen Toggle",
            "beginner": "Fullscreen Toggle"
          }
        }
      },
      "panel": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "chat": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "placeholder": {
            "label": {
              "normal": "Fragen Sie etwas...",
              "beginner": "Fragen Sie etwas..."
            }
          }
        },
        "settings": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "theme": {
            "label": {
              "normal": "Design",
              "beginner": "Wählen Sie das Farbschema für die Anwendung"
            },
            "dark": {
              "label": {
                "normal": "Dunkel",
                "beginner": "Dunkles Farbschema verwenden"
              }
            },
            "light": {
              "label": {
                "normal": "Hell",
                "beginner": "Helles Farbschema verwenden"
              }
            },
            "system": {
              "label": {
                "normal": "System",
                "beginner": "Systemdesign-Einstellung folgen"
              }
            }
          },
          "device": {
            "label": {
              "normal": "Geraet",
              "beginner": "Geraetemodus fuer die Interaktion waehlen"
            },
            "desktop": {
              "label": {
                "normal": "Desktop",
                "beginner": "Optimiertes Layout für Desktop-Computer"
              }
            },
            "tablet": {
              "label": {
                "normal": "Tablet",
                "beginner": "Optimiertes Layout für Tablets"
              }
            },
            "mobile": {
              "label": {
                "normal": "Mobil",
                "beginner": "Optimiertes Layout für mobile Geräte"
              }
            }
          },
          "mode": {
            "label": {
              "normal": "Modus",
              "beginner": "Wählen Sie den Benutzeroberflächenmodus: Experte (minimale Tooltips), Normal (Standard) oder Anfänger (detaillierte Hilfe)"
            },
            "dev": {
              "label": {
                "normal": "Entwickler",
                "beginner": "Entwicklermodus mit erweiterten Werkzeugen und Debugging-Funktionen"
              }
            },
            "user": {
              "label": {
                "normal": "Benutzer",
                "beginner": "Standardbenutzermodus für reguläre Operationen"
              }
            }
          },
          "expertise": {
            "label": {
              "normal": "Erfahrung",
              "beginner": "Wählen Sie Ihre Erfahrungsstufe um die Komplexität der Oberfläche anzupassen"
            },
            "beginner": {
              "label": {
                "normal": "Anfänger",
                "beginner": "Detaillierte Erklärungen und Tutorials anzeigen"
              }
            },
            "normal": {
              "label": {
                "normal": "Normal",
                "beginner": "Standard-Tooltips und Hilfe anzeigen"
              }
            },
            "expert": {
              "label": {
                "normal": "Experte",
                "beginner": "Minimale Tooltips für erfahrene Benutzer"
              }
            }
          }
        }
      },
      "common": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "selectVariant": "Variante auswaehlen...",
        "selectView": "Ansicht auswaehlen...",
        "search": {
          "label": {
            "normal": "Suche",
            "beginner": "Suche"
          }
        },
        "mixedValues": {
          "label": {
            "normal": "Gemischte Werte",
            "beginner": "Gemischte Werte"
          }
        },
        "selectDesign": {
          "label": {
            "normal": "Entwurf auswaehlen",
            "beginner": "Entwurf auswaehlen"
          }
        },
        "selectType": {
          "label": {
            "normal": "Typ auswaehlen",
            "beginner": "Typ auswaehlen"
          }
        },
        "no": {
          "label": {
            "normal": "Nein",
            "beginner": "Nein"
          }
        },
        "yes": {
          "label": {
            "normal": "Ja",
            "beginner": "Ja"
          }
        },
        "add": {
          "label": {
            "normal": "Hinzufügen",
            "beginner": "Hinzufügen"
          }
        },
        "remove": {
          "label": {
            "normal": "Entfernen",
            "beginner": "Entfernen"
          }
        },
        "addChild": {
          "label": {
            "normal": "Kind hinzufügen",
            "beginner": "Kind hinzufügen"
          }
        },
        "duplicateType": {
          "label": {
            "normal": "Typ duplizieren (Hover)",
            "beginner": "Typ duplizieren (Hover)"
          }
        },
        "addType": {
          "label": {
            "normal": "Typ hinzufügen",
            "beginner": "Typ hinzufügen"
          }
        },
        "addDesign": {
          "label": {
            "normal": "Entwurf hinzufügen",
            "beginner": "Entwurf hinzufügen"
          }
        },
        "settings": {
          "label": {
            "normal": "Sketchpad",
            "beginner": "Globale Sketchpad-Einstellungen"
          },
          "theme": {
            "label": {
              "normal": "Design",
              "beginner": "Farbschema wählen"
            }
          },
          "layout": {
            "label": {
              "normal": "Layout",
              "beginner": "Layoutmodus wählen"
            }
          },
          "mode": {
            "label": {
              "normal": "Modus",
              "beginner": "Oberflächenmodus wählen"
            }
          },
          "expertise": {
            "label": {
              "normal": "Erfahrungsstufe",
              "beginner": "Erfahrungsstufe wählen"
            }
          }
        }
      },
      "footer": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "feedback": {
          "label": {
            "normal": "Feedback",
            "beginner": "Feedback senden, um compose zu verbessern"
          }
        }
      },
      "app": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "home": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "title": "Home",
          "fileInput": {
            "label": {
              "normal": "Kit-Datei waehlen",
              "beginner": "Eine .zip-Kit-Datei von Ihrem Geraet auswaehlen"
            }
          },
          "searchPlaceholder": {
            "label": {
              "normal": "Kits suchen...",
              "beginner": "Kits suchen..."
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Name"
            }
          },
          "kind": {
            "label": {
              "normal": "Art",
              "beginner": "Art"
            }
          },
          "lastUpdated": {
            "label": {
              "normal": "Zuletzt aktualisiert",
              "beginner": "Zuletzt aktualisiert"
            }
          },
          "created": {
            "label": {
              "normal": "Erstellt",
              "beginner": "Erstellt"
            }
          },
          "filter": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "kind": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "show": {
                "label": {
                  "normal": "Nach Art filtern",
                  "beginner": "Kits nach ihrem Speichertyp filtern (temporaer, lokal oder remote)"
                },
                "hotkey": "Ctrl+K"
              },
              "create": {
                "label": {
                  "normal": "Neues Kit erstellen",
                  "beginner": "Ein neues Kit dieses Typs erstellen"
                },
                "hotkey": "Ctrl+Shift+K"
              },
              "temporary": {
                "label": {
                  "normal": "Temporaere Kits anzeigen",
                  "beginner": "Kits anzeigen, die im Browser-Speicher gespeichert sind (gehen beim Neuladen verloren)"
                },
                "hotkey": "Ctrl+1"
              },
              "createTemporary": {
                "label": {
                  "normal": "Temporaeres Kit erstellen",
                  "beginner": "Ein neues temporaeres Kit erstellen, das im Browser-Speicher gespeichert wird"
                },
                "hotkey": "Ctrl+Shift+1"
              },
              "local": {
                "label": {
                  "normal": "Lokale Kits anzeigen",
                  "beginner": "Kits anzeigen, die lokal auf Ihrem Geraet gespeichert sind"
                },
                "hotkey": "Ctrl+2"
              },
              "createLocal": {
                "label": {
                  "normal": "Lokales Kit erstellen",
                  "beginner": "Ein neues Kit erstellen, das lokal auf Ihrem Geraet gespeichert wird"
                },
                "hotkey": "Ctrl+Shift+2"
              },
              "remote": {
                "label": {
                  "normal": "Remote-Kits anzeigen",
                  "beginner": "Kits anzeigen, die mit Remote-Speicher synchronisiert sind"
                },
                "hotkey": "Ctrl+3"
              },
              "createRemote": {
                "label": {
                  "normal": "Remote-Kit erstellen",
                  "beginner": "Ein neues Kit erstellen, das mit Remote-Speicher synchronisiert wird"
                },
                "hotkey": "Ctrl+Shift+3"
              }
            },
            "name": {
              "label": {
                "normal": "Nach Name filtern",
                "beginner": "Kits nach diesem spezifischen Namen filtern"
              },
              "hotkey": "Ctrl+N"
            },
            "version": {
              "label": {
                "normal": "Nach Version filtern",
                "beginner": "Kits nach dieser spezifischen Version filtern"
              },
              "hotkey": "Ctrl+V"
            },
            "band": {
              "label": {
                "normal": "Filterleiste",
                "beginner": "Filterleiste ein- oder ausblenden"
              },
              "hotkey": "Ctrl+F"
            }
          },
          "search": {
            "label": {
              "normal": "Suche",
              "beginner": "Nach Kits suchen"
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Home-Einstellungen"
            },
            "theme": {
              "label": {
                "normal": "Design",
                "beginner": "Waehlen Sie das Farbschema fuer die Anwendung"
              }
            },
            "language": {
              "label": {
                "normal": "Sprache",
                "beginner": "Waehlen Sie die Sprache fuer die Anwendungsoberflaeche"
              },
              "placeholder": {
                "label": {
                  "normal": "Sprache waehlen...",
                  "beginner": "Waehlen Sie die Sprache, in der die Anwendung angezeigt wird"
                }
              }
            },
            "mode": {
              "label": {
                "normal": "Modus",
                "beginner": "Waehlen Sie den Benutzeroberflaechen-Modus"
              }
            },
            "expertise": {
              "label": {
                "normal": "Erfahrung",
                "beginner": "Waehlen Sie Ihr Erfahrungsniveau"
              }
            },
            "device": {
              "label": {
                "normal": "Geraet",
                "beginner": "Waehlen Sie das Eingabegeraet"
              }
            },
            "layout": {
              "label": {
                "normal": "Layout",
                "beginner": "Waehlen Sie das Layout fuer die Kit-Uebersicht"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagramm",
                "beginner": "Konfigurieren Sie das kraftgerichtete Diagramm-Layout"
              },
              "chargeStrength": {
                "label": {
                  "normal": "Ladungsstaerke",
                  "beginner": "Steuert, wie stark Knoten sich abstossen. Negativere Werte druecken Knoten weiter auseinander."
                }
              },
              "linkDistance": {
                "label": {
                  "normal": "Verbindungsabstand",
                  "beginner": "Der Zielabstand zwischen verbundenen Knoten. Groessere Werte verteilen das Diagramm."
                }
              },
              "collideRadius": {
                "label": {
                  "normal": "Kollisionsradius",
                  "beginner": "Der Mindestabstand zwischen Knotenzentren zur Vermeidung von Ueberlappung."
                }
              },
              "centerStrength": {
                "label": {
                  "normal": "Zentrierungsstaerke",
                  "beginner": "Wie stark Knoten zur Mitte des Diagramms gezogen werden."
                }
              }
            }
          },
          "canvas": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "table": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "createKit": {
                "label": {
                  "normal": "Kit erstellen",
                  "beginner": "Neues Kit aus der Home-Tabelle anlegen"
                }
              },
              "createVersion": {
                "label": {
                  "normal": "Version erstellen",
                  "beginner": "Neue Kit-Version aus der Home-Tabelle anlegen"
                }
              },
              "hover": {
                "label": {
                  "normal": "Kit hervorheben",
                  "beginner": "Ueber eine Kit-Zeile fahren, um sie im Diagramm hervorzuheben"
                }
              },
              "toggleSort": {
                "label": {
                  "normal": "Sortierung umschalten",
                  "beginner": "Sortierrichtung fuer diese Spalte aendern"
                }
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "kit": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "name": {
                  "label": {
                    "normal": "Name",
                    "beginner": "Der Name des Kits"
                  }
                },
                "version": {
                  "label": {
                    "normal": "Version",
                    "beginner": "Die Version des Kits"
                  }
                },
                "description": {
                  "label": {
                    "normal": "Beschreibung",
                    "beginner": "Eine Beschreibung des Kits"
                  }
                },
                "icon": {
                  "label": {
                    "normal": "Symbol",
                    "beginner": "Das Symbol des Kits"
                  }
                },
                "image": {
                  "label": {
                    "normal": "Bild",
                    "beginner": "Das Vorschaubild des Kits"
                  }
                }
              },
              "kits": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "name": {
                  "label": {
                    "normal": "Name",
                    "beginner": "Der Name der ausgewählten Kits"
                  }
                },
                "version": {
                  "label": {
                    "normal": "Version",
                    "beginner": "Die Version der ausgewählten Kits"
                  }
                },
                "description": {
                  "label": {
                    "normal": "Beschreibung",
                    "beginner": "Eine Beschreibung der ausgewählten Kits"
                  }
                },
                "icon": {
                  "label": {
                    "normal": "Symbol",
                    "beginner": "Das Symbol der ausgewählten Kits"
                  }
                },
                "image": {
                  "label": {
                    "normal": "Bild",
                    "beginner": "Das Vorschaubild der ausgewählten Kits"
                  }
                }
              }
            }
          },
          "dropzone": {
            "label": {
              "normal": "Zip-Datei zum Importieren eines Kits ablegen",
              "beginner": "Legen Sie eine Zip-Datei hier ab, um sie als neues Kit zu importieren"
            },
            "description": {
              "normal": "Nur Kits mit .compose-Ordner können importiert werden",
              "beginner": "Die Zip-Datei muss einen .compose-Ordner mit kit.db enthalten, um als Kit importiert zu werden."
            }
          },
          "noKits": {
            "label": {
              "normal": "Keine Kits",
              "beginner": "Keine Kits"
            }
          },
          "sortByName": {
            "label": {
              "normal": "Sort By Name",
              "beginner": "Sort By Name"
            }
          },
          "toggleRow": {
            "label": {
              "normal": "Zeile umschalten",
              "beginner": "Zeile umschalten"
            }
          },
          "createVersion": {
            "label": {
              "normal": "Version erstellen",
              "beginner": "Version erstellen"
            }
          },
          "hideKind": {
            "label": {
              "normal": "Art ausblenden",
              "beginner": "Art ausblenden"
            }
          },
          "showTemporary": {
            "label": {
              "normal": "Temporär anzeigen",
              "beginner": "Temporär anzeigen"
            }
          },
          "showLocal": {
            "label": {
              "normal": "Lokal anzeigen",
              "beginner": "Lokal anzeigen"
            }
          },
          "showRemote": {
            "label": {
              "normal": "Remote anzeigen",
              "beginner": "Remote anzeigen"
            }
          },
          "sortByType": {
            "label": {
              "normal": "Sort By Type",
              "beginner": "Sort By Type"
            }
          },
          "sortByUpdatedAt": {
            "label": {
              "normal": "Sort By Updated At",
              "beginner": "Sort By Updated At"
            }
          },
          "sortByCreatedAt": {
            "label": {
              "normal": "Sort By Created At",
              "beginner": "Sort By Created At"
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Home-Chat"
            }
          },
          "createKit": {
            "label": {
              "normal": "Kit erstellen",
              "beginner": "Kit erstellen"
            }
          },
          "createTemporary": {
            "label": {
              "normal": "Temporär erstellen",
              "beginner": "Temporär erstellen"
            }
          },
          "createLocal": {
            "label": {
              "normal": "Lokal erstellen",
              "beginner": "Lokal erstellen"
            }
          },
          "createRemote": {
            "label": {
              "normal": "Remote erstellen",
              "beginner": "Remote erstellen"
            }
          },
          "importKit": {
            "label": {
              "normal": "Kit importieren",
              "beginner": "Kit importieren"
            }
          },
          "toolbar": {
            "showTemporary": {
              "label": {
                "normal": "Temporär",
                "beginner": "Temporär"
              }
            },
            "showLocal": {
              "label": {
                "normal": "Lokal",
                "beginner": "Lokal"
              }
            },
            "showRemote": {
              "label": {
                "normal": "Remote",
                "beginner": "Remote"
              }
            },
            "createTemporary": {
              "label": {
                "normal": "Temporär",
                "beginner": "Temporär"
              }
            },
            "createLocal": {
              "label": {
                "normal": "Lokal",
                "beginner": "Lokal"
              }
            },
            "createRemote": {
              "label": {
                "normal": "Remote",
                "beginner": "Remote"
              }
            },
            "filters": {
              "label": {
                "normal": "Filter",
                "beginner": "Kits nach Standort filtern"
              }
            },
            "create": {
              "label": {
                "normal": "Erstellen",
                "beginner": "Neues Kit erstellen"
              }
            },
            "createKit": {
              "label": {
                "normal": "Neues Kit",
                "beginner": "Ein neues leeres Kit erstellen"
              }
            },
            "openFolder": {
              "label": {
                "normal": "Ordner oeffnen",
                "beginner": "Ein Kit aus einem Ordner oeffnen"
              }
            },
            "openFile": {
              "label": {
                "normal": "Datei oeffnen",
                "beginner": "Ein Kit aus einer Zip-Datei oeffnen"
              }
            },
            "openRemote": {
              "label": {
                "normal": "Remote oeffnen",
                "beginner": "Ein Kit von einer Remote-URL oeffnen"
              }
            },
            "createFile": {
              "label": {
                "normal": "Neues Datei-Kit",
                "beginner": "Ein Kit anlegen, das in einer Datei gespeichert wird"
              }
            },
            "createFolder": {
              "label": {
                "normal": "Neues Ordner-Kit",
                "beginner": "Ein Kit anlegen, das in einem Ordner gespeichert wird"
              }
            },
            "showFile": {
              "label": {
                "normal": "Datei-Kits anzeigen",
                "beginner": "Kits anzeigen, die als Datei gespeichert sind"
              }
            },
            "showFolder": {
              "label": {
                "normal": "Ordner-Kits anzeigen",
                "beginner": "Kits anzeigen, die in Ordnern gespeichert sind"
              }
            },
            "exportArchive": {
              "label": {
                "normal": "Archiv exportieren",
                "beginner": "Das ausgewaehlte Kit als Zip-Archiv exportieren"
              }
            }
          }
        },
        "kit": {
          "label": {
            "normal": "Kit",
            "beginner": "Kit"
          },
          "properties": {
            "label": {
              "normal": "Kit-Eigenschaften",
              "beginner": "Kit-Eigenschaften"
            }
          },
          "notFound": {
            "label": {
              "normal": "Kit nicht gefunden",
              "beginner": "Das angeforderte Kit wurde nicht gefunden"
            },
            "description": {
              "normal": "Das Kit wurde moeglicherweise entfernt oder der Link ist ungueltig.",
              "beginner": "Zurueck zur Startseite und ein anderes Kit oeffnen oder ein neues erstellen."
            }
          },
          "noKitLoaded": {
            "label": {
              "normal": "Kein Kit geladen",
              "beginner": "Kein Kit geladen"
            }
          },
          "loading": {
            "label": {
              "normal": "Kit wird geladen...",
              "beginner": "Kit wird geladen..."
            }
          },
          "notAvailable": {
            "label": {
              "normal": "Kit nicht verfuegbar",
              "beginner": "Kit nicht verfuegbar"
            }
          },
          "dropzone": {
            "label": {
              "normal": "Zip-Datei zum Importieren ablegen",
              "beginner": "Legen Sie eine Zip-Datei hier ab, um sie als Kit zu importieren oder Dateien zum aktuellen Kit hinzuzufuegen"
            },
            "description": {
              "normal": "Kits mit .compose-Ordner werden importiert, andere als Dateien hinzugefuegt",
              "beginner": "Wenn die Zip-Datei einen .compose-Ordner enthaelt, wird sie als vollstaendiges Kit importiert. Andernfalls werden die Dateien zum aktuellen Kit hinzugefuegt."
            }
          },
          "versionPlaceholder": {
            "label": {
              "label": {
                "normal": "z.B., 1.0.0",
                "beginner": "z.B., 1.0.0"
              }
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie den Inhalt dieses Kits...",
                "beginner": "Beschreiben Sie den Inhalt dieses Kits..."
              }
            }
          },
          "iconPlaceholder": {
            "label": {
              "label": {
                "normal": "🎨 oder URL zum Icon",
                "beginner": "🎨 oder URL zum Icon"
              }
            }
          },
          "imagePlaceholder": {
            "label": {
              "label": {
                "normal": "URL zum Vorschaubild",
                "beginner": "URL zum Vorschaubild"
              }
            }
          },
          "homepagePlaceholder": {
            "label": {
              "label": {
                "normal": "https://beispiel.de",
                "beginner": "https://beispiel.de"
              }
            }
          },
          "licensePlaceholder": {
            "label": {
              "label": {
                "normal": "z.B., MIT, GPL-3.0, Apache-2.0",
                "beginner": "z.B., MIT, GPL-3.0, Apache-2.0"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "Neues Kit",
              "beginner": "Neues Kit"
            }
          },
          "defaultDesignName": {
            "label": {
              "normal": "Neuer Entwurf",
              "beginner": "Neuer Entwurf"
            }
          },
          "defaultTypeName": {
            "label": {
              "normal": "Neuer Typ",
              "beginner": "Neuer Typ"
            }
          },
          "newVersion": {
            "label": {
              "normal": "Neue Version",
              "beginner": "Neue Version"
            }
          },
          "defaultVersion": {
            "label": {
              "normal": "Standard",
              "beginner": "Standard"
            }
          },
          "filter": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "name": {
              "label": {
                "normal": "Nach Name filtern",
                "beginner": "Klicken, um Artefakte nach diesem Namen zu filtern"
              },
              "manual": "manuals/compose/kit",
              "tutorial": "hello-compose/model-design",
              "hotkey": "Ctrl+N",
              "hide": {
                "label": {
                  "normal": "Namensfilter ausblenden",
                  "beginner": "Klicken, um den Namensfilter auszublenden"
                },
                "hotkey": "Ctrl+Shift+N"
              }
            },
            "band": {
              "label": {
                "normal": "Filterleiste",
                "beginner": "Filterleiste ein- oder ausblenden"
              },
              "hotkey": "Ctrl+F"
            },
            "search": {
              "label": {
                "normal": "Suchfilter",
                "beginner": "Nach bestimmten Filtern suchen"
              },
              "hotkey": "Ctrl+Shift+F"
            }
          },
          "pieces": {
            "label": {
              "normal": "Teile",
              "beginner": "Typen und Entwuerfe in diesem Kit"
            }
          },
          "designs": {
            "label": {
              "normal": "Entwuerfe",
              "beginner": "Entwuerfe in diesem Kit"
            },
            "manual": "manuals/compose/kit#designs",
            "tutorial": "hello-compose/model-design",
            "multipleSelected": {
              "label": {
                "normal": "Mehrere ausgewählt",
                "beginner": "Mehrere ausgewählt"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "types": {
            "label": {
              "normal": "Typen",
              "beginner": "Typen in diesem Kit"
            },
            "manual": "manuals/compose/kit#types",
            "tutorial": "hello-compose/model-brick-set",
            "multipleSelected": {
              "label": {
                "normal": "Mehrere ausgewählt",
                "beginner": "Mehrere ausgewählt"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "sortByArtifact": {
            "label": {
              "normal": "Nach Artefakt sortieren",
              "beginner": "Artefakte nach ihrem Namen sortieren"
            },
            "hotkey": "Ctrl+Shift+A"
          },
          "sortByKind": {
            "label": {
              "normal": "Nach Art sortieren",
              "beginner": "Artefakte nach ihrem Typ sortieren (Entwurf, Typ, Qualitaet, usw.)"
            },
            "hotkey": "Ctrl+Shift+K"
          },
          "sortByCreatedAt": {
            "label": {
              "normal": "Nach Erstellungsdatum sortieren",
              "beginner": "Artefakte nach ihrem Erstellungsdatum sortieren"
            },
            "hotkey": "Ctrl+Shift+C"
          },
          "sortByUpdatedAt": {
            "label": {
              "normal": "Nach Aenderungsdatum sortieren",
              "beginner": "Artefakte nach ihrem letzten Aenderungsdatum sortieren"
            },
            "hotkey": "Ctrl+Shift+U"
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showDesigns": {
              "label": {
                "normal": "Entwuerfe anzeigen",
                "beginner": "Klicken, um alle Entwuerfe in diesem Kit anzuzeigen"
              },
              "manual": "manuals/compose/kit#designs",
              "tutorial": "hello-compose/model-design"
            },
            "createDesign": {
              "label": {
                "normal": "Entwurf erstellen",
                "beginner": "Klicken, um einen neuen Entwurf in diesem Kit zu erstellen"
              },
              "manual": "manuals/compose/kit#designs",
              "tutorial": "hello-compose/model-design"
            },
            "showTypes": {
              "label": {
                "normal": "Typen anzeigen",
                "beginner": "Klicken, um alle Typen in diesem Kit anzuzeigen"
              },
              "manual": "manuals/compose/kit#types",
              "tutorial": "hello-compose/model-brick-set"
            },
            "createType": {
              "label": {
                "normal": "Typ erstellen",
                "beginner": "Klicken, um einen neuen Typ in diesem Kit zu erstellen"
              },
              "manual": "manuals/compose/kit#types",
              "tutorial": "hello-compose/model-brick-set"
            },
            "showQualities": {
              "label": {
                "normal": "Qualitaeten anzeigen",
                "beginner": "Klicken, um alle Qualitaeten in diesem Kit anzuzeigen"
              },
              "manual": "manuals/compose/kit#qualities",
              "tutorial": "getting-started/intro#quality"
            },
            "createQuality": {
              "label": {
                "normal": "Qualitaet erstellen",
                "beginner": "Klicken, um eine neue Qualitaetsdefinition in diesem Kit zu erstellen"
              },
              "manual": "manuals/compose/kit#qualities",
              "tutorial": "getting-started/intro#quality"
            },
            "showPorts": {
              "label": {
                "normal": "Schnittstellen anzeigen",
                "beginner": "Klicken, um alle Schnittstellen in diesem Kit anzuzeigen"
              },
              "manual": "manuals/compose/kit#ports",
              "tutorial": "getting-started/intro#port"
            },
            "createPort": {
              "label": {
                "normal": "Schnittstelle erstellen",
                "beginner": "Klicken, um eine neue Schnittstellendefinition in diesem Kit zu erstellen"
              },
              "manual": "manuals/compose/kit#ports",
              "tutorial": "getting-started/intro#port"
            },
            "showFiles": {
              "label": {
                "normal": "Dateien anzeigen",
                "beginner": "Klicken, um alle Dateien in diesem Kit anzuzeigen"
              },
              "manual": "manuals/compose/kit#files",
              "tutorial": "getting-started/intro#files"
            },
            "createFile": {
              "label": {
                "normal": "Datei erstellen",
                "beginner": "Klicken, um eine neue Datei zu diesem Kit hinzuzufuegen"
              },
              "manual": "manuals/compose/kit#files",
              "tutorial": "getting-started/intro#files"
            },
            "showFolders": {
              "label": {
                "normal": "Ordner anzeigen",
                "beginner": "Klicken, um alle Ordner in diesem Kit anzuzeigen"
              },
              "manual": "manuals/compose/kit#folders",
              "tutorial": "hello-compose/model-design"
            },
            "createFolder": {
              "label": {
                "normal": "Ordner erstellen",
                "beginner": "Klicken, um einen neuen Ordner in diesem Kit zu erstellen"
              },
              "manual": "manuals/compose/kit#folders",
              "tutorial": "hello-compose/model-design"
            },
            "reset": {
              "label": {
                "normal": "Zuruecksetzen",
                "beginner": "Klicken, um das Kit auf den urspruenglichen Zustand zurueckzusetzen"
              }
            },
            "showAuthors": {
              "label": {
                "normal": "Autoren anzeigen",
                "beginner": "Klicken, um alle Autoren dieses Kits anzuzeigen"
              },
              "manual": "manuals/compose/kit#authors",
              "tutorial": "getting-started/intro#authors"
            },
            "createAuthor": {
              "label": {
                "normal": "Autor erstellen",
                "beginner": "Klicken, um einen neuen Autor zu diesem Kit hinzuzufuegen"
              },
              "manual": "manuals/compose/kit#authors",
              "tutorial": "getting-started/intro#authors"
            },
            "hideKind": {
              "label": {
                "normal": "Ausblenden",
                "beginner": "Klicken, um diese Artefaktkategorie auszublenden"
              }
            },
            "createArtifact": {
              "label": {
                "normal": "Erstellen",
                "beginner": "Klicken, um ein neues Artefakt dieses Typs zu erstellen"
              }
            },
            "createChild": {
              "label": {
                "normal": "Kind erstellen",
                "beginner": "Ein Kindelement erstellen"
              }
            },
            "showTags": {
              "label": {
                "normal": "Tags anzeigen",
                "beginner": "Tags anzeigen"
              }
            },
            "showConcepts": {
              "label": {
                "normal": "Konzepte anzeigen",
                "beginner": "Konzepte anzeigen"
              }
            },
            "filters": {
              "label": {
                "normal": "Filter",
                "beginner": "Artefakte nach Typ filtern"
              }
            },
            "resetFilters": {
              "label": {
                "normal": "Filter Zurücksetzen",
                "beginner": "Aktive Artefaktfilter entfernen und alle Artefaktarten anzeigen"
              }
            },
            "selection": {
              "label": {
                "normal": "Auswahl",
                "beginner": "Auswahlwerkzeuge"
              }
            },
            "create": {
              "label": {
                "normal": "Erstellen",
                "beginner": "Neue Artefakte erstellen"
              }
            }
          },
          "canvas": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "table": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "search": {
                "label": {
                  "normal": "Tabelle durchsuchen",
                  "beginner": "Nach Artefakten in der Tabelle suchen"
                },
                "hotkey": "Ctrl+F"
              },
              "header": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "kind": {
                  "label": {
                    "normal": "Art",
                    "beginner": "Art des Artefakts"
                  }
                },
                "artifact": {
                  "label": {
                    "normal": "Name",
                    "beginner": "Der Name"
                  }
                },
                "updatedAt": {
                  "label": {
                    "normal": "Aktualisiert",
                    "beginner": "Letzte Aktualisierungszeit"
                  }
                },
                "createdAt": {
                  "label": {
                    "normal": "Erstellt",
                    "beginner": "Erstellungszeit"
                  }
                }
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagramm",
                "beginner": "Ein kraftgerichteter Graph, der alle Kit-Artefakte und ihre Beziehungen zeigt"
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "kit": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Kit-Name",
                      "beginner": "Der Name des Kits. Dies ist die primaere Kennung fuer Ihr Kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "version": {
                    "label": {
                      "normal": "Version",
                      "beginner": "Die Version des Kits im semantischen Versionierungsformat (z.B. 1.0.0)."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine detaillierte Beschreibung dessen, was dieses Kit enthaelt und wie es verwendet werden sollte."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "icon": {
                    "label": {
                      "normal": "Symbol",
                      "beginner": "Ein Icon zur Darstellung dieses Kits. Kann ein Emoji oder eine URL zu einem Bild sein."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "image": {
                    "label": {
                      "normal": "Bild",
                      "beginner": "URL zu einem Vorschaubild, das dieses Kit praesentiert."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "homepage": {
                    "label": {
                      "normal": "Homepage",
                      "beginner": "URL zur Homepage oder Dokumentation fuer dieses Kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "license": {
                    "label": {
                      "normal": "Lizenz",
                      "beginner": "Die Lizenz, unter der dieses Kit verteilt wird (z.B. MIT, GPL)."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  }
                },
                "folder": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Anzeigename des Ordners."
                    },
                    "manual": "kit#folders",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Optionale Beschreibung, die den Zweck dieses Ordners erlaeutert."
                    },
                    "manual": "kit#folders",
                    "tutorial": "hello-compose/save-kit"
                  }
                },
                "port": {
                  "compatible": {
                    "label": {
                      "normal": "Kompatibel",
                      "beginner": "Kompatibel"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Beschreibung"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  }
                },
                "tag": {
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Beschreibung"
                    }
                  }
                },
                "concept": {
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Beschreibung"
                    }
                  }
                }
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Kit-Einstellungen"
            },
            "diagram": {
              "chargeStrength": {
                "label": {
                  "normal": "Ladungsstärke",
                  "beginner": "Abstoßungskraft zwischen Knoten"
                }
              },
              "linkDistance": {
                "label": {
                  "normal": "Verbindungsabstand",
                  "beginner": "Zielabstand zwischen verbundenen Knoten"
                }
              },
              "collideRadius": {
                "label": {
                  "normal": "Kollisionsradius",
                  "beginner": "Kollisionsradius zur Vermeidung von Knotenüberlappung"
                }
              },
              "centerStrength": {
                "label": {
                  "normal": "Zentrierungsstärke",
                  "beginner": "Kraft, die Knoten zur Mitte zieht"
                }
              }
            },
            "theme": {
              "label": {
                "normal": "Thema",
                "beginner": "Farbthema"
              }
            },
            "language": {
              "label": {
                "normal": "Sprache",
                "beginner": "Oberflächensprache"
              }
            },
            "device": {
              "label": {
                "normal": "Gerät",
                "beginner": "Eingabegerätetyp"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "Benutzerexpertise-Stufe"
              }
            },
            "mode": {
              "label": {
                "normal": "Modus",
                "beginner": "Benutzer- oder Entwicklermodus"
              }
            }
          },
          "port": {
            "allCompatible": {
              "label": {
                "normal": "Alle kompatibel",
                "beginner": "Alle kompatibel"
              }
            },
            "compatiblePorts": {
              "label": {
                "normal": "Kompatible Schnittstellen",
                "beginner": "Kompatible Schnittstellen"
              }
            },
            "descriptionPlaceholder": {
              "label": {
                "label": {
                  "normal": "Label",
                  "beginner": "Label"
                }
              }
            }
          },
          "ports": {
            "multipleSelected": {
              "label": {
                "normal": "Mehrere ausgewählt",
                "beginner": "Mehrere ausgewählt"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "{{count}} Schnittstellen",
                "beginner": "{{count}} Schnittstellen ausgewaehlt"
              }
            }
          },
          "qualities": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "files": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "authors": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "tag": {
            "descriptionPlaceholder": {
              "label": {
                "normal": "Beschreiben Sie diesen Tag...",
                "beginner": "Beschreiben Sie diesen Tag..."
              }
            }
          },
          "tags": {
            "multipleSelected": {
              "label": {
                "normal": "Mehrere Tags ausgewaehlt",
                "beginner": "Mehrere Tags ausgewaehlt"
              }
            },
            "multipleTitle": "{{count}} Tags"
          },
          "concept": {
            "descriptionPlaceholder": {
              "label": {
                "normal": "Beschreiben Sie dieses Konzept...",
                "beginner": "Beschreiben Sie dieses Konzept..."
              }
            }
          },
          "concepts": {
            "multipleSelected": {
              "label": {
                "normal": "Mehrere Konzepte ausgewaehlt",
                "beginner": "Mehrere Konzepte ausgewaehlt"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Titel",
              "beginner": "Titel"
            }
          },
          "tools": {
            "label": {
              "normal": "Werkzeuge",
              "beginner": "Werkzeuge"
            },
            "select": {
              "mode": {
                "additive": {
                  "label": {
                    "normal": "Additiv",
                    "beginner": "Additiver Auswahlmodus - zur vorhandenen Auswahl hinzufügen"
                  }
                },
                "subtractive": {
                  "label": {
                    "normal": "Subtraktiv",
                    "beginner": "Subtraktiver Auswahlmodus - aus vorhandener Auswahl entfernen"
                  }
                },
                "intersect": {
                  "label": {
                    "normal": "Schnittmenge",
                    "beginner": "Schnittmengen-Auswahlmodus - nur überlappende Elemente auswählen"
                  }
                }
              },
              "shape": {
                "rectangular": {
                  "label": {
                    "normal": "Rechteckig",
                    "beginner": "Rechteckige Auswahl - ziehen Sie, um in einem Rechteck auszuwählen"
                  }
                },
                "lasso": {
                  "label": {
                    "normal": "Lasso",
                    "beginner": "Freiform-Lasso-Auswahl - zeichnen Sie eine Freiform-Form"
                  }
                }
              },
              "navigation": {
                "hand": {
                  "label": {
                    "normal": "Hand",
                    "beginner": "Hand-Werkzeug - schwenken und navigieren Sie auf der Leinwand"
                  }
                }
              }
            }
          },
          "folder": {
            "label": {
              "normal": "Ordner",
              "beginner": "Ordner"
            },
            "descriptionPlaceholder": {
              "label": {
                "label": {
                  "normal": "Describe this folder...",
                  "beginner": "Describe this folder..."
                }
              }
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Kit-Chat"
            }
          }
        },
        "port": {
          "label": {
            "normal": "Schnittstelle",
            "beginner": "Schnittstelle"
          },
          "defaultName": {
            "label": {
              "normal": "Neue Schnittstelle",
              "beginner": "Neue Schnittstelle"
            }
          }
        },
        "tag": {
          "label": {
            "normal": "Tag",
            "beginner": "Tag"
          },
          "defaultName": {
            "label": {
              "normal": "Neuer Tag",
              "beginner": "Neuer Tag"
            }
          }
        },
        "concept": {
          "label": {
            "normal": "Konzept",
            "beginner": "Konzept"
          },
          "defaultName": {
            "label": {
              "normal": "Neues Konzept",
              "beginner": "Neues Konzept"
            }
          }
        },
        "folder": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "defaultName": {
            "label": {
              "normal": "Neuer Ordner",
              "beginner": "Neuer Ordner"
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie diesen Ordner...",
                "beginner": "Beschreiben Sie diesen Ordner..."
              }
            }
          }
        },
        "design": {
          "label": {
            "normal": "Entwurf",
            "beginner": "Entwurf"
          },
          "properties": {
            "label": {
              "normal": "Entwurfs-Eigenschaften",
              "beginner": "Entwurfs-Eigenschaften"
            }
          },
          "defaultName": {
            "label": {
              "normal": "Neuer Entwurf",
              "beginner": "Neuer Entwurf"
            }
          },
          "windowLibrary": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "scene": {
              "label": {
                "normal": "Szenen-Fenster",
                "beginner": "Durchsuchen und Hinzufuegen von 3D-Szenen-Fenstern zum Betrachten Ihres Entwurfs im 3D-Raum."
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagramm-Fenster",
                "beginner": "Durchsuchen und Hinzufuegen von 2D-Diagramm-Fenstern zum Betrachten der Verbindungstopologie."
              }
            },
            "table": {
              "label": {
                "normal": "Tabellen-Fenster",
                "beginner": "Durchsuchen und Hinzufuegen von Tabellen-Fenstern zum Betrachten von Entwurfsdaten in tabellarischer Form."
              }
            }
          },
          "diagram": {
            "clusterMenu": {
              "cluster": {
                "label": {
                  "normal": "Gruppieren",
                  "beginner": "Ausgewählte Entwurfsteile zu einem einzelnen Cluster gruppieren"
                }
              }
            },
            "expandMenu": {
              "expand": {
                "label": {
                  "normal": "Erweitern",
                  "beginner": "Das Entwurfsteil erweitern, um seine internen Komponenten anzuzeigen"
                }
              }
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "normal": "Beschreiben Sie diesen Entwurf...",
              "beginner": "Beschreiben Sie diesen Entwurf..."
            }
          },
          "iconPlaceholder": {
            "label": {
              "normal": "???",
              "beginner": "???"
            }
          },
          "imagePlaceholder": {
            "label": {
              "normal": "https://example.com/image.png",
              "beginner": "https://example.com/image.png"
            }
          },
          "variantPlaceholder": {
            "label": {
              "normal": "z.B. klein, mittel, gross",
              "beginner": "z.B. klein, mittel, gross"
            }
          },
          "viewPlaceholder": {
            "label": {
              "normal": "z.B. vorne, seite, oben",
              "beginner": "z.B. vorne, seite, oben"
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Name"
            }
          },
          "variant": {
            "label": {
              "normal": "Variante",
              "beginner": "Variante"
            }
          },
          "view": {
            "label": {
              "normal": "Ansicht",
              "beginner": "Ansicht"
            }
          },
          "location": {
            "label": {
              "normal": "Standort",
              "beginner": "Standort"
            }
          },
          "authors": {
            "label": {
              "normal": "Autoren",
              "beginner": "Autoren"
            }
          },
          "author": {
            "label": {
              "normal": "Autor",
              "beginner": "Autor"
            }
          },
          "attributes": {
            "label": {
              "normal": "Attribute",
              "beginner": "Attribute"
            }
          },
          "attribute": {
            "label": {
              "normal": "Attribut",
              "beginner": "Attribut"
            }
          },
          "attributeValuePlaceholder": {
            "label": {
              "normal": "Wert...",
              "beginner": "Wert..."
            }
          },
          "attributeUnitPlaceholder": {
            "label": {
              "normal": "Einheit...",
              "beginner": "Einheit..."
            }
          },
          "attributeDefinitionPlaceholder": {
            "label": {
              "normal": "Definition oder URL...",
              "beginner": "Definition oder URL..."
            }
          },
          "piece": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "id": {
              "label": {
                "normal": "ID",
                "beginner": "ID"
              }
            },
            "type": {
              "label": {
                "normal": "Typ",
                "beginner": "Typ"
              }
            },
            "center": {
              "label": {
                "normal": "Zentrum",
                "beginner": "Zentrum"
              }
            },
            "plane": {
              "label": {
                "normal": "Ebene",
                "beginner": "Ebene"
              }
            },
            "planeOrigin": {
              "label": {
                "normal": "Ursprung",
                "beginner": "Ursprung"
              }
            },
            "planeXAxis": {
              "label": {
                "normal": "X-Achse",
                "beginner": "X-Achse"
              }
            },
            "planeYAxis": {
              "label": {
                "normal": "Y-Achse",
                "beginner": "Y-Achse"
              }
            },
            "mixedSelectionMessage": {
              "label": {
                "normal": "Mehrere Teile mit unterschiedlichen Werten ausgewaehlt",
                "beginner": "Mehrere Teile mit unterschiedlichen Werten ausgewaehlt"
              }
            },
            "connectedPieceInfo": {
              "label": {
                "normal": "Dieses Teil ist mit einem anderen Teil verbunden. Seine Position und Ausrichtung werden aus der Verbindung berechnet. Um es unabhängig zu machen, klicken Sie auf 'Teil fixieren'.",
                "beginner": "Dieses Teil ist mit einem anderen Teil verbunden. Seine Position und Ausrichtung werden aus der Verbindung berechnet. Um es unabhängig zu machen, klicken Sie auf 'Teil fixieren'."
              }
            },
            "fixPiece": {
              "label": {
                "normal": "Teil fixieren",
                "beginner": "Teil fixieren"
              }
            }
          },
          "connection": {
            "label": {},
            "rotation": {
              "label": {
                "normal": "Rotation",
                "beginner": "Rotation"
              }
            },
            "turn": {
              "label": {
                "normal": "Drehung",
                "beginner": "Drehung"
              }
            },
            "tilt": {
              "label": {
                "normal": "Neigung",
                "beginner": "Neigung"
              }
            },
            "plane": {
              "label": {
                "normal": "Ebene",
                "beginner": "Ebene"
              }
            },
            "translation": {
              "label": {
                "normal": "Verschiebung",
                "beginner": "Verschiebung"
              }
            },
            "orientation": {
              "label": {
                "normal": "Ausrichtung",
                "beginner": "Ausrichtung"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagramm",
                "beginner": "Diagramm"
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "design": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Name des Entwurfs. Dies ist die primaere Kennung fuer Ihre Komposition."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine detaillierte Beschreibung dessen, was dieser Entwurf darstellt und wie er verwendet werden sollte."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "icon": {
                    "label": {
                      "normal": "Symbol",
                      "beginner": "URL oder Pfad zu einem Icon, das diesen Entwurf in Listen und Vorschauen darstellt."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "image": {
                    "label": {
                      "normal": "Bild",
                      "beginner": "URL oder Pfad zu einem Vorschaubild, das diesen Entwurf praesentiert."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "variant": {
                    "label": {
                      "normal": "Variante",
                      "beginner": "Eine Variantenkennung fuer verschiedene Versionen oder Konfigurationen dieses Entwurfs."
                    },
                    "manual": "design#variants",
                    "tutorial": "hello-compose/model-design"
                  },
                  "view": {
                    "label": {
                      "normal": "Ansicht",
                      "beginner": "Die Ansichtsperspektive oder Kamerawinkel zur Anzeige dieses Entwurfs."
                    },
                    "manual": "design#views",
                    "tutorial": "hello-compose/model-design"
                  },
                  "unit": {
                    "label": {
                      "normal": "Einheit",
                      "beginner": "Die Masseinheit fuer alle Abmessungen in diesem Entwurf (z.B. mm, cm, m)."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "createdAt": {
                    "label": {
                      "normal": "Erstellt am",
                      "beginner": "Das Datum und die Uhrzeit, wann dieser Entwurf erstmals erstellt wurde."
                    }
                  },
                  "updatedAt": {
                    "label": {
                      "normal": "Aktualisiert am",
                      "beginner": "Das Datum und die Uhrzeit, wann dieser Entwurf zuletzt geaendert wurde."
                    }
                  },
                  "pieceCount": {
                    "label": {
                      "normal": "Bauteile",
                      "beginner": "Die Gesamtanzahl der Bauteile in diesem Entwurf."
                    }
                  },
                  "connectionCount": {
                    "label": {
                      "normal": "Verbindungen",
                      "beginner": "Die Gesamtanzahl der Verbindungen in diesem Entwurf."
                    }
                  }
                },
                "location": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "longitude": {
                    "label": {
                      "normal": "Laengengrad",
                      "beginner": "Die Ost-West-Position dieses Entwurfsstandorts in Dezimalgrad."
                    },
                    "manual": "design#location"
                  },
                  "latitude": {
                    "label": {
                      "normal": "Breitengrad",
                      "beginner": "Die Nord-Sued-Position dieses Entwurfsstandorts in Dezimalgrad."
                    },
                    "manual": "design#location"
                  }
                },
                "authors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der vollstaendige Name der Person, die zu diesem Entwurf beigetragen hat."
                    },
                    "manual": "design#authors"
                  },
                  "email": {
                    "label": {
                      "normal": "E-Mail",
                      "beginner": "Kontakt-E-Mail-Adresse fuer diesen Autor."
                    },
                    "manual": "design#authors"
                  }
                },
                "attributes": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Die eindeutige Kennung fuer dieses Attribut im Kebab-Case-Format (z.B. 'material.type')."
                    },
                    "manual": "design#attributes"
                  },
                  "value": {
                    "label": {
                      "normal": "Wert",
                      "beginner": "Der Wert, der mit diesem Attribut verbunden ist. Leer lassen, um als Kategorie-Flag zu verwenden."
                    },
                    "manual": "design#attributes"
                  },
                  "unit": {
                    "label": {
                      "normal": "Einheit",
                      "beginner": "Die Masseinheit fuer den Wert dieses Attributs (z.B. mm, kg, °C)."
                    },
                    "manual": "design#attributes"
                  },
                  "definition": {
                    "label": {
                      "normal": "Definition",
                      "beginner": "Eine URL oder ein Text, der definiert, was dieses Attribut bedeutet und wie es interpretiert werden sollte."
                    },
                    "manual": "design#attributes"
                  }
                },
                "piece": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "properties": {
                    "label": {
                      "normal": "Bauteil",
                      "beginner": "Eigenschaften des ausgewaehlten Bauteils."
                    }
                  },
                  "multipleTitle": {
                    "label": {
                      "normal": "Bauteile",
                      "beginner": "Eigenschaften der ausgewaehlten Bauteile."
                    }
                  },
                  "pieceInfo": {
                    "label": {
                      "normal": "Bauteil",
                      "beginner": "Grundinformationen ueber das Bauteil."
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Ein optionaler Name für dieses Bauteil zur Identifikation innerhalb des Entwurfs."
                    }
                  },
                  "namePlaceholder": {
                    "label": {
                      "normal": "Bauteilname eingeben...",
                      "beginner": "Bauteilname eingeben..."
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung des Zwecks oder der Rolle dieses Bauteils im Entwurf."
                    }
                  },
                  "descriptionPlaceholder": {
                    "label": {
                      "normal": "Bauteilbeschreibung eingeben...",
                      "beginner": "Bauteilbeschreibung eingeben..."
                    }
                  },
                  "scale": {
                    "label": {
                      "normal": "Skalierung",
                      "beginner": "Der Skalierungsfaktor für dieses Bauteil. Standard ist 1.0."
                    }
                  },
                  "color": {
                    "label": {
                      "normal": "Farbe",
                      "beginner": "Eine optionale Farbüberschreibung für dieses Bauteil (z.B. #FF0000)."
                    }
                  },
                  "colorPlaceholder": {
                    "label": {
                      "normal": "Farbe eingeben...",
                      "beginner": "Farbe eingeben..."
                    }
                  },
                  "attributes": {
                    "label": {
                      "normal": "Attribute",
                      "beginner": "Benutzerdefinierte Schlüssel-Wert-Attribute für dieses Bauteil."
                    },
                    "name": {
                      "label": {
                        "normal": "Name",
                        "beginner": "Der Schlüsselbezeichner für dieses Attribut."
                      }
                    },
                    "value": {
                      "label": {
                        "normal": "Wert",
                        "beginner": "Der Wert für dieses Attribut."
                      }
                    },
                    "unit": {
                      "label": {
                        "normal": "Einheit",
                        "beginner": "Die Maßeinheit für dieses Attribut."
                      }
                    },
                    "definition": {
                      "label": {
                        "normal": "Definition",
                        "beginner": "Eine URL oder ein Text, der dieses Attribut definiert."
                      }
                    }
                  },
                  "attribute": {
                    "label": {
                      "normal": "Attribut",
                      "beginner": "Ein benutzerdefiniertes Attribut dieses Bauteils."
                    }
                  },
                  "center": {
                    "label": {
                      "normal": "Zentrum",
                      "beginner": "Die Zentrumsposition des Bauteils im 2D-Diagramm-Layout."
                    },
                    "manual": "design#diagram",
                    "tutorial": "metabolism/thinking-about-the-diagram",
                    "x": {
                      "label": {
                        "normal": "U",
                        "beginner": "U-Diagrammkoordinate des Zentrums des Bauteils im 2D-Layoutraum."
                      },
                      "manual": "design#diagram",
                      "tutorial": "metabolism/thinking-about-the-diagram"
                    },
                    "y": {
                      "label": {
                        "normal": "V",
                        "beginner": "V-Diagrammkoordinate des Zentrums des Bauteils im 2D-Layoutraum."
                      },
                      "manual": "design#diagram",
                      "tutorial": "metabolism/thinking-about-the-diagram"
                    }
                  },
                  "plane": {
                    "label": {
                      "normal": "Ebene",
                      "beginner": "Die 3D-Platzierungsebene fuer dieses Bauteil. Definiert Position und Ausrichtung im 3D-Raum."
                    },
                    "manual": "design#pieces",
                    "tutorial": "hello-compose/model-design#pieces",
                    "origin": {
                      "label": {
                        "normal": "Ursprung",
                        "beginner": "Ursprung"
                      },
                      "x": {
                        "label": {
                          "normal": "Ursprung X",
                          "beginner": "X-Koordinate des Ursprungs"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Ursprung Y",
                          "beginner": "Y-Koordinate des Ursprungs"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Ursprung Z",
                          "beginner": "Z-Koordinate des Ursprungs"
                        }
                      }
                    },
                    "xaxis": {
                      "label": {
                        "normal": "X-Achse",
                        "beginner": "X-Achse"
                      },
                      "x": {
                        "label": {
                          "normal": "X-Achse X",
                          "beginner": "X-Komponente der X-Achse"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "X-Achse Y",
                          "beginner": "Y-Komponente der X-Achse"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "X-Achse Z",
                          "beginner": "Z-Komponente der X-Achse"
                        }
                      }
                    },
                    "yaxis": {
                      "label": {
                        "normal": "Y-Achse",
                        "beginner": "Y-Achse"
                      },
                      "x": {
                        "label": {
                          "normal": "Y-Achse X",
                          "beginner": "X-Komponente der Y-Achse"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Y-Achse Y",
                          "beginner": "Y-Komponente der Y-Achse"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Y-Achse Z",
                          "beginner": "Z-Komponente der Y-Achse"
                        }
                      }
                    }
                  }
                },
                "connection": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "properties": {
                    "label": {
                      "normal": "Verbindung",
                      "beginner": "Eigenschaften der ausgewaehlten Verbindung."
                    }
                  },
                  "multipleTitle": {
                    "label": {
                      "normal": "Verbindungen",
                      "beginner": "Eigenschaften der ausgewaehlten Verbindungen."
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung des Zwecks dieser Verbindung."
                    }
                  },
                  "descriptionPlaceholder": {
                    "label": {
                      "normal": "Verbindungsbeschreibung eingeben...",
                      "beginner": "Verbindungsbeschreibung eingeben..."
                    }
                  },
                  "multipleEditing": {
                    "label": {
                      "normal": "{{count}} Verbindungen werden gleichzeitig bearbeitet",
                      "beginner": "{{count}} Verbindungen werden gleichzeitig bearbeitet"
                    }
                  },
                  "connecting": {
                    "label": {
                      "normal": "Verbindend",
                      "beginner": "Verbindend"
                    }
                  },
                  "connectingPieceId": {
                    "label": {
                      "normal": "Verbindendes Stück",
                      "beginner": "Verbindendes Stück"
                    }
                  },
                  "connectingPortId": {
                    "label": {
                      "normal": "Verbindender Anschluss",
                      "beginner": "Verbindender Anschluss"
                    }
                  },
                  "connectingDesignPieceId": {
                    "label": {
                      "normal": "Verbindendes Designstück",
                      "beginner": "Verbindendes Designstück"
                    }
                  },
                  "connected": {
                    "label": {
                      "normal": "Verbunden",
                      "beginner": "Verbunden"
                    }
                  },
                  "connectedPieceId": {
                    "label": {
                      "normal": "Verbundenes Stück",
                      "beginner": "Verbundenes Stück"
                    }
                  },
                  "connectedPortId": {
                    "label": {
                      "normal": "Verbundener Anschluss",
                      "beginner": "Verbundener Anschluss"
                    }
                  },
                  "connectedDesignPieceId": {
                    "label": {
                      "normal": "Verbundenes Designstück",
                      "beginner": "Verbundenes Designstück"
                    }
                  },
                  "gap": {
                    "label": {
                      "normal": "Abstand",
                      "beginner": "Abstand"
                    }
                  },
                  "shift": {
                    "label": {
                      "normal": "Verschiebung",
                      "beginner": "Verschiebung"
                    }
                  },
                  "rise": {
                    "label": {
                      "normal": "Anstieg",
                      "beginner": "Anstieg"
                    }
                  },
                  "rotation": {
                    "label": {
                      "normal": "Rotation",
                      "beginner": "Rotation"
                    }
                  },
                  "turn": {
                    "label": {
                      "normal": "Drehung",
                      "beginner": "Drehung"
                    }
                  },
                  "tilt": {
                    "label": {
                      "normal": "Neigung",
                      "beginner": "Neigung"
                    }
                  },
                  "x": {
                    "label": {
                      "normal": "Diagramm X-Versatz",
                      "beginner": "X-Versatz im Diagramm"
                    }
                  },
                  "y": {
                    "label": {
                      "normal": "Diagramm Y-Versatz",
                      "beginner": "Y-Versatz im Diagramm"
                    }
                  },
                  "u": {
                    "label": {
                      "normal": "X-Versatz",
                      "beginner": "X-Versatz"
                    }
                  },
                  "v": {
                    "label": {
                      "normal": "Y-Versatz",
                      "beginner": "Y-Versatz"
                    }
                  }
                },
                "connector": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "id": {
                    "label": {
                      "normal": "Connector-ID",
                      "beginner": "Die eindeutige Kennung des Connectors"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Name dieses Connectors."
                    }
                  },
                  "t": {
                    "label": {
                      "normal": "T",
                      "beginner": "Der Parameter t entlang der Kurve des Typs für diesen Connector."
                    }
                  },
                  "position": {
                    "label": {
                      "normal": "Position",
                      "beginner": "Die Position des Connectors"
                    }
                  },
                  "direction": {
                    "label": {
                      "normal": "Richtung",
                      "beginner": "Der Richtungsvektor des Connectors"
                    }
                  },
                  "mandatory": {
                    "label": {
                      "normal": "Erforderlich",
                      "beginner": "Ob dieser Connector verbunden sein muss"
                    }
                  },
                  "port": {
                    "label": {
                      "normal": "Familie",
                      "beginner": "Die Connectorfamilie für Kompatibilitätsprüfung"
                    }
                  },
                  "compatiblePort": {
                    "label": {
                      "normal": "Kompatible Familie",
                      "beginner": "Die Familien, mit denen dieser Connector kompatibel ist"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung des Connectors"
                    }
                  },
                  "attribute": {
                    "label": {
                      "normal": "Attribut",
                      "beginner": "Benutzerdefinierte Attribute des Connectors"
                    }
                  },
                  "notFound": {
                    "label": {
                      "normal": "Nicht gefunden",
                      "beginner": "Nicht gefunden"
                    }
                  }
                }
              },
              "parentConnection": {
                "label": {
                  "normal": "Elternverbindung",
                  "beginner": "Elternverbindung"
                }
              },
              "parentConnections": {
                "label": {
                  "normal": "Elternverbindungen",
                  "beginner": "Elternverbindungen"
                }
              }
            },
            "hud": {
              "overview": {
                "label": {
                  "normal": "HUD-Uebersicht",
                  "beginner": "HUD-Uebersicht"
                }
              },
              "selection": {
                "pieces": {
                  "label": {
                    "normal": "Ausgewaehlte Bauteile",
                    "beginner": "Ausgewaehlte Bauteile"
                  }
                },
                "connections": {
                  "label": {
                    "normal": "Ausgewaehlte Verbindungen",
                    "beginner": "Ausgewaehlte Verbindungen"
                  }
                },
                "connector": {
                  "label": {
                    "normal": "Connector ausgewaehlt",
                    "beginner": "Connector ausgewaehlt"
                  }
                }
              }
            },
            "stats": {
              "overview": {
                "label": {
                  "normal": "Statistik",
                  "beginner": "Statistik"
                }
              },
              "pieces": {
                "label": {
                  "normal": "Bauteile gesamt",
                  "beginner": "Bauteile gesamt"
                }
              },
              "connections": {
                "label": {
                  "normal": "Verbindungen gesamt",
                  "beginner": "Verbindungen gesamt"
                }
              },
              "windows": {
                "label": {
                  "normal": "Fensterlayout geladen",
                  "beginner": "Fensterlayout geladen"
                }
              }
            },
            "workbench": {
              "types": {
                "addPiece": {
                  "label": {
                    "normal": "Bauteil hinzufügen",
                    "beginner": "Ein neues Bauteil dieses Typs zum Entwurf hinzufügen"
                  }
                },
                "duplicateType": {
                  "label": {
                    "normal": "Typ duplizieren",
                    "beginner": "Eine Kopie dieses Typs erstellen"
                  }
                }
              },
              "designs": {
                "addPiece": {
                  "label": {
                    "normal": "Bauteil hinzufügen",
                    "beginner": "Ein neues Bauteil dieses Entwurfs zum aktuellen Entwurf hinzufügen"
                  }
                }
              }
            }
          },
          "gridSize": {
            "label": {
              "normal": "Rastergröße",
              "beginner": "Rastergröße"
            }
          },
          "proximityConnectDistance": {
            "label": {
              "normal": "Näherungsverbindungs-Abstand",
              "beginner": "Näherungsverbindungs-Abstand"
            }
          },
          "selectOnlyPiecesOrConnections": {
            "label": {
              "normal": "Nur Bauteile oder Verbindungen auswählen",
              "beginner": "Nur Bauteile oder Verbindungen auswählen"
            }
          },
          "title": {
            "label": {
              "normal": "Titel",
              "beginner": "Titel"
            }
          },
          "tools": {
            "label": {
              "normal": "Werkzeuge",
              "beginner": "Werkzeuge"
            },
            "select": {
              "label": {
                "normal": "Auswählen",
                "beginner": "Auswahlwerkzeug"
              },
              "mode": {
                "additive": {
                  "label": {
                    "normal": "Additiv",
                    "beginner": "Additiver Auswahlmodus - zur vorhandenen Auswahl hinzufügen"
                  }
                },
                "subtractive": {
                  "label": {
                    "normal": "Subtraktiv",
                    "beginner": "Subtraktiver Auswahlmodus - von vorhandener Auswahl entfernen"
                  }
                },
                "intersect": {
                  "label": {
                    "normal": "Schnittmenge",
                    "beginner": "Schnittmengen-Auswahlmodus - nur überlappende Elemente auswählen"
                  }
                }
              },
              "shape": {
                "rectangular": {
                  "label": {
                    "normal": "Rechteckig",
                    "beginner": "Rechteckige Auswahl - ziehen Sie, um in einem Rechteck auszuwählen"
                  }
                },
                "lasso": {
                  "label": {
                    "normal": "Lasso",
                    "beginner": "Freiform-Lasso-Auswahl - zeichnen Sie eine Freiformform"
                  }
                }
              },
              "navigation": {
                "hand": {
                  "label": {
                    "normal": "Hand",
                    "beginner": "Hand-Werkzeug - die Leinwand schwenken und navigieren"
                  }
                }
              }
            },
            "lasso": {
              "rectangular": {
                "label": {
                  "normal": "Rechteck-Lasso",
                  "beginner": "Rechteck fuer Lasso-Auswahl ziehen"
                }
              },
              "freeform": {
                "label": {
                  "normal": "Freiform-Lasso",
                  "beginner": "Freihand-Lasso-Pfad zeichnen"
                }
              }
            }
          },
          "windows": {
            "label": {
              "normal": "Fenster",
              "beginner": "Fenster"
            }
          },
          "appTitle": {
            "label": {
              "normal": "App Title",
              "beginner": "App Title"
            }
          },
          "canvas": {
            "diagram": {
              "label": {
                "normal": "Diagramm",
                "beginner": "Diagramm"
              },
              "pieceNode": {
                "label": {
                  "normal": "Bauteil",
                  "beginner": "Diagramm-Bauteilknoten"
                }
              }
            },
            "label": {
              "normal": "Leinwand",
              "beginner": "Leinwand"
            }
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showPieces": {
              "label": {
                "normal": "Bauteile",
                "beginner": "Sichtbarkeit von Bauteilen umschalten"
              }
            },
            "showConnections": {
              "label": {
                "normal": "Verbindungen",
                "beginner": "Sichtbarkeit von Verbindungen zwischen Bauteilen umschalten"
              }
            },
            "showPorts": {
              "label": {
                "normal": "Anschlüsse",
                "beginner": "Sichtbarkeit von Anschlüssen an Bauteilen umschalten"
              }
            },
            "addPiece": {
              "label": {
                "normal": "Bauteil hinzufügen",
                "beginner": "Ein neues Bauteil zum Entwurf hinzufügen"
              }
            },
            "filters": {
              "label": {
                "normal": "Filter",
                "beginner": "Design-Elemente nach Sichtbarkeit filtern"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Entwurfs-Einstellungen"
            },
            "theme": {
              "label": {
                "normal": "Design",
                "beginner": "Wählen Sie das Farbschema für die Anwendung"
              }
            },
            "language": {
              "label": {
                "normal": "Sprache",
                "beginner": "Wählen Sie die Sprache für die Anwendungsoberfläche"
              },
              "placeholder": {
                "label": {
                  "normal": "Sprache wählen...",
                  "beginner": "Wählen Sie die Sprache, in der die Anwendung angezeigt wird"
                }
              }
            },
            "device": {
              "label": {
                "normal": "Gerät",
                "beginner": "Wählen Sie das Eingabegerät"
              }
            },
            "expertise": {
              "label": {
                "normal": "Erfahrung",
                "beginner": "Wählen Sie Ihr Erfahrungsniveau"
              }
            },
            "mode": {
              "label": {
                "normal": "Modus",
                "beginner": "Wählen Sie den Benutzeroberflächenmodus"
              }
            },
            "panel": {
              "label": {
                "normal": "Panels",
                "beginner": "Panel-Sichtbarkeit konfigurieren"
              },
              "toolbar": {
                "label": {
                  "normal": "Toolbar anzeigen",
                  "beginner": "Toolbar-Panel umschalten"
                }
              },
              "workbench": {
                "label": {
                  "normal": "Werkbank anzeigen",
                  "beginner": "Werkbank-Panel umschalten"
                }
              },
              "windows": {
                "label": {
                  "normal": "Fenster anzeigen",
                  "beginner": "Fenster-Panel umschalten"
                }
              },
              "details": {
                "label": {
                  "normal": "Details anzeigen",
                  "beginner": "Details-Panel umschalten"
                }
              }
            }
          }
        },
        "docs": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "noHeadings": {
            "label": {
              "normal": "Keine Ueberschriften gefunden",
              "beginner": "Keine Ueberschriften gefunden"
            }
          },
          "docs": {
            "label": {
              "normal": "Dokumentation",
              "beginner": "Dokumentation"
            }
          },
          "overview": {
            "label": {
              "normal": "Übersicht",
              "beginner": "Übersicht"
            }
          },
          "page": {
            "label": {
              "normal": "Seite",
              "beginner": "Seite"
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Einstellungen"
            },
            "theme": {
              "label": {
                "normal": "Design",
                "beginner": "Farbschema waehlen"
              }
            },
            "language": {
              "label": {
                "normal": "Sprache",
                "beginner": "Sprache auswaehlen"
              }
            },
            "device": {
              "label": {
                "normal": "Geraet",
                "beginner": "Eingabegeraet waehlen"
              }
            },
            "expertise": {
              "label": {
                "normal": "Erfahrung",
                "beginner": "Erfahrungsniveau waehlen"
              }
            },
            "mode": {
              "label": {
                "normal": "Modus",
                "beginner": "Benutzer- oder Entwicklermodus waehlen"
              }
            }
          },
          "navigation": {
            "previous": {
              "label": {
                "normal": "Zurück",
                "beginner": "Zur vorherigen Seite navigieren"
              }
            },
            "next": {
              "label": {
                "normal": "Weiter",
                "beginner": "Zur nächsten Seite navigieren"
              }
            }
          }
        },
        "feedback": {
          "label": {
            "normal": "Feedback",
            "beginner": "Feedback"
          },
          "form": {
            "label": {
              "normal": "Feedback-Formular",
              "beginner": "Teilen Sie Ihre Gedanken mit uns"
            }
          },
          "kind": {
            "label": {
              "normal": "Art",
              "beginner": "Welche Art von Feedback möchten Sie geben?"
            }
          },
          "bugReport": {
            "label": {
              "normal": "Fehlerbericht",
              "beginner": "Einen Fehler oder ein Problem melden"
            }
          },
          "featureIdea": {
            "label": {
              "normal": "Feature-Idee",
              "beginner": "Ein neues Feature oder eine Verbesserung vorschlagen"
            }
          },
          "title": {
            "label": {
              "normal": "Titel",
              "beginner": "Eine kurze Zusammenfassung Ihres Feedbacks"
            },
            "placeholder": {
              "label": {
                "normal": "Kurze Zusammenfassung eingeben...",
                "beginner": "Kurze Zusammenfassung eingeben..."
              }
            }
          },
          "description": {
            "label": {
              "normal": "Beschreibung",
              "beginner": "Detaillierte Beschreibung Ihres Feedbacks"
            },
            "bugPlaceholder": {
              "label": {
                "normal": "Beschreiben Sie, was passiert ist und wie es reproduziert werden kann...",
                "beginner": "Beschreiben Sie, was passiert ist und wie es reproduziert werden kann..."
              }
            },
            "ideaPlaceholder": {
              "label": {
                "normal": "Beschreiben Sie Ihre Feature-Idee oder Verbesserung...",
                "beginner": "Beschreiben Sie Ihre Feature-Idee oder Verbesserung..."
              }
            }
          },
          "app": {
            "label": {
              "normal": "App",
              "beginner": "In welcher App ist der Fehler aufgetreten?"
            },
            "placeholder": {
              "label": {
                "normal": "App auswählen...",
                "beginner": "App auswählen..."
              }
            },
            "options": {
              "home": {
                "label": {
                  "normal": "Home",
                  "beginner": "Home"
                }
              },
              "kit": {
                "label": {
                  "normal": "Kit",
                  "beginner": "Kit"
                }
              },
              "design": {
                "label": {
                  "normal": "Design",
                  "beginner": "Design"
                }
              },
              "type": {
                "label": {
                  "normal": "Typ",
                  "beginner": "Typ"
                }
              },
              "quality": {
                "label": {
                  "normal": "Qualität",
                  "beginner": "Qualität"
                }
              },
              "docs": {
                "label": {
                  "normal": "Dokumentation",
                  "beginner": "Dokumentation"
                }
              },
              "feedback": {
                "label": {
                  "normal": "Feedback",
                  "beginner": "Feedback"
                }
              }
            }
          },
          "name": {
            "label": {
              "normal": "Ihr Name",
              "beginner": "Optional: Ihr Name für Rückmeldungen"
            },
            "placeholder": {
              "label": {
                "normal": "Name (optional)",
                "beginner": "Name (optional)"
              }
            }
          },
          "email": {
            "label": {
              "normal": "E-Mail",
              "beginner": "Optional: Ihre E-Mail für Rückmeldungen"
            },
            "placeholder": {
              "label": {
                "normal": "email@beispiel.de (optional)",
                "beginner": "email@beispiel.de (optional)"
              }
            }
          },
          "submit": {
            "label": {
              "normal": "Feedback senden",
              "beginner": "Klicken, um Ihr Feedback zu senden"
            }
          },
          "submitting": {
            "label": {
              "normal": "Wird gesendet...",
              "beginner": "Ihr Feedback wird gesendet"
            }
          },
          "success": {
            "title": {
              "label": {
                "normal": "Vielen Dank!",
                "beginner": "Vielen Dank!"
              }
            },
            "message": {
              "label": {
                "normal": "Ihr Feedback wurde erfolgreich gesendet. Wir schätzen Ihren Beitrag zur Verbesserung von Compose.",
                "beginner": "Ihr Feedback wurde erfolgreich gesendet. Wir schätzen Ihren Beitrag zur Verbesserung von Compose."
              }
            },
            "sendAnother": {
              "label": {
                "normal": "Weiteres Feedback senden",
                "beginner": "Klicken, um ein weiteres Feedback zu senden"
              }
            }
          },
          "error": {
            "title": {
              "label": {
                "normal": "Fehler",
                "beginner": "Fehler"
              }
            },
            "message": {
              "label": {
                "normal": "Beim Senden Ihres Feedbacks ist ein Fehler aufgetreten. Bitte versuchen Sie es erneut.",
                "beginner": "Beim Senden Ihres Feedbacks ist ein Fehler aufgetreten. Bitte versuchen Sie es erneut."
              }
            },
            "retry": {
              "label": {
                "normal": "Erneut versuchen",
                "beginner": "Klicken, um erneut zu versuchen"
              }
            }
          },
          "validation": {
            "titleRequired": {
              "label": {
                "normal": "Titel ist erforderlich",
                "beginner": "Titel ist erforderlich"
              }
            },
            "descriptionRequired": {
              "label": {
                "normal": "Beschreibung ist erforderlich",
                "beginner": "Beschreibung ist erforderlich"
              }
            },
            "appRequired": {
              "label": {
                "normal": "Bitte wählen Sie die App aus, in der der Fehler aufgetreten ist",
                "beginner": "Bitte wählen Sie die App aus, in der der Fehler aufgetreten ist"
              }
            },
            "invalidEmail": {
              "label": {
                "normal": "Bitte geben Sie eine gültige E-Mail-Adresse ein",
                "beginner": "Bitte geben Sie eine gültige E-Mail-Adresse ein"
              }
            }
          },
          "toolbar": {
            "send": {
              "label": {
                "normal": "Senden",
                "beginner": "Feedback senden"
              }
            }
          }
        },
        "type": {
          "label": {
            "normal": "Typ",
            "beginner": "Typ"
          },
          "properties": {
            "label": {
              "normal": "Typ-Eigenschaften",
              "beginner": "Typ-Eigenschaften"
            }
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showConnectors": {
              "label": {
                "normal": "Connectors",
                "beginner": "Connectors"
              }
            },
            "showModels": {
              "label": {
                "normal": "Darstellungen",
                "beginner": "Darstellungen"
              }
            },
            "filters": {
              "label": {
                "normal": "Filter",
                "beginner": "Typ-Elemente nach Sichtbarkeit filtern"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "Neuer Typ",
              "beginner": "Neuer Typ"
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie diesen Typ...",
                "beginner": "Beschreiben Sie diesen Typ..."
              }
            }
          },
          "iconPlaceholder": {
            "label": {
              "label": {
                "normal": "??",
                "beginner": "??"
              }
            }
          },
          "imagePlaceholder": {
            "label": {
              "label": {
                "normal": "https://example.com/image.png",
                "beginner": "https://example.com/image.png"
              }
            }
          },
          "variantPlaceholder": {
            "label": {
              "label": {
                "normal": "z.B. gross, klein",
                "beginner": "z.B. gross, klein"
              }
            }
          },
          "parentPlaceholder": {
            "label": {
              "label": {
                "normal": "Elterntyp auswaehlen...",
                "beginner": "Elterntyp auswaehlen..."
              }
            }
          },
          "variant": {
            "label": {
              "normal": "Variante",
              "beginner": "Variante"
            }
          },
          "models": {
            "label": {
              "normal": "Darstellungen",
              "beginner": "Verschiedene 3D-Modelle, Bilder und visuelle Darstellungen fuer diesen Typ verwalten."
            },
            "manual": "type#models",
            "tutorial": "hello-compose/model-brick-set#models"
          },
          "model": {
            "label": {
              "normal": "Darstellung",
              "beginner": "Darstellung"
            }
          },
          "modelDescriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie diese Darstellung...",
                "beginner": "Beschreiben Sie diese Darstellung..."
              }
            }
          },
          "modelTagsPlaceholder": {
            "label": {
              "label": {
                "normal": "tag1, tag2, tag3",
                "beginner": "tag1, tag2, tag3"
              }
            }
          },
          "connectors": {
            "label": {
              "normal": "Connectors",
              "beginner": "Verbindungsports fuer diesen Typ verwalten. Connectors definieren, wo und wie Bauteile verbunden werden koennen."
            },
            "manual": "type#connectors",
            "tutorial": "hello-compose/model-brick-set#connectors"
          },
          "connector": {
            "label": {
              "normal": "Connector",
              "beginner": "Connector"
            },
            "properties": {
              "label": {
                "normal": "Connector-Eigenschaften",
                "beginner": "Connector-Eigenschaften"
              }
            },
            "title": {
              "label": {
                "normal": "Titel",
                "beginner": "Titel"
              }
            }
          },
          "connectorPortPlaceholder": {
            "label": {
              "label": {
                "normal": "z.B. elektrisch, mechanisch",
                "beginner": "z.B. elektrisch, mechanisch"
              }
            }
          },
          "connectorNamePlaceholder": {
            "label": {
              "label": {
                "normal": "Name hinzufuegen",
                "beginner": "Name hinzufuegen"
              }
            }
          },
          "connectorDescriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie diesen Connector...",
                "beginner": "Beschreiben Sie diesen Connector..."
              }
            }
          },
          "connectorPoint": {
            "label": {
              "normal": "Punkt",
              "beginner": "Die 3D-Position des Connectors in lokalen Koordinaten."
            },
            "manual": "type#connectors",
            "tutorial": "hello-compose/model-brick-set#connectors"
          },
          "connectorDirection": {
            "label": {
              "normal": "Richtung",
              "beginner": "Der Auswaerts-Richtungsvektor des Connectors in lokalen Koordinaten."
            },
            "manual": "type#connectors",
            "tutorial": "hello-compose/model-brick-set#connectors"
          },
          "connectorCompatiblePortsPlaceholder": {
            "label": {
              "label": {
                "normal": "familie1, familie2",
                "beginner": "familie1, familie2"
              }
            }
          },
          "connectorNotFound": {
            "label": {
              "normal": "Connector nicht gefunden",
              "beginner": "Connector nicht gefunden"
            }
          },
          "connectorsNotFound": {
            "label": {
              "normal": "Keine Connectors gefunden",
              "beginner": "Keine Connectors gefunden"
            }
          },
          "authors": {
            "label": {
              "normal": "Autoren",
              "beginner": "Autoren"
            }
          },
          "author": {
            "label": {
              "normal": "Autor",
              "beginner": "Autor"
            }
          },
          "attributes": {
            "label": {
              "normal": "Attribute",
              "beginner": "Attribute"
            }
          },
          "attribute": {
            "label": {
              "normal": "Attribut",
              "beginner": "Attribut"
            }
          },
          "attributeValuePlaceholder": {
            "label": {
              "label": {
                "normal": "Wert...",
                "beginner": "Wert..."
              }
            }
          },
          "attributeDefinitionPlaceholder": {
            "label": {
              "label": {
                "normal": "Definition oder URL...",
                "beginner": "Definition oder URL..."
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "type": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Name des Typs. Dies ist die primaere Kennung fuer die Komponente."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine detaillierte Beschreibung dessen, was dieser Typ darstellt und wie er verwendet werden sollte."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "icon": {
                    "label": {
                      "normal": "Symbol",
                      "beginner": "Ein Icon zur visuellen Darstellung dieses Typs. Kann ein Emoji, Iconname oder URL zu einem Bild sein."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "image": {
                    "label": {
                      "normal": "Bild",
                      "beginner": "URL zu einem Bild, das diesen Typ darstellt. Wird fuer Vorschauen und visuelle Identifikation verwendet."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "unit": {
                    "label": {
                      "normal": "Einheit",
                      "beginner": "Die Masseinheit fuer diesen Typ (z.B. mm, m, ft)."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "parent": {
                    "label": {
                      "normal": "Elterntyp",
                      "beginner": "Der Elterntyp, von dem dieser Typ erbt"
                    }
                  },
                  "abstract": {
                    "label": {
                      "normal": "Abstrakt",
                      "beginner": "Ob dies ein abstrakter Typ ist"
                    }
                  }
                },
                "models": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "url": {
                    "label": {
                      "normal": "URL",
                      "beginner": "URL zu einem 3D-Modell, Bild oder einer anderen Ressource, die diesen Typ darstellt."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung dessen, was diese Darstellung zeigt oder wie sie verwendet werden sollte."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "tags": {
                    "label": {
                      "normal": "Tags",
                      "beginner": "Tags zum Kategorisieren und Filtern von Darstellungen (z.B. 'detailliert', 'vereinfacht', 'lod1')."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-compose/model-brick-set"
                  }
                },
                "connectors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "port": {
                    "label": {
                      "normal": "Familie",
                      "beginner": "Connector-Familienname. Connectors derselben Familie koennen miteinander verbunden werden."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "compatiblePorts": {
                    "label": {
                      "normal": "Kompatible Familien",
                      "beginner": "Liste anderer Connector-Familien, mit denen dieser Connector verbunden werden kann. Leer lassen, um alle Familien zuzulassen."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung dessen, was dieser Connector darstellt und wie er fuer Verbindungen verwendet werden sollte."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "t": {
                    "label": {
                      "normal": "T",
                      "beginner": "Position auf dem Diagrammring (0-1). Steuert, wo der Connector in der 2D-Diagrammansicht erscheint."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "ring": {
                    "label": {
                      "normal": "Ring",
                      "beginner": "Position auf dem Diagrammring (0-1). Steuert, wo der Connector in der 2D-Diagrammansicht erscheint."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "direction": {
                    "label": {
                      "normal": "",
                      "beginner": ""
                    },
                    "x": {
                      "label": {
                        "normal": "X",
                        "beginner": "X-Koordinate des Connector-Richtungsvektors. Dies definiert, in welche Richtung der Connector im 3D-Raum zeigt."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    },
                    "y": {
                      "label": {
                        "normal": "Y",
                        "beginner": "Y-Koordinate des Connector-Richtungsvektors. Dies definiert, in welche Richtung der Connector im 3D-Raum zeigt."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    },
                    "z": {
                      "label": {
                        "normal": "Z",
                        "beginner": "Z-Koordinate des Connector-Richtungsvektors. Dies definiert, in welche Richtung der Connector im 3D-Raum zeigt."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Ein optionaler Name fuer diesen Connector."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "mandatory": {
                    "label": {
                      "normal": "Pflichtfeld",
                      "beginner": "Ob dieser Connector fuer eine gueltige Verbindung zwingend erforderlich ist."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "maxChildren": {
                    "label": {
                      "normal": "Max. Kinder",
                      "beginner": "Die maximale Anzahl von Verbindungen, die an diesem Connector erlaubt sind."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "point": {
                    "label": {
                      "normal": "",
                      "beginner": ""
                    },
                    "x": {
                      "label": {
                        "normal": "X",
                        "beginner": "X-Position des Connectors im 3D-Raum relativ zum Ursprung des Typs."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    },
                    "y": {
                      "label": {
                        "normal": "Y",
                        "beginner": "Y-Position des Connectors im 3D-Raum relativ zum Ursprung des Typs."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    },
                    "z": {
                      "label": {
                        "normal": "Z",
                        "beginner": "Z-Position des Connectors im 3D-Raum relativ zum Ursprung des Typs."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    }
                  }
                },
                "connector": {
                  "ring": {
                    "label": {
                      "normal": "Ring",
                      "beginner": "Position auf dem Diagrammring (0-1). Steuert, wo der Connector in der 2D-Diagrammansicht erscheint."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  }
                },
                "attributes": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Name des Attributs im Kebab-Case (z.B. 'material.holz', 'kosten.arbeit')."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "value": {
                    "label": {
                      "normal": "Wert",
                      "beginner": "Der Wert des Attributs. Leer lassen fuer boolesche Attribute (Anwesenheit = wahr)."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "definition": {
                    "label": {
                      "normal": "Definition",
                      "beginner": "Optionale Definition oder Dokumentation fuer dieses Attribut. Kann Text oder eine URL sein."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-compose/model-brick-set"
                  }
                },
                "authors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Vollstaendiger Name des Autors oder Mitwirkenden."
                    },
                    "manual": "type#authors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "email": {
                    "label": {
                      "normal": "E-Mail",
                      "beginner": "E-Mail-Adresse zur Kontaktaufnahme mit dem Autor."
                    },
                    "manual": "type#authors",
                    "tutorial": "hello-compose/model-brick-set"
                  }
                }
              }
            }
          },
          "footer": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "someAction": {
              "label": {
                "normal": "Some Action",
                "beginner": "Some Action"
              }
            }
          },
          "tools": {
            "label": {
              "normal": "Werkzeuge",
              "beginner": "Werkzeuge"
            },
            "select": {
              "normal": {
                "label": {
                  "normal": "Normal",
                  "beginner": "Normale Auswahl"
                }
              },
              "additive": {
                "label": {
                  "normal": "Additiv",
                  "beginner": "Additive Auswahl - zur bestehenden Auswahl hinzufügen"
                }
              },
              "subtractive": {
                "label": {
                  "normal": "Subtraktiv",
                  "beginner": "Subtraktive Auswahl - von bestehender Auswahl entfernen"
                }
              },
              "intersect": {
                "label": {
                  "normal": "Schnittmenge",
                  "beginner": "Schnittmengen-Auswahl - nur überlappende Elemente auswählen"
                }
              }
            },
            "lasso": {
              "rectangular": {
                "label": {
                  "normal": "Rechteckig",
                  "beginner": "Rechteckige Lasso-Auswahl"
                }
              },
              "freeform": {
                "label": {
                  "normal": "Freihand",
                  "beginner": "Freihand-Lasso-Auswahl"
                }
              }
            },
            "hand": {
              "label": {
                "normal": "Hand",
                "beginner": "Hand-Werkzeug - Schwenken und Navigieren"
              }
            },
            "connector": {
              "label": {
                "normal": "Konnektor",
                "beginner": "Konnektor-Erstellungswerkzeug"
              }
            },
            "selection": {
              "label": {
                "normal": "Auswahl",
                "beginner": "Auswahlwerkzeug"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Titel",
              "beginner": "Titel"
            }
          }
        },
        "quality": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "description": {
            "label": {
              "normal": "Messqualitaeten definieren",
              "beginner": "Messqualitaeten definieren"
            }
          },
          "tools": {
            "select": {
              "additive": {
                "label": {
                  "normal": "Additiv",
                  "beginner": "Additive Auswahl - zur bestehenden Auswahl hinzufügen"
                }
              },
              "subtractive": {
                "label": {
                  "normal": "Subtraktiv",
                  "beginner": "Subtraktive Auswahl - von bestehender Auswahl entfernen"
                }
              },
              "intersect": {
                "label": {
                  "normal": "Schnittmenge",
                  "beginner": "Schnittmengen-Auswahl - nur überlappende Elemente auswählen"
                }
              }
            },
            "selection": {
              "label": {
                "normal": "Auswahl",
                "beginner": "Auswahlwerkzeug"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "Neue Qualitaet",
              "beginner": "Neue Qualitaet"
            }
          },
          "numericFunctions": {
            "label": {
              "normal": "Numerische Funktionen",
              "beginner": "Numerische Funktionen"
            }
          },
          "add": {
            "label": {
              "normal": "Addieren",
              "beginner": "Addieren"
            }
          },
          "subtract": {
            "label": {
              "normal": "Subtrahieren",
              "beginner": "Subtrahieren"
            }
          },
          "multiply": {
            "label": {
              "normal": "Multiplizieren",
              "beginner": "Multiplizieren"
            }
          },
          "divide": {
            "label": {
              "normal": "Dividieren",
              "beginner": "Dividieren"
            }
          },
          "branchingFunctions": {
            "label": {
              "normal": "Verzweigungsfunktionen",
              "beginner": "Verzweigungsfunktionen"
            }
          },
          "if": {
            "label": {
              "normal": "Wenn",
              "beginner": "Wenn"
            }
          },
          "switch": {
            "label": {
              "normal": "Schalter",
              "beginner": "Schalter"
            }
          },
          "dataStructures": {
            "label": {
              "normal": "Datenstrukturen",
              "beginner": "Datenstrukturen"
            }
          },
          "list": {
            "label": {
              "normal": "Liste",
              "beginner": "Liste"
            }
          },
          "dictionary": {
            "label": {
              "normal": "Woerterbuch",
              "beginner": "Woerterbuch"
            }
          },
          "noQualities": {
            "label": {
              "normal": "Keine Qualitaeten definiert",
              "beginner": "Keine Qualitaeten definiert"
            }
          },
          "key": {
            "label": {
              "normal": "Schluessel",
              "beginner": "Der eindeutige Bezeichner fuer diese Qualitaet"
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Der Anzeigename fuer diese Qualitaet"
            }
          },
          "kind": {
            "label": {
              "normal": "Art",
              "beginner": "Der Entitaetstyp, auf den diese Qualitaet anwendbar ist"
            }
          },
          "formula": {
            "label": {
              "normal": "Formel",
              "beginner": "Die Formel zur Berechnung dieser Qualitaet"
            }
          },
          "formulaPlaceholder": "Formel eingeben...",
          "defaultValue": {
            "label": {
              "normal": "Standardwert",
              "beginner": "Der Standardwert fuer diese Qualitaet"
            }
          },
          "defaultSiUnit": {
            "label": {
              "normal": "Standard-SI-Einheit",
              "beginner": "Die Standardeinheit im SI-System"
            }
          },
          "defaultImperialUnit": {
            "label": {
              "normal": "Standard-Imperial-Einheit",
              "beginner": "Die Standardeinheit im Imperial-System"
            }
          },
          "min": {
            "label": {
              "normal": "Minimum",
              "beginner": "Der minimal zulaessige Wert"
            }
          },
          "isMinExcluded": {
            "label": {
              "normal": "Minimum ausschliessen",
              "beginner": "Ob der Minimalwert ausgeschlossen ist"
            }
          },
          "max": {
            "label": {
              "normal": "Maximum",
              "beginner": "Der maximal zulaessige Wert"
            }
          },
          "isMaxExcluded": {
            "label": {
              "normal": "Maximum ausschliessen",
              "beginner": "Ob der Maximalwert ausgeschlossen ist"
            }
          },
          "canScale": {
            "label": {
              "normal": "Kann skalieren",
              "beginner": "Ob diese Qualitaet mit der Teilegroesse skaliert"
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "key": {
                "label": {
                  "normal": "Schlüssel",
                  "beginner": "Die eindeutige Schlüsselkennung der Qualität"
                }
              },
              "name": {
                "label": {
                  "normal": "Name",
                  "beginner": "Der Anzeigename der Qualität"
                }
              },
              "description": {
                "label": {
                  "normal": "Beschreibung",
                  "beginner": "Eine Beschreibung dessen, was diese Qualität misst"
                }
              },
              "formula": {
                "label": {
                  "normal": "Formel",
                  "beginner": "Die Formel zur Berechnung dieser Qualität"
                }
              },
              "defaultSiUnit": {
                "label": {
                  "normal": "Standard-SI-Einheit",
                  "beginner": "Die Standardeinheit im SI-System"
                }
              },
              "defaultImperialUnit": {
                "label": {
                  "normal": "Standard-Imperial-Einheit",
                  "beginner": "Die Standardeinheit im imperialen System"
                }
              },
              "kind": {
                "label": {
                  "normal": "Art",
                  "beginner": "Der Entitätstyp, auf den diese Qualität anwendbar ist"
                }
              },
              "canScale": {
                "label": {
                  "normal": "Skalierbar",
                  "beginner": "Ob diese Qualität mit der Teilgröße skaliert"
                }
              },
              "defaultValue": {
                "label": {
                  "normal": "Standardwert",
                  "beginner": "Der Standardwert für diese Qualität"
                }
              },
              "min": {
                "label": {
                  "normal": "Minimum",
                  "beginner": "Der minimal zulässige Wert"
                }
              },
              "max": {
                "label": {
                  "normal": "Maximum",
                  "beginner": "Der maximal zulässige Wert"
                }
              },
              "isMinExcluded": {
                "label": {
                  "normal": "Minimum ausgeschlossen",
                  "beginner": "Ob der Minimalwert exklusiv ist"
                }
              },
              "isMaxExcluded": {
                "label": {
                  "normal": "Maximum ausgeschlossen",
                  "beginner": "Ob der Maximalwert exklusiv ist"
                }
              }
            }
          },
          "title": {
            "label": {
              "normal": "Qualität",
              "beginner": "Qualität"
            }
          },
          "functions": {
            "label": {
              "normal": "Funktionen",
              "beginner": "Funktionen"
            }
          },
          "qualities": {
            "label": {
              "normal": "Qualitäten",
              "beginner": "Qualitäten"
            }
          },
          "toolbar": {
            "view": {
              "label": {
                "normal": "Ansicht",
                "beginner": "Ansichtsoptionen"
              }
            },
            "actions": {
              "label": {
                "normal": "Aktionen",
                "beginner": "Qualitätsaktionen"
              }
            }
          },
          "workbench": {
            "nodes": {
              "label": {
                "normal": "Knoten",
                "beginner": "Qualitätsformel-Knoten"
              }
            },
            "qualities": {
              "label": {
                "normal": "Qualitäten",
                "beginner": "Verfügbare Qualitäten"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Qualitäts-Einstellungen"
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Qualitäts-Chat"
            }
          }
        }
      },
      "settings": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "theme": {
          "label": {
            "normal": "Design",
            "beginner": "Farbschema für die Oberfläche"
          },
          "system": {
            "label": {
              "normal": "System",
              "beginner": "System"
            }
          },
          "light": {
            "label": {
              "normal": "Hell",
              "beginner": "Hell"
            }
          },
          "dark": {
            "label": {
              "normal": "Dunkel",
              "beginner": "Dunkel"
            }
          }
        },
        "device": {
          "label": {
            "normal": "Geraet",
            "beginner": "Layoutmodus für die Oberfläche"
          },
          "desktop": {
            "label": {
              "normal": "Desktop",
              "beginner": "Desktop"
            }
          },
          "tablet": {
            "label": {
              "normal": "Tablet",
              "beginner": "Tablet"
            }
          }
        },
        "mode": {
          "label": {
            "normal": "Modus",
            "beginner": "Oberflächenmodus"
          },
          "user": {
            "label": {
              "normal": "Benutzer",
              "beginner": "Benutzer"
            }
          },
          "dev": {
            "label": {
              "normal": "Dev",
              "beginner": "Dev"
            }
          }
        },
        "expertise": {
          "label": {
            "normal": "Erfahrung",
            "beginner": "Ihre Erfahrungsstufe"
          },
          "beginner": {
            "label": {
              "normal": "Anfänger",
              "beginner": "Detaillierte Hilfe und Tutorials anzeigen"
            }
          },
          "normal": {
            "label": {
              "normal": "Normal",
              "beginner": "Standard-Tooltips anzeigen"
            }
          },
          "expert": {
            "label": {
              "normal": "Experte",
              "beginner": "Experte"
            }
          }
        },
        "language": {
          "label": {
            "normal": "Sprache",
            "beginner": "Waehlen Sie die Sprache fuer die Anwendungsoberflaeche"
          },
          "placeholder": {
            "label": {
              "normal": "Sprache waehlen...",
              "beginner": "Waehlen Sie die Sprache, in der die Anwendung angezeigt wird"
            }
          },
          "de": {
            "label": {
              "normal": "Deutsch",
              "beginner": "Deutsch"
            }
          },
          "en": {
            "label": {
              "normal": "Englisch",
              "beginner": "Englisch"
            }
          }
        }
      },
      "tool": {
        "label": {
          "normal": "Werkzeuge",
          "beginner": "Werkzeuge"
        },
        "selection": {
          "label": {
            "normal": "Auswahl",
            "beginner": "Auswahl"
          },
          "normal": {
            "label": {
              "normal": "Normale Auswahl",
              "beginner": "Klicken Sie, um jeweils ein Element auszuwählen"
            },
            "manual": "selection",
            "hotkey": "1"
          },
          "additive": {
            "label": {
              "normal": "Zur Auswahl hinzufügen",
              "beginner": "Klicken Sie, um Elemente zur Auswahl hinzuzufügen, ohne Strg zu halten"
            },
            "manual": "selection",
            "hotkey": "2"
          },
          "subtractive": {
            "label": {
              "normal": "Von Auswahl entfernen",
              "beginner": "Klicken Sie, um Elemente aus der Auswahl zu entfernen, ohne Alt zu halten"
            },
            "manual": "selection",
            "hotkey": "3"
          }
        }
      },
      "sort": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "ascending": {
          "label": {
            "normal": "Aufsteigend",
            "beginner": "Aufsteigend"
          }
        },
        "descending": {
          "label": {
            "normal": "Absteigend",
            "beginner": "Absteigend"
          }
        }
      },
      "docs": {
        "navigation": {
          "previous": {
            "label": {
              "normal": "Previous",
              "beginner": "Previous"
            }
          },
          "next": {
            "label": {
              "normal": "Next",
              "beginner": "Next"
            }
          }
        }
      },
      "toolbar": {
        "label": {
          "normal": "Werkzeugleiste",
          "beginner": "Werkzeugleiste"
        },
        "group": {
          "hand": {
            "label": {
              "normal": "Hand",
              "beginner": "Hand"
            }
          },
          "selection": {
            "label": {
              "normal": "Auswahl",
              "beginner": "Auswahl"
            }
          },
          "lasso": {
            "label": {
              "normal": "Lasso",
              "beginner": "Lasso"
            }
          },
          "filter": {
            "label": {
              "normal": "Filter",
              "beginner": "Filter"
            }
          },
          "open": {
            "label": {
              "normal": "Öffnen",
              "beginner": "Öffnen"
            }
          },
          "create": {
            "label": {
              "normal": "Erstellen",
              "beginner": "Erstellen"
            }
          },
          "view": {
            "label": {
              "normal": "Ansicht",
              "beginner": "Ansicht"
            }
          },
          "actions": {
            "label": {
              "normal": "Aktionen",
              "beginner": "Aktionen"
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Einstellungen"
            }
          }
        },
        "parent": {
          "selection": {
            "label": {
              "normal": "Auswahl",
              "beginner": "Auswahl"
            }
          },
          "hand": {
            "label": {
              "normal": "Hand",
              "beginner": "Hand"
            }
          },
          "lasso": {
            "label": {
              "normal": "Lasso",
              "beginner": "Lasso"
            }
          },
          "filter": {
            "label": {
              "normal": "Filter",
              "beginner": "Filter"
            }
          },
          "open": {
            "label": {
              "normal": "Öffnen",
              "beginner": "Öffnen"
            }
          },
          "create": {
            "label": {
              "normal": "Erstellen",
              "beginner": "Erstellen"
            }
          },
          "view": {
            "label": {
              "normal": "Ansicht",
              "beginner": "Ansicht"
            }
          },
          "actions": {
            "label": {
              "normal": "Aktionen",
              "beginner": "Aktionen"
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Einstellungen"
            }
          }
        }
      },
      "tutorial": {
        "controls": {
          "stop": {
            "label": {
              "normal": "Stop",
              "beginner": "Stop"
            }
          },
          "previous": {
            "label": {
              "normal": "Previous",
              "beginner": "Previous"
            }
          },
          "playPause": {
            "label": {
              "normal": "Play Pause",
              "beginner": "Play Pause"
            }
          },
          "next": {
            "label": {
              "normal": "Next",
              "beginner": "Next"
            }
          }
        }
      },
      "recording": {
        "controls": {
          "playPause": {
            "label": {
              "normal": "Play Pause",
              "beginner": "Play Pause"
            }
          },
          "stop": {
            "label": {
              "normal": "Stop",
              "beginner": "Stop"
            }
          }
        }
      }
    }
  }
}`),
  },
  en: {
    translation: JSON.parse(String.raw`{
  "compose": {
    "label": {
      "normal": "",
      "beginner": ""
    },
    "file": {
      "name": "Name",
      "size": "Size",
      "created": "Created",
      "updated": "Updated"
    },
    "folder": {
      "created": "Created",
      "updated": "Updated"
    },
    "sketchpad": {
      "label": {
        "normal": "",
        "beginner": ""
      },
      "navbar": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "back": {
          "label": {
            "normal": "Go back",
            "beginner": "Click to go back, hold to see history"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Left"
        },
        "forward": {
          "label": {
            "normal": "Go forward",
            "beginner": "Click to go forward, hold to see history"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Right"
        },
        "up": {
          "label": {
            "normal": "Go up one level",
            "beginner": "Click to go up one level in the navigation hierarchy"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Up"
        },
        "kits": {
          "label": {
            "normal": "Kits",
            "beginner": "Click to see all kits"
          }
        },
        "navigationButtons": {
          "label": {
            "normal": "Navigation",
            "beginner": "Navigation buttons"
          }
        },
        "docs": {
          "label": {
            "normal": "Documentation",
            "beginner": "Click to view documentation"
          },
          "hotkey": "Ctrl+Shift+D"
        },
        "search": {
          "label": {
            "normal": "Search",
            "beginner": "Search for content"
          },
          "open": {
            "label": {
              "normal": "Search",
              "beginner": "Click to open search to quickly find and navigate to any element"
            },
            "manual": "navigation#search",
            "tutorial": "getting-started/intro#search",
            "hotkey": "Ctrl+K"
          },
          "close": {
            "label": {
              "normal": "Close Search",
              "beginner": "Click to close the search dialog"
            },
            "manual": "navigation#search",
            "tutorial": "getting-started/intro#search",
            "hotkey": "Escape"
          },
          "title": {
            "label": {
              "normal": "Search",
              "beginner": "Search"
            }
          },
          "description": {
            "label": {
              "normal": "Search for kits, designs, types, and more",
              "beginner": "Search for kits, designs, types, and more"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Search...",
              "beginner": "Search..."
            }
          },
          "noResults": {
            "label": {
              "normal": "No results found",
              "beginner": "No results found"
            }
          }
        },
        "find": {
          "label": {
            "normal": "Find",
            "beginner": "Find items in the current view"
          },
          "open": {
            "label": {
              "normal": "Find",
              "beginner": "Click to find and jump to items in the current app view"
            },
            "hotkey": "Ctrl+F"
          },
          "close": {
            "label": {
              "normal": "Close Find",
              "beginner": "Click to close the find dialog"
            },
            "hotkey": "Escape"
          },
          "title": {
            "label": {
              "normal": "Find",
              "beginner": "Find"
            }
          },
          "description": {
            "label": {
              "normal": "Find items in this view",
              "beginner": "Find items in this view"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Find...",
              "beginner": "Find..."
            }
          },
          "noResults": {
            "label": {
              "normal": "No results found",
              "beginner": "No results found"
            }
          }
        },
        "focus": {
          "label": {
            "normal": "Focus Mode",
            "beginner": "Toggle focus mode to hide distractions"
          },
          "open": {
            "label": {
              "normal": "Focus",
              "beginner": "Click to enter focus mode and hide distractions"
            },
            "manual": "navigation#focus",
            "tutorial": "getting-started/intro#focus",
            "hotkey": "Ctrl+Shift+F"
          },
          "close": {
            "label": {
              "normal": "Exit Focus",
              "beginner": "Click to exit focus mode and show all UI elements"
            },
            "manual": "navigation#focus",
            "tutorial": "getting-started/intro#focus",
            "hotkey": "Escape"
          },
          "input": {
            "label": {
              "normal": "Focus Input",
              "beginner": "Type to search for an element to focus on"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Search for an element...",
              "beginner": "Search for an element..."
            }
          },
          "title": {
            "label": {
              "normal": "Focus",
              "beginner": "Focus"
            }
          },
          "description": {
            "label": {
              "normal": "Focus on an element in the current view",
              "beginner": "Focus on an element in the current view"
            }
          },
          "other": {
            "label": {
              "normal": "Other",
              "beginner": "Other"
            }
          }
        },
        "copyJsonToClipboard": {
          "label": {
            "normal": "Copy JSON",
            "beginner": "Copy the current sketchpad JSON state to clipboard"
          },
          "hotkey": "Ctrl+Shift+J"
        },
        "breadcrumb": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "designs": {
            "label": {
              "normal": "Designs",
              "beginner": "Designs"
            }
          },
          "types": {
            "label": {
              "normal": "Types",
              "beginner": "Types"
            }
          },
          "qualities": {
            "label": {
              "normal": "Qualities",
              "beginner": "Qualities"
            }
          },
          "temporary": {
            "label": {
              "normal": "Temporary",
              "beginner": "Temporary"
            }
          },
          "local": {
            "label": {
              "normal": "Local",
              "beginner": "Local"
            }
          },
          "remote": {
            "label": {
              "normal": "Remote",
              "beginner": "Remote"
            }
          },
          "files": {
            "label": {
              "normal": "Files",
              "beginner": "Files"
            }
          },
          "authors": {
            "label": {
              "normal": "Authors",
              "beginner": "Authors"
            }
          }
        },
        "tutorials": {
          "label": {
            "normal": "Tutorials",
            "beginner": "Tutorials"
          }
        },
        "tutorial": {
          "controls": {
            "stop": {
              "label": {
                "normal": "Stop Tutorial",
                "beginner": "Click to stop the current tutorial"
              }
            },
            "previous": {
              "label": {
                "normal": "Previous Step",
                "beginner": "Go to the previous step in the tutorial"
              }
            },
            "playPause": {
              "label": {
                "normal": "Play/Pause",
                "beginner": "Play or pause the tutorial"
              }
            },
            "next": {
              "label": {
                "normal": "Next Step",
                "beginner": "Go to the next step in the tutorial"
              }
            }
          }
        },
        "recording": {
          "controls": {
            "playPause": {
              "label": {
                "normal": "Play/Pause Recording",
                "beginner": "Play or pause the recording"
              }
            },
            "stop": {
              "label": {
                "normal": "Stop Recording",
                "beginner": "Stop the recording and save it"
              }
            }
          }
        },
        "createKit": {
          "label": {
            "normal": "Create Kit",
            "beginner": "Click to create a new kit"
          }
        },
        "createDesign": {
          "label": {
            "normal": "Create Design",
            "beginner": "Click to create a new design"
          }
        },
        "createChild": {
          "label": {
            "normal": "Create Child",
            "beginner": "Click to create a child artifact"
          }
        },
        "createType": {
          "label": {
            "normal": "Create Type",
            "beginner": "Click to create a new type"
          }
        },
        "createVersion": {
          "label": {
            "normal": "Create Version",
            "beginner": "Click to create a new version"
          }
        },
        "searchInput": {
          "label": {
            "normal": "Search Input",
            "beginner": "Type to search for elements"
          }
        },
        "panelToggle": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "workbench": {
            "label": {
              "normal": "Workbench",
              "beginner": "Show or hide the workbench panel on the left"
            },
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "hud": {
            "label": {
              "normal": "Toggle HUD",
              "beginner": "Toggle the HUD panel in the middle"
            },
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "right": {
            "label": {
              "normal": "Toggle Right Panel",
              "beginner": "Toggle the right panel for details and settings"
            }
          },
          "tools": {
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "toolbar": {
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "stats": {
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "details": {
            "label": {
              "normal": "Details",
              "beginner": "Show or hide the details panel on the right"
            },
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "chat": {
            "label": {
              "normal": "Toggle Chat",
              "beginner": "Toggle the chat panel"
            },
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "console": {
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Toggle Settings",
              "beginner": "Toggle the settings panel"
            },
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "leftSidePanel": {
            "label": {
              "normal": "Toggle Left Panel",
              "beginner": "Toggle the left side panel with workbench tabs"
            }
          },
          "rightSidePanel": {
            "label": {
              "normal": "Toggle Right Panel",
              "beginner": "Toggle the right side panel with details tabs"
            }
          },
          "hudPanel": {
            "label": {
              "normal": "Toggle HUD Panel",
              "beginner": "Toggle the center HUD panel"
            }
          }
        },
        "home": {
          "label": {
            "normal": "Home",
            "beginner": "Home"
          }
        },
        "kitName": {
          "label": {
            "normal": "Kit Name",
            "beginner": "Kit Name"
          }
        },
        "kitVersion": {
          "label": {
            "normal": "Kit Version",
            "beginner": "Kit Version"
          }
        },
        "name": {
          "label": {
            "normal": "Name",
            "beginner": "Name"
          }
        },
        "design": {
          "label": {
            "normal": "Design",
            "beginner": "Design"
          }
        },
        "type": {
          "label": {
            "normal": "Type",
            "beginner": "Type"
          }
        },
        "quality": {
          "label": {
            "normal": "Quality",
            "beginner": "Quality"
          }
        },
        "navigation": {
          "label": {
            "normal": "Navigation",
            "beginner": "Navigation"
          }
        },
        "panelToggles": {
          "label": {
            "normal": "Panel Toggles",
            "beginner": "Panel Toggles"
          }
        },
        "fullscreenToggle": {
          "label": {
            "normal": "Fullscreen Toggle",
            "beginner": "Fullscreen Toggle"
          }
        }
      },
      "panel": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "chat": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "placeholder": {
            "label": {
              "normal": "Ask anything...",
              "beginner": "Ask anything..."
            }
          }
        },
        "settings": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "theme": {
            "label": {
              "normal": "Theme",
              "beginner": "Choose the color theme for the application"
            },
            "dark": {
              "label": {
                "normal": "Dark",
                "beginner": "Use dark color scheme"
              }
            },
            "light": {
              "label": {
                "normal": "Light",
                "beginner": "Use light color scheme"
              }
            },
            "system": {
              "label": {
                "normal": "System",
                "beginner": "Follow system theme preference"
              }
            }
          },
          "device": {
            "label": {
              "normal": "Device",
              "beginner": "Choose the device mode for interaction"
            },
            "desktop": {
              "label": {
                "normal": "Desktop",
                "beginner": "Optimized for mouse and keyboard"
              }
            },
            "tablet": {
              "label": {
                "normal": "Tablet",
                "beginner": "Optimized for touch interaction"
              }
            },
            "mobile": {
              "label": {
                "normal": "Mobile",
                "beginner": "Optimized for touch interaction on small screens"
              }
            }
          },
          "mode": {
            "label": {
              "normal": "Mode",
              "beginner": "Select the user port mode: Expert (minimal tooltips), Normal (standard), or Beginner (detailed help)"
            },
            "dev": {
              "label": {
                "normal": "Developer",
                "beginner": "Developer mode with advanced tools and debugging features"
              }
            },
            "user": {
              "label": {
                "normal": "User",
                "beginner": "Standard user mode for regular operations"
              }
            }
          },
          "expertise": {
            "label": {
              "normal": "Expertise",
              "beginner": "Select your expertise level to adjust the port complexity"
            },
            "beginner": {
              "label": {
                "normal": "Beginner",
                "beginner": "Show detailed explanations and tutorials"
              }
            },
            "normal": {
              "label": {
                "normal": "Normal",
                "beginner": "Show standard tooltips and help"
              }
            },
            "expert": {
              "label": {
                "normal": "Expert",
                "beginner": "Minimal tooltips for experienced users"
              }
            }
          }
        }
      },
      "common": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "selectVariant": "Select variant...",
        "selectView": "Select view...",
        "search": {
          "label": {
            "normal": "Search",
            "beginner": "Search"
          }
        },
        "mixedValues": {
          "label": {
            "normal": "Mixed values",
            "beginner": "Mixed values"
          }
        },
        "selectDesign": {
          "label": {
            "normal": "Select design",
            "beginner": "Select design"
          }
        },
        "selectType": {
          "label": {
            "normal": "Select type",
            "beginner": "Select type"
          }
        },
        "settings": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "theme": {
            "label": {
              "normal": "Theme",
              "beginner": "Choose the color theme"
            }
          },
          "layout": {
            "label": {
              "normal": "Layout",
              "beginner": "Choose the layout mode"
            }
          },
          "mode": {
            "label": {
              "normal": "Mode",
              "beginner": "Choose the port mode"
            }
          },
          "expertise": {
            "label": {
              "normal": "Expertise Level",
              "beginner": "Choose your expertise level"
            }
          }
        },
        "no": {
          "label": {
            "normal": "No",
            "beginner": "No"
          }
        },
        "yes": {
          "label": {
            "normal": "Yes",
            "beginner": "Yes"
          }
        },
        "add": {
          "label": {
            "normal": "Add",
            "beginner": "Add"
          }
        },
        "remove": {
          "label": {
            "normal": "Remove",
            "beginner": "Remove"
          }
        },
        "addChild": {
          "label": {
            "normal": "Add Child",
            "beginner": "Add Child"
          }
        },
        "duplicateType": {
          "label": {
            "normal": "Duplicate Type by Hover",
            "beginner": "Duplicate Type by Hover"
          }
        },
        "addType": {
          "label": {
            "normal": "Add Type",
            "beginner": "Add Type"
          }
        },
        "addDesign": {
          "label": {
            "normal": "Add Design",
            "beginner": "Add Design"
          }
        }
      },
      "footer": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "feedback": {
          "label": {
            "normal": "Feedback",
            "beginner": "Send feedback to help improve Compose"
          }
        }
      },
      "settings": {
        "label": {
          "normal": "Sketchpad",
          "beginner": "Global sketchpad settings"
        },
        "theme": {
          "label": {
            "normal": "Theme",
            "beginner": "Color scheme for the port"
          },
          "system": {
            "label": {
              "normal": "System",
              "beginner": "System"
            }
          },
          "light": {
            "label": {
              "normal": "Light",
              "beginner": "Light"
            }
          },
          "dark": {
            "label": {
              "normal": "Dark",
              "beginner": "Dark"
            }
          }
        },
        "device": {
          "label": {
            "normal": "Device",
            "beginner": "Device mode for the port"
          },
          "desktop": {
            "label": {
              "normal": "Desktop",
              "beginner": "Desktop"
            }
          },
          "tablet": {
            "label": {
              "normal": "Tablet",
              "beginner": "Tablet"
            }
          }
        },
        "mode": {
          "label": {
            "normal": "Mode",
            "beginner": "Port mode"
          },
          "user": {
            "label": {
              "normal": "User",
              "beginner": "User"
            }
          },
          "dev": {
            "label": {
              "normal": "Dev",
              "beginner": "Dev"
            }
          }
        },
        "expertise": {
          "label": {
            "normal": "Expertise",
            "beginner": "Your expertise level"
          },
          "beginner": {
            "label": {
              "normal": "Beginner",
              "beginner": "Show detailed help and tutorials"
            }
          },
          "normal": {
            "label": {
              "normal": "Normal",
              "beginner": "Show standard tooltips"
            }
          },
          "expert": {
            "label": {
              "normal": "Expert",
              "beginner": "Expert"
            }
          }
        },
        "language": {
          "label": {
            "normal": "Language",
            "beginner": "Select the language for the application port"
          },
          "placeholder": {
            "label": {
              "normal": "Select language...",
              "beginner": "Select the port language"
            }
          },
          "de": {
            "label": {
              "normal": "Deutsch",
              "beginner": "Deutsch"
            }
          },
          "en": {
            "label": {
              "normal": "English",
              "beginner": "English"
            }
          }
        }
      },
      "tool": {
        "label": {
          "normal": "Tools",
          "beginner": "Tools"
        },
        "selection": {
          "label": {
            "normal": "Selection",
            "beginner": "Selection"
          },
          "normal": {
            "label": {
              "normal": "Normal Selection",
              "beginner": "Click to select one item at a time"
            },
            "manual": "selection",
            "hotkey": "1"
          },
          "additive": {
            "label": {
              "normal": "Add to Selection",
              "beginner": "Click to add items to your selection without holding Ctrl"
            },
            "manual": "selection",
            "hotkey": "2"
          },
          "subtractive": {
            "label": {
              "normal": "Remove from Selection",
              "beginner": "Click to remove items from your selection without holding Alt"
            },
            "manual": "selection",
            "hotkey": "3"
          }
        }
      },
      "sort": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "ascending": {
          "label": {
            "normal": "Ascending",
            "beginner": "Ascending"
          }
        },
        "descending": {
          "label": {
            "normal": "Descending",
            "beginner": "Descending"
          }
        }
      },
      "app": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "home": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "title": "Home",
          "fileInput": {
            "label": {
              "normal": "Choose kit file",
              "beginner": "Select a .zip kit file from your device"
            }
          },
          "searchPlaceholder": {
            "label": {
              "normal": "Search kits...",
              "beginner": "Search kits..."
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Name"
            }
          },
          "kind": {
            "label": {
              "normal": "Kind",
              "beginner": "Kind"
            }
          },
          "lastUpdated": {
            "label": {
              "normal": "Last Updated",
              "beginner": "Last Updated"
            }
          },
          "created": {
            "label": {
              "normal": "Created",
              "beginner": "Created"
            }
          },
          "filter": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "band": {
              "label": {
                "normal": "Filter band",
                "beginner": "Toggle the filter band to show or hide filter options"
              },
              "hotkey": "Ctrl+F"
            },
            "kind": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "show": {
                "label": {
                  "normal": "Filter by kind",
                  "beginner": "Filter kits by their storage type (temporary, local, or remote)"
                },
                "hotkey": "Ctrl+K"
              },
              "create": {
                "label": {
                  "normal": "Create new kit",
                  "beginner": "Create a new kit of this type"
                },
                "hotkey": "Ctrl+Shift+K"
              },
              "temporary": {
                "label": {
                  "normal": "Show temporary kits",
                  "beginner": "Show kits stored in browser memory (lost on refresh)"
                },
                "hotkey": "Ctrl+1"
              },
              "createTemporary": {
                "label": {
                  "normal": "Create temporary kit",
                  "beginner": "Create a new temporary kit stored in browser memory"
                },
                "hotkey": "Ctrl+Shift+1"
              },
              "local": {
                "label": {
                  "normal": "Show local kits",
                  "beginner": "Show kits stored locally on your device"
                },
                "hotkey": "Ctrl+2"
              },
              "createLocal": {
                "label": {
                  "normal": "Create local kit",
                  "beginner": "Create a new kit stored locally on your device"
                },
                "hotkey": "Ctrl+Shift+2"
              },
              "remote": {
                "label": {
                  "normal": "Show remote kits",
                  "beginner": "Show kits synced with remote storage"
                },
                "hotkey": "Ctrl+3"
              },
              "createRemote": {
                "label": {
                  "normal": "Create remote kit",
                  "beginner": "Create a new kit synced with remote storage"
                },
                "hotkey": "Ctrl+Shift+3"
              }
            },
            "name": {
              "label": {
                "normal": "Filter by name",
                "beginner": "Filter kits by this specific name"
              },
              "hotkey": "Ctrl+N"
            },
            "version": {
              "label": {
                "normal": "Filter by version",
                "beginner": "Filter kits to this specific version"
              },
              "hotkey": "Ctrl+V"
            }
          },
          "search": {
            "label": {
              "normal": "Search",
              "beginner": "Search for kits"
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Home settings"
            },
            "theme": {
              "label": {
                "normal": "Theme",
                "beginner": "Choose the color theme for the application"
              }
            },
            "language": {
              "label": {
                "normal": "Language",
                "beginner": "Select the language for the application port"
              },
              "placeholder": {
                "label": {
                  "normal": "Select language...",
                  "beginner": "Select the port language"
                }
              }
            },
            "mode": {
              "label": {
                "normal": "Mode",
                "beginner": "Select the user port mode"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "Select your expertise level"
              }
            },
            "device": {
              "label": {
                "normal": "Device",
                "beginner": "Select the input device"
              }
            },
            "layout": {
              "label": {
                "normal": "Layout",
                "beginner": "Choose the layout for the kit overview"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagram",
                "beginner": "Configure the force-directed diagram layout"
              },
              "chargeStrength": {
                "label": {
                  "normal": "Charge Strength",
                  "beginner": "Controls how strongly nodes repel each other. More negative values push nodes further apart."
                }
              },
              "linkDistance": {
                "label": {
                  "normal": "Link Distance",
                  "beginner": "The target distance between connected nodes. Larger values spread the diagram out."
                }
              },
              "collideRadius": {
                "label": {
                  "normal": "Collision Radius",
                  "beginner": "The minimum distance between node centers to prevent overlap."
                }
              },
              "centerStrength": {
                "label": {
                  "normal": "Center Strength",
                  "beginner": "How strongly nodes are pulled toward the center of the diagram."
                }
              }
            }
          },
          "canvas": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "table": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "createKit": {
                "label": {
                  "normal": "Create kit",
                  "beginner": "Create a new kit from the home table"
                }
              },
              "createVersion": {
                "label": {
                  "normal": "Create version",
                  "beginner": "Create a new kit version from the home table"
                }
              },
              "hover": {
                "label": {
                  "normal": "Highlight kit",
                  "beginner": "Hover a kit row to highlight it in the diagram"
                }
              },
              "toggleSort": {
                "label": {
                  "normal": "Toggle sort",
                  "beginner": "Change sort direction for this column"
                }
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "kit": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "name": {
                  "label": {
                    "normal": "Name",
                    "beginner": "The name of the kit"
                  }
                },
                "version": {
                  "label": {
                    "normal": "Version",
                    "beginner": "The version of the kit"
                  }
                },
                "description": {
                  "label": {
                    "normal": "Description",
                    "beginner": "A description of the kit"
                  }
                },
                "icon": {
                  "label": {
                    "normal": "Icon",
                    "beginner": "The icon of the kit"
                  }
                },
                "image": {
                  "label": {
                    "normal": "Image",
                    "beginner": "The preview image of the kit"
                  }
                }
              },
              "kits": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "name": {
                  "label": {
                    "normal": "Name",
                    "beginner": "The name of the selected kits"
                  }
                },
                "version": {
                  "label": {
                    "normal": "Version",
                    "beginner": "The version of the selected kits"
                  }
                },
                "description": {
                  "label": {
                    "normal": "Description",
                    "beginner": "A description of the selected kits"
                  }
                },
                "icon": {
                  "label": {
                    "normal": "Icon",
                    "beginner": "The icon of the selected kits"
                  }
                },
                "image": {
                  "label": {
                    "normal": "Image",
                    "beginner": "The preview image of the selected kits"
                  }
                }
              }
            }
          },
          "dropzone": {
            "label": {
              "normal": "Drop zip file to import kit",
              "beginner": "Drop a zip file here to import it as a new kit"
            },
            "description": {
              "normal": "Only kits with .compose folder can be imported",
              "beginner": "The zip file must contain a .compose folder with kit.db to be imported as a kit."
            }
          },
          "noKits": {
            "label": {
              "normal": "No Kits",
              "beginner": "No Kits"
            }
          },
          "sortByName": {
            "label": {
              "normal": "Sort By Name",
              "beginner": "Sort By Name"
            }
          },
          "toggleRow": {
            "label": {
              "normal": "Toggle Row",
              "beginner": "Toggle Row"
            }
          },
          "createVersion": {
            "label": {
              "normal": "Create Version",
              "beginner": "Create Version"
            }
          },
          "hideKind": {
            "label": {
              "normal": "Hide Kind",
              "beginner": "Hide Kind"
            }
          },
          "showTemporary": {
            "label": {
              "normal": "Show Temporary",
              "beginner": "Show Temporary"
            }
          },
          "showLocal": {
            "label": {
              "normal": "Show Local",
              "beginner": "Show Local"
            }
          },
          "showRemote": {
            "label": {
              "normal": "Show Remote",
              "beginner": "Show Remote"
            }
          },
          "sortByType": {
            "label": {
              "normal": "Sort By Type",
              "beginner": "Sort By Type"
            }
          },
          "sortByUpdatedAt": {
            "label": {
              "normal": "Sort By Updated At",
              "beginner": "Sort By Updated At"
            }
          },
          "sortByCreatedAt": {
            "label": {
              "normal": "Sort By Created At",
              "beginner": "Sort By Created At"
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Chat"
            }
          },
          "createKit": {
            "label": {
              "normal": "Create Kit",
              "beginner": "Create Kit"
            }
          },
          "createTemporary": {
            "label": {
              "normal": "Create Temporary",
              "beginner": "Create Temporary"
            }
          },
          "createLocal": {
            "label": {
              "normal": "Create Local",
              "beginner": "Create Local"
            }
          },
          "createRemote": {
            "label": {
              "normal": "Create Remote",
              "beginner": "Create Remote"
            }
          },
          "importKit": {
            "label": {
              "normal": "Import Kit",
              "beginner": "Import Kit"
            }
          },
          "toolbar": {
            "showTemporary": {
              "label": {
                "normal": "Temporary",
                "beginner": "Temporary"
              }
            },
            "showLocal": {
              "label": {
                "normal": "Local",
                "beginner": "Local"
              }
            },
            "showRemote": {
              "label": {
                "normal": "Remote",
                "beginner": "Remote"
              }
            },
            "createTemporary": {
              "label": {
                "normal": "Temporary",
                "beginner": "Temporary"
              }
            },
            "createLocal": {
              "label": {
                "normal": "Local",
                "beginner": "Local"
              }
            },
            "createRemote": {
              "label": {
                "normal": "Remote",
                "beginner": "Remote"
              }
            },
            "filters": {
              "label": {
                "normal": "Filters",
                "beginner": "Filter kits by location"
              }
            },
            "create": {
              "label": {
                "normal": "Create",
                "beginner": "Create a new kit"
              }
            },
            "createKit": {
              "label": {
                "normal": "New kit",
                "beginner": "Create a new empty kit"
              }
            },
            "openFolder": {
              "label": {
                "normal": "Open folder",
                "beginner": "Open a kit from a folder on disk"
              }
            },
            "openFile": {
              "label": {
                "normal": "Open file",
                "beginner": "Open a kit from a .zip file"
              }
            },
            "openRemote": {
              "label": {
                "normal": "Open remote",
                "beginner": "Open a kit from a remote URL"
              }
            },
            "createFile": {
              "label": {
                "normal": "New file kit",
                "beginner": "Create a kit backed by a file"
              }
            },
            "createFolder": {
              "label": {
                "normal": "New folder kit",
                "beginner": "Create a kit backed by a folder"
              }
            },
            "showFile": {
              "label": {
                "normal": "Show file kits",
                "beginner": "Show kits stored as files"
              }
            },
            "showFolder": {
              "label": {
                "normal": "Show folder kits",
                "beginner": "Show kits stored in folders"
              }
            },
            "exportArchive": {
              "label": {
                "normal": "Export archive",
                "beginner": "Export the selected kit as a .zip archive"
              }
            }
          }
        },
        "kit": {
          "label": {
            "normal": "Kit",
            "beginner": "Kit"
          },
          "properties": {
            "label": {
              "normal": "Kit Properties",
              "beginner": "Kit properties"
            }
          },
          "notFound": {
            "label": {
              "normal": "Kit not found",
              "beginner": "The requested kit could not be found"
            },
            "description": {
              "normal": "The kit may have been removed or the link is invalid.",
              "beginner": "Return home and open another kit, or create a new one."
            }
          },
          "noKitLoaded": {
            "label": {
              "normal": "No kit loaded",
              "beginner": "No kit loaded"
            }
          },
          "loading": {
            "label": {
              "normal": "Loading kit...",
              "beginner": "Loading kit..."
            }
          },
          "notAvailable": {
            "label": {
              "normal": "Kit not available",
              "beginner": "Kit not available"
            }
          },
          "search": {
            "placeholder": {
              "label": {
                "normal": "Filter...",
                "beginner": "Search for artifacts..."
              }
            }
          },
          "dropzone": {
            "label": {
              "normal": "Drop zip file to import",
              "beginner": "Drop a zip file here to import it as a kit or add files to the current kit"
            },
            "description": {
              "normal": "Kits with .compose folder will be imported, others will be added as files",
              "beginner": "If the zip contains a .compose folder, it will be imported as a complete kit. Otherwise, the files will be added to the current kit."
            }
          },
          "versionPlaceholder": {
            "label": {
              "label": {
                "normal": "e.g., 1.0.0",
                "beginner": "e.g., 1.0.0"
              }
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe what this kit contains...",
                "beginner": "Describe what this kit contains..."
              }
            }
          },
          "iconPlaceholder": {
            "label": {
              "label": {
                "normal": "🎨 or URL to icon",
                "beginner": "🎨 or URL to icon"
              }
            }
          },
          "imagePlaceholder": {
            "label": {
              "label": {
                "normal": "URL to preview image",
                "beginner": "URL to preview image"
              }
            }
          },
          "homepagePlaceholder": {
            "label": {
              "label": {
                "normal": "https://example.com",
                "beginner": "https://example.com"
              }
            }
          },
          "licensePlaceholder": {
            "label": {
              "label": {
                "normal": "e.g., MIT, GPL-3.0, Apache-2.0",
                "beginner": "e.g., MIT, GPL-3.0, Apache-2.0"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "New Kit",
              "beginner": "New Kit"
            }
          },
          "defaultDesignName": {
            "label": {
              "normal": "New Design",
              "beginner": "New Design"
            }
          },
          "defaultTypeName": {
            "label": {
              "normal": "New Type",
              "beginner": "New Type"
            }
          },
          "newVersion": {
            "label": {
              "normal": "New Version",
              "beginner": "New Version"
            }
          },
          "defaultVersion": {
            "label": {
              "normal": "Default",
              "beginner": "Default"
            }
          },
          "filter": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "name": {
              "label": {
                "normal": "Filter by name",
                "beginner": "Click to filter artifacts by this name"
              },
              "manual": "manuals/compose/kit",
              "tutorial": "hello-compose/model-design",
              "hotkey": "Ctrl+N",
              "hide": {
                "label": {
                  "normal": "Hide name filter",
                  "beginner": "Click to hide the name filter"
                },
                "hotkey": "Ctrl+Shift+N"
              }
            },
            "band": {
              "label": {
                "normal": "Filter band",
                "beginner": "Toggle the filter band to show or hide filter options"
              },
              "hotkey": "Ctrl+F"
            },
            "search": {
              "label": {
                "normal": "Search filter",
                "beginner": "Search for specific filters"
              },
              "hotkey": "Ctrl+Shift+F"
            }
          },
          "pieces": {
            "label": {
              "normal": "Pieces",
              "beginner": "Types and designs in this kit"
            }
          },
          "designs": {
            "label": {
              "normal": "Designs",
              "beginner": "Designs in this kit"
            },
            "manual": "manuals/compose/kit#designs",
            "tutorial": "hello-compose/model-design",
            "multipleSelected": {
              "label": {
                "normal": "Multiple Selected",
                "beginner": "Multiple Selected"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "types": {
            "label": {
              "normal": "Types",
              "beginner": "Types in this kit"
            },
            "manual": "manuals/compose/kit#types",
            "tutorial": "hello-compose/model-brick-set",
            "multipleSelected": {
              "label": {
                "normal": "Multiple Selected",
                "beginner": "Multiple Selected"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "folder": {
            "label": {
              "normal": "Folder",
              "beginner": "Folder"
            },
            "descriptionPlaceholder": {
              "label": {
                "label": {
                  "normal": "Describe this folder...",
                  "beginner": "Describe this folder..."
                }
              }
            }
          },
          "sortByArtifact": {
            "label": {
              "normal": "Sort by artifact",
              "beginner": "Sort artifacts by their name"
            },
            "hotkey": "Ctrl+Shift+A"
          },
          "sortByKind": {
            "label": {
              "normal": "Sort by kind",
              "beginner": "Sort artifacts by their type (design, type, quality, etc.)"
            },
            "hotkey": "Ctrl+Shift+K"
          },
          "sortByCreatedAt": {
            "label": {
              "normal": "Sort by creation date",
              "beginner": "Sort artifacts by when they were created"
            },
            "hotkey": "Ctrl+Shift+C"
          },
          "sortByUpdatedAt": {
            "label": {
              "normal": "Sort by update date",
              "beginner": "Sort artifacts by when they were last updated"
            },
            "hotkey": "Ctrl+Shift+U"
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showDesigns": {
              "label": {
                "normal": "Designs",
                "beginner": "Designs"
              }
            },
            "createDesign": {
              "label": {
                "normal": "Design",
                "beginner": "Create Design"
              },
              "manual": "manuals/compose/kit#designs",
              "tutorial": "hello-compose/model-design"
            },
            "showTypes": {
              "label": {
                "normal": "Types",
                "beginner": "Types"
              }
            },
            "createType": {
              "label": {
                "normal": "Type",
                "beginner": "Create Type"
              },
              "manual": "manuals/compose/kit#types",
              "tutorial": "hello-compose/model-brick-set"
            },
            "showQualities": {
              "label": {
                "normal": "Qualities",
                "beginner": "Qualities"
              }
            },
            "createQuality": {
              "label": {
                "normal": "Quality",
                "beginner": "Create Quality"
              },
              "manual": "manuals/compose/kit#qualities",
              "tutorial": "getting-started/intro#quality"
            },
            "showPorts": {
              "label": {
                "normal": "Ports",
                "beginner": "Ports"
              }
            },
            "createPort": {
              "label": {
                "normal": "Port",
                "beginner": "Create Port"
              },
              "manual": "manuals/compose/kit#ports",
              "tutorial": "getting-started/intro#port"
            },
            "showFiles": {
              "label": {
                "normal": "Files",
                "beginner": "Files"
              }
            },
            "createFile": {
              "label": {
                "normal": "Create File",
                "beginner": "Click to add a new file to this kit"
              },
              "manual": "manuals/compose/kit#files",
              "tutorial": "getting-started/intro#files"
            },
            "showFolders": {
              "label": {
                "normal": "Folders",
                "beginner": "Folders"
              }
            },
            "createFolder": {
              "label": {
                "normal": "Folder",
                "beginner": "Create Folder"
              },
              "manual": "manuals/compose/kit#folders",
              "tutorial": "hello-compose/model-design"
            },
            "reset": {
              "label": {
                "normal": "Reset",
                "beginner": "Click to reset the kit to its original state"
              }
            },
            "showAuthors": {
              "label": {
                "normal": "Authors",
                "beginner": "Authors"
              }
            },
            "createAuthor": {
              "label": {
                "normal": "Create Author",
                "beginner": "Click to add a new author to this kit"
              },
              "manual": "manuals/compose/kit#authors",
              "tutorial": "getting-started/intro#authors"
            },
            "hideKind": {
              "label": {
                "normal": "Hide",
                "beginner": "Click to hide this artifact category"
              }
            },
            "createArtifact": {
              "label": {
                "normal": "Create",
                "beginner": "Click to create a new artifact of this type"
              }
            },
            "createChild": {
              "label": {
                "normal": "Create Child",
                "beginner": "Create a child element"
              }
            },
            "showTags": {
              "label": {
                "normal": "Tags",
                "beginner": "Tags"
              }
            },
            "showConcepts": {
              "label": {
                "normal": "Concepts",
                "beginner": "Concepts"
              }
            },
            "selection": {
              "label": {
                "normal": "Selection",
                "beginner": "Selection tools"
              }
            },
            "filters": {
              "label": {
                "normal": "Filters",
                "beginner": "Filter artifacts by type"
              }
            },
            "create": {
              "label": {
                "normal": "Create",
                "beginner": "Create new artifacts"
              }
            },
            "resetFilters": {
              "label": {
                "normal": "Reset Filters",
                "beginner": "Clear active artifact filters and show all artifact kinds"
              }
            },
            "createTag": {
              "label": {
                "normal": "Tag",
                "beginner": "Create Tag"
              }
            },
            "createConcept": {
              "label": {
                "normal": "Concept",
                "beginner": "Create Concept"
              }
            }
          },
          "canvas": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "table": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "search": {
                "label": {
                  "normal": "Search table",
                  "beginner": "Search for artifacts in the table"
                },
                "hotkey": "Ctrl+F"
              },
              "header": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "kind": {
                  "label": {
                    "normal": "Kind",
                    "beginner": "Type of artifact"
                  }
                },
                "artifact": {
                  "label": {
                    "normal": "Name",
                    "beginner": "The name"
                  }
                },
                "updatedAt": {
                  "label": {
                    "normal": "Updated",
                    "beginner": "Last update time"
                  }
                },
                "createdAt": {
                  "label": {
                    "normal": "Created",
                    "beginner": "Creation time"
                  }
                }
              }
            },
            "wires": {
              "label": {
                "normal": "Wires",
                "beginner": "Relationships between visible file-system nodes (Owns, Is, References, Has)"
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "kit": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Kit name",
                      "beginner": "The name of the kit. This is the primary identifier for your kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "version": {
                    "label": {
                      "normal": "Version",
                      "beginner": "The version of the kit in semantic versioning format (e.g., 1.0.0)."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A detailed description of what this kit contains and how it should be used."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "icon": {
                    "label": {
                      "normal": "Icon",
                      "beginner": "An icon to represent this kit. Can be an emoji or URL to an image."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "image": {
                    "label": {
                      "normal": "Image",
                      "beginner": "URL to a preview image that showcases this kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "homepage": {
                    "label": {
                      "normal": "Homepage",
                      "beginner": "URL to the homepage or documentation for this kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "license": {
                    "label": {
                      "normal": "License",
                      "beginner": "The license under which this kit is distributed (e.g., MIT, GPL)."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-compose/save-kit"
                  }
                },
                "folder": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The display name of the folder."
                    },
                    "manual": "kit#folders",
                    "tutorial": "hello-compose/save-kit"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "Optional description that explains the purpose of this folder."
                    },
                    "manual": "kit#folders",
                    "tutorial": "hello-compose/save-kit"
                  }
                },
                "port": {
                  "compatible": {
                    "label": {
                      "normal": "Compatible",
                      "beginner": "Compatible"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "Description"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  }
                },
                "tag": {
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "Description"
                    }
                  }
                },
                "concept": {
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "Description"
                    }
                  }
                }
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Kit settings"
            },
            "diagram": {
              "chargeStrength": {
                "label": {
                  "normal": "Charge Strength",
                  "beginner": "Repulsion force between nodes"
                }
              },
              "linkDistance": {
                "label": {
                  "normal": "Link Distance",
                  "beginner": "Target distance between connected nodes"
                }
              },
              "collideRadius": {
                "label": {
                  "normal": "Collide Radius",
                  "beginner": "Collision radius preventing node overlap"
                }
              },
              "centerStrength": {
                "label": {
                  "normal": "Center Strength",
                  "beginner": "Force pulling nodes toward center"
                }
              }
            },
            "theme": {
              "label": {
                "normal": "Theme",
                "beginner": "Color theme"
              }
            },
            "language": {
              "label": {
                "normal": "Language",
                "beginner": "Port language"
              }
            },
            "device": {
              "label": {
                "normal": "Device",
                "beginner": "Input device type"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "User expertise level"
              }
            },
            "mode": {
              "label": {
                "normal": "Mode",
                "beginner": "User or developer mode"
              }
            }
          },
          "port": {
            "allCompatible": {
              "label": {
                "normal": "All Compatible",
                "beginner": "All Compatible"
              }
            },
            "compatiblePorts": {
              "label": {
                "normal": "Compatible Ports",
                "beginner": "Compatible Ports"
              }
            },
            "descriptionPlaceholder": {
              "label": {
                "label": {
                  "normal": "Label",
                  "beginner": "Label"
                }
              }
            }
          },
          "ports": {
            "multipleSelected": {
              "label": {
                "normal": "Multiple Selected",
                "beginner": "Multiple Selected"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "{{count}} ports",
                "beginner": "{{count}} ports selected"
              }
            }
          },
          "qualities": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "files": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "authors": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "tag": {
            "descriptionPlaceholder": {
              "label": {
                "normal": "Describe this tag...",
                "beginner": "Describe this tag..."
              }
            }
          },
          "tags": {
            "multipleSelected": {
              "label": {
                "normal": "Multiple tags selected",
                "beginner": "Multiple tags selected"
              }
            },
            "multipleTitle": "{{count}} tags"
          },
          "concept": {
            "descriptionPlaceholder": {
              "label": {
                "normal": "Describe this concept...",
                "beginner": "Describe this concept..."
              }
            }
          },
          "concepts": {
            "multipleSelected": {
              "label": {
                "normal": "Multiple concepts selected",
                "beginner": "Multiple concepts selected"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Title",
              "beginner": "Title"
            }
          },
          "tools": {
            "label": {
              "normal": "Tools",
              "beginner": "Tools"
            },
            "select": {
              "mode": {
                "additive": {
                  "label": {
                    "normal": "Additive",
                    "beginner": "Additive selection mode - add to existing selection"
                  }
                },
                "subtractive": {
                  "label": {
                    "normal": "Subtractive",
                    "beginner": "Subtractive selection mode - remove from existing selection"
                  }
                },
                "intersect": {
                  "label": {
                    "normal": "Intersect",
                    "beginner": "Intersect selection mode - select only overlapping items"
                  }
                }
              },
              "shape": {
                "rectangular": {
                  "label": {
                    "normal": "Rectangular",
                    "beginner": "Rectangular selection - drag to select in a rectangle"
                  }
                },
                "lasso": {
                  "label": {
                    "normal": "Lasso",
                    "beginner": "Freeform lasso selection - draw a freeform shape"
                  }
                }
              },
              "navigation": {
                "hand": {
                  "label": {
                    "normal": "Hand",
                    "beginner": "Hand tool - pan and navigate the canvas"
                  }
                }
              }
            }
          },
          "tool": {
            "pointer": {
              "label": {
                "normal": "Selection",
                "beginner": "Selection"
              }
            },
            "hand": {
              "label": {
                "normal": "Hand",
                "beginner": "Hand"
              }
            }
          }
        },
        "port": {
          "label": {
            "normal": "Port",
            "beginner": "Port"
          },
          "defaultName": {
            "label": {
              "normal": "New Port",
              "beginner": "New Port"
            }
          }
        },
        "tag": {
          "label": {
            "normal": "Tag",
            "beginner": "Tag"
          },
          "defaultName": {
            "label": {
              "normal": "New Tag",
              "beginner": "New Tag"
            }
          }
        },
        "concept": {
          "label": {
            "normal": "Concept",
            "beginner": "Concept"
          },
          "defaultName": {
            "label": {
              "normal": "New Concept",
              "beginner": "New Concept"
            }
          }
        },
        "folder": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "defaultName": {
            "label": {
              "normal": "New Folder",
              "beginner": "New Folder"
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe this folder...",
                "beginner": "Describe this folder..."
              }
            }
          }
        },
        "design": {
          "label": {
            "normal": "Design",
            "beginner": "Design"
          },
          "properties": {
            "label": {
              "normal": "Design Properties",
              "beginner": "Design properties"
            }
          },
          "console": {
            "label": {
              "normal": "Console",
              "beginner": "Console"
            },
            "empty": {
              "label": {
                "normal": "Console output will appear here.",
                "beginner": "Console output will appear here."
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "New Design",
              "beginner": "New Design"
            }
          },
          "windowLibrary": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "scene": {
              "label": {
                "normal": "Scene Windows",
                "beginner": "Scene Windows"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagram Windows",
                "beginner": "Diagram Windows"
              }
            },
            "table": {
              "label": {
                "normal": "Table Windows",
                "beginner": "Table Windows"
              }
            }
          },
          "diagram": {
            "clusterMenu": {
              "cluster": {
                "label": {
                  "normal": "Cluster",
                  "beginner": "Group selected design pieces into a single cluster"
                }
              }
            },
            "expandMenu": {
              "expand": {
                "label": {
                  "normal": "Expand",
                  "beginner": "Expand the design piece to show its internal components"
                }
              }
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "normal": "Describe this design...",
              "beginner": "Describe this design..."
            }
          },
          "iconPlaceholder": {
            "label": {
              "normal": "???",
              "beginner": "???"
            }
          },
          "imagePlaceholder": {
            "label": {
              "normal": "https://example.com/image.png",
              "beginner": "https://example.com/image.png"
            }
          },
          "variantPlaceholder": {
            "label": {
              "normal": "e.g., small, medium, large",
              "beginner": "e.g., small, medium, large"
            }
          },
          "viewPlaceholder": {
            "label": {
              "normal": "e.g., front, side, top",
              "beginner": "e.g., front, side, top"
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Name"
            }
          },
          "variant": {
            "label": {
              "normal": "Variant",
              "beginner": "Variant"
            }
          },
          "view": {
            "label": {
              "normal": "View",
              "beginner": "View"
            }
          },
          "location": {
            "label": {
              "normal": "Location",
              "beginner": "Location"
            }
          },
          "authors": {
            "label": {
              "normal": "Authors",
              "beginner": "Authors"
            }
          },
          "author": {
            "label": {
              "normal": "Author",
              "beginner": "Author"
            }
          },
          "attributes": {
            "label": {
              "normal": "Attributes",
              "beginner": "Attributes"
            }
          },
          "attribute": {
            "label": {
              "normal": "Attribute",
              "beginner": "Attribute"
            }
          },
          "attributeValuePlaceholder": {
            "label": {
              "normal": "Value...",
              "beginner": "Value..."
            }
          },
          "attributeUnitPlaceholder": {
            "label": {
              "normal": "Unit...",
              "beginner": "Unit..."
            }
          },
          "attributeDefinitionPlaceholder": {
            "label": {
              "normal": "Definition or URL...",
              "beginner": "Definition or URL..."
            }
          },
          "piece": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "id": {
              "label": {
                "normal": "ID",
                "beginner": "ID"
              }
            },
            "type": {
              "label": {
                "normal": "Type",
                "beginner": "Type"
              }
            },
            "center": {
              "label": {
                "normal": "Center",
                "beginner": "Center"
              }
            },
            "plane": {
              "label": {
                "normal": "Plane",
                "beginner": "Plane"
              }
            },
            "planeOrigin": {
              "label": {
                "normal": "Origin",
                "beginner": "Origin"
              }
            },
            "planeXAxis": {
              "label": {
                "normal": "X Axis",
                "beginner": "X Axis"
              }
            },
            "planeYAxis": {
              "label": {
                "normal": "Y Axis",
                "beginner": "Y Axis"
              }
            },
            "mixedSelectionMessage": {
              "label": {
                "normal": "Multiple pieces selected with different values",
                "beginner": "Multiple pieces selected with different values"
              }
            },
            "connectedPieceInfo": {
              "label": {
                "normal": "This piece is connected to another piece. Its position and orientation are computed from the connection. To make it independent, click 'Fix Piece'.",
                "beginner": "This piece is connected to another piece. Its position and orientation are computed from the connection. To make it independent, click 'Fix Piece'."
              }
            },
            "fixPiece": {
              "label": {
                "normal": "Fix Piece",
                "beginner": "Fix Piece"
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "design": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The name of the design. This is the primary identifier for your composition."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A detailed description of what this design represents and how it should be used."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "icon": {
                    "label": {
                      "normal": "Icon",
                      "beginner": "URL or path to an icon that represents this design in listings and previews."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "image": {
                    "label": {
                      "normal": "Image",
                      "beginner": "URL or path to a preview image that showcases this design."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "variant": {
                    "label": {
                      "normal": "Variant",
                      "beginner": "A variant identifier for different versions or configurations of this design."
                    },
                    "manual": "design#variants",
                    "tutorial": "hello-compose/model-design"
                  },
                  "view": {
                    "label": {
                      "normal": "View",
                      "beginner": "The viewing perspective or camera angle for displaying this design."
                    },
                    "manual": "design#views",
                    "tutorial": "hello-compose/model-design"
                  },
                  "unit": {
                    "label": {
                      "normal": "Unit",
                      "beginner": "The measurement unit used for all dimensions in this design (e.g., mm, cm, m)."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-compose/model-design"
                  },
                  "createdAt": {
                    "label": {
                      "normal": "Created At",
                      "beginner": "The date and time when this design was first created."
                    }
                  },
                  "updatedAt": {
                    "label": {
                      "normal": "Updated At",
                      "beginner": "The date and time when this design was last modified."
                    }
                  },
                  "pieceCount": {
                    "label": {
                      "normal": "Pieces",
                      "beginner": "The total number of pieces in this design."
                    }
                  },
                  "connectionCount": {
                    "label": {
                      "normal": "Connections",
                      "beginner": "The total number of connections in this design."
                    }
                  }
                },
                "location": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "longitude": {
                    "label": {
                      "normal": "Longitude",
                      "beginner": "The east-west position of this design's location in decimal degrees."
                    },
                    "manual": "design#location"
                  },
                  "latitude": {
                    "label": {
                      "normal": "Latitude",
                      "beginner": "The north-south position of this design's location in decimal degrees."
                    },
                    "manual": "design#location"
                  }
                },
                "authors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The full name of the person who contributed to this design."
                    },
                    "manual": "design#authors"
                  },
                  "email": {
                    "label": {
                      "normal": "Email",
                      "beginner": "Contact email address for this author."
                    },
                    "manual": "design#authors"
                  }
                },
                "attributes": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The unique identifier for this attribute in kebab-case format (e.g., 'material.type')."
                    },
                    "manual": "design#attributes"
                  },
                  "value": {
                    "label": {
                      "normal": "Value",
                      "beginner": "The value associated with this attribute. Leave empty to use as a category flag."
                    },
                    "manual": "design#attributes"
                  },
                  "unit": {
                    "label": {
                      "normal": "Unit",
                      "beginner": "The measurement unit for this attribute's value (e.g., mm, kg, °C)."
                    },
                    "manual": "design#attributes"
                  },
                  "definition": {
                    "label": {
                      "normal": "Definition",
                      "beginner": "A URL or text that defines what this attribute means and how it should be interpreted."
                    },
                    "manual": "design#attributes"
                  }
                },
                "piece": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "properties": {
                    "label": {
                      "normal": "Piece",
                      "beginner": "Properties of the selected piece."
                    }
                  },
                  "multipleTitle": {
                    "label": {
                      "normal": "Pieces",
                      "beginner": "Properties of the selected pieces."
                    }
                  },
                  "pieceInfo": {
                    "label": {
                      "normal": "Piece",
                      "beginner": "Basic information about the piece."
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "An optional name for this piece to identify it within the design."
                    }
                  },
                  "namePlaceholder": {
                    "label": {
                      "normal": "Enter piece name...",
                      "beginner": "Enter piece name..."
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of this piece's purpose or role in the design."
                    }
                  },
                  "descriptionPlaceholder": {
                    "label": {
                      "normal": "Enter piece description...",
                      "beginner": "Enter piece description..."
                    }
                  },
                  "scale": {
                    "label": {
                      "normal": "Scale",
                      "beginner": "The scaling factor applied to this piece. Default is 1.0."
                    }
                  },
                  "color": {
                    "label": {
                      "normal": "Color",
                      "beginner": "An optional color override for this piece (e.g., #FF0000)."
                    }
                  },
                  "colorPlaceholder": {
                    "label": {
                      "normal": "Enter color...",
                      "beginner": "Enter color..."
                    }
                  },
                  "attributes": {
                    "label": {
                      "normal": "Attributes",
                      "beginner": "Custom key-value attributes for this piece."
                    },
                    "name": {
                      "label": {
                        "normal": "Name",
                        "beginner": "The key identifier for this attribute."
                      }
                    },
                    "value": {
                      "label": {
                        "normal": "Value",
                        "beginner": "The value for this attribute."
                      }
                    },
                    "unit": {
                      "label": {
                        "normal": "Unit",
                        "beginner": "The measurement unit for this attribute."
                      }
                    },
                    "definition": {
                      "label": {
                        "normal": "Definition",
                        "beginner": "A URL or text defining this attribute."
                      }
                    }
                  },
                  "attribute": {
                    "label": {
                      "normal": "Attribute",
                      "beginner": "A custom attribute of this piece."
                    }
                  },
                  "center": {
                    "label": {
                      "normal": "Center",
                      "beginner": "The center position of the piece in the 2D diagram layout."
                    },
                    "manual": "design#diagram",
                    "tutorial": "metabolism/thinking-about-the-diagram",
                    "x": {
                      "label": {
                        "normal": "U",
                        "beginner": "U diagram coordinate of the center of the piece in 2D layout space."
                      },
                      "manual": "design#diagram",
                      "tutorial": "metabolism/thinking-about-the-diagram"
                    },
                    "y": {
                      "label": {
                        "normal": "V",
                        "beginner": "V diagram coordinate of the center of the piece in 2D layout space."
                      },
                      "manual": "design#diagram",
                      "tutorial": "metabolism/thinking-about-the-diagram"
                    }
                  },
                  "plane": {
                    "label": {
                      "normal": "Plane",
                      "beginner": "The 3D placement plane for this piece. Defines position and orientation in 3D space."
                    },
                    "manual": "design#pieces",
                    "tutorial": "hello-compose/model-design#pieces",
                    "origin": {
                      "label": {
                        "normal": "Origin",
                        "beginner": "Origin"
                      },
                      "x": {
                        "label": {
                          "normal": "Origin X",
                          "beginner": "Origin X"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Origin Y",
                          "beginner": "Origin Y"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Origin Z",
                          "beginner": "Origin Z"
                        }
                      }
                    },
                    "xaxis": {
                      "label": {
                        "normal": "X Axis",
                        "beginner": "X Axis"
                      },
                      "x": {
                        "label": {
                          "normal": "Axis X",
                          "beginner": "Axis X"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Axis Y",
                          "beginner": "Axis Y"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Axis Z",
                          "beginner": "Axis Z"
                        }
                      }
                    },
                    "yaxis": {
                      "label": {
                        "normal": "Y Axis",
                        "beginner": "Y Axis"
                      },
                      "x": {
                        "label": {
                          "normal": "Axis X",
                          "beginner": "Axis X"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Axis Y",
                          "beginner": "Axis Y"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Axis Z",
                          "beginner": "Axis Z"
                        }
                      }
                    }
                  }
                },
                "connection": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "properties": {
                    "label": {
                      "normal": "Connection",
                      "beginner": "Properties of the selected connection."
                    }
                  },
                  "multipleTitle": {
                    "label": {
                      "normal": "Connections",
                      "beginner": "Properties of the selected connections."
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of this connection's purpose."
                    }
                  },
                  "descriptionPlaceholder": {
                    "label": {
                      "normal": "Enter connection description...",
                      "beginner": "Enter connection description..."
                    }
                  },
                  "multipleEditing": {
                    "label": {
                      "normal": "Editing {{count}} connections simultaneously",
                      "beginner": "Editing {{count}} connections simultaneously"
                    }
                  },
                  "connecting": {
                    "label": {
                      "normal": "Connecting",
                      "beginner": "Connecting"
                    }
                  },
                  "connectingPieceId": {
                    "label": {
                      "normal": "Connecting Piece",
                      "beginner": "Connecting Piece"
                    }
                  },
                  "connectingPortId": {
                    "label": {
                      "normal": "Connecting Port",
                      "beginner": "Connecting Port"
                    }
                  },
                  "connectingDesignPieceId": {
                    "label": {
                      "normal": "Connecting Design Piece",
                      "beginner": "Connecting Design Piece"
                    }
                  },
                  "connected": {
                    "label": {
                      "normal": "Connected",
                      "beginner": "Connected"
                    }
                  },
                  "connectedPieceId": {
                    "label": {
                      "normal": "Connected Piece",
                      "beginner": "Connected Piece"
                    }
                  },
                  "connectedPortId": {
                    "label": {
                      "normal": "Connected Port",
                      "beginner": "Connected Port"
                    }
                  },
                  "connectedDesignPieceId": {
                    "label": {
                      "normal": "Connected Design Piece",
                      "beginner": "Connected Design Piece"
                    }
                  },
                  "gap": {
                    "label": {
                      "normal": "Gap",
                      "beginner": "Gap"
                    }
                  },
                  "shift": {
                    "label": {
                      "normal": "Shift",
                      "beginner": "Shift"
                    }
                  },
                  "rise": {
                    "label": {
                      "normal": "Rise",
                      "beginner": "Rise"
                    }
                  },
                  "rotation": {
                    "label": {
                      "normal": "Rotation",
                      "beginner": "Rotation"
                    }
                  },
                  "turn": {
                    "label": {
                      "normal": "Turn",
                      "beginner": "Turn"
                    }
                  },
                  "tilt": {
                    "label": {
                      "normal": "Tilt",
                      "beginner": "Tilt"
                    }
                  },
                  "x": {
                    "label": {
                      "normal": "Diagram X Offset",
                      "beginner": "Diagram X Offset"
                    }
                  },
                  "y": {
                    "label": {
                      "normal": "Diagram Y Offset",
                      "beginner": "Diagram Y Offset"
                    }
                  },
                  "u": {
                    "label": {
                      "normal": "X Offset",
                      "beginner": "X Offset"
                    }
                  },
                  "v": {
                    "label": {
                      "normal": "Y Offset",
                      "beginner": "Y Offset"
                    }
                  }
                },
                "connector": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "id": {
                    "label": {
                      "normal": "Connector ID",
                      "beginner": "The unique identifier of the connector"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The name of this connector."
                    }
                  },
                  "t": {
                    "label": {
                      "normal": "T",
                      "beginner": "The parameter t along the type's curve for this connector."
                    }
                  },
                  "position": {
                    "label": {
                      "normal": "Position",
                      "beginner": "The position of the connector"
                    }
                  },
                  "direction": {
                    "label": {
                      "normal": "Direction",
                      "beginner": "The direction vector of the connector"
                    }
                  },
                  "mandatory": {
                    "label": {
                      "normal": "Mandatory",
                      "beginner": "Whether this connector must be connected"
                    }
                  },
                  "port": {
                    "label": {
                      "normal": "Port",
                      "beginner": "The connector port for compatibility checking"
                    }
                  },
                  "compatiblePort": {
                    "label": {
                      "normal": "Compatible Port",
                      "beginner": "The ports this connector is compatible with"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of the connector"
                    }
                  },
                  "attribute": {
                    "label": {
                      "normal": "Attribute",
                      "beginner": "Custom attributes of the connector"
                    }
                  },
                  "notFound": {
                    "label": {
                      "normal": "Not Found",
                      "beginner": "Not Found"
                    }
                  }
                }
              },
              "parentConnection": {
                "label": {
                  "normal": "Parent Connection",
                  "beginner": "Parent Connection"
                }
              },
              "parentConnections": {
                "label": {
                  "normal": "Parent Connections",
                  "beginner": "Parent Connections"
                }
              }
            },
            "hud": {
              "overview": {
                "label": {
                  "normal": "HUD Overview",
                  "beginner": "HUD Overview"
                }
              },
              "selection": {
                "pieces": {
                  "label": {
                    "normal": "Selected Pieces",
                    "beginner": "Selected Pieces"
                  }
                },
                "connections": {
                  "label": {
                    "normal": "Selected Connections",
                    "beginner": "Selected Connections"
                  }
                },
                "connector": {
                  "label": {
                    "normal": "Connector Selected",
                    "beginner": "Connector Selected"
                  }
                }
              }
            },
            "stats": {
              "overview": {
                "label": {
                  "normal": "Statistics",
                  "beginner": "Statistics"
                }
              },
              "pieces": {
                "label": {
                  "normal": "Total Pieces",
                  "beginner": "Total Pieces"
                }
              },
              "connections": {
                "label": {
                  "normal": "Total Connections",
                  "beginner": "Total Connections"
                }
              },
              "windows": {
                "label": {
                  "normal": "Window Layout Loaded",
                  "beginner": "Window Layout Loaded"
                }
              }
            },
            "workbench": {
              "types": {
                "addPiece": {
                  "label": {
                    "normal": "Add Piece",
                    "beginner": "Add a new piece of this type to the design"
                  }
                },
                "duplicateType": {
                  "label": {
                    "normal": "Duplicate Type",
                    "beginner": "Create a duplicate of this type"
                  }
                }
              },
              "designs": {
                "addPiece": {
                  "label": {
                    "normal": "Add Piece",
                    "beginner": "Add a new piece of this design to the current design"
                  }
                }
              }
            }
          },
          "gridSize": {
            "label": {
              "normal": "Grid Size",
              "beginner": "Grid Size"
            }
          },
          "proximityConnectDistance": {
            "label": {
              "normal": "Proximity Connect Distance",
              "beginner": "Proximity Connect Distance"
            }
          },
          "selectOnlyPiecesOrConnections": {
            "label": {
              "normal": "Select Only Pieces Or Connections",
              "beginner": "Select Only Pieces Or Connections"
            }
          },
          "connection": {
            "rotation": {
              "label": {
                "normal": "Rotation",
                "beginner": "Rotation"
              }
            },
            "tilt": {
              "label": {
                "normal": "Tilt",
                "beginner": "Tilt"
              }
            },
            "turn": {
              "label": {
                "normal": "Turn",
                "beginner": "Turn"
              }
            },
            "plane": {
              "label": {
                "normal": "Plane",
                "beginner": "Plane"
              }
            },
            "translation": {
              "label": {
                "normal": "Translation",
                "beginner": "Translation"
              }
            },
            "orientation": {
              "label": {
                "normal": "Orientation",
                "beginner": "Orientation"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagram",
                "beginner": "Diagram"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Title",
              "beginner": "Title"
            }
          },
          "tools": {
            "label": {
              "normal": "Tools",
              "beginner": "Tools"
            },
            "select": {
              "label": {
                "normal": "Select",
                "beginner": "Selection tool"
              },
              "mode": {
                "additive": {
                  "label": {
                    "normal": "Additive",
                    "beginner": "Additive selection mode - add to existing selection"
                  }
                },
                "subtractive": {
                  "label": {
                    "normal": "Subtractive",
                    "beginner": "Subtractive selection mode - remove from existing selection"
                  }
                },
                "intersect": {
                  "label": {
                    "normal": "Intersect",
                    "beginner": "Intersect selection mode - select only overlapping items"
                  }
                }
              },
              "shape": {
                "rectangular": {
                  "label": {
                    "normal": "Rectangular",
                    "beginner": "Rectangular selection - drag to select in a rectangle"
                  }
                },
                "lasso": {
                  "label": {
                    "normal": "Lasso",
                    "beginner": "Freeform lasso selection - draw a freeform shape"
                  }
                }
              },
              "navigation": {
                "hand": {
                  "label": {
                    "normal": "Hand",
                    "beginner": "Hand tool - pan and navigate the canvas"
                  }
                }
              }
            },
            "lasso": {
              "rectangular": {
                "label": {
                  "normal": "Rectangular lasso",
                  "beginner": "Drag a rectangle for lasso selection"
                }
              },
              "freeform": {
                "label": {
                  "normal": "Freeform lasso",
                  "beginner": "Draw a freehand lasso path"
                }
              }
            }
          },
          "windows": {
            "label": {
              "normal": "Windows",
              "beginner": "Windows"
            }
          },
          "appTitle": {
            "label": {
              "normal": "App Title",
              "beginner": "App Title"
            }
          },
          "canvas": {
            "diagram": {
              "label": {
                "normal": "Diagram",
                "beginner": "Diagram"
              },
              "pieceNode": {
                "label": {
                  "normal": "Piece",
                  "beginner": "Diagram piece node"
                }
              }
            },
            "label": {
              "normal": "Canvas",
              "beginner": "Canvas"
            }
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showPieces": {
              "label": {
                "normal": "Pieces",
                "beginner": "Toggle visibility of design pieces"
              }
            },
            "showConnections": {
              "label": {
                "normal": "Connections",
                "beginner": "Toggle visibility of connections between pieces"
              }
            },
            "showPorts": {
              "label": {
                "normal": "Ports",
                "beginner": "Toggle visibility of ports on pieces"
              }
            },
            "addPiece": {
              "label": {
                "normal": "Add Piece",
                "beginner": "Add a new piece to the design"
              }
            },
            "filters": {
              "label": {
                "normal": "Filters",
                "beginner": "Filter design elements by visibility"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Design settings"
            },
            "theme": {
              "label": {
                "normal": "Theme",
                "beginner": "Choose the color scheme for the application"
              }
            },
            "language": {
              "label": {
                "normal": "Language",
                "beginner": "Choose the language for the application interface"
              },
              "placeholder": {
                "label": {
                  "normal": "Select language...",
                  "beginner": "Select the language in which the application is displayed"
                }
              }
            },
            "device": {
              "label": {
                "normal": "Device",
                "beginner": "Choose the input device"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "Choose your experience level"
              }
            },
            "mode": {
              "label": {
                "normal": "Mode",
                "beginner": "Choose the user interface mode"
              }
            },
            "panel": {
              "label": {
                "normal": "Panels",
                "beginner": "Configure panel visibility"
              },
              "toolbar": {
                "label": {
                  "normal": "Show Toolbar",
                  "beginner": "Toggle the toolbar panel"
                }
              },
              "workbench": {
                "label": {
                  "normal": "Show Workbench",
                  "beginner": "Toggle the workbench panel"
                }
              },
              "windows": {
                "label": {
                  "normal": "Show Windows",
                  "beginner": "Toggle the windows panel"
                }
              },
              "details": {
                "label": {
                  "normal": "Show Details",
                  "beginner": "Toggle the details panel"
                }
              }
            }
          }
        },
        "type": {
          "label": {
            "normal": "Type",
            "beginner": "Type"
          },
          "properties": {
            "label": {
              "normal": "Type Properties",
              "beginner": "Type properties"
            }
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showConnectors": {
              "label": {
                "normal": "Connectors",
                "beginner": "Connectors"
              }
            },
            "showModels": {
              "label": {
                "normal": "Models",
                "beginner": "Models"
              }
            },
            "filters": {
              "label": {
                "normal": "Filters",
                "beginner": "Filter type elements by visibility"
              }
            }
          },
          "footer": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "someAction": {
              "label": {
                "normal": "Some Action",
                "beginner": "Some Action"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "New Type",
              "beginner": "New Type"
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe this type...",
                "beginner": "Describe this type..."
              }
            }
          },
          "iconPlaceholder": {
            "label": {
              "label": {
                "normal": "??",
                "beginner": "??"
              }
            }
          },
          "imagePlaceholder": {
            "label": {
              "label": {
                "normal": "https://example.com/image.png",
                "beginner": "https://example.com/image.png"
              }
            }
          },
          "variantPlaceholder": {
            "label": {
              "label": {
                "normal": "e.g., large, small",
                "beginner": "e.g., large, small"
              }
            }
          },
          "parentPlaceholder": {
            "label": {
              "label": {
                "normal": "Select parent type...",
                "beginner": "Select parent type..."
              }
            }
          },
          "variant": {
            "label": {
              "normal": "Variant",
              "beginner": "Variant"
            }
          },
          "models": {
            "label": {
              "normal": "Models",
              "beginner": "Manage different 3D models, images, and visual models for this type."
            },
            "manual": "type#models",
            "tutorial": "hello-compose/model-brick-set#models"
          },
          "model": {
            "label": {
              "normal": "Model",
              "beginner": "Model"
            }
          },
          "modelDescriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe this model...",
                "beginner": "Describe this model..."
              }
            }
          },
          "modelTagsPlaceholder": {
            "label": {
              "label": {
                "normal": "tag1, tag2, tag3",
                "beginner": "tag1, tag2, tag3"
              }
            }
          },
          "connectors": {
            "label": {
              "normal": "Connectors",
              "beginner": "Manage connection connectors for this type. Connectors define where and how pieces can connect."
            },
            "manual": "type#connectors",
            "tutorial": "hello-compose/model-brick-set#connectors"
          },
          "connector": {
            "label": {
              "normal": "Connector",
              "beginner": "Connector"
            },
            "properties": {
              "label": {
                "normal": "Connector Properties",
                "beginner": "Connector properties"
              }
            },
            "title": {
              "label": {
                "normal": "Title",
                "beginner": "Title"
              }
            }
          },
          "connectorPortPlaceholder": {
            "label": {
              "label": {
                "normal": "e.g., electrical, mechanical",
                "beginner": "e.g., electrical, mechanical"
              }
            }
          },
          "connectorNamePlaceholder": {
            "label": {
              "label": {
                "normal": "Add a name",
                "beginner": "Add a name"
              }
            }
          },
          "connectorDescriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe this connector...",
                "beginner": "Describe this connector..."
              }
            }
          },
          "connectorPoint": {
            "label": {
              "normal": "Point",
              "beginner": "The 3D position of the connector in local coordinates."
            },
            "manual": "type#connectors",
            "tutorial": "hello-compose/model-brick-set#connectors"
          },
          "connectorDirection": {
            "label": {
              "normal": "Direction",
              "beginner": "The outward direction vector of the connector in local coordinates."
            },
            "manual": "type#connectors",
            "tutorial": "hello-compose/model-brick-set#connectors"
          },
          "connectorCompatiblePortsPlaceholder": {
            "label": {
              "label": {
                "normal": "port1, port2",
                "beginner": "port1, port2"
              }
            }
          },
          "connectorNotFound": {
            "label": {
              "normal": "Connector not found",
              "beginner": "Connector not found"
            }
          },
          "connectorsNotFound": {
            "label": {
              "normal": "No connectors found",
              "beginner": "No connectors found"
            }
          },
          "authors": {
            "label": {
              "normal": "Authors",
              "beginner": "Authors"
            }
          },
          "author": {
            "label": {
              "normal": "Author",
              "beginner": "Author"
            }
          },
          "attributes": {
            "label": {
              "normal": "Attributes",
              "beginner": "Attributes"
            }
          },
          "attribute": {
            "label": {
              "normal": "Attribute",
              "beginner": "Attribute"
            }
          },
          "attributeValuePlaceholder": {
            "label": {
              "label": {
                "normal": "Value...",
                "beginner": "Value..."
              }
            }
          },
          "attributeDefinitionPlaceholder": {
            "label": {
              "label": {
                "normal": "Definition or URL...",
                "beginner": "Definition or URL..."
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "type": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The name of the type. This is the primary identifier for the component."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A detailed description of what this type represents and how it should be used."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "icon": {
                    "label": {
                      "normal": "Icon",
                      "beginner": "An icon to visually represent this type. Can be an emoji, icon name, or URL to an image."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "image": {
                    "label": {
                      "normal": "Image",
                      "beginner": "URL to an image that represents this type. Used for previews and visual identification."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "unit": {
                    "label": {
                      "normal": "Unit",
                      "beginner": "The unit of measurement used for this type (e.g., mm, m, ft)."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "parent": {
                    "label": {
                      "normal": "Parent",
                      "beginner": "The parent type this type inherits from"
                    }
                  },
                  "abstract": {
                    "label": {
                      "normal": "Abstract",
                      "beginner": "Whether this is an abstract type"
                    }
                  }
                },
                "models": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "url": {
                    "label": {
                      "normal": "URL",
                      "beginner": "URL to a 3D model, image, or other resource representing this type."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of what this model shows or how it should be used."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "tags": {
                    "label": {
                      "normal": "Tags",
                      "beginner": "Tags to categorize and filter models (e.g., 'detailed', 'simplified', 'lod1')."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-compose/model-brick-set"
                  }
                },
                "connectors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "port": {
                    "label": {
                      "normal": "Port",
                      "beginner": "Connector port name. Connectors of the same port can connect to each other."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "compatiblePorts": {
                    "label": {
                      "normal": "Compatible Ports",
                      "beginner": "List of other connector ports this connector can connect to. Leave empty to allow all ports."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of what this connector represents and how it should be used for connections."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "t": {
                    "label": {
                      "normal": "T",
                      "beginner": "Position on the diagram ring (0-1). Controls where the connector appears in the 2D diagram view."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "ring": {
                    "label": {
                      "normal": "Ring",
                      "beginner": "Position on the diagram ring (0-1). Controls where the connector appears in the 2D diagram view."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "direction": {
                    "label": {
                      "normal": "",
                      "beginner": ""
                    },
                    "x": {
                      "label": {
                        "normal": "X",
                        "beginner": "X coordinate of the connector direction vector. This defines which direction the connector points in 3D space."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    },
                    "y": {
                      "label": {
                        "normal": "Y",
                        "beginner": "Y coordinate of the connector direction vector. This defines which direction the connector points in 3D space."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    },
                    "z": {
                      "label": {
                        "normal": "Z",
                        "beginner": "Z coordinate of the connector direction vector. This defines which direction the connector points in 3D space."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "An optional name for this connector."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "mandatory": {
                    "label": {
                      "normal": "Mandatory",
                      "beginner": "Whether this connector is required for a valid connection."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "maxChildren": {
                    "label": {
                      "normal": "Max Children",
                      "beginner": "The maximum number of connections allowed at this connector."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "point": {
                    "label": {
                      "normal": "",
                      "beginner": ""
                    },
                    "x": {
                      "label": {
                        "normal": "X",
                        "beginner": "X position of the connector in 3D space relative to the type's origin."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    },
                    "y": {
                      "label": {
                        "normal": "Y",
                        "beginner": "Y position of the connector in 3D space relative to the type's origin."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    },
                    "z": {
                      "label": {
                        "normal": "Z",
                        "beginner": "Z position of the connector in 3D space relative to the type's origin."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-compose/model-brick-set"
                    }
                  }
                },
                "connector": {
                  "ring": {
                    "label": {
                      "normal": "Ring",
                      "beginner": "Position on the diagram ring (0-1). Controls where the connector appears in the 2D diagram view."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-compose/model-brick-set"
                  }
                },
                "attributes": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The name of the attribute in kebab-case (e.g., 'material.wood', 'cost.labor')."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "value": {
                    "label": {
                      "normal": "Value",
                      "beginner": "The value of the attribute. Leave empty for boolean attributes (presence = true)."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "definition": {
                    "label": {
                      "normal": "Definition",
                      "beginner": "Optional definition or documentation for this attribute. Can be text or a URL."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-compose/model-brick-set"
                  }
                },
                "authors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Full name of the author or contributor."
                    },
                    "manual": "type#authors",
                    "tutorial": "hello-compose/model-brick-set"
                  },
                  "email": {
                    "label": {
                      "normal": "Email",
                      "beginner": "Email address for contacting the author."
                    },
                    "manual": "type#authors",
                    "tutorial": "hello-compose/model-brick-set"
                  }
                }
              }
            }
          },
          "tools": {
            "label": {
              "normal": "Tools",
              "beginner": "Tools"
            },
            "select": {
              "normal": {
                "label": {
                  "normal": "Normal",
                  "beginner": "Normal selection"
                }
              },
              "additive": {
                "label": {
                  "normal": "Additive",
                  "beginner": "Additive selection - add to existing selection"
                }
              },
              "subtractive": {
                "label": {
                  "normal": "Subtractive",
                  "beginner": "Subtractive selection - remove from existing selection"
                }
              },
              "intersect": {
                "label": {
                  "normal": "Intersect",
                  "beginner": "Intersect selection - select only overlapping items"
                }
              }
            },
            "lasso": {
              "rectangular": {
                "label": {
                  "normal": "Rectangular",
                  "beginner": "Rectangular lasso selection"
                }
              },
              "freeform": {
                "label": {
                  "normal": "Freeform",
                  "beginner": "Freeform lasso selection"
                }
              }
            },
            "selection": {
              "label": {
                "normal": "Selection",
                "beginner": "Selection tool"
              }
            },
            "hand": {
              "label": {
                "normal": "Hand",
                "beginner": "Hand tool - pan and navigate"
              }
            },
            "connector": {
              "label": {
                "normal": "Connector",
                "beginner": "Connector creation tool"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Title",
              "beginner": "Title"
            }
          }
        },
        "quality": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "description": {
            "label": {
              "normal": "Define measurement qualities",
              "beginner": "Define measurement qualities"
            }
          },
          "title": {
            "label": {
              "normal": "Quality",
              "beginner": "Quality"
            }
          },
          "toolbar": {
            "view": {
              "label": {
                "normal": "View",
                "beginner": "View options"
              }
            },
            "actions": {
              "label": {
                "normal": "Actions",
                "beginner": "Quality actions"
              }
            }
          },
          "workbench": {
            "nodes": {
              "label": {
                "normal": "Nodes",
                "beginner": "Quality formula nodes"
              }
            },
            "qualities": {
              "label": {
                "normal": "Qualities",
                "beginner": "Available qualities"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Quality settings"
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Quality chat"
            }
          },
          "tools": {
            "selection": {
              "label": {
                "normal": "Selection",
                "beginner": "Selection tool"
              }
            },
            "select": {
              "additive": {
                "label": {
                  "normal": "Additive",
                  "beginner": "Additive selection - add to existing selection"
                }
              },
              "subtractive": {
                "label": {
                  "normal": "Subtractive",
                  "beginner": "Subtractive selection - remove from existing selection"
                }
              },
              "intersect": {
                "label": {
                  "normal": "Intersect",
                  "beginner": "Intersect selection - select only overlapping items"
                }
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "New Quality",
              "beginner": "New Quality"
            }
          },
          "numericFunctions": {
            "label": {
              "normal": "Numeric Functions",
              "beginner": "Numeric Functions"
            }
          },
          "add": {
            "label": {
              "normal": "Add",
              "beginner": "Add"
            }
          },
          "subtract": {
            "label": {
              "normal": "Subtract",
              "beginner": "Subtract"
            }
          },
          "multiply": {
            "label": {
              "normal": "Multiply",
              "beginner": "Multiply"
            }
          },
          "divide": {
            "label": {
              "normal": "Divide",
              "beginner": "Divide"
            }
          },
          "branchingFunctions": {
            "label": {
              "normal": "Branching Functions",
              "beginner": "Branching Functions"
            }
          },
          "if": {
            "label": {
              "normal": "If",
              "beginner": "If"
            }
          },
          "switch": {
            "label": {
              "normal": "Switch",
              "beginner": "Switch"
            }
          },
          "dataStructures": {
            "label": {
              "normal": "Data Structures",
              "beginner": "Data Structures"
            }
          },
          "list": {
            "label": {
              "normal": "List",
              "beginner": "List"
            }
          },
          "dictionary": {
            "label": {
              "normal": "Dictionary",
              "beginner": "Dictionary"
            }
          },
          "noQualities": {
            "label": {
              "normal": "No qualities defined",
              "beginner": "No qualities defined"
            }
          },
          "key": {
            "label": {
              "normal": "Key",
              "beginner": "The unique identifier for this quality"
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "The display name for this quality"
            }
          },
          "kind": {
            "label": {
              "normal": "Kind",
              "beginner": "The type of entity this quality applies to"
            }
          },
          "formula": {
            "label": {
              "normal": "Formula",
              "beginner": "The formula to calculate this quality"
            }
          },
          "formulaPlaceholder": "Enter formula...",
          "defaultValue": {
            "label": {
              "normal": "Default value",
              "beginner": "The default value for this quality"
            }
          },
          "defaultSiUnit": {
            "label": {
              "normal": "Default SI unit",
              "beginner": "The default unit in the SI system"
            }
          },
          "defaultImperialUnit": {
            "label": {
              "normal": "Default Imperial unit",
              "beginner": "The default unit in the Imperial system"
            }
          },
          "min": {
            "label": {
              "normal": "Minimum",
              "beginner": "The minimum allowed value"
            }
          },
          "isMinExcluded": {
            "label": {
              "normal": "Exclude minimum",
              "beginner": "Whether the minimum value is excluded"
            }
          },
          "max": {
            "label": {
              "normal": "Maximum",
              "beginner": "The maximum allowed value"
            }
          },
          "isMaxExcluded": {
            "label": {
              "normal": "Exclude maximum",
              "beginner": "Whether the maximum value is excluded"
            }
          },
          "canScale": {
            "label": {
              "normal": "Can scale",
              "beginner": "Whether this quality scales with piece size"
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "key": {
                "label": {
                  "normal": "Key",
                  "beginner": "The unique key identifier of the quality"
                }
              },
              "name": {
                "label": {
                  "normal": "Name",
                  "beginner": "The display name of the quality"
                }
              },
              "description": {
                "label": {
                  "normal": "Description",
                  "beginner": "A description of what this quality measures"
                }
              },
              "formula": {
                "label": {
                  "normal": "Formula",
                  "beginner": "The formula to calculate this quality"
                }
              },
              "defaultSiUnit": {
                "label": {
                  "normal": "Default SI Unit",
                  "beginner": "The default unit in the SI system"
                }
              },
              "defaultImperialUnit": {
                "label": {
                  "normal": "Default Imperial Unit",
                  "beginner": "The default unit in the Imperial system"
                }
              },
              "kind": {
                "label": {
                  "normal": "Kind",
                  "beginner": "The type of entity this quality applies to"
                }
              },
              "canScale": {
                "label": {
                  "normal": "Can Scale",
                  "beginner": "Whether this quality scales with piece size"
                }
              },
              "defaultValue": {
                "label": {
                  "normal": "Default Value",
                  "beginner": "The default value for this quality"
                }
              },
              "min": {
                "label": {
                  "normal": "Minimum",
                  "beginner": "The minimum allowed value"
                }
              },
              "max": {
                "label": {
                  "normal": "Maximum",
                  "beginner": "The maximum allowed value"
                }
              },
              "isMinExcluded": {
                "label": {
                  "normal": "Minimum Excluded",
                  "beginner": "Whether the minimum value is exclusive"
                }
              },
              "isMaxExcluded": {
                "label": {
                  "normal": "Maximum Excluded",
                  "beginner": "Whether the maximum value is exclusive"
                }
              }
            }
          },
          "functions": {
            "label": {
              "normal": "Functions",
              "beginner": "Functions"
            }
          },
          "qualities": {
            "label": {
              "normal": "Qualities",
              "beginner": "Qualities"
            }
          }
        },
        "docs": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "noHeadings": {
            "label": {
              "normal": "No headings found",
              "beginner": "No headings found"
            }
          },
          "docs": {
            "label": {
              "normal": "Docs",
              "beginner": "Docs"
            }
          },
          "overview": {
            "label": {
              "normal": "Overview",
              "beginner": "Overview"
            }
          },
          "page": {
            "label": {
              "normal": "Page",
              "beginner": "Page"
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Settings"
            },
            "theme": {
              "label": {
                "normal": "Theme",
                "beginner": "Choose the color theme"
              }
            },
            "language": {
              "label": {
                "normal": "Language",
                "beginner": "Select the language"
              }
            },
            "device": {
              "label": {
                "normal": "Device",
                "beginner": "Select the input device"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "Select your expertise level"
              }
            },
            "mode": {
              "label": {
                "normal": "Mode",
                "beginner": "Select user or developer mode"
              }
            }
          },
          "navigation": {
            "previous": {
              "label": {
                "normal": "Previous",
                "beginner": "Navigate to the previous page"
              }
            },
            "next": {
              "label": {
                "normal": "Next",
                "beginner": "Navigate to the next page"
              }
            }
          }
        },
        "feedback": {
          "label": {
            "normal": "Feedback",
            "beginner": "Send feedback to help improve Compose"
          },
          "title": {
            "label": {
              "normal": "Feedback",
              "beginner": "Feedback"
            }
          },
          "subtitle": {
            "label": {
              "normal": "Help us improve compose by reporting bugs or sharing ideas.",
              "beginner": "Help us improve compose by reporting bugs or sharing ideas."
            }
          },
          "form": {
            "kind": {
              "label": {
                "normal": "Type",
                "beginner": "Select bug report or feature idea"
              }
            },
            "title": {
              "label": {
                "normal": "Title",
                "beginner": "A brief summary of your feedback"
              }
            },
            "titlePlaceholder": {
              "label": {
                "normal": "Enter a brief title...",
                "beginner": "Enter a brief title..."
              }
            },
            "description": {
              "label": {
                "normal": "Description",
                "beginner": "Detailed description of the problem or idea"
              }
            },
            "bugDescriptionPlaceholder": {
              "label": {
                "normal": "Describe what happened...",
                "beginner": "Describe what happened..."
              }
            },
            "ideaDescriptionPlaceholder": {
              "label": {
                "normal": "Describe your idea...",
                "beginner": "Describe your idea..."
              }
            },
            "app": {
              "label": {
                "normal": "App",
                "beginner": "Which app did the bug occur in?"
              }
            },
            "appPlaceholder": {
              "label": {
                "normal": "Select app...",
                "beginner": "Select app..."
              }
            },
            "name": {
              "label": {
                "normal": "Name",
                "beginner": "Your name (optional)"
              }
            },
            "namePlaceholder": {
              "label": {
                "normal": "Your name (optional)",
                "beginner": "Your name (optional)"
              }
            },
            "email": {
              "label": {
                "normal": "Email",
                "beginner": "Your email address (optional)"
              }
            },
            "emailPlaceholder": {
              "label": {
                "normal": "your@email.com (optional)",
                "beginner": "your@email.com (optional)"
              }
            },
            "submit": {
              "label": {
                "normal": "Submit Feedback",
                "beginner": "Submit your feedback"
              }
            },
            "submitting": {
              "label": {
                "normal": "Submitting...",
                "beginner": "Submitting..."
              }
            }
          },
          "kind": {
            "bug": {
              "label": {
                "normal": "Bug Report",
                "beginner": "Report a problem or error"
              }
            },
            "idea": {
              "label": {
                "normal": "Feature Idea",
                "beginner": "Suggest a new feature or improvement"
              }
            }
          },
          "appOption": {
            "home": {
              "label": {
                "normal": "Home",
                "beginner": "Home"
              }
            },
            "kit": {
              "label": {
                "normal": "Kit",
                "beginner": "Kit"
              }
            },
            "design": {
              "label": {
                "normal": "Design",
                "beginner": "Design"
              }
            },
            "type": {
              "label": {
                "normal": "Type",
                "beginner": "Type"
              }
            },
            "quality": {
              "label": {
                "normal": "Quality",
                "beginner": "Quality"
              }
            },
            "docs": {
              "label": {
                "normal": "Docs",
                "beginner": "Docs"
              }
            },
            "feedback": {
              "label": {
                "normal": "Feedback",
                "beginner": "Feedback"
              }
            }
          },
          "optional": {
            "label": {
              "normal": "Optional contact information",
              "beginner": "Optional contact information"
            }
          },
          "success": {
            "thankYou": {
              "label": {
                "normal": "Thank You!",
                "beginner": "Thank You!"
              }
            },
            "message": {
              "label": {
                "normal": "Your feedback has been received. We appreciate your contribution!",
                "beginner": "Your feedback has been received. We appreciate your contribution!"
              }
            },
            "sendAnother": {
              "label": {
                "normal": "Send Another",
                "beginner": "Submit another feedback"
              }
            },
            "goHome": {
              "label": {
                "normal": "Go Home",
                "beginner": "Return to home page"
              }
            }
          },
          "error": {
            "titleRequired": {
              "label": {
                "normal": "Title is required",
                "beginner": "Title is required"
              }
            },
            "descriptionRequired": {
              "label": {
                "normal": "Description is required",
                "beginner": "Description is required"
              }
            },
            "appRequired": {
              "label": {
                "normal": "Please select which app the bug occurred in",
                "beginner": "Please select which app the bug occurred in"
              }
            },
            "submitFailed": {
              "label": {
                "normal": "Failed to submit feedback. Please try again.",
                "beginner": "Failed to submit feedback. Please try again."
              }
            }
          },
          "toolbar": {
            "send": {
              "label": {
                "normal": "Send",
                "beginner": "Send feedback"
              }
            }
          }
        }
      },
      "docs": {
        "navigation": {
          "previous": {
            "label": {
              "normal": "Previous",
              "beginner": "Previous"
            }
          },
          "next": {
            "label": {
              "normal": "Next",
              "beginner": "Next"
            }
          }
        }
      },
      "toolbar": {
        "label": {
          "normal": "Toolbar",
          "beginner": "Toolbar"
        },
        "group": {
          "hand": {
            "label": {
              "normal": "Hand",
              "beginner": "Hand"
            }
          },
          "selection": {
            "label": {
              "normal": "Selection",
              "beginner": "Selection"
            }
          },
          "lasso": {
            "label": {
              "normal": "Lasso",
              "beginner": "Lasso"
            }
          },
          "filter": {
            "label": {
              "normal": "Filter",
              "beginner": "Filter"
            }
          },
          "open": {
            "label": {
              "normal": "Open",
              "beginner": "Open"
            }
          },
          "create": {
            "label": {
              "normal": "Create",
              "beginner": "Create"
            }
          },
          "view": {
            "label": {
              "normal": "View",
              "beginner": "View"
            }
          },
          "actions": {
            "label": {
              "normal": "Actions",
              "beginner": "Actions"
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Settings"
            }
          }
        },
        "parent": {}
      },
      "tutorial": {
        "controls": {
          "stop": {
            "label": {
              "normal": "Stop",
              "beginner": "Stop"
            }
          },
          "previous": {
            "label": {
              "normal": "Previous",
              "beginner": "Previous"
            }
          },
          "playPause": {
            "label": {
              "normal": "Play Pause",
              "beginner": "Play Pause"
            }
          },
          "next": {
            "label": {
              "normal": "Next",
              "beginner": "Next"
            }
          }
        }
      },
      "recording": {
        "controls": {
          "playPause": {
            "label": {
              "normal": "Play Pause",
              "beginner": "Play Pause"
            }
          },
          "stop": {
            "label": {
              "normal": "Stop",
              "beginner": "Stop"
            }
          }
        }
      }
    }
  }
}`),
  },
} as const;

applyComposeSketchpadToolbarParentEntries(
	composeSketchpadTranslationBundles as Record<UiLocale, { translation: ComposeSketchpadTranslationTree }>,
);

/** @emoji 🏷️ Maps generic `ui.*` shell control ids to compose sketchpad i18n keys. */
export function sketchpadResolveControlLabelId(id: string): ComposeSketchpadControlTranslationKey | string {
	if (id.startsWith("ui.nav.")) {
		const segment = id.slice("ui.nav.".length);
		if (segment === "back" || segment === "forward" || segment === "up") {
			return `compose.sketchpad.navbar.${segment}`;
		}
	}
	if (id === "ui.search.toggle") {
		return "compose.sketchpad.navbar.search.open";
	}
	if (id === "ui.find.toggle") {
		return "compose.sketchpad.navbar.find.open";
	}
	if (id.startsWith("ui.panelToggle.")) {
		return `compose.sketchpad.navbar.panelToggle.${id.slice("ui.panelToggle.".length)}`;
	}
	if (id.startsWith("ui.toolbar.group.")) {
		return `compose.sketchpad.toolbar.parent.${id.slice("ui.toolbar.group.".length)}`;
	}
	return id;
}

/** @emoji 🪁 Registers compose sketchpad translation bundles and shell label resolver on the shared UI i18n instance. */
export function registerComposeSketchpadUiChrome(): void {
	registerUiTranslationBundles(composeSketchpadTranslationBundles);
	setControlLabelIdResolver(sketchpadResolveControlLabelId);
}

registerComposeSketchpadUiChrome();
//#endregion 🪁ComposeUiI18n


//#region 🔖KitImport
type ComposeBundleJson = Record<string, unknown>;

/** @emoji 🧾 Recursively flattens `{ items: [...] }` and Relay `edges` for GraphQL install payloads. */
function composeDenormalizeBundleValue(v: unknown): unknown {
	if (v == null || typeof v !== "object") return v;
	if (Array.isArray(v)) return v.map(composeDenormalizeBundleValue);
	const o = v as ComposeBundleJson;
	if (Array.isArray(o["items"])) return (o["items"] as unknown[]).map(composeDenormalizeBundleValue);
	if (Array.isArray(o["edges"])) {
		const out: unknown[] = [];
		for (const e of o["edges"] as unknown[]) {
			if (e != null && typeof e === "object" && !Array.isArray(e) && "node" in (e as ComposeBundleJson)) {
				out.push(composeDenormalizeBundleValue((e as ComposeBundleJson)["node"]));
			}
		}
		return out;
	}
	const flat: ComposeBundleJson = {};
	for (const [k, val] of Object.entries(o)) flat[k] = composeDenormalizeBundleValue(val) as never;
	return flat;
}

/** @emoji 🧾 Lifts `*.kit.compose.json` (`initialKit` / `wip.initialKit`) then flattens bundle lists. */
export function decodeKitComposeEnvelopeToFullFromValue(v: unknown): unknown {
	let inner: unknown = v;
	if (inner && typeof inner === "object" && !Array.isArray(inner)) {
		const top = inner as ComposeBundleJson;
		if (top["initialKit"] != null && typeof top["initialKit"] === "object" && !Array.isArray(top["initialKit"])) {
			inner = top["initialKit"];
		} else if (top["wip"] != null && typeof top["wip"] === "object" && !Array.isArray(top["wip"])) {
			const wr = (top["wip"] as ComposeBundleJson)["initialKit"];
			if (wr != null && typeof wr === "object" && !Array.isArray(wr)) inner = wr;
		}
	}
	return composeDenormalizeBundleValue(inner);
}

/** @emoji 🧾 Reads a kit DTO root from a decoded compose bundle value. */
export function sketchpadKitFromDecodedBundle(value: unknown): Kit | null {
	const denorm = decodeKitComposeEnvelopeToFullFromValue(value);
	if (denorm == null || typeof denorm !== "object" || Array.isArray(denorm)) return null;
	if ("id" in denorm) return denorm as Kit;
	return null;
}

/** @emoji 📦 Decode gzip-or-JSON kit bytes into a live {@link Kit} via {@link Session.openInMemory}. */
export async function importKit(
	data: ArrayBuffer | Uint8Array | Blob | File | string,
): Promise<{ readonly kit: Kit; readonly session: Session; readonly portCompatSource: Kit }> {
	let bytes: Uint8Array;
	if (typeof data === "string") {
		const res = await fetch(data);
		bytes = new Uint8Array(await res.arrayBuffer());
	} else if (data instanceof Uint8Array) {
		bytes = data;
	} else if (data instanceof ArrayBuffer) {
		bytes = new Uint8Array(data);
	} else {
		bytes = new Uint8Array(await data.arrayBuffer());
	}
	if (bytes.length >= 2 && bytes[0] === 0x1f && bytes[1] === 0x8b) {
		bytes = gunzipSync(bytes);
	}
	const text = new TextDecoder().decode(bytes);
	const plainUnknown = decodeKitComposeEnvelopeToFullFromValue(JSON.parse(text));
	const payload = typeof plainUnknown === "object" && plainUnknown != null ? JSON.stringify(plainUnknown) : String(plainUnknown);
	const bundleKit = sketchpadKitFromDecodedBundle(plainUnknown);
	const session = await ComposeSession.openInMemory();
	const stores = await session.stores();
	if (stores.length === 0) throw new Error("compose/sketchpad: importKit found zero stores after openInMemory");
	const store = stores[0]!;
	const installed = await store.installProjection(payload);
	if (!installed.ok) throw new Error(`compose/sketchpad: importKit installProjection failed: ${installed.error?.message ?? "unknown"}`);
	const kitDto = await sketchpadKitDtoFromJsStore(store);
	const portCompatSource = (bundleKit ?? kitDto) as Kit;
	const compat = sketchpadMergePortCompatMaps(sketchpadExtractPortCompatById(portCompatSource), sketchpadExtractPortCompatById(kitDto));
	const kit = sketchpadApplyPortCompatById(sketchpadMergeKitDtoFromBundleProjection(kitDto, portCompatSource), compat);
	return { kit, session, portCompatSource };
}

/** @emoji 📤 Wraps a kit DTO in the `wip.initialKit` envelope used by {@link importKit}. */
export function sketchpadKitToComposeEnvelope(kit: Kit): { readonly wip: { readonly initialKit: Kit } } {
	return { wip: { initialKit: kit } };
}

/** @emoji 💾 Triggers a browser download of kit JSON (compose envelope). */
export function sketchpadDownloadKitJson(kit: Kit, filename?: string): void {
	if (typeof document === "undefined") return;
	const json = JSON.stringify(sketchpadKitToComposeEnvelope(kit), null, 2);
	const blob = new Blob([json], { type: "application/json" });
	const url = URL.createObjectURL(blob);
	const anchor = document.createElement("a");
	const safeName = (kit.name ?? kit.id ?? "kit").replace(/[^\w.-]+/g, "-");
	anchor.href = url;
	anchor.download = filename ?? `${safeName}.kit.compose.json`;
	anchor.click();
	URL.revokeObjectURL(url);
}

/** @emoji 📋 Copies kit JSON (compose envelope) to the clipboard when available. */
export async function sketchpadCopyKitJsonToClipboard(kit: Kit): Promise<boolean> {
	if (typeof navigator === "undefined" || !navigator.clipboard?.writeText) return false;
	await navigator.clipboard.writeText(JSON.stringify(sketchpadKitToComposeEnvelope(kit), null, 2));
	return true;
}
//#endregion 🔖KitImport

//#region 🔖KitHost
export type SketchpadKitPersistenceKind = "temporary" | "file" | "folder" | "remote" | "fixture";

/** @emoji 🏭 Host-provided kit open factory (Electron, VS Code, browser file picker, …). */
export type SketchpadKitBackendFactory = () => Promise<ComposeKitStore>;

let sketchpadKitBackendFactories: Partial<Record<SketchpadKitPersistenceKind, SketchpadKitBackendFactory>> = {};

function sketchpadKitStoreFromFactory(result: ComposeKitStore | ComposeKitStoreBackend): ComposeKitStore {
	return result instanceof ComposeKitStore ? result : new ComposeKitStore(result);
}

function sketchpadPromptServerUrl(preset?: string): string | null {
	if (typeof window === "undefined" || typeof window.prompt !== "function") return preset ?? null;
	return window.prompt("Compose store URL", preset ?? "http://localhost:8080");
}

/** @emoji 🌐 Default browser remote kit factory ({@link Session.openHttp}). */
export async function sketchpadDefaultRemoteKitFactory(): Promise<ComposeKitStore> {
	const serverUrl = sketchpadPromptServerUrl()?.trim();
	if (!serverUrl) throw new Error("compose/sketchpad: remote kit open cancelled");
	return sketchpadOpenRemoteKitStore(serverUrl);
}

/** @emoji 🌐 Opens an HTTP {@link Session} kit and returns a {@link ComposeJsKitStore}. */
export async function sketchpadOpenRemoteKitStore(serverUrl: string): Promise<ComposeJsKitStore> {
	const session = await ComposeSession.openHttp(serverUrl);
	const stores = await session.stores();
	const jsStore = stores[0];
	if (!jsStore) {
		await session.dispose();
		throw new Error("compose/sketchpad: remote session has no stores");
	}
	return createComposeKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose() });
}

/** @emoji 🔧 Registers host kit open factories used by {@link SketchpadShellController} `openKit` commands. */
export function configureSketchpadKitFactories(factories: Partial<Record<SketchpadKitPersistenceKind, SketchpadKitBackendFactory>>): void {
	sketchpadKitBackendFactories = { remote: sketchpadDefaultRemoteKitFactory, ...sketchpadKitBackendFactories, ...factories };
}

configureSketchpadKitFactories({});

/** @emoji 📂 Picks a kit archive or JSON file in the browser (File System Access API or hidden input). */
export async function sketchpadPickKitImportFile(): Promise<File | null> {
	if (typeof window === "undefined") return null;
	const accept = {
		"application/json": [".json", ".compose.json"],
		"application/zip": [".zip", ".compose.zip"],
		"application/gzip": [".gz"],
		"application/x-gzip": [".gz"],
	};
	if ("showOpenFilePicker" in window) {
		try {
			const handles = await (
				window as Window & { showOpenFilePicker: (o: unknown) => Promise<FileSystemFileHandle[]> }
			).showOpenFilePicker({
				multiple: false,
				types: [{ description: "Compose kit", accept }],
			});
			const handle = handles[0];
			return handle ? await handle.getFile() : null;
		} catch {
			return null;
		}
	}
	return new Promise((resolve) => {
		const input = document.createElement("input");
		input.type = "file";
		input.accept = ".json,.compose.json,.zip,.compose.zip,.gz,application/json,application/zip";
		input.onchange = () => resolve(input.files?.[0] ?? null);
		input.click();
	});
}

/** @emoji 📂 Opens a user-selected kit file via {@link importKit} and returns a {@link ComposeJsKitStore}. */
export async function sketchpadBrowserFileKitFactory(): Promise<ComposeJsKitStore> {
	const file = await sketchpadPickKitImportFile();
	if (!file) throw new Error("compose/sketchpad: file kit open cancelled");
	const { session, portCompatSource } = await importKit(file);
	const jsStore = (await session.stores())[0];
	if (!jsStore) {
		await session.dispose();
		throw new Error("compose/sketchpad: file kit open found no stores");
	}
	return createComposeKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose(), portCompatSource });
}

/** @emoji 📁 Opens a folder kit when {@link showDirectoryPicker} is available (kit.compose.json at folder root). */
export async function sketchpadBrowserFolderKitFactory(): Promise<ComposeJsKitStore> {
	if (typeof window === "undefined" || !("showDirectoryPicker" in window)) {
		throw new Error("compose/sketchpad: folder kit open requires showDirectoryPicker");
	}
	const dir = await (
		window as Window & { showDirectoryPicker: () => Promise<FileSystemDirectoryHandle> }
	).showDirectoryPicker();
	const kitFile =
		(await dir.getFileHandle("kit.compose.json", { create: false }).then((h) => h.getFile()).catch(() => null)) ??
		(await dir.getFileHandle("wip/initialKit/kit.compose.json", { create: false }).then((h) => h.getFile()).catch(() => null));
	if (!kitFile) throw new Error("compose/sketchpad: no kit.compose.json in selected folder");
	const { session, portCompatSource } = await importKit(kitFile);
	const jsStore = (await session.stores())[0];
	if (!jsStore) {
		await session.dispose();
		throw new Error("compose/sketchpad: folder kit open found no stores");
	}
	return createComposeKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose(), portCompatSource });
}

/** @emoji 🌐 Registers browser file/folder/remote kit factories for {@link SketchpadShellController}. */
export function sketchpadConfigureBrowserKitFactories(): void {
	if (typeof window === "undefined") return;
	configureSketchpadKitFactories({
		file: sketchpadBrowserFileKitFactory,
		folder: sketchpadBrowserFolderKitFactory,
		remote: sketchpadDefaultRemoteKitFactory,
	});
}

let sketchpadHomeDropzoneInstalled = false;
let sketchpadHomeDropzoneDragDepth = 0;

const SKETCHPAD_HOME_DROPZONE_OVERLAY_ID = "compose-sketchpad-home-dropzone-overlay";
const SKETCHPAD_HOME_KIT_FILE_INPUT_ID = "compose-sketchpad-home-kit-file-input";

function sketchpadHomeRouteActive(): boolean {
	return (getSketchpadPlatform()?.uri.split("?")[0] ?? "/") === "/";
}

function sketchpadTransferHasKitArchive(transfer: DataTransfer | null): boolean {
	if (!transfer) return false;
	if (transfer.types.includes("Files")) return true;
	const file = transfer.files?.[0];
	if (!file) return false;
	return /\.(compose\.)?zip$/i.test(file.name) || file.type.includes("zip");
}

/** @emoji 🖼️ Toggles the full-screen home kit import drop overlay. */
export function sketchpadSetHomeDropzoneOverlayVisible(visible: boolean): void {
	if (typeof document === "undefined") return;
	let overlay = document.getElementById(SKETCHPAD_HOME_DROPZONE_OVERLAY_ID);
	if (!overlay && visible) {
		overlay = document.createElement("div");
		overlay.id = SKETCHPAD_HOME_DROPZONE_OVERLAY_ID;
		overlay.setAttribute("data-testid", "sketchpad-home-dropzone-overlay");
		overlay.className =
			"pointer-events-none fixed inset-0 z-50 flex items-center justify-center bg-base/80 backdrop-blur-sm";
		const inner = document.createElement("div");
		inner.className = "flex flex-col items-center gap-2 px-6 text-center";
		const title = document.createElement("p");
		title.className = "text-lg font-medium";
		title.textContent = "Drop kit archive";
		const hint = document.createElement("p");
		hint.className = "text-sm text-muted-foreground";
		hint.textContent = "Release a .zip or .compose.zip file to import";
		inner.append(title, hint);
		overlay.append(inner);
		document.body.appendChild(overlay);
	}
	if (overlay) overlay.classList.toggle("hidden", !visible);
}

function sketchpadEnsureHomeKitFileInput(): HTMLInputElement {
	let input = document.getElementById(SKETCHPAD_HOME_KIT_FILE_INPUT_ID) as HTMLInputElement | null;
	if (!input) {
		input = document.createElement("input");
		input.type = "file";
		input.id = SKETCHPAD_HOME_KIT_FILE_INPUT_ID;
		input.accept = ".zip,.compose.zip,application/zip,application/x-zip-compressed";
		input.className = "hidden";
		input.setAttribute("data-testid", SKETCHPAD_HOME_KIT_FILE_INPUT_ID);
		document.body.appendChild(input);
	}
	return input;
}

/** @emoji 📂 Opens the hidden home kit archive file picker (`.zip` / `.compose.zip`). */
export function sketchpadPromptHomeKitArchiveFile(): void {
	if (typeof document === "undefined") return;
	sketchpadEnsureHomeKitFileInput().click();
}

/** @emoji 📥 Installs document-level home drag/drop (overlay + kit import on `/`). */
export function sketchpadInstallHomeDropzone(): void {
	if (typeof window === "undefined" || sketchpadHomeDropzoneInstalled) return;
	sketchpadHomeDropzoneInstalled = true;
	const fileInput = sketchpadEnsureHomeKitFileInput();
	fileInput.addEventListener("change", () => {
		const file = fileInput.files?.[0];
		fileInput.value = "";
		if (!file) return;
		const ctrl = getSketchpadShellController();
		if (!ctrl) return;
		ctrl.run("importKitFromDrop", { file });
	});
	const onDragEnter = (event: DragEvent) => {
		if (!sketchpadHomeRouteActive()) return;
		if (!sketchpadTransferHasKitArchive(event.dataTransfer)) return;
		event.preventDefault();
		sketchpadHomeDropzoneDragDepth += 1;
		sketchpadSetHomeDropzoneOverlayVisible(true);
	};
	const onDragOver = (event: DragEvent) => {
		if (!sketchpadHomeRouteActive()) return;
		if (!sketchpadTransferHasKitArchive(event.dataTransfer)) return;
		event.preventDefault();
	};
	const onDragLeave = (event: DragEvent) => {
		if (!sketchpadHomeRouteActive()) return;
		if (sketchpadHomeDropzoneDragDepth <= 0) return;
		sketchpadHomeDropzoneDragDepth -= 1;
		if (sketchpadHomeDropzoneDragDepth === 0) sketchpadSetHomeDropzoneOverlayVisible(false);
	};
	const onDrop = (event: DragEvent) => {
		if (!sketchpadHomeRouteActive()) return;
		event.preventDefault();
		sketchpadHomeDropzoneDragDepth = 0;
		sketchpadSetHomeDropzoneOverlayVisible(false);
		const file = event.dataTransfer?.files?.[0];
		if (!file) return;
		const ctrl = getSketchpadShellController();
		if (!ctrl) return;
		ctrl.run("importKitFromDrop", { file });
	};
	window.addEventListener("dragenter", onDragEnter);
	window.addEventListener("dragover", onDragOver);
	window.addEventListener("dragleave", onDragLeave);
	window.addEventListener("drop", onDrop);
}

/** @emoji 📎 Registers a {@link ComposeKitStore} on the shell controller. */
export function attachSketchpadKitStore(
	kitId: string,
	store: ComposeKitStore,
	options?: { readonly kind?: SketchpadKitPersistenceKind; readonly navigate?: boolean; readonly assetBaseUrl?: string },
): void {
	const ctrl = getSketchpadShellController();
	if (!ctrl) throw new Error("compose/sketchpad: platform not initialized — call ensureSketchpadPlatform first");
	ctrl.registerKitStore(kitId, store, { kind: options?.kind, assetBaseUrl: options?.assetBaseUrl });
	if (options?.navigate !== false) {
		navigateSketchpadTo(`/kits/${kitId}`);
	}
}

/** @emoji 📎 Attaches a kit backend to the shell controller and optionally navigates to it. */
export function attachSketchpadKit(
	kitId: string,
	backend: ComposeKitStoreBackend,
	options?: { readonly kind?: SketchpadKitPersistenceKind; readonly navigate?: boolean; readonly assetBaseUrl?: string },
): void {
	attachSketchpadKitStore(kitId, new ComposeKitStore(backend), options);
}

/** @emoji 🔗 Syncs platform chrome then optional browser history navigation. */
function sketchpadCommitUri(platform: Platform, uri: string): void {
	applySketchpadUri(platform, uri);
	if (platform.onNavigate) platform.onNavigate(uri);
}

/** @emoji 🧭 Navigates the sketchpad {@link Platform} (updates history when in a browser). */
export function navigateSketchpadTo(uri: string): void {
	const platform = getSketchpadPlatform();
	if (!platform) throw new Error("compose/sketchpad: platform not initialized — call ensureSketchpadPlatform first");
	sketchpadCommitUri(platform, uri);
}

/** @emoji 📦 Imports kit bytes/URL and registers them on the active platform. */
export async function openSketchpadKitFromImport(
	data: ArrayBuffer | Blob | File | string,
	options?: { readonly kind?: SketchpadKitPersistenceKind; readonly navigate?: boolean; readonly assetBaseUrl?: string },
): Promise<string> {
	const { kit, session, portCompatSource } = await importKit(data);
	const jsStores = await session.stores();
	const jsStore = jsStores[0];
	const store = jsStore
		? await createComposeKitStoreFromJsStore(jsStore, {
				onDispose: () => void session.dispose(),
				portCompatSource,
			})
		: new InMemoryComposeKitStore(kit);
	const assetBaseUrl =
		options?.assetBaseUrl ?? (typeof data === "string" ? sketchpadKitAssetBaseUrlFromImportUrl(data) : undefined);
	attachSketchpadKitStore(kit.id, store, { kind: options?.kind ?? "fixture", navigate: options?.navigate, assetBaseUrl });
	return kit.id;
}

/** @emoji 📂 Derives the kit asset root directory from a kit import URL. */
export function sketchpadKitAssetBaseUrlFromImportUrl(importUrl: string): string {
	const pathOnly = importUrl.split(/[?#]/, 1)[0] ?? importUrl;
	const slash = pathOnly.lastIndexOf("/");
	return slash > 0 ? pathOnly.slice(0, slash) : "";
}

/** @emoji 🔗 Parses `COMPOSE_SKETCHPAD_PRELOAD_KITS` (comma/whitespace-separated kit URLs). */
export function sketchpadPreloadKitUrls(raw?: string): string[] {
	const source =
		raw ??
		(typeof import.meta !== "undefined"
			? ((import.meta as { env?: { COMPOSE_SKETCHPAD_PRELOAD_KITS?: string } }).env?.COMPOSE_SKETCHPAD_PRELOAD_KITS ?? "")
			: "");
	return source
		.split(/[\s,]+/)
		.map((entry) => entry.trim())
		.filter(Boolean);
}

/** @emoji 📥 Preloads kits from env or an explicit URL list (skips navigation). */
export async function preloadSketchpadKits(urls?: readonly string[]): Promise<void> {
	const ctrl = getSketchpadShellController();
	if (!ctrl) return;
	for (const url of urls ?? sketchpadPreloadKitUrls()) {
		try {
			await openSketchpadKitFromImport(url, { kind: "fixture", navigate: false });
		} catch (error) {
			console.warn("[compose.sketchpad] preload kit failed:", url, error);
		}
	}
}
//#endregion 🔖KitHost

//#region 🔖KitStore
export const SKETCHPAD_SHELL_STORE_SHELL = "shell";
export const SKETCHPAD_KIT_STORE_PREFIX = "kit:";

/** @emoji 📸 Kit row snapshot for {@link ComposeKitStore}. */
export type SketchpadKitSnapshot = { readonly kit: Kit };

/** @emoji 🎯 Selection within the active kit/design route (diagrams). */
export interface SketchpadRouteSelection {
	readonly pieceIds: readonly string[];
	readonly connectionIds: readonly string[];
	readonly kitWiresNodeIds: readonly string[];
	readonly kitWiresHoveredNodeId: string | null;
}

/** @emoji 📥 Home kit import progress surfaced in workbench chrome. */
export interface SketchpadImportStatus {
	readonly phase: "idle" | "importing" | "success" | "error";
	readonly label?: string;
	readonly error?: string;
}

/** @emoji 💬 In-progress feedback form draft stored on the shell snapshot. */
export interface SketchpadFeedbackDraft {
	readonly message: string;
	readonly contact: string;
}

/** @emoji 🏠 Home table UI state (expand, selection, URL-synced filters). */
export interface SketchpadHomeUiState {
	readonly expandedRowIds: readonly string[];
	readonly selectedKitIds: readonly string[];
	readonly kindFilter: string | null;
	readonly searchQuery: string;
	readonly nameFilter: string | null;
	readonly versionFilter: string | null;
	readonly sortColumnId: string | null;
	readonly sortDescending: boolean;
}

/** @emoji 🧭 Shell chrome snapshot (navigation, panels, open kits). */
export interface SketchpadShellSnapshot {
	readonly navigationPath: string;
	readonly panelVisibility: { readonly leftSidePanel: boolean; readonly rightSidePanel: boolean };
	readonly openKitIds: readonly string[];
	readonly routeSelection: SketchpadRouteSelection;
	readonly home: SketchpadHomeUiState;
	readonly importStatus: SketchpadImportStatus;
	readonly feedback: SketchpadFeedbackDraft;
}

function sketchpadEmptyRouteSelection(): SketchpadRouteSelection {
	return { pieceIds: [], connectionIds: [], kitWiresNodeIds: [], kitWiresHoveredNodeId: null };
}

function sketchpadEmptyHomeUiState(): SketchpadHomeUiState {
	return {
		expandedRowIds: [],
		selectedKitIds: [],
		kindFilter: null,
		searchQuery: "",
		nameFilter: null,
		versionFilter: null,
		sortColumnId: null,
		sortDescending: false,
	};
}

function sketchpadEmptyImportStatus(): SketchpadImportStatus {
	return { phase: "idle" };
}

function sketchpadEmptyFeedbackDraft(): SketchpadFeedbackDraft {
	return { message: "", contact: "" };
}

/** @emoji ✉️ Builds a `mailto:` URI for the sketchpad feedback draft. */
export function sketchpadFeedbackMailtoUri(draft: SketchpadFeedbackDraft): string | null {
	const message = draft.message.trim();
	if (!message) return null;
	const contact = draft.contact.trim();
	const subject = encodeURIComponent("Compose Sketchpad feedback");
	const body = encodeURIComponent(contact.length > 0 ? `${message}\n\n— ${contact}` : message);
	return `mailto:feedback@compose-tech.de?subject=${subject}&body=${body}`;
}

function sketchpadPathSupportsRouteSelectionQuery(pathOnly: string): boolean {
	return pathOnly.startsWith("/kits/");
}

/** @emoji 🔎 Parses kit/design diagram selection query params from a platform URI. */
export function parseSketchpadRouteSelectionQuery(uri: string): SketchpadRouteSelection {
	const query = uri.includes("?") ? uri.slice(uri.indexOf("?") + 1) : "";
	const params = new URLSearchParams(query);
	return {
		pieceIds: params.getAll("piece"),
		connectionIds: params.getAll("conn"),
		kitWiresNodeIds: params.getAll("wires"),
		kitWiresHoveredNodeId: null,
	};
}

/** @emoji 🔗 Serializes {@link SketchpadRouteSelection} into kit-route query params. */
export function sketchpadRouteSelectionUriFilters(selection: SketchpadRouteSelection): string {
	const params = new URLSearchParams();
	for (const id of selection.pieceIds) params.append("piece", id);
	for (const id of selection.connectionIds) params.append("conn", id);
	for (const id of selection.kitWiresNodeIds) params.append("wires", id);
	const serialized = params.toString();
	return serialized.length > 0 ? `?${serialized}` : "";
}

/** @emoji 🔎 Parses home filter query params from a platform URI. */
export function parseSketchpadHomeQuery(uri: string): SketchpadHomeUiState {
	const query = uri.includes("?") ? uri.slice(uri.indexOf("?") + 1) : "";
	const params = new URLSearchParams(query);
	return {
		expandedRowIds: params.getAll("e"),
		selectedKitIds: params.getAll("sel"),
		kindFilter: params.get("kind"),
		searchQuery: params.get("q") ?? "",
		nameFilter: params.get("name"),
		versionFilter: params.get("version"),
		sortColumnId: params.get("sort"),
		sortDescending: params.get("dir") === "desc",
	};
}

function sketchpadHomeUriFilters(home: SketchpadHomeUiState): string {
	const params = new URLSearchParams();
	if (home.kindFilter) params.set("kind", home.kindFilter);
	if (home.searchQuery) params.set("q", home.searchQuery);
	if (home.nameFilter) params.set("name", home.nameFilter);
	if (home.versionFilter) params.set("version", home.versionFilter);
	for (const id of home.expandedRowIds) params.append("e", id);
	for (const id of home.selectedKitIds) params.append("sel", id);
	if (home.sortColumnId) params.set("sort", home.sortColumnId);
	if (home.sortDescending) params.set("dir", "desc");
	const serialized = params.toString();
	return serialized.length > 0 ? `?${serialized}` : "";
}

function sketchpadTitleFromDocPath(relativePath: string): string {
	const segment = relativePath.replace(/\/index$/, "").split("/").pop() ?? relativePath;
	return segment
		.split(/[-_]/)
		.filter((part) => part.length > 0)
		.map((part) => part.charAt(0).toUpperCase() + part.slice(1))
		.join(" ");
}

type SketchpadDocPage = { readonly path: string; readonly title: string };
type SketchpadDocSection = { readonly id: string; readonly label: string; readonly pages: readonly SketchpadDocPage[] };

/** @emoji 📄 Lazy-loaded MDX module shape from the sketchpad pages bundle. */
export type SketchpadMdxModule = {
	readonly default: unknown;
	readonly frontmatter?: Readonly<Record<string, unknown>>;
};

const SKETCHPAD_MDX_MODULE_LOADERS = import.meta.glob<SketchpadMdxModule>("./page/**/*.mdx");
const SKETCHPAD_MDX_MODULE_PATHS = Object.keys(SKETCHPAD_MDX_MODULE_LOADERS);

/** @emoji 🔍 Resolves a docs route path to a Vite MDX module key. */
export function sketchpadResolveMdxModuleKey(docsPath: string): string | null {
	const clean = docsPath.replace(/^\/+/, "").replace(/\.mdx$/, "");
	const matches = SKETCHPAD_MDX_MODULE_PATHS.filter((key) => {
		const keyPath = key.replace(/^\.\/page\//, "").replace(/\.mdx$/, "");
		return keyPath === clean || keyPath === `${clean}/index`;
	});
	return matches[0] ?? null;
}

/** @emoji 📥 Loads an MDX page module for a docs route (`getting-started/index`, …). */
export async function sketchpadLoadMdxModule(docsPath: string): Promise<SketchpadMdxModule | null> {
	const moduleKey = sketchpadResolveMdxModuleKey(docsPath);
	if (!moduleKey) return null;
	try {
		return await SKETCHPAD_MDX_MODULE_LOADERS[moduleKey]!();
	} catch {
		return null;
	}
}

/** @emoji 🏷️ Reads a display title from MDX frontmatter or route path. */
export function sketchpadMdxTitle(module: SketchpadMdxModule | null, docsPath: string): string {
	const frontmatter = module?.frontmatter;
	if (frontmatter && typeof frontmatter["title"] === "string" && frontmatter["title"].length > 0) {
		return frontmatter["title"];
	}
	return sketchpadTitleFromDocPath(docsPath);
}

/** @emoji 📚 Builds the sketchpad docs tree from bundled MDX pages (Vite glob). */
export function sketchpadBuildDocsRegistry(): readonly SketchpadDocSection[] {
	const sectionMap = new Map<string, SketchpadDocPage[]>();
	for (const modulePath of SKETCHPAD_MDX_MODULE_PATHS) {
		const relative = modulePath.replace(/^\.\/pages\//, "").replace(/\.mdx$/, "");
		const sectionId = relative.split("/")[0] ?? "root";
		const pages = sectionMap.get(sectionId) ?? [];
		pages.push({ path: relative, title: sketchpadTitleFromDocPath(relative) });
		sectionMap.set(sectionId, pages);
	}
	if (sectionMap.size === 0) {
		return [
			{
				id: "getting-started",
				label: "Getting started",
				pages: [
					{ path: "getting-started/index", title: "Getting started" },
					{ path: "getting-started/installation", title: "Installation" },
				],
			},
		];
	}
	return [...sectionMap.entries()]
		.map(([id, pages]) => ({
			id,
			label: sketchpadTitleFromDocPath(id),
			pages: pages.sort((left, right) => left.path.localeCompare(right.path)),
		}))
		.sort((left, right) => left.label.localeCompare(right.label));
}

/** @emoji 🔌 Backend contract for {@link ComposeKitStore} (memory, WASM worker, HTTP, …). */
export type ComposeKitStoreBackend = {
	getSnapshot(): SketchpadKitSnapshot;
	subscribe?(listener: () => void): () => void;
	replace?(next: Kit): void;
};

/** @emoji 🗄️ Kit authority store; adapts any {@link ComposeKitStoreBackend} to {@link Store}. */
export class ComposeKitStore extends Store<SketchpadKitSnapshot> {
	private detach?: () => void;

	constructor(private readonly backend: ComposeKitStoreBackend) {
		super();
		if (backend.subscribe) {
			this.detach = backend.subscribe(() => this.notify());
		}
	}

	override getSnapshot(): SketchpadKitSnapshot {
		return this.backend.getSnapshot();
	}

	replaceKit(next: Kit): void {
		this.backend.replace?.(next);
		this.notify();
	}

	override dispose(): void {
		this.detach?.();
		super.dispose();
	}
}

/** @emoji 💾 In-memory kit store for hosts without a live {@link @semio-tech/compose-js} session yet. */
export class InMemoryComposeKitStore extends ComposeKitStore {
	constructor(kit: Kit) {
		let current = kit;
		super({
			getSnapshot: () => ({ kit: current }),
			replace: (next) => {
				current = next;
			},
		});
	}
}

/** @emoji 🌐 {@link ComposeKitStore} backed by {@link @semio-tech/compose-js} with live kit mutations. */
export class ComposeJsKitStore extends ComposeKitStore {
	constructor(
		backend: ComposeKitStoreBackend,
		readonly jsStore: JsKitStore,
		private readonly onSessionDispose: (() => void | Promise<void>) | undefined,
		private readonly portCompatById: ReadonlyMap<string, readonly { readonly id: string }[]>,
	) {
		super(backend);
	}

	/** @emoji 🏛 WIP {@link JsKitEntity} handle for GraphQL kit commands. */
	async jsKitEntity(): Promise<JsKitEntity> {
		return this.jsStore.wip().theKit().kit();
	}

	/** @emoji 🔄 Re-reads kit DTO from rs and notifies subscribers. */
	async refreshFromJs(): Promise<void> {
		const kit = await sketchpadKitDtoFromJsStore(this.jsStore);
		const fromGraphql = sketchpadExtractPortCompatById(kit);
		const compat = sketchpadMergePortCompatMaps(this.portCompatById, fromGraphql);
		this.replaceKit(sketchpadApplyPortCompatById(kit, compat));
	}

	override dispose(): void {
		super.dispose();
		void this.onSessionDispose?.();
	}
}

const SKETCHPAD_KIT_READ_INNER = `id name description version createdAt updatedAt
hasDesigns {
  edges {
    node {
      id name description unit
      hasPieces {
        edges {
          node {
            id name
            blueprint { id }
            position { center { u v } plane { origin { x y z } xAxis { x y z } yAxis { x y z } } }
            flatPosition { center { u v } plane { origin { x y z } xAxis { x y z } yAxis { x y z } } }
          }
        }
      }
      hasConnections {
        edges {
          node {
            id
            gap shift rise rotation turn tilt u v
            parent { referencesPiece { id } referencesConnector { id } }
            child { referencesPiece { id } referencesConnector { id } }
          }
        }
      }
    }
  }
}
hasTypes {
  edges {
    node {
      id name description
      hasConnectors { edges { node { id name point { x y z } direction { x y z } t port { id label code copatibleWith { edges { node { id } } } } } } }
      hasPorts { edges { node { id label code copatibleWith { edges { node { id } } } } } }
      hasRepresentations { edges { node { id name file { id } } } }
    }
  }
}
qualities { edges { node { id key value } } }
hasFolders { edges { node { id path description files { edges { node { id name url description folderId } } } } } }
authors { edges { node { id name } } }
hasFiles { edges { node { id name url description folderId } } }`;

/** @emoji 📁 Maps GraphQL {@code folderId} onto kit file DTO {@code folder} refs. */
function sketchpadFileDtoFromGraphqlNode(node: Record<string, unknown>): Record<string, unknown> {
	const folderRef = node["folder"];
	if (folderRef != null && typeof folderRef === "object" && "id" in folderRef) return node;
	const folderId = node["folderId"];
	if (folderId == null || folderId === "") {
		const { folderId: _omit, ...rest } = node;
		return rest;
	}
	const { folderId: _omit, ...rest } = node;
	return { ...rest, folder: { id: String(folderId) } };
}

function sketchpadFormatKitTimestamp(value: unknown): string {
	if (value == null || value === "") return "";
	const date = typeof value === "string" || typeof value === "number" ? new Date(value) : value instanceof Date ? value : null;
	if (!date || Number.isNaN(date.getTime())) return "";
	return date.toLocaleString();
}

function sketchpadKitTimestampIso(value: unknown): string | undefined {
	if (value == null || value === "") return undefined;
	const date = typeof value === "string" || typeof value === "number" ? new Date(value) : value instanceof Date ? value : null;
	if (!date || Number.isNaN(date.getTime())) return undefined;
	return date.toISOString();
}

/** @emoji 🔌 Maps GraphQL {@code copatibleWith} relay edges onto {@code compatiblePorts} DTO refs. */
export function sketchpadPortDtoFromGraphqlNode(node: Record<string, unknown>): Record<string, unknown> {
	const compatEdges =
		(node["copatibleWith"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
	const compatiblePorts = compatEdges
		.map((edge) => edge.node)
		.filter((port): port is Record<string, unknown> => port != null)
		.map((port) => ({ id: port["id"] }));
	if (compatiblePorts.length === 0) return node;
	return { ...node, compatiblePorts };
}

/** @emoji 📸 Materializes a kit DTO from rs GraphQL for platform snapshots. */
export async function sketchpadKitDtoFromJsStore(jsStore: JsKitStore): Promise<Kit> {
	const data = await jsStore.readKitInner(SKETCHPAD_KIT_READ_INNER);
	if (!data) return { id: "", name: "" } as Kit;
	const nodes = (key: string): readonly Record<string, unknown>[] => {
		const edges = (data[key] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
		return edges.map((edge) => edge.node).filter((node): node is Record<string, unknown> => node != null);
	};
	const parseDesigns = (): Design[] => {
		const edges = (data["hasDesigns"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
		return edges
			.map((edge) => edge.node)
			.filter((node): node is Record<string, unknown> => node != null)
			.map((node) => {
				const pieceEdges = (node["hasPieces"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const pieces = pieceEdges.map((pe) => pe.node).filter((n): n is Record<string, unknown> => n != null) as Design["pieces"];
				const connectionEdges = (node["hasConnections"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const connections = connectionEdges.map((ce) => {
					const raw = ce.node;
					if (raw == null || typeof raw !== "object") return raw;
					const remapSide = (side: unknown): unknown => {
						if (side == null || typeof side !== "object") return side;
						const s = side as Record<string, unknown>;
						const piece = s["referencesPiece"] ?? s["piece"];
						const connector = s["referencesConnector"] ?? s["connector"];
						return { ...s, piece, connector };
					};
					return { ...raw, parent: remapSide(raw["parent"]), child: remapSide(raw["child"]) };
				}).filter((n): n is Record<string, unknown> => n != null);
				return { ...node, pieces, connections } as Design;
			});
	};
	const parseTypes = (): Type[] => {
		const edges = (data["hasTypes"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
		return edges
			.map((edge) => edge.node)
			.filter((node): node is Record<string, unknown> => node != null)
			.map((node) => {
				const repEdges = (node["hasRepresentations"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const representations = repEdges.map((re) => re.node).filter((n): n is Record<string, unknown> => n != null);
				const portEdges = (node["hasPorts"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const ports = portEdges
					.map((pe) => pe.node)
					.filter((n): n is Record<string, unknown> => n != null)
					.map((port) => sketchpadPortDtoFromGraphqlNode(port));
				const conEdges = (node["hasConnectors"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const connectors = conEdges
					.map((ce) => ce.node)
					.filter((n): n is Record<string, unknown> => n != null)
					.map((connector) => {
						const port = connector["port"];
						if (port == null || typeof port !== "object") return connector;
						return { ...connector, port: sketchpadPortDtoFromGraphqlNode(port as Record<string, unknown>) };
					});
				return { ...node, representations, ports, connectors } as Type;
			});
	};
	const filesById = new Map<string, Record<string, unknown>>();
	for (const file of nodes("hasFiles")) {
		if (sketchpadKitFileHiddenRepresentationBlob(file, new Set())) continue;
		const normalized = sketchpadFileDtoFromGraphqlNode(file);
		const id = String(normalized["id"] ?? "");
		if (id) filesById.set(id, normalized);
	}
	for (const folder of nodes("hasFolders")) {
		const nested = (folder["files"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
		for (const edge of nested) {
			const file = edge.node;
			if (!file) continue;
			const normalized = sketchpadFileDtoFromGraphqlNode(file);
			const id = String(normalized["id"] ?? "");
			if (id) filesById.set(id, normalized);
		}
	}
	return {
		id: String(data["id"] ?? ""),
		name: String(data["name"] ?? ""),
		description: data["description"] != null ? String(data["description"]) : undefined,
		version: data["version"] != null ? String(data["version"]) : undefined,
		createdAt: data["createdAt"] != null ? String(data["createdAt"]) : undefined,
		updatedAt: data["updatedAt"] != null ? String(data["updatedAt"]) : undefined,
		files: [...filesById.values()] as Kit["files"],
		folders: nodes("hasFolders") as Kit["folders"],
		authors: nodes("authors") as Kit["authors"],
		qualities: nodes("qualities") as Kit["qualities"],
		designs: parseDesigns(),
		types: parseTypes(),
	} as Kit;
}

/** @emoji 🌐 Builds a {@link ComposeJsKitStore} from a live {@link @semio-tech/compose-js} store. */
export async function createComposeKitStoreFromJsStore(
	jsStore: JsKitStore,
	options?: { readonly onDispose?: () => void | Promise<void>; readonly portCompatSource?: Kit },
): Promise<ComposeJsKitStore> {
	const portCompatById = sketchpadExtractPortCompatById(
		options?.portCompatSource ?? ({ id: "", name: "" } as Kit),
	);
	const materializeKit = async (): Promise<Kit> => {
		const dto = await sketchpadKitDtoFromJsStore(jsStore);
		const merged = options?.portCompatSource ? sketchpadMergeKitDtoFromBundleProjection(dto, options.portCompatSource) : dto;
		const compat = sketchpadMergePortCompatMaps(portCompatById, sketchpadExtractPortCompatById(merged));
		return sketchpadApplyPortCompatById(merged, compat);
	};
	let kit = await materializeKit();
	const refresh = async (): Promise<void> => {
		kit = await materializeKit();
	};
	await refresh();
	return new ComposeJsKitStore(
		{
			getSnapshot: () => ({ kit }),
			replace: (next) => {
				kit = next;
			},
			subscribe: (listener) =>
				jsStore.session.subscribe(() => {
					void refresh().then(listener);
				}),
		},
		jsStore,
		options?.onDispose,
		portCompatById,
	);
}

/** @emoji ⚡ Runs a {@link JsKitEntity} mutation on the active js-backed kit store. */
export async function executeSketchpadJsKitMutation(
	kitId: string,
	run: (kit: JsKitEntity) => Promise<SetResult>,
	storeOverride?: ComposeKitStore,
): Promise<SetResult> {
	const store = storeOverride ?? getSketchpadShellController()?.getKitStore(kitId);
	if (!(store instanceof ComposeJsKitStore)) {
		return { ok: false, error: { kind: "NotSupported", message: "compose/sketchpad: kit is not backed by @semio-tech/compose-js" } };
	}
	const result = await run(await store.jsKitEntity());
	await store.refreshFromJs();
	return result;
}

function sketchpadActiveKitIdFromPath(path: string): string | null {
	return path.split("?")[0]?.match(/^\/kits\/([^/]+)/)?.[1] ?? null;
}

export function sketchpadKitStoreId(kitId: string): string {
	return `${SKETCHPAD_KIT_STORE_PREFIX}${kitId}`;
}

let sketchpadShellControllerSingleton: SketchpadShellController | null = null;

/** @emoji 🎛 Active sketchpad shell controller after {@link buildSketchpadPlatform}. */
export function getSketchpadShellController(): SketchpadShellController | null {
	return sketchpadShellControllerSingleton;
}
//#endregion 🔖KitStore

//#region 🔖SketchpadRouteScope
/** @emoji 🧭 Kit/design/type/docs scope parsed from a sketchpad URL (path + kit query params). */
export function parseSketchpadRouteScopeFromPath(pathOrUri: string): {
	readonly kitId: string | null;
	readonly designId: string | null;
	readonly typeId: string | null;
	readonly docsPath: string;
	readonly qualityId: string | null;
} {
	const queryIndex = pathOrUri.indexOf("?");
	const pathOnly = queryIndex >= 0 ? pathOrUri.slice(0, queryIndex) : pathOrUri;
	const query = queryIndex >= 0 ? pathOrUri.slice(queryIndex + 1) : "";
	const pathParts = pathOnly.split("/").filter((part) => part.length > 0);
	const isUuidPattern = (value: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
	if (pathParts[0] === "docs") {
		const docsPath = pathParts.slice(1).join("/") || "index";
		return { kitId: null, designId: null, typeId: null, docsPath, qualityId: null };
	}
	if (pathParts[0] !== "kits") {
		return { kitId: null, designId: null, typeId: null, docsPath: "index", qualityId: null };
	}
	const kitId = pathParts[1] && isUuidPattern(pathParts[1]) ? pathParts[1] : null;
	const designId = pathParts[2] === "design" && pathParts[3] && isUuidPattern(pathParts[3]) ? pathParts[3] : null;
	const typeId = pathParts[2] === "type" && pathParts[3] && isUuidPattern(pathParts[3]) ? pathParts[3] : null;
	const qualityParam = kitId && query.length > 0 ? new URLSearchParams(query).get("quality") : null;
	const qualityId = qualityParam && qualityParam.length > 0 ? qualityParam : null;
	return { kitId, designId, typeId, docsPath: "index", qualityId };
}

/** @emoji 🧭 Maps a location path to the sketchpad {@link Platform} active app id. */
export function sketchpadAppIdFromPath(path: string): string {
	const pathParts = path.split("/").filter((part) => part.length > 0);
	const isUuidPattern = (value: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
	if (pathParts[0] === "docs") return SKETCHPAD_DOCS_APP_ID;
	if (pathParts[0] === "feedback") return SKETCHPAD_FEEDBACK_APP_ID;
	if (pathParts[0] !== "kits") return SKETCHPAD_HOME_APP_ID;
	if (pathParts.length >= 4 && pathParts[2] === "design" && isUuidPattern(pathParts[3] ?? "")) return SKETCHPAD_DESIGN_APP_ID;
	if (pathParts.length >= 4 && pathParts[2] === "type" && isUuidPattern(pathParts[3] ?? "")) return SKETCHPAD_TYPE_APP_ID;
	if (pathParts.length >= 2 && isUuidPattern(pathParts[1] ?? "")) return SKETCHPAD_KIT_APP_ID;
	return SKETCHPAD_HOME_APP_ID;
}
//#endregion 🔖SketchpadRouteScope

//#region 🔖KitHelpers
/** @emoji 📋 Normalizes list-or-`{ items }` blocks on kit DTO snapshots. */
function sketchpadKitItemsOf<T>(node: unknown): readonly T[] {
	if (Array.isArray(node)) return node as readonly T[];
	if (node && typeof node === "object" && Array.isArray((node as { items?: unknown[] }).items)) {
		return (node as { items: T[] }).items;
	}
	return [];
}

/** @emoji 🔍 Flattens kit kinds from root `types` or nested `typologies[].types`. */
function sketchpadKitTypeRows(kit: Kit): readonly Type[] {
	const root = sketchpadKitItemsOf<Type>(kit.types);
	if (root.length > 0) return root;
	return sketchpadKitTypologyRows(kit).flatMap((topo) => sketchpadKitItemsOf<Type>(topo.types));
}

/** @emoji 🔍 Flattens kit designs from root `designs` or nested `typologies[].designs`. */
function sketchpadKitDesignRows(kit: Kit): readonly Design[] {
	const root = sketchpadKitItemsOf<Design>(kit.designs);
	if (root.length > 0) return root;
	return sketchpadKitTypologyRows(kit).flatMap((topo) => sketchpadKitItemsOf<Design>(topo.designs));
}

/** @emoji 🔍 Finds a type row on a kit snapshot. */
export function findTypeInKit(kit: Kit, typeId: string | null | undefined): Type | undefined {
	if (!typeId) return undefined;
	return sketchpadKitTypeRows(kit).find((t) => t.id === typeId);
}

/** @emoji 🔍 Finds a quality row on a kit snapshot. */
export function findQualityInKit(kit: Kit, qualityId: string | null | undefined): { readonly id: string; readonly key?: string; readonly value?: string } | undefined {
	if (!qualityId) return undefined;
	return (kit.qualities ?? []).find((entry) => {
		if (typeof entry !== "object" || entry === null || !("id" in entry)) return false;
		return (entry as { id: string }).id === qualityId;
	}) as { readonly id: string; readonly key?: string; readonly value?: string } | undefined;
}

/** @emoji 🔍 Finds a design row on a kit snapshot. */
export function findDesignInKit(kit: Kit, designId: string | null | undefined): Design | undefined {
	if (!designId) return undefined;
	return sketchpadKitDesignRows(kit).find((d) => d.id === designId);
}

/** @emoji 🧭 Builds a navigation destination for sketchpad breadcrumb trails. */
function sketchpadNavigationDestination(id: string, label: string, uri: string): NavigationDestination {
	return { id, label, uri };
}

/** @emoji 🧭 Builds one navigation level (node + separator alternatives). */
function sketchpadNavigationLevel(node: NavigationDestination, alternatives: readonly NavigationDestination[]): NavigationLevel {
	return { node, alternatives };
}

/** @emoji 🧭 Top-level destinations reachable from Home. */
function sketchpadHomeNavigationAlternatives(): readonly NavigationDestination[] {
	return [
		sketchpadNavigationDestination("sketchpad.nav.kits", "Kits", "/"),
		sketchpadNavigationDestination("sketchpad.nav.documentation", "Documentation", "/docs"),
		sketchpadNavigationDestination("sketchpad.nav.feedback", "Feedback", "/feedback"),
	];
}

/** @emoji 🔍 Reads an entity id from a kit row snapshot. */
function sketchpadKitRowEntityId(entity: unknown): string | null {
	if (typeof entity !== "object" || entity === null || !("id" in entity)) return null;
	return String((entity as { id: unknown }).id);
}

/** @emoji 🔍 Reads an entity display name from a kit row snapshot. */
function sketchpadKitRowEntityName(entity: unknown, fallback: string): string {
	if (typeof entity !== "object" || entity === null) return fallback;
	const name = (entity as { name?: string }).name;
	return name && name.length > 0 ? name : fallback;
}

/** @emoji 🔍 Finds the typology that owns a design on a kit snapshot. */
function findSketchpadTypologyForDesign(
	kit: Kit,
	designId: string,
): ReturnType<typeof sketchpadKitTypologyRows>[number] | undefined {
	for (const typology of sketchpadKitTypologyRows(kit)) {
		if (typology.designs.some((design) => sketchpadKitRowEntityId(design) === designId)) return typology;
	}
	return undefined;
}

/** @emoji 🔍 Finds the typology that owns a type on a kit snapshot. */
function findSketchpadTypologyForType(kit: Kit, typeId: string): ReturnType<typeof sketchpadKitTypologyRows>[number] | undefined {
	for (const typology of sketchpadKitTypologyRows(kit)) {
		if (typology.types.some((type) => sketchpadKitRowEntityId(type) === typeId)) return typology;
	}
	return undefined;
}

/** @emoji 🧭 Lists design destinations within a typology for breadcrumb alternatives. */
function sketchpadTypologyDesignDestinations(kitId: string, typology: ReturnType<typeof sketchpadKitTypologyRows>[number]): NavigationDestination[] {
	const out: NavigationDestination[] = [];
	for (const design of typology.designs) {
		const designId = sketchpadKitRowEntityId(design);
		if (!designId) continue;
		out.push(
			sketchpadNavigationDestination(
				`sketchpad.nav.design.${designId}`,
				sketchpadKitRowEntityName(design, designId),
				`/kits/${kitId}/design/${designId}`,
			),
		);
	}
	return out;
}

/** @emoji 🧭 Lists type destinations within a typology for breadcrumb alternatives. */
function sketchpadTypologyTypeDestinations(kitId: string, typology: ReturnType<typeof sketchpadKitTypologyRows>[number]): NavigationDestination[] {
	const out: NavigationDestination[] = [];
	for (const type of typology.types) {
		const typeId = sketchpadKitRowEntityId(type);
		if (!typeId) continue;
		out.push(
			sketchpadNavigationDestination(`sketchpad.nav.type.${typeId}`, sketchpadKitRowEntityName(type, typeId), `/kits/${kitId}/type/${typeId}`),
		);
	}
	return out;
}

/** @emoji 🔍 Finds a piece row on a design snapshot. */
export function findPieceInDesign(design: Design, pieceId: string | null | undefined) {
	if (!pieceId) return undefined;
	return design.pieces?.find((p) => p.id === pieceId);
}

function sketchpadReadEntityId(ref: unknown): string | null {
	if (ref == null) return null;
	if (typeof ref === "string") return ref;
	if (typeof ref === "object" && "id" in ref) return String((ref as { id: unknown }).id);
	return null;
}

/** @emoji 📁 Resolves the owning folder id for a kit file row (`folder`, `folderId`, …). */
function sketchpadKitFileFolderId(file: Record<string, unknown>): string | null {
	return (
		sketchpadReadEntityId(file["folder"]) ??
		sketchpadReadEntityId(file["folderId"]) ??
		sketchpadReadEntityId(file["folder_id"])
	);
}

/** @emoji 📄 File ids used as representation backing blobs on kit kinds. */
function sketchpadKitRepresentationFileIds(kit: Kit): ReadonlySet<string> {
	const ids = new Set<string>();
	for (const type of sketchpadKitTypeRows(kit)) {
		for (const rep of sketchpadKitItemsOf<unknown>((type as { representations?: unknown }).representations)) {
			const fileId = sketchpadReadEntityId((rep as Record<string, unknown>)["file"]);
			if (fileId) ids.add(fileId);
		}
	}
	return ids;
}

/** @emoji 📄 Speckle-style representation blob basename (`e5267da44d`). */
function sketchpadKitFileLooksLikeRepresentationBlobName(name: string): boolean {
	return name.length === 10 && /^[0-9a-f]+$/i.test(name);
}

/** @emoji 📄 True when a loose kit file is a representation backing blob, not a browsable VFS asset. */
function sketchpadKitFileHiddenRepresentationBlob(
	file: Record<string, unknown>,
	representationFileIds: ReadonlySet<string>,
): boolean {
	const id = String(file["id"] ?? "");
	if (id && representationFileIds.has(id)) return true;
	if (sketchpadKitFileFolderId(file) != null) return false;
	const name = String(file["name"] ?? "");
	if (sketchpadKitFileLooksLikeRepresentationBlobName(name)) return true;
	const url = String(file["url"] ?? file["uri"] ?? file["remote"] ?? "");
	if (url.includes("/representation/")) return true;
	return false;
}

/** @emoji 📄 True when a kit file row belongs at kit VFS root (not under a folder or representation). */
function sketchpadKitFileAtKitRoot(file: Record<string, unknown>, representationFileIds: ReadonlySet<string> = new Set()): boolean {
	if (sketchpadKitFileHiddenRepresentationBlob(file, representationFileIds)) return false;
	return sketchpadKitFileFolderId(file) == null;
}

/** @emoji 📁 Resolves a folder row's parent folder id (`parent`, `parentFolderId`, …). */
function sketchpadKitFolderParentId(folder: Record<string, unknown>): string | null {
	return (
		sketchpadReadEntityId(folder["parent"]) ??
		sketchpadReadEntityId(folder["parentFolder"]) ??
		sketchpadReadEntityId(folder["parentFolderId"]) ??
		sketchpadReadEntityId(folder["parent_folder_id"])
	);
}

/** @emoji 📁 True when a folder row belongs at kit VFS root (no parent folder). */
function sketchpadKitFolderAtRoot(folder: Record<string, unknown>): boolean {
	return sketchpadKitFolderParentId(folder) == null;
}

/** @emoji 📁 VFS label and path for a kit folder row (prefers `name`, then `path`, then id). */
function sketchpadKitFolderVfsFields(folder: Record<string, unknown>): { readonly name: string; readonly path: string } {
	const explicitName = typeof folder["name"] === "string" ? folder["name"].trim() : "";
	const path =
		typeof folder["path"] === "string" && folder["path"].trim().length > 0
			? folder["path"].trim()
			: explicitName.length > 0
				? `/${explicitName}`
				: `/${String(folder["id"] ?? "")}`;
	const slash = path.lastIndexOf("/");
	const name = explicitName.length > 0 ? explicitName : slash >= 0 ? path.slice(slash + 1) : path;
	return { name, path };
}

function sketchpadKitVfsPushFolderRow(
	rows: VirtualFileSystemNodeRecord[],
	kitId: string,
	parentId: string,
	folder: Record<string, unknown>,
): void {
	const id = String(folder["id"] ?? "");
	if (!id) return;
	const { name, path } = sketchpadKitFolderVfsFields(folder);
	rows.push({
		id,
		fileNodeKindId: "folder",
		name,
		path,
		parentId,
		hasChildren: true,
		navigateUri: `/kits/${kitId}?folder=${encodeURIComponent(id)}`,
		descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("folder", { path }),
	});
}

function sketchpadKitVfsPushFileRow(
	rows: VirtualFileSystemNodeRecord[],
	kitId: string,
	parentId: string,
	file: Record<string, unknown>,
): void {
	const id = String(file["id"] ?? "");
	if (!id) return;
	const vfsFile = sketchpadKitVfsFileRowFields(file);
	const filePath = `/${sketchpadKitFileBasename(file)}`;
	rows.push({
		id,
		fileNodeKindId: "file",
		name: vfsFile.name,
		icon: vfsFile.icon,
		path: filePath,
		parentId,
		hasChildren: false,
		navigateUri: `/kits/${kitId}?file=${encodeURIComponent(id)}`,
		descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("file", { path: filePath }),
	});
}

/** @emoji 📍 Normalizes a path relative to a kit asset root (supports `../representation/*.glb`). */
export function sketchpadFixtureUrlFromKitRelativePath(relativePath: string, baseRoot: string): string {
	if (relativePath.startsWith("/")) return relativePath;
	const segments = baseRoot.split("/").filter(Boolean);
	for (const part of relativePath.replace(/^\.\//, "").split("/")) {
		if (part === "..") segments.pop();
		else if (part !== ".") segments.push(part);
	}
	return `/${segments.join("/")}`;
}

/** @emoji 🧊 Maps representation GLBs to puzzle 3d `/mesh/*` URLs (see {@link puzzle3dMeshesVitePlugin}). */
export function sketchpadPuzzle3dMeshUrlForKitFile(row: { readonly name?: string; readonly path?: string }): string | undefined {
	const path = row.path?.replace(/^\.\//, "") ?? "";
	if (path.includes("representations/") || path.includes("representation/")) {
		if (path.endsWith(".glb")) {
			const base = path.split("/").pop();
			return base ? `/mesh/${base}` : undefined;
		}
	}
	const name = row.name?.trim();
	if (!path && name?.endsWith(".glb")) {
		return `/mesh/${name}`;
	}
	return undefined;
}

/** @emoji 🗂️ Resolves kit file ids to fetchable mesh URLs (http, absolute, or kit-relative assets). */
export function sketchpadKitFileUrlById(kit: Kit): ReadonlyMap<string, string> {
	const map = new Map<string, string>();
	const assetBaseUrl = getSketchpadShellController()?.getKitAssetBaseUrl(kit.id);
	for (const file of kit.files ?? []) {
		const row = file as { id: string; url?: string; uri?: string; path?: string; name?: string; blob?: string };
		const puzzleMesh = sketchpadPuzzle3dMeshUrlForKitFile(row);
		if (puzzleMesh) {
			map.set(row.id, puzzleMesh);
			continue;
		}
		if (row.blob && typeof row.blob === "string" && /^(?:blob:|data:|https?:)/i.test(row.blob)) {
			map.set(row.id, row.blob);
			continue;
		}
		const direct = row.url ?? row.uri;
		if (direct) {
			map.set(row.id, direct);
			continue;
		}
		if (row.path) {
			map.set(row.id, assetBaseUrl ? sketchpadFixtureUrlFromKitRelativePath(row.path, assetBaseUrl) : row.path);
		}
	}
	return map;
}

const SKETCHPAD_PLACEHOLDER_MESH_URL = "puzzle.3d.placeholder://box";

/** @emoji 🧊 Picks a representation mesh URL for a design piece (placeholder when unresolved). */
export function sketchpadResolvePieceMeshUrl(
	piece: { readonly type?: unknown; readonly blueprint?: unknown },
	kit: Kit,
	fileUrls: ReadonlyMap<string, string> = sketchpadKitFileUrlById(kit),
): string {
	const typeId = sketchpadReadEntityId(piece.type ?? piece.blueprint);
	if (!typeId) return SKETCHPAD_PLACEHOLDER_MESH_URL;
	const type = findTypeInKit(kit, typeId);
	const reps = type ? sketchpadListTypeRepresentations(type) : [];
	if (reps.length === 0) return SKETCHPAD_PLACEHOLDER_MESH_URL;
	const untagged =
		reps.find((rep) => {
			const tags = rep.tags as { items?: readonly unknown[] } | readonly unknown[] | undefined;
			if (Array.isArray(tags)) return tags.length === 0;
			return !tags?.items?.length;
		}) ?? reps[0];
	return sketchpadResolveRepresentationMeshUrl(untagged!, kit, fileUrls);
}

/** @emoji 🏷️ Normalized type representation row for routing and topology. */
export interface SketchpadTypeRepresentationRef {
	readonly id: string;
	readonly name: string;
	readonly file?: unknown;
	readonly tags?: unknown;
}

function sketchpadMergeKitTypeRepresentations(target: Type, source: Type | undefined): Type {
	if (!source) return target;
	const liveReps = sketchpadListTypeRepresentations(target);
	const bundleReps = sketchpadListTypeRepresentations(source);
	if (liveReps.length > 0 || bundleReps.length === 0) return target;
	return { ...target, representations: source.representations } as Type;
}

function sketchpadMergeKitDesignRows(target: Design, source: Design | undefined): Design {
	if (!source) return target;
	const targetPieces = sketchpadKitItemsOf(target.pieces);
	const sourcePieces = sketchpadKitItemsOf(source.pieces);
	const targetConnections = sketchpadKitItemsOf((target as { connections?: unknown }).connections);
	const sourceConnections = sketchpadKitItemsOf((source as { connections?: unknown }).connections);
	return {
		...target,
		pieces: (sourcePieces.length > targetPieces.length ? sourcePieces : targetPieces) as Design["pieces"],
		connections: (sourceConnections.length > targetConnections.length ? sourceConnections : targetConnections) as Design["connections"],
	} as Design;
}

/** @emoji 🔀 Overlays bundle projection types/files when GraphQL materialization omits representations. */
export function sketchpadMergeKitDtoFromBundleProjection(target: Kit, source: Kit): Kit {
	const sourceFiles = source.files ?? [];
	const targetFiles = target.files ?? [];
	const files =
		sourceFiles.length === 0
			? targetFiles
			: [
					...targetFiles,
					...sourceFiles.filter((file) => !targetFiles.some((row) => row.id === file.id)),
				];
	const sourceTypes = sketchpadKitTypeRows(source);
	const types = sketchpadKitTypeRows(target).map((type) =>
		sketchpadMergeKitTypeRepresentations(type, sourceTypes.find((row) => row.id === type.id)),
	);
	const targetTypologies = sketchpadKitTypologyRows(target);
	const sourceTypologies = sketchpadKitTypologyRows(source);
	const typologies =
		sourceTypologies.length === 0
			? (target as { typologies?: unknown }).typologies
			: targetTypologies.length === 0
				? (source as { typologies?: unknown }).typologies
				: (target as { typologies?: unknown }).typologies;
	const targetDesignRows = sketchpadKitDesignRows(target);
	const sourceDesignRows = sketchpadKitDesignRows(source);
	const designs =
		targetDesignRows.length === 0
			? sourceDesignRows
			: targetDesignRows.map((design) => sketchpadMergeKitDesignRows(design, sourceDesignRows.find((row) => row.id === design.id)));
	return { ...target, files, types, typologies, designs: designs.length > 0 ? designs : target.designs } as Kit;
}

/** @emoji 📋 Lists representation rows on a kit kind. */
export function sketchpadListTypeRepresentations(type: Type): readonly SketchpadTypeRepresentationRef[] {
	return sketchpadKitItemsOf<Record<string, unknown>>(type.representations)
		.filter((entry): entry is Record<string, unknown> => typeof entry === "object" && entry !== null && "id" in entry)
		.map((entry) => ({
			id: String(entry["id"]),
			name: typeof entry["name"] === "string" && entry["name"].length > 0 ? entry["name"] : String(entry["id"]),
			file: entry["file"],
			tags: entry["tags"],
		}));
}

/** @emoji 🧊 Resolves the mesh URL for one type representation. */
export function sketchpadResolveRepresentationMeshUrl(
	representation: Pick<SketchpadTypeRepresentationRef, "file">,
	kit: Kit,
	fileUrls: ReadonlyMap<string, string> = sketchpadKitFileUrlById(kit),
): string {
	const fileId = sketchpadReadEntityId(representation.file);
	if (!fileId) return SKETCHPAD_PLACEHOLDER_MESH_URL;
	return fileUrls.get(fileId) ?? SKETCHPAD_PLACEHOLDER_MESH_URL;
}

/** @emoji 🧊 Picks the primary representation mesh URL for a kit kind. */
export function sketchpadResolveTypeMeshUrl(
	type: Type,
	kit: Kit,
	fileUrls: ReadonlyMap<string, string> = sketchpadKitFileUrlById(kit),
): string {
	const reps = sketchpadListTypeRepresentations(type);
	if (reps.length === 0) return SKETCHPAD_PLACEHOLDER_MESH_URL;
	const untagged =
		reps.find((rep) => {
			const tags = rep.tags as { items?: readonly unknown[] } | readonly unknown[] | undefined;
			if (Array.isArray(tags)) return tags.length === 0;
			return !tags?.items?.length;
		}) ?? reps[0];
	return sketchpadResolveRepresentationMeshUrl(untagged!, kit, fileUrls);
}

function sketchpadNewKitId(): string {
	if (typeof crypto !== "undefined" && typeof crypto.randomUUID === "function") {
		return crypto.randomUUID();
	}
	return `kit-${Date.now()}`;
}

function sketchpadPanelTextStack(lines: readonly { readonly text: string; readonly emphasize?: boolean }[]): PanelModel {
	return {
		body: {
			type: "stack",
			direction: "vertical",
			padding: "standard",
			children: lines.map((line) => ({ type: "text", value: line.text, emphasize: line.emphasize })),
		},
	};
}

function sketchpadPanelCommandButton(
	label: string,
	command: string,
	args?: unknown,
): { readonly type: "button"; readonly label: string; readonly command: { readonly controllerId: string; readonly command: string; readonly args?: unknown } } {
	return {
		type: "button",
		label,
		command: { controllerId: "compose.sketchpad.shell", command, ...(args !== undefined ? { args } : {}) },
	};
}
//#endregion 🔖KitHelpers

//#region 🔖Topology
const SKETCHPAD_FLAT_HANDLE_SEPARATOR = "::";

/** @emoji 🔗 Re-exports {@link PLATFORM_TOPOLOGY_STORE_PREFIX} for sketchpad topology stores. */
export const SKETCHPAD_TOPOLOGY_STORE_PREFIX = PLATFORM_TOPOLOGY_STORE_PREFIX;

type SketchpadPuzzle2dFixtureV1 = {
	readonly schema: "puzzle.2d.fixture/v1";
	readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
	readonly nodes: readonly Record<string, unknown>[];
	readonly edges: readonly Record<string, unknown>[];
};

type SketchpadVolumeFixtureV1 = {
	readonly schema: "puzzle.3d.fixture/v1";
	readonly domain: string;
	readonly camera: {
		readonly position: readonly [number, number, number];
		readonly target: readonly [number, number, number];
		readonly zoom: number;
	};
	readonly objects: readonly Record<string, unknown>[];
	readonly attractions: readonly Record<string, unknown>[];
};

function sketchpadFlatPartCenterFromTopLeft(
	position: { readonly x: number; readonly y: number },
	frame: { readonly width: number; readonly height: number },
): { x: number; y: number } {
	return { x: position.x + frame.width / 2, y: position.y + frame.height / 2 };
}

function sketchpadFlatCameraFromPartCenters(centers: readonly { x: number; y: number }[]): SketchpadPuzzle2dFixtureV1["camera"] {
	if (centers.length === 0) return { x: 0, y: 0, zoom: 1 };
	let minX = Infinity;
	let maxX = -Infinity;
	let minY = Infinity;
	let maxY = -Infinity;
	for (const point of centers) {
		minX = Math.min(minX, point.x);
		maxX = Math.max(maxX, point.x);
		minY = Math.min(minY, point.y);
		maxY = Math.max(maxY, point.y);
	}
	const cx = (minX + maxX) / 2;
	const cy = (minY + maxY) / 2;
	const halfSpan = Math.max((maxX - minX) / 2, (maxY - minY) / 2, 40, 1);
	const refShort = SKETCHPAD_TOPOLOGY_ICON_WIDTH * 8;
	const zoom = Math.min(1, refShort / (2 * halfSpan));
	return { x: cx, y: cy, zoom };
}

function sketchpadFlatHandleCompoundId(left: string, right: string): string {
	return `${left}${SKETCHPAD_FLAT_HANDLE_SEPARATOR}${right}`;
}

function sketchpadConnectionTransformParamsFromDto(connection: Record<string, unknown>): Record<string, number> {
	const out: Record<string, number> = {};
	for (const key of ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"] as const) {
		const value = connection[key];
		if (typeof value === "number" && Number.isFinite(value)) {
			out[key] = value;
		}
	}
	return out;
}

type SketchpadConnectorGeometry = {
	readonly id: string;
	readonly point: readonly [number, number, number];
	readonly direction: readonly [number, number, number];
	readonly t: number;
};

/** @emoji 🔌 Lists type-local connector geometry for a design piece. */
function sketchpadPieceConnectorGeometry(
	piece: { readonly id: string; readonly type?: unknown; readonly blueprint?: unknown },
	kit?: Kit,
): readonly SketchpadConnectorGeometry[] {
	const typeId = sketchpadReadEntityId((piece as { type?: unknown; blueprint?: unknown }).type ?? piece.blueprint);
	const type = typeId && kit ? findTypeInKit(kit, typeId) : undefined;
	const connectors = ((type as { connectors?: readonly Record<string, unknown>[] } | undefined)?.connectors ?? []) as readonly Record<string, unknown>[];
	return connectors
		.map((connector) => {
			const id = sketchpadReadEntityId(connector);
			if (!id) return null;
			const point = connector["point"] as { readonly x?: number; readonly y?: number; readonly z?: number } | undefined;
			const direction = connector["direction"] as { readonly x?: number; readonly y?: number; readonly z?: number } | undefined;
			return {
				id,
				point: [point?.x ?? 0, point?.y ?? 0, point?.z ?? 0] as const,
				direction: [direction?.x ?? 0, direction?.y ?? 0, direction?.z ?? 1] as const,
				t: typeof connector["t"] === "number" && Number.isFinite(connector["t"]) ? connector["t"] : 0,
			};
		})
		.filter((row): row is SketchpadConnectorGeometry => row !== null);
}

function sketchpadTopologyAnchorFullId(partId: string, anchorId: string): string {
	return `${partId}:${anchorId}`;
}

/** @emoji 🧩 Stable FiveD instance id for kit wires surfaces. */
export function sketchpadKitWiresInstanceId(kitId: string): string {
	return wiresKitInstanceId(kitId);
}

/** @emoji 🧩 Stable FiveD instance id for a design scene (volume). */
export function sketchpadDesignSceneInstanceId(kitId: string, designId: string): string {
	return `${kitId}:${designId}:scene`;
}

/** @emoji 🧩 Stable FiveD instance id for a design diagram (flat). */
export function sketchpadDesignDiagramInstanceId(kitId: string, designId: string): string {
	return `${kitId}:${designId}:diagram`;
}

/** @emoji 🧩 Surface id prefix for per-representation type CAD windows. */
export const SKETCHPAD_SURFACE_TYPE_REP_PREFIX = "compose.sketchpad.surface.type.representation/v1";

/** @emoji 🧩 Stable FiveD instance id for a type CAD scene (volume). */
export function sketchpadTypeSceneInstanceId(kitId: string, typeId: string): string {
	return `${kitId}:type:${typeId}:scene`;
}

/** @emoji 🧩 Surface id for one type representation CAD window. */
export function sketchpadTypeRepresentationSurfaceId(kitId: string, typeId: string, representationId: string): string {
	return `${SKETCHPAD_SURFACE_TYPE_REP_PREFIX}:${kitId}:${typeId}:${representationId}`;
}

/** @emoji 🔍 Parses a type representation surface id into route segments. */
export function sketchpadParseTypeRepresentationSurfaceId(surfaceId: string): {
	readonly kitId: string;
	readonly typeId: string;
	readonly representationId: string;
} | null {
	if (!surfaceId.startsWith(`${SKETCHPAD_SURFACE_TYPE_REP_PREFIX}:`)) return null;
	const parts = surfaceId.slice(SKETCHPAD_SURFACE_TYPE_REP_PREFIX.length + 1).split(":");
	if (parts.length !== 3) return null;
	return { kitId: parts[0]!, typeId: parts[1]!, representationId: parts[2]! };
}

/** @emoji 🧩 Stable FiveD instance id for one type representation scene. */
export function sketchpadTypeRepresentationSceneInstanceId(kitId: string, typeId: string, representationId: string): string {
	return `${kitId}:type:${typeId}:rep:${representationId}:scene`;
}

/** @emoji 🔍 Parses sketchpad FiveD {@link Puzzle5dModel.instanceId} segments. */
export function parseSketchpadPuzzleInstanceId(instanceId: string): {
	readonly kitId: string | null;
	readonly designId: string | null;
	readonly typeId: string | null;
	readonly pane: "kit-wires" | "scene" | "diagram" | "type-scene" | null;
} {
	const parts = instanceId.split(":");
	if (parts.length === 3 && parts[1] === "kit" && parts[2] === "wires") {
		return { kitId: parts[0] ?? null, designId: null, typeId: null, pane: "kit-wires" };
	}
	if (parts.length === 6 && parts[1] === "type" && parts[3] === "rep" && parts[5] === "scene") {
		return { kitId: parts[0] ?? null, designId: null, typeId: parts[2] ?? null, pane: "type-scene" };
	}
	if (parts.length === 4 && parts[1] === "type" && parts[3] === "scene") {
		return { kitId: parts[0] ?? null, designId: null, typeId: parts[2] ?? null, pane: "type-scene" };
	}
	if (parts.length === 3 && parts[2] === "scene") {
		return { kitId: parts[0] ?? null, designId: parts[1] ?? null, typeId: null, pane: "scene" };
	}
	if (parts.length === 3 && parts[2] === "diagram") {
		return { kitId: parts[0] ?? null, designId: parts[1] ?? null, typeId: null, pane: "diagram" };
	}
	return { kitId: null, designId: null, typeId: null, pane: null };
}

/** @emoji 🔑 Delegates to {@link platformTopologyStoreId}. */
export function sketchpadTopologyStoreId(instanceId: string): string {
	return platformTopologyStoreId(instanceId);
}

function sketchpadEmptyVolumeFixture(): SketchpadVolumeFixtureV1 {
	return {
		schema: "puzzle.3d.fixture/v1",
		domain: "architecture",
		camera: { position: [12, 12, 12], target: [0, 0, 0], zoom: 1 },
		objects: [],
		attractions: [],
	};
}

//#region 🔖KitWires
/** @emoji 🔗 Rust-backed reference rows for kit wires (design transitive refs, piece blueprints). */
export type SketchpadKitWiresReferenceData = {
	readonly designTransitiveTypes: ReadonlyMap<string, readonly string[]>;
	readonly designTransitiveDesigns: ReadonlyMap<string, readonly string[]>;
	readonly pieceBlueprints: ReadonlyMap<string, { readonly kind: "Type" | "Design"; readonly id: string }>;
};

const SKETCHPAD_KIT_WIRES_FILE_NODE_KIND_CATALOGS: NonNullable<WiresFixtureV1["kindCatalogs"]>["identityKinds"] = [
	{ id: "compose.kit.kit", name: "Kit", shape: "rectangle", color: "var(--color-dark-8-9)" },
	{ id: "compose.kit.typology", name: "Typology", shape: "rectangle", color: "var(--color-dark-6-7)" },
	{ id: "compose.kit.folder", name: "Folder", shape: "rectangle", color: "var(--color-dark-5-7)" },
	{ id: "compose.kit.design", name: "Design", shape: "circle", color: "var(--color-secondary)" },
	{ id: "compose.kit.type", name: "Type", shape: "rectangle", color: "var(--color-tertiary)" },
	{ id: "compose.kit.piece", name: "Piece", shape: "rectangle", color: "var(--color-warning)" },
	{ id: "compose.kit.connection", name: "Connection", shape: "rectangle", color: "var(--color-primary)" },
	{ id: "compose.kit.representation", name: "Representation", shape: "rectangle", color: "var(--color-info)" },
	{ id: "compose.kit.port", name: "Port", shape: "rectangle", color: "var(--color-success)" },
	{ id: "compose.kit.connector", name: "Connector", shape: "rectangle", color: "var(--color-secondary)" },
	{ id: "compose.kit.file", name: "File", shape: "rectangle", color: "var(--color-gray)" },
	{ id: "compose.kit.family", name: "Family", shape: "rectangle", color: "var(--color-gray-400)" },
];

const SKETCHPAD_KIT_WIRES_RELATIONSHIP_KIND_CATALOGS: NonNullable<WiresFixtureV1["kindCatalogs"]>["relationshipKinds"] = [
	{ id: "wires.owns", name: "Owns", color: "var(--color-muted-foreground)", pattern: "solid" },
	{ id: "wires.is", name: "Is", color: "var(--color-secondary)", pattern: "solid" },
	{ id: "wires.references", name: "References", color: "var(--color-tertiary)", pattern: "dashed" },
	{ id: "wires.has", name: "Has", color: "var(--color-success)", pattern: "dotted" },
];

function sketchpadKitWiresFileNodeKindCatalog(fileNodeKindId: string) {
	return SKETCHPAD_KIT_WIRES_FILE_NODE_KIND_CATALOGS.find((row) => row.id === `compose.kit.${fileNodeKindId}`);
}

function sketchpadKitWiresIdentityKindId(nodeId: string): string {
	return `compose.kit.identity.${nodeId}`;
}

function sketchpadKitEntityIconField(value: unknown): string | undefined {
	return typeof value === "string" && value.trim() !== "" ? value.trim() : undefined;
}

/** @emoji 🖼️ Reads the declared icon on a kit entity backing a VFS node (not the generic file-node-kind icon). */
export function sketchpadKitEntityAvailableIcon(kit: Kit, nodeId: string, fileNodeKindId: string): string | undefined {
	switch (fileNodeKindId) {
		case "kit":
			return (
				sketchpadKitEntityIconField((kit as { icon?: unknown }).icon) ??
				sketchpadKitEntityIconField((kit.metadata as { icon?: unknown } | undefined)?.icon)
			);
		case "type":
			return sketchpadKitEntityIconField(findTypeInKit(kit, nodeId)?.icon);
		case "design":
			return sketchpadKitEntityIconField(findDesignInKit(kit, nodeId)?.icon);
		case "typology": {
			const topo = sketchpadKitTypologyRows(kit).find((row) => row.id === nodeId);
			return sketchpadKitEntityIconField((topo as { icon?: unknown } | undefined)?.icon);
		}
		case "family": {
			const family = sketchpadReadKitFamilyRows(kit).find((row) => String(row["id"] ?? "") === nodeId);
			return sketchpadKitEntityIconField(family?.icon);
		}
		case "piece": {
			for (const design of sketchpadKitDesignRows(kit)) {
				const piece = design.pieces?.find((row) => row.id === nodeId);
				if (!piece) continue;
				const blueprint = sketchpadPieceBlueprintFromDto(piece, kit);
				if (blueprint?.kind === "Type") return sketchpadKitEntityAvailableIcon(kit, blueprint.id, "type");
				if (blueprint?.kind === "Design") return sketchpadKitEntityAvailableIcon(kit, blueprint.id, "design");
			}
			return undefined;
		}
		default:
			return undefined;
	}
}

function sketchpadKitWiresCatalogIconKey(raw: string | undefined): string | undefined {
	const icon = raw?.trim();
	if (!icon) return undefined;
	const svgMatch = icon.match(/(?:^|\/)([^/]+)\.svg$/i);
	if (svgMatch) return svgMatch[1]!;
	const decoded = decodeIcon(icon);
	if (decoded) return encodeIcon(decoded);
	if (icon.includes("<svg") || icon.startsWith("<?xml")) return icon;
	if (iconSvgMarkup(icon)) return icon;
	return encodeIcon({ kind: "catalog", key: icon });
}

/** @emoji 🖼️ Puzzle 2d {@link iconKind} for a kit wires identity icon (catalog id, emoji, typst, data URL, or inline SVG). */
function sketchpadKitWiresResolveIconKind(raw: string | undefined): string | undefined {
	const catalogKey = sketchpadKitWiresCatalogIconKey(raw);
	if (catalogKey) return catalogKey;
	const icon = raw?.trim();
	if (!icon) return undefined;
	if (icon.includes("<svg") || icon.startsWith("<?xml")) return icon;
	return undefined;
}

/** @emoji 🖼️ Avatar icon for a kit wires identity: entity icon only (never the generic file-node-kind icon). */
function sketchpadKitWiresNodeIconKind(kit: Kit, node: VirtualFileSystemVisibleNode): string | undefined {
	const raw = node.icon ?? sketchpadKitEntityAvailableIcon(kit, node.id, node.fileNodeKindId);
	return sketchpadKitWiresResolveIconKind(raw);
}

function sketchpadKitWiresIdentityKindCatalogRow(
	kit: Kit,
	node: VirtualFileSystemVisibleNode,
): NonNullable<WiresFixtureV1["kindCatalogs"]>["identityKinds"][number] {
	const identityKindId = sketchpadKitWiresIdentityKindId(node.id);
	const template = sketchpadKitWiresFileNodeKindCatalog(node.fileNodeKindId);
	const rawIcon = node.icon ?? sketchpadKitEntityAvailableIcon(kit, node.id, node.fileNodeKindId);
	const catalogIcon = sketchpadKitWiresCatalogIconKey(rawIcon);
	return {
		id: identityKindId,
		name: node.name,
		shape: node.fileNodeKindId === "design" ? "circle" : "rectangle",
		...(template?.color !== undefined ? { color: template.color } : {}),
		...(catalogIcon ? { icon: catalogIcon } : {}),
	};
}

function sketchpadKitWiresContainmentKind(parentKind: string): RelationshipKind {
	switch (parentKind) {
		case "type":
		case "design":
		case "piece":
			return "has";
		default:
			return "owns";
	}
}

function sketchpadKitWiresLayoutPosition(fileNodeKindId: string, index: number): { readonly x: number; readonly y: number } {
	const layerY: Record<string, number> = {
		kit: 0,
		typology: 80,
		folder: 80,
		family: 80,
		design: 160,
		type: 160,
		piece: 240,
		connection: 280,
		representation: 320,
		port: 360,
		connector: 400,
		file: 440,
	};
	const y = layerY[fileNodeKindId] ?? 200;
	return { x: (index % 12) * 96, y: y + Math.floor(index / 12) * 56 };
}

function sketchpadKitWiresEdgeId(kind: RelationshipKind, sourceId: string, targetId: string): string {
	return `wires-${kind}-${sourceId}-${targetId}`;
}

/** @emoji 🔗 Visible VFS nodes for kit wires, including the kit root ({@link visibleVirtualFileSystemNodesFromTree} omits it). */
export function sketchpadKitWiresVisibleNodes(
	root: VirtualFileSystemNodeRecord,
	visibleFromTree: readonly VirtualFileSystemVisibleNode[],
): readonly VirtualFileSystemVisibleNode[] {
	if (visibleFromTree.some((node) => node.id === root.id)) return visibleFromTree;
	return [{ ...root, parentId: null }, ...visibleFromTree];
}

function sketchpadExpandedDesignIds(
	scope: VirtualFileSystemScope,
	expandedIds: readonly string[],
	vfsMeta: ReadonlyMap<string, { readonly fileNodeKindId: string }>,
): ReadonlySet<string> {
	const out = new Set<string>();
	for (const id of expandedIds) {
		if (vfsMeta.get(id)?.fileNodeKindId === "design") out.add(id);
	}
	return out;
}

function sketchpadCollectDesignTransitiveTypesFromDto(design: Design, kit: Kit, rootDesignId: string): readonly string[] {
	const out = new Set<string>();
	const visit = (row: Design) => {
		for (const piece of row.pieces ?? []) {
			const typeId = (piece as { type?: { id?: string } }).type?.id ?? (piece as { ref?: { type?: { id?: string } } }).ref?.type?.id;
			if (typeId) out.add(String(typeId));
			const nestedId =
				(piece as { design?: { id?: string } }).design?.id ?? (piece as { ref?: { design?: { id?: string } } }).ref?.design?.id;
			if (nestedId && nestedId !== rootDesignId) {
				const nested = findDesignInKit(kit, nestedId);
				if (nested) visit(nested);
			}
		}
	};
	visit(design);
	return [...out];
}

function sketchpadCollectDesignTransitiveDesignsFromDto(design: Design, kit: Kit, rootDesignId: string): readonly string[] {
	const out = new Set<string>();
	const visit = (row: Design) => {
		for (const piece of row.pieces ?? []) {
			const nestedId =
				(piece as { design?: { id?: string } }).design?.id ?? (piece as { ref?: { design?: { id?: string } } }).ref?.design?.id;
			if (nestedId && nestedId !== rootDesignId) {
				out.add(String(nestedId));
				const nested = findDesignInKit(kit, nestedId);
				if (nested) visit(nested);
			}
		}
	};
	visit(design);
	return [...out];
}

function sketchpadPieceBlueprintFromDto(piece: Piece, kit: Kit): { readonly kind: "Type" | "Design"; readonly id: string } | null {
	const row = piece as {
		readonly type?: { readonly id?: string };
		readonly design?: { readonly id?: string };
		readonly blueprint?: { readonly id?: string };
		readonly ref?: { readonly type?: { readonly id?: string }; readonly design?: { readonly id?: string } };
	};
	if (row.type?.id) return { kind: "Type", id: String(row.type.id) };
	if (row.design?.id) return { kind: "Design", id: String(row.design.id) };
	if (row.ref?.type?.id) return { kind: "Type", id: String(row.ref.type.id) };
	if (row.ref?.design?.id) return { kind: "Design", id: String(row.ref.design.id) };
	const blueprintId = row.blueprint?.id;
	if (!blueprintId) return null;
	if (findTypeInKit(kit, blueprintId)) return { kind: "Type", id: blueprintId };
	if (findDesignInKit(kit, blueprintId)) return { kind: "Design", id: blueprintId };
	return null;
}

/** @emoji 🔗 Loads design transitive refs and piece blueprints from the rust-backed {@link ComposeJsKitStore}. */
export async function sketchpadFetchKitWiresReferences(
	store: ComposeKitStore,
	kit: Kit,
	visible: readonly VirtualFileSystemVisibleNode[],
	expandedDesignIds: ReadonlySet<string>,
	vfsMeta: ReadonlyMap<string, { readonly fileNodeKindId: string; readonly designId?: string }>,
): Promise<SketchpadKitWiresReferenceData> {
	const visibleIds = new Set(visible.map((node) => node.id));
	const designTransitiveTypes = new Map<string, readonly string[]>();
	const designTransitiveDesigns = new Map<string, readonly string[]>();
	const pieceBlueprints = new Map<string, { readonly kind: "Type" | "Design"; readonly id: string }>();

	if (store instanceof ComposeJsKitStore) {
		const jsStore = store.jsStore;
		for (const node of visible) {
			if (node.fileNodeKindId === "design" && !expandedDesignIds.has(node.id)) {
				const design = jsStore.design(node.id) as JsDesign;
				const [types, designs] = await Promise.all([design.referencesTypesTransitive(), design.referencesDesignsTransitive()]);
				designTransitiveTypes.set(
					node.id,
					types.map((t) => t.id).filter((id) => visibleIds.has(id)),
				);
				designTransitiveDesigns.set(
					node.id,
					designs.map((d) => d.id).filter((id) => visibleIds.has(id) && id !== node.id),
				);
			}
			if (node.fileNodeKindId === "piece") {
				const designId = vfsMeta.get(node.id)?.designId ?? node.parentId ?? "";
				if (!designId) continue;
				const piece = jsStore.design(designId).piece(node.id) as JsPiece;
				const blueprint = await piece.blueprint();
				if (blueprint) pieceBlueprints.set(node.id, { kind: blueprint.blueprintKind, id: blueprint.id });
			}
		}
		return { designTransitiveTypes, designTransitiveDesigns, pieceBlueprints };
	}

	for (const node of visible) {
		if (node.fileNodeKindId === "design" && !expandedDesignIds.has(node.id)) {
			const design = findDesignInKit(kit, node.id);
			if (!design) continue;
			designTransitiveTypes.set(
				node.id,
				sketchpadCollectDesignTransitiveTypesFromDto(design, kit, node.id).filter((id) => visibleIds.has(id)),
			);
			designTransitiveDesigns.set(
				node.id,
				sketchpadCollectDesignTransitiveDesignsFromDto(design, kit, node.id).filter((id) => visibleIds.has(id)),
			);
		}
		if (node.fileNodeKindId === "piece") {
			const designId = vfsMeta.get(node.id)?.designId ?? node.parentId ?? "";
			const design = findDesignInKit(kit, designId);
			const piece = design?.pieces?.find((row) => row.id === node.id);
			if (!piece) continue;
			const blueprint = sketchpadPieceBlueprintFromDto(piece, kit);
			if (blueprint) pieceBlueprints.set(node.id, blueprint);
		}
	}
	return { designTransitiveTypes, designTransitiveDesigns, pieceBlueprints };
}

/** @emoji 🔗 Builds a {@link WiresFixtureV1} from visible kit VFS nodes and rust reference data. */
export function sketchpadKitWiresFixtureFromVisible(
	kit: Kit,
	kitId: string,
	visible: readonly VirtualFileSystemVisibleNode[],
	references: SketchpadKitWiresReferenceData,
	expandedDesignIds: ReadonlySet<string>,
): WiresFixtureV1 {
	const visibleIds = new Set(visible.map((node) => node.id));
	const identityByNodeId = new Map<string, number>();
	const identities: WiresFixtureV1["identities"][number][] = [];
	const relationships: WiresFixtureV1["relationships"][number][] = [];
	const boardNodes: { id: string; x: number; y: number; text?: string; iconKind?: string; nodeKind?: string; shape?: "circle" | "rectangle"; width?: number; height?: number; radius?: number }[] =
		[];
	const boardEdges: { id: string; source: string; target: string }[] = [];
	const edgeIds = new Set<string>();
	let relationshipId = 1;
	let identityId = 1;
	const kindIndex = new Map<string, number>();

	const pushRelationship = (kind: RelationshipKind, sourceId: string, targetId: string) => {
		if (!visibleIds.has(sourceId) || !visibleIds.has(targetId) || sourceId === targetId) return;
		const edgeId = sketchpadKitWiresEdgeId(kind, sourceId, targetId);
		if (edgeIds.has(edgeId)) return;
		edgeIds.add(edgeId);
		const sourceIdentityId = identityByNodeId.get(sourceId);
		const targetIdentityId = identityByNodeId.get(targetId);
		if (sourceIdentityId == null || targetIdentityId == null) return;
		relationships.push({
			relationshipId: relationshipId++,
			sourceIdentityId,
			targetIdentityId,
			kind,
			edgeId,
		});
		boardEdges.push({ id: edgeId, source: sourceId, target: targetId });
	};

	const identityKindCatalogRows = new Map<string, NonNullable<WiresFixtureV1["kindCatalogs"]>["identityKinds"][number]>();
	for (const node of visible) {
		const kindCount = kindIndex.get(node.fileNodeKindId) ?? 0;
		kindIndex.set(node.fileNodeKindId, kindCount + 1);
		const position = sketchpadKitWiresLayoutPosition(node.fileNodeKindId, kindCount);
		const identityKindId = sketchpadKitWiresIdentityKindId(node.id);
		identityKindCatalogRows.set(identityKindId, sketchpadKitWiresIdentityKindCatalogRow(kit, node));
		identityByNodeId.set(node.id, identityId);
		identities.push({
			identityId: identityId++,
			label: node.name,
			nodeId: node.id,
			identityKind: identityKindId,
		});
		const shape = node.fileNodeKindId === "design" ? "circle" : "rectangle";
		const iconKind = sketchpadKitWiresNodeIconKind(kit, node);
		boardNodes.push({
			id: node.id,
			x: position.x,
			y: position.y,
			text: node.name,
			nodeKind: identityKindId,
			shape,
			...(iconKind ? { iconKind } : {}),
			...(shape === "circle" ? { radius: 28 } : { width: 120, height: 40 }),
		});
	}

	const kindById = new Map(visible.map((node) => [node.id, node.fileNodeKindId] as const));
	for (const node of visible) {
		if (node.parentId == null) continue;
		const parentKind = kindById.get(node.parentId) ?? "kit";
		pushRelationship(sketchpadKitWiresContainmentKind(parentKind), node.parentId, node.id);
	}

	for (const [pieceId, blueprint] of references.pieceBlueprints) {
		if (!visibleIds.has(blueprint.id)) continue;
		pushRelationship("is", pieceId, blueprint.id);
	}

	for (const node of visible) {
		if (node.fileNodeKindId !== "design" || expandedDesignIds.has(node.id)) continue;
		for (const typeId of references.designTransitiveTypes.get(node.id) ?? []) {
			pushRelationship("references", node.id, typeId);
		}
		for (const designId of references.designTransitiveDesigns.get(node.id) ?? []) {
			pushRelationship("references", node.id, designId);
		}
	}

	const camera = sketchpadFlatCameraFromPartCenters(boardNodes.map((node) => ({ x: node.x, y: node.y })));
	return {
		schema: "reasoning.wires.fixture/v1",
		source: { kitPath: `/kits/${kitId}`, kitId, kitName: String(kit.name ?? kitId) },
		identities,
		relationships,
		board: {
			schema: "reasoning.mindmap.fixture/v1",
			camera,
			nodes: boardNodes,
			edges: boardEdges,
		},
		kindCatalogs: {
			identityKinds: [...identityKindCatalogRows.values()],
			relationshipKinds: SKETCHPAD_KIT_WIRES_RELATIONSHIP_KIND_CATALOGS,
		},
	};
}

function sketchpadTopologyPayloadForKitWires(fixture: WiresFixtureV1): PlatformTopologyPayload {
	return sketchpadTopologyPayload(wiresFixtureBoard(fixture) as unknown as SketchpadPuzzle2dFixtureV1, sketchpadEmptyVolumeFixture());
}
//#endregion 🔖KitWires

type SketchpadKitDiagramNodeKind = "type" | "design" | "quality" | "port" | "file" | "folder" | "author";

function sketchpadKitDiagramNodeFrame(kind: SketchpadKitDiagramNodeKind): {
	readonly width: number;
	readonly height: number;
	readonly shape: "circle" | "rectangle";
} {
	switch (kind) {
		case "design":
			return { width: 48, height: 48, shape: "circle" };
		case "type":
			return { width: 120, height: 48, shape: "rectangle" };
		case "file":
			return { width: 100, height: 48, shape: "rectangle" };
		default:
			return { width: 140, height: 36, shape: "rectangle" };
	}
}

function sketchpadKitDiagramPortLabel(port: Record<string, unknown>): string {
	const label = port["label"];
	if (typeof label === "string" && label.length > 0) return label;
	const code = port["code"];
	if (typeof code === "string" && code.length > 0) return code;
	const name = port["name"];
	if (typeof name === "string" && name.length > 0) return name;
	return String(port["id"] ?? "");
}

/** @emoji 👨‍👩‍👦 Reads kit-level {@code families} rows from a denormalized bundle or projection DTO. */
export function sketchpadReadKitFamilyRows(kit: Kit): readonly Record<string, unknown>[] {
	const raw = (kit as { families?: unknown }).families;
	if (raw == null) return [];
	const asRow = (entry: unknown): entry is Record<string, unknown> =>
		entry != null && typeof entry === "object" && !Array.isArray(entry);
	if (Array.isArray(raw)) return raw.filter(asRow);
	if (typeof raw === "object") {
		const items = (raw as { items?: readonly unknown[] }).items;
		if (Array.isArray(items)) return items.filter(asRow);
	}
	return [];
}

function sketchpadReadFamilyPortRows(family: Record<string, unknown>): readonly Record<string, unknown>[] {
	const raw = family["ports"];
	if (raw == null) return [];
	const asRow = (entry: unknown): entry is Record<string, unknown> =>
		entry != null && typeof entry === "object" && !Array.isArray(entry);
	if (Array.isArray(raw)) return raw.filter(asRow);
	if (typeof raw === "object") {
		const items = (raw as { items?: readonly unknown[] }).items;
		if (Array.isArray(items)) return items.filter(asRow);
	}
	return [];
}

function sketchpadForEachKitPortRecord(kit: Kit, visit: (port: Record<string, unknown>) => void): void {
	for (const type of kit.types ?? []) {
		for (const port of (type as { ports?: readonly unknown[] }).ports ?? []) {
			if (port != null && typeof port === "object" && !Array.isArray(port)) visit(port as Record<string, unknown>);
		}
		for (const connector of type.connectors ?? []) {
			const port = (connector as { port?: unknown }).port;
			if (port != null && typeof port === "object" && !Array.isArray(port)) visit(port as Record<string, unknown>);
		}
	}
	for (const family of sketchpadReadKitFamilyRows(kit)) {
		for (const port of sketchpadReadFamilyPortRows(family)) visit(port);
	}
}

/** @emoji 🔌 Collects unique ports on kit kinds, connectors, and kit-level families. */
export function sketchpadCollectKitPorts(kit: Kit): readonly { readonly id: string; readonly name: string }[] {
	const byId = new Map<string, { id: string; name: string }>();
	const remember = (port: Record<string, unknown>) => {
		const id = sketchpadReadEntityId(port);
		if (!id || byId.has(id)) return;
		byId.set(id, { id, name: sketchpadKitDiagramPortLabel(port) });
	};
	sketchpadForEachKitPortRecord(kit, remember);
	return [...byId.values()];
}

function sketchpadCollectKitPortRecords(kit: Kit): readonly Record<string, unknown>[] {
	const byId = new Map<string, Record<string, unknown>>();
	const remember = (port: Record<string, unknown>) => {
		const id = sketchpadReadEntityId(port);
		if (!id) return;
		const prev = byId.get(id);
		if (!prev) {
			byId.set(id, { ...port });
			return;
		}
		const mergedCompat = new Set<string>();
		for (const ref of sketchpadReadCompatiblePortIds(prev)) mergedCompat.add(ref);
		for (const ref of sketchpadReadCompatiblePortIds(port)) mergedCompat.add(ref);
		byId.set(id, {
			...prev,
			...port,
			compatiblePorts: [...mergedCompat].map((compatId) => ({ id: compatId })),
		});
	};
	sketchpadForEachKitPortRecord(kit, remember);
	return [...byId.values()];
}

/** @emoji 🔀 Merges port compat maps; later map entries override earlier ones for the same port id. */
export function sketchpadMergePortCompatMaps(
	primary: ReadonlyMap<string, readonly { readonly id: string }[]>,
	overlay: ReadonlyMap<string, readonly { readonly id: string }[]>,
): Map<string, readonly { readonly id: string }[]> {
	const merged = new Map(primary);
	for (const [portId, refs] of overlay) merged.set(portId, refs);
	return merged;
}

/** @emoji 🗺️ Collects port {@code compatiblePorts} refs from a kit snapshot (bundle or DTO). */
export function sketchpadExtractPortCompatById(kit: Kit): Map<string, readonly { readonly id: string }[]> {
	const map = new Map<string, readonly { readonly id: string }[]>();
	for (const port of sketchpadCollectKitPortRecords(kit)) {
		const id = sketchpadReadEntityId(port);
		const compatIds = sketchpadReadCompatiblePortIds(port);
		if (id && compatIds.length > 0) map.set(id, compatIds.map((compatId) => ({ id: compatId })));
	}
	return map;
}

/** @emoji 🔗 Re-applies stored {@code compatiblePorts} onto a GraphQL-shaped kit DTO. */
export function sketchpadApplyPortCompatById(
	kit: Kit,
	compatById: ReadonlyMap<string, readonly { readonly id: string }[]>,
): Kit {
	if (compatById.size === 0) return kit;
	const enrichPort = (port: unknown): unknown => {
		if (port == null || typeof port !== "object") return port;
		const row = { ...(port as Record<string, unknown>) };
		const id = sketchpadReadEntityId(row);
		const compat = id ? compatById.get(id) : undefined;
		if (compat?.length) row.compatiblePorts = compat;
		return row;
	};
	const types = (kit.types ?? []).map((type) => ({
		...type,
		ports: ((type as { ports?: readonly unknown[] }).ports ?? []).map(enrichPort),
		connectors: (type.connectors ?? []).map((connector) => ({
			...connector,
			port: enrichPort((connector as { port?: unknown }).port),
		})),
	}));
	const familyRows = sketchpadReadKitFamilyRows(kit);
	const families =
		familyRows.length === 0
			? undefined
			: familyRows.map((family) => ({
					...family,
					ports: sketchpadReadFamilyPortRows(family).map(enrichPort),
				}));
	return { ...kit, types, ...(families != null ? { families } : {}) } as Kit;
}

function sketchpadReadCompatiblePortIds(port: Record<string, unknown>): readonly string[] {
	const raw = port["compatiblePorts"];
	if (raw == null) return [];
	const ids: string[] = [];
	const visit = (entry: unknown) => {
		const id = sketchpadReadEntityId(entry);
		if (id) ids.push(id);
	};
	if (Array.isArray(raw)) {
		for (const entry of raw) visit(entry);
		return ids;
	}
	if (typeof raw === "object" && raw !== null) {
		const items = (raw as { items?: readonly unknown[] }).items;
		if (Array.isArray(items)) {
			for (const entry of items) visit(entry);
			return ids;
		}
	}
	return ids;
}

/** @emoji 🔗 Union-find map grouping kit ports by {@code compatiblePorts} and shared {@code code}. */
export function sketchpadCreatePortGroupMap(
	ports: readonly { readonly id: string; readonly code?: string | null; readonly compatiblePorts?: readonly unknown[] }[],
): Map<string, string> {
	const parent = new Map<string, string>();
	const register = (id: string) => {
		if (!parent.has(id)) parent.set(id, id);
	};
	for (const port of ports) {
		const id = sketchpadReadEntityId(port);
		if (id) register(id);
	}
	const find = (id: string): string => {
		const direct = parent.get(id);
		if (!direct) return id;
		if (direct === id) return direct;
		const root = find(direct);
		parent.set(id, root);
		return root;
	};
	const union = (left: string, right: string) => {
		const leftRoot = find(left);
		const rightRoot = find(right);
		if (leftRoot === rightRoot) return;
		parent.set(rightRoot, leftRoot);
	};
	for (const port of ports) {
		const id = sketchpadReadEntityId(port);
		if (!id) continue;
		for (const relatedId of sketchpadReadCompatiblePortIds(port as Record<string, unknown>)) {
			register(relatedId);
			union(id, relatedId);
		}
		const code = typeof port.code === "string" ? port.code.trim() : "";
		if (code.length > 0) {
			for (const other of ports) {
				const otherId = sketchpadReadEntityId(other);
				const otherCode = typeof other.code === "string" ? other.code.trim() : "";
				if (otherId && otherId !== id && otherCode === code) union(id, otherId);
			}
		}
	}
	const groups = new Map<string, string>();
	for (const id of parent.keys()) groups.set(id, find(id));
	return groups;
}

/** @emoji ↔️ Adds dashed type adjacency edges for types that share compatible port groups. */
export function sketchpadKitDiagramPushTypeCompatEdges(
	kit: Kit,
	edges: SketchpadPuzzle2dFixtureV1["edges"],
	edgeIds: Set<string>,
): void {
	const ports = sketchpadCollectKitPortRecords(kit);
	if (ports.length === 0) return;
	const groups = sketchpadCreatePortGroupMap(
		ports.map((port) => ({
			id: String(port["id"] ?? ""),
			code: typeof port["code"] === "string" ? port["code"] : null,
			compatiblePorts: sketchpadReadCompatiblePortIds(port).map((compatId) => ({ id: compatId })),
		})),
	);
	const portToTypes = new Map<string, Set<string>>();
	for (const type of kit.types ?? []) {
		for (const connector of type.connectors ?? []) {
			const portId = sketchpadReadEntityId((connector as { port?: unknown }).port);
			if (!portId) continue;
			const typeIds = portToTypes.get(portId) ?? new Set<string>();
			typeIds.add(type.id);
			portToTypes.set(portId, typeIds);
		}
	}
	const rootToTypes = new Map<string, Set<string>>();
	for (const [portId, typeIds] of portToTypes) {
		const root = groups.get(portId) ?? portId;
		const merged = rootToTypes.get(root) ?? new Set<string>();
		for (const typeId of typeIds) merged.add(typeId);
		rootToTypes.set(root, merged);
	}
	for (const typeIds of rootToTypes.values()) {
		if (typeIds.size < 2) continue;
		const sorted = [...typeIds].sort();
		for (let i = 0; i < sorted.length; i++) {
			for (let j = i + 1; j < sorted.length; j++) {
				const left = sorted[i]!;
				const right = sorted[j]!;
				sketchpadKitDiagramPushEdge(edges, edgeIds, `compat-type:${left}-type:${right}`, `type:${left}`, `type:${right}`);
			}
		}
	}
}

/** @emoji 📄 Basename for a kit file row (prefers `name`, then url/path tail, then id). */
function sketchpadKitFileBasename(file: Record<string, unknown>): string {
	const name = file["name"];
	if (typeof name === "string" && name.trim().length > 0) return name.trim();
	const description = file["description"];
	if (typeof description === "string" && description.trim().length > 0) return description.trim();
	const url = file["url"];
	if (typeof url === "string" && url.length > 0) {
		const slash = url.lastIndexOf("/");
		return slash >= 0 ? url.slice(slash + 1) : url;
	}
	const path = file["path"];
	if (typeof path === "string" && path.length > 0) {
		const slash = path.lastIndexOf("/");
		return slash >= 0 ? path.slice(slash + 1) : path;
	}
	return String(file["id"] ?? "");
}

/** @emoji 📄 VFS label for a kit file: basename without extension. */
function sketchpadKitFileDisplayName(basename: string): string {
	const dot = basename.lastIndexOf(".");
	if (dot <= 0) return basename;
	return basename.slice(0, dot);
}

/** @emoji 📄 VFS icon id from a file basename extension (maps in {@link resolveVirtualFileSystemSchemaIcon}). */
function sketchpadKitFileExtensionIconId(basename: string): string {
	const dot = basename.lastIndexOf(".");
	if (dot <= 0 || dot === basename.length - 1) return "file";
	return basename.slice(dot + 1).toLowerCase();
}

function sketchpadKitDiagramFileLabel(file: Record<string, unknown>): string {
	return sketchpadKitFileDisplayName(sketchpadKitFileBasename(file));
}

function sketchpadKitVfsFileRowFields(file: Record<string, unknown>): { readonly name: string; readonly icon: string } {
	const basename = sketchpadKitFileBasename(file);
	return {
		name: sketchpadKitFileDisplayName(basename),
		icon: sketchpadKitFileExtensionIconId(basename),
	};
}

function sketchpadKitDiagramPushEdge(
	edges: SketchpadPuzzle2dFixtureV1["edges"],
	edgeIds: Set<string>,
	id: string,
	source: string,
	target: string,
): void {
	if (edgeIds.has(id)) return;
	edgeIds.add(id);
	edges.push({ id, source, target });
}

function sketchpadKitDiagramNode(
	kind: SketchpadKitDiagramNodeKind,
	entityId: string,
	label: string,
	root: boolean,
): { node: SketchpadPuzzle2dFixtureV1["nodes"][number]; center: { x: number; y: number } } {
	const nodeId = `${kind}:${entityId}`;
	const frame = sketchpadKitDiagramNodeFrame(kind);
	const center = sketchpadFlatPartCenterFromTopLeft({ x: 0, y: 0 }, frame);
	const base = {
		id: nodeId,
		x: center.x,
		y: center.y,
		text: label,
		nodeKind: `compose.kit.${kind}`,
		root,
		handles: [] as readonly Record<string, unknown>[],
	};
	if (frame.shape === "circle") {
		return {
			node: { ...base, shape: "circle", radius: frame.width / 2 },
			center,
		};
	}
	return {
		node: { ...base, shape: "rectangle", width: frame.width, height: frame.height },
		center,
	};
}

function sketchpadTopologyPayload(flat: SketchpadPuzzle2dFixtureV1, volume: SketchpadVolumeFixtureV1): PlatformTopologyPayload {
	return { flat: flat as unknown as Record<string, unknown>, volume: volume as unknown as Record<string, unknown> };
}

/** @emoji 🗺️ Builds a flat kit topology diagram from kit entities (types, designs, ports, files, …). */
export function sketchpadKitPuzzle2dFixtureFromKit(kit: Kit): SketchpadPuzzle2dFixtureV1 {
	const nodes: SketchpadPuzzle2dFixtureV1["nodes"] = [];
	const edges: SketchpadPuzzle2dFixtureV1["edges"] = [];
	const edgeIds = new Set<string>();
	const centers: { x: number; y: number }[] = [];
	const kindGroups: readonly SketchpadKitDiagramNodeKind[] = ["type", "design", "quality", "port", "file", "folder", "author"];
	for (const kind of kindGroups) {
		let items: readonly { readonly id: string; readonly name: string; readonly parentId?: string }[] = [];
		switch (kind) {
			case "type":
				items = (kit.types ?? []).map((t) => ({
					id: t.id,
					name: t.name ?? t.id,
					parentId: sketchpadReadEntityId((t as { parent?: unknown }).parent) ?? undefined,
				}));
				break;
			case "design":
				items = (kit.designs ?? []).map((d) => ({
					id: d.id,
					name: d.name ?? d.id,
					parentId: sketchpadReadEntityId((d as { parent?: unknown }).parent) ?? undefined,
				}));
				break;
			case "quality":
				items = (kit.qualities ?? []).map((q) => {
					const row = q as { id: string; key?: string; value?: string };
					const key = row.key ?? row.id;
					const label = row.value != null && row.value !== "" ? `${key} · ${row.value}` : key;
					return { id: row.id, name: label };
				});
				break;
			case "port":
				items = sketchpadCollectKitPorts(kit);
				break;
			case "file":
				items = (kit.files ?? []).map((f) => {
					const row = f as Record<string, unknown>;
					return {
						id: String(row["id"] ?? ""),
						name: sketchpadKitDiagramFileLabel(row),
						parentId: sketchpadReadEntityId(row["folder"]) ?? undefined,
					};
				});
				break;
			case "folder":
				items = (kit.folders ?? []).map((f) => {
					const row = f as Record<string, unknown>;
					const path = typeof row["path"] === "string" ? row["path"] : "";
					const slash = path.lastIndexOf("/");
					const name = slash >= 0 ? path.slice(slash + 1) : path || String(row["id"] ?? "");
					return {
						id: String(row["id"] ?? ""),
						name,
						parentId: sketchpadReadEntityId(row["parent"]) ?? undefined,
					};
				});
				break;
			case "author":
				items = (kit.authors ?? []).map((a) => ({
					id: String((a as { id: string }).id),
					name: String((a as { name?: string }).name ?? (a as { id: string }).id),
				}));
				break;
		}
		for (const item of items) {
			if (!item.id) continue;
			const { node, center } = sketchpadKitDiagramNode(kind, item.id, item.name, !item.parentId);
			nodes.push(node);
			centers.push(center);
			if (item.parentId) {
				const parentKind = kind === "file" ? "folder" : kind;
				sketchpadKitDiagramPushEdge(
					edges,
					edgeIds,
					`${kind}-${item.parentId}-${item.id}`,
					`${parentKind}:${item.parentId}`,
					`${kind}:${item.id}`,
				);
			}
		}
	}
	for (const design of kit.designs ?? []) {
		for (const piece of design.pieces ?? []) {
			const typeId = sketchpadReadEntityId((piece as { type?: unknown; blueprint?: unknown }).type ?? (piece as { blueprint?: unknown }).blueprint);
			if (typeId) {
				sketchpadKitDiagramPushEdge(
					edges,
					edgeIds,
					`ref-type:${typeId}-design:${design.id}`,
					`type:${typeId}`,
					`design:${design.id}`,
				);
			}
		}
	}
	for (const type of kit.types ?? []) {
		for (const connector of type.connectors ?? []) {
			const portId = sketchpadReadEntityId((connector as { port?: unknown }).port);
			if (!portId) continue;
			sketchpadKitDiagramPushEdge(
				edges,
				edgeIds,
				`ref-port:${portId}-type:${type.id}`,
				`port:${portId}`,
				`type:${type.id}`,
			);
		}
	}
	sketchpadKitDiagramPushTypeCompatEdges(kit, edges, edgeIds);
	return {
		schema: "puzzle.2d.fixture/v1",
		camera: sketchpadFlatCameraFromPartCenters(centers.length > 0 ? centers : [{ x: 0, y: 0 }]),
		nodes,
		edges,
	};
}

const SKETCHPAD_TOPOLOGY_ICON_WIDTH = 48;
const SKETCHPAD_DESIGN_DIAGRAM_NODE = { width: 80, height: 40 } as const;

type SketchpadKitConnection = {
	readonly id?: string;
	readonly connecting?: { readonly piece?: unknown; readonly connector?: unknown };
	readonly connected?: { readonly piece?: unknown; readonly connector?: unknown };
	readonly parent?: { readonly piece?: unknown; readonly connector?: unknown };
	readonly child?: { readonly piece?: unknown; readonly connector?: unknown };
};

function sketchpadConnectionEndpoints(connection: SketchpadKitConnection): {
	readonly sourcePieceId: string | null;
	readonly targetPieceId: string | null;
	readonly sourceConnectorId: string | null;
	readonly targetConnectorId: string | null;
} {
	const sourcePieceId =
		sketchpadReadEntityId(connection.connecting?.piece) ?? sketchpadReadEntityId(connection.parent?.piece);
	const targetPieceId =
		sketchpadReadEntityId(connection.connected?.piece) ?? sketchpadReadEntityId(connection.child?.piece);
	const sourceConnectorId =
		sketchpadReadEntityId(connection.connecting?.connector) ?? sketchpadReadEntityId(connection.parent?.connector);
	const targetConnectorId =
		sketchpadReadEntityId(connection.connected?.connector) ?? sketchpadReadEntityId(connection.child?.connector);
	return { sourcePieceId, targetPieceId, sourceConnectorId, targetConnectorId };
}

function sketchpadPieceLabel(piece: { readonly id: string; readonly name?: string | null }, kit?: Kit): string {
	const typeId = sketchpadReadEntityId((piece as { type?: unknown }).type);
	const type = typeId && kit ? findTypeInKit(kit, typeId) : undefined;
	return piece.name ?? type?.name ?? piece.id;
}

type SketchpadPiecePose = {
	readonly center?: { readonly u?: number; readonly v?: number };
	readonly plane?: {
		readonly origin?: { readonly x?: number; readonly y?: number; readonly z?: number };
		readonly xAxis?: { readonly x?: number; readonly y?: number; readonly z?: number };
		readonly yAxis?: { readonly x?: number; readonly y?: number; readonly z?: number };
	};
};

function sketchpadPieceAbsolutePose(piece: { readonly id: string }): SketchpadPiecePose | undefined {
	const row = piece as {
		readonly flatPosition?: SketchpadPiecePose;
		readonly position?: SketchpadPiecePose;
		readonly center?: SketchpadPiecePose["center"];
		readonly plane?: SketchpadPiecePose["plane"];
	};
	return row.flatPosition ?? row.position ?? (row.center || row.plane ? { center: row.center, plane: row.plane } : undefined);
}

function sketchpadPieceDiagramUv(piece: { readonly id: string }, index: number): { readonly u: number; readonly v: number } {
	const pose = sketchpadPieceAbsolutePose(piece);
	const center = pose?.center;
	if (center && typeof center.u === "number") {
		return { u: center.u, v: typeof center.v === "number" ? center.v : 0 };
	}
	return { u: (index % 8) * 1.2, v: Math.floor(index / 8) * 1.2 };
}

function sketchpadPieceSceneOrigin(piece: { readonly id: string }, index: number): [number, number, number] {
	const o = sketchpadPieceAbsolutePose(piece)?.plane?.origin;
	if (o) return [o.x ?? 0, o.y ?? 0, o.z ?? 0];
	return [index * 2, 0, 0];
}

/** @emoji 🧭 Converts compose absolute plane axes to a volume orientation quaternion. */
export function sketchpadPlaneAxesToQuaternion(plane: NonNullable<SketchpadPiecePose["plane"]>): [number, number, number, number] {
	const xAxis = plane.xAxis ?? { x: 1, y: 0, z: 0 };
	const yAxis = plane.yAxis ?? { x: 0, y: 1, z: 0 };
	const xx = xAxis.x ?? 1;
	const xy = xAxis.y ?? 0;
	const xz = xAxis.z ?? 0;
	const yx = yAxis.x ?? 0;
	const yy = yAxis.y ?? 1;
	const yz = yAxis.z ?? 0;
	const zx = xy * yz - xz * yy;
	const zy = xz * yx - xx * yz;
	const zz = xx * yy - xy * yx;
	const m00 = xx;
	const m01 = yx;
	const m02 = zx;
	const m10 = xy;
	const m11 = yy;
	const m12 = zy;
	const m20 = xz;
	const m21 = yz;
	const m22 = zz;
	const trace = m00 + m11 + m22;
	if (trace > 0) {
		const s = Math.sqrt(trace + 1) * 2;
		return [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s];
	}
	if (m00 > m11 && m00 > m22) {
		const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
		return [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
	}
	if (m11 > m22) {
		const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
		return [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s];
	}
	const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
	return [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s];
}

function sketchpadPieceSceneOrientation(piece: { readonly id: string }): [number, number, number, number] {
	const plane = sketchpadPieceAbsolutePose(piece)?.plane;
	return plane ? sketchpadPlaneAxesToQuaternion(plane) : [0, 0, 0, 1];
}

function sketchpadPieceSceneScale(piece: { readonly id: string }): [number, number, number] {
	const scale = (piece as { readonly scale?: number }).scale;
	const s = typeof scale === "number" && Number.isFinite(scale) ? scale : 1;
	return [s, s, s];
}

/** @emoji 🧭 Maps kit wires node ids to sketchpad routes. */
export function sketchpadPathFromWiresNodeId(kitId: string, nodeId: string, fileNodeKindId: string): string | null {
	switch (fileNodeKindId) {
		case "type":
			return `/kits/${kitId}/type/${nodeId}`;
		case "design":
			return `/kits/${kitId}/design/${nodeId}`;
		case "folder":
			return `/kits/${kitId}?folder=${encodeURIComponent(nodeId)}`;
		case "file":
			return `/kits/${kitId}?file=${encodeURIComponent(nodeId)}`;
		default:
			return null;
	}
}

/** @emoji 🧭 Navigates from the first recognized kit wires selection entry. */
export function sketchpadNavigateFromWiresSelection(
	instanceId: string,
	puzzle2dIds: readonly string[],
	controller?: SketchpadShellController,
): void {
	const { kitId, pane } = parseSketchpadPuzzleInstanceId(instanceId);
	if (!kitId || pane !== "kit-wires") return;
	const ctrl = controller ?? getSketchpadShellController();
	if (!ctrl) return;
	const meta = ctrl.kitWiresNodeMetaForKit(kitId);
	for (const nodeId of puzzle2dIds) {
		const kind = meta.get(nodeId)?.fileNodeKindId;
		if (!kind) continue;
		const path = sketchpadPathFromWiresNodeId(kitId, nodeId, kind);
		if (path) {
			ctrl.navigateTo(path);
			return;
		}
	}
}

/** @emoji 🎯 Applies FiveD puzzle2d/volume selection (kit navigation or design piece/connection selection). */
export function sketchpadApplyPuzzle2dSelection(
	instanceId: string,
	puzzle2dIds: readonly string[],
	controller?: SketchpadShellController,
): void {
	const scope = parseSketchpadPuzzleInstanceId(instanceId);
	const ctrl = controller ?? getSketchpadShellController();
	if (!ctrl || !scope.kitId) return;
	if (scope.pane === "kit-wires") {
		ctrl.setKitWiresPreselect(SKETCHPAD_KIT_WIRES_PRESELECT_EMPTY);
		ctrl.setRouteSelection({ ...ctrl.routeSelection, kitWiresNodeIds: [...puzzle2dIds] });
		return;
	}
	if (scope.pane === "diagram" || scope.pane === "scene") {
		const kit = ctrl.getKitStore(scope.kitId)?.getSnapshot().kit;
		const design = scope.designId && kit ? findDesignInKit(kit, scope.designId) : undefined;
		const pieceIdSet = new Set((design?.pieces ?? []).map((piece) => piece.id).filter((id): id is string => Boolean(id)));
		const connectionIdSet = new Set(
			(((design as { connections?: readonly SketchpadKitConnection[] } | undefined)?.connections ?? []) as readonly SketchpadKitConnection[])
				.map((connection) => connection.id)
				.filter((id): id is string => Boolean(id)),
		);
		const pieceIds: string[] = [];
		const connectionIds: string[] = [];
		for (const id of puzzle2dIds) {
			if (!id || id.includes(":")) continue;
			if (id.includes("compose.connection") || id.startsWith("connection:") || connectionIdSet.has(id)) {
				connectionIds.push(id);
			} else if (pieceIdSet.has(id) || !design) {
				pieceIds.push(id);
			}
		}
		ctrl.setRouteSelection({ pieceIds, connectionIds, kitWiresNodeIds: [] });
	}
}

/** @emoji 🔍 Parses sketchpad CAD {@link CadModel.instanceId}. */
export function parseSketchpadCadInstanceId(instanceId: string): { readonly kitId: string | null; readonly typeId: string | null } {
	const parts = instanceId.split(":");
	if (parts.length === 2) return { kitId: parts[0] ?? null, typeId: parts[1] ?? null };
	return { kitId: null, typeId: null };
}

/** @emoji 🗺️ Builds a flat design diagram from design pieces and connections. */
export function sketchpadDesignPuzzle2dFixtureFromDesign(design: Design, kit?: Kit): SketchpadPuzzle2dFixtureV1 {
	const pieces = design.pieces ?? [];
	const connections = ((design as { connections?: readonly SketchpadKitConnection[] }).connections ?? []) as readonly SketchpadKitConnection[];
	const edges = connections
		.map((connection) => {
			const { sourcePieceId, targetPieceId, sourceConnectorId, targetConnectorId } = sketchpadConnectionEndpoints(connection);
			if (!sourcePieceId || !targetPieceId || !sourceConnectorId || !targetConnectorId) return null;
			return {
				id: connection.id ?? `${sourcePieceId}-${targetPieceId}`,
				source: sketchpadTopologyAnchorFullId(sourcePieceId, sourceConnectorId),
				target: sketchpadTopologyAnchorFullId(targetPieceId, targetConnectorId),
				edgeKind: "compose.connection",
				...sketchpadConnectionTransformParamsFromDto(connection as Record<string, unknown>),
			};
		})
		.filter((edge): edge is NonNullable<typeof edge> => edge !== null);
	return {
		schema: "puzzle.2d.fixture/v1",
		camera: sketchpadFlatCameraFromPartCenters([{ x: 0, y: 0 }]),
		nodes: pieces.map((piece) => {
			const connectors = sketchpadPieceConnectorGeometry(piece, kit);
			const storedCenter = (piece as { position?: { center?: { u?: number; v?: number } } }).position?.center;
			return {
				id: piece.id,
				shape: "circle",
				radius: 20,
				x: typeof storedCenter?.u === "number" ? storedCenter.u : 0,
				y: typeof storedCenter?.v === "number" ? storedCenter.v : 0,
				text: sketchpadPieceLabel(piece, kit),
				nodeKind: "compose.design.piece",
				handles: connectors.map((connector) => ({
					id: sketchpadTopologyAnchorFullId(piece.id, connector.id),
					handleKind: "compose.connector",
					angle: connector.t * 2 * Math.PI,
					t: connector.t,
				})),
			};
		}),
		edges,
	};
}

/** @emoji 🌐 Builds a 3D design scene volume from design pieces (placeholder meshes until file URLs are wired). */
export function sketchpadDesignVolumeFixtureFromDesign(design: Design, kit?: Kit): SketchpadVolumeFixtureV1 {
	const pieces = design.pieces ?? [];
	const connections = ((design as { connections?: readonly SketchpadKitConnection[] }).connections ?? []) as readonly SketchpadKitConnection[];
	const fileUrls = kit ? sketchpadKitFileUrlById(kit) : new Map<string, string>();
	const objects = pieces.map((piece) => {
		const connectors = sketchpadPieceConnectorGeometry(piece, kit);
		const storedPlane = (piece as { position?: SketchpadPiecePose["plane"] }).position?.plane;
		const origin = storedPlane?.origin;
		const storedOrigin: [number, number, number] = [
			typeof origin?.x === "number" ? origin.x : 0,
			typeof origin?.y === "number" ? origin.y : 0,
			typeof origin?.z === "number" ? origin.z : 0,
		];
		const orientation = storedPlane ? sketchpadPieceSceneOrientation(piece) : ([0, 0, 0, 1] as [number, number, number, number]);
		return {
			id: piece.id,
			objectKind: "compose.design.piece",
			meshUrl: kit ? sketchpadResolvePieceMeshUrl(piece, kit, fileUrls) : SKETCHPAD_PLACEHOLDER_MESH_URL,
			origin: storedOrigin,
			orientation,
			scale: sketchpadPieceSceneScale(piece),
			label: sketchpadPieceLabel(piece, kit),
			vortices: connectors.map((connector) => ({
				id: sketchpadTopologyAnchorFullId(piece.id, connector.id),
				position: connector.point,
				direction: connector.direction,
				vortexKind: "compose.connector",
			})),
		};
	});
	const attractions = connections
		.map((connection) => {
			const { sourcePieceId, targetPieceId, sourceConnectorId, targetConnectorId } = sketchpadConnectionEndpoints(connection);
			if (!sourcePieceId || !targetPieceId || !sourceConnectorId || !targetConnectorId) return null;
			return {
				id: connection.id ?? `${sourcePieceId}-${targetPieceId}`,
				attracting: sketchpadTopologyAnchorFullId(sourcePieceId, sourceConnectorId),
				attracted: sketchpadTopologyAnchorFullId(targetPieceId, targetConnectorId),
				attractionKind: "compose.connection",
				...sketchpadConnectionTransformParamsFromDto(connection as Record<string, unknown>),
			};
		})
		.filter((attraction): attraction is NonNullable<typeof attraction> => attraction !== null);
	const camera = sketchpadSceneCameraFromDesign(design);
	return {
		schema: "puzzle.3d.fixture/v1",
		domain: "architecture",
		camera,
		objects,
		attractions,
	};
}

function sketchpadSceneCameraFromDesign(design: Design): SketchpadVolumeFixtureV1["camera"] {
	return { position: [8, 8, 8], target: [0, 0, 0], zoom: 1 };
}


function sketchpadTopologyPayloadForDesignScene(design: Design, kit?: Kit): PlatformTopologyPayload {
	return sketchpadTopologyPayload(
		sketchpadDesignPuzzle2dFixtureFromDesign(design, kit),
		sketchpadDesignVolumeFixtureFromDesign(design, kit),
	);
}

function sketchpadTopologyPayloadForDesignDiagram(design: Design, kit?: Kit): PlatformTopologyPayload {
	return sketchpadTopologyPayload(sketchpadDesignPuzzle2dFixtureFromDesign(design, kit), sketchpadEmptyVolumeFixture());
}

/** @emoji 🌐 Builds a single-mesh 3D volume for one type representation. */
export function sketchpadTypeVolumeFixtureForRepresentation(
	type: Type,
	representation: SketchpadTypeRepresentationRef,
	kit: Kit,
): SketchpadVolumeFixtureV1 {
	const fileUrls = sketchpadKitFileUrlById(kit);
	return {
		schema: "puzzle.3d.fixture/v1",
		domain: "architecture",
		camera: { position: [4, 4, 4], target: [0, 0, 0], zoom: 1 },
		objects: [
			{
				id: representation.id,
				objectKind: "compose.representation",
				meshUrl: sketchpadResolveRepresentationMeshUrl(representation, kit, fileUrls),
				origin: [0, 0, 0] as [number, number, number],
				orientation: [0, 0, 0, 1] as [number, number, number, number],
				scale: [1, 1, 1] as [number, number, number, number],
				label: representation.name,
				vortices: [],
			},
		],
		attractions: [],
	};
}

/** @emoji 🌐 Builds a single-mesh 3D volume for a kit kind (primary representation). */
export function sketchpadTypeVolumeFixtureFromType(type: Type, kit: Kit): SketchpadVolumeFixtureV1 {
	const reps = sketchpadListTypeRepresentations(type);
	if (reps.length === 0) {
		return {
			schema: "puzzle.3d.fixture/v1",
			domain: "architecture",
			camera: { position: [4, 4, 4], target: [0, 0, 0], zoom: 1 },
			objects: [],
			attractions: [],
		};
	}
	return sketchpadTypeVolumeFixtureForRepresentation(type, reps[0]!, kit);
}

function sketchpadTopologyPayloadForTypeRepresentation(type: Type, representation: SketchpadTypeRepresentationRef, kit: Kit): PlatformTopologyPayload {
	return sketchpadTopologyPayload(sketchpadEmptyPuzzle2dFixture(), sketchpadTypeVolumeFixtureForRepresentation(type, representation, kit));
}

function sketchpadTopologyPayloadForTypeScene(type: Type, kit: Kit): PlatformTopologyPayload {
	const reps = sketchpadListTypeRepresentations(type);
	if (reps.length === 0) {
		return sketchpadTopologyPayload(sketchpadEmptyPuzzle2dFixture(), sketchpadTypeVolumeFixtureFromType(type, kit));
	}
	return sketchpadTopologyPayloadForTypeRepresentation(type, reps[0]!, kit);
}

function sketchpadEmptyPuzzle2dFixture(): SketchpadPuzzle2dFixtureV1 {
	return { schema: "puzzle.2d.fixture/v1", camera: { x: 0, y: 0, zoom: 1 }, nodes: [], edges: [] };
}
//#endregion 🔖Topology

//#region 📁SketchpadVfs
const SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_SCHEMA_MODEL: VirtualFileSystemSchemaModel = {
	descriptorKinds: {
		text: { id: "text", name: "Text", presentation: "text" },
		time: { id: "time", name: "Time", presentation: "time", format: "datetime" },
		avatar: { id: "avatar", name: "Avatar", presentation: "avatar" },
	},
	fileNodeKinds: {
		kit: {
			id: "kit",
			name: "Kit",
			icon: "layout-grid",
			description: "Open kit workspace",
			descriptors: [
				{ id: "version", descriptorKindId: "text", label: "Version" },
				{ id: "kitKind", descriptorKindId: "text", label: "Kind" },
				{ id: "updated", descriptorKindId: "time", label: "Updated" },
				{ id: "createdBy", descriptorKindId: "avatar", label: "Created by" },
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		folder: {
			id: "folder",
			name: "Folder",
			icon: "folder",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		file: {
			id: "file",
			name: "File",
			icon: "file",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		design: {
			id: "design",
			name: "Design",
			icon: "layout",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		type: {
			id: "type",
			name: "Type",
			icon: "component",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		family: {
			id: "family",
			name: "Family",
			icon: "users",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		typology: {
			id: "typology",
			name: "Typology",
			icon: "landmark",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		piece: {
			id: "piece",
			name: "Piece",
			icon: "puzzle",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		connection: {
			id: "connection",
			name: "Connection",
			icon: "link",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		representation: {
			id: "representation",
			name: "Representation",
			icon: "box",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		port: {
			id: "port",
			name: "Port",
			icon: "circle-dot",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		connector: {
			id: "connector",
			name: "Connector",
			icon: "plug",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
	},
	descriptorColumnIds: [],
};

const SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_HOME_SCHEMA_MODEL: VirtualFileSystemSchemaModel = {
	...SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_SCHEMA_MODEL,
	descriptorColumnIds: [],
};

const SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_TREE_SCHEMA_MODEL: VirtualFileSystemSchemaModel = {
	...SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_SCHEMA_MODEL,
	descriptorColumnIds: [],
};

function sketchpadKitVirtualFileSystemDescriptorValues(
	fileNodeKindId: string,
	options: {
		readonly path?: string;
		readonly version?: string;
		readonly kitKind?: string;
		readonly updatedIso?: string;
		readonly createdBy?: { readonly name: string; readonly icon?: string };
		readonly extra?: Readonly<Record<string, VirtualFileSystemDescriptorValueModel>>;
	} = {},
): Readonly<Record<string, VirtualFileSystemDescriptorValueModel>> {
	const textByDescriptorId: Record<string, string> = {};
	if (options.version !== undefined) textByDescriptorId.version = options.version;
	if (options.kitKind !== undefined) textByDescriptorId.kitKind = options.kitKind;
	return virtualFileSystemDescriptorValues(SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_SCHEMA_MODEL, fileNodeKindId, {
		path: options.path,
		updatedIso: options.updatedIso,
		createdBy: options.createdBy,
		extra: options.extra,
		...(Object.keys(textByDescriptorId).length ? { textByDescriptorId } : {}),
	});
}

function sketchpadVfsScope(appId: string): VirtualFileSystemScope {
	return { appId, surfaceId: virtualFileSystemSurfaceId(appId) };
}

const SKETCHPAD_RS_FILE_NODE_KIND_TO_VFS: Readonly<Record<string, string>> = {
	KIT: "kit",
	FOLDER: "folder",
	FILE: "file",
	DESIGN: "design",
	TYPE: "type",
	FAMILY: "family",
	TYPOLOGY: "typology",
	PIECE: "piece",
	CONNECTION: "connection",
	REPRESENTATION: "representation",
	PORT: "port",
	CONNECTOR: "connector",
};

function sketchpadVfsFileNodeKindId(rsKind: string): string {
	return SKETCHPAD_RS_FILE_NODE_KIND_TO_VFS[rsKind] ?? "file";
}

function sketchpadRsVfsParentRef(
	parentId: string,
	root: VirtualFileSystemNodeRecord,
	route: ReturnType<typeof parseSketchpadRouteScopeFromPath>,
	vfsNodeMeta: ReadonlyMap<string, { readonly fileNodeKindId: string; readonly typeId?: string; readonly designId?: string }>,
): ComposeFileSystemParentRef {
	if (parentId === root.id) {
		return { kind: "KIT", id: String(route.kitId ?? parentId) };
	}
	const meta = vfsNodeMeta.get(parentId);
	const vfsKind = meta?.fileNodeKindId ?? root.fileNodeKindId;
	switch (vfsKind) {
		case "folder":
			return { kind: "FOLDER", id: parentId };
		case "file":
			return { kind: "FILE", id: parentId };
		case "design":
			return { kind: "DESIGN", id: parentId };
		case "type":
			return { kind: "TYPE", id: parentId };
		case "family":
			return { kind: "FAMILY", id: parentId };
		case "typology":
			return { kind: "TYPOLOGY", id: parentId };
		case "piece":
			return { kind: "PIECE", id: parentId, designId: meta?.designId ?? route.designId ?? "" };
		case "connection":
			return { kind: "CONNECTION", id: parentId, designId: meta?.designId ?? route.designId ?? "" };
		default:
			return { kind: "KIT", id: String(route.kitId ?? parentId) };
	}
}

function sketchpadVfsNavigateUri(
	kitId: string,
	route: ReturnType<typeof parseSketchpadRouteScopeFromPath>,
	fileNodeKindId: string,
	nodeId: string,
	child: ComposeFileSystemChildRef,
): string | undefined {
	switch (fileNodeKindId) {
		case "type":
			return `/kits/${kitId}/type/${nodeId}`;
		case "design":
			return `/kits/${kitId}/design/${nodeId}`;
		case "folder":
			return `/kits/${kitId}?folder=${encodeURIComponent(nodeId)}`;
		case "file":
			return `/kits/${kitId}?file=${encodeURIComponent(nodeId)}`;
		case "representation": {
			const typeId = child.typeId ?? route.typeId;
			if (!typeId) return undefined;
			return `/kits/${kitId}/type/${typeId}?rep=${encodeURIComponent(nodeId)}`;
		}
		case "piece":
		case "connection": {
			const designId = child.designId ?? route.designId;
			if (!designId) return undefined;
			const param = fileNodeKindId === "piece" ? "piece" : "conn";
			return `/kits/${kitId}/design/${designId}?${param}=${encodeURIComponent(nodeId)}`;
		}
		default:
			return undefined;
	}
}

function sketchpadVfsRecordsFromRsChildren(
	kitId: string,
	route: ReturnType<typeof parseSketchpadRouteScopeFromPath>,
	parentId: string,
	children: readonly ComposeFileSystemChildRef[],
): readonly VirtualFileSystemNodeRecord[] {
	return children.map((child) => {
		const fileNodeKindId = sketchpadVfsFileNodeKindId(child.kind);
		const path = child.path || `/${child.name || child.id}`;
		const fileBasename = child.name || child.id;
		const vfsFile = fileNodeKindId === "file" ? sketchpadKitVfsFileRowFields({ name: fileBasename }) : undefined;
		return {
			id: child.id,
			fileNodeKindId,
			name: vfsFile?.name ?? (child.name || child.id),
			...(vfsFile ? { icon: vfsFile.icon } : {}),
			path,
			parentId,
			hasChildren: child.hasChildren,
			navigateUri: sketchpadVfsNavigateUri(kitId, route, fileNodeKindId, child.id, child),
			descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues(fileNodeKindId, { path }),
		} satisfies VirtualFileSystemNodeRecord;
	});
}

function sketchpadKitTypologyRows(kit: Kit): readonly { readonly id: string; readonly name: string; readonly types: readonly unknown[]; readonly designs: readonly unknown[] }[] {
	const block = (kit as { typologies?: unknown }).typologies;
	const items = Array.isArray(block)
		? block
		: block && typeof block === "object" && Array.isArray((block as { items?: unknown[] }).items)
			? (block as { items: unknown[] }).items
			: [];
	return items
		.map((topo) => {
			if (typeof topo !== "object" || topo === null) return null;
			const row = topo as { id?: string; name?: string; types?: unknown; designs?: unknown };
			const id = String(row.id ?? "");
			if (!id) return null;
			const typesBlock = row.types;
			const designsBlock = row.designs;
			const types = Array.isArray(typesBlock)
				? typesBlock
				: typesBlock && typeof typesBlock === "object" && Array.isArray((typesBlock as { items?: unknown[] }).items)
					? (typesBlock as { items: unknown[] }).items
					: [];
			const designs = Array.isArray(designsBlock)
				? designsBlock
				: designsBlock && typeof designsBlock === "object" && Array.isArray((designsBlock as { items?: unknown[] }).items)
					? (designsBlock as { items: unknown[] }).items
					: [];
			return { id, name: String(row.name ?? id), types, designs };
		})
		.filter((row): row is NonNullable<typeof row> => row !== null);
}

function sketchpadKitVfsChildren(kit: Kit, parentId: string): readonly VirtualFileSystemNodeRecord[] {
	const kitId = String(kit.id ?? "");
	const typologies = sketchpadKitTypologyRows(kit);
	const kitFolders = sketchpadKitItemsOf<unknown>(kit.folders);
	const kitFiles = sketchpadKitItemsOf<unknown>(kit.files);
	const representationFileIds = sketchpadKitRepresentationFileIds(kit);
	if (parentId === kitId) {
		const rows: VirtualFileSystemNodeRecord[] = [];
		for (const topo of typologies) {
			const topoPath = `/${topo.name}`;
			rows.push({
				id: topo.id,
				fileNodeKindId: "typology",
				name: topo.name,
				path: topoPath,
				parentId: kitId,
				hasChildren: topo.types.length > 0 || topo.designs.length > 0,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("typology", { path: topoPath }),
			});
		}
		if (typologies.length > 0) {
			for (const folder of kitFolders) {
				const row = folder as Record<string, unknown>;
				if (!sketchpadKitFolderAtRoot(row)) continue;
				sketchpadKitVfsPushFolderRow(rows, kitId, kitId, row);
			}
			for (const file of kitFiles) {
				const row = file as Record<string, unknown>;
				if (!sketchpadKitFileAtKitRoot(row, representationFileIds)) continue;
				sketchpadKitVfsPushFileRow(rows, kitId, kitId, row);
			}
			return rows;
		}
		for (const folder of kitFolders) {
			const row = folder as Record<string, unknown>;
			if (!sketchpadKitFolderAtRoot(row)) continue;
			sketchpadKitVfsPushFolderRow(rows, kitId, kitId, row);
		}
		for (const type of kit.types ?? []) {
			if (typeof type !== "object" || type === null || !("id" in type)) continue;
			const t = type as Type;
			const typePath = `/${t.name ?? t.id}`;
			const typeHasChildren =
				(t.representations?.length ?? 0) > 0 || (t.ports?.length ?? 0) > 0 || (t.connectors?.length ?? 0) > 0;
			const typeIcon = sketchpadKitEntityAvailableIcon(kit, String(t.id), "type");
			rows.push({
				id: String(t.id),
				fileNodeKindId: "type",
				name: t.name ?? t.id,
				...(typeIcon ? { icon: typeIcon } : {}),
				path: typePath,
				parentId: kitId,
				hasChildren: typeHasChildren,
				navigateUri: `/kits/${kitId}/type/${t.id}`,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("type", { path: typePath }),
			});
		}
		for (const design of kit.designs ?? []) {
			if (typeof design !== "object" || design === null || !("id" in design)) continue;
			const d = design as Design;
			const designPath = `/${d.name ?? d.id}`;
			const designIcon = sketchpadKitEntityAvailableIcon(kit, String(d.id), "design");
			rows.push({
				id: String(d.id),
				fileNodeKindId: "design",
				name: d.name ?? d.id,
				...(designIcon ? { icon: designIcon } : {}),
				path: designPath,
				parentId: kitId,
				hasChildren: true,
				navigateUri: `/kits/${kitId}/design/${d.id}`,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("design", { path: designPath }),
			});
		}
		for (const file of kitFiles) {
			const row = file as Record<string, unknown>;
			if (!sketchpadKitFileAtKitRoot(row, representationFileIds)) continue;
			sketchpadKitVfsPushFileRow(rows, kitId, kitId, row);
		}
		return rows;
	}
	const typology = typologies.find((topo) => topo.id === parentId);
	if (typology) {
		const rows: VirtualFileSystemNodeRecord[] = [];
		for (const type of typology.types) {
			if (typeof type !== "object" || type === null || !("id" in type)) continue;
			const t = type as Type;
			const typePath = `/${typology.name}/${t.name ?? t.id}`;
			const typeHasChildren =
				(t.representations?.length ?? 0) > 0 || (t.ports?.length ?? 0) > 0 || (t.connectors?.length ?? 0) > 0;
			const typeIcon = sketchpadKitEntityAvailableIcon(kit, String(t.id), "type");
			rows.push({
				id: String(t.id),
				fileNodeKindId: "type",
				name: t.name ?? t.id,
				...(typeIcon ? { icon: typeIcon } : {}),
				path: typePath,
				parentId,
				hasChildren: typeHasChildren,
				navigateUri: `/kits/${kitId}/type/${t.id}`,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("type", { path: typePath }),
			});
		}
		for (const design of typology.designs) {
			if (typeof design !== "object" || design === null || !("id" in design)) continue;
			const d = design as Design;
			const designPath = `/${typology.name}/${d.name ?? d.id}`;
			const designIcon = sketchpadKitEntityAvailableIcon(kit, String(d.id), "design");
			rows.push({
				id: String(d.id),
				fileNodeKindId: "design",
				name: d.name ?? d.id,
				...(designIcon ? { icon: designIcon } : {}),
				path: designPath,
				parentId,
				hasChildren: true,
				navigateUri: `/kits/${kitId}/design/${d.id}`,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("design", { path: designPath }),
			});
		}
		return rows;
	}
	const design = findDesignInKit(kit, parentId);
	if (design) {
		const out: VirtualFileSystemNodeRecord[] = [];
		for (const piece of design.pieces ?? []) {
			if (typeof piece !== "object" || piece === null || !("id" in piece)) continue;
			const p = piece as { id: string; name?: string };
			const piecePath = `/${design.name ?? design.id}/${p.name ?? p.id}`;
			out.push({
				id: String(p.id),
				fileNodeKindId: "piece",
				name: p.name ?? p.id,
				path: piecePath,
				parentId,
				hasChildren: false,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("piece", { path: piecePath }),
			});
		}
		for (const connection of (design as { connections?: readonly unknown[] }).connections ?? []) {
			const c = connection as { id: string; description?: string };
			if (!c.id) continue;
			const connectionPath = `/${design.name ?? design.id}/${c.description ?? c.id}`;
			out.push({
				id: c.id,
				fileNodeKindId: "connection",
				name: c.description ?? c.id,
				path: connectionPath,
				parentId,
				hasChildren: false,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("connection", { path: connectionPath }),
			});
		}
		return out;
	}
	for (const folder of kitFolders) {
		const row = folder as Record<string, unknown>;
		const folderId = String(row["id"] ?? "");
		if (folderId !== parentId) continue;
		const out: VirtualFileSystemNodeRecord[] = [];
		for (const sub of kitFolders) {
			const subRow = sub as Record<string, unknown>;
			if (sketchpadKitFolderParentId(subRow) !== folderId) continue;
			sketchpadKitVfsPushFolderRow(out, kitId, folderId, subRow);
		}
		for (const file of kitFiles) {
			const fileRow = file as Record<string, unknown>;
			if (sketchpadKitFileFolderId(fileRow) !== folderId) continue;
			sketchpadKitVfsPushFileRow(out, kitId, folderId, fileRow);
		}
		return out;
	}
	return [];
}

function sketchpadHomeVfsChildren(
	openKitIds: readonly string[],
	kitById: (kitId: string) => Kit | undefined,
	kitKind: (kitId: string) => string,
	home: SketchpadHomeUiState,
	parentId: string,
): readonly VirtualFileSystemNodeRecord[] {
	if (parentId === "sketchpad-home") {
		const kitEntries: { kitId: string; kit: Kit; kind: string }[] = [];
		for (const kitId of openKitIds) {
			const kit = kitById(kitId);
			if (!kit) continue;
			const kind = kitKind(kitId) || "temporary";
			if (home.kindFilter && home.kindFilter !== kind) continue;
			const name = kit.name ?? kitId;
			if (home.searchQuery && !name.toLowerCase().includes(home.searchQuery.toLowerCase())) continue;
			if (home.nameFilter && name !== home.nameFilter) continue;
			const version = kit.version ?? "";
			if (home.versionFilter && version !== home.versionFilter) continue;
			kitEntries.push({ kitId, kit, kind });
		}
		kitEntries.sort((left, right) => {
			const column = home.sortColumnId;
			if (!column) return (left.kit.name ?? left.kitId).localeCompare(right.kit.name ?? right.kitId);
			let comparison = 0;
			switch (column) {
				case "name":
					comparison = (left.kit.name ?? left.kitId).localeCompare(right.kit.name ?? right.kitId);
					break;
				case "version":
					comparison = (left.kit.version ?? "").localeCompare(right.kit.version ?? "");
					break;
				case "kind":
					comparison = left.kind.localeCompare(right.kind);
					break;
				case "updated":
					comparison = sketchpadFormatKitTimestamp(left.kit.updatedAt ?? left.kit.createdAt).localeCompare(
						sketchpadFormatKitTimestamp(right.kit.updatedAt ?? right.kit.createdAt),
					);
					break;
				default:
					comparison = (left.kit.name ?? left.kitId).localeCompare(right.kit.name ?? right.kitId);
			}
			return home.sortDescending ? -comparison : comparison;
		});
		return [
			{
				id: "docs-root",
				fileNodeKindId: "folder",
				name: "Documentation",
				path: "/Documentation",
				parentId,
				hasChildren: true,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("folder", { path: "/Documentation" }),
			},
			...kitEntries.map(({ kitId, kit, kind }) => {
				const name = kit.name ?? kitId;
				const path = `/kits/${name}`;
				const author = kit.authors?.[0];
				const authorName = author && typeof author === "object" && "name" in author ? String(author.name ?? "") : "";
				return {
					id: `kit:${kitId}`,
					fileNodeKindId: "kit",
					name,
					path,
					parentId,
					hasChildren: false,
					canDrag: false,
					navigateUri: `/kits/${kitId}`,
					descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("kit", {
						path,
						version: kit.version ?? "",
						kitKind: kind,
						updatedIso: sketchpadKitTimestampIso(kit.updatedAt ?? kit.createdAt),
						...(authorName ? { createdBy: { name: authorName } } : {}),
					}),
				} satisfies VirtualFileSystemNodeRecord;
			}),
		];
	}
	if (parentId === "docs-root") {
		return sketchpadBuildDocsRegistry().map((section) => {
			const path = `/Documentation/${section.label}`;
			return {
				id: `docs-section-${section.id}`,
				fileNodeKindId: "folder",
				name: section.label,
				path,
				parentId,
				hasChildren: section.pages.length > 0,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("folder", { path }),
			};
		});
	}
	const sectionMatch = /^docs-section-(.+)$/.exec(parentId);
	if (sectionMatch) {
		const section = sketchpadBuildDocsRegistry().find((entry) => entry.id === sectionMatch[1]);
		if (!section) return [];
		return section.pages.map((page) => {
			const path = `/Documentation/${page.title}`;
			return {
				id: `docs-page-${page.path}`,
				fileNodeKindId: "file",
				name: page.title,
				path,
				parentId,
				hasChildren: false,
				navigateUri: `/doc/${page.path}`,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("file", { path }),
			};
		});
	}
	return [];
}
//#endregion 📁SketchpadVfs

export const SKETCHPAD_SHELL_CONTROLLER_ID = "compose.sketchpad.shell";
const SKETCHPAD_EXTENSION_ID = "compose.sketchpad.builtin";
export const SKETCHPAD_HOME_APP_ID = "home";
export const SKETCHPAD_KIT_APP_ID = "kit";
export const SKETCHPAD_DESIGN_APP_ID = "design";
export const SKETCHPAD_TYPE_APP_ID = "type";
export const SKETCHPAD_DOCS_APP_ID = "docs";
export const SKETCHPAD_FEEDBACK_APP_ID = "feedback";

/** @emoji 🏠 Home virtual file system surface id. */
export const SKETCHPAD_SURFACE_HOME_VFS = virtualFileSystemSurfaceId(SKETCHPAD_HOME_APP_ID);

const SKETCHPAD_BODY_HOME = "compose.sketchpad.window.home";
const SKETCHPAD_BODY_KIT_VFS = "compose.sketchpad.window.kit.vfs";
const SKETCHPAD_BODY_KIT_WIRES = "compose.sketchpad.window.kit.wires";
const SKETCHPAD_BODY_DESIGN_SCENE = "compose.sketchpad.window.design.scene";
const SKETCHPAD_BODY_DESIGN_DIAGRAM = "compose.sketchpad.window.design.diagram";
const SKETCHPAD_BODY_TYPE = "compose.sketchpad.window.type";
const SKETCHPAD_BODY_TYPE_REP = "compose.sketchpad.window.type.representation";
const SKETCHPAD_BODY_DOCS = "compose.sketchpad.window.docs";
const SKETCHPAD_BODY_FEEDBACK = "compose.sketchpad.window.feedback";
const SKETCHPAD_SURFACE_KIT_VFS = virtualFileSystemSurfaceId(SKETCHPAD_KIT_APP_ID);
const SKETCHPAD_SURFACE_DESIGN_VFS = virtualFileSystemSurfaceId(SKETCHPAD_DESIGN_APP_ID);
const SKETCHPAD_SURFACE_KIT_WIRES = "compose.sketchpad.surface.kit.wires/v1";
const SKETCHPAD_SURFACE_DESIGN_SCENE = "compose.sketchpad.surface.design.scene/v1";
const SKETCHPAD_SURFACE_DESIGN_DIAGRAM = "compose.sketchpad.surface.design.diagram/v1";
const SKETCHPAD_SURFACE_TYPE_SCENE = "compose.sketchpad.surface.type.scene/v1";
export const SKETCHPAD_SURFACE_DOCS_PAGE = "compose.sketchpad.surface.docs.page/v1";
export const SKETCHPAD_SURFACE_FEEDBACK_FORM = "compose.sketchpad.surface.feedback.form/v1";
const SKETCHPAD_PANEL_WINDOWS_BODY = "compose.sketchpad.panel.windows";
const SKETCHPAD_PANEL_HIERARCHY_BODY = "compose.sketchpad.panel.hierarchy";
const SKETCHPAD_PANEL_CATALOGUE_BODY = "compose.sketchpad.panel.catalogue";
const SKETCHPAD_PANEL_INSPECTION_BODY = "compose.sketchpad.panel.inspection";

const sketchpadRegisteredTypeRepSurfaces = new Set<string>();

function sketchpadWindowKindIdForRepresentation(representationId: string): string {
	return `rep-${representationId}`;
}

function sketchpadUnregisterTypeRepresentationComponents(platform: Platform): void {
	for (const surfaceId of sketchpadRegisteredTypeRepSurfaces) {
		platform.unregisterComponent(surfaceId);
	}
	sketchpadRegisteredTypeRepSurfaces.clear();
}

function sketchpadSyncTypeRepresentationComponents(platform: Platform, kitId: string, typeId: string): void {
	sketchpadUnregisterTypeRepresentationComponents(platform);
	const kit = getSketchpadShellController()?.getKitStore(kitId)?.getSnapshot().kit;
	const type = kit ? findTypeInKit(kit, typeId) : undefined;
	if (!type) return;
	for (const representation of sketchpadListTypeRepresentations(type)) {
		const surfaceId = sketchpadTypeRepresentationSurfaceId(kitId, typeId, representation.id);
		const component = new SketchpadTypeRepresentationScene(platform, surfaceId, representation.id);
		registerPlatformComponent(platform, component);
		sketchpadRegisteredTypeRepSurfaces.add(surfaceId);
		component.refresh();
		getSketchpadShellController()?.syncTopologyForSurface(surfaceId, { kitId, designId: null, typeId });
	}
}

/** @emoji 🪟 Rebuilds type-app window kinds (tab stack per representation) and topology components. */
export function sketchpadSyncTypeAppChrome(platform: Platform): void {
	const typeApp = platform.apps.find((app) => app.id === SKETCHPAD_TYPE_APP_ID);
	if (!typeApp) return;
	const route = parseSketchpadRouteScopeFromPath(platform.uri);
	if (!route.kitId || !route.typeId) {
		sketchpadUnregisterTypeRepresentationComponents(platform);
		typeApp.windowKinds = [new WindowKindRuntime("type-empty", "Type", SKETCHPAD_BODY_TYPE_REP)];
		typeApp.defaultLayout = createTabStackLayout(["type-empty"], ["Type"]);
		return;
	}
	const kit = getSketchpadShellController()?.getKitStore(route.kitId)?.getSnapshot().kit;
	const type = kit ? findTypeInKit(kit, route.typeId) : undefined;
	const representations = type ? sketchpadListTypeRepresentations(type) : [];
	sketchpadSyncTypeRepresentationComponents(platform, route.kitId, route.typeId);
	if (representations.length === 0) {
		typeApp.windowKinds = [new WindowKindRuntime("type-empty", "No representations", SKETCHPAD_BODY_TYPE_REP)];
		typeApp.defaultLayout = createTabStackLayout(["type-empty"], ["No representations"]);
	} else {
		const windowKindIds = representations.map((rep) => sketchpadWindowKindIdForRepresentation(rep.id));
		const labels = representations.map((rep) => rep.name);
		typeApp.windowKinds = representations.map(
			(rep, index) => new WindowKindRuntime(windowKindIds[index]!, labels[index]!, SKETCHPAD_BODY_TYPE_REP),
		);
		typeApp.defaultLayout = createTabStackLayout(windowKindIds, labels);
	}
}

//#region 🔖SketchpadPlatformComponents
abstract class SketchpadRoutedComponent<TSnapshot> extends Component<TSnapshot> {
	protected route = parseSketchpadRouteScopeFromPath("/");
	private readonly detachRoute: () => void;
	private readonly detachShellStore?: () => void;
	private detachKitStore?: () => void;

	constructor(componentKind: ComponentKind, surfaceId: string, controllerId: string, initialSnapshot: TSnapshot, platform: Platform) {
		super(componentKind, surfaceId, controllerId, initialSnapshot);
		this.route = parseSketchpadRouteScopeFromPath(platform.uri);
		let lastPlatformUri = platform.uri;
		this.detachRoute = platform.subscribe(() => {
			const nextUri = platform.uri;
			if (nextUri === lastPlatformUri) return;
			lastPlatformUri = nextUri;
			const nextRoute = parseSketchpadRouteScopeFromPath(nextUri);
			const routeStructureChanged =
				nextRoute.kitId !== this.route.kitId ||
				nextRoute.designId !== this.route.designId ||
				nextRoute.typeId !== this.route.typeId ||
				nextRoute.docsPath !== this.route.docsPath ||
				nextRoute.qualityId !== this.route.qualityId;
			this.route = nextRoute;
			if (routeStructureChanged) this.attachActiveKitStore();
			this.refresh();
		});
		const shellStore = getSketchpadShellController()?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL);
		if (shellStore) {
			this.detachShellStore = shellStore.subscribe(() => {
				this.attachActiveKitStore();
				this.refresh();
			});
		}
		this.attachActiveKitStore();
	}

	protected attachActiveKitStore(): void {
		this.detachKitStore?.();
		this.detachKitStore = undefined;
		const { kitId } = this.route;
		if (!kitId) return;
		const store = getSketchpadShellController()?.getKitStore(kitId);
		if (store) {
			this.detachKitStore = store.subscribe(() => {
				this.syncTopologyForSurface();
				this.refresh();
			});
			this.syncTopologyForSurface();
		}
	}

	/** @emoji 🔄 Pushes kit/design data into controller-owned topology stores for FiveD surfaces. */
	protected syncTopologyForSurface(): void {
		getSketchpadShellController()?.syncTopologyForSurface(this.surfaceId, this.route);
	}

	dispose(): void {
		this.detachRoute();
		this.detachShellStore?.();
		this.detachKitStore?.();
		super.dispose();
	}
}

function sketchpadSameStringIds(left: readonly string[] | undefined, right: readonly string[]): boolean {
	const a = left ?? [];
	if (a.length !== right.length) return false;
	for (let index = 0; index < a.length; index += 1) {
		if (a[index] !== right[index]) return false;
	}
	return true;
}

/** @emoji 👁️ Kit wires rectangle/lasso preview ids while area-select is in progress. */
export interface SketchpadKitWiresPreselectSnapshot {
	readonly ids: readonly string[];
	readonly removedIds: readonly string[];
}

const SKETCHPAD_KIT_WIRES_PRESELECT_EMPTY: SketchpadKitWiresPreselectSnapshot = { ids: [], removedIds: [] };

function sketchpadKitWiresPreselectSnapshotsEqual(
	left: SketchpadKitWiresPreselectSnapshot,
	right: SketchpadKitWiresPreselectSnapshot,
): boolean {
	return sketchpadSameStringIds(left.ids, [...right.ids]) && sketchpadSameStringIds(left.removedIds, [...right.removedIds]);
}

/** @emoji 🎯 Committed kit wires ids merged with an in-progress area-select preview. */
export function sketchpadKitWiresEffectiveSelectionIds(
	committed: readonly string[],
	preselect: SketchpadKitWiresPreselectSnapshot,
): string[] {
	if (!preselect.ids.length && !preselect.removedIds.length) return [...committed];
	const out = new Set(committed);
	for (const id of preselect.ids) out.add(id);
	for (const id of preselect.removedIds) out.delete(id);
	return [...out];
}

/** @emoji 📁 Per-app virtual file system surface backed by {@link SketchpadShellController}. */
class SketchpadAppVirtualFileSystem extends SketchpadRoutedComponent<VirtualFileSystemModel> {
	private readonly detachKitWiresHover?: () => void;
	private readonly detachKitWiresSelection?: () => void;
	private readonly detachKitWiresPreselect?: () => void;

	constructor(
		readonly vfsAppId: string,
		platform: Platform,
	) {
		super("virtualFileSystem", virtualFileSystemSurfaceId(vfsAppId), SKETCHPAD_SHELL_CONTROLLER_ID, { rows: [] }, platform);
		if (vfsAppId !== SKETCHPAD_KIT_APP_ID) return;
		const ctrl = getSketchpadShellController();
		const shellStore = ctrl?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL);
		if (!ctrl || !shellStore) return;
		const syncKitVfsSelectionSnapshot = (): void => {
			const selectedRowIds = ctrl.kitWiresEffectiveSelectedRowIds();
			const snap = this.getSnapshot();
			if (sketchpadSameStringIds(snap.selectedRowIds, selectedRowIds)) return;
			this.setSnapshot({ ...snap, selectedRowIds });
		};
		this.detachKitWiresHover = ctrl.kitWiresHoverStore.subscribe(() => {
			const hoveredRowId = ctrl.kitWiresHoverStore.getSnapshot();
			const snap = this.getSnapshot();
			if (snap.hoveredRowId === hoveredRowId) return;
			this.setSnapshot({ ...snap, hoveredRowId });
		});
		this.detachKitWiresSelection = shellStore.subscribe(syncKitVfsSelectionSnapshot);
		this.detachKitWiresPreselect = ctrl.kitWiresPreselectStore.subscribe(syncKitVfsSelectionSnapshot);
	}

	override dispose(): void {
		this.detachKitWiresHover?.();
		this.detachKitWiresSelection?.();
		this.detachKitWiresPreselect?.();
		super.dispose();
	}

	override buildSnapshot(): VirtualFileSystemModel {
		const ctrl = getSketchpadShellController();
		if (!ctrl) {
			return { rows: [], emptyMessage: "Platform loading…" };
		}
		if (this.vfsAppId === SKETCHPAD_KIT_APP_ID && !this.route.kitId) {
			return { rows: [], emptyMessage: "Open a kit to browse the file system" };
		}
		if (this.vfsAppId === SKETCHPAD_DESIGN_APP_ID && (!this.route.kitId || !this.route.designId)) {
			return { rows: [], emptyMessage: "Open a design to browse the file system" };
		}
		if (this.vfsAppId === SKETCHPAD_HOME_APP_ID) {
			const shell = ctrl.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL)?.getSnapshot();
			ctrl.expandedStore(sketchpadVfsScope(SKETCHPAD_HOME_APP_ID)).setAll(shell?.home.expandedRowIds ?? []);
		}
		if (this.vfsAppId === SKETCHPAD_KIT_APP_ID && this.route.kitId) {
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), this.route.kitId);
		}
		if (this.vfsAppId === SKETCHPAD_DESIGN_APP_ID && this.route.designId) {
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_DESIGN_APP_ID), this.route.designId);
		}
		return ctrl.buildVirtualFileSystemModel(sketchpadVfsScope(this.vfsAppId));
	}
}

/** @emoji 🔗 Kit wires surface (FiveD flat WIRES topology). */
export class SketchpadKitWires extends SketchpadRoutedComponent<Puzzle5dModel> {
	private readonly detachKitWiresHover?: () => void;
	private readonly detachKitWiresSelection?: () => void;

	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_KIT_WIRES,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "flat", instanceId: SKETCHPAD_SURFACE_KIT_WIRES },
			platform,
		);
		const ctrl = getSketchpadShellController();
		const shellStore = ctrl?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL);
		if (!ctrl || !shellStore) return;
		this.detachKitWiresHover = ctrl.kitWiresHoverStore.subscribe(() => {
			const puzzle2dHoveredId = ctrl.kitWiresHoverStore.getSnapshot();
			const snap = this.getSnapshot();
			if (snap.puzzle2dHoveredId === puzzle2dHoveredId) return;
			this.setSnapshot({ ...snap, puzzle2dHoveredId });
		});
		this.detachKitWiresSelection = shellStore.subscribe(() => {
			const puzzle2dSelection = [...shellStore.getSnapshot().routeSelection.kitWiresNodeIds];
			const snap = this.getSnapshot();
			if (sketchpadSameStringIds(snap.puzzle2dSelection, puzzle2dSelection)) return;
			this.setSnapshot({ ...snap, puzzle2dSelection });
		});
	}

	override dispose(): void {
		this.detachKitWiresHover?.();
		this.detachKitWiresSelection?.();
		super.dispose();
	}

	override buildSnapshot(): Puzzle5dModel {
		const { kitId } = this.route;
		if (!kitId) {
			return { presentation: "flat", instanceId: SKETCHPAD_SURFACE_KIT_WIRES, emptyMessage: "Open a kit to view wires" };
		}
		const store = getSketchpadShellController()?.getKitStore(kitId);
		if (!store) {
			return { presentation: "flat", instanceId: SKETCHPAD_SURFACE_KIT_WIRES, emptyMessage: "Kit loading…" };
		}
		const ctrl = getSketchpadShellController();
		const routeSelection = ctrl?.routeSelection;
		return {
			presentation: "flat",
			instanceId: sketchpadKitWiresInstanceId(kitId),
			puzzle2dSelection: routeSelection?.kitWiresNodeIds ?? [],
			puzzle2dHoveredId: ctrl?.kitWiresHoverStore.getSnapshot() ?? null,
		};
	}
}

/** @emoji 🎬 Design scene (5D volume). */
export class SketchpadDesignScene extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_DESIGN_SCENE,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "volume", instanceId: SKETCHPAD_SURFACE_DESIGN_SCENE },
			platform,
		);
	}

	override buildSnapshot(): Puzzle5dModel {
		const { kitId, designId } = this.route;
		if (!kitId || !designId) {
			return { presentation: "volume", instanceId: SKETCHPAD_SURFACE_DESIGN_SCENE, emptyMessage: "Open a design to view the scene" };
		}
		const kit = getSketchpadShellController()?.getKitStore(kitId)?.getSnapshot().kit;
		if (!kit) {
			return { presentation: "volume", instanceId: sketchpadDesignSceneInstanceId(kitId, designId), emptyMessage: "Kit loading…" };
		}
		const design = findDesignInKit(kit, designId);
		return {
			presentation: "volume",
			instanceId: sketchpadDesignSceneInstanceId(kitId, designId),
			emptyMessage: design ? undefined : `Design ${designId} not found`,
		};
	}
}

/** @emoji 📐 Design diagram (5D flat). */
export class SketchpadDesignDiagram extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_DESIGN_DIAGRAM,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "flat", instanceId: SKETCHPAD_SURFACE_DESIGN_DIAGRAM },
			platform,
		);
	}

	override buildSnapshot(): Puzzle5dModel {
		const { kitId, designId } = this.route;
		if (!kitId || !designId) {
			return { presentation: "flat", instanceId: SKETCHPAD_SURFACE_DESIGN_DIAGRAM, emptyMessage: "Open a design to view the diagram" };
		}
		const kit = getSketchpadShellController()?.getKitStore(kitId)?.getSnapshot().kit;
		if (!kit) {
			return { presentation: "flat", instanceId: sketchpadDesignDiagramInstanceId(kitId, designId), emptyMessage: "Kit loading…" };
		}
		const row = findDesignInKit(kit, designId);
		return {
			presentation: "flat",
			instanceId: sketchpadDesignDiagramInstanceId(kitId, designId),
			emptyMessage: row ? undefined : `Design ${designId} not found`,
		};
	}
}

/** @emoji 📐 Type representation CAD scene (one mesh per representation window). */
export class SketchpadTypeRepresentationScene extends SketchpadRoutedComponent<Puzzle5dModel> {
	readonly representationId: string;

	constructor(platform: Platform, surfaceId: string, representationId: string) {
		super(
			"puzzle5d",
			surfaceId,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "volume", instanceId: surfaceId },
			platform,
		);
		this.representationId = representationId;
	}

	override buildSnapshot(): Puzzle5dModel {
		const { kitId, typeId } = this.route;
		if (!kitId || !typeId) {
			return { presentation: "volume", instanceId: this.surfaceId, emptyMessage: "Open a type to view representations" };
		}
		const kit = getSketchpadShellController()?.getKitStore(kitId)?.getSnapshot().kit;
		const type = kit ? findTypeInKit(kit, typeId) : undefined;
		const representation = type ? sketchpadListTypeRepresentations(type).find((row) => row.id === this.representationId) : undefined;
		const instanceId = sketchpadTypeRepresentationSceneInstanceId(kitId, typeId, this.representationId);
		if (!type || !representation) {
			return { presentation: "volume", instanceId, emptyMessage: `Representation ${this.representationId} not found` };
		}
		const meshUrl = sketchpadResolveRepresentationMeshUrl(representation, kit!, sketchpadKitFileUrlById(kit!));
		if (meshUrl === SKETCHPAD_PLACEHOLDER_MESH_URL) {
			return { presentation: "volume", instanceId, emptyMessage: `Mesh unavailable for ${representation.name}` };
		}
		return { presentation: "volume", instanceId, emptyMessage: undefined };
	}
}

function sketchpadShellCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: SKETCHPAD_SHELL_CONTROLLER_ID, command, args: args as never };
}

function sketchpadAllEqual<T>(values: readonly T[]): boolean {
	if (values.length <= 1) return true;
	const first = values[0];
	for (let index = 1; index < values.length; index += 1) {
		if (values[index] !== first) return false;
	}
	return true;
}

function sketchpadHierarchySelectedIds(selection: SketchpadRouteSelection): string[] {
	return [
		...selection.pieceIds.map((id) => `sketchpad.hierarchy.piece.${id}`),
		...selection.connectionIds.map((id) => `sketchpad.hierarchy.connection.${id}`),
		...selection.kitWiresNodeIds.map((id) => `sketchpad.hierarchy.kit-node.${id}`),
	];
}

/** @emoji 🌳 Workbench hierarchy for the active sketchpad route. */
function buildSketchpadHierarchyPanelBody(ctx: WindowBodyViewContext): UiTreeNode {
	const ctrl = getSketchpadShellController();
	const routeUri = ctrl?.navigationPath ?? ctx.platform.uri ?? "/";
	const pathOnly = routeUri.split("?")[0] ?? "/";
	const { kitId, designId, typeId, qualityId } = parseSketchpadRouteScopeFromPath(routeUri);
	const selection = ctrl?.routeSelection ?? sketchpadEmptyRouteSelection();
	const sections: UiSectionNode[] = [];
	if (!kitId) {
		const open = ctrl?.listOpenKitIds() ?? [];
		const shell = ctrl?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL)?.getSnapshot();
		const homeSelected = shell?.home.selectedKitIds ?? [];
		if (pathOnly.startsWith("/docs")) {
			sections.push({
				type: "section",
				id: "sketchpad.hierarchy.docs",
				label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
				children: [{ type: "text", value: sketchpadTitleFromDocPath(parseSketchpadRouteScopeFromPath(pathOnly).docsPath) }],
			});
			return uiDeclarativeSectionsToTree(sections);
		}
		const kitItems: UiTreeItemNode[] = open.map((id) => {
			const kit = ctrl?.getKitStore(id)?.getSnapshot().kit;
			return {
				id: `sketchpad.hierarchy.kit.${id}`,
				label: kit?.name ?? id,
				command: sketchpadShellCmd("navigate", { path: `/kits/${id}` }),
			};
		});
		if (homeSelected.length > 0) {
			sections.push({
				type: "section",
				id: "sketchpad.hierarchy.home-selected",
				label: "Selected kits",
				children: homeSelected.map((id) => {
					const kit = ctrl?.getKitStore(id)?.getSnapshot().kit;
					return {
						type: "button",
						id: `sketchpad.hierarchy.home-kit.${id}`,
						label: kit?.name ?? id,
						command: sketchpadShellCmd("navigate", { path: `/kits/${id}` }),
					};
				}),
			});
		}
		sections.push({
			type: "section",
			id: "sketchpad.hierarchy.open-kits",
			label: "Open kits",
			children: kitItems.length
				? kitItems.map((item) => ({ type: "button", id: item.id, label: item.label, command: item.command }))
				: [
						{ type: "text", value: `${open.length} kit(s) open` },
						sketchpadPanelCommandButton("Import kit archive…", "importKitFromFile"),
						sketchpadPanelCommandButton("Create empty kit", "createTemporaryKit", { name: "Untitled Kit" }),
					],
		});
		return { ...uiDeclarativeSectionsToTree(sections), selectedIds: sketchpadHierarchySelectedIds(selection) };
	}
	const kit = ctrl?.getKitStore(kitId)?.getSnapshot().kit;
	if (!kit) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "sketchpad.hierarchy.loading", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, children: [{ type: "text", value: "Kit loading…" }] },
		]);
	}
	const designItems: UiTreeItemNode[] = sketchpadKitItemsOf<Design>(kit.designs).map((design) => ({
		id: `sketchpad.hierarchy.design.${design.id}`,
		label: design.name ?? design.id ?? "",
		command: sketchpadShellCmd("navigate", { path: `/kits/${kitId}/design/${design.id}` }),
	}));
	const typeItems: UiTreeItemNode[] = sketchpadKitTypeRows(kit).map((type) => ({
		id: `sketchpad.hierarchy.type.${type.id}`,
		label: type.name ?? type.id ?? "",
		command: sketchpadShellCmd("navigate", { path: `/kits/${kitId}/type/${type.id}` }),
	}));
	sections.push({
		type: "section",
		id: "sketchpad.hierarchy.kit",
		label: kit.name ?? kitId,
		children: [
			{ type: "text", value: `${designItems.length} design(s) · ${typeItems.length} type(s)` },
			sketchpadPanelCommandButton("Create design", "createDesignInActiveKit", { name: "New design" }),
		],
	});
	if (designId) {
		const design = findDesignInKit(kit, designId);
		const pieceItems: UiTreeItemNode[] = (design?.pieces ?? []).map((piece) => ({
			id: `sketchpad.hierarchy.piece.${piece.id}`,
			label: sketchpadPieceLabel(piece, kit),
			command: sketchpadShellCmd("setRouteSelection", {
				pieceIds: [piece.id!],
				connectionIds: [],
				kitWiresNodeIds: [],
				kitWiresHoveredNodeId: null,
			}),
		}));
		const connections = ((design as { connections?: readonly SketchpadKitConnection[] } | undefined)?.connections ?? []) as readonly SketchpadKitConnection[];
		const connectionItems: UiTreeItemNode[] = connections
			.filter((connection) => Boolean(connection.id))
			.map((connection) => ({
				id: `sketchpad.hierarchy.connection.${connection.id}`,
				label: connection.id!,
				command: sketchpadShellCmd("setRouteSelection", {
					pieceIds: [],
					connectionIds: [connection.id!],
					kitWiresNodeIds: [],
					kitWiresHoveredNodeId: null,
				}),
			}));
		sections.push({
			type: "section",
			id: "sketchpad.hierarchy.design",
			label: design?.name ?? designId,
			children: [
				{ type: "text", value: `${pieceItems.length} piece(s) · ${connectionItems.length} connection(s)` },
				...pieceItems.map((item) => ({ type: "button" as const, id: item.id, label: item.label, command: item.command })),
				...connectionItems.map((item) => ({ type: "button" as const, id: item.id, label: item.label, command: item.command })),
			],
		});
	} else if (typeId) {
		const type = findTypeInKit(kit, typeId);
		sections.push({
			type: "section",
			id: "sketchpad.hierarchy.type",
			label: type?.name ?? typeId,
			children: [{ type: "text", value: `${type?.representations?.length ?? 0} representation(s)` }],
		});
	} else if (qualityId) {
		const quality = findQualityInKit(kit, qualityId);
		sections.push({
			type: "section",
			id: "sketchpad.hierarchy.quality",
			label: quality?.key ?? qualityId,
			children: [{ type: "text", value: quality?.value ?? "" }],
		});
	} else {
		sections.push(
			{
				type: "section",
				id: "sketchpad.hierarchy.designs",
				label: "Designs",
				children: designItems.length
					? designItems.map((item) => ({ type: "button", id: item.id, label: item.label, command: item.command }))
					: [{ type: "text", value: "(none)" }],
			},
			{
				type: "section",
				id: "sketchpad.hierarchy.types",
				label: "Types",
				children: typeItems.length
					? typeItems.map((item) => ({ type: "button", id: item.id, label: item.label, command: item.command }))
					: [{ type: "text", value: "(none)" }],
			},
		);
	}
	return { ...uiDeclarativeSectionsToTree(sections), selectedIds: sketchpadHierarchySelectedIds(selection) };
}

/** @emoji 📚 Workbench catalogue of kit types for the active route. */
function buildSketchpadCataloguePanelBody(ctx: WindowBodyViewContext): UiTreeNode {
	const ctrl = getSketchpadShellController();
	const routeUri = ctrl?.navigationPath ?? ctx.platform.uri ?? "/";
	const { kitId } = parseSketchpadRouteScopeFromPath(routeUri);
	if (!kitId) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "sketchpad.catalogue.empty",
				label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
				children: [{ type: "text", value: "Open a kit to browse types." }],
			},
		]);
	}
	const kit = ctrl?.getKitStore(kitId)?.getSnapshot().kit;
	if (!kit) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "sketchpad.catalogue.loading", label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, children: [{ type: "text", value: "Kit loading…" }] },
		]);
	}
	const typeRows = sketchpadKitTypeRows(kit);
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "sketchpad.catalogue.types",
			label: `Types (${typeRows.length})`,
			children: typeRows.length
				? typeRows.map((type) => ({
						type: "button",
						id: `sketchpad.catalogue.type.${type.id}`,
						label: type.name ?? type.id ?? "",
						command: sketchpadShellCmd("navigate", { path: `/kits/${kitId}/type/${type.id}` }),
					}))
				: [{ type: "text", value: "(none)" }],
		},
	]);
}

/** @emoji 🔎 Editable inspection panel bound to {@link SketchpadRouteSelection}. */
function buildSketchpadInspectionPanelBody(ctx: WindowBodyViewContext): UiTreeNode {
	const ctrl = getSketchpadShellController();
	const routeUri = ctrl?.navigationPath ?? ctx.platform.uri ?? "/";
	const { kitId, designId, typeId, qualityId } = parseSketchpadRouteScopeFromPath(routeUri);
	const selection = ctrl?.routeSelection ?? sketchpadEmptyRouteSelection();
	const children: UiNode[] = [
		{
			type: "section",
			id: "sketchpad.inspection.header",
			label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
			children: [
				{
					type: "text",
					value: `${selection.pieceIds.length} piece(s) · ${selection.connectionIds.length} connection(s) · ${selection.kitWiresNodeIds.length} kit node(s)`,
				},
			],
		},
	];
	if (!kitId) {
		const shell = ctrl?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL)?.getSnapshot();
		const homeSelected = shell?.home.selectedKitIds ?? [];
		if (homeSelected.length > 0) {
			children.push({
				type: "section",
				id: "sketchpad.inspection.home",
				label: "Home selection",
				children: homeSelected.map((id) => {
					const kit = ctrl?.getKitStore(id)?.getSnapshot().kit;
					return { type: "text", id: `sketchpad.inspection.home.${id}`, value: kit?.name ?? id };
				}),
			});
		} else {
			children[0]!.children!.push({ type: "text", value: "Select a kit on Home or open a design to inspect pieces and connections." });
		}
		return uiDeclarativeSectionsToTree(children);
	}
	const kit = ctrl?.getKitStore(kitId)?.getSnapshot().kit;
	if (!kit) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "sketchpad.inspection.loading", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Kit loading…" }] },
		]);
	}
	if (designId && selection.pieceIds.length > 0) {
		const design = findDesignInKit(kit, designId);
		const pieces = selection.pieceIds
			.map((pieceId) => findPieceInDesign(design!, pieceId))
			.filter((piece): piece is NonNullable<typeof piece> => piece != null);
		const names = pieces.map((piece) => piece.name ?? "");
		const descriptions = pieces.map((piece) => piece.description ?? "");
		const nameUniform = sketchpadAllEqual(names);
		const descriptionUniform = sketchpadAllEqual(descriptions);
		const pieceFields: UiNode[] = [];
		if (selection.pieceIds.length === 1) {
			pieceFields.push({
				type: "field",
				id: "sketchpad.inspection.piece.id",
				label: "Id",
				child: { type: "text", value: selection.pieceIds[0]! },
			});
		}
		pieceFields.push(
			{
				type: "field",
				id: "sketchpad.inspection.piece.name",
				label: "Name",
				child: {
					type: "input",
					id: "sketchpad.inspection.piece.name.input",
					inputKind: "text",
					value: nameUniform ? (names[0] ?? "") : "",
					placeholder: nameUniform ? undefined : "Mixed",
					onChange: sketchpadShellCmd("patchRoutePieces", { kitId, designId, pieceIds: selection.pieceIds, field: "name" }),
				},
			},
			{
				type: "field",
				id: "sketchpad.inspection.piece.description",
				label: "Description",
				child: {
					type: "input",
					id: "sketchpad.inspection.piece.description.input",
					inputKind: "text",
					value: descriptionUniform ? (descriptions[0] ?? "") : "",
					placeholder: descriptionUniform ? undefined : "Mixed",
					onChange: sketchpadShellCmd("patchRoutePieces", { kitId, designId, pieceIds: selection.pieceIds, field: "description" }),
				},
			},
		);
		children.push({
			type: "section",
			id: "sketchpad.inspection.pieces",
			label: `Pieces (${selection.pieceIds.length})`,
			children: pieceFields,
		});
	}
	if (designId && selection.connectionIds.length > 0) {
		const design = findDesignInKit(kit, designId);
		const connections = ((design as { connections?: readonly SketchpadKitConnection[] } | undefined)?.connections ?? []) as readonly SketchpadKitConnection[];
		const selected = connections.filter((connection) => connection.id && selection.connectionIds.includes(connection.id));
		const gaps = selected
			.map((connection) => (connection as Record<string, unknown>).gap)
			.filter((value): value is number => typeof value === "number");
		const gapUniform = sketchpadAllEqual(gaps);
		const connectionFields: UiNode[] = [];
		if (selection.connectionIds.length === 1) {
			connectionFields.push({
				type: "field",
				id: "sketchpad.inspection.connection.id",
				label: "Id",
				child: { type: "text", value: selection.connectionIds[0]! },
			});
		}
		if (gaps.length > 0) {
			connectionFields.push({
				type: "field",
				id: "sketchpad.inspection.connection.gap",
				label: "Gap",
				child: {
					type: "text",
					value: gapUniform ? String(gaps[0] ?? 0) : "Mixed",
				},
			});
		}
		children.push({
			type: "section",
			id: "sketchpad.inspection.connections",
			label: `Connections (${selection.connectionIds.length})`,
			children: connectionFields.length ? connectionFields : [{ type: "text", value: "No editable connection fields" }],
		});
	}
	if (selection.kitWiresNodeIds.length > 0) {
		const meta = ctrl?.kitWiresNodeMetaForKit(kitId) ?? new Map();
		children.push({
			type: "section",
			id: "sketchpad.inspection.kit-wires",
			label: `Kit wires (${selection.kitWiresNodeIds.length})`,
			children: selection.kitWiresNodeIds.slice(0, 12).map((nodeId) => {
				const kind = meta.get(nodeId)?.fileNodeKindId;
				let label = nodeId;
				if (kind === "type") label = findTypeInKit(kit, nodeId)?.name ?? nodeId;
				else if (kind === "design") label = findDesignInKit(kit, nodeId)?.name ?? nodeId;
				return { type: "text", id: `sketchpad.inspection.kit-node.${nodeId}`, value: kind ? `${kind} · ${label}` : label };
			}),
		});
	}
	if (typeId && selection.pieceIds.length === 0 && selection.connectionIds.length === 0 && selection.kitWiresNodeIds.length === 0) {
		const type = findTypeInKit(kit, typeId);
		children.push({
			type: "section",
			id: "sketchpad.inspection.type",
			label: "Type",
			children: [
				{ type: "text", value: type?.name ?? typeId },
				{ type: "text", value: type?.description ?? "" },
				{ type: "text", value: `${type?.representations?.length ?? 0} representation(s)` },
			],
		});
	}
	if (qualityId && selection.pieceIds.length === 0 && selection.connectionIds.length === 0) {
		const quality = findQualityInKit(kit, qualityId);
		children.push({
			type: "section",
			id: "sketchpad.inspection.quality",
			label: "Quality",
			children: [
				{ type: "text", value: quality?.key ?? qualityId },
				{ type: "text", value: quality?.value ?? "" },
			],
		});
	}
	if (
		children.length === 1 &&
		selection.pieceIds.length === 0 &&
		selection.connectionIds.length === 0 &&
		selection.kitWiresNodeIds.length === 0 &&
		!typeId &&
		!qualityId
	) {
		children[0]!.children!.push({ type: "text", value: "Select pieces or connections in the canvas or hierarchy." });
	}
	return uiDeclarativeSectionsToTree(children);
}

class SketchpadPlatformComponents {
	readonly components: readonly Component<unknown>[];

	constructor(platform: Platform) {
		this.components = [
			new SketchpadAppVirtualFileSystem(SKETCHPAD_HOME_APP_ID, platform),
			new SketchpadAppVirtualFileSystem(SKETCHPAD_KIT_APP_ID, platform),
			new SketchpadAppVirtualFileSystem(SKETCHPAD_DESIGN_APP_ID, platform),
			new SketchpadKitWires(platform),
			new SketchpadDesignScene(platform),
			new SketchpadDesignDiagram(platform),
		];
		for (const component of this.components) {
			registerPlatformComponent(platform, component);
			component.refresh();
		}
		platform.subscribe(() => {
			for (const component of this.components) {
				component.refresh();
			}
		});
	}
}
//#endregion 🔖SketchpadPlatformComponents

/** @emoji 🧭 Routes sketchpad navigation and panel chrome through {@link CommandBus}. */
export class SketchpadShellController extends VirtualFileSystemController {
	private readonly vfsRouteRootByScope = new Map<string, string>();
	private readonly shellStore: ObservableCell<SketchpadShellSnapshot>;
	private readonly kitKinds = new Map<string, string>();
	private readonly kitAssetBaseUrls = new Map<string, string>();
	private readonly vfsNodeMetaByScope = new Map<
		string,
		Map<string, { readonly fileNodeKindId: string; readonly typeId?: string; readonly designId?: string }>
	>();
	private readonly kitWiresReferenceCache = new Map<string, SketchpadKitWiresReferenceData>();
	private kitWiresSyncGeneration = 0;
	readonly kitWiresHoverStore = new ObservableCell<string | null>(null);
	readonly kitWiresPreselectStore = new ObservableCell<SketchpadKitWiresPreselectSnapshot>(SKETCHPAD_KIT_WIRES_PRESELECT_EMPTY);

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SKETCHPAD_SHELL_CONTROLLER_ID, commandBus, hostNotify);
		this.shellStore = new ObservableCell<SketchpadShellSnapshot>({
			navigationPath: "/",
			panelVisibility: { leftSidePanel: false, rightSidePanel: false },
			openKitIds: [],
			routeSelection: sketchpadEmptyRouteSelection(),
			home: sketchpadEmptyHomeUiState(),
			importStatus: sketchpadEmptyImportStatus(),
			feedback: sketchpadEmptyFeedbackDraft(),
		});
		this.provideStore(SKETCHPAD_SHELL_STORE_SHELL, this.shellStore);
	}

	get navigationPath(): string {
		return this.shellStore.get().navigationPath;
	}

	get panelVisibility(): SketchpadShellSnapshot["panelVisibility"] {
		return this.shellStore.get().panelVisibility;
	}

	get routeSelection(): SketchpadRouteSelection {
		return this.shellStore.get().routeSelection;
	}

	/** @emoji 📥 Updates home kit import status for workbench feedback. */
	setImportStatus(status: SketchpadImportStatus): void {
		this.shellStore.set({ ...this.shellStore.get(), importStatus: status });
		this.emit();
	}

	/** @emoji 🎯 Updates diagram/scene selection and syncs `/kits/...` query params when applicable. */
	setRouteSelection(selection: SketchpadRouteSelection): void {
		const shell = this.shellStore.get();
		const pathOnly = shell.navigationPath.split("?")[0] ?? "/";
		const kitId = parseSketchpadRouteScopeFromPath(shell.navigationPath).kitId;
		if (!sketchpadPathSupportsRouteSelectionQuery(pathOnly)) {
			this.shellStore.set({ ...shell, routeSelection: selection });
			if (kitId) this.syncKitWiresSelectionToVfs(kitId, selection.kitWiresNodeIds);
			this.emit();
			return;
		}
		const navigationPath = `${pathOnly}${sketchpadRouteSelectionUriFilters(selection)}`;
		this.shellStore.set({ ...shell, routeSelection: selection, navigationPath });
		if (kitId) this.syncKitWiresSelectionToVfs(kitId, selection.kitWiresNodeIds);
		const platform = getSketchpadPlatform();
		if (platform) sketchpadCommitUri(platform, navigationPath);
		this.emit();
	}

	/** @emoji 📋 Open kit ids from the shell store snapshot. */
	listOpenKitIds(): readonly string[] {
		return this.shellStore.get().openKitIds;
	}

	/** @emoji 🗄️ Registers a kit store on this controller (`kit:<id>`). */
	registerKitStore(kitId: string, store: ComposeKitStore, options?: { readonly kind?: string; readonly assetBaseUrl?: string }): void {
		this.provideStore(sketchpadKitStoreId(kitId), store);
		if (options?.kind) this.kitKinds.set(kitId, options.kind);
		if (options?.assetBaseUrl) this.kitAssetBaseUrls.set(kitId, options.assetBaseUrl);
		const openKitIds = this.shellStore.get().openKitIds;
		if (!openKitIds.includes(kitId)) {
			this.shellStore.set({ ...this.shellStore.get(), openKitIds: [...openKitIds, kitId] });
		}
		const platform = getSketchpadPlatform();
		if (platform) sketchpadSyncTypeAppChrome(platform);
		store.subscribe(() => this.invalidateKitVirtualFileSystem(kitId));
		this.emit();
	}

	/** @emoji 🔗 VFS node kind metadata for kit wires selection navigation. */
	kitWiresNodeMetaForKit(kitId: string): ReadonlyMap<string, { readonly fileNodeKindId: string }> {
		const scopeKey = virtualFileSystemScopeKey(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID));
		const map = this.vfsNodeMetaByScope.get(scopeKey);
		if (!map) return new Map();
		const root = this.getRoot(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID));
		const out = new Map<string, { readonly fileNodeKindId: string }>();
		if (root.id === kitId || root.fileNodeKindId === "kit") {
			out.set(root.id, { fileNodeKindId: root.fileNodeKindId });
		}
		for (const [id, meta] of map) out.set(id, meta);
		return out;
	}

	/** @emoji 📁 Awaits kit VFS children for the kit root and every expanded branch so wires identities match visible table rows. */
	async prepareKitWiresVfsForTopology(kitId: string): Promise<void> {
		const scope = sketchpadVfsScope(SKETCHPAD_KIT_APP_ID);
		const expandedIds = this.expandedStore(scope).getSnapshot();
		const branchIds = new Set<string>([kitId, ...expandedIds]);
		await Promise.all([...branchIds].map((id) => this.ensureChildrenLoadedAsync(id, scope)));
	}

	/** @emoji 🎯 Mirrors kit wires diagram selection onto the kit VFS table. */
	syncKitWiresSelectionToVfs(kitId: string, nodeIds: readonly string[]): void {
		const scope = sketchpadVfsScope(SKETCHPAD_KIT_APP_ID);
		if (this.vfsRouteRootByScope.get(virtualFileSystemScopeKey(scope)) !== kitId) return;
		this.selectedRowIdsByScope.set(virtualFileSystemScopeKey(scope), [...nodeIds]);
	}

	/** @emoji 🖱️ Updates kit wires hover (diagram ↔ VFS), without URL query sync. */
	setKitWiresHoveredNodeId(nodeId: string | null): void {
		if (this.kitWiresHoverStore.getSnapshot() === nodeId) return;
		this.kitWiresHoverStore.set(nodeId);
	}

	/** @emoji 👁️ Updates kit wires area-select preview (rectangle/lasso ↔ VFS), without URL query sync. */
	setKitWiresPreselect(preselect: SketchpadKitWiresPreselectSnapshot): void {
		const snap = this.kitWiresPreselectStore.getSnapshot();
		if (sketchpadKitWiresPreselectSnapshotsEqual(snap, preselect)) return;
		this.kitWiresPreselectStore.set({ ids: [...preselect.ids], removedIds: [...preselect.removedIds] });
	}

	/** @emoji 🎯 Kit VFS row ids including any in-progress wires area-select preview. */
	kitWiresEffectiveSelectedRowIds(): string[] {
		return sketchpadKitWiresEffectiveSelectionIds(
			this.shellStore.get().routeSelection.kitWiresNodeIds,
			this.kitWiresPreselectStore.getSnapshot(),
		);
	}

	/** @emoji 🔗 Rebuilds kit wires topology from visible VFS nodes and rust reference queries. */
	syncKitWiresTopology(kitId: string): void {
		const store = this.getKitStore(kitId);
		const kit = store?.getSnapshot().kit;
		if (!kit) return;
		const generation = ++this.kitWiresSyncGeneration;
		void this.prepareKitWiresVfsForTopology(kitId).then(() => {
			if (generation !== this.kitWiresSyncGeneration) return;
			const scope = sketchpadVfsScope(SKETCHPAD_KIT_APP_ID);
			const root = this.getRoot(scope);
			const visible = sketchpadKitWiresVisibleNodes(root, this.visibleVirtualFileSystemNodes(scope));
			const scopeKey = virtualFileSystemScopeKey(scope);
			const vfsMeta = this.vfsNodeMetaByScope.get(scopeKey) ?? new Map();
			const expandedDesignIds = sketchpadExpandedDesignIds(scope, this.expandedStore(scope).getSnapshot(), vfsMeta);
			void sketchpadFetchKitWiresReferences(store!, kit, visible, expandedDesignIds, vfsMeta).then((references) => {
				if (generation !== this.kitWiresSyncGeneration) return;
				this.kitWiresReferenceCache.set(kitId, references);
				const fixture = sketchpadKitWiresFixtureFromVisible(kit, kitId, visible, references, expandedDesignIds);
				this.upsertTopologyStore(sketchpadKitWiresInstanceId(kitId), sketchpadTopologyPayloadForKitWires(fixture));
				this.emit();
			});
		});
	}

	/** @emoji 🔍 Resolves a controller-owned kit store. */
	getKitStore(kitId: string): ComposeKitStore | undefined {
		return this.getStore<SketchpadKitSnapshot>(sketchpadKitStoreId(kitId)) as ComposeKitStore | undefined;
	}

	/** @emoji 🏷️ Persistence kind recorded when the kit was opened. */
	getKitPersistenceKind(kitId: string): string | undefined {
		return this.kitKinds.get(kitId);
	}

	/** @emoji 📂 Returns the asset root URL for kit-relative file paths. */
	getKitAssetBaseUrl(kitId: string): string | undefined {
		return this.kitAssetBaseUrls.get(kitId);
	}

	/** @emoji 🗺️ Refreshes topology stores for routed FiveD surfaces (kit diagram, design/type scene/diagram). */
	syncTopologyForSurface(
		surfaceId: string,
		route: { readonly kitId: string | null; readonly designId: string | null; readonly typeId: string | null },
	): void {
		const { kitId, designId, typeId } = route;
		if (!kitId) return;
		const kit = this.getKitStore(kitId)?.getSnapshot().kit;
		if (!kit) return;
		if (surfaceId === SKETCHPAD_SURFACE_KIT_WIRES) {
			this.syncKitWiresTopology(kitId);
			return;
		}
		const typeRepSurface = sketchpadParseTypeRepresentationSurfaceId(surfaceId);
		if (typeRepSurface && typeRepSurface.kitId === kitId && typeRepSurface.typeId === typeId) {
			const type = findTypeInKit(kit, typeId);
			const representation = type
				? sketchpadListTypeRepresentations(type).find((row) => row.id === typeRepSurface.representationId)
				: undefined;
			if (type && representation) {
				this.upsertTopologyStore(
					sketchpadTypeRepresentationSceneInstanceId(kitId, typeId, representation.id),
					sketchpadTopologyPayloadForTypeRepresentation(type, representation, kit),
				);
			}
			return;
		}
		if (surfaceId === SKETCHPAD_SURFACE_TYPE_SCENE && typeId) {
			const type = findTypeInKit(kit, typeId);
			if (type) {
				this.upsertTopologyStore(sketchpadTypeSceneInstanceId(kitId, typeId), sketchpadTopologyPayloadForTypeScene(type, kit));
			}
			return;
		}
		if (!designId) return;
		const design = findDesignInKit(kit, designId);
		if (!design) return;
		if (surfaceId === SKETCHPAD_SURFACE_DESIGN_SCENE) {
			this.upsertTopologyStore(sketchpadDesignSceneInstanceId(kitId, designId), sketchpadTopologyPayloadForDesignScene(design, kit));
			return;
		}
		if (surfaceId === SKETCHPAD_SURFACE_DESIGN_DIAGRAM) {
			this.upsertTopologyStore(sketchpadDesignDiagramInstanceId(kitId, designId), sketchpadTopologyPayloadForDesignDiagram(design, kit));
		}
	}

	private upsertTopologyStore(instanceId: string, payload: PlatformTopologyPayload): void {
		const storeId = platformTopologyStoreId(instanceId);
		const existing = this.getStore(storeId) as PlatformTopologyStore | undefined;
		if (existing) {
			existing.replacePayload(payload);
			this.emit();
			return;
		}
		this.provideStore(storeId, new PlatformTopologyStore(payload));
		this.emit();
	}

	/** @emoji 📂 Opens a kit via host factories or in-memory import and navigates to it. */
	async openKit(kind: SketchpadKitPersistenceKind, options?: { readonly serverUrl?: string; readonly importUrl?: string }): Promise<string> {
		if (options?.importUrl) {
			return openSketchpadKitFromImport(options.importUrl, { kind, navigate: true });
		}
		if (kind === "remote" && options?.serverUrl?.trim()) {
			const store = await sketchpadOpenRemoteKitStore(options.serverUrl.trim());
			const kitId = store.getSnapshot().kit.id;
			this.registerKitStore(kitId, store, { kind });
			this.navigateTo(`/kits/${kitId}`);
			return kitId;
		}
		const factory = sketchpadKitBackendFactories[kind];
		if (!factory) {
			throw new Error(`compose/sketchpad: no kit factory registered for kind "${kind}"`);
		}
		const store = sketchpadKitStoreFromFactory(await factory());
		const kitId = store.getSnapshot().kit.id;
		this.registerKitStore(kitId, store, { kind });
		this.navigateTo(`/kits/${kitId}`);
		return kitId;
	}

	/** @emoji 🆕 Creates an empty in-memory kit backed by {@link @semio-tech/compose-js} and opens it. */
	async createTemporaryKit(name = "Untitled Kit"): Promise<string> {
		const session = await ComposeSession.openInMemory();
		const jsStore = (await session.stores())[0];
		if (!jsStore) {
			await session.dispose();
			throw new Error("compose/sketchpad: createTemporaryKit found no stores");
		}
		const store = await createComposeKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose() });
		if (name.trim()) {
			const renamed = await (await store.jsKitEntity()).rename(name.trim());
			if (!renamed.ok) throw new Error(`compose/sketchpad: rename failed: ${renamed.error?.message ?? "unknown"}`);
			await store.refreshFromJs();
		}
		const kitId = store.getSnapshot().kit.id;
		this.registerKitStore(kitId, store, { kind: "temporary" });
		this.navigateTo(`/kits/${kitId}`);
		return kitId;
	}

	/** @emoji 🗑️ Closes a kit store and navigates home when it was active. */
	closeKit(kitId: string): void {
		const shell = this.shellStore.get();
		const openKitIds = shell.openKitIds.filter((id) => id !== kitId);
		this.shellStore.set({ ...shell, openKitIds });
		this.revokeStore(sketchpadKitStoreId(kitId));
		for (const storeId of [...this.stores.keys()]) {
			if (storeId.startsWith(PLATFORM_TOPOLOGY_STORE_PREFIX) && storeId.includes(kitId)) {
				this.revokeStore(storeId);
			}
		}
		this.kitKinds.delete(kitId);
		this.kitAssetBaseUrls.delete(kitId);
		const platform = getSketchpadPlatform();
		const activePath = platform?.uri.split("?")[0] ?? shell.navigationPath;
		if (activePath.startsWith(`/kits/${kitId}`)) {
			this.navigateTo(openKitIds.length > 0 ? `/kits/${openKitIds[openKitIds.length - 1]}` : "/");
		}
		this.emit();
	}

	/** @emoji 🏠 Merges home UI state and syncs `/` query params when on the home route. */
	updateHome(home: SketchpadHomeUiState): void {
		const shell = this.shellStore.get();
		const pathOnly = shell.navigationPath.split("?")[0] ?? "/";
		const navigationPath = pathOnly === "/" ? `/${sketchpadHomeUriFilters(home)}` : shell.navigationPath;
		this.shellStore.set({ ...shell, home, navigationPath });
		if (pathOnly === "/") {
			const platform = getSketchpadPlatform();
			if (platform) sketchpadCommitUri(platform, navigationPath);
		}
		this.emit();
	}

	/** @emoji 🧭 Navigates to a path (updates shell snapshot; drives platform when mounted). */
	navigateTo(path: string): void {
		const pathOnly = path.split("?")[0] ?? "/";
		const shell = this.shellStore.get();
		const home = pathOnly === "/" ? parseSketchpadHomeQuery(path) : shell.home;
		const routeSelection = sketchpadPathSupportsRouteSelectionQuery(pathOnly)
			? parseSketchpadRouteSelectionQuery(path)
			: sketchpadEmptyRouteSelection();
		this.shellStore.set({ ...this.shellStore.get(), navigationPath: path, routeSelection, home });
		const platform = getSketchpadPlatform();
		if (!platform) return;
		sketchpadCommitUri(platform, path);
	}

	protected override getSchema(scope: VirtualFileSystemScope): VirtualFileSystemSchemaModel {
		if (scope.appId === SKETCHPAD_HOME_APP_ID) return SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_HOME_SCHEMA_MODEL;
		return SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_TREE_SCHEMA_MODEL;
	}

	protected override getRoot(scope: VirtualFileSystemScope): VirtualFileSystemNodeRecord {
		const route = parseSketchpadRouteScopeFromPath(this.shellStore.get().navigationPath);
		if (scope.appId === SKETCHPAD_HOME_APP_ID) {
			return {
				id: "sketchpad-home",
				fileNodeKindId: "kit",
				name: "Home",
				path: "/",
				parentId: null,
				hasChildren: true,
				canDrag: false,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("kit", { path: "/" }),
			};
		}
		if (scope.appId === SKETCHPAD_KIT_APP_ID) {
			if (!route.kitId) {
				return {
					id: "kit-empty",
					fileNodeKindId: "kit",
					name: "Kit",
					path: "/",
					parentId: null,
					hasChildren: false,
					canDrag: false,
					descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("kit", { path: "/" }),
				};
			}
			const kit = this.getKitStore(route.kitId)?.getSnapshot().kit;
			return {
				id: route.kitId,
				fileNodeKindId: "kit",
				name: kit?.name ?? route.kitId,
				path: "/",
				parentId: null,
				hasChildren: true,
				canDrag: false,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("kit", { path: "/" }),
			};
		}
		if (scope.appId === SKETCHPAD_DESIGN_APP_ID) {
			if (!route.kitId || !route.designId) {
				return {
					id: "design-empty",
					fileNodeKindId: "design",
					name: "Design",
					path: "/",
					parentId: null,
					hasChildren: false,
					descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("design", { path: "/" }),
				};
			}
			const kit = this.getKitStore(route.kitId)?.getSnapshot().kit;
			const design = kit ? findDesignInKit(kit, route.designId) : undefined;
			const path = `/${design?.name ?? route.designId}`;
			return {
				id: route.designId,
				fileNodeKindId: "design",
				name: design?.name ?? route.designId,
				path,
				parentId: route.kitId,
				hasChildren: true,
				descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("design", { path }),
			};
		}
		return {
			id: scope.appId,
			fileNodeKindId: "kit",
			name: scope.appId,
			path: "/",
			parentId: null,
			hasChildren: false,
			canDrag: false,
			descriptorValues: sketchpadKitVirtualFileSystemDescriptorValues("kit", { path: "/" }),
		};
	}

	protected override loadChildren(parentId: string, scope: VirtualFileSystemScope): readonly VirtualFileSystemNodeRecord[] {
		const route = parseSketchpadRouteScopeFromPath(this.shellStore.get().navigationPath);
		if (scope.appId === SKETCHPAD_HOME_APP_ID) {
			const shell = this.shellStore.get();
			return sketchpadHomeVfsChildren(
				this.listOpenKitIds(),
				(kitId) => this.getKitStore(kitId)?.getSnapshot().kit,
				(kitId) => this.getKitPersistenceKind(kitId) ?? "",
				shell.home,
				parentId,
			);
		}
		return [];
	}

	/** @emoji 📁 Loads kit/design VFS children from rs {@link FileSystemNode} over GraphQL. */
	protected override virtualFileSystemUsesAsyncChildren(): boolean {
		return true;
	}

	protected override loadChildrenAsync(
		parentId: string,
		scope: VirtualFileSystemScope,
	): Promise<readonly VirtualFileSystemNodeRecord[]> {
		const route = parseSketchpadRouteScopeFromPath(this.shellStore.get().navigationPath);
		const kitId = route.kitId;
		if (!kitId) return Promise.resolve([]);
		const store = this.getKitStore(kitId);
		if (!store) return Promise.resolve([]);
		if (!(store instanceof ComposeJsKitStore)) {
			const rows = sketchpadKitVfsChildren(store.getSnapshot().kit, parentId);
			this.rememberVfsNodes(scope, rows, route);
			return Promise.resolve(rows);
		}
		const root = this.getRoot(scope);
		const parentRef = sketchpadRsVfsParentRef(parentId, root, route, this.vfsMetaForScope(scope));
		return fetchComposeFileSystemChildren(store.jsStore, parentRef).then((children) => {
			const rows = sketchpadVfsRecordsFromRsChildren(kitId, route, parentId, children);
			this.rememberVfsNodes(scope, rows, route);
			return rows;
		});
	}

	private vfsMetaForScope(
		scope: VirtualFileSystemScope,
	): ReadonlyMap<string, { readonly fileNodeKindId: string; readonly typeId?: string; readonly designId?: string }> {
		return this.vfsNodeMetaByScope.get(virtualFileSystemScopeKey(scope)) ?? new Map();
	}

	private rememberVfsNodes(
		scope: VirtualFileSystemScope,
		rows: readonly VirtualFileSystemNodeRecord[],
		route: ReturnType<typeof parseSketchpadRouteScopeFromPath>,
	): void {
		const key = virtualFileSystemScopeKey(scope);
		let map = this.vfsNodeMetaByScope.get(key);
		if (!map) {
			map = new Map();
			this.vfsNodeMetaByScope.set(key, map);
		}
		const root = this.getRoot(scope);
		map.set(root.id, { fileNodeKindId: root.fileNodeKindId, designId: route.designId ?? undefined, typeId: route.typeId ?? undefined });
		for (const row of rows) {
			const parentMeta = row.parentId ? map.get(row.parentId) : undefined;
			const designId =
				row.fileNodeKindId === "piece" || row.fileNodeKindId === "connection"
					? parentMeta?.fileNodeKindId === "design"
						? row.parentId!
						: (route.designId ?? parentMeta?.designId)
					: (route.designId ?? parentMeta?.designId);
			map.set(row.id, {
				fileNodeKindId: row.fileNodeKindId,
				designId: designId ?? undefined,
				typeId: row.fileNodeKindId === "type" ? row.id : (route.typeId ?? parentMeta?.typeId),
			});
		}
	}

	/** @emoji 🔄 Home vfs stays synchronous; kit/design use async rs-backed children. */
	protected override ensureChildrenLoaded(parentId: string, scope: VirtualFileSystemScope): void {
		if (scope.appId === SKETCHPAD_HOME_APP_ID) {
			const childrenStore = this.childrenStore(scope);
			const key = parentId === this.getRoot(scope).id ? "__root__" : parentId;
			childrenStore.setChildren(key, this.loadChildren(parentId, scope));
			return;
		}
		if (scope.appId === SKETCHPAD_KIT_APP_ID || scope.appId === SKETCHPAD_DESIGN_APP_ID) {
			super.ensureChildrenLoaded(parentId, scope);
			return;
		}
		super.ensureChildrenLoaded(parentId, scope);
	}

	/** @emoji 🧭 Rebinds VFS expansion and drops lazy children when the routed root entity changes (e.g. home → another open kit). */
	syncVirtualFileSystemRoute(scope: VirtualFileSystemScope, rootNodeId: string): void {
		const scopeKey = virtualFileSystemScopeKey(scope);
		if (this.vfsRouteRootByScope.get(scopeKey) === rootNodeId) return;
		this.vfsRouteRootByScope.set(scopeKey, rootNodeId);
		this.expandedStore(scope).setAll([]);
		this.vfsNodeMetaByScope.delete(scopeKey);
		this.childrenByScope.delete(scopeKey);
		this.pendingChildrenLoadsByScope.delete(scopeKey);
		if (scope.appId === SKETCHPAD_KIT_APP_ID) {
			this.kitWiresHoverStore.set(null);
			this.kitWiresPreselectStore.set(SKETCHPAD_KIT_WIRES_PRESELECT_EMPTY);
			this.syncKitWiresTopology(rootNodeId);
		}
	}

	/** @emoji 🔄 Drops cached vfs children when the live kit changes. */
	invalidateKitVirtualFileSystem(kitId: string): void {
		for (const appId of [SKETCHPAD_KIT_APP_ID, SKETCHPAD_DESIGN_APP_ID] as const) {
			const scope = sketchpadVfsScope(appId);
			const scopeKey = virtualFileSystemScopeKey(scope);
		this.vfsNodeMetaByScope.delete(scopeKey);
		this.childrenByScope.delete(scopeKey);
		this.pendingChildrenLoadsByScope.delete(scopeKey);
		}
		this.kitWiresReferenceCache.delete(kitId);
		this.syncKitWiresTopology(kitId);
		this.emit();
	}

	protected override selectedRows(scope: VirtualFileSystemScope): string[] {
		if (scope.appId === SKETCHPAD_HOME_APP_ID) {
			return this.shellStore.get().home.selectedKitIds.map((kitId) => `kit:${kitId}`);
		}
		return super.selectedRows(scope);
	}

	protected override buildVirtualFileSystemModel(scope: VirtualFileSystemScope): VirtualFileSystemModel {
		const model = super.buildVirtualFileSystemModel(scope);
		if (scope.appId === SKETCHPAD_HOME_APP_ID) {
			return { ...model, dragDropEnabled: false };
		}
		if (scope.appId === SKETCHPAD_KIT_APP_ID) {
			return {
				...model,
				selectedRowIds: this.kitWiresEffectiveSelectedRowIds(),
				hoveredRowId: this.kitWiresHoverStore.getSnapshot(),
			};
		}
		return model;
	}

	protected override runVirtualFileSystemCommand(command: string, args?: unknown): boolean {
		const scope = this.resolveScope(args);
		if (scope?.appId === SKETCHPAD_KIT_APP_ID) {
			if (command === "setVirtualFileSystemRowSelection") {
				const selectionPayload = args as { rowIds?: readonly string[]; anchorRowId?: string };
				const rowIds = selectionPayload.rowIds ? [...selectionPayload.rowIds] : [];
				const key = virtualFileSystemScopeKey(scope);
				this.selectedRowIdsByScope.set(key, rowIds);
				if (selectionPayload.anchorRowId) {
					this.selectionAnchorRowIdByScope.set(key, selectionPayload.anchorRowId);
				} else if (!rowIds.length) {
					this.selectionAnchorRowIdByScope.delete(key);
				}
				const kitId = parseSketchpadRouteScopeFromPath(this.shellStore.get().navigationPath).kitId;
				if (kitId) {
					this.setKitWiresPreselect(SKETCHPAD_KIT_WIRES_PRESELECT_EMPTY);
					this.setRouteSelection({ ...this.routeSelection, kitWiresNodeIds: rowIds });
				} else {
					this.emit();
				}
				return true;
			}
			if (command === "virtualFileSystemRowHover") {
				const rowId = (args as { rowId?: string | null }).rowId ?? null;
				this.setKitWiresHoveredNodeId(rowId);
				return true;
			}
		}
		if (scope?.appId === SKETCHPAD_HOME_APP_ID) {
			const payload = (args ?? {}) as { nodeId?: string; rowId?: string };
			if (command === "toggleVirtualFileSystemExpand" && payload.nodeId) {
				const shell = this.shellStore.get();
				const expanded = new Set(shell.home.expandedRowIds);
				if (expanded.has(payload.nodeId)) expanded.delete(payload.nodeId);
				else expanded.add(payload.nodeId);
				const expandedRowIds = [...expanded];
				this.updateHome({ ...shell.home, expandedRowIds });
				const homeScope = sketchpadVfsScope(SKETCHPAD_HOME_APP_ID);
				const expandedStore = this.expandedStore(homeScope);
				expandedStore.setAll(expandedRowIds);
				if (expanded.has(payload.nodeId)) {
					this.ensureChildrenLoaded(payload.nodeId, homeScope);
				}
				this.emit();
				return true;
			}
			if (command === "setVirtualFileSystemRowSelection") {
				const selectionPayload = args as { rowIds?: readonly string[] };
				const kitIds = (selectionPayload.rowIds ?? [])
					.filter((rowId) => rowId.startsWith("kit:"))
					.map((rowId) => rowId.slice(4));
				const shell = this.shellStore.get();
				const selectedKitIds = kitIds.filter((kitId) => shell.openKitIds.includes(kitId));
				this.updateHome({ ...shell.home, selectedKitIds });
				this.emit();
				return true;
			}
		}
		return super.runVirtualFileSystemCommand(command, args);
	}

	override run(command: string, args?: unknown): void {
		if (this.runVirtualFileSystemCommand(command, args)) {
			if (command !== "virtualFileSystemRowHover") {
				const scope = this.resolveScope(args);
				if (scope?.appId === SKETCHPAD_KIT_APP_ID) {
					const kitId = parseSketchpadRouteScopeFromPath(this.shellStore.get().navigationPath).kitId;
					if (kitId) this.syncKitWiresTopology(kitId);
				}
			}
			return;
		}
		const shell = this.shellStore.get();
		switch (command) {
			case "setNavigation": {
				const path = (args as { path: string }).path;
				const pathOnly = path.split("?")[0] ?? "/";
				const home = pathOnly === "/" ? parseSketchpadHomeQuery(path) : shell.home;
				const routeSelection =
					pathOnly === "/"
						? sketchpadEmptyRouteSelection()
						: sketchpadPathSupportsRouteSelectionQuery(pathOnly)
							? parseSketchpadRouteSelectionQuery(path)
							: sketchpadEmptyRouteSelection();
				this.shellStore.set({ ...shell, navigationPath: path, home, routeSelection });
				break;
			}
			case "setHomeFilters": {
				const payload = args as {
					kind?: string | null;
					q?: string;
					name?: string | null;
					version?: string | null;
				};
				this.updateHome({
					...shell.home,
					kindFilter: payload.kind === undefined ? shell.home.kindFilter : payload.kind,
					searchQuery: payload.q === undefined ? shell.home.searchQuery : payload.q,
					nameFilter: payload.name === undefined ? shell.home.nameFilter : payload.name,
					versionFilter: payload.version === undefined ? shell.home.versionFilter : payload.version,
				});
				break;
			}
			case "setHomeSort": {
				const payload = args as { columnId?: string | null; descending?: boolean };
				this.updateHome({
					...shell.home,
					sortColumnId: payload.columnId === undefined ? shell.home.sortColumnId : payload.columnId,
					sortDescending: payload.descending === undefined ? shell.home.sortDescending : payload.descending,
				});
				break;
			}
			case "exportActiveKit": {
				const kitId = sketchpadActiveKitIdFromPath(shell.navigationPath);
				if (!kitId) break;
				const kit = this.getKitStore(kitId)?.getSnapshot().kit;
				if (kit) sketchpadDownloadKitJson(kit);
				break;
			}
			case "copyActiveKitJson": {
				const kitId = sketchpadActiveKitIdFromPath(shell.navigationPath);
				if (!kitId) break;
				const kit = this.getKitStore(kitId)?.getSnapshot().kit;
				if (!kit) break;
				void sketchpadCopyKitJsonToClipboard(kit).catch((error) => {
					console.error("[compose/sketchpad] copyActiveKitJson failed:", error);
				});
				break;
			}
			case "importKitFromDrop": {
				const file = (args as { file?: File }).file;
				if (!file) break;
				this.setImportStatus({ phase: "importing", label: file.name });
				void (async () => {
					const { kit, session, portCompatSource } = await importKit(file);
					const jsStore = (await session.stores())[0];
					if (!jsStore) throw new Error("compose/sketchpad: importKitFromDrop found no stores");
					const store = await createComposeKitStoreFromJsStore(jsStore, {
						onDispose: () => void session.dispose(),
						portCompatSource,
					});
					const kitId = kit.id;
					this.registerKitStore(kitId, store, { kind: "file" });
					this.setImportStatus({ phase: "success", label: kit.name ?? kitId });
					this.navigateTo(`/kits/${kitId}`);
				})().catch((error) => {
					const message = error instanceof Error ? error.message : String(error);
					console.error("[compose/sketchpad] importKitFromDrop failed:", error);
					this.setImportStatus({ phase: "error", error: message });
				});
				break;
			}
			case "togglePanel": {
				const panel = (args as { panel: "leftSidePanel" | "rightSidePanel" }).panel;
				this.shellStore.set({
					...shell,
					panelVisibility: { ...shell.panelVisibility, [panel]: !shell.panelVisibility[panel] },
				});
				break;
			}
			case "openKit": {
				const payload = args as { kind: SketchpadKitPersistenceKind; serverUrl?: string; importUrl?: string };
				void this.openKit(payload.kind, { serverUrl: payload.serverUrl, importUrl: payload.importUrl }).catch((error) => {
					console.error("[compose.sketchpad] openKit failed:", error);
				});
				break;
			}
			case "importKitFromFile": {
				sketchpadPromptHomeKitArchiveFile();
				break;
			}
			case "navigate": {
				this.navigateTo((args as { path: string }).path);
				break;
			}
			case "setFeedbackDraft": {
				const payload = args as { message?: string; contact?: string };
				this.shellStore.set({
					...shell,
					feedback: {
						message: payload.message ?? shell.feedback.message,
						contact: payload.contact ?? shell.feedback.contact,
					},
				});
				break;
			}
			case "submitFeedback": {
				if (typeof window !== "undefined") {
					const mailto = sketchpadFeedbackMailtoUri(shell.feedback);
					if (mailto) window.location.assign(mailto);
				}
				break;
			}
			case "setRouteSelection": {
				this.setRouteSelection(args as SketchpadRouteSelection);
				break;
			}
			case "puzzle5dSelection":
			case "applyPuzzle2dSelection": {
				const payload = args as { instanceId: string; puzzle2dIds: readonly string[] };
				sketchpadApplyPuzzle2dSelection(payload.instanceId, payload.puzzle2dIds);
				break;
			}
			case "puzzle5dActivate": {
				const payload = args as { instanceId: string; puzzle2dIds: readonly string[] };
				sketchpadNavigateFromWiresSelection(payload.instanceId, payload.puzzle2dIds);
				break;
			}
			case "puzzle5dHover": {
				const payload = args as { instanceId: string; nodeId: string | null };
				const { pane } = parseSketchpadPuzzleInstanceId(payload.instanceId);
				if (pane === "kit-wires") {
					this.setKitWiresHoveredNodeId(payload.nodeId);
				}
				break;
			}
			case "puzzle5dPreselect": {
				const payload = args as { instanceId: string; preselect: SketchpadKitWiresPreselectSnapshot };
				const { pane } = parseSketchpadPuzzleInstanceId(payload.instanceId);
				if (pane === "kit-wires") {
					this.setKitWiresPreselect(payload.preselect);
				}
				break;
			}
			case "createTemporaryKit": {
				const name = (args as { name?: string }).name;
				void this.createTemporaryKit(name).catch((error) => {
					console.error("[compose.sketchpad] createTemporaryKit failed:", error);
				});
				break;
			}
			case "renameActiveKit": {
				const kitId = sketchpadActiveKitIdFromPath(shell.navigationPath);
				const name = (args as { name?: string }).name?.trim();
				if (!kitId || !name) break;
				void executeSketchpadJsKitMutation(kitId, (kit) => kit.rename(name))
					.then((result) => {
						if (!result.ok) console.error("[compose.sketchpad] renameActiveKit failed:", result.error?.message);
					})
					.catch((error) => console.error("[compose.sketchpad] renameActiveKit failed:", error));
				break;
			}
			case "createDesignInActiveKit": {
				const kitId = sketchpadActiveKitIdFromPath(shell.navigationPath);
				const designName = (args as { name?: string }).name?.trim() ?? "New design";
				if (!kitId) break;
				void executeSketchpadJsKitMutation(kitId, (kit) => kit.createDesign(designName))
					.then((result) => {
						if (!result.ok) console.error("[compose.sketchpad] createDesignInActiveKit failed:", result.error?.message);
					})
					.catch((error) => console.error("[compose.sketchpad] createDesignInActiveKit failed:", error));
				break;
			}
			case "patchRoutePieces": {
				const payload = args as {
					kitId: string;
					designId: string;
					pieceIds: readonly string[];
					field: "name" | "description";
					value?: unknown;
				};
				if (!payload.kitId || !payload.designId || !payload.pieceIds.length) break;
				const value = String(payload.value ?? "");
				void executeSketchpadJsKitMutation(payload.kitId, async (kit) => {
					const design = kit.design(payload.designId);
					let last: SetResult = { ok: true };
					for (const pieceId of payload.pieceIds) {
						const piece = design.piece(pieceId);
						last =
							payload.field === "name"
								? await piece.rename(value)
								: await piece.changeDescription(value);
						if (!last.ok) return last;
					}
					return last;
				})
					.then((result) => {
						if (!result.ok) console.error("[compose.sketchpad] patchRoutePieces failed:", result.error?.message);
					})
					.catch((error) => console.error("[compose.sketchpad] patchRoutePieces failed:", error));
				break;
			}
			case "closeKit": {
				this.closeKit((args as { kitId: string }).kitId);
				break;
			}
			case "closeActiveKit": {
				const kitId = sketchpadActiveKitIdFromPath(this.shellStore.get().navigationPath);
				if (kitId) this.closeKit(kitId);
				break;
			}
			default:
				break;
		}
		this.emit();
	}
}

let sketchpadPlatformSingleton: Platform | null = null;
let sketchpadPluginHostSingleton: PluginHost | null = null;
let sketchpadPlatformReady: Promise<Platform> | null = null;
let sketchpadBodiesRegistered = false;

function sketchpadShellCommand(
	id: string,
	label: string,
	command: string,
	args?: unknown,
	category = "Sketchpad",
): SearchItemSpec {
	return { id, label, category, controllerId: SKETCHPAD_SHELL_CONTROLLER_ID, command, args };
}

function sketchpadHomeCommands(): readonly SearchItemSpec[] {
	return [
		sketchpadShellCommand("compose.sketchpad.home.createKit", "Create empty kit", "createTemporaryKit", { name: "Untitled Kit" }),
		sketchpadShellCommand("compose.sketchpad.home.importFile", "Import kit from file", "importKitFromFile"),
		sketchpadShellCommand("compose.sketchpad.home.openFolder", "Open folder kit", "openKit", { kind: "folder" }),
		sketchpadShellCommand("compose.sketchpad.home.openFile", "Open file kit", "openKit", { kind: "file" }),
		sketchpadShellCommand("compose.sketchpad.home.openRemote", "Open remote kit", "openKit", { kind: "remote" }),
		sketchpadShellCommand("compose.sketchpad.home.filterTemporary", "Filter · temporary kits", "setHomeFilters", { kind: "temporary" }),
		sketchpadShellCommand("compose.sketchpad.home.filterFile", "Filter · file kits", "setHomeFilters", { kind: "file" }),
		sketchpadShellCommand("compose.sketchpad.home.clearFilters", "Clear home filters", "setHomeFilters", {
			kind: null,
			q: "",
			name: null,
			version: null,
		}),
		sketchpadShellCommand("compose.sketchpad.home.sortUpdated", "Sort home by updated", "setHomeSort", {
			columnId: "updated",
			descending: true,
		}),
		sketchpadShellCommand("compose.sketchpad.home.sortName", "Sort home by name", "setHomeSort", { columnId: "name", descending: false }),
		sketchpadShellCommand("compose.sketchpad.home.openDocs", "Open documentation", "navigate", { path: "/doc/getting-started/index" }),
		sketchpadShellCommand("compose.sketchpad.home.openFeedback", "Open feedback", "navigate", { path: "/feedback" }),
	];
}

function sketchpadKitAppCommands(): readonly SearchItemSpec[] {
	return [
		sketchpadShellCommand("compose.sketchpad.kit.goHome", "Go to Home", "navigate", { path: "/" }),
		sketchpadShellCommand("compose.sketchpad.kit.close", "Close active kit", "closeActiveKit"),
		sketchpadShellCommand("compose.sketchpad.kit.rename", "Rename kit", "renameActiveKit", { name: "Renamed kit" }),
		sketchpadShellCommand("compose.sketchpad.kit.createDesign", "Create design", "createDesignInActiveKit", { name: "New design" }),
		sketchpadShellCommand("compose.sketchpad.kit.export", "Export active kit JSON", "exportActiveKit"),
		sketchpadShellCommand("compose.sketchpad.kit.copyJson", "Copy active kit JSON", "copyActiveKitJson"),
	];
}

function sketchpadHomePanelTabs(): readonly SideTabSpec[] {
	return [
		{ id: FRAMEWORK_PANEL_TAB_HIERARCHY_ID, iconId: FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, panel: "workbench", order: 0, bodyKey: SKETCHPAD_PANEL_HIERARCHY_BODY, label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL },
		{ id: FRAMEWORK_PANEL_TAB_INSPECTION_ID, iconId: FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, panel: "details", order: 0, bodyKey: SKETCHPAD_PANEL_INSPECTION_BODY, label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL },
	];
}

function sketchpadKitPanelTabs(): readonly SideTabSpec[] {
	return [
		{ id: "display", iconId: "compose.sketchpad.icon.windows", panel: "display", bodyKey: SKETCHPAD_PANEL_WINDOWS_BODY, label: "Display" },
		{ id: FRAMEWORK_PANEL_TAB_HIERARCHY_ID, iconId: FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, panel: "workbench", order: 0, bodyKey: SKETCHPAD_PANEL_HIERARCHY_BODY, label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL },
		{ id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID, iconId: FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, panel: "workbench", order: 1, bodyKey: SKETCHPAD_PANEL_CATALOGUE_BODY, label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL },
		{ id: FRAMEWORK_PANEL_TAB_INSPECTION_ID, iconId: FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, panel: "details", order: 0, bodyKey: SKETCHPAD_PANEL_INSPECTION_BODY, label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL },
	];
}

function buildSketchpadExtensionManifest(): PluginManifest {
	return {
		id: SKETCHPAD_EXTENSION_ID,
		label: "Compose Sketchpad",
		contributes: {
			apps: [
				{
					id: SKETCHPAD_HOME_APP_ID,
					label: "Home",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					modes: [{ id: "explore", label: "Explore" }],
					windowKinds: [{ id: "home-main", label: "Home", bodyKey: SKETCHPAD_BODY_HOME }],
					defaultLayout: createTabStackLayout(["home-main"], ["Home"]),
					commands: sketchpadHomeCommands(),
					panelTabs: sketchpadHomePanelTabs(),
				},
				{
					id: SKETCHPAD_KIT_APP_ID,
					label: "Kit",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					modes: [{ id: "explore", label: "Explore" }],
					windowKinds: [
						{ id: "vfs", label: "File System", bodyKey: SKETCHPAD_BODY_KIT_VFS },
						{ id: "wires", label: "Wires", bodyKey: SKETCHPAD_BODY_KIT_WIRES },
					],
					defaultLayout: createDefaultLayout(["vfs", "wires"], "row", [50, 50], ["File System", "Wires"]),
					commands: sketchpadKitAppCommands(),
					panelTabs: sketchpadKitPanelTabs(),
				},
				{
					id: SKETCHPAD_DESIGN_APP_ID,
					label: "Design",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					modes: [{ id: "edit", label: "Edit" }],
					windowKinds: [
						{ id: "scene", label: "Scene", bodyKey: SKETCHPAD_BODY_DESIGN_SCENE },
						{ id: "diagram", label: "Diagram", bodyKey: SKETCHPAD_BODY_DESIGN_DIAGRAM },
					],
					defaultLayout: createDefaultLayout(["scene", "diagram"], "row", [60, 40], ["Scene", "Diagram"]),
					panelTabs: sketchpadKitPanelTabs(),
				},
				{
					id: SKETCHPAD_TYPE_APP_ID,
					label: "Type",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					modes: [{ id: "edit", label: "Edit" }],
					windowKinds: [{ id: "type-empty", label: "Type", bodyKey: SKETCHPAD_BODY_TYPE_REP }],
					defaultLayout: createTabStackLayout(["type-empty"], ["Type"]),
					panelTabs: sketchpadKitPanelTabs(),
				},
				{
					id: SKETCHPAD_DOCS_APP_ID,
					label: "Docs",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					modes: [{ id: "explore", label: "Explore" }],
					windowKinds: [{ id: "docs-main", label: "Docs", bodyKey: SKETCHPAD_BODY_DOCS }],
					defaultLayout: createTabStackLayout(["docs-main"], ["Docs"]),
				},
				{
					id: SKETCHPAD_FEEDBACK_APP_ID,
					label: "Feedback",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					modes: [{ id: "explore", label: "Explore" }],
					windowKinds: [{ id: "feedback-main", label: "Feedback", bodyKey: SKETCHPAD_BODY_FEEDBACK }],
					defaultLayout: createTabStackLayout(["feedback-main"], ["Feedback"]),
				},
			],
		},
	};
}

function registerSketchpadWindowBodies(): void {
	if (sketchpadBodiesRegistered) return;
	sketchpadBodiesRegistered = true;
	registerWindowBody(SKETCHPAD_BODY_HOME, () =>
		buildVirtualFileSystemWindowBody(SKETCHPAD_SURFACE_HOME_VFS, SKETCHPAD_SHELL_CONTROLLER_ID, "home-main"),
	);
	registerWindowBody(SKETCHPAD_BODY_KIT_VFS, () =>
		buildVirtualFileSystemWindowBody(SKETCHPAD_SURFACE_KIT_VFS, SKETCHPAD_SHELL_CONTROLLER_ID, "vfs"),
	);
	registerWindowBody(SKETCHPAD_BODY_KIT_WIRES, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_KIT_WIRES, SKETCHPAD_SHELL_CONTROLLER_ID, "wires"),
	);
	registerWindowBody(SKETCHPAD_BODY_DESIGN_SCENE, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_DESIGN_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID, "scene"),
	);
	registerWindowBody(SKETCHPAD_BODY_DESIGN_DIAGRAM, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_DESIGN_DIAGRAM, SKETCHPAD_SHELL_CONTROLLER_ID, "diagram"),
	);
	registerWindowBody(SKETCHPAD_BODY_TYPE_REP, (ctx: WindowBodyViewContext) => {
		const route = parseSketchpadRouteScopeFromPath(ctx.platform.uri);
		const representationId = ctx.windowKindId.startsWith("rep-") ? ctx.windowKindId.slice(4) : "";
		if (!route.kitId || !route.typeId || !representationId) {
			return buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_TYPE_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID, ctx.windowKindId);
		}
		const surfaceId = sketchpadTypeRepresentationSurfaceId(route.kitId, route.typeId, representationId);
		return buildPuzzle5dWindowBody(surfaceId, SKETCHPAD_SHELL_CONTROLLER_ID, ctx.windowKindId);
	});
	registerWindowBody(SKETCHPAD_BODY_DOCS, () => buildPanelWindowBody(SKETCHPAD_SURFACE_DOCS_PAGE, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerWindowBody(SKETCHPAD_BODY_FEEDBACK, () => buildPanelWindowBody(SKETCHPAD_SURFACE_FEEDBACK_FORM, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerSidePanelBody(SKETCHPAD_PANEL_WINDOWS_BODY, (ctx) => {
		const app = ctx.platform.getActiveApp();
		const windowKinds = app?.windowKinds ?? [];
		const tree: UiTreeNode = {
			type: "tree",
			sections: [
				{
					id: "sketchpad.windows.list",
					label: "Windows",
					defaultOpen: false,
					items: windowKinds.map((windowKind) => ({
						id: `sketchpad.windows.${windowKind.id}`,
						label: windowKind.label,
					})),
				},
			],
		};
		return tree;
	});
	registerSidePanelBody(SKETCHPAD_PANEL_HIERARCHY_BODY, buildSketchpadHierarchyPanelBody);
	registerSidePanelBody(SKETCHPAD_PANEL_CATALOGUE_BODY, buildSketchpadCataloguePanelBody);
	registerSidePanelBody(SKETCHPAD_PANEL_INSPECTION_BODY, buildSketchpadInspectionPanelBody);
}

function applySketchpadUri(platform: Platform, uri: string): void {
	const pathOnly = uri.split("?")[0] ?? "/";
	platform.uri = uri;
	platform.activeAppId = sketchpadAppIdFromPath(pathOnly);
	platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "setNavigation", { path: uri });
	sketchpadSyncTypeAppChrome(platform);
	platform.notify();
}

/** @emoji 🧭 Navigation trail for the current sketchpad URI (decoupled from URL path shape). */
export function sketchpadNavigation(platform: Platform, uri: string): NavigationLevel[] {
	const pathOnly = uri.split("?")[0] ?? "/";
	const scope = parseSketchpadRouteScopeFromPath(uri);
	const controller = getPlatformControllerById(platform, SKETCHPAD_SHELL_CONTROLLER_ID) as SketchpadShellController | undefined;
	const homeAlternatives = sketchpadHomeNavigationAlternatives();
	const homeLevel = sketchpadNavigationLevel(sketchpadNavigationDestination("sketchpad.nav.home", "Home", "/"), homeAlternatives);

	if (pathOnly === "/feedback" || pathOnly.startsWith("/feedback/")) {
		return [
			homeLevel,
			sketchpadNavigationLevel(sketchpadNavigationDestination("sketchpad.nav.feedback", "Feedback", "/feedback"), []),
		];
	}

	if (pathOnly.startsWith("/docs")) {
		const registry = sketchpadBuildDocsRegistry();
		const sectionAlternatives = registry.map((section) =>
			sketchpadNavigationDestination(
				`sketchpad.nav.docs.section.${section.id}`,
				section.label,
				`/doc/${section.pages[0]?.path ?? section.id}`,
			),
		);
		const trail: NavigationLevel[] = [
			homeLevel,
			sketchpadNavigationLevel(sketchpadNavigationDestination("sketchpad.nav.documentation", "Documentation", "/docs"), sectionAlternatives),
		];
		const sectionId = scope.docsPath.split("/")[0] ?? "";
		const section = registry.find((entry) => entry.id === sectionId);
		if (section) {
			trail.push(
				sketchpadNavigationLevel(
					sketchpadNavigationDestination(`sketchpad.nav.docs.section.${section.id}`, section.label, `/doc/${section.pages[0]?.path ?? section.id}`),
					sectionAlternatives,
				),
			);
			const page = section.pages.find((entry) => entry.path === scope.docsPath) ?? section.pages[0];
			if (page && scope.docsPath !== section.pages[0]?.path) {
				trail.push(
					sketchpadNavigationLevel(sketchpadNavigationDestination(`sketchpad.nav.docs.page.${page.path}`, page.title, `/doc/${page.path}`), []),
				);
			}
		}
		return trail;
	}

	if (!scope.kitId) {
		return [homeLevel];
	}

	const kit = controller?.getKitStore(scope.kitId)?.getSnapshot().kit;
	const kitName = kit?.name ?? scope.kitId;
	const kitUri = `/kits/${scope.kitId}`;
	const openKitIds = controller?.listOpenKitIds() ?? (scope.kitId ? [scope.kitId] : []);
	const kitAlternatives = openKitIds.map((kitId) => {
		const openKit = controller?.getKitStore(kitId)?.getSnapshot().kit;
		return sketchpadNavigationDestination(`sketchpad.nav.kit.${kitId}`, openKit?.name ?? kitId, `/kits/${kitId}`);
	});

	const trail: NavigationLevel[] = [
		homeLevel,
		sketchpadNavigationLevel(sketchpadNavigationDestination("sketchpad.nav.kits", "Kits", "/"), kitAlternatives),
		sketchpadNavigationLevel(sketchpadNavigationDestination(`sketchpad.nav.kit.${scope.kitId}`, kitName, kitUri), [
			sketchpadNavigationDestination("sketchpad.nav.typologies", "Typologies", kitUri),
		]),
	];

	const typologies = kit ? sketchpadKitTypologyRows(kit) : [];
	const typologyAlternatives = typologies.map((typology) =>
		sketchpadNavigationDestination(`sketchpad.nav.typology.${typology.id}`, typology.name, kitUri),
	);

	if (scope.designId && kit) {
		const typology = findSketchpadTypologyForDesign(kit, scope.designId);
		const design = findDesignInKit(kit, scope.designId);
		const typologyDesign = typology?.designs.find((row) => sketchpadKitRowEntityId(row) === scope.designId);
		const designName = design?.name ?? (typologyDesign ? sketchpadKitRowEntityName(typologyDesign, scope.designId) : scope.designId);
		const designUri = `/kits/${scope.kitId}/design/${scope.designId}`;
		if (typology && typologies.length > 0) {
			const typologySectionAlternatives: NavigationDestination[] = [];
			if (typology.designs.length > 0) {
				typologySectionAlternatives.push(sketchpadNavigationDestination("sketchpad.nav.typology-designs", "Designs", kitUri));
			}
			if (typology.types.length > 0) {
				typologySectionAlternatives.push(sketchpadNavigationDestination("sketchpad.nav.typology-types", "Types", kitUri));
			}
			trail.push(
				sketchpadNavigationLevel(sketchpadNavigationDestination("sketchpad.nav.typologies", "Typologies", kitUri), typologyAlternatives),
				sketchpadNavigationLevel(
					sketchpadNavigationDestination(`sketchpad.nav.typology.${typology.id}`, typology.name, kitUri),
					typologySectionAlternatives,
				),
				sketchpadNavigationLevel(
					sketchpadNavigationDestination("sketchpad.nav.designs", "Designs", kitUri),
					sketchpadTypologyDesignDestinations(scope.kitId, typology),
				),
				sketchpadNavigationLevel(sketchpadNavigationDestination(`sketchpad.nav.design.${scope.designId}`, designName, designUri), []),
			);
		} else {
			const designAlternatives = (kit.designs ?? [])
				.filter((row): row is Design => typeof row === "object" && row !== null && "id" in row)
				.map((row) =>
					sketchpadNavigationDestination(
						`sketchpad.nav.design.${row.id}`,
						row.name ?? String(row.id),
						`/kits/${scope.kitId}/design/${row.id}`,
					),
				);
			trail.push(
				sketchpadNavigationLevel(sketchpadNavigationDestination("sketchpad.nav.designs", "Designs", kitUri), designAlternatives),
				sketchpadNavigationLevel(sketchpadNavigationDestination(`sketchpad.nav.design.${scope.designId}`, designName, designUri), []),
			);
		}
		return trail;
	}

	if (scope.typeId && kit) {
		const typology = findSketchpadTypologyForType(kit, scope.typeId);
		const type = findTypeInKit(kit, scope.typeId);
		const typologyType = typology?.types.find((row) => sketchpadKitRowEntityId(row) === scope.typeId);
		const typeName = type?.name ?? (typologyType ? sketchpadKitRowEntityName(typologyType, scope.typeId) : scope.typeId);
		const typeUri = `/kits/${scope.kitId}/type/${scope.typeId}`;
		if (typology && typologies.length > 0) {
			const typologySectionAlternatives: NavigationDestination[] = [];
			if (typology.designs.length > 0) {
				typologySectionAlternatives.push(sketchpadNavigationDestination("sketchpad.nav.typology-designs", "Designs", kitUri));
			}
			if (typology.types.length > 0) {
				typologySectionAlternatives.push(sketchpadNavigationDestination("sketchpad.nav.typology-types", "Types", kitUri));
			}
			trail.push(
				sketchpadNavigationLevel(sketchpadNavigationDestination("sketchpad.nav.typologies", "Typologies", kitUri), typologyAlternatives),
				sketchpadNavigationLevel(
					sketchpadNavigationDestination(`sketchpad.nav.typology.${typology.id}`, typology.name, kitUri),
					typologySectionAlternatives,
				),
				sketchpadNavigationLevel(
					sketchpadNavigationDestination("sketchpad.nav.types", "Types", kitUri),
					sketchpadTypologyTypeDestinations(scope.kitId, typology),
				),
				sketchpadNavigationLevel(sketchpadNavigationDestination(`sketchpad.nav.type.${scope.typeId}`, typeName, typeUri), []),
			);
		} else {
			const typeAlternatives = (kit.types ?? [])
				.filter((row): row is Type => typeof row === "object" && row !== null && "id" in row)
				.map((row) =>
					sketchpadNavigationDestination(`sketchpad.nav.type.${row.id}`, row.name ?? String(row.id), `/kits/${scope.kitId}/type/${row.id}`),
				);
			trail.push(
				sketchpadNavigationLevel(sketchpadNavigationDestination("sketchpad.nav.types", "Types", kitUri), typeAlternatives),
				sketchpadNavigationLevel(sketchpadNavigationDestination(`sketchpad.nav.type.${scope.typeId}`, typeName, typeUri), []),
			);
		}
		return trail;
	}

	if (typologies.length > 0) {
		trail.push(sketchpadNavigationLevel(sketchpadNavigationDestination("sketchpad.nav.typologies", "Typologies", kitUri), typologyAlternatives));
	}

	return trail;
}

const SKETCHPAD_PLATFORM_SPEC: PlatformSpec = {
	id: "compose.sketchpad",
	name: "Compose Sketchpad",
	defaultActiveAppId: SKETCHPAD_HOME_APP_ID,
	initialPanelVisibility: PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY,
};

/** @emoji 🧱 Builds the sketchpad {@link Platform} (apps, window bodies, {@link Component} registry). */
export async function buildSketchpadPlatform(): Promise<Platform> {
	sketchpadConfigureBrowserKitFactories();
	registerSketchpadWindowBodies();
	const platform = new Platform(SKETCHPAD_PLATFORM_SPEC);
	const controller = new SketchpadShellController(platform.commandBus, () => platform.notify());
	sketchpadShellControllerSingleton = controller;
	const host = new PluginHost(platform);
	host.register(buildSketchpadExtensionManifest(), {
		id: SKETCHPAD_EXTENSION_ID,
		activate() {},
	} satisfies PluginModule);
	await host.activateAll((controllerId) => (controllerId === SKETCHPAD_SHELL_CONTROLLER_ID ? controller : undefined));
	new SketchpadPlatformComponents(platform);
	platform.applyUri = (uri) => applySketchpadUri(platform, uri);
	platform.navigation = (uri) => sketchpadNavigation(platform, uri);
	if (typeof window === "undefined") {
		platform.activeAppId = SKETCHPAD_HOME_APP_ID;
		platform.notify();
	}
	sketchpadPlatformSingleton = platform;
	sketchpadPluginHostSingleton = host;
	if (typeof window !== "undefined") {
		sketchpadInstallHomeDropzone();
	}
	if (sketchpadPreloadKitUrls().length > 0) {
		void preloadSketchpadKits();
	}
	return platform;
}

/** @emoji 🚀 Ensures the sketchpad {@link Platform} is initialized once per session. */
export async function ensureSketchpadPlatform(): Promise<Platform> {
	if (sketchpadPlatformSingleton) return sketchpadPlatformSingleton;
	if (!sketchpadPlatformReady) {
		sketchpadPlatformReady = buildSketchpadPlatform();
	}
	return sketchpadPlatformReady;
}

/** @emoji 🔍 Returns the live sketchpad {@link Platform}, if built. */
export function getSketchpadPlatform(): Platform | null {
	return sketchpadPlatformSingleton;
}

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("Compose sketchpad i18n", () => {
		function resourceAt(path: string): unknown {
			const tr = composeSketchpadTranslationBundles.en.translation as Record<string, unknown>;
			return path.split(".").reduce<unknown>((acc, key) => (acc && typeof acc === "object" ? (acc as Record<string, unknown>)[key] : undefined), tr);
		}

		it("defines kit-level tag, tags, concept, and concepts strings", () => {
			expect(resourceAt("compose.sketchpad.app.kit.tags.multipleTitle")).toBeDefined();
			expect(resourceAt("compose.sketchpad.app.kit.tag.descriptionPlaceholder.label")).toBeDefined();
			expect(resourceAt("compose.sketchpad.app.kit.concept.descriptionPlaceholder.label")).toBeDefined();
			expect(resourceAt("compose.sketchpad.app.kit.concepts.multipleSelected")).toBeDefined();
		});

		it("defines every toolbar parent category with label objects in en and de", () => {
			const categories: readonly UiToolbarParentCategory[] = [
				"history",
				"hand",
				"selection",
				"lasso",
				"filter",
				"open",
				"save",
				"transfer",
				"transform",
				"create",
				"view",
				"actions",
				"settings",
			];
			for (const locale of ["en", "de"] as const) {
				const parent = composeSketchpadTranslationBundles[locale].translation.compose.sketchpad.toolbar.parent;
				for (const category of categories) {
					const entry = parent[category];
					expect(entry?.label?.normal, `${locale}:${category}`).toBeTruthy();
					expect(entry?.label?.beginner, `${locale}:${category}`).toBeTruthy();
				}
			}
		});
	});

	describe("ComposeKitStore", () => {
		it("InMemoryComposeKitStore exposes kit snapshot", () => {
			const store = new InMemoryComposeKitStore({ id: "k1", name: "Demo" } as Kit);
			expect(store.getSnapshot().kit.name).toBe("Demo");
		});
	});

	describe("Sketchpad virtual file system", () => {
		it("kit file vfs rows use basename without extension and extension icon ids", () => {
			const kit = {
				id: "k1",
				files: [
					{ id: "f1", name: "Tower.glb" },
					{ id: "f2", name: "notes.md" },
					{ id: "f3", path: "assets/plan.pdf" },
				],
			} as Kit;
			const rows = sketchpadKitVfsChildren(kit, "k1");
			const glb = rows.find((row) => row.id === "f1");
			const md = rows.find((row) => row.id === "f2");
			const pdf = rows.find((row) => row.id === "f3");
			expect(glb?.name).toBe("Tower");
			expect(glb?.icon).toBe("glb");
			expect(md?.name).toBe("notes");
			expect(md?.icon).toBe("md");
			expect(pdf?.name).toBe("plan");
			expect(pdf?.icon).toBe("pdf");
		});

		it("schema shows name column only (no path or node kind descriptors)", () => {
			expect(SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_SCHEMA_MODEL.descriptorColumnIds).toEqual([]);
			expect(SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_HOME_SCHEMA_MODEL.descriptorColumnIds).toEqual([]);
			expect(SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_TREE_SCHEMA_MODEL.descriptorColumnIds).toEqual([]);
		});

		it("schema includes representation port and connector node kinds", () => {
			expect(SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_SCHEMA_MODEL.fileNodeKinds.representation?.name).toBe("Representation");
			expect(SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_SCHEMA_MODEL.fileNodeKinds.port?.name).toBe("Port");
			expect(SKETCHPAD_KIT_VIRTUAL_FILE_SYSTEM_SCHEMA_MODEL.fileNodeKinds.connector?.name).toBe("Connector");
		});

		it("maps rs vfs records from graphql child refs", () => {
			const rows = sketchpadVfsRecordsFromRsChildren(
				"kit-1",
				parseSketchpadRouteScopeFromPath("/kits/kit-1"),
				"kit-1",
				[
					{
						id: "type-1",
						kind: "TYPE",
						name: "Window",
						path: "/Window",
						hasChildren: true,
					},
					{
						id: "rep-1",
						kind: "REPRESENTATION",
						name: "Mesh",
						path: "/Window/Mesh",
						hasChildren: false,
						typeId: "type-1",
					},
				],
			);
			expect(rows[0]?.fileNodeKindId).toBe("type");
			expect(rows[0]?.hasChildren).toBe(true);
			expect(rows[1]?.fileNodeKindId).toBe("representation");
			expect(rows[1]?.navigateUri).toContain("/kits/kit-1/type/type-1");
		});

		it("home vfs kit and docs page rows expose navigateUri", () => {
			const kit = { id: "metabolism-id", name: "Metabolism" } as Kit;
			const homeChildren = sketchpadHomeVfsChildren(
				["metabolism-id"],
				(kitId) => (kitId === "metabolism-id" ? kit : undefined),
				() => "temporary",
				sketchpadEmptyHomeUiState(),
				"sketchpad-home",
			);
			const kitRow = homeChildren.find((row) => row.id === "kit:metabolism-id");
			expect(kitRow?.navigateUri).toBe("/kits/metabolism-id");
			const docsSection = sketchpadHomeVfsChildren(["metabolism-id"], () => kit, () => "temporary", sketchpadEmptyHomeUiState(), "docs-root");
			expect(docsSection.length).toBeGreaterThan(0);
			const firstSectionId = docsSection[0]?.id;
			expect(firstSectionId).toBeTruthy();
			const pages = sketchpadHomeVfsChildren(["metabolism-id"], () => kit, () => "temporary", sketchpadEmptyHomeUiState(), firstSectionId!);
			expect(pages.some((page) => page.navigateUri?.startsWith("/doc/"))).toBe(true);
		});
	});

	describe("Sketchpad navigation", () => {
		it("design route trail includes typology levels and home alternatives", () => {
			const kitId = "00000000-0000-4000-8000-000000000001";
			const designId = "00000000-0000-4000-8000-000000000011";
			const designSiblingId = "00000000-0000-4000-8000-000000000012";
			const platform = new Platform({ id: "nav-test", name: "Nav" });
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => platform.notify());
			const app = new AppRuntime(
				SKETCHPAD_HOME_APP_ID,
				"Home",
				undefined,
				ctrl,
				createTabStackLayout(["main"], ["Main"]),
				[new WindowKindRuntime("main", "Main", "test.sketchpad.nav.main")],
			);
			platform.addApp(app);
			const kit = {
				id: kitId,
				name: "Demo Kit",
				typologies: [
					{
						id: "00000000-0000-4000-8000-000000000021",
						name: "Residential",
						types: [],
						designs: [
							{ id: designId, name: "Plan A" },
							{ id: designSiblingId, name: "Plan B" },
						],
					},
				],
			} as Kit;
			ctrl.registerKitStore(kitId, new InMemoryComposeKitStore(kit));
			const trail = sketchpadNavigation(platform, `/kits/${kitId}/design/${designId}`);
			expect(trail.map((level) => level.node.label)).toEqual(["Home", "Kits", "Demo Kit", "Typologies", "Residential", "Designs", "Plan A"]);
			expect(trail[0]?.alternatives.some((alternative) => alternative.label === "Documentation")).toBe(true);
			expect(trail[0]?.alternatives.some((alternative) => alternative.label === "Feedback")).toBe(true);
			const designsLevel = trail.find((level) => level.node.label === "Designs");
			expect(designsLevel?.alternatives.map((alternative) => alternative.label).sort()).toEqual(["Plan A", "Plan B"]);
			ctrl.dispose();
		});
	});

	describe("SketchpadShellController stores", () => {
		it("provideStore registers shell and kit stores", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitStore = new InMemoryComposeKitStore({ id: "k1", name: "A" } as Kit);
			ctrl.registerKitStore("k1", kitStore, { kind: "temporary" });
			expect(ctrl.getStore(SKETCHPAD_SHELL_STORE_SHELL)?.getSnapshot().openKitIds).toEqual(["k1"]);
			expect(ctrl.routeSelection.pieceIds).toEqual([]);
			expect(ctrl.getKitStore("k1")?.getSnapshot().kit.name).toBe("A");
			expect(ctrl.getKitPersistenceKind("k1")).toBe("temporary");
			ctrl.dispose();
		});
	});

	describe("decodeKitComposeEnvelopeToFullFromValue", () => {
		it("unwraps wip.initialKit envelope", () => {
			const inner = decodeKitComposeEnvelopeToFullFromValue({ wip: { initialKit: { id: "k", name: "N" } } });
			expect((inner as { id: string }).id).toBe("k");
		});
	});

	describe("sketchpad preload kits", () => {
		it("parses comma- and whitespace-separated preload kit URLs", () => {
			expect(sketchpadPreloadKitUrls("/a/kit.compose.json, /b/kit.compose.json")).toEqual([
				"/a/kit.compose.json",
				"/b/kit.compose.json",
			]);
			expect(sketchpadPreloadKitUrls("  ")).toEqual([]);
		});

		it(
			"preloads kits on home without navigating to kit app",
			async () => {
			const { dirname, join } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const { readInitialKitFixtureFromPath } = await import("../../../../fixture/script.ts");
			const fixturePath = join(dirname(fileURLToPath(import.meta.url)), "../../../../fixture/kit/dev/metabolism/wip/initialKit/kit.compose.json");
			const fixtureUrl = "/fixture/kit/dev/metabolism/wip/initialKit/kit.compose.json";
			const fixtureJson = JSON.stringify(readInitialKitFixtureFromPath(fixturePath));
			const previousFetch = globalThis.fetch;
			globalThis.fetch = async (input: RequestInfo | URL, init?: RequestInit) => {
				const url =
					typeof input === "string" ? input : input instanceof URL ? input.href : input instanceof Request ? input.url : String(input);
				if (url.includes(fixtureUrl)) {
					return new Response(fixtureJson, { status: 200, headers: { "Content-Type": "application/json" } });
				}
				return previousFetch(input, init);
			};
			try {
				const platform = await buildSketchpadPlatform();
				applySketchpadUri(platform, "/");
				const ctrl = getSketchpadShellController()!;
				for (const openKitId of ctrl.listOpenKitIds()) {
					ctrl.closeKit(openKitId);
				}
				applySketchpadUri(platform, "/");
				await preloadSketchpadKits([fixtureUrl]);
				const kitId = ctrl.listOpenKitIds()[0];
				expect(kitId).toBeTruthy();
				expect(platform.uri.split("?")[0]).toBe("/");
				expect(platform.activeAppId).toBe(SKETCHPAD_HOME_APP_ID);
				expect(ctrl.listOpenKitIds()).toContain(kitId);
				expect(ctrl.getKitAssetBaseUrl(kitId!)).toBe("/fixture/kit/dev/metabolism/wip/initialKit");
			} finally {
				globalThis.fetch = previousFetch;
				getSketchpadShellController()?.dispose();
				sketchpadShellControllerSingleton = null;
				sketchpadPlatformSingleton = null;
				sketchpadPlatformReady = null;
			}
			},
			120_000,
		);
	});

	describe("sketchpadKitFromDecodedBundle", () => {
		it("reads metabolism.kit.light.compose.json fixture file", async () => {
			const { readFileSync } = await import("node:fs");
			const { dirname, join } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const fixturePath = join(dirname(fileURLToPath(import.meta.url)), "../../../../fixture/metabolism.kit.light.compose.json");
			const kit = sketchpadKitFromDecodedBundle(JSON.parse(readFileSync(fixturePath, "utf8")));
			expect(kit?.name).toBe("Metabolism");
			expect(sketchpadExtractPortCompatById(kit!).size).toBeGreaterThan(0);
			expect(sketchpadCollectKitPorts(kit!).length).toBeGreaterThan(0);
			expect(sketchpadReadKitFamilyRows(kit!).some((f) => f["name"] === "Nakagin Capsule Tower")).toBe(true);
		});

		it("reads metabolism-shaped wip.initialKit bundle", () => {
			const raw = {
				schema: "test",
				wip: {
					initialKit: {
						id: "f042c2a4-3ba5-44b0-b22c-0ae8f568aacc",
						name: "Metabolism",
						types: { items: [{ id: "t1", name: "Base" }] },
						designs: { items: [] },
						families: {
							items: [
								{
									id: "fam-nakagin",
									name: "Nakagin Capsule Tower",
									ports: {
										items: [
											{ id: "p1", name: "bottom", compatiblePorts: { items: [{ id: "p2" }] } },
											{ id: "p2", name: "top", compatiblePorts: { items: [{ id: "p1" }] } },
										],
									},
								},
							],
						},
					},
				},
			};
			const kit = sketchpadKitFromDecodedBundle(raw);
			expect(kit?.name).toBe("Metabolism");
			expect(sketchpadExtractPortCompatById(kit!).size).toBe(2);
		});
	});

	describe("sketchpadAppIdFromPath", () => {
		it("resolves type and design apps from singular kit routes", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			const typeId = "22222222-3333-4444-5555-666666666666";
			expect(sketchpadAppIdFromPath(`/kits/${kitId}/design/${designId}`)).toBe(SKETCHPAD_DESIGN_APP_ID);
			expect(sketchpadAppIdFromPath(`/kits/${kitId}/type/${typeId}`)).toBe(SKETCHPAD_TYPE_APP_ID);
		});
	});

	describe("parseSketchpadRouteScopeFromPath", () => {
		it("parses type and design ids from singular kit routes", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			const typeId = "22222222-3333-4444-5555-666666666666";
			expect(parseSketchpadRouteScopeFromPath(`/kits/${kitId}/design/${designId}`)).toMatchObject({ kitId, designId, typeId: null });
			expect(parseSketchpadRouteScopeFromPath(`/kits/${kitId}/type/${typeId}`)).toMatchObject({ kitId, designId: null, typeId });
		});

		it("parses quality query on kit routes", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			expect(parseSketchpadRouteScopeFromPath(`/kits/${kitId}?quality=q1`)).toMatchObject({
				kitId,
				qualityId: "q1",
				designId: null,
				typeId: null,
			});
		});
	});

	describe("sketchpadFeedbackMailtoUri", () => {
		it("requires a non-empty message", () => {
			expect(sketchpadFeedbackMailtoUri({ message: "", contact: "" })).toBeNull();
			expect(sketchpadFeedbackMailtoUri({ message: "Hello", contact: "dev@compose.tech" })).toContain("mailto:feedback@compose-tech.de");
		});
	});

	describe("SketchpadShellController navigation", () => {
		it("closeKit removes store and open id", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			ctrl.registerKitStore("k1", new InMemoryComposeKitStore({ id: "k1", name: "A" } as Kit));
			ctrl.closeKit("k1");
			expect(ctrl.listOpenKitIds()).toEqual([]);
			expect(ctrl.getKitStore("k1")).toBeUndefined();
			ctrl.dispose();
		});

		it("createTemporaryKit registers navigable kit", async () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const id = await ctrl.createTemporaryKit("Test");
			expect(ctrl.listOpenKitIds()).toContain(id);
			expect(ctrl.getKitStore(id)?.getSnapshot().kit.name).toBe("Test");
			expect(ctrl.navigationPath).toBe(`/kits/${id}`);
			expect(ctrl.getKitStore(id)).toBeInstanceOf(ComposeJsKitStore);
			ctrl.dispose();
		});

		it("navigateTo syncs platform uri before onNavigate", () => {
			const bus = new CommandBus();
			const platform = new Platform({ id: "t", name: "T", defaultActiveAppId: SKETCHPAD_HOME_APP_ID });
			platform.applyUri = (uri) => applySketchpadUri(platform, uri);
			let uriWhenHistoryUpdates: string | undefined;
			platform.onNavigate = (uri: string) => {
				uriWhenHistoryUpdates = platform.uri;
				expect(uriWhenHistoryUpdates).toBe(uri);
			};
			sketchpadPlatformSingleton = platform;
			const ctrl = new SketchpadShellController(bus, () => platform.notify());
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const path = `/kits/${kitId}`;
			ctrl.navigateTo(path);
			expect(platform.uri).toBe(path);
			expect(uriWhenHistoryUpdates).toBe(path);
			ctrl.dispose();
			sketchpadPlatformSingleton = null;
		});
	});

	describe("importKit", () => {
		it("materializes type representations after projection install", async () => {
			const { dirname, join } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const { readInitialKitFixtureFromPath } = await import("../../../../fixture/script.ts");
			const fixturePath = join(dirname(fileURLToPath(import.meta.url)), "../../../../fixture/kit/dev/metabolism/wip/initialKit/kit.compose.json");
			const assembled = readInitialKitFixtureFromPath(fixturePath);
			const bundleKit = sketchpadKitFromDecodedBundle(assembled);
			const bundleType = sketchpadKitTypeRows(bundleKit!).find((t) => sketchpadListTypeRepresentations(t).length > 0);
			expect(bundleType).toBeDefined();
			const bundleRepCount = bundleType ? sketchpadListTypeRepresentations(bundleType).length : 0;
			expect(bundleRepCount).toBeGreaterThan(0);
			const { kit, session } = await importKit(new TextEncoder().encode(JSON.stringify(assembled)));
			try {
				const liveType = sketchpadKitTypeRows(kit).find((t) => t.id === bundleType!.id);
				expect(liveType).toBeDefined();
				expect(sketchpadListTypeRepresentations(liveType!)).toHaveLength(bundleRepCount);
			} finally {
				await session.dispose();
			}
		}, 300_000);

		it("hydrates nakagin capsule tower with 180 pieces and 179 connections", async () => {
			const { dirname, join } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const { readInitialKitFixtureFromPath } = await import("../../../../fixture/script.ts");
			const fixturePath = join(dirname(fileURLToPath(import.meta.url)), "../../../../fixture/kit/dev/metabolism/wip/initialKit/kit.compose.json");
			const assembled = readInitialKitFixtureFromPath(fixturePath);
			const { kit, session } = await importKit(new TextEncoder().encode(JSON.stringify(assembled)));
			try {
				const nak = findDesignInKit(kit, "9a890dd4-0a9c-48ac-920a-9e62666465ef");
				expect(nak).toBeDefined();
				expect(nak!.pieces?.length).toBe(180);
				expect(nak!.connections?.length).toBe(179);
				const flat = sketchpadDesignPuzzle2dFixtureFromDesign(nak!, kit);
				expect(flat.nodes).toHaveLength(180);
				expect(flat.edges.length).toBeGreaterThan(0);
				expect(flat.edges[0]?.source).toContain(":");
				expect(flat.nodes.some((node) => node.handles.length > 0)).toBe(true);
			} finally {
				await session.dispose();
			}
		}, 300_000);

		it("hydrates family ports into live kit and diagram compat edges", async () => {
			const payload = JSON.stringify({
				id: "kit-import-test",
				name: "Import Test",
				families: [
					{
						id: "fam1",
						name: "Tower",
						ports: [
							{ id: "p1", name: "bottom", compatiblePorts: [{ id: "p2" }] },
							{ id: "p2", name: "top", compatiblePorts: [{ id: "p1" }] },
						],
					},
				],
				types: [
					{ id: "t1", name: "A", connectors: [{ id: "c1", name: "c1", port: { id: "p1" } }] },
					{ id: "t2", name: "B", connectors: [{ id: "c2", name: "c2", port: { id: "p2" } }] },
				],
				designs: [],
			});
			const { kit, session } = await importKit(new TextEncoder().encode(payload));
			try {
				expect(kit.name).toBe("Import Test");
				expect(Array.isArray(kit.types)).toBe(true);
				expect(sketchpadExtractPortCompatById(kit).size).toBe(2);
				const fixture = sketchpadKitPuzzle2dFixtureFromKit(kit);
				expect(fixture.edges.some((edge) => edge.id === "compat-type:t1-type:t2")).toBe(true);
			} finally {
				await session.dispose();
			}
		}, 120_000);
	});

	describe("executeSketchpadJsKitMutation", () => {
		it("createDesign updates kit snapshot", async () => {
			const session = await ComposeSession.openInMemory({ timeoutMs: 120_000 });
			try {
				const jsStore = (await session.stores())[0]!;
				const store = await createComposeKitStoreFromJsStore(jsStore);
				const bus = new CommandBus();
				const ctrl = new SketchpadShellController(bus, () => {});
				const kitId = store.getSnapshot().kit.id;
				ctrl.registerKitStore(kitId, store);
				const created = await executeSketchpadJsKitMutation(kitId, (kit) => kit.createDesign("Layout A"), store);
				expect(created.ok).toBe(true);
				expect(store.getSnapshot().kit.designs?.some((d) => d.name === "Layout A")).toBe(true);
				ctrl.dispose();
			} finally {
				await session.dispose();
			}
		});
	});

	describe("findDesignInKit", () => {
		it("returns design by id", () => {
			const kit = { id: "k", designs: [{ id: "d1", name: "D" }] } as Kit;
			expect(findDesignInKit(kit, "d1")?.name).toBe("D");
		});
	});

	describe("findTypeInKit", () => {
		it("finds kinds nested under typologies when root types is empty", () => {
			const bridgeTypeId = "0e240cd2-7f98-42b6-af39-34e7ee4fad35";
			const bridgeFileId = "60ace9d9-441d-412a-8c91-69e7993fafee";
			const kit = {
				id: "k",
				typologies: {
					items: [
						{
							id: "topo-bridge",
							name: "Bridge",
							types: {
								items: [
									{
										id: bridgeTypeId,
										name: "Bridge",
										representations: {
											items: [{ id: "rep-bridge", name: "bridge", file: { id: bridgeFileId } }],
										},
									},
								],
							},
						},
					],
				},
				files: [{ id: bridgeFileId, name: "bridge.glb" }],
			} as Kit;
			expect(findTypeInKit(kit, bridgeTypeId)?.name).toBe("Bridge");
			expect(sketchpadResolvePieceMeshUrl({ type: { id: bridgeTypeId } }, kit)).toBe("/mesh/bridge.glb");
		});
	});

	describe("sketchpadKitWiresVisibleNodes", () => {
		it("prepends kit root when the platform helper omits it", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const root: VirtualFileSystemNodeRecord = {
				id: kitId,
				fileNodeKindId: "kit",
				name: "K",
				path: "/",
				parentId: null,
				hasChildren: true,
			};
			const visible: VirtualFileSystemVisibleNode[] = [
				{ id: typeId, fileNodeKindId: "type", name: "T", path: "/T", parentId: kitId, hasChildren: false },
			];
			const merged = sketchpadKitWiresVisibleNodes(root, visible);
			expect(merged).toHaveLength(2);
			expect(merged[0]?.id).toBe(kitId);
			expect(merged[1]?.id).toBe(typeId);
		});
	});

	describe("sketchpadKitWiresFixtureFromVisible", () => {
		it("emits one identity per visible vfs node and owns containment edges", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const kit = { id: kitId, name: "K" } as Kit;
			const visible: VirtualFileSystemVisibleNode[] = [
				{ id: kitId, fileNodeKindId: "kit", name: "K", path: "/K", parentId: null, hasChildren: true },
				{ id: typeId, fileNodeKindId: "type", name: "T", path: "/T", parentId: kitId, hasChildren: false },
			];
			const fixture = sketchpadKitWiresFixtureFromVisible(
				kit,
				kitId,
				visible,
				{ designTransitiveTypes: new Map(), designTransitiveDesigns: new Map(), pieceBlueprints: new Map() },
				new Set(),
			);
			expect(fixture.identities).toHaveLength(2);
			expect(fixture.relationships.some((rel) => rel.kind === "owns" && rel.edgeId.includes(kitId) && rel.edgeId.includes(typeId))).toBe(true);
		});

		it("renders identity icons from entity declarations and omits generic file-node-kind icons", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const plainTypeId = "22222222-3333-4444-5555-666666666666";
			const unknownId = "99999999-8888-7777-6666-555555555555";
			const kit = {
				id: kitId,
				name: "K",
				icon: "icons/metabolism.svg",
				types: [
					{ id: typeId, name: "Beam", icon: "icons/base.svg" },
					{ id: plainTypeId, name: "Plain" },
				],
				designs: [],
			} as Kit;
			const visible: VirtualFileSystemVisibleNode[] = [
				{ id: kitId, fileNodeKindId: "kit", name: "Kit Alpha", path: "/K", parentId: null, hasChildren: true },
				{ id: typeId, fileNodeKindId: "type", name: "Beam", path: "/T", parentId: kitId, hasChildren: false },
				{ id: plainTypeId, fileNodeKindId: "type", name: "Plain", path: "/P", parentId: kitId, hasChildren: false },
				{ id: unknownId, fileNodeKindId: "mystery", name: "Mystery", path: "/M", parentId: kitId, hasChildren: false },
			];
			const fixture = sketchpadKitWiresFixtureFromVisible(
				kit,
				kitId,
				visible,
				{ designTransitiveTypes: new Map(), designTransitiveDesigns: new Map(), pieceBlueprints: new Map() },
				new Set(),
			);
			const byId = new Map(fixture.board.nodes.map((node) => [node.id, node] as const));
			const kitNode = byId.get(kitId) as { text?: string; iconKind?: string; nodeKind?: string };
			const typeNode = byId.get(typeId) as { text?: string; iconKind?: string; nodeKind?: string };
			const plainTypeNode = byId.get(plainTypeId) as { iconKind?: string };
			const unknownNode = byId.get(unknownId) as { text?: string; iconKind?: string };
			expect(kitNode.text).toBe("Kit Alpha");
			expect(typeNode.text).toBe("Beam");
			expect(kitNode.iconKind).toBe("metabolism");
			expect(typeNode.iconKind).toBe("base");
			expect(plainTypeNode.iconKind).toBeUndefined();
			expect(unknownNode.text).toBe("Mystery");
			expect(unknownNode.iconKind).toBeUndefined();
			expect(kitNode.nodeKind).toBe(`compose.kit.identity.${kitId}`);
			expect(typeNode.nodeKind).toBe(`compose.kit.identity.${typeId}`);
			expect(fixture.kindCatalogs?.identityKinds?.some((row) => row.id === `compose.kit.identity.${typeId}` && row.icon === "base")).toBe(true);
			expect(fixture.kindCatalogs?.identityKinds?.some((row) => row.id === `compose.kit.identity.${plainTypeId}` && row.icon === undefined)).toBe(true);
			const board = wiresFixtureBoard(fixture);
			const boardType = board.nodes.find((node) => node.id === typeId) as { text?: string; iconKind?: string };
			expect(boardType.text).toBe("Beam");
			expect(boardType.iconKind).toBe("base");
		});

		it("includes kit file vfs rows and folder containment edges", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const folderId = "22222222-3333-4444-5555-666666666666";
			const fileId = "33333333-4444-5555-6666-777777777777";
			const kit = {
				id: kitId,
				name: "K",
				folders: [{ id: folderId, path: "/docs" }],
				files: [{ id: fileId, name: "readme.txt", folder: { id: folderId } }],
			} as Kit;
			const visible: VirtualFileSystemVisibleNode[] = [
				{ id: kitId, fileNodeKindId: "kit", name: "K", path: "/K", parentId: null, hasChildren: true },
				{ id: folderId, fileNodeKindId: "folder", name: "docs", path: "/docs", parentId: kitId, hasChildren: true },
				{ id: fileId, fileNodeKindId: "file", name: "readme.txt", path: "/readme.txt", parentId: folderId, hasChildren: false },
			];
			const fixture = sketchpadKitWiresFixtureFromVisible(
				kit,
				kitId,
				visible,
				{ designTransitiveTypes: new Map(), designTransitiveDesigns: new Map(), pieceBlueprints: new Map() },
				new Set(),
			);
			expect(fixture.identities.some((row) => row.nodeId === fileId)).toBe(true);
			expect(fixture.relationships.some((rel) => rel.kind === "owns" && rel.edgeId.includes(folderId) && rel.edgeId.includes(fileId))).toBe(true);
		});

		it("uses references for collapsed design and has+is when design is expanded", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "22222222-3333-4444-5555-666666666666";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const pieceId = "33333333-4444-5555-6666-777777777777";
			const kit = { id: kitId, name: "K" } as Kit;
			const visibleCollapsed: VirtualFileSystemVisibleNode[] = [
				{ id: designId, fileNodeKindId: "design", name: "D", path: "/D", parentId: kitId, hasChildren: true },
				{ id: typeId, fileNodeKindId: "type", name: "T", path: "/T", parentId: kitId, hasChildren: false },
			];
			const collapsedFixture = sketchpadKitWiresFixtureFromVisible(
				kit,
				kitId,
				visibleCollapsed,
				{
					designTransitiveTypes: new Map([[designId, [typeId]]]),
					designTransitiveDesigns: new Map(),
					pieceBlueprints: new Map(),
				},
				new Set(),
			);
			expect(collapsedFixture.relationships.some((rel) => rel.kind === "references")).toBe(true);
			expect(collapsedFixture.relationships.some((rel) => rel.kind === "has" && rel.edgeId.includes(pieceId))).toBe(false);

			const visibleExpanded: VirtualFileSystemVisibleNode[] = [
				...visibleCollapsed,
				{ id: pieceId, fileNodeKindId: "piece", name: "P", path: "/P", parentId: designId, hasChildren: false },
			];
			const expandedFixture = sketchpadKitWiresFixtureFromVisible(
				kit,
				kitId,
				visibleExpanded,
				{
					designTransitiveTypes: new Map([[designId, [typeId]]]),
					designTransitiveDesigns: new Map(),
					pieceBlueprints: new Map([[pieceId, { kind: "Type", id: typeId }]]),
				},
				new Set([designId]),
			);
			expect(expandedFixture.relationships.some((rel) => rel.kind === "references")).toBe(false);
			expect(expandedFixture.relationships.some((rel) => rel.kind === "has" && rel.edgeId.includes(pieceId))).toBe(true);
			expect(expandedFixture.relationships.some((rel) => rel.kind === "is" && rel.edgeId.includes(pieceId) && rel.edgeId.includes(typeId))).toBe(true);
		});
	});

	describe("parseSketchpadPuzzleInstanceId", () => {
		it("parses kit wires and design panes", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			expect(parseSketchpadPuzzleInstanceId(sketchpadKitWiresInstanceId(kitId))).toEqual({
				kitId,
				designId: null,
				typeId: null,
				pane: "kit-wires",
			});
			expect(parseSketchpadPuzzleInstanceId(sketchpadDesignSceneInstanceId(kitId, designId))).toEqual({
				kitId,
				designId,
				typeId: null,
				pane: "scene",
			});
			const typeId = "22222222-3333-4444-5555-666666666666";
			expect(parseSketchpadPuzzleInstanceId(sketchpadTypeSceneInstanceId(kitId, typeId))).toEqual({
				kitId,
				designId: null,
				typeId,
				pane: "type-scene",
			});
		});
	});

	describe("sketchpadTypeVolumeFixtureFromType", () => {
		it("places one mesh object at the origin", () => {
			const kit = {
				id: "k",
				types: [{ id: "t1", name: "Chair", representations: [{ id: "r1", name: "chair", file: { id: "f1" } }] }],
				files: [{ id: "f1", path: "files/chair.glb" }],
			} as Kit;
			const volume = sketchpadTypeVolumeFixtureFromType(kit.types![0]!, kit);
			expect(volume.objects).toHaveLength(1);
			expect(volume.objects[0]?.id).toBe("r1");
			expect(volume.objects[0]?.meshUrl).toContain("chair.glb");
		});
	});

	describe("sketchpadKitFileUrlById", () => {
		it("maps embedded file blobs to data URLs when the row is not a metabolism glb name", () => {
			const kit = {
				id: "k",
				files: [{ id: "f1", name: "mesh.bin", blob: "data:application/octet-stream;base64,AAAA" }],
			} as Kit;
			expect(sketchpadKitFileUrlById(kit).get("f1")).toBe("data:application/octet-stream;base64,AAAA");
		});

		it("resolves metabolism glbs by file name for puzzle 3d even without url", () => {
			const kit = {
				id: "k",
				files: [{ id: "60ace9d9-441d-412a-8c91-69e7993fafee", name: "bridge.glb" }],
			} as Kit;
			expect(sketchpadKitFileUrlById(kit).get("60ace9d9-441d-412a-8c91-69e7993fafee")).toBe("/mesh/bridge.glb");
		});

		it("prefers fixture /meshes over inline blobs for metabolism glb names", () => {
			const kit = {
				id: "k",
				files: [{ id: "f1", name: "bridge.glb", blob: "data:model/gltf-binary;base64,AAAA" }],
			} as Kit;
			expect(sketchpadKitFileUrlById(kit).get("f1")).toBe("/mesh/bridge.glb");
		});

		it("resolves metabolism representation glbs for puzzle 3d via /meshes", () => {
			const kit = {
				id: "k",
				files: [{ id: "60ace9d9-441d-412a-8c91-69e7993fafee", name: "bridge.glb" }],
			} as Kit;
			expect(sketchpadKitFileUrlById(kit).get("60ace9d9-441d-412a-8c91-69e7993fafee")).toBe("/mesh/bridge.glb");
		});

		it("resolves capsule-with-balcony representation glbs for puzzle 3d via /meshes", () => {
			const kit = {
				id: "k",
				files: [{ id: "f-slash", path: "representations/capsule-with-balcony_slash.glb", name: "capsule-with-balcony_slash.glb" }],
			} as Kit;
			expect(sketchpadKitFileUrlById(kit).get("f-slash")).toBe("/mesh/capsule-with-balcony_slash.glb");
		});
	});

	describe("sketchpadMergeKitDtoFromBundleProjection", () => {
		it("copies representations and files from bundle when live kit has none", () => {
			const live = {
				id: "k",
				types: [{ id: "t1", name: "A", representations: [] }],
				files: [],
			} as Kit;
			const bundle = {
				id: "k",
				types: [{ id: "t1", name: "A", representations: [{ id: "r1", name: "mesh", file: { id: "f1" } }] }],
				files: [{ id: "f1", blob: "data:model/gltf-binary;base64,AAAA" }],
			} as Kit;
			const merged = sketchpadMergeKitDtoFromBundleProjection(live, bundle);
			expect(sketchpadListTypeRepresentations(merged.types![0]!)).toHaveLength(1);
			expect(merged.files).toHaveLength(1);
		});
	});

	describe("sketchpadSyncTypeAppChrome", () => {
		it("creates one window kind per representation", () => {
			const platform = new Platform({ id: "t", name: "T", defaultActiveAppId: SKETCHPAD_TYPE_APP_ID });
			const ctrl = new SketchpadShellController(new CommandBus(), () => platform.notify());
			sketchpadShellControllerSingleton = ctrl;
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const kit = {
				id: kitId,
				types: [
					{
						id: typeId,
						name: "Blob",
						representations: [
							{ id: "r1", name: "alpha", file: { id: "f1" } },
							{ id: "r2", name: "beta", file: { id: "f2" } },
						],
					},
				],
				files: [
					{ id: "f1", blob: "data:model/gltf-binary;base64,AAAA" },
					{ id: "f2", blob: "data:model/gltf-binary;base64,BBBB" },
				],
			} as Kit;
			platform.addApp(
				new AppRuntime(
					SKETCHPAD_TYPE_APP_ID,
					"Type",
					undefined,
					ctrl,
					createTabStackLayout(["type-empty"], ["Type"]),
					[new WindowKindRuntime("type-empty", "Type", SKETCHPAD_BODY_TYPE_REP)],
				),
			);
			ctrl.registerKitStore(kitId, new InMemoryComposeKitStore(kit));
			platform.uri = `/kits/${kitId}/type/${typeId}`;
			sketchpadSyncTypeAppChrome(platform);
			const typeApp = platform.apps.find((app) => app.id === SKETCHPAD_TYPE_APP_ID);
			expect(typeApp?.windowKinds.map((wk) => wk.id)).toEqual(["rep-r1", "rep-r2"]);
			expect(typeApp?.defaultLayout.root.kind).toBe("stack");
			ctrl.dispose();
		});
	});

	describe("sketchpadPortDtoFromGraphqlNode", () => {
		it("maps copatibleWith edges to compatiblePorts", () => {
			const port = sketchpadPortDtoFromGraphqlNode({
				id: "p1",
				copatibleWith: { edges: [{ node: { id: "p2" } }] },
			});
			expect(sketchpadReadCompatiblePortIds(port)).toEqual(["p2"]);
		});
	});

	describe("sketchpadMergePortCompatMaps", () => {
		it("overlays graphql compat onto bundle-derived compat", () => {
			const base = new Map<string, readonly { readonly id: string }[]>([["p1", [{ id: "p-old" }]]]);
			const overlay = new Map<string, readonly { readonly id: string }[]>([["p1", [{ id: "p-new" }]]]);
			const merged = sketchpadMergePortCompatMaps(base, overlay);
			expect(merged.get("p1")).toEqual([{ id: "p-new" }]);
		});
	});

	describe("sketchpadApplyPortCompatById", () => {
		it("restores compatiblePorts stripped by GraphQL-shaped reads", () => {
			const bundle = {
				id: "k",
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1", compatiblePorts: [{ id: "p2" }] } }] },
					{ id: "t2", connectors: [{ port: { id: "p2", compatiblePorts: [{ id: "p1" }] } }] },
				],
			} as Kit;
			const graphqlKit = {
				id: "k",
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1", label: "A" } }] },
					{ id: "t2", connectors: [{ port: { id: "p2", label: "B" } }] },
				],
			} as Kit;
			const compat = sketchpadExtractPortCompatById(bundle);
			const merged = sketchpadApplyPortCompatById(graphqlKit, compat);
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(merged);
			expect(fixture.edges.some((e) => e.id === "compat-type:t1-type:t2")).toBe(true);
		});

		it("reads port compat from kit families and wires type adjacency via connectors", () => {
			const bundle = {
				id: "k",
				families: [
					{
						id: "fam1",
						name: "Tower",
						ports: [
							{ id: "p1", name: "core bottom", compatiblePorts: [{ id: "p2" }] },
							{ id: "p2", name: "core top", compatiblePorts: [{ id: "p1" }] },
						],
					},
				],
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1" } }] },
					{ id: "t2", connectors: [{ port: { id: "p2" } }] },
				],
			} as Kit;
			const compat = sketchpadExtractPortCompatById(bundle);
			expect(compat.size).toBe(2);
			const graphqlKit = {
				id: "k",
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1" } }] },
					{ id: "t2", connectors: [{ port: { id: "p2" } }] },
				],
			} as Kit;
			const merged = sketchpadApplyPortCompatById(graphqlKit, compat);
			expect(sketchpadCollectKitPorts(merged).map((p) => p.id).sort()).toEqual(["p1", "p2"]);
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(merged);
			expect(fixture.edges.some((e) => e.id === "compat-type:t1-type:t2")).toBe(true);
		});
	});

	describe("sketchpadReadKitFamilyRows", () => {
		it("accepts denormalized arrays and block items", () => {
			const fromArray = sketchpadReadKitFamilyRows({ id: "k", families: [{ id: "f1" }] } as Kit);
			expect(fromArray).toHaveLength(1);
			const fromBlock = sketchpadReadKitFamilyRows({
				id: "k",
				families: { items: [{ id: "f2" }] },
			} as Kit);
			expect(fromBlock[0]?.["id"]).toBe("f2");
		});
	});

	describe("sketchpadEnsureHomeKitFileInput", () => {
		it("creates a hidden file input once", () => {
			if (typeof document === "undefined") return;
			sketchpadInstallHomeDropzone();
			const input = document.getElementById(SKETCHPAD_HOME_KIT_FILE_INPUT_ID);
			expect(input?.getAttribute("type")).toBe("file");
		});
	});

	describe("sketchpadSetHomeDropzoneOverlayVisible", () => {
		it("creates and toggles the overlay element", () => {
			if (typeof document === "undefined") return;
			sketchpadSetHomeDropzoneOverlayVisible(true);
			const overlay = document.getElementById(SKETCHPAD_HOME_DROPZONE_OVERLAY_ID);
			expect(overlay).toBeTruthy();
			expect(overlay?.classList.contains("hidden")).toBe(false);
			sketchpadSetHomeDropzoneOverlayVisible(false);
			expect(overlay?.classList.contains("hidden")).toBe(true);
		});
	});

	describe("parseSketchpadRouteSelectionQuery", () => {
		it("reads piece connection and diagram ids from query params", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			const selection = parseSketchpadRouteSelectionQuery(
				`/kits/${kitId}/design/${designId}?piece=p1&piece=p2&conn=c1&wires=type:t1`,
			);
			expect(selection.pieceIds).toEqual(["p1", "p2"]);
			expect(selection.connectionIds).toEqual(["c1"]);
			expect(selection.kitWiresNodeIds).toEqual(["type:t1"]);
			expect(selection.kitWiresHoveredNodeId).toBeNull();
		});
	});

	describe("sketchpadRouteSelectionUriFilters", () => {
		it("round-trips selection through query serialization", () => {
			const selection = { pieceIds: ["a"], connectionIds: ["b"], kitWiresNodeIds: ["type:x"], kitWiresHoveredNodeId: null };
			const uri = `/kits/k/design/d${sketchpadRouteSelectionUriFilters(selection)}`;
			expect(parseSketchpadRouteSelectionQuery(uri)).toEqual(selection);
		});
	});

	describe("SketchpadShellController route selection URL", () => {
		it("syncs navigation path when selection changes on a design route", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			ctrl.navigateTo(`/kits/${kitId}/design/${designId}`);
			ctrl.setRouteSelection({ pieceIds: ["piece-a"], connectionIds: [], kitWiresNodeIds: [], kitWiresHoveredNodeId: null });
			expect(ctrl.navigationPath).toBe(`/kits/${kitId}/design/${designId}?piece=piece-a`);
			expect(ctrl.routeSelection.pieceIds).toEqual(["piece-a"]);
			ctrl.dispose();
		});
	});

	describe("parseSketchpadHomeQuery", () => {
		it("reads kind and search filters from the URI", () => {
			const home = parseSketchpadHomeQuery("/?kind=file&q=metab&e=docs-root&sel=k1&sort=updated&dir=desc");
			expect(home.kindFilter).toBe("file");
			expect(home.searchQuery).toBe("metab");
			expect(home.expandedRowIds).toEqual(["docs-root"]);
			expect(home.selectedKitIds).toEqual(["k1"]);
			expect(home.sortColumnId).toBe("updated");
			expect(home.sortDescending).toBe(true);
		});
	});

	describe("Sketchpad inspection panel", () => {
		it("reflects home kit selection in the inspection tree", async () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(kitId, new InMemoryComposeKitStore({ id: kitId, name: "Selected Kit" } as Kit), { kind: "fixture" });
			ctrl.navigateTo("/");
			platform.uri = "/";
			ctrl.updateHome({ ...sketchpadEmptyHomeUiState(), selectedKitIds: [kitId] });
			const tree = buildSketchpadInspectionPanelBody({ platform, windowKindId: "home-main", bus: platform.commandBus });
			const text = JSON.stringify(tree);
			expect(text).toContain("Selected Kit");
			ctrl.dispose();
		});

		it("reflects kit virtual file system row selection", async () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({ id: kitId, name: "Kit", types: [{ id: typeId, name: "Column" }] } as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			platform.uri = `/kits/${kitId}`;
			ctrl.buildVirtualFileSystemModel(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID));
			ctrl.setRouteSelection({ pieceIds: [], connectionIds: [], kitWiresNodeIds: [typeId], kitWiresHoveredNodeId: null });
			const tree = buildSketchpadInspectionPanelBody({ platform, windowKindId: "vfs", bus: platform.commandBus });
			const text = JSON.stringify(tree);
			expect(text).toContain("Column");
			ctrl.dispose();
		});
	});

	describe("sketchpadKitToComposeEnvelope", () => {
		it("wraps kit in wip.initialKit", () => {
			const envelope = sketchpadKitToComposeEnvelope({ id: "k1", name: "Demo" } as Kit);
			expect((envelope.wip.initialKit as Kit).name).toBe("Demo");
		});
	});

	describe("sketchpadResolveMdxModuleKey", () => {
		it("resolves index and leaf docs paths", () => {
			expect(sketchpadResolveMdxModuleKey("getting-started/index")).toMatch(/getting-started\/index\.mdx$/);
			expect(sketchpadResolveMdxModuleKey("getting-started/installation")).toMatch(/installation\.mdx$/);
		});
	});

	describe("Sketchpad home virtual file system", () => {
		it("lists open kits under the home root", async () => {
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(
				"k-home",
				new InMemoryComposeKitStore({ id: "k-home", name: "Demo Kit", version: "r1", updatedAt: "2025-06-01T12:00:00.000Z" } as Kit),
				{ kind: "fixture" },
			);
			ctrl.navigateTo("/");
			platform.uri = "/";
			const scope = sketchpadVfsScope(SKETCHPAD_HOME_APP_ID);
			const snap = ctrl.buildVirtualFileSystemModel(scope);
			expect(snap.rows.map((row) => row.id)).toContain("kit:k-home");
			expect(snap.rows.find((row) => row.id === "kit:k-home")?.name).toBe("Demo Kit");
			ctrl.dispose();
		});
	});

	describe("Sketchpad kit virtual file system", () => {
		it("projects kit entities as hierarchical vfs rows", async () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const designId = "66666666-7777-8888-9999-aaaaaaaaaaaa";
			const folderId = "bbbbbbbb-cccc-dddd-eeee-ffffffffffff";
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({
					id: kitId,
					name: "VFS Kit",
					types: [{ id: typeId, name: "Base" }],
					designs: [{ id: designId, name: "Tower", pieces: [] }],
					folders: [{ id: folderId, path: "/inbox" }],
				} as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			platform.uri = `/kits/${kitId}`;
			const vfs = new SketchpadAppVirtualFileSystem(SKETCHPAD_KIT_APP_ID, platform);
			await new Promise<void>((resolve) => setTimeout(resolve, 0));
			const snap = vfs.buildSnapshot();
			expect(snap.rows.some((row) => row.id === kitId && row.fileNodeKindId === "kit")).toBe(false);
			expect(snap.rows.some((row) => row.id === typeId)).toBe(true);
			expect(snap.rows.some((row) => row.id === designId && row.navigateUri?.includes(`/design/${designId}`))).toBe(true);
			expect(snap.rows.some((row) => row.id === folderId)).toBe(true);
			ctrl.dispose();
		});

		it("rebinds kit vfs expansion when switching between open kits", async () => {
			const kitA = "aaaaaaaa-bbbb-cccc-dddd-111111111111";
			const kitB = "aaaaaaaa-bbbb-cccc-dddd-222222222222";
			const typeA = "11111111-2222-3333-4444-aaaaaaaaaaaa";
			const typeB = "22222222-3333-4444-5555-bbbbbbbbbbbb";
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(kitA, new InMemoryComposeKitStore({ id: kitA, name: "Kit A", types: [{ id: typeA, name: "Type A" }] } as Kit));
			ctrl.registerKitStore(kitB, new InMemoryComposeKitStore({ id: kitB, name: "Kit B", types: [{ id: typeB, name: "Type B" }] } as Kit));
			ctrl.navigateTo(`/kits/${kitA}`);
			platform.uri = `/kits/${kitA}`;
			const vfs = new SketchpadAppVirtualFileSystem(SKETCHPAD_KIT_APP_ID, platform);
			await new Promise<void>((resolve) => setTimeout(resolve, 0));
			let snap = vfs.buildSnapshot();
			expect(snap.rows.some((row) => row.id === typeA)).toBe(true);
			ctrl.navigateTo(`/kits/${kitB}`);
			platform.uri = `/kits/${kitB}`;
			vfs.refresh();
			await new Promise<void>((resolve) => setTimeout(resolve, 0));
			snap = vfs.buildSnapshot();
			expect(snap.rows.some((row) => row.id === typeB)).toBe(true);
			expect(snap.rows.some((row) => row.id === typeA)).toBe(false);
			ctrl.dispose();
		});

		it("keeps kit vfs collapsed when opening a kit and through kit wires prepare", async () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-444444444444";
			const folderId = "cccccccc-dddd-eeee-ffff-111111111111";
			const typeId = "dddddddd-eeee-ffff-1111-222222222222";
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({
					id: kitId,
					name: "Collapsed Kit",
					types: [{ id: typeId, name: "Base" }],
					folders: [{ id: folderId, path: "/inbox" }],
				} as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			const scope = sketchpadVfsScope(SKETCHPAD_KIT_APP_ID);
			ctrl.syncVirtualFileSystemRoute(scope, kitId);
			await ctrl.prepareKitWiresVfsForTopology(kitId);
			let snap = ctrl.buildVirtualFileSystemModel(scope);
			expect(snap.rows.every((row) => !row.expanded)).toBe(true);
			expect(ctrl.expandedStore(scope).getSnapshot()).toEqual([]);
			ctrl.dispose();
		});

		it("keeps manually expanded kit vfs folders through kit wires prepare", async () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-333333333333";
			const folderId = "bbbbbbbb-cccc-dddd-eeee-ffffffffffff";
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({
					id: kitId,
					name: "Expand Kit",
					folders: [{ id: folderId, path: "/representations" }],
				} as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			const scope = sketchpadVfsScope(SKETCHPAD_KIT_APP_ID);
			ctrl.syncVirtualFileSystemRoute(scope, kitId);
			ctrl.expandedStore(scope).toggle(folderId);
			await ctrl.prepareKitWiresVfsForTopology(kitId);
			expect(ctrl.expandedStore(scope).getSnapshot()).toContain(folderId);
			const snap = ctrl.buildVirtualFileSystemModel(scope);
			expect(snap.rows.some((row) => row.id === folderId && row.expanded)).toBe(true);
			ctrl.dispose();
		});
	});

	describe("sketchpadKitPuzzle2dFixtureFromKit", () => {
		it("materializes type and design nodes", () => {
			const kit = {
				id: "k",
				types: [{ id: "t1", name: "Window" }],
				designs: [{ id: "d1", name: "Plan", pieces: [{ id: "p1", type: { id: "t1" } }] }],
			} as Kit;
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(kit);
			expect(fixture.nodes.some((n) => n.id === "type:t1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "design:d1")).toBe(true);
			expect(fixture.edges.length).toBeGreaterThan(0);
		});

		it("materializes ports qualities files folders authors", () => {
			const kit = {
				id: "k",
				types: [
					{
						id: "t1",
						name: "Window",
						connectors: [{ id: "c1", port: { id: "p1", label: "Frame" } }],
						ports: [{ id: "p2", label: "Glass" }],
					},
				],
				qualities: [{ id: "q1", key: "Thermal", value: "1.2" }],
				files: [{ id: "f1", url: "files/mesh.glb" }],
				folders: [{ id: "fo1", path: "assets/models" }],
				authors: [{ id: "a1", name: "Ada" }],
			} as Kit;
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(kit);
			expect(fixture.nodes.some((n) => n.id === "port:p1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "port:p2")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "quality:q1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "file:f1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "folder:fo1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "author:a1")).toBe(true);
			expect(fixture.edges.some((e) => e.id === "ref-port:p1-type:t1")).toBe(true);
		});
	});

	describe("sketchpadCreatePortGroupMap", () => {
		it("unions ports linked by compatiblePorts", () => {
			const groups = sketchpadCreatePortGroupMap([
				{ id: "p1", compatiblePorts: [{ id: "p2" }] },
				{ id: "p2", compatiblePorts: [{ id: "p1" }] },
				{ id: "p3" },
			]);
			expect(groups.get("p1")).toBe(groups.get("p2"));
			expect(groups.get("p3")).toBe("p3");
		});
	});

	describe("sketchpadKitPuzzle2dFixtureFromKit type compat", () => {
		it("draws type adjacency edges for compatible ports", () => {
			const kit = {
				id: "k",
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1", compatiblePorts: [{ id: "p2" }] } }] },
					{ id: "t2", connectors: [{ port: { id: "p2", compatiblePorts: [{ id: "p1" }] } }] },
				],
			} as Kit;
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(kit);
			expect(fixture.edges.some((e) => e.id === "compat-type:t1-type:t2")).toBe(true);
		});
	});

	describe("sketchpadCollectKitPorts", () => {
		it("deduplicates ports from connectors and type ports", () => {
			const kit = {
				id: "k",
				types: [
					{
						id: "t1",
						connectors: [{ port: { id: "p1", label: "A" } }],
						ports: [{ id: "p1", label: "A" }, { id: "p2", code: "B" }],
					},
				],
			} as Kit;
			const ports = sketchpadCollectKitPorts(kit);
			expect(ports).toHaveLength(2);
			expect(ports.map((p) => p.id).sort()).toEqual(["p1", "p2"]);
		});
	});

	describe("sketchpadDesignVolumeFixtureFromDesign", () => {
		it("creates placeholder mesh objects at identity with local vortices", () => {
			const design = {
				id: "d",
				pieces: [{ id: "p1", name: "A", blueprint: { id: "t1" } }],
			} as Design;
			const kit = {
				id: "k",
				types: [{ id: "t1", connectors: [{ id: "c1", point: { x: 1, y: 2, z: 3 }, direction: { x: 0, y: 0, z: 1 }, t: 0 }] }],
			} as Kit;
			const volume = sketchpadDesignVolumeFixtureFromDesign(design, kit);
			expect(volume.objects).toHaveLength(1);
			expect(volume.objects[0]?.origin).toEqual([0, 0, 0]);
			expect(volume.objects[0]?.vortices[0]?.position).toEqual([1, 2, 3]);
		});

		it("emits relative fixtures flattened through puzzle 5d", async () => {
			const { compose5d, prepareTopologyModel } = await import("@semio-tech/puzzle-5d-react");
			const design = {
				id: "d",
				pieces: [
					{
						id: "root",
						blueprint: { id: "type-a" },
						position: { center: { u: 1, v: 2 }, plane: { origin: { x: 1, y: 2, z: 3 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } } },
					},
					{ id: "child", blueprint: { id: "type-a" } },
				],
				connections: [
					{
						id: "link-1",
						u: 0.5,
						v: 0.25,
						parent: { piece: { id: "root" }, connector: { id: "conn-a" } },
						child: { piece: { id: "child" }, connector: { id: "conn-a" } },
					},
				],
			} as Design;
			const kit = {
				id: "k",
				types: [
					{
						id: "type-a",
						connectors: [{ id: "conn-a", point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, t: 0 }],
					},
				],
			} as Kit;
			const flat = sketchpadDesignPuzzle2dFixtureFromDesign(design, kit);
			const volume = sketchpadDesignVolumeFixtureFromDesign(design, kit);
			const prepared = prepareTopologyModel(compose5d(flat, volume));
			const child = prepared.parts.find((part) => part.id === "child");
			expect(child?.["3d"]?.origin?.[0]).toBeCloseTo(1, 2);
			expect(child?.["2d"]?.x).toBeCloseTo(1.5 * SKETCHPAD_TOPOLOGY_ICON_WIDTH, 2);
		});

		it("frames diagram camera placeholder before flatten in host fixture", () => {
			const design = {
				id: "d",
				pieces: [{ id: "a" }, { id: "b" }],
			} as Design;
			const flat = sketchpadDesignPuzzle2dFixtureFromDesign(design);
			expect(flat.camera.zoom).toBe(1);
		});
	});

	describe("sketchpadPlaneAxesToQuaternion", () => {
		it("returns identity for world axes", () => {
			const q = sketchpadPlaneAxesToQuaternion({
				origin: { x: 0, y: 0, z: 0 },
				xAxis: { x: 1, y: 0, z: 0 },
				yAxis: { x: 0, y: 1, z: 0 },
			});
			expect(q[3]).toBeCloseTo(1, 4);
		});
	});

	describe("sketchpadKitFileUrlById", () => {
		it("resolves kit-relative file paths against registered asset base", async () => {
			const kit = {
				id: "asset-base-kit",
				files: [{ id: "f1", path: "files/mesh.glb" }],
			} as Kit;
			await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore("asset-base-kit", new InMemoryComposeKitStore(kit), {
				assetBaseUrl: "/fixture/kit/dev/metabolism/wip/initialKit",
			});
			expect(sketchpadKitFileUrlById(kit).get("f1")).toBe("/fixture/kit/dev/metabolism/wip/initialKit/files/mesh.glb");
			ctrl.dispose();
			sketchpadShellControllerSingleton = null;
			sketchpadPlatformSingleton = null;
			sketchpadPlatformReady = null;
		});

		it("maps parent-relative representation paths to /meshes", () => {
			const kit = {
				id: "k",
				files: [{ id: "f1", path: "../../representation/bridge.glb" }],
			} as Kit;
			expect(sketchpadKitFileUrlById(kit).get("f1")).toBe("/mesh/bridge.glb");
		});
	});

	describe("sketchpadApplyPuzzle2dSelection", () => {
		it("stores design piece selection on shell", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			ctrl.navigateTo(`/kits/${kitId}/design/${designId}`);
			sketchpadApplyPuzzle2dSelection(sketchpadDesignDiagramInstanceId(kitId, designId), ["piece-a", "piece-b"], ctrl);
			expect(ctrl.routeSelection.pieceIds).toEqual(["piece-a", "piece-b"]);
			ctrl.dispose();
		});

		it("stores design piece selection from scene volume object ids", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			ctrl.navigateTo(`/kits/${kitId}/design/${designId}/scene`);
			sketchpadApplyPuzzle2dSelection(sketchpadDesignSceneInstanceId(kitId, designId), ["piece-x"], ctrl);
			expect(ctrl.routeSelection.pieceIds).toEqual(["piece-x"]);
			ctrl.dispose();
		});

		it("maps volume attraction ids to connection selection", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			const kit = {
				id: kitId,
				name: "K",
				designs: [
					{
						id: designId,
						name: "D",
						pieces: [{ id: "piece-a", name: "A" }],
						connections: [{ id: "conn-1", parent: { piece: { id: "piece-a" }, connector: { id: "c1" } }, child: { piece: { id: "piece-a" }, connector: { id: "c2" } } }],
					},
				],
			} as Kit;
			ctrl.registerKitStore(kitId, new InMemoryComposeKitStore(kit));
			ctrl.navigateTo(`/kits/${kitId}/design/${designId}`);
			sketchpadApplyPuzzle2dSelection(sketchpadDesignSceneInstanceId(kitId, designId), ["piece-a", "conn-1"], ctrl);
			expect(ctrl.routeSelection.pieceIds).toEqual(["piece-a"]);
			expect(ctrl.routeSelection.connectionIds).toEqual(["conn-1"]);
			ctrl.dispose();
		});

		it("stores kit wires selection on single click without navigating", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			ctrl.registerKitStore(kitId, new InMemoryComposeKitStore({ id: kitId, name: "K", types: [{ id: typeId, name: "T" }], designs: [] } as Kit));
			ctrl.navigateTo(`/kits/${kitId}`);
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), kitId);
			sketchpadApplyPuzzle2dSelection(sketchpadKitWiresInstanceId(kitId), [typeId], ctrl);
			expect(ctrl.routeSelection.kitWiresNodeIds).toEqual([typeId]);
			expect(ctrl.navigationPath).toBe(
				`/kits/${kitId}${sketchpadRouteSelectionUriFilters({ pieceIds: [], connectionIds: [], kitWiresNodeIds: [typeId], kitWiresHoveredNodeId: null })}`,
			);
			ctrl.dispose();
		});

		it("syncs kit wires selection to vfs component snapshot without manual refresh", async () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({ id: kitId, name: "K", types: [{ id: typeId, name: "Column" }], designs: [] } as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			platform.uri = `/kits/${kitId}`;
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), kitId);
			const vfs = platform.getComponent(virtualFileSystemSurfaceId(SKETCHPAD_KIT_APP_ID));
			expect(vfs?.getSnapshot().selectedRowIds ?? []).toEqual([]);
			platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "puzzle5dSelection", {
				instanceId: sketchpadKitWiresInstanceId(kitId),
				puzzle2dIds: [typeId],
			});
			expect(vfs?.getSnapshot().selectedRowIds).toEqual([typeId]);
			ctrl.dispose();
		});

		it("clears kit wires selection on vfs and wires surfaces without manual refresh", async () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({ id: kitId, name: "K", types: [{ id: typeId, name: "Column" }], designs: [] } as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			platform.uri = `/kits/${kitId}`;
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), kitId);
			platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "puzzle5dSelection", {
				instanceId: sketchpadKitWiresInstanceId(kitId),
				puzzle2dIds: [typeId],
			});
			platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "setVirtualFileSystemRowSelection", {
				appId: SKETCHPAD_KIT_APP_ID,
				surfaceId: virtualFileSystemSurfaceId(SKETCHPAD_KIT_APP_ID),
				rowIds: [],
			});
			const vfs = platform.getComponent(virtualFileSystemSurfaceId(SKETCHPAD_KIT_APP_ID));
			const wires = platform.getComponent(SKETCHPAD_SURFACE_KIT_WIRES);
			expect(vfs?.getSnapshot().selectedRowIds ?? []).toEqual([]);
			expect(wires?.getSnapshot().puzzle2dSelection ?? []).toEqual([]);
			ctrl.dispose();
		});

		it("merges kit wires preselect preview into effective vfs selection ids", () => {
			expect(sketchpadKitWiresEffectiveSelectionIds([], { ids: ["a", "b"], removedIds: [] })).toEqual(["a", "b"]);
			expect(sketchpadKitWiresEffectiveSelectionIds(["a"], { ids: ["b"], removedIds: [] })).toEqual(["a", "b"]);
			expect(sketchpadKitWiresEffectiveSelectionIds(["a", "b"], { ids: [], removedIds: ["a"] })).toEqual(["b"]);
			expect(sketchpadKitWiresEffectiveSelectionIds(["a"], { ids: [], removedIds: [] })).toEqual(["a"]);
		});

		it("syncs kit wires rectangle preselect to vfs without manual refresh", async () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeA = "11111111-2222-3333-4444-555555555555";
			const typeB = "22222222-3333-4444-5555-666666666666";
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({
					id: kitId,
					name: "K",
					types: [
						{ id: typeA, name: "A" },
						{ id: typeB, name: "B" },
					],
					designs: [],
				} as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			platform.uri = `/kits/${kitId}`;
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), kitId);
			const vfs = platform.getComponent(virtualFileSystemSurfaceId(SKETCHPAD_KIT_APP_ID));
			expect(vfs?.getSnapshot().selectedRowIds ?? []).toEqual([]);
			platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "puzzle5dPreselect", {
				instanceId: sketchpadKitWiresInstanceId(kitId),
				preselect: { ids: [typeA, typeB], removedIds: [] },
			});
			expect(vfs?.getSnapshot().selectedRowIds).toEqual([typeA, typeB]);
			platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "puzzle5dPreselect", {
				instanceId: sketchpadKitWiresInstanceId(kitId),
				preselect: { ids: [], removedIds: [] },
			});
			expect(vfs?.getSnapshot().selectedRowIds ?? []).toEqual([]);
			ctrl.dispose();
		});

		it("syncs kit wires hover between vfs and wires surfaces without manual refresh", async () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({ id: kitId, name: "K", types: [{ id: typeId, name: "Column" }], designs: [] } as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			platform.uri = `/kits/${kitId}`;
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), kitId);
			platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "virtualFileSystemRowHover", {
				appId: SKETCHPAD_KIT_APP_ID,
				surfaceId: virtualFileSystemSurfaceId(SKETCHPAD_KIT_APP_ID),
				rowId: typeId,
			});
			const vfs = platform.getComponent(virtualFileSystemSurfaceId(SKETCHPAD_KIT_APP_ID));
			const wires = platform.getComponent(SKETCHPAD_SURFACE_KIT_WIRES);
			expect(vfs?.getSnapshot().hoveredRowId).toBe(typeId);
			expect(wires?.getSnapshot().puzzle2dHoveredId).toBe(typeId);
			platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "puzzle5dHover", {
				instanceId: sketchpadKitWiresInstanceId(kitId),
				nodeId: null,
			});
			expect(vfs?.getSnapshot().hoveredRowId).toBeNull();
			expect(wires?.getSnapshot().puzzle2dHoveredId).toBeNull();
			ctrl.dispose();
		});

		it("navigates kit wires on double-click activate", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			ctrl.registerKitStore(kitId, new InMemoryComposeKitStore({ id: kitId, name: "K", types: [{ id: typeId, name: "T" }], designs: [] } as Kit));
			ctrl.navigateTo(`/kits/${kitId}`);
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), kitId);
			sketchpadNavigateFromWiresSelection(sketchpadKitWiresInstanceId(kitId), [typeId], ctrl);
			expect(ctrl.navigationPath).toBe(`/kits/${kitId}/type/${typeId}`);
			ctrl.dispose();
		});
	});

	describe("sketchpadKitVfsChildren", () => {
		it("lists files under a folder parent", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const folderId = "22222222-3333-4444-5555-666666666666";
			const fileId = "33333333-4444-5555-6666-777777777777";
			const kit = {
				id: kitId,
				folders: [{ id: folderId, path: "/docs" }],
				files: [{ id: fileId, name: "readme.txt", folder: { id: folderId } }],
			} as Kit;
			const rows = sketchpadKitVfsChildren(kit, folderId);
			expect(rows).toHaveLength(1);
			expect(rows[0]?.id).toBe(fileId);
			expect(rows[0]?.fileNodeKindId).toBe("file");
		});

		it("keeps folder files off kit root when typologies are present", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const folderId = "22222222-3333-4444-5555-666666666666";
			const fileId = "33333333-4444-5555-6666-777777777777";
			const kit = {
				id: kitId,
				typologies: [{ id: "topo-1", name: "Capsule", types: [], designs: [] }],
				folders: [{ id: folderId, name: "representations" }],
				files: [{ id: fileId, name: "tower.glb", folder: { id: folderId } }],
			} as Kit;
			const rootRows = sketchpadKitVfsChildren(kit, kitId);
			expect(rootRows.some((row) => row.id === fileId)).toBe(false);
			expect(rootRows.find((row) => row.id === folderId)?.name).toBe("representations");
			const folderRows = sketchpadKitVfsChildren(kit, folderId);
			expect(folderRows).toHaveLength(1);
			expect(folderRows[0]?.id).toBe(fileId);
		});

		it("resolves folderId-only file rows under the folder, not at kit root", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const folderId = "22222222-3333-4444-5555-666666666666";
			const fileId = "33333333-4444-5555-6666-777777777777";
			const kit = {
				id: kitId,
				folders: [{ id: folderId, path: "/asset" }],
				files: [{ id: fileId, name: "mesh.glb", folderId }],
			} as Kit;
			expect(sketchpadKitVfsChildren(kit, kitId).some((row) => row.id === fileId)).toBe(false);
			expect(sketchpadKitVfsChildren(kit, folderId).map((row) => row.id)).toEqual([fileId]);
		});

		it("reads bundle-shaped files and folders blocks", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const folderId = "22222222-3333-4444-5555-666666666666";
			const fileId = "33333333-4444-5555-6666-777777777777";
			const kit = {
				id: kitId,
				folders: { items: [{ id: folderId, name: "representations" }] },
				files: { items: [{ id: fileId, name: "mesh.glb", folder: { id: folderId } }] },
			} as unknown as Kit;
			const rootRows = sketchpadKitVfsChildren(kit, kitId);
			expect(rootRows.some((row) => row.id === fileId)).toBe(false);
			expect(sketchpadKitVfsChildren(kit, folderId)).toHaveLength(1);
		});

		it("hides representation backing blobs from kit root", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const folderId = "22222222-3333-4444-5555-666666666666";
			const repFileId = "33333333-4444-5555-6666-777777777777";
			const hashBlobId = "44444444-5555-6666-7777-888888888888";
			const looseFileId = "55555555-6666-7777-8888-999999999999";
			const kit = {
				id: kitId,
				folders: [{ id: folderId, path: "/asset" }],
				files: [
					{ id: repFileId, name: "chair.glb" },
					{ id: hashBlobId, name: "e5267da44d" },
					{ id: looseFileId, name: "notes.md" },
				],
				types: [
					{
						id: "type-1",
						name: "Base",
						representations: [{ id: "rep-1", name: "Mesh", file: { id: repFileId } }],
					},
				],
			} as Kit;
			const rootRows = sketchpadKitVfsChildren(kit, kitId);
			expect(rootRows.some((row) => row.id === repFileId)).toBe(false);
			expect(rootRows.some((row) => row.id === hashBlobId)).toBe(false);
			expect(rootRows.some((row) => row.id === looseFileId)).toBe(true);
		});

		it("lists only kit-root folders when nested folders exist", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const rootFolderId = "22222222-3333-4444-5555-666666666666";
			const nestedFolderId = "33333333-4444-5555-6666-777777777777";
			const kit = {
				id: kitId,
				folders: [
					{ id: rootFolderId, path: "/docs" },
					{ id: nestedFolderId, path: "/doc/nested", parent: { id: rootFolderId } },
				],
			} as Kit;
			const rootRows = sketchpadKitVfsChildren(kit, kitId).filter((row) => row.fileNodeKindId === "folder");
			expect(rootRows.map((row) => row.id)).toEqual([rootFolderId]);
			const nestedRows = sketchpadKitVfsChildren(kit, rootFolderId).filter((row) => row.fileNodeKindId === "folder");
			expect(nestedRows.map((row) => row.id)).toEqual([nestedFolderId]);
		});
	});

	describe("sketchpadPathFromWiresNodeId", () => {
		it("maps kit wires nodes to routes", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			const designId = "22222222-3333-4444-5555-666666666666";
			expect(sketchpadPathFromWiresNodeId(kitId, typeId, "type")).toBe(`/kits/${kitId}/type/${typeId}`);
			expect(sketchpadPathFromWiresNodeId(kitId, designId, "design")).toBe(`/kits/${kitId}/design/${designId}`);
		});
	});

	describe("SketchpadShellController topology", () => {
		it("upserts topology store for kit wires surface", async () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({
					id: kitId,
					name: "K",
					types: [{ id: typeId, name: "T" }],
					designs: [],
				} as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), kitId);
			ctrl.syncTopologyForSurface(SKETCHPAD_SURFACE_KIT_WIRES, { kitId, designId: null, typeId: null });
			await new Promise((resolve) => setTimeout(resolve, 0));
			const topo = ctrl.getStore(platformTopologyStoreId(sketchpadKitWiresInstanceId(kitId))) as PlatformTopologyStore;
			expect(topo).toBeDefined();
			const flat = topo!.getSnapshot().flat as { schema?: string; edges?: readonly unknown[] };
			expect(flat.schema).toBe("puzzle.2d.fixture/v1");
			expect(flat.edges?.length).toBeGreaterThan(0);
			ctrl.dispose();
		});

		it("concurrent kit wires syncs share one vfs prepare and emit containment edges", async () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const typeId = "11111111-2222-3333-4444-555555555555";
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({
					id: kitId,
					name: "K",
					types: [{ id: typeId, name: "T" }],
					designs: [],
				} as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), kitId);
			ctrl.syncKitWiresTopology(kitId);
			ctrl.syncKitWiresTopology(kitId);
			await new Promise((resolve) => setTimeout(resolve, 0));
			const topo = ctrl.getStore(platformTopologyStoreId(sketchpadKitWiresInstanceId(kitId))) as PlatformTopologyStore;
			const flat = topo!.getSnapshot().flat as { edges?: readonly { source?: string; target?: string; edgeKind?: string }[] };
			expect(flat.edges?.some((edge) => edge.source === kitId && edge.target === typeId)).toBe(true);
			ctrl.dispose();
		});

		it("matches wires identities to vfs rows when a typology is expanded", async () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-555555555555";
			const typologyId = "bbbbbbbb-cccc-dddd-eeee-111111111111";
			const typeId = "cccccccc-dddd-eeee-ffff-222222222222";
			const designId = "dddddddd-eeee-ffff-1111-333333333333";
			const scope = sketchpadVfsScope(SKETCHPAD_KIT_APP_ID);
			ctrl.registerKitStore(
				kitId,
				new InMemoryComposeKitStore({
					id: kitId,
					name: "Typology Kit",
					typologies: [
						{
							id: typologyId,
							name: "Core",
							types: [{ id: typeId, name: "Window" }],
							designs: [{ id: designId, name: "Plan" }],
						},
					],
				} as Kit),
			);
			ctrl.navigateTo(`/kits/${kitId}`);
			ctrl.syncVirtualFileSystemRoute(scope, kitId);
			ctrl.run("toggleVirtualFileSystemExpand", { ...scope, nodeId: typologyId });
			await ctrl.prepareKitWiresVfsForTopology(kitId);
			await new Promise((resolve) => setTimeout(resolve, 0));
			const root = ctrl.getRoot(scope);
			const visible = sketchpadKitWiresVisibleNodes(root, ctrl.visibleVirtualFileSystemNodes(scope));
			const visibleIds = visible.map((node) => node.id).sort();
			const topo = ctrl.getStore(platformTopologyStoreId(sketchpadKitWiresInstanceId(kitId))) as PlatformTopologyStore;
			const flat = topo!.getSnapshot().flat as { nodes?: readonly { id?: string }[] };
			const wireNodeIds = (flat.nodes ?? []).map((node) => String(node.id)).sort();
			expect(visibleIds).toContain(typologyId);
			expect(visibleIds).toContain(typeId);
			expect(visibleIds).toContain(designId);
			expect(wireNodeIds).toEqual(visibleIds);
			ctrl.dispose();
		});
	});
}
//#endregion 🧪Tests

//#region 🧪E2E
if (typeof __COMPOSE_SKETCHPAD_RUN_EMBEDDED_TESTS__ !== "undefined" && __COMPOSE_SKETCHPAD_RUN_EMBEDDED_TESTS__) {
	const { test, expect } = await import("@playwright/test");
	test.describe("sketchpad platform", () => {
		async function openSketchpadCommandPalette(page: import("@playwright/test").Page): Promise<void> {
			const searchToggle = page.locator('[id="ui.search.toggle"]');
			await expect(searchToggle).toBeVisible({ timeout: 30_000 });
			await searchToggle.click();
			const dialog = page.getByRole("dialog");
			await expect(dialog).toBeVisible({ timeout: 10_000 });
			await expect(dialog.getByPlaceholder("Search...")).toBeVisible({ timeout: 10_000 });
		}

		async function openPreloadedKitFromHome(page: import("@playwright/test").Page): Promise<void> {
			await page.goto("/", { waitUntil: "networkidle" });
			await expect(page.getByRole("columnheader", { name: "Name" })).toBeVisible({ timeout: 120_000 });
			const kitRow = page.locator("tr[data-row-id]").filter({ hasText: /metabolism/i });
			await expect(kitRow).toBeVisible({ timeout: 120_000 });
			await kitRow.dblclick();
			await expect(page).toHaveURL(/\/kits\/[0-9a-f-]{36}/i, { timeout: 120_000 });
		}

		test("home table mounts on root", async ({ page }) => {
			await page.goto("/", { waitUntil: "networkidle" });
			await expect(page.getByRole("columnheader", { name: "Name" })).toBeVisible({ timeout: 120_000 });
		});

		test("workbench panel is present when platform loads", async ({ page }) => {
			await page.goto("/", { waitUntil: "networkidle" });
			const workbenchToggle = page.locator('[id="ui.panelToggle.workbench"]');
			await expect(workbenchToggle).toBeVisible({ timeout: 120_000 });
		});

		test("command palette lists kit import commands", async ({ page }) => {
			await page.goto("/", { waitUntil: "networkidle" });
			await expect(page.getByRole("columnheader", { name: "Name" })).toBeVisible({ timeout: 120_000 });
			await openSketchpadCommandPalette(page);
			const dialog = page.getByRole("dialog");
			await dialog.getByPlaceholder("Search...").fill("import kit");
			await expect(dialog.getByText("Import kit from file")).toBeVisible({ timeout: 30_000 });
		});

		test("preloaded kit navigates to kit vfs from home", async ({ page }) => {
			await openPreloadedKitFromHome(page);
			await expect(page.getByRole("columnheader", { name: "Kind" })).toBeVisible({ timeout: 120_000 });
			await expect(page.getByText("Nakagin Capsule Tower")).toBeVisible({ timeout: 120_000 });
		});

		test("docs route renders MDX getting started page", async ({ page }) => {
			await page.goto("/doc/getting-started/index", { waitUntil: "networkidle" });
			await expect(page.getByText("Welcome to the Getting Started section")).toBeVisible({ timeout: 120_000 });
		});

		test("feedback route shows feedback form", async ({ page }) => {
			await page.goto("/feedback", { waitUntil: "networkidle" });
			await expect(page.getByPlaceholder("What should we know?")).toBeVisible({ timeout: 120_000 });
			await expect(page.getByRole("button", { name: "Send feedback" })).toBeVisible({ timeout: 30_000 });
		});

		test("home vfs double-click opens preloaded kit row", async ({ page }) => {
			await page.goto("/", { waitUntil: "networkidle" });
			await expect(page.getByRole("columnheader", { name: "Name" })).toBeVisible({ timeout: 120_000 });
			const kitRow = page.locator("tr[data-row-id]").filter({ hasText: /metabolism/i });
			await expect(kitRow).toBeVisible({ timeout: 120_000 });
			await kitRow.dblclick();
			await expect(page).toHaveURL(/\/kits\/[0-9a-f-]{36}/i, { timeout: 120_000 });
			await page.goBack({ waitUntil: "networkidle" });
			await expect(page.getByRole("columnheader", { name: "Name" })).toBeVisible({ timeout: 120_000 });
			await expect(kitRow).toBeVisible({ timeout: 120_000 });
			await kitRow.dblclick();
			await expect(page).toHaveURL(/\/kits\/[0-9a-f-]{36}/i, { timeout: 120_000 });
		});

		test("kit vfs selects design row on click and navigates on double-click", async ({ page }) => {
			await openPreloadedKitFromHome(page);
			await expect(page.getByText("Nakagin Capsule Tower")).toBeVisible({ timeout: 120_000 });
			const kitUrl = page.url();
			const designRow = page.locator("tr[data-row-id]").filter({ hasText: "Nakagin Capsule Tower" });
			await designRow.click();
			await expect(page).toHaveURL(kitUrl, { timeout: 5_000 });
			await expect(designRow).toHaveClass(/bg-active-base/);
			await designRow.dblclick();
			await expect(page).toHaveURL(/\/design\/[0-9a-f-]{36}/i, { timeout: 120_000 });
		});

		test("type route opens representation tab stack", async ({ page }) => {
			await openPreloadedKitFromHome(page);
			await expect(page.getByRole("columnheader", { name: "Kind" })).toBeVisible({ timeout: 120_000 });
			const baseRow = page.locator("tr[data-row-id]").filter({ has: page.getByRole("cell", { name: "Base", exact: true }) });
			await expect(baseRow).toBeVisible({ timeout: 120_000 });
			await baseRow.click();
			await expect(page).not.toHaveURL(/\/type\/[0-9a-f-]{36}/i);
			await expect(baseRow).toHaveClass(/bg-active-base/);
			await baseRow.dblclick();
			await expect(page).toHaveURL(/\/type\/[0-9a-f-]{36}/i, { timeout: 120_000 });
			await expect(page.getByText("Mesh unavailable")).toHaveCount(0, { timeout: 60_000 });
			await expect(page.getByText("Topology loading")).toHaveCount(0, { timeout: 60_000 });
		});
	});
}
//#endregion 🧪E2E
