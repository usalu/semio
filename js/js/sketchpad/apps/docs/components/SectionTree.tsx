// #region Header

// SectionTree.tsx
// App-specific wrapper around Tree.Files for the docs app
//
// Architecture:
// - Tree.tsx exports Tree, Tree.Files (generic UI components)
// - SectionTree.tsx wraps Tree.Files with docs-specific logic (registry, routing)

// 2025 Ueli Saluz

// #endregion

import { FC } from "react";
import { useLocation, useNavigate } from "react-router";
import { Tree } from "../../../elements/aggregation/Tree";
import { docsRegistry } from "../registry";

interface SectionTreeProps {
  title?: string;
  section?: string;
}

/**
 * SectionTree - Docs app wrapper around Tree.Files
 *
 * Automatically fetches the file tree for a docs section and wires up
 * React Router navigation. For use in MDX files.
 *
 * @example
 * ```mdx
 * <SectionTree />
 * <SectionTree section="tutorials" title="Available Tutorials" />
 * ```
 */
const SectionTree: FC<SectionTreeProps> = ({ title, section }) => {
  const location = useLocation();
  const navigate = useNavigate();

  // Determine the section from the current path if not provided
  const currentSection =
    section ||
    (() => {
      const path = location.pathname.replace(/^\/docs\//, "");
      const parts = path.split("/");
      return parts[0];
    })();

  const currentPath = location.pathname.replace(/^\//, "");
  const tree = docsRegistry.getSectionTree(currentSection);

  const handleNavigate = (path: string) => {
    navigate(`/${path}`);
  };

  return <Tree.Files title={title} nodes={tree} currentPath={currentPath} onNavigate={handleNavigate} as="div" />;
};

export default SectionTree;
