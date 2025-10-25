// #region Header

// Page.tsx

// 2025 Ueli Saluz

// #endregion

import { FC } from "react";
import Page from "../../../../elements/docs/Page";

interface PageCanvasProps {
  content?: string;
  frontmatter?: any;
}

const PageCanvas: FC<PageCanvasProps> = ({ content, frontmatter }) => {
  return (
    <Page frontmatter={frontmatter}>
      <div dangerouslySetInnerHTML={{ __html: content || "<p>No content</p>" }} />
    </Page>
  );
};

export default PageCanvas;
