// #region Header

// Navbar.tsx

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
import { DesignId, Layout, Mode, Theme } from "@semio/js";
import { Avatar, AvatarFallback, AvatarImage } from "@semio/js/components/ui/Avatar";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "@semio/js/components/ui/Breadcrumb";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@semio/js/components/ui/Select";
import { Toggle } from "@semio/js/components/ui/Toggle";
import { ToggleCycle } from "@semio/js/components/ui/ToggleCycle";
import { ToggleGroup, ToggleGroupItem } from "@semio/js/components/ui/ToggleGroup";
import { AppWindow, ArrowLeft, Fingerprint, Home, Minus, Moon, Share2, Square, Sun, X } from "lucide-react";
import { FC, ReactNode } from "react";
import { useSketchpad } from "./Sketchpad";

interface NavbarProps {
  mode?: Mode;
  toolbarContent?: ReactNode;
  layout?: Layout;
  theme?: Theme;
  setLayout?: (layout: Layout) => void;
  setTheme?: (theme: Theme) => void;
  designId: DesignId;
  onDesignIdChange?: (designId: DesignId) => void;
  availableDesigns?: DesignId[];
  onWindowEvents?: {
    minimize: () => void;
    maximize: () => void;
    close: () => void;
  };
}

const Navbar: FC<NavbarProps> = ({ mode, toolbarContent, layout, theme, setLayout, setTheme, onWindowEvents, designId, onDesignIdChange, availableDesigns }) => {
  const { previousDesign, sketchpadState } = useSketchpad();
  // Create a unique key for each design
  const getDesignKey = (design: DesignId): string => {
    const parts = [design.name];
    if (design.variant && design.variant !== "default") parts.push(design.variant);
    if (design.view && design.view !== "default") parts.push(design.view);
    return parts.join("|"); // Use | as separator to avoid conflicts
  };

  // Create display text for each design
  const getDesignDisplayText = (design: DesignId): string => {
    if (!availableDesigns) return design.name;

    // Check if there are multiple designs with the same name
    const sameNameDesigns = availableDesigns.filter((d) => d.name === design.name);

    if (sameNameDesigns.length === 1) {
      // Only one design with this name, show just the name
      return design.name;
    } else {
      // Multiple designs with same name, include variant/view for disambiguation
      const parts = [design.name];
      if (design.variant && design.variant !== "default") {
        parts.push(design.variant);
      }
      if (design.view && design.view !== "default") {
        parts.push(design.view);
      }
      return parts.join(" - ");
    }
  };

  // Handle design selection using the compound key
  const handleDesignChange = (designKey: string) => {
    if (!availableDesigns || !onDesignIdChange) return;

    const selectedDesign = availableDesigns.find((design) => getDesignKey(design) === designKey);
    if (selectedDesign) {
      onDesignIdChange(selectedDesign);
    }
  };

  // Get current design key for the selected value
  const currentDesignKey = getDesignKey(designId);

  return (
    <div className={`w-full h-12 bg-background border-b flex items-center justify-between px-4`}>
      <div className="flex items-center gap-2">
        {/* Back button */}
        <Toggle variant="outline" tooltip="Previous design" disabled={sketchpadState.designHistory.length === 0} onClick={previousDesign} className="h-8 w-8 p-0">
          <ArrowLeft size={16} />
        </Toggle>

        <Breadcrumb>
          <BreadcrumbList>
            <BreadcrumbItem>
              <BreadcrumbLink href="/">
                <Home size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={[
                { label: "Starter", href: "/metabolism/starter" },
                { label: "Geometry", href: "/metabolism/geometry" },
              ]}
              onNavigate={(href) => console.log("Navigate to:", href)}
            />
            <BreadcrumbItem>
              <BreadcrumbLink href="/metabolism">Metabolism</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={[
                { label: "Types", href: "/designs/types" },
                { label: "Representations", href: "/designs/representations" },
              ]}
              onNavigate={(href) => console.log("Navigate to:", href)}
            />
            <BreadcrumbItem>
              <BreadcrumbLink href="/designs">Designs</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={[
                {
                  label: "Capsule Dream",
                  href: "/designs/nakagin/capsule-dream",
                },
              ]}
              onNavigate={(href) => console.log("Navigate to:", href)}
            />
            <BreadcrumbItem>
              <Select value={currentDesignKey} onValueChange={handleDesignChange}>
                <SelectTrigger className="border-none bg-transparent hover:bg-accent/50 px-2 py-1 text-sm font-medium">
                  <SelectValue>{getDesignDisplayText(designId)}</SelectValue>
                </SelectTrigger>
                <SelectContent>
                  {availableDesigns?.map((design, index) => (
                    <SelectItem key={`${getDesignKey(design)}-${index}`} value={getDesignKey(design)}>
                      {getDesignDisplayText(design)}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
      </div>
      <div className="flex items-center gap-4">
        {toolbarContent}
        <ToggleCycle
          value={theme}
          onValueChange={setTheme}
          items={[
            {
              value: Theme.LIGHT,
              tooltip: "Turn theme dark",
              label: <Moon />,
            },
            {
              value: Theme.DARK,
              tooltip: "Turn theme light",
              label: <Sun />,
            },
          ]}
        />
        <ToggleCycle
          value={layout}
          onValueChange={setLayout}
          items={[
            {
              value: Layout.NORMAL,
              tooltip: "Turn touch layout on",
              label: <Fingerprint />,
            },
            {
              value: Layout.TOUCH,
              tooltip: "Return to normal layout",
              label: <AppWindow />,
            },
          ]}
        />

        <Avatar className="h-8 w-8">
          <AvatarImage src="https://github.com/usalu.png" />
          <AvatarFallback>US</AvatarFallback>
        </Avatar>

        <Toggle variant="outline" tooltip="Share">
          <Share2 />
        </Toggle>

        {onWindowEvents && (
          <div className="flex items-center gap-2 ml-4">
            <ToggleGroup type="single">
              <ToggleGroupItem value="minimize" onClick={onWindowEvents.minimize}>
                <Minus size={16} />
              </ToggleGroupItem>
              <ToggleGroupItem value="maximize" onClick={onWindowEvents.maximize}>
                <Square size={16} />
              </ToggleGroupItem>
              <ToggleGroupItem value="close" onClick={onWindowEvents.close} className="hover:bg-danger">
                <X size={16} />
              </ToggleGroupItem>
            </ToggleGroup>
          </div>
        )}
      </div>
    </div>
  );
};
export default Navbar;
