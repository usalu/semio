// #region Header

// Avatar.tsx

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
import * as AvatarPrimitive from "@radix-ui/react-avatar";
import * as React from "react";

import { cn } from "../../semio";

const Avatar = React.forwardRef<React.ElementRef<typeof AvatarPrimitive.Root>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Root>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Root ref={ref} data-slot="avatar" className={cn("relative flex size-8 shrink-0 overflow-hidden rounded-full border", className)} {...props} />
));
Avatar.displayName = "Avatar";

const AvatarImage = React.forwardRef<React.ElementRef<typeof AvatarPrimitive.Image>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Image>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Image ref={ref} data-slot="avatar-image" className={cn("aspect-square size-full", className)} {...props} />
));
AvatarImage.displayName = "AvatarImage";

const AvatarFallback = React.forwardRef<React.ElementRef<typeof AvatarPrimitive.Fallback>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Fallback>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Fallback ref={ref} data-slot="avatar-fallback" className={cn("bg-muted flex size-full items-center justify-center rounded-full", className)} {...props} />
));
AvatarFallback.displayName = "AvatarFallback";

export interface DraggableAvatarProps {
  /** Content to display (usually 1-2 characters) */
  content: string;
  /** Whether the avatar is selected */
  isSelected?: boolean;
  /** Whether the avatar is hovered */
  isHovered?: boolean;
  /** Whether the avatar should fade (when another item is being dragged) */
  shouldFade?: boolean;
  /** Optional title/tooltip */
  title?: string;
  /** Ref for drag-and-drop */
  dragRef?: React.Ref<HTMLDivElement>;
  /** Drag listeners */
  dragListeners?: any;
  /** Drag attributes */
  dragAttributes?: any;
  /** Click handlers */
  onClick?: () => void;
  onDoubleClick?: () => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  /** Additional class names */
  className?: string;
}

/**
 * Generalized draggable avatar component
 * Used for types, designs, qualities, functions, etc.
 */
export const DraggableAvatar = React.forwardRef<HTMLDivElement, DraggableAvatarProps>(
  ({ content, isSelected, isHovered, shouldFade, title, dragRef, dragListeners, dragAttributes, onClick, onDoubleClick, onPointerEnter, onPointerLeave, className }, ref) => {
    return (
      <div
        ref={dragRef || ref}
        {...dragListeners}
        {...dragAttributes}
        onClick={onClick}
        onDoubleClick={onDoubleClick}
        onPointerEnter={onPointerEnter}
        onPointerLeave={onPointerLeave}
        title={title}
        className={className}
      >
        <Avatar
          className={cn(
            "cursor-grab active:cursor-grabbing select-none border-[color:var(--border-color)]",
            isSelected && "ring-1 ring-inset ring-[color:var(--active-base)]",
            isHovered && !isSelected && "ring-1 ring-inset ring-[color:var(--hover-base)]"
          )}
          style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}
        >
          <AvatarFallback
            className={cn(
              "select-none",
              isSelected && "bg-[var(--active-base)] text-[var(--active-foreground)]",
              isHovered && !isSelected && "bg-[var(--hover-base)] text-foreground",
              !isSelected && !isHovered && "bg-muted"
            )}
          >
            {content}
          </AvatarFallback>
        </Avatar>
      </div>
    );
  }
);
DraggableAvatar.displayName = "DraggableAvatar";

export { Avatar, AvatarFallback, AvatarImage };
