// #region Header

// Stepper.tsx

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

// #region TODOs

// #endregion TODOs
import { Minus, Plus } from "lucide-react";
import { FC, useCallback, useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useActiveInteraction, useSketchpadCommands } from "../../sketchpad/store";
import { I18nTooltipContent, Tooltip, TooltipContent, TooltipTrigger, useTooltipMode } from "../display/Tooltip";
import { Input } from "./Input";

interface StepperProps {
  value?: number;
  defaultValue?: number;
  min?: number;
  max?: number;
  step?: number;
  onChange?: (value: number) => void;
  onPointerDown?: () => void;
  onPointerUp?: () => void;
  onPointerCancel?: () => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
  abortTransaction?: () => void;
  interactionId?: string;
  i18n: string;
}

const Stepper: FC<StepperProps> = ({ value, defaultValue = 0, min, max, step = 1, onChange, onPointerDown, onPointerUp, onPointerCancel, startTransaction, finalizeTransaction, abortTransaction, interactionId, i18n }) => {
  const [internalValue, setInternalValue] = useState(value ?? defaultValue);
  const { t } = useTranslation();
  const [isEditing, setIsEditing] = useState(false);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();
  const mode = useTooltipMode();

  useEffect(() => {
    if (value !== undefined) {
      setInternalValue(value);
    }
  }, [value]);

  const clampValue = useCallback(
    (val: number): number => {
      let clampedValue = val;
      if (min !== undefined) clampedValue = Math.max(clampedValue, min);
      if (max !== undefined) clampedValue = Math.min(clampedValue, max);
      return clampedValue;
    },
    [min, max],
  );

  const updateValue = useCallback(
    (newValue: number) => {
      const clampedValue = clampValue(newValue);
      setInternalValue(clampedValue);
      onChange?.(clampedValue);
    },
    [clampValue, onChange],
  );

  const startContinuousChange = useCallback(
    (increment: number) => {
      // Clear any existing intervals
      if (intervalRef.current) clearInterval(intervalRef.current);
      if (timeoutRef.current) clearTimeout(timeoutRef.current);

      // Start after a delay
      timeoutRef.current = setTimeout(() => {
        intervalRef.current = setInterval(() => {
          setInternalValue((prev) => {
            const newValue = clampValue(prev + increment);
            onChange?.(newValue);
            return newValue;
          });
        }, 100); // Update every 100ms
      }, 500); // Start continuous after 500ms
    },
    [clampValue, onChange],
  );

  const stopContinuousChange = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
  }, []);

  useEffect(() => {
    return () => {
      stopContinuousChange();
    };
  }, [stopContinuousChange]);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = parseFloat(e.target.value);
    if (!isNaN(newValue)) {
      updateValue(newValue);
    }
  };

  const handleStepUp = () => {
    updateValue(internalValue + step);
  };

  const handleStepDown = () => {
    updateValue(internalValue - step);
  };

  const handleMouseDown = (increment: number) => {
    return () => {
      if (interactionId) setActiveInteraction(interactionId);
      if (!isEditing) {
        setIsEditing(true);
        startTransaction?.();
      }
      onPointerDown?.();
      if (increment > 0) {
        handleStepUp();
      } else {
        handleStepDown();
      }
      startContinuousChange(increment);
    };
  };

  const handleMouseUp = () => {
    stopContinuousChange();
    if (interactionId) setActiveInteraction(undefined);
    if (isEditing) {
      setIsEditing(false);
      finalizeTransaction?.();
    }
    onPointerUp?.();
  };

  const handleMouseLeave = () => {
    stopContinuousChange();
    if (interactionId) setActiveInteraction(undefined);
    if (isEditing) {
      setIsEditing(false);
      finalizeTransaction?.();
    }
    onPointerCancel?.();
  };

  const canStepDown = min === undefined || internalValue > min;
  const canStepUp = max === undefined || internalValue < max;

  const isInteracting = interactionId && activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;

  const stepperElement = (
    <div className="flex items-center flex-1 min-w-0 h-9">
      <button
        type="button"
        onMouseDown={handleMouseDown(-step)}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleMouseDown(-step)}
        onTouchEnd={handleMouseUp}
        disabled={!canStepDown}
        className="h-9 w-9 border border-r-0 hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
      >
        <Minus className="h-4 w-4" />
      </button>
      <Input
        type="number"
        value={internalValue.toString()}
        onChange={handleInputChange}
        onFocus={() => {
          if (!isEditing) {
            setIsEditing(true);
            startTransaction?.();
          }
          onPointerDown?.();
        }}
        onBlur={() => {
          if (isEditing) {
            setIsEditing(false);
            finalizeTransaction?.();
          }
          onPointerUp?.();
        }}
        onKeyDown={(e) => {
          if (e.key === "ArrowUp" || e.key === "ArrowDown") {
            e.preventDefault();
            if (!isEditing) {
              setIsEditing(true);
              startTransaction?.();
            }
            if (e.key === "ArrowUp") {
              handleStepUp();
            } else {
              handleStepDown();
            }
          } else if (e.key === "Escape") {
            if (isEditing) {
              setIsEditing(false);
              setInternalValue(value ?? defaultValue);
              abortTransaction?.();
              (e.target as HTMLInputElement).blur();
            }
          } else if (e.key === "Enter") {
            if (isEditing) {
              setIsEditing(false);
              finalizeTransaction?.();
              (e.target as HTMLInputElement).blur();
            }
          }
        }}
        className="border-l-0 border-r-0 text-center"
        step={step}
        min={min}
        max={max}
        i18n={i18n}
      />
      <button
        type="button"
        onMouseDown={handleMouseDown(step)}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleMouseDown(step)}
        onTouchEnd={handleMouseUp}
        disabled={!canStepUp}
        className="h-9 w-9 border border-l-0 hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
      >
        <Plus className="h-4 w-4" />
      </button>
    </div>
  );

  const label = t(`${i18n}.label`);

  return (
    <div className="flex items-center gap-2 min-w-0 h-9 w-full transition-colors hover:bg-hover-panel" style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}>
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate cursor-pointer transition-colors">{label}</span>
        </TooltipTrigger>
        <TooltipContent>
          <I18nTooltipContent i18nKey={i18n} mode={mode} />
        </TooltipContent>
      </Tooltip>
      {stepperElement}
    </div>
  );
};

export default Stepper;
