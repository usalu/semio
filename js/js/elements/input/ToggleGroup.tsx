// #region Header

// ToggleGroup.tsx

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
import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";
import { type VariantProps } from "class-variance-authority";
import * as React from "react";

import { cn } from "../../semio";
import { Tooltip, TooltipContent, TooltipTrigger } from "../display/Tooltip";
import { toggleVariants } from "./Toggle";

const ToggleGroupContext = React.createContext<VariantProps<typeof toggleVariants>>({
  variant: "default",
  level: "base",
});

interface ToggleGroupProps extends Omit<React.ComponentProps<typeof ToggleGroupPrimitive.Root>, "children"> {
  label?: string;
  children: React.ReactNode;
  level?: "base" | "panel" | "temporary";
}

function ToggleGroup({ className, label, level = "base", children, ...restProps }: ToggleGroupProps) {
  const variant = "default";
  const toggleGroupElement = (
    <ToggleGroupPrimitive.Root data-slot="toggle-group" data-variant={variant} className={cn("group/toggle-group flex w-fit items-stretch border overflow-hidden h-9", className)} {...restProps}>
      <ToggleGroupContext.Provider value={{ variant, level }}>{children}</ToggleGroupContext.Provider>
    </ToggleGroupPrimitive.Root>
  );

  if (label) {
    return (
      <div className="flex items-center gap-2 min-w-0">
        <span className="text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate">{label}</span>
        {toggleGroupElement}
      </div>
    );
  }

  return toggleGroupElement;
}

function ToggleGroupItem({
  className,
  children,
  tooltip,
  hotkey,
  ...props
}: React.ComponentProps<typeof ToggleGroupPrimitive.Item> & {
  tooltip?: string;
  hotkey?: string;
}) {
  const context = React.useContext(ToggleGroupContext);

  const toggleGroupItemElement = (
    <ToggleGroupPrimitive.Item
      data-slot="toggle-group-item"
      data-variant={context.variant}
      className={cn(
        toggleVariants({
          variant: context.variant,
          level: context.level,
        }),
        "min-w-0 flex-1 shrink-0 focus:z-10 focus-visible:z-10 data-[state=on]:bg-active-base data-[state=on]:hover:bg-active-base/90 border-0 border-l first:border-l-0 !h-full",
        className,
      )}
      {...props}
    >
      {children}
    </ToggleGroupPrimitive.Item>
  );

  if (tooltip) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>
          {/* Wrapping in span to avoid styling issue with data-[state=on]: https://github.com/radix-ui/primitives/discussions/560 */}
          <span>{toggleGroupItemElement}</span>
        </TooltipTrigger>
        <TooltipContent>
          {tooltip}
          {hotkey && <span className="text-xs ml-1 opacity-60">({hotkey})</span>}
        </TooltipContent>
      </Tooltip>
    );
  }

  return toggleGroupItemElement;
}

export { ToggleGroup, ToggleGroupItem };
