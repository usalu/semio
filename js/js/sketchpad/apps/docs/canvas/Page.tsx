// #region Header

// Page.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, Suspense, useEffect, useMemo, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router";
import { TreeItem } from "../../../../elements/aggregation/Tree";
import { TreeStateProvider } from "../../../../elements/aggregation/TreeStateProvider";
import PageNavigation from "../../../../elements/navigation/PageNavigation";
import Page from "../../../../elements/windows/Page";
import { useFocus } from "../../../Navbar";
import { MDXProvider, useHeadings } from "../mdx-provider";
import { docsRegistry, DocsPage } from "../registry";

interface PageCanvasProps {
  MDXContent?: React.ComponentType;
  frontmatter?: any;
}

interface TreeNode {
  name: string;
  page?: DocsPage;
  children: Map<string, TreeNode>;
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

  // Build tree structure for current section/folder
  const treeData = useMemo(() => {
    if (!isIndexPage) return null;

    const path = location.pathname.replace(/^\//, "");
    const parts = path.split("/").filter(Boolean);
    const section = parts[0];
    
    // Get all pages in this section
    const pages = docsRegistry.getPagesBySection(section);
    if (pages.length === 0) return null;

    // Build tree structure
    const root: TreeNode = { name: "root", children: new Map() };
    
    pages.forEach((page) => {
      const pageParts = page.path.replace(`${section}/`, "").split("/");
      let currentNode = root;
      
      pageParts.forEach((part, index) => {
        if (!currentNode.children.has(part)) {
          currentNode.children.set(part, {
            name: part,
            children: new Map(),
          });
        }
        currentNode = currentNode.children.get(part)!;
        
        // If this is the last part and it's "index", assign to parent
        if (index === pageParts.length - 1 && part === "index") {
          const parent = pageParts.length === 1 ? root : 
            pageParts.slice(0, -1).reduce((node, p) => node.children.get(p)!, root);
          parent.page = page;
        } else if (index === pageParts.length - 1) {
          currentNode.page = page;
        }
      });
    });
    
    return root.children.size > 0 ? root : null;
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

  // Render tree node recursively
  const renderTreeNode = (node: TreeNode): React.ReactNode[] => {
    const items: React.ReactNode[] = [];

    node.children.forEach((childNode, name) => {
      const hasChildren = childNode.children.size > 0;
      const hasPage = !!childNode.page;

      if (hasChildren) {
        // Folder with possible index page
        const folderLabel = childNode.page?.title || name
          .split("-")
          .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
          .join(" ");

        const isCurrentPage = !!(childNode.page && currentPath && childNode.page.path === currentPath);
        const folderIcon = childNode.page?.icon;

        items.push(
          <TreeItem
            key={childNode.page?.path || name}
            label={folderLabel}
            icon={folderIcon ? <span className="text-sm">{folderIcon}</span> : undefined}
            defaultOpen={true}
            isHighlighted={isCurrentPage}
            onClick={childNode.page ? () => {
              navigate(`/${childNode.page!.path}`);
            } : undefined}
          >
            {renderTreeNode(childNode)}
          </TreeItem>,
        );
      } else if (hasPage && childNode.page) {
        // Leaf page
        const isCurrentPage = !!(currentPath && childNode.page.path === currentPath);
        const pageIcon = childNode.page.icon;

        items.push(
          <TreeItem
            key={childNode.page.path}
            label={childNode.page.title}
            icon={pageIcon ? <span className="text-sm">{pageIcon}</span> : undefined}
            isHighlighted={isCurrentPage}
            onClick={() => {
              navigate(`/${childNode.page!.path}`);
            }}
          />,
        );
      }
    });

    return items;
  };

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
        {treeData && (
          <TreeStateProvider>
            <div className="not-prose my-8 p-6 rounded-lg border border-border bg-card">
              <h3 className="text-lg font-semibold mb-4">In this section</h3>
              <div className="flex flex-col gap-0.5">
                {renderTreeNode(treeData)}
              </div>
            </div>
          </TreeStateProvider>
        )}
      </Page>
    </div>
  );
};

export default PageCanvas;
