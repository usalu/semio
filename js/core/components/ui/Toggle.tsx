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
import { cva, type VariantProps } from "class-variance-authority";
import * as React from "react";

import { cn } from "@semio/js/lib/utils";
import { Tooltip, TooltipContent, TooltipTrigger } from "./Tooltip";
import { Popover, PopoverAnchor, PopoverContent } from "./Popover";

const toggleVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-2 text-sm font-medium hover:bg-muted hover:text-muted-foreground disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap data-[state=on]:bg-primary data-[state=on]:text-primary-foreground data-[state=on]:hover:bg-primary data-[state=on]:hover:text-primary-foreground h-9 px-2 min-w-9",
  {
    variants: {
      variant: {
        default: "border bg-transparent hover:bg-accent hover:text-accent-foreground",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  },
);

export interface ToggleItem<T extends string> {
  value: T;
  label: React.ReactNode;
  tooltip?: string;
  hotkey?: string;
}

interface ToggleStandardProps extends React.ComponentProps<typeof TogglePrimitive.Root> {
  type?: "default";
  tooltip?: string;
  hotkey?: string;
  label?: string;
}

interface ToggleCycleProps<T extends string> extends React.ComponentProps<typeof TogglePrimitive.Root> {
  type: "cycle";
  value?: T;
  onValueChange?: (value: T) => void;
  items: ToggleItem<T>[];
  tooltip?: string;
  hotkey?: string;
  label?: string;
}

interface ToggleDropdownProps<T extends string> extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "pressed" | "onPressedChange"> {
  type: "dropdown";
  value?: T;
  onValueChange?: (value: T) => void;
  items: ToggleItem<T>[];
  tooltip?: string;
  hotkey?: string;
  label?: string;
  placeholder?: string;
  dropdownTooltip?: string;
}

interface ToggleWithActionProps extends React.ComponentProps<typeof TogglePrimitive.Root> {
  type: "withAction";
  actionIcon: React.ReactNode;
  onActionClick: () => void;
  tooltip?: string;
  hotkey?: string;
  label?: string;
  actionTooltip?: string;
}

type ToggleProps<T extends string = string> = ToggleStandardProps | ToggleCycleProps<T> | ToggleDropdownProps<T> | ToggleWithActionProps;

function Toggle<T extends string = string>(props: ToggleProps<T>) {
  // Cycle type
  if ("type" in props && props.type === "cycle") {
    const { className, value, onValueChange, items, tooltip, hotkey, label, pressed, onPressedChange, ...restProps } = props;

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
        className={cn(toggleVariants(), className)}
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
          <span className="text-sm font-medium flex-shrink-0 min-w-[80px] text-left truncate" title={label}>
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
    const { className, actionIcon, onActionClick, tooltip, hotkey, label, actionTooltip, ...restProps } = props;

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

    const actionButton = actionTooltip ? (
      <Tooltip>
        <TooltipTrigger asChild>
          <button
            type="button"
            className="shrink-0 p-0.5 hover:bg-muted z-10"
            onClick={(e) => {
              e.stopPropagation();
              onActionClick();
            }}
          >
            {actionIcon}
          </button>
        </TooltipTrigger>
        <TooltipContent>{actionTooltip}</TooltipContent>
      </Tooltip>
    ) : (
      <button
        type="button"
        className="shrink-0 p-0.5 hover:bg-muted z-10"
        onClick={(e) => {
          e.stopPropagation();
          onActionClick();
        }}
      >
        {actionIcon}
      </button>
    );

    const toggleElement = (
      <TogglePrimitive.Root data-slot="toggle" className={cn(toggleVariants(), "gap-1 pr-1 [&:has(button:hover)]:bg-transparent [&:has(button:hover)]:text-foreground", className)} {...restProps}>
        {mainContent}
        {actionButton}
      </TogglePrimitive.Root>
    );

    const wrappedToggle = toggleElement;

    if (label) {
      return (
        <div className="flex items-center gap-2 min-w-0">
          <span className="text-sm font-medium flex-shrink-0 min-w-[80px] text-left truncate" title={label}>
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
    const { className, value, onValueChange, items, tooltip, hotkey, label, placeholder = "Select...", dropdownTooltip, pressed, onPressedChange, ...restProps } = props;

    if (!items || items.length === 0) return null;

    const [open, setOpen] = React.useState(false);
    const currentItem = items.find((item) => item.value === value);

    const handleSelect = (selectedValue: T) => {
      onValueChange?.(selectedValue);
      setOpen(false);
    };

    const mainContent = tooltip ? (
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="truncate flex-1 min-w-0">{currentItem?.label || placeholder}</span>
        </TooltipTrigger>
        <TooltipContent>
          {tooltip}
          {hotkey && <span className="text-xs ml-1 opacity-60">({hotkey})</span>}
        </TooltipContent>
      </Tooltip>
    ) : (
      <span className="truncate flex-1 min-w-0">{currentItem?.label || placeholder}</span>
    );

    const dropdownButton = dropdownTooltip ? (
      <Tooltip>
        <TooltipTrigger asChild>
          <button
            type="button"
            className="shrink-0 p-0.5 hover:bg-muted z-10"
            onClick={(e) => {
              e.stopPropagation();
              setOpen(!open);
            }}
          >
            <svg className="size-3.5 opacity-50" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={2} stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
            </svg>
          </button>
        </TooltipTrigger>
        <TooltipContent>{dropdownTooltip}</TooltipContent>
      </Tooltip>
    ) : (
      <button
        type="button"
        className="shrink-0 p-0.5 hover:bg-muted z-10"
        onClick={(e) => {
          e.stopPropagation();
          setOpen(!open);
        }}
      >
        <svg className="size-3.5 opacity-50" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={2} stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
        </svg>
      </button>
    );

    const toggleElement = (
      <TogglePrimitive.Root data-slot="toggle" className={cn(toggleVariants(), "gap-1 pr-1 [&:has(button:hover)]:bg-transparent [&:has(button:hover)]:text-foreground", className)} pressed={pressed} onPressedChange={onPressedChange} {...restProps}>
        {mainContent}
        {dropdownButton}
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
              const itemButton = (
                <button
                  key={item.value}
                  type="button"
                  onClick={() => handleSelect(item.value)}
                  className={cn(
                    "flex items-center justify-center text-sm hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground outline-none transition-colors",
                    toggleVariants(),
                    "border-0 min-w-0 w-auto",
                    isSelected && "bg-primary/10 font-medium",
                  )}
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
          <span className="text-sm font-medium flex-shrink-0 min-w-[80px] text-left truncate" title={label}>
            {label}
          </span>
          {dropdownElement}
        </div>
      );
    }

    return dropdownElement;
  }

  // Standard toggle
  const { className, tooltip, hotkey, label, ...restProps } = props as ToggleStandardProps;

  const toggleElement = <TogglePrimitive.Root data-slot="toggle" className={cn(toggleVariants(), className)} {...restProps} />;

  const wrappedToggle = tooltip ? (
    <Tooltip>
      <TooltipTrigger asChild>{toggleElement}</TooltipTrigger>
      <TooltipContent>
        {tooltip}
        {hotkey && <span className="text-xs ml-1 opacity-60">({hotkey})</span>}
      </TooltipContent>
    </Tooltip>
  ) : (
    toggleElement
  );

  if (label) {
    return (
      <div className="flex items-center gap-2 min-w-0">
        <span className="text-sm font-medium flex-shrink-0 min-w-[80px] text-left truncate" title={label}>
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
