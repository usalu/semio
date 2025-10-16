// #region Header

// Breadcrumb.tsx

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
import * as DropdownMenuPrimitive from "@radix-ui/react-dropdown-menu";
import { Slot } from "@radix-ui/react-slot";
import { ChevronDown, ChevronRight, MoreHorizontal } from "lucide-react";
import * as React from "react";

import { cn } from "../../semio";
import { Tooltip, TooltipContent, TooltipTrigger } from "../display/Tooltip";

function Breadcrumb({ ...props }: React.ComponentProps<"nav">) {
  return <nav aria-label="breadcrumb" data-slot="breadcrumb" {...props} />;
}

function BreadcrumbList({ className, ...props }: React.ComponentProps<"ol">) {
  return <ol data-slot="breadcrumb-list" className={cn("flex flex-wrap items-stretch text-xs break-words border overflow-hidden h-auto min-h-9", className)} {...props} />;
}

function BreadcrumbItem({ className, tooltip, children, ...props }: React.ComponentProps<"li"> & { tooltip?: string }) {
  const itemElement = (
    <li data-slot="breadcrumb-item" className={cn("flex items-stretch", className)} {...props}>
      {children}
    </li>
  );

  if (tooltip) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{itemElement}</TooltipTrigger>
        <TooltipContent>{tooltip}</TooltipContent>
      </Tooltip>
    );
  }

  return itemElement;
}

function BreadcrumbLink({
  asChild,
  className,
  level = "base",
  ...props
}: React.ComponentProps<"a"> & {
  asChild?: boolean;
  level?: "base" | "panel" | "temporary";
}) {
  const Comp = asChild ? Slot : "a";
  const hoverClass = level === "panel" ? "hover:bg-hover-panel" : level === "temporary" ? "hover:bg-hover-temporary" : "hover:bg-hover-base";

  return <Comp data-slot="breadcrumb-link" className={cn("text-foreground transition-colors px-1 flex items-center gap-1 h-full", hoverClass, className)} {...props} />;
}

function BreadcrumbPage({ className, ...props }: React.ComponentProps<"span">) {
  return <span data-slot="breadcrumb-page" role="link" aria-disabled="true" aria-current="page" className={cn("text-foreground font-normal", className)} {...props} />;
}

interface BreadcrumbSeparatorProps extends React.ComponentProps<"li"> {
  items?: { label: React.ReactNode; href: string; tooltip?: string }[];
  onNavigate?: (href: string) => void;
  tooltip?: string;
  level?: "base" | "panel" | "temporary";
}

function BreadcrumbSeparator({ children, className, items, onNavigate, tooltip, level = "base", ...props }: BreadcrumbSeparatorProps) {
  const [open, setOpen] = React.useState(false);
  const hoverClass = level === "panel" ? "hover:bg-hover-panel" : level === "temporary" ? "hover:bg-hover-temporary" : "hover:bg-hover-base";

  const handleSelect = (href: string) => {
    setOpen(false);
    onNavigate?.(href);
  };

  if (!items?.length) {
    const separatorElement = (
      <li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" className={cn("[&>svg]:size-3 px-1 flex items-center self-stretch", className)} {...props}>
        {children ?? <ChevronRight />}
      </li>
    );

    if (tooltip) {
      return (
        <Tooltip>
          <TooltipTrigger asChild>{separatorElement}</TooltipTrigger>
          <TooltipContent>{tooltip}</TooltipContent>
        </Tooltip>
      );
    }

    return separatorElement;
  }

  return (
    <DropdownMenuPrimitive.Root open={open} onOpenChange={setOpen}>
      {tooltip ? (
        <Tooltip>
          <TooltipTrigger asChild>
            <DropdownMenuPrimitive.Trigger asChild>
              <li data-slot="breadcrumb-separator" className={cn("[&>svg]:size-3 px-1 flex items-center transition-colors self-stretch", hoverClass, className)} {...props} role="button">
                {open ? <ChevronDown /> : <ChevronRight />}
              </li>
            </DropdownMenuPrimitive.Trigger>
          </TooltipTrigger>
          <TooltipContent>{tooltip}</TooltipContent>
        </Tooltip>
      ) : (
        <DropdownMenuPrimitive.Trigger asChild>
          <li data-slot="breadcrumb-separator" className={cn("[&>svg]:size-3 px-1 flex items-center transition-colors self-stretch", hoverClass, className)} {...props} role="button">
            {open ? <ChevronDown /> : <ChevronRight />}
          </li>
        </DropdownMenuPrimitive.Trigger>
      )}
      <DropdownMenuPrimitive.Portal>
        <DropdownMenuPrimitive.Content align="start" sideOffset={8} className="bg-temporary w-auto overflow-hidden border p-1">
          {items.map((item, index) => {
            const menuItem = (
              <DropdownMenuPrimitive.Item
                key={index}
                className="text-foreground hover:bg-hover-temporary focus:bg-hover-temporary relative flex items-center px-1 py-1 text-sm outline-none whitespace-nowrap"
                onClick={() => handleSelect(item.href)}
                role="button"
              >
                {item.label}
              </DropdownMenuPrimitive.Item>
            );

            const wrappedItem = item.tooltip ? (
              <Tooltip key={index}>
                <TooltipTrigger asChild>{menuItem}</TooltipTrigger>
                <TooltipContent>{item.tooltip}</TooltipContent>
              </Tooltip>
            ) : (
              menuItem
            );

            // Add separator after each item except the last one
            if (index < items.length - 1) {
              return (
                <React.Fragment key={index}>
                  {wrappedItem}
                  <DropdownMenuPrimitive.Separator className="h-px bg-border my-1" />
                </React.Fragment>
              );
            }

            return wrappedItem;
          })}
        </DropdownMenuPrimitive.Content>
      </DropdownMenuPrimitive.Portal>
    </DropdownMenuPrimitive.Root>
  );
}

function BreadcrumbEllipsis({ className, ...props }: React.ComponentProps<"span">) {
  return (
    <span data-slot="breadcrumb-ellipsis" role="presentation" aria-hidden="true" className={cn("flex size-9 items-center justify-center", className)} {...props}>
      <MoreHorizontal className="size-4" />
      <span className="sr-only">More</span>
    </span>
  );
}

function BreadcrumbBreak({ className, ...props }: React.ComponentProps<"li">) {
  return <li data-slot="breadcrumb-break" role="presentation" aria-hidden="true" className={cn("basis-full h-0 hidden", className)} {...props} />;
}

export { Breadcrumb, BreadcrumbBreak, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator };
