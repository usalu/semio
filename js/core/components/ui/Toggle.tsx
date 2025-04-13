"use client"

import * as React from "react"
import * as TogglePrimitive from "@radix-ui/react-toggle"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@semio/js/lib/utils"
import { Tooltip, TooltipContent, TooltipTrigger } from "./Tooltip"

const toggleVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-2 text-sm font-medium hover:bg-muted hover:text-muted-foreground disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap",
  {
    variants: {
      variant: {
        default: "bg-transparent hover:bg-accent hover:text-accent-foreground",
        outline:
          "border bg-transparent hover:bg-accent hover:text-accent-foreground",
      },
      size: {
        default: "h-9 px-2 min-w-9",
        sm: "h-8 px-1.5 min-w-8",
        lg: "h-10 px-2.5 min-w-10",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
)

interface ToggleProps extends React.ComponentProps<typeof TogglePrimitive.Root>,
  VariantProps<typeof toggleVariants> {
  tooltip?: string;
  hotkey?: string;
}

function Toggle({
  className,
  variant,
  size,
  tooltip,
  hotkey,
  ...props
}: ToggleProps) {
  const toggleElement = (
    <TogglePrimitive.Root
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
}

export { Toggle, toggleVariants }
export type { ToggleProps }
