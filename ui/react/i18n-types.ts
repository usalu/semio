// #region 🪁I18n Types


// Domain-neutral UI translation bundles (settings, tooltip, generic shell `ui.*` ids).
// Product-specific bundles (e.g. semio sketchpad) register via {@link registerUiTranslationBundles}.

/** @emoji 🪁 Supported UI locale codes. */
export type UiLocale = "en" | "de";

/** @emoji 🪁 Expertise-specific label pair. */
export type UiLabelPair = { readonly normal: string; readonly beginner: string };

/** @emoji 🪁 Translation leaf with optional manual, tutorial, and hotkey metadata. */
export type UiLabelValue = {
  readonly label: UiLabelPair;
  readonly manual?: string;
  readonly tutorial?: string;
  readonly hotkey?: string;
};

/** @emoji 🪁 Toolbar parent category ids mirrored from {@link @framework/core AppToolCategory}. */
export type UiToolbarParentCategory =
  | "history"
  | "hand"
  | "selection"
  | "lasso"
  | "filter"
  | "open"
  | "save"
  | "transfer"
  | "transform"
  | "create"
  | "view"
  | "actions"
  | "settings";

/** @emoji 🪁 i18n key for a toolbar parent category toggle. */
export type UiToolbarParentKey = `ui.toolbar.parent.${UiToolbarParentCategory}`;

type UiToolbarParentEntries = { readonly [K in UiToolbarParentCategory]: UiLabelValue };

type DeepUiTranslationKeys<T, Prefix extends string = ""> = T extends UiLabelValue
  ? Prefix extends ""
    ? never
    : Prefix
  : T extends string
    ? Prefix extends ""
      ? never
      : Prefix
    : T extends number | boolean | null | undefined
      ? never
      : T extends readonly unknown[]
        ? never
        : {
            [K in keyof T & string]: DeepUiTranslationKeys<T[K], Prefix extends "" ? K : `${Prefix}.${K}`>;
          }[keyof T & string];

/** @emoji 🪁 Domain-neutral chrome translation tree (settings, tooltip, `ui.*`). */
export type UiTranslationSchema = {
  readonly ui: {
    readonly nav: {
      readonly back: UiLabelValue;
      readonly forward: UiLabelValue;
      readonly up: UiLabelValue;
    };
    readonly search: {
      readonly toggle: UiLabelValue;
      readonly close: UiLabelValue;
    };
    readonly panelToggle: {
      readonly workbench: UiLabelValue;
    };
    readonly toolbar: {
      readonly group: {
        readonly parent: UiLabelValue;
      };
      readonly parent: UiToolbarParentEntries;
    };
    readonly common: {
      readonly mixedValues: UiLabelValue;
    };
    readonly docs: {
      readonly navigation: {
        readonly previous: UiLabelValue;
        readonly next: UiLabelValue;
      };
    };
    readonly ring: {
      readonly demo: UiLabelValue;
    };
    readonly stepper: {
      readonly demo: UiLabelValue;
    };
  };
  readonly settings: {
    readonly layout: {
      readonly normal: UiLabelValue;
      readonly desktop: UiLabelValue;
      readonly tablet: UiLabelValue;
      readonly mobile: UiLabelValue;
    };
    readonly compact: UiLabelValue;
    readonly mode: {
      readonly dev: UiLabelValue;
      readonly user: UiLabelValue;
      readonly beginner: UiLabelValue;
      readonly normal: UiLabelValue;
    };
    readonly expertise: {
      readonly beginner: UiLabelValue;
      readonly normal: UiLabelValue;
      readonly expert: UiLabelValue;
    };
  };
  readonly tooltip: {
    readonly manual: UiLabelValue;
    readonly tutorial: UiLabelValue;
  };
};

/** @emoji 🪁 Dot-path union of keys in {@link UiTranslationSchema}. */
export type UiTranslationKey = DeepUiTranslationKeys<UiTranslationSchema>;

/** @emoji 🪁 Compile-time check that every toolbar category has a chrome translation key. */
export type AssertUiToolbarParentKeysCovered<Categories extends string> = {
  readonly [K in Categories]: `ui.toolbar.parent.${K & UiToolbarParentCategory}` extends UiTranslationKey ? true : false;
}[Categories] extends true
  ? true
  : false;

/** @emoji 🪁 Typed translate function for domain-neutral chrome keys. */
export type UiTranslateFn = <K extends UiTranslationKey>(key: K, options?: Record<string, unknown>) => unknown;

