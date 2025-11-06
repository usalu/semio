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
import { Check } from "lucide-react";
import * as React from "react";

import { cn } from "../../semio";
import { IdTooltipContent, Tooltip, TooltipContent, TooltipTrigger, useTooltipMode } from "../display/Tooltip";
import { Popover, PopoverContent, PopoverTrigger } from "../Popover";

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

interface ActionDropdownOption {
  value: string;
  icon: React.ReactNode;
  label?: string;
}

interface ActionDropdownProps extends Omit<VariantProps<typeof actionVariants>, "variant">, Omit<React.ComponentProps<"button">, "children"> {
  options: ActionDropdownOption[];
  value: string;
  onValueChange?: (value: string) => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
  id: string;
}

function ActionDropdown({ className, level, id, options, value, onValueChange, startTransaction, finalizeTransaction, ...props }: ActionDropdownProps) {
  const [open, setOpen] = React.useState(false);
  const mode = useTooltipMode();

  const selectedOption = options.find((option) => option.value === value);

  const handleOpenChange = (isOpen: boolean) => {
    if (isOpen && startTransaction) startTransaction();
    setOpen(isOpen);
    if (!isOpen && finalizeTransaction) finalizeTransaction();
  };

  const handleSelect = (optionValue: string) => {
    if (onValueChange) onValueChange(optionValue);
    setOpen(false);
  };

  const buttonElement = (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <button data-slot="action-dropdown" type="button" className={cn(actionVariants({ variant: "default", level }), className)} {...props}>
          {selectedOption?.icon}
        </button>
      </PopoverTrigger>
      <PopoverContent className="w-auto p-1 min-w-[120px]" align="start">
        <div className="flex flex-col">
          {options.map((option) => (
            <button
              key={option.value}
              onClick={() => handleSelect(option.value)}
              className={cn("flex items-center gap-2 px-2 py-1.5 text-xs cursor-selectable transition-colors", "hover:bg-hover-temporary outline-none focus-visible:bg-hover-temporary", value === option.value && "bg-active-temporary")}
            >
              <span className="flex items-center justify-center size-3">{option.icon}</span>
              {option.label && <span className="flex-1 text-left">{option.label}</span>}
              {value === option.value && <Check className="size-3 ml-auto" />}
            </button>
          ))}
        </div>
      </PopoverContent>
    </Popover>
  );

  return (
    <Tooltip>
      <TooltipTrigger asChild>{buttonElement}</TooltipTrigger>
      <TooltipContent>
        <IdTooltipContent id={id} mode={mode} />
      </TooltipContent>
    </Tooltip>
  );
}

export { Action, ActionDropdown, actionVariants };
export type { ActionDropdownOption, ActionDropdownProps, ActionProps };
