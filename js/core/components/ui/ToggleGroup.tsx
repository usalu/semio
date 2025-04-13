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

// Define separate types for single and multiple toggle groups to match Radix UI's typing
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

// Use a type union with the common props from Radix and our variants
type ToggleGroupProps =
  Omit<React.ComponentPropsWithoutRef<typeof ToggleGroupPrimitive.Root>, "type" | "value" | "defaultValue" | "onValueChange"> &
  VariantProps<typeof toggleGroupVariants> &
  (ToggleGroupSingleProps | ToggleGroupMultipleProps) & {
    className?: string;
  };

const ToggleGroup = React.forwardRef<
  React.ElementRef<typeof ToggleGroupPrimitive.Root>,
  ToggleGroupProps
>(({ className, variant, size, ...props }, ref) => (
  <ToggleGroupPrimitive.Root
    ref={ref}
    data-slot="toggle-group"
    className={cn(toggleGroupVariants({ variant, size, className }))}
    // Type is now properly passed through
    {...props}
  />
))

ToggleGroup.displayName = ToggleGroupPrimitive.Root.displayName

interface ToggleGroupItemProps extends React.ComponentProps<typeof ToggleGroupPrimitive.Item>,
  VariantProps<typeof toggleVariants> {
  tooltip?: React.ReactNode;
}

const ToggleGroupItem = React.forwardRef<
  React.ElementRef<typeof ToggleGroupPrimitive.Item>,
  ToggleGroupItemProps
>(({ className, variant, size, tooltip, ...props }, ref) => {
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
        </TooltipContent>
      </Tooltip>
    );
  }

  return toggleElement;
})

ToggleGroupItem.displayName = ToggleGroupPrimitive.Item.displayName

export { ToggleGroup, ToggleGroupItem, toggleGroupVariants }
export type { ToggleGroupProps, ToggleGroupItemProps, ToggleGroupSingleProps, ToggleGroupMultipleProps }
