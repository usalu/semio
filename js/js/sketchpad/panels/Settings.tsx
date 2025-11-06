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

import { BrainCircuit, FingerprintIcon, GraduationCap, Laptop, MonitorIcon, MoonIcon, Sparkles, SunIcon } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { TreeContent, TreeItem, TreeSection } from "../../elements/aggregation/Tree";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../elements/input/Select";
import { ToggleGroup, ToggleGroupItem } from "../../elements/input/ToggleGroup";
import Panel from "../Panel.js";
import { ResizablePanelProps } from "../Sketchpad";
import { Expertise, type Layout, Mode, Theme, useExpertise, useIsMobile, useLayout, useMode, useSketchpadCommands, useTheme, useTooltip } from "../store";
import { HotkeySettings } from "./HotkeySettings";

const LanguageSwitcher: FC = () => {
  const { i18n } = useTranslation();
  return (
    <Select id="semio.sketchpad.panel.settings.language" value={i18n.language} onValueChange={(value) => i18n.changeLanguage(value)}>
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
  const tooltip = useTooltip();
  const theme = useTheme();
  const layout = useLayout();
  const expertise = useExpertise();
  const mode = useMode();
  const { setTheme, setLayout, setExpertise, setMode } = useSketchpadCommands();
  const isMobile = useIsMobile();

  return (
    <Panel
      panelId="settings"
      visible={visible}
      onWidthChange={onWidthChange}
      width={width}
      resizeSide="left"
      additionalSections={
        <TreeSection id="semio.sketchpad.panel.settings.general" defaultOpen={true}>
          <TreeItem>
            <TreeContent>
              <ToggleGroup id="semio.sketchpad.panel.settings.theme" type="single" value={theme} onValueChange={(value: string) => setTheme("semio.sketchpad.panel.settings.theme", value as Theme)} level="panel">
                <ToggleGroupItem id="semio.sketchpad.panel.settings.theme.system" value={Theme.SYSTEM}>
                  <Laptop />
                </ToggleGroupItem>
                <ToggleGroupItem id="semio.sketchpad.panel.settings.theme.light" value={Theme.LIGHT}>
                  <SunIcon />
                </ToggleGroupItem>
                <ToggleGroupItem id="semio.sketchpad.panel.settings.theme.dark" value={Theme.DARK}>
                  <MoonIcon />
                </ToggleGroupItem>
              </ToggleGroup>
            </TreeContent>
          </TreeItem>
          {!isMobile && (
            <TreeItem>
              <TreeContent>
                <ToggleGroup
                  id="semio.sketchpad.panel.settings.layout"
                  type="single"
                  value={typeof layout === "string" ? layout : "desktop"}
                  onValueChange={(value: string) => setLayout("semio.sketchpad.panel.settings.layout", value as Layout)}
                  level="panel"
                >
                  <ToggleGroupItem id="semio.sketchpad.panel.settings.layout.desktop" value="desktop">
                    <MonitorIcon />
                  </ToggleGroupItem>
                  <ToggleGroupItem id="semio.sketchpad.panel.settings.layout.tablet" value="tablet">
                    <FingerprintIcon />
                  </ToggleGroupItem>
                </ToggleGroup>
              </TreeContent>
            </TreeItem>
          )}
          <TreeItem>
            <TreeContent>
              <ToggleGroup id="semio.sketchpad.panel.settings.expertise" type="single" value={expertise} onValueChange={(value: string) => setExpertise("semio.sketchpad.panel.settings.expertise", value as Expertise)} level="panel">
                <ToggleGroupItem id="semio.sketchpad.panel.settings.expertise.beginner" value={Expertise.BEGINNER}>
                  <GraduationCap />
                </ToggleGroupItem>
                <ToggleGroupItem id="semio.sketchpad.panel.settings.expertise.normal" value={Expertise.NORMAL}>
                  <Sparkles />
                </ToggleGroupItem>
                <ToggleGroupItem id="semio.sketchpad.panel.settings.expertise.expert" value={Expertise.EXPERT}>
                  <BrainCircuit />
                </ToggleGroupItem>
              </ToggleGroup>
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <ToggleGroup id="semio.sketchpad.panel.settings.mode" type="single" value={mode} onValueChange={(value: string) => setMode("semio.sketchpad.panel.settings.mode", value as Mode)} level="panel">
                <ToggleGroupItem id="semio.sketchpad.panel.settings.mode.user" value={Mode.USER}>
                  User
                </ToggleGroupItem>
                <ToggleGroupItem id="semio.sketchpad.panel.settings.mode.dev" value={Mode.DEV}>
                  Dev
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
      footer={<HotkeySettings />}
    />
  );
};

export default Settings;
