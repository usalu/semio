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

import { AppWindow, Fingerprint, Home, Layout, Minus, Moon, Share2, Square, Sun, X } from "lucide-react";
import { createContext, FC, ReactNode, useContext } from "react";
import { DesignId } from "../../semio";
import { Theme, useDesignId, useDesigns, useLayout, useSketchpadCommands, useTheme } from "../../store";
import { Avatar, AvatarFallback, AvatarImage } from "./Avatar";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "./Breadcrumb";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "./Select";
import { Toggle } from "./Toggle";
import { ToggleCycle } from "./ToggleCycle";
import { ToggleGroup, ToggleGroupItem } from "./ToggleGroup";

interface NavbarContextType {
  navbarToolbar: ReactNode | null;
  setNavbarToolbar: (toolbar: ReactNode) => void;
}

export const NavbarContext = createContext<NavbarContextType | null>(null);

export const useNavbar = () => {
  const context = useContext(NavbarContext);
  if (!context) {
    throw new Error("useNavbar must be used within a NavbarProvider");
  }
  return context;
};

interface NavbarProps {
  toolbarContent?: ReactNode;
  onWindowEvents?: {
    minimize: () => void;
    maximize: () => void;
    close: () => void;
  };
}

const Navbar: FC<NavbarProps> = ({ toolbarContent, onWindowEvents }) => {
  const { setTheme, setLayout } = useSketchpadCommands();
  const { navbarToolbar } = useNavbar();
  const layout = useLayout();
  const theme = useTheme();
  const designId = useDesignId();
  const availableDesigns = useDesigns();
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
    // if (!availableDesigns || !onDesignIdChange) return;
    // const selectedDesign = availableDesigns.find((design) => getDesignKey(design) === designKey);
    // if (selectedDesign) {
    //   onDesignIdChange(selectedDesign);
    // }
  };

  // Get current design key for the selected value
  const currentDesignKey = getDesignKey(designId);

  return (
    <div className={`w-full h-12 bg-background border-b flex items-center justify-between px-4`}>
      <div className="flex items-center gap-2">
        {/* Back button */}
        {/* <Toggle variant="outline" tooltip="Previous design" disabled={sketchpadState.designHistory.length === 0} onClick={previousDesign} className="h-8 w-8 p-0">
          <ArrowLeft size={16} />
        </Toggle> */}

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
              onNavigate={(href) => {}}
            />
            <BreadcrumbItem>
              <BreadcrumbLink href="/metabolism">Metabolism</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={[
                { label: "Types", href: "/designs/types" },
                { label: "Representations", href: "/designs/representations" },
              ]}
              onNavigate={(href) => {}}
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
              onNavigate={(href) => {}}
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
        {navbarToolbar || toolbarContent}
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
