// #region Header

// Page.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, Suspense, useEffect, useRef, useState } from "react";
import Page from "../../../../elements/docs/Page";
import { useFocus } from "../../../Navbar";
import { MDXProvider } from "../mdx-provider";

interface PageCanvasProps {
  MDXContent?: React.ComponentType;
  frontmatter?: any;
}

const PageCanvas: FC<PageCanvasProps> = ({ MDXContent, frontmatter }) => {
  const { setFocusItems, setOnFocusItem } = useFocus();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const containerRef = useRef<HTMLDivElement>(null);
  const prevItemsRef = useRef<string>("");

  useEffect(() => {
    if (containerRef.current) {
      const headings = containerRef.current.querySelectorAll("h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]");
      const items = Array.from(headings).map((heading) => ({
        id: heading.id,
        label: heading.textContent || heading.id,
        category: heading.tagName,
      }));
      // Only update if the items have actually changed
      const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
      if (prevItemsRef.current !== itemsKey) {
        prevItemsRef.current = itemsKey;
        setFocusItems(items);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [MDXContent]);

  useEffect(() => {
    const handleFocus = (itemId: string) => {
      setFocusedItemId(itemId);
    };
    setOnFocusItem(handleFocus);
    return () => setOnFocusItem(undefined);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div ref={containerRef} className="h-full">
      <Page frontmatter={frontmatter} focusedItemId={focusedItemId} onFocusComplete={() => setFocusedItemId(undefined)}>
        <MDXProvider>
          <Suspense fallback={<div className="text-muted-foreground">Loading...</div>}>{MDXContent ? <MDXContent /> : <p className="text-muted-foreground">No content available</p>}</Suspense>
        </MDXProvider>
      </Page>
    </div>
  );
};

export default PageCanvas;
