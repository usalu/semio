// #region Header

// ButtonGroup.tsx

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

import { cn } from "../../semio";
import { Tooltip, TooltipContent, TooltipTrigger } from "../display/Tooltip";

const buttonGroupItemVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-2 text-sm font-medium disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap",
  {
    variants: {
      variant: {
        default: "bg-transparent",
      },
      size: {
        default: "h-9 px-2 py-2 min-w-9",
        sm: "h-8 px-1.5 py-1.5 min-w-8",
        lg: "h-10 px-2.5 py-2.5 min-w-10",
      },
      level: {
        base: "hover:bg-hover-base",
        panel: "hover:bg-hover-panel",
        temporary: "hover:bg-hover-temporary",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
      level: "base",
    },
  },
);

const ButtonGroupContext = React.createContext<VariantProps<typeof buttonGroupItemVariants>>({
  size: "default",
  variant: "default",
  level: "base",
});

function ButtonGroup({ className, variant, size, level = "base", label, children, ...props }: React.ComponentProps<"div"> & VariantProps<typeof buttonGroupItemVariants> & { label?: string }) {
  const buttonGroupElement = (
    <div data-slot="button-group" data-variant={variant} data-size={size} data-level={level} className={cn("group/button-group flex w-fit items-center border overflow-hidden", className)} {...props}>
      <ButtonGroupContext.Provider value={{ variant, size, level }}>{children}</ButtonGroupContext.Provider>
    </div>
  );

  if (label) {
    return (
      <div className="flex items-center gap-2 min-w-0">
        <span className="text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate">{label}</span>
        {buttonGroupElement}
      </div>
    );
  }

  return buttonGroupElement;
}

function ButtonGroupItem({
  className,
  children,
  variant,
  size,
  level,
  tooltip,
  hotkey,
  ...props
}: React.ComponentProps<"button"> &
  VariantProps<typeof buttonGroupItemVariants> & {
    tooltip?: string;
    hotkey?: string;
  }) {
  const context = React.useContext(ButtonGroupContext);

  const buttonGroupItemElement = (
    <button
      data-slot="button-group-item"
      data-variant={context.variant || variant}
      data-size={context.size || size}
      data-level={context.level || level}
      className={cn(
        buttonGroupItemVariants({
          variant: context.variant || variant,
          size: context.size || size,
          level: context.level || level,
        }),
        "min-w-0 flex-1 shrink-0 focus:z-10 focus-visible:z-10 border-0 border-l first:border-l-0",
        className,
      )}
      {...props}
    >
      {children}
    </button>
  );

  if (tooltip) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>
          <span>{buttonGroupItemElement}</span>
        </TooltipTrigger>
        <TooltipContent>
          {tooltip}
          {hotkey && <span className="text-xs ml-1 opacity-60">({hotkey})</span>}
        </TooltipContent>
      </Tooltip>
    );
  }

  return buttonGroupItemElement;
}

export { ButtonGroup, ButtonGroupItem };