/** @emoji 🪁 Shared UI i18n port (wraps i18next; do not import i18next outside this bundle). */
export interface UiI18nPort {
  readonly t: UiTranslateFn;
  changeLanguage(locale: UiLocale): Promise<unknown>;
  readonly language: string | undefined;
  readonly resolvedLanguage: string | undefined;
  readonly isInitialized: boolean;
}

declare module "i18next" {
  interface CustomTypeOptions {
    defaultNS: "translation";
    resources: {
      readonly en: { readonly translation: UiTranslationSchema };
      readonly de: { readonly translation: UiTranslationSchema };
    };
  }
}

const uiToolbarParentDe: UiToolbarParentEntries = {
  history: { label: { normal: "Verlauf", beginner: "Verlauf" } },
  hand: { label: { normal: "Hand", beginner: "Hand" } },
  selection: { label: { normal: "Auswahl", beginner: "Auswahl" } },
  lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
  filter: { label: { normal: "Filter", beginner: "Filter" } },
  open: { label: { normal: "Oeffnen", beginner: "Oeffnen" } },
  save: { label: { normal: "Speichern", beginner: "Speichern" } },
  transfer: { label: { normal: "Transfer", beginner: "Transfer" } },
  transform: { label: { normal: "Transformieren", beginner: "Transformieren" } },
  create: { label: { normal: "Erstellen", beginner: "Erstellen" } },
  view: { label: { normal: "Ansicht", beginner: "Ansicht" } },
  actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
  settings: { label: { normal: "Einstellungen", beginner: "Einstellungen" } },
};

