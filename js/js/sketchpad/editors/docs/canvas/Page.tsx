// #region Header

// Page.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, Suspense } from "react";
import Page from "../../../../elements/docs/Page";
import { MDXProvider } from "../mdx-provider";

interface PageCanvasProps {
  MDXContent?: React.ComponentType;
  frontmatter?: any;
}

const PageCanvas: FC<PageCanvasProps> = ({ MDXContent, frontmatter }) => {
  return (
    <Page frontmatter={frontmatter}>
      <MDXProvider>
        <Suspense fallback={<div className="text-muted-foreground">Loading...</div>}>{MDXContent ? <MDXContent /> : <p className="text-muted-foreground">No content available</p>}</Suspense>
      </MDXProvider>
    </Page>
  );
};

export default PageCanvas;
