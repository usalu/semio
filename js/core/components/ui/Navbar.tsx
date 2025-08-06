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
import { AppWindow, Fingerprint, Home, Minus, Moon, Share2, Square, Sun, X } from "lucide-react";
import { FC, ReactNode } from "react";

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
  // Handle design selection
  const handleDesignChange = (designName: string) => {
    const selectedDesign = availableDesigns?.find((design) => design.name === designName);
    if (selectedDesign && onDesignIdChange) {
      onDesignIdChange(selectedDesign);
    }
  };

  return (
    <div className={`w-full h-12 bg-background border-b flex items-center justify-between px-4`}>
      <div className="flex items-center">
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
              <Select value={designId.name} onValueChange={handleDesignChange}>
                <SelectTrigger className="border-none bg-transparent hover:bg-accent/50 px-2 py-1 text-sm font-medium">
                  <SelectValue>{designId.name}</SelectValue>
                </SelectTrigger>
                <SelectContent>
                  {availableDesigns?.map((design) => (
                    <SelectItem key={design.name} value={design.name}>
                      {design.name}
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
