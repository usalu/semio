// #region Header

// Tooltip.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion
import * as TooltipPrimitive from "@radix-ui/react-tooltip";
import { BookOpen, GraduationCap } from "lucide-react";
import * as React from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router";
import { cn } from "../../semio";
import { Expertise } from "../../sketchpad/store";

export interface TooltipConfig {
  labelKey: string;
  manualPath?: string;
  tutorialPath?: string;
  hotkey?: string;
}

export interface IdTooltipData {
  label?: string;
  description?: string;
  descriptionBeginner?: string;
  manual?: string;
  tutorial?: string;
  hotkey?: string;
}

let getExpertiseFunction: (() => Expertise) | undefined;

export function setTooltipModeProvider(fn: () => Expertise) {
  getExpertiseFunction = fn;
}

export function useTooltipMode(): Expertise {
  if (!getExpertiseFunction) return Expertise.BEGINNER;
  return getExpertiseFunction();
}

function TooltipProvider({ delayDuration = 400, ...props }: React.ComponentProps<typeof TooltipPrimitive.Provider>) {
  return <TooltipPrimitive.Provider data-slot="tooltip-provider" delayDuration={delayDuration} {...props} />;
}

function Tooltip({ ...props }: React.ComponentProps<typeof TooltipPrimitive.Root>) {
  return (
    <TooltipProvider>
      <TooltipPrimitive.Root data-slot="tooltip" {...props} />
    </TooltipProvider>
  );
}

function TooltipTrigger({ className, asChild, ...props }: React.ComponentProps<typeof TooltipPrimitive.Trigger>) {
  return <TooltipPrimitive.Trigger data-slot="tooltip-trigger" asChild={asChild} className={cn(className)} {...props} />;
}

function TooltipContent({ className, sideOffset = 8, children, ...props }: React.ComponentProps<typeof TooltipPrimitive.Content>) {
  return (
    <TooltipPrimitive.Portal>
      <TooltipPrimitive.Content
        data-slot="tooltip-content"
        sideOffset={sideOffset}
        className={cn(
          "bg-temporary border border-accent-foreground text-foreground animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-fit origin-(--radix-tooltip-content-transform-origin) px-3 py-1.5 text-xs text-balance",
          className,
        )}
        {...props}
      >
        {children}
      </TooltipPrimitive.Content>
    </TooltipPrimitive.Portal>
  );
}

interface EnhancedTooltipContentProps {
  config: TooltipConfig;
  mode: Expertise;
}

