// #region Header

// Workbench.tsx

// 2025 Ueli Saluz

// #endregion

import { BookOpen, CheckCircle, Folder, GraduationCap, Lightbulb, Puzzle, Rocket, Star } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { TreeContent, TreeItem, TreeSection } from "../../../../elements/aggregation/Tree";
import { useDocs, useDocsCommands } from "../store";

const sectionIcons: Record<string, any> = {
  "getting-started": Rocket,
  tutorials: GraduationCap,
  integrations: Puzzle,
  manuals: BookOpen,
  theory: Lightbulb,
  showcases: Star,
};

const sectionLabels: Record<string, string> = {
  "getting-started": "🚀 Getting Started",
  tutorials: "📝 Tutorials",
  integrations: "🔀 Integrations",
  manuals: "📖 Manuals",
  theory: "📚 Theory",
  showcases: "🌟 Showcases",
};

interface WorkbenchProps {}

const Workbench: FC<WorkbenchProps> = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const docsState = useDocs();
  const { selectPage, toggleSection } = useDocsCommands();

  const sections = Object.keys(sectionLabels);

  return (
    <>
      {sections.map((section) => {
        const sectionState = docsState.sectionStates[section] || { isExpanded: false };
        const Icon = sectionIcons[section] || Folder;
        return (
          <TreeSection key={section} label={sectionLabels[section]} defaultOpen={sectionState.isExpanded} icon={Icon}>
            <TreeItem>
              <TreeContent>
                <div className="text-sm text-muted-foreground">
                  <p>{t(`docs.sections.${section}.description`, `Section: ${section}`)}</p>
                  {sectionState.progress !== undefined && (
                    <div className="flex items-center gap-2 mt-2">
                      <div className="flex-1 bg-accent h-2 rounded-full overflow-hidden">
                        <div className="bg-primary h-full transition-all" style={{ width: `${sectionState.progress}%` }} />
                      </div>
                      <span className="text-xs">{Math.round(sectionState.progress)}%</span>
                    </div>
                  )}
                  {sectionState.completedPages && sectionState.completedPages.length > 0 && (
                    <div className="flex items-center gap-1 mt-2 text-xs">
                      <CheckCircle className="w-3 h-3" />
                      <span>
                        {sectionState.completedPages.length} {t("docs.pagesCompleted")}
                      </span>
                    </div>
                  )}
                </div>
              </TreeContent>
            </TreeItem>
          </TreeSection>
        );
      })}
    </>
  );
};

export default Workbench;