const uiToolbarParentEn: UiToolbarParentEntries = {
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

const _assertUiToolbarParentKeys: AssertUiToolbarParentKeysCovered<UiToolbarParentCategory> = true;

export const uiChromeTranslationBundles = {
  de: {
    translation: {
  "ui": {
    "nav": {
      "back": {
        "label": {
          "normal": "Zurueck",
          "beginner": "Zurueck"
        }
      },
      "forward": {
        "label": {
          "normal": "Vorwaerts",
          "beginner": "Vorwaerts"
        }
      },
      "up": {
        "label": {
          "normal": "Eine Ebene hoch",
          "beginner": "Eine Ebene hoch"
        }
      }
    },
    "search": {
      "toggle": {
        "label": {
          "normal": "Suche",
          "beginner": "Suche"
        }
      },
      "close": {
        "label": {
          "normal": "Suche schliessen",
          "beginner": "Suche schliessen"
        }
      }
    },
    "panelToggle": {
      "workbench": {
        "label": {
          "normal": "Arbeitsbereich",
          "beginner": "Arbeitsbereich"
        }
      }
    },
    toolbar: {
      group: {
        parent: {
          label: {
            normal: "Werkzeug",
            beginner: "Werkzeug",
          },
        },
      },
      parent: uiToolbarParentDe,
    },
    common: {
      mixedValues: {
        label: {
          normal: "Gemischt",
          beginner: "Gemischt",
        },
      },
    },
    docs: {
      navigation: {
        previous: {
          label: {
            normal: "Zurueck",
            beginner: "Zurueck",
          },
        },
        next: {
          label: {
            normal: "Weiter",
            beginner: "Weiter",
          },
        },
      },
    },
    ring: {
      demo: {
        label: {
          normal: "Ring",
          beginner: "Ring",
        },
      },
    },
    stepper: {
      demo: {
        label: {
          normal: "Wert",
          beginner: "Wert",
        },
      },
    },
  },
  settings: {
    "layout": {
      "normal": {
        "label": {
          "normal": "Normal layout",
          "beginner": "Use the standard layout optimized for mouse and keyboard."
        }
      },
      "desktop": {
        "label": {
          "normal": "Desktop layout",
          "beginner": "Use the desktop layout optimized for large screens."
        }
      },
      "tablet": {
        "label": {
          "normal": "Tablet layout",
          "beginner": "Use the tablet layout optimized for medium screens."
        }
      },
      "mobile": {
        "label": {
          "normal": "Mobile layout",
          "beginner": "Use the mobile layout optimized for small screens."
        }
      }
    },
    "compact": {
      "label": {
        "normal": "Kompakt",
        "beginner": "Schaltflaechen und Umschalter nur mit Symbol anzeigen, um Platz zu sparen"
      }
    },
    "mode": {
      "dev": {
        "label": {
          "normal": "Developer mode",
          "beginner": "Show developer tools and advanced options."
        }
      },
      "user": {
        "label": {
          "normal": "User mode",
          "beginner": "Show standard user port."
        }
      },
      "beginner": {
        "label": {
          "normal": "Beginner mode",
          "beginner": "Show full guidance, tutorials, and detailed tooltips."
        }
      },
      "normal": {
        "label": {
          "normal": "Normal mode",
          "beginner": "Show contextual help without tutorials."
        }
      }
    },
    "expertise": {
      "beginner": {
        "label": {
          "normal": "Anfänger",
          "beginner": "Show full guidance and tutorials."
        }
      },
      "normal": {
        "label": {
          "normal": "Normal",
          "beginner": "Show contextual help."
        }
      },
      "expert": {
        "label": {
          "normal": "Experte",
          "beginner": "Hide guidance."
        }
      }
    }
  },
  "tooltip": {
    "manual": {
      "label": {
        "normal": "Handbuch",
        "beginner": "Handbuch"
      }
    },
    tutorial: {
      label: {
        normal: "Tutorial",
        beginner: "Tutorial",
      },
    },
  },
} satisfies UiTranslationSchema,
  },
  en: {
    translation: {
  "ui": {
    "nav": {
      "back": {
        "label": {
          "normal": "Go back",
          "beginner": "Go back"
        }
      },
      "forward": {
        "label": {
          "normal": "Go forward",
          "beginner": "Go forward"
        }
      },
      "up": {
        "label": {
          "normal": "Go up one level",
          "beginner": "Go up one level"
        }
      }
    },
    "search": {
      "toggle": {
        "label": {
          "normal": "Search",
          "beginner": "Search"
        }
      },
      "close": {
        "label": {
          "normal": "Close search",
          "beginner": "Close search"
        }
      }
    },
    "panelToggle": {
      "workbench": {
        "label": {
          "normal": "Workbench",
          "beginner": "Workbench"
        }
      }
    },
    toolbar: {
      group: {
        parent: {
          label: {
            normal: "Tool",
            beginner: "Tool",
          },
        },
      },
      parent: uiToolbarParentEn,
    },
    common: {
      mixedValues: {
        label: {
          normal: "Mixed",
          beginner: "Mixed",
        },
      },
    },
    docs: {
      navigation: {
        previous: {
          label: {
            normal: "Previous",
            beginner: "Previous",
          },
        },
        next: {
          label: {
            normal: "Next",
            beginner: "Next",
          },
        },
      },
    },
    ring: {
      demo: {
        label: {
          normal: "Ring",
          beginner: "Ring",
        },
      },
    },
    stepper: {
      demo: {
        label: {
          normal: "Value",
          beginner: "Value",
        },
      },
    },
  },
  settings: {
    layout: {
      normal: {
        label: {
          normal: "Normal layout",
          beginner: "Use the standard layout optimized for mouse and keyboard.",
        },
      },
      "desktop": {
        "label": {
          "normal": "Desktop layout",
          "beginner": "Use the desktop layout optimized for large screens."
        }
      },
      "tablet": {
        "label": {
          "normal": "Tablet layout",
          "beginner": "Use the tablet layout optimized for medium screens."
        }
      },
      "mobile": {
        "label": {
          "normal": "Mobile layout",
          "beginner": "Use the mobile layout optimized for small screens."
        }
      }
    },
    "compact": {
      "label": {
        "normal": "Compact",
        "beginner": "Show icon-only buttons and toggles to save space"
      }
    },
    "mode": {
      "dev": {
        "label": {
          "normal": "Developer mode",
          "beginner": "Show developer tools and advanced options."
        }
      },
      "user": {
        "label": {
          "normal": "User mode",
          "beginner": "Show standard user port."
        }
      },
      "beginner": {
        "label": {
          "normal": "Beginner mode",
          "beginner": "Show full guidance, tutorials, and detailed tooltips."
        }
      },
      "normal": {
        "label": {
          "normal": "Normal mode",
          "beginner": "Show contextual help without tutorials."
        }
      }
    },
    "expertise": {
      "beginner": {
        "label": {
          "normal": "Beginner",
          "beginner": "Show full guidance and tutorials."
        }
      },
      "normal": {
        "label": {
          "normal": "Normal",
          "beginner": "Show contextual help."
        }
      },
      "expert": {
        "label": {
          "normal": "Expert",
          "beginner": "Hide guidance."
        }
      }
    }
  },
  "tooltip": {
    "manual": {
      "label": {
        "normal": "Manual",
        "beginner": "Manual"
      }
    },
    tutorial: {
      label: {
        normal: "Tutorial",
        beginner: "Tutorial",
      },
    },
  },
} satisfies UiTranslationSchema,
  },
} satisfies Record<UiLocale, { readonly translation: UiTranslationSchema }>;

// #endregion 🪁I18n Types
