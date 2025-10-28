// #region Header

// Page.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, Suspense, useEffect, useMemo, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router";
import { Tree } from "../../../../elements/aggregation/Tree";
import PageNavigation from "../../../../elements/navigation/PageNavigation";
import Page from "../../../../elements/windows/Page";
import { useFocus } from "../../../Navbar";
import { MDXProvider, useHeadings } from "../mdx-provider";
import { docsRegistry } from "../registry";

interface PageCanvasProps {
  MDXContent?: React.ComponentType;
  frontmatter?: any;
}

const PageCanvas: FC<PageCanvasProps> = ({ MDXContent, frontmatter }) => {
  const location = useLocation();
  const navigate = useNavigate();
  const { setFocusItems, setOnFocusItem } = useFocus();
  const { clearHeadings, registerHeading } = useHeadings();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const containerRef = useRef<HTMLDivElement>(null);
  const prevItemsRef = useRef<string>("");

  // Get current page path (remove leading slash and "docs/" prefix if present)
  const currentPath = useMemo(() => {
    const path = location.pathname.replace(/^\//, "");
    return path;
  }, [location.pathname]);

  // Detect if this is an index page (folder page)
  const isIndexPage = useMemo(() => {
    return currentPath.endsWith("/index") || currentPath.split("/").pop() === currentPath.split("/")[0];
  }, [currentPath]);

  // Get section tree for index pages
  const sectionTree = useMemo(() => {
    if (!isIndexPage) return null;

    const path = location.pathname.replace(/^\/docs\//, "");
    const parts = path.split("/");
    const section = parts[0];

    const tree = docsRegistry.getSectionTree(section);
    return tree.length > 0 ? tree : null;
  }, [isIndexPage, location.pathname]);

  // Get navigation links
  const navigation = useMemo(() => {
    const prev = docsRegistry.getPreviousPage(currentPath);
    const next = docsRegistry.getNextPage(currentPath);
    return {
      prev: prev
        ? {
            path: prev.path,
            title: prev.title,
            section: prev.section,
          }
        : undefined,
      next: next
        ? {
            path: next.path,
            title: next.title,
            section: next.section,
          }
        : undefined,
    };
  }, [currentPath]);

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
      <Page
        frontmatter={frontmatter}
        focusedItemId={focusedItemId}
        onFocusComplete={() => setFocusedItemId(undefined)}
        footer={<PageNavigation prev={navigation.prev} next={navigation.next} />}
      >
        <MDXProvider>
          <Suspense fallback={<div className="text-muted-foreground">Loading...</div>}>
            {MDXContent ? <MDXContent /> : <p className="text-muted-foreground">No content available</p>}
          </Suspense>
        </MDXProvider>

        {/* Auto-inject section tree on index pages */}
        {sectionTree && (
          <Tree.Section
            nodes={sectionTree}
            currentPath={currentPath}
            onNavigate={(path) => navigate(`/${path}`)}
            as="div"
          />
        )}
      </Page>
    </div>
  );
};

export default PageCanvas;
