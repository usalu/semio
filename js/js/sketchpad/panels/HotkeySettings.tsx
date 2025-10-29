// #region Header

// HotkeySettings.tsx

// 2025 Ueli Saluz

// #endregion

import { RotateCcw } from "lucide-react";
import { FC, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { TreeContent, TreeItem, TreeSection } from "../../elements/aggregation/Tree";
import { TreeStateProvider } from "../../elements/aggregation/TreeStateProvider";
import { Button } from "../../elements/input/Button";
import { Input } from "../../elements/input/Input";
import { getHotkeyPath, useHotkey, useHotkeyOverrides, useResetAllHotkeys, useResetHotkey, useSetHotkey } from "../hotkeys";
import { useSketchpad } from "../store";

interface HotkeyItemProps {
  path: string;
  label: string;
}

const HotkeyItem: FC<HotkeyItemProps> = ({ path, label }) => {
  const hotkey = useHotkey(path);
  const setHotkey = useSetHotkey();
  const resetHotkey = useResetHotkey();
  const overrides = useHotkeyOverrides();
  const isOverridden = path in overrides;
  const activeHotkeySetting = useSketchpad((s) => s.activeHotkeySetting) as string | undefined;
  const isActive = activeHotkeySetting === path;

  return (
    <TreeItem defaultOpen={isActive}>
      <TreeContent>
        <div className="flex items-center gap-2 w-full">
          <span className="flex-1 text-sm">{label}</span>
          <Input className={`w-32 text-xs ${isActive ? "ring-2 ring-primary" : ""}`} value={hotkey || ""} onChange={(e) => setHotkey(path, e.target.value)} placeholder="None" />
          {isOverridden && (
            <Button variant="ghost" onClick={() => resetHotkey(path)}>
              <RotateCcw className="h-3 w-3" />
            </Button>
          )}
        </div>
      </TreeContent>
    </TreeItem>
  );
};

interface HotkeySectionProps {
  basePath: string;
  label: string;
  hotkeys: Record<string, string>;
}

const HotkeySection: FC<HotkeySectionProps> = ({ basePath, label, hotkeys }) => {
  const activeHotkeySetting = useSketchpad((s) => s.activeHotkeySetting) as string | undefined;
  const hasActiveChild = activeHotkeySetting?.startsWith(basePath) || false;

  return (
    <TreeSection label={label} defaultOpen={hasActiveChild}>
      {Object.entries(hotkeys).map(([key, itemLabel]) => {
        const fullPath = getHotkeyPath(basePath, key);
        return <HotkeyItem key={fullPath} path={fullPath} label={itemLabel} />;
      })}
    </TreeSection>
  );
};

export const HotkeySettings: FC = () => {
  const { t } = useTranslation();
  const resetAllHotkeys = useResetAllHotkeys();

  const hotkeyGroups = useMemo(
    () => ({
      navigation: {
        label: t("semio.sketchpad.navbar.breadcrumb.home"),
        hotkeys: {
          "semio.sketchpad.navbar.back": t("semio.sketchpad.navbar.back.label"),
          "semio.sketchpad.navbar.forward": t("semio.sketchpad.navbar.forward.label"),
          "semio.sketchpad.navbar.up": t("semio.sketchpad.navbar.up.label"),
          "semio.sketchpad.navbar.home": t("semio.sketchpad.navbar.home.label"),
          "semio.sketchpad.navbar.docs": t("semio.sketchpad.navbar.docs.label"),
        },
      },
      panels: {
        label: "Panels",
        hotkeys: {
          "semio.sketchpad.navbar.panelToggle.workbench.show": t("semio.sketchpad.navbar.panelToggle.workbench.show.label"),
          "semio.sketchpad.navbar.panelToggle.tools.show": t("semio.sketchpad.navbar.panelToggle.tools.show.label"),
          "semio.sketchpad.navbar.panelToggle.toolbar.show": t("semio.sketchpad.navbar.panelToggle.toolbar.show.label"),
          "semio.sketchpad.navbar.panelToggle.hud.show": t("semio.sketchpad.navbar.panelToggle.hud.show.label"),
          "semio.sketchpad.navbar.panelToggle.stats.show": t("semio.sketchpad.navbar.panelToggle.stats.show.label"),
          "semio.sketchpad.navbar.panelToggle.details.show": t("semio.sketchpad.navbar.panelToggle.details.show.label"),
          "semio.sketchpad.navbar.panelToggle.chat.show": t("semio.sketchpad.navbar.panelToggle.chat.show.label"),
          "semio.sketchpad.navbar.panelToggle.settings.show": t("semio.sketchpad.navbar.panelToggle.settings.show.label"),
        },
      },
      view: {
        label: "View",
        hotkeys: {
          "semio.sketchpad.navbar.search.open": t("semio.sketchpad.navbar.search.open.label"),
          "semio.sketchpad.navbar.focus.open": t("semio.sketchpad.navbar.focus.open.label"),
          "semio.sketchpad.navbar.fullscreen": t("semio.sketchpad.navbar.fullscreen.label"),
          "semio.sketchpad.navbar.expand": t("semio.sketchpad.navbar.expand.label"),
        },
      },
      tools: {
        label: "Tools",
        hotkeys: {
          "semio.sketchpad.app.design.tools.1": "Tool 1",
          "semio.sketchpad.app.design.tools.2": "Tool 2",
          "semio.sketchpad.app.design.tools.3": "Tool 3",
          "semio.sketchpad.app.design.tools.4": "Tool 4",
          "semio.sketchpad.app.design.tools.5": "Tool 5",
          "semio.sketchpad.app.design.tools.6": "Tool 6",
          "semio.sketchpad.app.design.tools.7": "Tool 7",
          "semio.sketchpad.app.design.tools.8": "Tool 8",
          "semio.sketchpad.app.design.tools.9": "Tool 9",
          "semio.sketchpad.app.design.tools.0": "Tool 10",
        },
      },
    }),
    [t],
  );

  return (
    <TreeStateProvider>
      <div className="space-y-2">
        <div className="flex items-center justify-between p-2 border-b">
          <span className="text-sm font-semibold">Hotkeys</span>
          <Button variant="ghost" onClick={resetAllHotkeys}>
            <RotateCcw className="h-4 w-4 mr-1" />
            Reset All
          </Button>
        </div>
        <div className="space-y-1">
          {Object.entries(hotkeyGroups).map(([key, { label, hotkeys }]) => (
            <HotkeySection key={key} basePath={key} label={label} hotkeys={hotkeys} />
          ))}
        </div>
      </div>
    </TreeStateProvider>
  );
};
