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

import { useTranslation } from "react-i18next";
import { cn } from "../../semio";
import { I18nTooltipContent, Tooltip, TooltipContent, TooltipTrigger, useTooltipMode } from "../display/Tooltip";

interface TextareaProps extends Omit<React.ComponentProps<"textarea">, "value" | "onChange"> {
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onLazyChange?: (value: string) => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
  abortTransaction?: () => void;
  i18n: string;
  placeholderI18n?: string;
}

function Textarea({ className, lazy, value: externalValue, onChange, onLazyChange, startTransaction, finalizeTransaction, abortTransaction, i18n, placeholderI18n, placeholder, ...props }: TextareaProps) {
  const [localValue, setLocalValue] = React.useState(externalValue?.toString() || "");
  const [isEditing, setIsEditing] = React.useState(false);
  const mode = useTooltipMode();
  const { t } = useTranslation();
  const computedPlaceholder = placeholderI18n ? t(placeholderI18n) : placeholder;

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

  const textareaElement = (
    <textarea
      data-slot="textarea"
      className={cn(
        "placeholder:text-muted-foreground text-foreground flex field-sizing-content min-h-16 w-full border bg-transparent px-3 py-2 text-base transition-[color,border-color] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "focus-visible:border-accent",
        "aria-invalid:border-destructive flex-1",
        className,
      )}
      value={textareaValue}
      onChange={handleChange}
      onFocus={handleFocus}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
      placeholder={computedPlaceholder}
      {...props}
    />
  );

  const label = t(`${i18n}.label`);

  return (
    <div className="group flex items-start gap-2 min-w-0 w-full">
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="inline-flex items-start h-9 px-3 py-2 text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate cursor-pointer transition-colors group-hover:bg-hover-panel">{label}</span>
        </TooltipTrigger>
        <TooltipContent>
          <I18nTooltipContent i18nKey={i18n} mode={mode} />
        </TooltipContent>
      </Tooltip>
      {textareaElement}
    </div>
  );
}

export { Textarea };
