// #region Header

// Footer.tsx

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

import { LucideIcon } from "lucide-react";
import { createContext, FC, ReactNode, useCallback, useContext, useEffect, useState } from "react";
import { Action } from "../elements/input/Action";
import { useIsFullscreen } from "./store";

export interface FooterItem {
  id: string;
  icon?: LucideIcon;
  label?: string;
  onClick?: () => void;
  order?: number;
}

interface FooterItemContextValue {
  items: FooterItem[];
  addItem: (item: FooterItem) => void;
  removeItem: (itemId: string) => void;
}

const FooterItemContext = createContext<FooterItemContextValue | null>(null);

export const FooterItemProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [items, setItems] = useState<FooterItem[]>([]);

  const addItem = useCallback((item: FooterItem) => {
    setItems((prev) => [...prev.filter((i) => i.id !== item.id), item].sort((a, b) => (a.order || 0) - (b.order || 0)));
  }, []);

  const removeItem = useCallback((itemId: string) => {
    setItems((prev) => prev.filter((i) => i.id !== itemId));
  }, []);

  return <FooterItemContext.Provider value={{ items, addItem, removeItem }}>{children}</FooterItemContext.Provider>;
};

export const useFooterItems = (): FooterItem[] => {
  const context = useContext(FooterItemContext);
  if (!context) throw new Error("useFooterItems must be used within FooterItemProvider");
  return context.items;
};

export const useAddFooterItem = () => {
  const context = useContext(FooterItemContext);
  if (!context) throw new Error("useAddFooterItem must be used within FooterItemProvider");
  return context.addItem;
};

export const useRemoveFooterItem = () => {
  const context = useContext(FooterItemContext);
  if (!context) throw new Error("useRemoveFooterItem must be used within FooterItemProvider");
  return context.removeItem;
};

const Footer: FC = ({}) => {
  const isFullscreen = useIsFullscreen();
  const [isVisible, setIsVisible] = useState(true);
  const items = useFooterItems();

  useEffect(() => {
    if (!isFullscreen) {
      setIsVisible(true);
      return;
    }

    const handleMouseMove = (e: MouseEvent) => {
      setIsVisible(e.clientY > window.innerHeight - 50);
    };

    window.addEventListener("mousemove", handleMouseMove);
    return () => window.removeEventListener("mousemove", handleMouseMove);
  }, [isFullscreen]);

  return (
    <footer className={`h-5 bg-base border-t flex items-center transition-transform duration-200 ${isFullscreen && !isVisible ? "translate-y-full" : "translate-y-0"}`}>
      {items.map((item, index) => (
        <div key={item.id} className="flex items-center h-full">
          {index > 0 && <div className="h-full w-px bg-border" />}
          <Action id={item.id} onClick={item.onClick} level="base" className="h-full rounded-none border-0">
            {item.icon && <item.icon className="size-3" />}
            {item.label && <span className="text-xs">{item.label}</span>}
          </Action>
        </div>
      ))}
    </footer>
  );
};

export default Footer;
