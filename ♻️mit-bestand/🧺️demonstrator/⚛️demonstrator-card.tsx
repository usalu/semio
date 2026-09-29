/** 🃏️ Overview card for one demonstrator pane — the shared {@link OverviewCard} as a whole-card button: icon title chip,
 * bold tagline, description paragraphs and open chip. */

import { cn, Icon, OverviewCard, OverviewCardOpenChip, registerUiTranslationBundles, uiDataLabel, useLabel } from "@semio-tech/ui-react";
import { demonstratorPaneDescriptionParagraphs, type DemonstratorPaneSpec } from "./🪧️brand.ts";

//#region 🌐️DemonstratorCardLabels
/** 🌐️ The card's own chrome string, for English AND German (no default language) — the German lock picks German. */
const demonstratorCardUiLabel = registerUiTranslationBundles({
  en: { translation: { demonstrator: { card: { open: { label: { normal: "Open demonstrator", beginner: "Open this demonstrator" } } } } } },
  de: { translation: { demonstrator: { card: { open: { label: { normal: "Demonstrator öffnen", beginner: "Diesen Demonstrator öffnen" } } } } } },
});
//#endregion 🌐️DemonstratorCardLabels

export function DemonstratorCard({
  pane,
  lifted,
  onClick,
  className,
}: {
  readonly pane: DemonstratorPaneSpec;
  /** 🎈️ Lifted while its app is revealed — never maps to window `active` (that paints the primary silhouette stroke). */
  readonly lifted?: boolean;
  readonly onClick: () => void;
  readonly className?: string;
}) {
  const bodyParagraphs = demonstratorPaneDescriptionParagraphs(pane.description);
  const openLabel = useLabel(demonstratorCardUiLabel("demonstrator.card.open"));

  return (
    <OverviewCard
      as="button"
      slot="demonstrator-pane-card"
      lifted={lifted}
      onClick={onClick}
      className={cn("max-w-sm", className)}
      icon={<Icon icon={pane.icon} size="small" className="shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" title={uiDataLabel(pane.label)} />}
      title={pane.label}
      contentClassName="max-w-sm"
      footerRight={<OverviewCardOpenChip slot="demonstrator-pane-card">{openLabel}</OverviewCardOpenChip>}
    >
      <span data-slot="demonstrator-pane-card-tagline" className="mb-double block text-xs font-medium leading-normal text-foreground">
        {pane.tagline}
      </span>
      {bodyParagraphs.length > 0 && (
        <span data-slot="introduction-body" className="flex flex-col gap-double">
          {bodyParagraphs.map((paragraph, index) => (
            <span key={index} data-slot="introduction-body-paragraph" className="block whitespace-pre-line text-xs leading-normal text-muted-foreground">
              {paragraph}
            </span>
          ))}
        </span>
      )}
    </OverviewCard>
  );
}
