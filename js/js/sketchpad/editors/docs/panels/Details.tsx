// #region Header

// Details.tsx

// 2025 Ueli Saluz

// #endregion

import { Hash } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { TreeContent, TreeItem, TreeSection } from "../../../../elements/aggregation/Tree";
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

const HeadingTree: FC<{ headings: HeadingNode[]; onNavigate?: (id: string) => void }> = ({ headings, onNavigate }) => {
  return (
    <>
      {headings.map((heading) => (
        <TreeItem key={heading.id}>
          <TreeContent>
            <div
              className="flex items-center gap-2 cursor-pointer hover:text-foreground transition-colors"
              onClick={() => {
                if (onNavigate) {
                  onNavigate(heading.id);
                } else {
                  const element = document.getElementById(heading.id);
                  element?.scrollIntoView({ behavior: "smooth" });
                }
              }}
            >
              <Hash className="w-3 h-3" />
              <span className="text-sm" style={{ paddingLeft: `${(heading.level - 1) * 0.5}rem` }}>
                {heading.text}
              </span>
            </div>
          </TreeContent>
          {heading.children && heading.children.length > 0 && <HeadingTree headings={heading.children} onNavigate={onNavigate} />}
        </TreeItem>
      ))}
    </>
  );
};

const Details: FC<DetailsProps> = ({ headings: propsHeadings, onNavigate }) => {
  const { t } = useTranslation();
  const { headings: contextHeadings } = useHeadings();
  const headings = propsHeadings || contextHeadings;

  if (headings.length === 0) {
    return (
      <TreeSection label={t("docs.onThisPage")} defaultOpen={true}>
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">{t("docs.noHeadings")}</p>
          </TreeContent>
        </TreeItem>
      </TreeSection>
    );
  }

  return (
    <TreeSection label={t("docs.onThisPage")} defaultOpen={true}>
      <HeadingTree headings={headings} onNavigate={onNavigate} />
    </TreeSection>
  );
};

export default Details;
