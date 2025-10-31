// #region Header

// Page.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, ReactNode, useEffect, useRef } from "react";
import { ScrollArea } from "../aggregation/ScrollArea";

export interface PageFrontmatter {
  title?: string;
  description?: string;
  icon?: string;
  order?: number;
  concepts?: string[];
  sidebar?: {
    label?: string;
    badge?: string;
  };
  draft?: boolean;
}

export interface PageProps {
  frontmatter?: PageFrontmatter;
  children: ReactNode;
  className?: string;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  footer?: ReactNode;
}

const Page: FC<PageProps> = ({ frontmatter, children, className = "", focusedItemId, onFocusComplete, footer }) => {
  const scrollAreaRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
      const headingElement = scrollAreaRef.current.querySelector(`[id="${focusedItemId}"]`);
      if (headingElement) {
        headingElement.scrollIntoView({ behavior: "smooth", block: "start" });
        if (onFocusComplete) {
          setTimeout(() => onFocusComplete(), 600);
        }
      }
    }
  }, [focusedItemId, onFocusComplete]);

  return (
    <ScrollArea ref={scrollAreaRef} className={`h-full ${className}`}>
      <article className="prose prose-sm max-w-4xl mx-auto p-6 md:p-8">
        {frontmatter?.description && <p className="text-lg text-muted-foreground mb-8">{frontmatter.description}</p>}
        <div className="docs-content">{children}</div>
        {footer && <div className="not-prose mt-8">{footer}</div>}
      </article>
    </ScrollArea>
  );
};

export default Page;
