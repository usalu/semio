// #region Header

// Page.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, Suspense, useEffect, useRef, useState } from "react";
import Page from "../../../../elements/windows/Page";
import { useFocus } from "../../../Navbar";
import { MDXProvider, useHeadings } from "../mdx-provider";

interface PageCanvasProps {
  MDXContent?: React.ComponentType;
  frontmatter?: any;
}

const PageCanvas: FC<PageCanvasProps> = ({ MDXContent, frontmatter }) => {
  const { setFocusItems, setOnFocusItem } = useFocus();
  const { clearHeadings, registerHeading } = useHeadings();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const containerRef = useRef<HTMLDivElement>(null);
  const prevItemsRef = useRef<string>("");

  // Clear headings when MDXContent changes
  useEffect(() => {
    clearHeadings();
  }, [MDXContent, clearHeadings]);

  useEffect(() => {
    const extractHeadings = () => {
      if (containerRef.current) {
        const headings = containerRef.current.querySelectorAll("h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]");

        if (headings.length > 0) {
          const items = Array.from(headings).map((heading) => ({
            id: heading.id,
            label: heading.textContent || heading.id,
            category: heading.tagName,
          }));

          // Update focus items
          const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
          if (prevItemsRef.current !== itemsKey) {
            prevItemsRef.current = itemsKey;
            setFocusItems(items);
          }

          // Register all headings with HeadingsProvider
          const headingNodes = items.map((item) => ({
            id: item.id,
            text: item.label,
            level: parseInt(item.category.substring(1)),
          }));

          headingNodes.forEach((node) => {
            registerHeading(node);
          });
        }
      }
    };

    // Use setTimeout to wait for MDX to render
    const timer = setTimeout(extractHeadings, 100);
    return () => clearTimeout(timer);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [MDXContent, registerHeading]);

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
