import { createContext, FC, ReactNode, useCallback, useContext, useEffect, useState } from "react";
import { useIsFullscreen } from "../../../store";
import { Tooltip, TooltipContent, TooltipTrigger } from "../Tooltip";

export interface FooterItem {
  id: string;
  content: ReactNode;
  tooltip?: string;
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
    <footer className={`h-5 bg-background border-t flex items-center transition-transform duration-200 ${isFullscreen && !isVisible ? "translate-y-full" : "translate-y-0"}`}>
      {items.map((item, index) => (
        <div key={item.id} className="flex items-center h-full">
          {index > 0 && <div className="h-full w-px bg-border" />}
          {item.tooltip ? (
            <Tooltip>
              <TooltipTrigger asChild>
                <div className="flex items-center h-full px-2 text-xs">{item.content}</div>
              </TooltipTrigger>
              <TooltipContent>{item.tooltip}</TooltipContent>
            </Tooltip>
          ) : (
            <div className="flex items-center h-full px-2 text-xs">{item.content}</div>
          )}
        </div>
      ))}
    </footer>
  );
};

export default Footer;
