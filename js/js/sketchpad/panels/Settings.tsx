// #region Header

// Settings.tsx

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

import { t } from "i18next";
import { BrainCircuit, FingerprintIcon, GraduationCap, Laptop, MonitorIcon, MoonIcon, Sparkles, SunIcon } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { TreeContent, TreeItem, TreeSection } from "../../elements/aggregation/Tree";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../elements/input/Select";
import { ToggleGroup, ToggleGroupItem } from "../../elements/input/ToggleGroup";
import Panel from "../Panel.js";
import { ResizablePanelProps } from "../Sketchpad";
import { Layout, Mode, Theme, useIsMobile, useLayout, useMode, useSketchpadCommands, useTheme, useTooltip } from "../store";

const LanguageSwitcher: FC = () => {
  const { i18n } = useTranslation();
  return (
    <Select label={t("semio.sketchpad.panel.settings.language.label")} value={i18n.language} onValueChange={(value) => i18n.changeLanguage(value)}>
      <SelectTrigger className="w-32" level="panel">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="en">English</SelectItem>
        <SelectItem value="de">Deutsch</SelectItem>
      </SelectContent>
    </Select>
  );
};

interface SettingsProps extends ResizablePanelProps {}

const Settings: FC<SettingsProps> = ({ visible, onWidthChange, width }) => {
  const { t } = useTranslation();
  const tooltip = useTooltip();
  const theme = useTheme();
  const layout = useLayout();
  const mode = useMode();
  const { setTheme, setLayout, setMode } = useSketchpadCommands();
  const isMobile = useIsMobile();

  return (
    <Panel
      panelId="settings"
      visible={visible}
      onWidthChange={onWidthChange}
      width={width}
      resizeSide="left"
      additionalSections={
        <TreeSection label={t("semio.sketchpad.panel.settings.general.label")} defaultOpen={true}>
          <TreeItem>
            <TreeContent>
              <ToggleGroup label={t("semio.sketchpad.panel.settings.theme.label")} type="single" value={theme} onValueChange={(value: string) => setTheme(value as Theme)} level="panel">
                <ToggleGroupItem value={Theme.SYSTEM} tooltip={tooltip("settings.theme.system")}>
                  <Laptop />
                </ToggleGroupItem>
                <ToggleGroupItem value={Theme.LIGHT} tooltip={tooltip("settings.theme.light")}>
                  <SunIcon />
                </ToggleGroupItem>
                <ToggleGroupItem value={Theme.DARK} tooltip={tooltip("settings.theme.dark")}>
                  <MoonIcon />
                </ToggleGroupItem>
              </ToggleGroup>
            </TreeContent>
          </TreeItem>
          {!isMobile && (
            <TreeItem>
              <TreeContent>
                <ToggleGroup label={t("semio.sketchpad.panel.settings.layout.label")} type="single" value={layout} onValueChange={(value: string) => setLayout(value as Layout)} level="panel">
                  <ToggleGroupItem value={Layout.NORMAL} tooltip={tooltip("settings.layout.normal")}>
                    <MonitorIcon />
                  </ToggleGroupItem>
                  <ToggleGroupItem value={Layout.TOUCH} tooltip={tooltip("settings.layout.touch")}>
                    <FingerprintIcon />
                  </ToggleGroupItem>
                </ToggleGroup>
              </TreeContent>
            </TreeItem>
          )}
          <TreeItem>
            <TreeContent>
              <ToggleGroup label={t("semio.sketchpad.panel.settings.mode.label")} type="single" value={mode} onValueChange={(value: string) => setMode(value as Mode)} level="panel">
                <ToggleGroupItem value={Mode.BEGINNER} tooltip={tooltip("settings.mode.beginner")}>
                  <GraduationCap />
                </ToggleGroupItem>
                <ToggleGroupItem value={Mode.NORMAL} tooltip={tooltip("settings.mode.normal")}>
                  <Sparkles />
                </ToggleGroupItem>
                <ToggleGroupItem value={Mode.EXPERT} tooltip={tooltip("settings.mode.expert")}>
                  <BrainCircuit />
                </ToggleGroupItem>
              </ToggleGroup>
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <LanguageSwitcher />
            </TreeContent>
          </TreeItem>
        </TreeSection>
      }
    />
  );
};

export default Settings;
