// #region Header

// Details.tsx

// 2025 Ueli Saluz

// #endregion

import { Hash } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { TreeItem } from "../../../../elements/aggregation/Tree";
import { useFocusSafe } from "../../../Navbar";
import { useHeadings } from "../mdx-provider";

export interface HeadingNode {
  id: string;
  text: string;
  level: number;
  children?: HeadingNode[];
}

interface DetailsProps {
  headings?: HeadingNode[];
  onNavigate?: (id: string) => void;
}

const buildHeadingHierarchy = (flatHeadings: HeadingNode[]): HeadingNode[] => {
  const root: HeadingNode[] = [];
  const stack: HeadingNode[] = [];

  flatHeadings.forEach((heading) => {
    const node: HeadingNode = { ...heading, children: [] };

    // Find the correct parent by going up the stack
    while (stack.length > 0 && stack[stack.length - 1].level >= node.level) {
      stack.pop();
    }

    if (stack.length === 0) {
      // Top-level heading
      root.push(node);
    } else {
      // Add as child to the last item in stack
      const parent = stack[stack.length - 1];
      if (!parent.children) parent.children = [];
      parent.children.push(node);
    }

    stack.push(node);
  });

  return root;
};

const HeadingTree: FC<{ headings: HeadingNode[]; onNavigate?: (id: string) => void; triggerFocus?: (id: string) => void }> = ({ headings, onNavigate, triggerFocus }) => {
  return (
    <>
      {headings.map((heading) => (
        <TreeItem
          key={heading.id}
          label={heading.text}
          icon={<Hash className="w-3 h-3" />}
          defaultOpen={heading.children && heading.children.length > 0}
          onClick={() => {
            if (onNavigate) {
              onNavigate(heading.id);
            } else if (triggerFocus) {
              triggerFocus(heading.id);
            } else {
              const element = document.getElementById(heading.id);
              element?.scrollIntoView({ behavior: "smooth" });
            }
          }}
        >
          {heading.children && heading.children.length > 0 && <HeadingTree headings={heading.children} onNavigate={onNavigate} triggerFocus={triggerFocus} />}
        </TreeItem>
      ))}
    </>
  );
};

const Details: FC<DetailsProps> = ({ headings: propsHeadings, onNavigate }) => {
  const { t } = useTranslation();
  const { headings: contextHeadings } = useHeadings();
  const focusContext = useFocusSafe();
  const flatHeadings = propsHeadings || contextHeadings;

  if (flatHeadings.length === 0) {
    return (
      <div className="p-2">
        <p className="text-sm text-muted-foreground">{t("semio.docs.noHeadings")}</p>
      </div>
    );
  }

  // Build hierarchical structure from flat list
  const hierarchicalHeadings = buildHeadingHierarchy(flatHeadings);

  return (
    <div className="p-2">
      <HeadingTree headings={hierarchicalHeadings} onNavigate={onNavigate} triggerFocus={focusContext?.triggerFocusItem} />
    </div>
  );
};

export default Details;
