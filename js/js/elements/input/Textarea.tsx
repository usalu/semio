// #region Header

// Textarea.tsx

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
import * as React from "react";

import { cn } from "../../semio";

interface TextareaProps extends Omit<React.ComponentProps<"textarea">, "value" | "onChange"> {
  label?: string;
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onLazyChange?: (value: string) => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
  abortTransaction?: () => void;
}

function Textarea({ className, label, lazy, value: externalValue, onChange, onLazyChange, startTransaction, finalizeTransaction, abortTransaction, ...props }: TextareaProps) {
  const [localValue, setLocalValue] = React.useState(externalValue?.toString() || "");
  const [isEditing, setIsEditing] = React.useState(false);

  React.useEffect(() => {
    if (!isEditing) setLocalValue(externalValue?.toString() || "");
  }, [externalValue, isEditing]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      setLocalValue(e.target.value);
    } else if (onChange) {
      onChange(e);
    }
  };

  const handleFocus = (e: React.FocusEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      setIsEditing(true);
      startTransaction?.();
    }
    props.onFocus?.(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      setIsEditing(false);
      onLazyChange?.(localValue);
      finalizeTransaction?.();
    }
    props.onBlur?.(e);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      if (e.key === "Escape") {
        setIsEditing(false);
        setLocalValue(externalValue?.toString() || "");
        abortTransaction?.();
        (e.target as HTMLTextAreaElement).blur();
      }
    }
    props.onKeyDown?.(e);
  };

  const textareaValue = lazy ? localValue : externalValue;

  if (label) {
    return (
      <div className="flex items-start gap-2 min-w-0">
        <span className="text-xs font-medium flex-shrink-0 pt-2 min-w-[80px] text-left truncate" title={label}>
          {label}
        </span>
        <textarea
          data-slot="textarea"
          className={cn(
            "placeholder:text-muted-foreground text-foreground flex field-sizing-content min-h-16 w-full border bg-transparent px-3 py-2 text-base transition-[color,border-color] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
            "focus-visible:border-primary",
            "aria-invalid:border-destructive flex-1",
            className,
          )}
          value={textareaValue}
          onChange={handleChange}
          onFocus={handleFocus}
          onBlur={handleBlur}
          onKeyDown={handleKeyDown}
          {...props}
        />
      </div>
    );
  }

  return (
    <textarea
      data-slot="textarea"
      className={cn(
        "placeholder:text-muted-foreground text-foreground flex field-sizing-content min-h-16 w-full border bg-transparent px-3 py-2 text-base transition-[color,border-color] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "focus-visible:border-primary",
        "aria-invalid:border-destructive",
        className,
      )}
      value={textareaValue}
      onChange={handleChange}
      onFocus={handleFocus}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
      {...props}
    />
  );
}

export { Textarea };
