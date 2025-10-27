// #region Header

// TooltipExamples.tsx

// Examples demonstrating the Semio Tooltip System

// #endregion

import { BookOpen, Home, Settings } from "lucide-react";
import { FC } from "react";
import { Button } from "../elements/input/Button";
import { SemioTooltipWrapper } from "./SemioTooltip";

// Example 1: Basic tooltip with label only
export const BasicTooltipExample: FC = () => (
  <SemioTooltipWrapper config={{ labelKey: "navbar.home" }}>
    <Button>
      <Home />
    </Button>
  </SemioTooltipWrapper>
);

// Example 2: Tooltip with label and manual link
export const TooltipWithManualExample: FC = () => (
  <SemioTooltipWrapper
    config={{
      labelKey: "navbar.home",
      manualPath: "/docs/manuals/sketchpad#navigation",
    }}
  >
    <Button>
      <Home />
    </Button>
  </SemioTooltipWrapper>
);

// Example 3: Tooltip with label, manual, and tutorial
export const TooltipWithTutorialExample: FC = () => (
  <SemioTooltipWrapper
    config={{
      labelKey: "navbar.home",
      manualPath: "/docs/manuals/sketchpad#navigation",
      tutorialPath: "/docs/tutorials/hello-semio",
    }}
  >
    <Button>
      <Home />
    </Button>
  </SemioTooltipWrapper>
);

// Example 4: Complete tooltip with all options
export const CompleteTooltipExample: FC = () => (
  <SemioTooltipWrapper
    config={{
      labelKey: "navbar.settings",
      manualPath: "/docs/manuals/sketchpad#settings",
      tutorialPath: "/docs/tutorials/hello-semio/sketch-setup",
      hotkey: "⌘,",
    }}
  >
    <Button>
      <Settings />
    </Button>
  </SemioTooltipWrapper>
);

// Example 5: Tooltip with hotkey only (no label)
export const HotkeyOnlyTooltipExample: FC = () => (
  <SemioTooltipWrapper config={{ hotkey: "⌘K" }}>
    <Button>Search</Button>
  </SemioTooltipWrapper>
);

// Example 6: Using in app config (TypeScript only, not a component)
export const appConfigExample = {
  getPanels: (t: (key: string) => string) => [
    {
      key: "workbench",
      icon: BookOpen,
      tooltip: {
        labelKey: "panels.workbench",
        manualPath: "/docs/manuals/sketchpad#workbench",
        tutorialPath: "/docs/tutorials/hello-semio",
        hotkey: "⌘1",
      },
      hotkey: "⌘1",
    },
  ],
};

// Example 7: Backward compatible - still using strings
export const BackwardCompatibleExample: FC = () => {
  // This pattern still works, though it won't have manual/tutorial links
  return (
    <Button title="Go to home">
      <Home />
    </Button>
  );
};
