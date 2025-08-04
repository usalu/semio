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

import { cn } from "@semio/js/lib/utils";

function Breadcrumb({ ...props }: React.ComponentProps<"nav">) {
  return <nav aria-label="breadcrumb" data-slot="breadcrumb" {...props} />;
}

function BreadcrumbList({ className, ...props }: React.ComponentProps<"ol">) {
  return <ol data-slot="breadcrumb-list" className={cn("text-muted-foreground flex flex-wrap items-center gap-1.5 text-xs break-words xs:gap-1.5", className)} {...props} />;
}

function BreadcrumbItem({ className, ...props }: React.ComponentProps<"li">) {
  return <li data-slot="breadcrumb-item" className={cn("inline-flex items-center gap-1.5", className)} {...props} />;
}

function BreadcrumbLink({
  asChild,
  className,
  ...props
}: React.ComponentProps<"a"> & {
  asChild?: boolean;
}) {
  const Comp = asChild ? Slot : "a";

  return <Comp data-slot="breadcrumb-link" className={cn("text-foreground hover:text-accent-foreground transition-colors", className)} {...props} />;
}

function BreadcrumbPage({ className, ...props }: React.ComponentProps<"span">) {
  return <span data-slot="breadcrumb-page" role="link" aria-disabled="true" aria-current="page" className={cn("text-foreground font-normal", className)} {...props} />;
}

interface BreadcrumbSeparatorProps extends React.ComponentProps<"li"> {
  items?: { label: string; href: string }[];
  onNavigate?: (href: string) => void;
}

function BreadcrumbSeparator({ children, className, items, onNavigate, ...props }: BreadcrumbSeparatorProps) {
  const [open, setOpen] = React.useState(false);

  const handleSelect = (href: string) => {
    setOpen(false);
    onNavigate?.(href);
  };

  if (!items?.length) {
    return (
      <li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" className={cn("[&>svg]:size-3", className)} {...props}>
        {children ?? <ChevronRight />}
      </li>
    );
  }

  return (
    <DropdownMenuPrimitive.Root open={open} onOpenChange={setOpen}>
      <DropdownMenuPrimitive.Trigger asChild>
        <li data-slot="breadcrumb-separator" className={cn("[&>svg]:size-3 cursor-pointer", className)} {...props}>
          {open ? <ChevronDown /> : <ChevronRight />}
        </li>
      </DropdownMenuPrimitive.Trigger>
      <DropdownMenuPrimitive.Portal>
        <DropdownMenuPrimitive.Content align="center" className="bg-background-level-3 min-w-[8rem] overflow-hidden border p-1">
          {items.map((item, index) => (
            <DropdownMenuPrimitive.Item key={index} className="text-foreground focus:bg-accent focus:text-accent-foreground relative flex cursor-pointer items-center px-2 py-1.5 text-sm outline-none" onClick={() => handleSelect(item.href)}>
              {item.label}
            </DropdownMenuPrimitive.Item>
          ))}
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

export { Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator };
