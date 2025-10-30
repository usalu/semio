// #region Header

// Input.tsx

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
import { useActiveInteraction, useSketchpadCommands } from "../../sketchpad/store";
import { IdTooltipContent, Tooltip, TooltipContent, TooltipTrigger, useTooltipMode } from "../display/Tooltip";

interface InputProps extends Omit<React.ComponentProps<"input">, "value" | "onChange"> {
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onLazyChange?: (value: string) => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
  abortTransaction?: () => void;
  interactionId?: string;
  id: string;
  placeholderId?: string;
  showLabel?: boolean;
}

function Input({ className, type, lazy, value: externalValue, onChange, onLazyChange, startTransaction, finalizeTransaction, abortTransaction, interactionId, id, placeholderId, placeholder, showLabel, ...props }: InputProps) {
  const [localValue, setLocalValue] = React.useState(externalValue?.toString() || "");
  const [isEditing, setIsEditing] = React.useState(false);
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();
  const mode = useTooltipMode();
  const { t } = useTranslation();
  const computedPlaceholder = placeholderId ? t(`${placeholderId}.label`) : placeholder;

  React.useEffect(() => {
    if (!isEditing) setLocalValue(externalValue?.toString() || "");
  }, [externalValue, isEditing]);

  const isInteracting = interactionId && activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (lazy) {
      setLocalValue(e.target.value);
    } else if (onChange) {
      onChange(e);
    }
  };

  const handleFocus = (e: React.FocusEvent<HTMLInputElement>) => {
    if (interactionId) setActiveInteraction(interactionId);
    if (lazy) {
      setIsEditing(true);
      startTransaction?.();
    }
    props.onFocus?.(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLInputElement>) => {
    if (interactionId) setActiveInteraction(undefined);
    if (lazy) {
      setIsEditing(false);
      onLazyChange?.(localValue);
      finalizeTransaction?.();
    }
    props.onBlur?.(e);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (lazy) {
      if (e.key === "Enter") {
        if (interactionId) setActiveInteraction(undefined);
        setIsEditing(false);
        onLazyChange?.(localValue);
        finalizeTransaction?.();
        (e.target as HTMLInputElement).blur();
      } else if (e.key === "Escape") {
        if (interactionId) setActiveInteraction(undefined);
        setIsEditing(false);
        setLocalValue(externalValue?.toString() || "");
        abortTransaction?.();
        (e.target as HTMLInputElement).blur();
      }
    }
    props.onKeyDown?.(e);
  };

  const inputValue = lazy ? localValue : externalValue;

  const inputElement = (
    <input
      type={type}
      data-slot="input"
      className={cn(
        "file:text-foreground placeholder:text-muted-foreground text-foreground flex h-9 w-full min-w-0 border bg-transparent px-3 py-2 text-base transition-[color,border-color] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "focus-visible:border-accent",
        "aria-invalid:ring-destructive/20 aria-invalid:border-destructive flex-1",
        type === "number" && "[&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [-moz-appearance:textfield]",
        className,
      )}
      value={inputValue}
      onChange={handleChange}
      onFocus={handleFocus}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
      placeholder={computedPlaceholder}
      {...props}
    />
  );

  if (!showLabel) {
    return <div style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}>{inputElement}</div>;
  }

  const label = t(`${id}.label`);

  return (
    <div className="group flex items-center gap-2 min-w-0 h-9 w-full" style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}>
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="inline-flex h-full items-center px-3 text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate cursor-pointer transition-colors group-hover:bg-hover-panel">{label}</span>
        </TooltipTrigger>
        <TooltipContent>
          <IdTooltipContent id={id} mode={mode} />
        </TooltipContent>
      </Tooltip>
      {inputElement}
    </div>
  );
}

export { Input };
