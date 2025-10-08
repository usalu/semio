import { TreeContent, TreeItem, TreeSection } from "../Tree";

import { FingerprintIcon, Laptop, MonitorIcon, MoonIcon, SunIcon } from "lucide-react";
import { FC, useState } from "react";
import { useTranslation } from "react-i18next";
import { Layout, Theme, useLayout, useSketchpadCommands, useTheme } from "../../../store";
import LanguageSwitcher from "../LanguageSwitcher";
import { ScrollArea } from "../ScrollArea";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";
import { Tree } from "../Tree";
import { usePanelSections } from "./Navbar";
import { ResizablePanelProps } from "./Sketchpad";

interface SettingsProps extends ResizablePanelProps {}

const Settings: FC<SettingsProps> = ({ visible, onWidthChange, width }) => {
  const { t } = useTranslation();
  const theme = useTheme();
  const layout = useLayout();
  const { setTheme, setLayout } = useSketchpadCommands();
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
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border min-w-0 overflow-hidden
                ${isResizing || isResizeHovered ? "border-l-primary" : "border-l"}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1 overflow-hidden min-w-0">
          <Tree className="min-w-0 overflow-hidden">
            <TreeSection label={t("settings.general")} defaultOpen={true}>
              <TreeItem>
                <TreeContent>
                  <ToggleGroup label={t("settings.theme")} type="single" value={theme} onValueChange={(value) => setTheme(value as Theme)}>
                    <ToggleGroupItem value="system">
                      <Laptop />
                    </ToggleGroupItem>
                    <ToggleGroupItem value="light">
                      <SunIcon />
                    </ToggleGroupItem>
                    <ToggleGroupItem value="dark">
                      <MoonIcon />
                    </ToggleGroupItem>
                  </ToggleGroup>
                </TreeContent>
              </TreeItem>
              <TreeItem>
                <TreeContent>
                  <ToggleGroup label={t("settings.layout")} type="single" value={layout} onValueChange={(value) => setLayout(value as Layout)}>
                    <ToggleGroupItem value="normal">
                      <MonitorIcon />
                    </ToggleGroupItem>
                    <ToggleGroupItem value="touch">
                      <FingerprintIcon />
                    </ToggleGroupItem>
                  </ToggleGroup>
                </TreeContent>
              </TreeItem>
              <TreeItem>
                <TreeContent>
                  <div className="flex flex-col gap-2">
                    <label className="text-sm font-medium">{t("settings.language")}</label>
                    <LanguageSwitcher />
                  </div>
                </TreeContent>
              </TreeItem>
            </TreeSection>

            {sortedSections.map((section) => (
              <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions}>
                {section.content}
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
