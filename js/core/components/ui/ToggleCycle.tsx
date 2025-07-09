// #region Header

// ToggleCycle.tsx

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
"use client"

import { type VariantProps } from "class-variance-authority"
import * as React from "react"

import { toggleVariants } from "@semio/js/components/ui/Toggle"
import { Tooltip, TooltipContent, TooltipTrigger } from "@semio/js/components/ui/Tooltip"
import { cn } from "@semio/js/lib/utils"

export interface ToggleCycleItem<T extends string> {
    value: T;
    label: React.ReactNode;
    tooltip?: string;
    hotkey?: string;
}

interface ToggleCycleProps<T extends string>
    extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "value" | "onValueChange">,
    VariantProps<typeof toggleVariants> {
    value?: T;
    onValueChange?: (value: T) => void;
    items: ToggleCycleItem<T>[];
    tooltip?: string;
    hotkey?: string;
}

export function ToggleCycle<T extends string>({
    className,
    variant = "outline",
    size = "default",
    value,
    onValueChange,
    items,
    tooltip,
    hotkey,
    ...props
}: ToggleCycleProps<T>) {
    if (!items || items.length === 0) return null;

    // Find the current item or default to first
    const currentIndex = Math.max(
        0,
        items.findIndex((item) => item.value === value)
    );
    const currentItem = items[currentIndex];

    // Handle click to cycle to next item
    const handleClick = () => {
        const nextIndex = (currentIndex + 1) % items.length;
        const nextValue = items[nextIndex].value;
        onValueChange?.(nextValue);
    };

    const buttonElement = (
        <button
            type="button"
            data-state={value ? "on" : "off"}
            className={cn(toggleVariants({ variant, size }), className)}
            onClick={handleClick}
            {...props}
        >
            {currentItem.label}
        </button>
    );

    // Use either the specific item's tooltip or the general tooltip
    const activeTooltip = currentItem.tooltip || tooltip;
    const activeHotkey = currentItem.hotkey || hotkey;

    if (activeTooltip) {
        return (
            <Tooltip>
                <TooltipTrigger asChild>
                    {buttonElement}
                </TooltipTrigger>
                <TooltipContent>
                    {activeTooltip}
                    {activeHotkey && <span className="text-xs ml-1 opacity-60">({activeHotkey})</span>}
                </TooltipContent>
            </Tooltip>
        );
    }

    return buttonElement;
}