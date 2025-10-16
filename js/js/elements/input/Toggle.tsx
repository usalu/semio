// #region Header

// Toggle.tsx

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

import * as TogglePrimitive from "@radix-ui/react-toggle";
import { cva } from "class-variance-authority";
import { ChevronDown } from "lucide-react";
import * as React from "react";

import { cn } from "../../semio";
import { Popover, PopoverAnchor, PopoverContent } from "../Popover";
import { Tooltip, TooltipContent, TooltipTrigger } from "../display/Tooltip";
import { Action } from "./Action";

const toggleVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-2 text-sm font-medium disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap data-[state=on]:bg-primary data-[state=on]:text-primary-foreground data-[state=on]:hover:bg-primary/90 data-[state=on]:hover:text-primary-foreground h-9 px-2 py-2 min-w-9",
  {
    variants: {
      variant: {
        default: "bg-transparent",
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

export interface ToggleItem<T extends string> {
  value: T;
  label: React.ReactNode;
  tooltip?: string;
  hotkey?: string;
}

interface ToggleStandardProps extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type"> {
  type?: "default";
  tooltip?: string;
  tooltipPressed?: string;
  hotkey?: string;
  label?: string;
  level?: "base" | "panel" | "temporary";
}

interface ToggleCycleProps<T extends string> extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type"> {
  type: "cycle";
  value?: T;
  onValueChange?: (value: T) => void;
  items: ToggleItem<T>[];
  tooltip?: string;
  hotkey?: string;
  label?: string;
  level?: "base" | "panel" | "temporary";
}

interface ToggleDropdownProps<T extends string> extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type"> {
  type: "dropdown";
  value?: T;
  onValueChange?: (value: T) => void;
  items: ToggleItem<T>[];
  tooltip?: string;
  hotkey?: string;
  label?: string;
  placeholder?: string;
  dropdownTooltip?: string;
  level?: "base" | "panel" | "temporary";
}

interface ToggleWithActionProps extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type"> {
  type: "withAction";
  actionIcon: React.ReactNode;
  onActionClick: () => void;
  tooltip?: string;
  hotkey?: string;
  label?: string;
  actionTooltip?: string;
  level?: "base" | "panel" | "temporary";
}

type ToggleProps<T extends string = string> = ToggleStandardProps | ToggleCycleProps<T> | ToggleDropdownProps<T> | ToggleWithActionProps;

