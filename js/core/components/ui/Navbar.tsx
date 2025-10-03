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

import { AppWindow, Fingerprint, Home, Minus, Moon, Square, Sun, X } from "lucide-react";
import { createContext, FC, ReactNode, useContext } from "react";
import { useNavigate } from "react-router";
import { Layout, SketchpadScope, Theme, useSketchpadScope } from "../../store";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "./Breadcrumb";
import { ToggleCycle } from "./ToggleCycle";
import { ToggleGroup, ToggleGroupItem } from "./ToggleGroup";

interface NavbarContextType {
  navbarToolbar: ReactNode | null;
  setToolbar: (toolbar: ReactNode) => void;
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
}

const Navbar: FC<NavbarProps> = ({ toolbarContent }) => {
  const { onWindowEvents } = useSketchpadScope() as SketchpadScope;
  let navigate = useNavigate();

  return (
    <div id="navbar" className={`w-full h-12 bg-background border-b flex items-center justify-between px-4 [-webkit-app-region: drag]`} style={{ WebkitAppRegion: "drag" }}>
      <div className="flex items-center gap-2">
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
            {/* <BreadcrumbItem>
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
            </BreadcrumbItem> */}
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
