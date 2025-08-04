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

import { toggleVariants } from "@semio/js/components/ui/Toggle";
import { Tooltip, TooltipContent, TooltipTrigger } from "@semio/js/components/ui/Tooltip";
import { cn } from "@semio/js/lib/utils";

const ToggleGroupContext = React.createContext<VariantProps<typeof toggleVariants>>({
  size: "default",
  variant: "default",
});

function ToggleGroup({ className, variant, size, children, ...props }: React.ComponentProps<typeof ToggleGroupPrimitive.Root> & VariantProps<typeof toggleVariants>) {
  return (
    <ToggleGroupPrimitive.Root data-slot="toggle-group" data-variant={variant} data-size={size} className={cn("group/toggle-group flex w-fit items-center border overflow-hidden", className)} {...props}>
      <ToggleGroupContext.Provider value={{ variant, size }}>{children}</ToggleGroupContext.Provider>
    </ToggleGroupPrimitive.Root>
  );
}

function ToggleGroupItem({
  className,
  children,
  variant,
  size,
  tooltip,
  hotkey,
  ...props
}: React.ComponentProps<typeof ToggleGroupPrimitive.Item> &
  VariantProps<typeof toggleVariants> & {
    tooltip?: string;
    hotkey?: string;
  }) {
  const context = React.useContext(ToggleGroupContext);

  const toggleGroupItemElement = (
    <ToggleGroupPrimitive.Item
      data-slot="toggle-group-item"
      data-variant={context.variant || variant}
      data-size={context.size || size}
      className={cn(
        toggleVariants({
          variant: context.variant || variant,
          size: context.size || size,
        }),
        "min-w-0 flex-1 shrink-0 focus:z-10 focus-visible:z-10 data-[state=on]:bg-primary data-[variant=outline]:border-l-0 data-[variant=outline]:first:border-l",
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
