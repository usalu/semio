// #region Header

// ActionGroup.tsx

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
import { cva, type VariantProps } from "class-variance-authority";
import * as React from "react";
import { useTranslation } from "react-i18next";

import { cn } from "../../semio";
import { IdTooltipContent, Tooltip, TooltipContent, TooltipTrigger, useTooltipMode } from "../display/Tooltip";

const actionGroupItemVariants = cva(
  "text-foreground inline-flex items-center justify-center shrink-0 transition-all cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-3 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  {
    variants: {
      variant: {
        default: "bg-transparent",
        primary: "bg-accent text-accent-foreground hover:bg-accent/90",
        destructive: "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
      },
      level: {
        base: "hover:bg-hover-base",
        panel: "hover:bg-hover-panel",
        temporary: "hover:bg-hover-temporary",
      },
    },
    defaultVariants: {
      variant: "default",
      level: "base",
    },
  },
);

const ActionGroupContext = React.createContext<VariantProps<typeof actionGroupItemVariants>>({
  variant: "default",
  level: "base",
});

interface ActionGroupProps extends Omit<React.ComponentProps<"div">, "children"> {
  id: string;
  showLabel?: boolean;
  children: React.ReactNode;
  variant?: "default" | "primary" | "destructive";
  level?: "base" | "panel" | "temporary";
}

function ActionGroup({ className, variant, level = "base", id, showLabel, children, ...props }: ActionGroupProps) {
  const { t } = useTranslation();
  const mode = useTooltipMode();

  const actionGroupElement = (
    <div data-slot="action-group" data-variant={variant} data-level={level} className={cn("group/action-group flex w-fit items-stretch border border-border divide-x divide-border overflow-hidden h-5", className)} {...props}>
      <ActionGroupContext.Provider value={{ variant, level }}>{children}</ActionGroupContext.Provider>
    </div>
  );

  if (showLabel) {
    const label = t(`${id}.label`);
    return (
      <div className="group flex items-center gap-2 min-w-0 w-full">
        <Tooltip>
          <TooltipTrigger asChild>
            <span className="inline-flex h-5 items-center px-2 text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate cursor-pointer transition-colors group-hover:bg-hover-panel">{label}</span>
          </TooltipTrigger>
          <TooltipContent>
            <IdTooltipContent id={id} mode={mode} />
          </TooltipContent>
        </Tooltip>
        {actionGroupElement}
      </div>
    );
  }

  return actionGroupElement;
}

function ActionGroupItem({
  className,
  children,
  variant,
  level,
  id,
  ...props
}: React.ComponentProps<"button"> &
  VariantProps<typeof actionGroupItemVariants> & {
    id?: string;
  }) {
  const context = React.useContext(ActionGroupContext);
  const mode = useTooltipMode();

  const actionGroupItemElement = (
    <button
      data-slot="action-group-item"
      data-variant={context.variant || variant}
      data-level={context.level || level}
      className={cn(
        actionGroupItemVariants({
          variant: context.variant || variant,
          level: context.level || level,
        }),
        "min-w-0 flex-1 shrink-0 focus:z-10 focus-visible:z-10 border-0 !h-full size-5 p-0.5",
        className,
      )}
      {...props}
    >
      {children}
    </button>
  );

  if (id) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>
          <span>{actionGroupItemElement}</span>
        </TooltipTrigger>
        <TooltipContent>
          <IdTooltipContent id={id} mode={mode} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return actionGroupItemElement;
}

export { ActionGroup, ActionGroupItem };
