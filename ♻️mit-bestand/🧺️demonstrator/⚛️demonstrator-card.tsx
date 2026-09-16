/** @emoji 🃏️ Window-silhouette overview card for one demonstrator pane — icon title chip, no drag handle. */

import { cn, Icon, WindowChrome, windowBodyFrameClass, windowChromeTitleChipClass } from "@semio-tech/ui-react";
import type { DemonstratorPaneSpec } from "./🪧️brand.ts";

export function DemonstratorCard({
  pane,
  active,
  onClick,
  onMouseEnter,
  onMouseLeave,
  className,
}: {
  readonly pane: DemonstratorPaneSpec;
  readonly active?: boolean;
  readonly onClick: () => void;
  readonly onMouseEnter?: () => void;
  readonly onMouseLeave?: () => void;
  readonly className?: string;
}) {
  return (
    <button
      type="button"
      data-demonstrator-pane-card=""
      data-pane-id={pane.id}
      onClick={onClick}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      className={cn(
        "pointer-events-auto group w-full max-w-[15rem] cursor-pointer border-0 bg-transparent p-0 text-left outline-none",
        "transition-transform duration-200",
        "hover:-translate-y-0.5 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background",
        active && "-translate-y-0.5",
        className,
      )}
    >
      <WindowChrome
        level="dialog"
        active={active}
        stackSlot="demonstrator-pane-card-stack"
        stackClassName="w-full min-w-0"
        titleChips={
          <div data-slot="demonstrator-pane-card-title-chip" className={cn(windowChromeTitleChipClass, "flex min-w-0 items-center gap-single px-single")}>
            <Icon icon={pane.icon} size="small" className="shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" title={pane.label} />
            <span className="truncate text-sm font-medium text-foreground">{pane.label}</span>
          </div>
        }
        body={
          <div data-slot="demonstrator-pane-card-body" className="flex flex-col items-center gap-double text-center">
            <span className="text-sm text-muted-foreground">{pane.tagline}</span>
            <span className="inline-flex items-center gap-single text-sm font-medium text-muted-foreground transition-colors group-hover:text-foreground">
              Demonstrator öffnen
              <Icon icon="chevron-right" size="small" className="transition-transform group-hover:translate-x-0.5" />
            </span>
          </div>
        }
        bodyClassName={cn("p-double shadow-lg", windowBodyFrameClass)}
      />
    </button>
  );
}
