// #region Header

// Page.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, ReactNode } from "react";
import { ScrollArea } from "../aggregation/ScrollArea";

export interface PageFrontmatter {
  title?: string;
  description?: string;
  sidebar?: {
    order?: number;
    label?: string;
    badge?: string;
  };
  draft?: boolean;
}

export interface PageProps {
  frontmatter?: PageFrontmatter;
  children: ReactNode;
  className?: string;
}

const Page: FC<PageProps> = ({ frontmatter, children, className = "" }) => {
  return (
    <ScrollArea className={`h-full ${className}`}>
      <article className="prose prose-sm max-w-4xl mx-auto p-6 md:p-8">
        {frontmatter?.title && <h1 className="text-4xl font-bold mb-2">{frontmatter.title}</h1>}
        {frontmatter?.description && <p className="text-lg text-muted-foreground mb-8">{frontmatter.description}</p>}
        <div className="docs-content">{children}</div>
      </article>
    </ScrollArea>
  );
};

export default Page;
