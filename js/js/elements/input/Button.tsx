// #region Header

// Button.tsx

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
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import * as React from "react";

import { cn } from "../../semio";
import { EnhancedTooltipContent, Tooltip, TooltipConfig, TooltipContent, TooltipTrigger, useTooltipMode } from "../display/Tooltip";

const buttonVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-all cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive h-9 px-4 py-2 has-[>svg]:px-3",
  {
    variants: {
      variant: {
        default: "hover:bg-hover-base",
        primary: "bg-accent text-accent-foreground hover:bg-accent/90",
        secondary: "bg-accent text-accent-foreground hover:bg-accent/80",
        destructive: "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
        ghost: "hover:bg-hover-base",
        link: "text-accent underline-offset-4 hover:underline",
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

type ButtonProps = React.ComponentProps<"button"> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean;
    tooltip?: TooltipConfig;
  };

function Button({ className, variant, level, asChild = false, tooltip, ...props }: ButtonProps) {
  const Comp = asChild ? Slot : "button";
  const mode = useTooltipMode();

  const buttonElement = <Comp data-slot="button" className={cn(buttonVariants({ variant, level }), "border", className)} {...props} />;

  if (tooltip) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{buttonElement}</TooltipTrigger>
        <TooltipContent>
          <EnhancedTooltipContent config={tooltip} mode={mode} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return buttonElement;
}

export { Button, buttonVariants };
export type { ButtonProps };
