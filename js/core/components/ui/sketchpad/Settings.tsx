import { TreeContent, TreeItem, TreeSection } from "../Tree";

import { t } from "i18next";
import { CircleOffIcon, FingerprintIcon, Laptop, MessageSquareTextIcon, MonitorIcon, MoonIcon, SunIcon, TextIcon } from "lucide-react";
import { FC, useState } from "react";
import { useTranslation } from "react-i18next";
import { Layout, Mode, Theme, useLayout, useMode, useSketchpadCommands, useTheme } from "../../../store";
import { ScrollArea } from "../ScrollArea";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../Select";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";
import { Tree } from "../Tree";
import { usePanelSections } from "./Navbar";
import { ResizablePanelProps } from "./Sketchpad";

const LanguageSwitcher: FC = () => {
  const { i18n } = useTranslation();
  return (
    <Select label={t("settings.language")} value={i18n.language} onValueChange={(value) => i18n.changeLanguage(value)}>
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
  const theme = useTheme();
  const layout = useLayout();
  const mode = useMode();
  const { setTheme, setLayout, setMode } = useSketchpadCommands();
  const sections = usePanelSections("settings");

  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startX = e.clientX;
    const startWidth = width;

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth - (e.clientX - startX);
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth);
      }
    };

    const handleMouseUp = () => {
      setIsResizing(false);
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  };

  // Sort sections by order
  const sortedSections = sections.sort((a, b) => (a.order || 0) - (b.order || 0));

  return (
    <div
      className={`absolute top-4 right-4 bottom-4 z-20 bg-panel text-foreground border min-w-0 overflow-hidden
                ${isResizing || isResizeHovered ? "border-l-primary" : "border-l"}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1 overflow-hidden min-w-0">
          <Tree className="min-w-0 overflow-hidden">
            <TreeSection label={t("settings.general")} defaultOpen={true}>
              <TreeItem>
                <TreeContent>
                  <ToggleGroup label={t("settings.theme")} type="single" value={theme} onValueChange={(value: string) => setTheme(value as Theme)} level="panel">
                    <ToggleGroupItem value={Theme.SYSTEM}>
                      <Laptop />
                    </ToggleGroupItem>
                    <ToggleGroupItem value={Theme.LIGHT}>
                      <SunIcon />
                    </ToggleGroupItem>
                    <ToggleGroupItem value={Theme.DARK}>
                      <MoonIcon />
                    </ToggleGroupItem>
                  </ToggleGroup>
                </TreeContent>
              </TreeItem>
              <TreeItem>
                <TreeContent>
                  <ToggleGroup label={t("settings.layout")} type="single" value={layout} onValueChange={(value: string) => setLayout(value as Layout)} level="panel">
                    <ToggleGroupItem value={Layout.NORMAL}>
                      <MonitorIcon />
                    </ToggleGroupItem>
                    <ToggleGroupItem value={Layout.TOUCH}>
                      <FingerprintIcon />
                    </ToggleGroupItem>
                  </ToggleGroup>
                </TreeContent>
              </TreeItem>
              <TreeItem>
                <TreeContent>
                  <ToggleGroup label={t("settings.mode")} type="single" value={mode} onValueChange={(value: string) => setMode(value as Mode)} level="panel">
                    <ToggleGroupItem value={Mode.BEGINNER}>
                      <CircleOffIcon />
                    </ToggleGroupItem>
                    <ToggleGroupItem value={Mode.NORMAL}>
                      <TextIcon />
                    </ToggleGroupItem>
                    <ToggleGroupItem value={Mode.EXPERT}>
                      <MessageSquareTextIcon />
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

            {sortedSections.map((section) => (
              <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions}>
                {typeof section.content === "function" ? section.content() : section.content}
              </TreeSection>
            ))}
          </Tree>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Settings;