function Toggle<T extends string = string>(props: ToggleProps<T>) {
  // Cycle type
  if ("type" in props && props.type === "cycle") {
    const { className, value, onValueChange, items, tooltip, hotkey, label, level = "base", pressed, onPressedChange, type, ...restProps } = props;

    if (!items || items.length === 0) return null;

    const currentIndex = Math.max(
      0,
      items.findIndex((item) => item.value === value),
    );
    const currentItem = items[currentIndex];

    const handleCycle = (e: React.MouseEvent) => {
      const nextIndex = (currentIndex + 1) % items.length;
      const nextValue = items[nextIndex].value;
      onValueChange?.(nextValue);
    };

    const toggleElement = (
      <TogglePrimitive.Root
        data-slot="toggle"
        className={cn(toggleVariants({ level }), "border", className)}
        pressed={pressed}
        onPressedChange={(newPressed) => {
          onPressedChange?.(newPressed);
          if (newPressed) {
            handleCycle({} as React.MouseEvent);
          }
        }}
        {...restProps}
      >
        {currentItem.label}
      </TogglePrimitive.Root>
    );

    const activeTooltip = currentItem.tooltip || tooltip;
    const activeHotkey = currentItem.hotkey || hotkey;

    const wrappedToggle = activeTooltip ? (
      <Tooltip>
        <TooltipTrigger asChild>{toggleElement}</TooltipTrigger>
        <TooltipContent>
          {activeTooltip}
          {activeHotkey && <span className="text-xs ml-1 opacity-60">({activeHotkey})</span>}
        </TooltipContent>
      </Tooltip>
    ) : (
      toggleElement
    );

    if (label) {
      return (
        <div className="flex items-center gap-2 min-w-0">
          <span className="text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate" title={label}>
            {label}
          </span>
          {wrappedToggle}
        </div>
      );
    }

    return wrappedToggle;
  }

  // WithAction type
  if ("type" in props && props.type === "withAction") {
    const { className, actionIcon, onActionClick, tooltip, hotkey, label, actionTooltip, level = "base", type, ...restProps } = props;

    const mainContent = tooltip ? (
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="flex items-center gap-2 flex-1 min-w-0">{restProps.children}</span>
        </TooltipTrigger>
        <TooltipContent>
          {tooltip}
          {hotkey && <span className="text-xs ml-1 opacity-60">({hotkey})</span>}
        </TooltipContent>
      </Tooltip>
    ) : (
      <span className="flex items-center gap-2 flex-1 min-w-0">{restProps.children}</span>
    );

    const toggleElement = (
      <TogglePrimitive.Root data-slot="toggle" className={cn(toggleVariants({ level }), "border gap-1 pr-1 [&:has(button:hover)]:bg-transparent", className)} {...restProps}>
        {mainContent}
        <Action
          level={level}
          onClick={(e) => {
            e.stopPropagation();
            onActionClick();
          }}
          tooltip={actionTooltip}
        >
          {actionIcon}
        </Action>
      </TogglePrimitive.Root>
    );

    const wrappedToggle = toggleElement;

    if (label) {
      return (
        <div className="flex items-center gap-2 min-w-0">
          <span className="text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate" title={label}>
            {label}
          </span>
          {wrappedToggle}
        </div>
      );
    }

    return wrappedToggle;
  }

  // Dropdown type
  if ("type" in props && props.type === "dropdown") {
    const { className, value, onValueChange, items, tooltip, hotkey, label, placeholder = "Select...", dropdownTooltip, level = "base", pressed, onPressedChange, type, ...restProps } = props;

    if (!items || items.length === 0) return null;

    const [open, setOpen] = React.useState(false);
    const currentItem = items.find((item) => item.value === value);

    const handleSelect = (selectedValue: T) => {
      onValueChange?.(selectedValue);
      setOpen(false);
    };

    const activeTooltip = currentItem?.tooltip || tooltip;
    const activeHotkey = currentItem?.hotkey || hotkey;

    const mainContent = activeTooltip ? (
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="truncate flex-1 min-w-0">{currentItem?.label || placeholder}</span>
        </TooltipTrigger>
        <TooltipContent>
          {activeTooltip}
          {activeHotkey && <span className="text-xs ml-1 opacity-60">({activeHotkey})</span>}
        </TooltipContent>
      </Tooltip>
    ) : (
      <span className="truncate flex-1 min-w-0">{currentItem?.label || placeholder}</span>
    );

    const toggleElement = (
      <TogglePrimitive.Root
        data-slot="toggle"
        className={cn(toggleVariants({ level }), "border gap-1 pr-1 [&:has(button:hover)]:bg-transparent", className)}
        pressed={pressed}
        onPressedChange={(newPressed) => {
          if (!open) {
            onPressedChange?.(newPressed);
          }
        }}
        {...restProps}
      >
        {mainContent}
        <Action
          level={level}
          onClick={(e) => {
            e.stopPropagation();
            setOpen(!open);
          }}
          tooltip={dropdownTooltip}
        >
          <ChevronDown className="size-3 opacity-50" />
        </Action>
      </TogglePrimitive.Root>
    );

    const wrappedToggle = toggleElement;

    const dropdownElement = (
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverAnchor asChild>{wrappedToggle}</PopoverAnchor>
        <PopoverContent className="w-auto p-0" align="start" sideOffset={4}>
          <div className="flex flex-col">
            {items.map((item) => {
              const isSelected = item.value === value;
              const itemHoverClass = level === "panel" ? "hover:bg-hover-panel focus:bg-hover-panel" : level === "temporary" ? "hover:bg-hover-temporary focus:bg-hover-temporary" : "hover:bg-hover-base focus:bg-hover-base";
              const itemButton = (
                <button
                  key={item.value}
                  type="button"
                  onClick={() => handleSelect(item.value)}
                  className={cn("flex items-center justify-center text-sm outline-none transition-colors", toggleVariants({ level }), "border-0 min-w-0 w-auto", itemHoverClass, isSelected && "bg-primary/10 font-medium")}
                >
                  {item.label}
                </button>
              );

              if (item.tooltip) {
                return (
                  <Tooltip key={item.value}>
                    <TooltipTrigger asChild>{itemButton}</TooltipTrigger>
                    <TooltipContent side="right">
                      {item.tooltip}
                      {item.hotkey && <span className="text-xs ml-1 opacity-60">({item.hotkey})</span>}
                    </TooltipContent>
                  </Tooltip>
                );
              }

              return itemButton;
            })}
          </div>
        </PopoverContent>
      </Popover>
    );

    if (label) {
      return (
        <div className="flex items-center gap-2 min-w-0">
          <span className="text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate" title={label}>
            {label}
          </span>
          {dropdownElement}
        </div>
      );
    }

    return dropdownElement;
  }

  // Standard toggle
  const { className, tooltip, tooltipPressed, hotkey, label, level = "base", type, pressed, ...restProps } = props as ToggleStandardProps;

  const activeTooltip = pressed && tooltipPressed ? tooltipPressed : tooltip;

  const toggleElement = <TogglePrimitive.Root data-slot="toggle" className={cn(toggleVariants({ level }), "border", className)} pressed={pressed} {...restProps} />;

  const wrappedToggle = activeTooltip ? (
    <Tooltip>
      <TooltipTrigger asChild>{toggleElement}</TooltipTrigger>
      <TooltipContent>
        {activeTooltip}
        {hotkey && <span className="text-xs ml-1 opacity-60">({hotkey})</span>}
      </TooltipContent>
    </Tooltip>
  ) : (
    toggleElement
  );

  if (label) {
    return (
      <div className="flex items-center gap-2 min-w-0">
        <span className="text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate" title={label}>
          {label}
        </span>
        {wrappedToggle}
      </div>
    );
  }

  return wrappedToggle;
}

export { Toggle, toggleVariants };
export type { ToggleProps };