function EnhancedTooltipContent({ config, mode }: EnhancedTooltipContentProps) {
  const { t } = useTranslation();

  if (mode === Expertise.EXPERT) return null;

  const { labelKey, manualPath, tutorialPath, hotkey } = config;
  const showManual = mode === Expertise.BEGINNER || mode === Expertise.NORMAL;
  const showTutorial = mode === Expertise.BEGINNER;

  const labelKeyToUse = mode === Expertise.BEGINNER ? `${labelKey}.beginner` : labelKey;
  const label = t(labelKeyToUse, { defaultValue: t(labelKey) });

  const fullManualPath = manualPath ? `/docs/manual/${manualPath}` : undefined;
  const fullTutorialPath = tutorialPath ? `/docs/tutorials/${tutorialPath}` : undefined;

  const handleHotkeyClick = () => {
    if (labelKey) {
      window.dispatchEvent(
        new CustomEvent("navigate-to-hotkey", {
          detail: { path: labelKey },
        }),
      );
    }
  };

  return (
    <div className="flex flex-col gap-2">
      <span>{label}</span>
      {(showManual && fullManualPath) || (showTutorial && fullTutorialPath) || hotkey ? (
        <div className="grid w-full grid-cols-3 items-center border-t border-accent-foreground pt-2 gap-2">
          {showManual && fullManualPath ? (
            <Link to={fullManualPath} className="flex items-center gap-1 cursor-pointer text-foreground transition-colors px-1 py-0.5 hover:bg-hover-temporary">
              <BookOpen className="size-3" />
              <span>{t("tooltip.manual")}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {showTutorial && fullTutorialPath ? (
            <Link to={fullTutorialPath} className="flex items-center gap-1 cursor-pointer text-foreground transition-colors px-1 py-0.5 hover:bg-hover-temporary">
              <GraduationCap className="size-3" />
              <span className="block text-center">{t("tooltip.tutorial")}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {hotkey ? (
            <kbd onClick={handleHotkeyClick} className="bg-panel border border-accent-foreground text-muted-foreground px-1.5 py-0.5 text-2xs font-mono justify-self-end cursor-pointer hover:bg-hover-panel">
              {hotkey}
            </kbd>
          ) : (
            <span className="block" />
          )}
        </div>
      ) : null}
    </div>
  );
}

interface IdTooltipContentProps {
  id: string;
  mode: Expertise;
}

function IdTooltipContent({ id, mode }: IdTooltipContentProps) {
  const { t } = useTranslation();

  if (mode === Expertise.EXPERT) return null;

  const label = t(`${id}.label`, { defaultValue: "" });
  const description = mode === Expertise.BEGINNER ? t(`${id}.beginner`, { defaultValue: label }) : label;

  const manualPath = t(`${id}.manual`, { defaultValue: "" });
  const tutorialPath = t(`${id}.tutorial`, { defaultValue: "" });
  const hotkey = t(`${id}.hotkey`, { defaultValue: "" });

  const showManual = (mode === Expertise.BEGINNER || mode === Expertise.NORMAL) && manualPath;
  const showTutorial = mode === Expertise.BEGINNER && tutorialPath;

  const fullManualPath = manualPath ? `/docs/manual/${manualPath}` : undefined;
  const fullTutorialPath = tutorialPath ? `/docs/tutorials/${tutorialPath}` : undefined;

  const displayText = description || label;

  const handleHotkeyClick = () => {
    window.dispatchEvent(
      new CustomEvent("navigate-to-hotkey", {
        detail: { path: id },
      }),
    );
  };

  return (
    <div className="flex flex-col gap-2">
      <span>{displayText}</span>
      {(showManual && fullManualPath) || (showTutorial && fullTutorialPath) || hotkey ? (
        <div className="grid w-full grid-cols-3 items-center border-t border-accent-foreground pt-2 gap-2">
          {showManual && fullManualPath ? (
            <Link to={fullManualPath} className="flex items-center gap-1 cursor-pointer text-foreground transition-colors px-1 py-0.5 hover:bg-hover-temporary">
              <BookOpen className="size-3" />
              <span>{t("tooltip.manual")}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {showTutorial && fullTutorialPath ? (
            <Link to={fullTutorialPath} className="flex items-center gap-1 cursor-pointer text-foreground transition-colors px-1 py-0.5 hover:bg-hover-temporary">
              <GraduationCap className="size-3" />
              <span className="block text-center">{t("tooltip.tutorial")}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {hotkey ? (
            <kbd onClick={handleHotkeyClick} className="bg-panel border border-accent-foreground text-muted-foreground px-1.5 py-0.5 text-2xs font-mono justify-self-end cursor-pointer hover:bg-hover-panel">
              {hotkey}
            </kbd>
          ) : (
            <span className="block" />
          )}
        </div>
      ) : null}
    </div>
  );
}

interface SemioTooltipProps {
  children: React.ReactElement;
  config: TooltipConfig;
  mode: Expertise;
}

function SemioTooltip({ children, config, mode }: SemioTooltipProps) {
  if (mode === Expertise.EXPERT) return children;
  return (
    <Tooltip>
      <TooltipTrigger asChild>{children}</TooltipTrigger>
      <TooltipContent>
        <EnhancedTooltipContent config={config} mode={mode} />
      </TooltipContent>
    </Tooltip>
  );
}

interface IdSemioTooltipProps {
  children: React.ReactElement;
  id: string;
  mode: Expertise;
}

function IdSemioTooltip({ children, id, mode }: IdSemioTooltipProps) {
  if (mode === Expertise.EXPERT) return children;
  return (
    <Tooltip>
      <TooltipTrigger asChild>{children}</TooltipTrigger>
      <TooltipContent>
        <IdTooltipContent id={id} mode={mode} />
      </TooltipContent>
    </Tooltip>
  );
}

export { EnhancedTooltipContent, IdSemioTooltip, IdTooltipContent, SemioTooltip, Tooltip, TooltipContent, TooltipProvider, TooltipTrigger };
