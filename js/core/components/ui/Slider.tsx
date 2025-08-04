// #region Header

// Slider.tsx

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
import * as SliderPrimitive from "@radix-ui/react-slider";
import * as React from "react";
import { Input } from "./Input";

import { cn } from "@semio/js/lib/utils";

function Slider({
  className,
  defaultValue,
  value,
  min = 0,
  max = 100,
  label,
  onValueChange,
  onPointerDown,
  onPointerUp,
  onPointerCancel,
  ...props
}: React.ComponentProps<typeof SliderPrimitive.Root> & {
  label?: string;
  onPointerDown?: () => void;
  onPointerUp?: () => void;
  onPointerCancel?: () => void;
}) {
  const [isEditing, setIsEditing] = React.useState(false);
  const [editValue, setEditValue] = React.useState("");

  const _values = React.useMemo(() => (Array.isArray(value) ? value : Array.isArray(defaultValue) ? defaultValue : [min, max]), [value, defaultValue, min, max]);

  const displayValue = _values[0] ?? min;

  const handleValueClick = () => {
    setEditValue(displayValue.toString());
    setIsEditing(true);
  };

  const handleEditKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      const newValue = parseFloat(editValue);
      if (!isNaN(newValue) && newValue >= min && newValue <= max) {
        onValueChange?.([newValue]);
      }
      setIsEditing(false);
    } else if (e.key === "Escape") {
      setIsEditing(false);
    }
  };

  const handleEditBlur = () => {
    setIsEditing(false);
  };

  return (
    <div className={cn("flex items-center gap-2 border-b border-border pb-1 min-w-0", className)}>
      {label && (
        <span className="text-sm font-medium flex-shrink-0 min-w-[80px] text-left truncate" title={label}>
          {label}
        </span>
      )}
      <div className="flex items-center gap-2 flex-1 min-w-0">
        <SliderPrimitive.Root
          data-slot="slider"
          defaultValue={defaultValue}
          value={value}
          min={min}
          max={max}
          onValueChange={onValueChange}
          onPointerDown={onPointerDown}
          onPointerUp={onPointerUp}
          onPointerCancel={onPointerCancel}
          className={cn(
            "relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50 data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-44 data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col",
          )}
          {...props}
        >
          <SliderPrimitive.Track
            data-slot="slider-track"
            className={cn("bg-muted relative grow overflow-hidden rounded-full data-[orientation=horizontal]:h-1 data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-1")}
          >
            <SliderPrimitive.Range data-slot="slider-range" className={cn("bg-foreground absolute data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full")} />
          </SliderPrimitive.Track>
          {Array.from({ length: _values.length }, (_, index) => (
            <SliderPrimitive.Thumb
              data-slot="slider-thumb"
              key={index}
              className="border-foreground bg-foreground ring-ring/50 block size-4 shrink-0 rounded-full border transition-colors hover:bg-accent focus-visible:bg-primary focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50 active:bg-primary"
            />
          ))}
        </SliderPrimitive.Root>
        {isEditing ? (
          <Input type="number" value={editValue} onChange={(e) => setEditValue(e.target.value)} onKeyDown={handleEditKeyDown} onBlur={handleEditBlur} className="w-20 text-sm" min={min} max={max} autoFocus />
        ) : (
          <span className="text-sm w-20 text-right cursor-pointer hover:bg-muted px-1 rounded select-none" onDoubleClick={handleValueClick} title="Double-click to edit">
            {displayValue}
          </span>
        )}
      </div>
    </div>
  );
}

export { Slider };
