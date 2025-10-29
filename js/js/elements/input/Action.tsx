// #region Header

// Action.tsx

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
"use client";

import { cva, type VariantProps } from "class-variance-authority";
import * as React from "react";

import { cn } from "../../semio";
import { IdTooltipContent, Tooltip, TooltipContent, TooltipTrigger, useTooltipMode } from "../display/Tooltip";

const actionVariants = cva(
  "text-foreground inline-flex items-center justify-center shrink-0 transition-all cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-3 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive border size-5 p-0.5",
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

interface ActionProps extends VariantProps<typeof actionVariants>, Omit<React.ComponentProps<"button">, "children"> {
  as?: "button" | "div";
  loading?: boolean;
  children: React.ReactNode;
  id?: string;
}

function Action({ className, variant, level, id, children, as: Component = "button", ...props }: ActionProps) {
  const mode = useTooltipMode();
  const buttonElement = (
    <Component
      data-slot="action"
      type={Component === "button" ? "button" : undefined}
      role={Component === "div" ? "button" : undefined}
      tabIndex={Component === "div" ? 0 : undefined}
      className={cn(actionVariants({ variant, level }), className)}
      {...(props as any)}
    >
      {children}
    </Component>
  );

  if (id) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{buttonElement}</TooltipTrigger>
        <TooltipContent>
          <IdTooltipContent id={id} mode={mode} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return buttonElement;
}

export { Action, actionVariants };
export type { ActionProps };
