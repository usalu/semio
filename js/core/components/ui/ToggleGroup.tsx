"use client"

import * as React from "react"
import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@semio/js/lib/utils"
import { toggleVariants } from "@semio/js/components/ui/Toggle"
import { Tooltip, TooltipContent, TooltipTrigger } from "@semio/js/components/ui/Tooltip"

const toggleGroupVariants = cva(
  "inline-flex",
  {
    variants: {
      variant: {
        default: "",
        outline: "border bg-transparent",
      },
      size: {
        default: "",
        sm: "",
        lg: "",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
)

// Define separate types for single, multiple, and cycle toggle groups to match Radix UI's typing
type ToggleGroupSingleProps = {
  type: "single";
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
};

type ToggleGroupMultipleProps = {
  type: "multiple";
  value?: string[];
  defaultValue?: string[];
  onValueChange?: (value: string[]) => void;
};

type ToggleGroupCycleProps = {
  type: "cycle";
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  items: { value: string; label: React.ReactNode }[];
};

// Use a type union with the common props from Radix and our variants
type ToggleGroupProps =
  Omit<React.ComponentPropsWithoutRef<typeof ToggleGroupPrimitive.Root>, "type" | "value" | "defaultValue" | "onValueChange"> &
  VariantProps<typeof toggleGroupVariants> &
  (ToggleGroupSingleProps | ToggleGroupMultipleProps | ToggleGroupCycleProps) & {
    className?: string;
  };

const ToggleGroup = React.forwardRef<
  React.ElementRef<typeof ToggleGroupPrimitive.Root>,
  ToggleGroupProps
>(({ className, variant, size, type, value, defaultValue, onValueChange, ...props }, ref) => {
  if (type === "cycle") {
    const cycleProps = props as ToggleGroupCycleProps;
    const currentIndex = cycleProps.items.findIndex(item => item.value === value);
    const nextValue = cycleProps.items[(currentIndex + 1) % cycleProps.items.length].value;

    return (
      <ToggleGroupPrimitive.Root
        ref={ref}
        data-slot="toggle-group"
        className={cn(toggleGroupVariants({ variant, size, className }))}
        type="single"
        value={value}
        defaultValue={defaultValue}
        onValueChange={(newValue) => {
          if (newValue) {
            onValueChange?.(nextValue);
          }
        }}
      >
        <ToggleGroupItem value={value || defaultValue || cycleProps.items[0].value}>
          {cycleProps.items.find(item => item.value === value)?.label || cycleProps.items[0].label}
        </ToggleGroupItem>
      </ToggleGroupPrimitive.Root>
    );
  }

  return (
    <ToggleGroupPrimitive.Root
      ref={ref}
      data-slot="toggle-group"
      className={cn(toggleGroupVariants({ variant, size, className }))}
      type={type}
      value={value}
      defaultValue={defaultValue}
      onValueChange={onValueChange}
      {...props}
    />
  );
});

ToggleGroup.displayName = ToggleGroupPrimitive.Root.displayName

interface ToggleGroupItemProps extends React.ComponentProps<typeof ToggleGroupPrimitive.Item>,
  VariantProps<typeof toggleVariants> {
  tooltip?: React.ReactNode;
  hotkey?: string;
}

const ToggleGroupItem = React.forwardRef<
  React.ElementRef<typeof ToggleGroupPrimitive.Item>,
  ToggleGroupItemProps
>(({ className, variant, size, tooltip, hotkey, ...props }, ref) => {
  const toggleElement = (
    <ToggleGroupPrimitive.Item
      ref={ref}
      data-slot="toggle"
      className={cn(toggleVariants({ variant, size, className }))}
      {...props}
    />
  );

  if (tooltip) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>
          {toggleElement}
        </TooltipTrigger>
        <TooltipContent>
          {tooltip}
          {hotkey && <span className="text-xs ml-1 opacity-60">({hotkey})</span>}
        </TooltipContent>
      </Tooltip>
    );
  }

  return toggleElement;
})

ToggleGroupItem.displayName = ToggleGroupPrimitive.Item.displayName

export { ToggleGroup, ToggleGroupItem, toggleGroupVariants }
export type { ToggleGroupProps, ToggleGroupItemProps, ToggleGroupSingleProps, ToggleGroupMultipleProps, ToggleGroupCycleProps }
