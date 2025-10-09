// #region Header

// TransactionalInput.tsx

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
import { Input } from "./Input";
import { Textarea } from "./Textarea";

export interface TransactionalInputProps extends Omit<React.ComponentProps<"input">, "value" | "onChange" | "onFocus" | "onBlur" | "onKeyDown"> {
  label?: string;
  value: string;
  onChange: (value: string) => void;
  startTransaction: () => void;
  finalizeTransaction: () => void;
  abortTransaction: () => void;
}

export interface TransactionalTextareaProps extends Omit<React.ComponentProps<"textarea">, "value" | "onChange" | "onFocus" | "onBlur" | "onKeyDown"> {
  label?: string;
  value: string;
  onChange: (value: string) => void;
  startTransaction: () => void;
  finalizeTransaction: () => void;
  abortTransaction: () => void;
}

export const TransactionalInput = React.forwardRef<HTMLInputElement, TransactionalInputProps>(({ label, value, onChange, startTransaction, finalizeTransaction, abortTransaction, ...props }, ref) => {
  const [localValue, setLocalValue] = React.useState(value);
  const [isEditing, setIsEditing] = React.useState(false);
  React.useEffect(() => {
    if (!isEditing) setLocalValue(value);
  }, [value, isEditing]);
  const handleFocus = () => {
    setIsEditing(true);
    startTransaction();
  };
  const handleBlur = () => {
    setIsEditing(false);
    onChange(localValue);
    finalizeTransaction();
  };
  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      setIsEditing(false);
      onChange(localValue);
      finalizeTransaction();
      (e.target as HTMLInputElement).blur();
    } else if (e.key === "Escape") {
      setIsEditing(false);
      setLocalValue(value);
      abortTransaction();
      (e.target as HTMLInputElement).blur();
    }
  };
  return <Input ref={ref} label={label} value={localValue} onChange={(e) => setLocalValue(e.target.value)} onFocus={handleFocus} onBlur={handleBlur} onKeyDown={handleKeyDown} {...props} />;
});

TransactionalInput.displayName = "TransactionalInput";

export const TransactionalTextarea = React.forwardRef<HTMLTextAreaElement, TransactionalTextareaProps>(({ label, value, onChange, startTransaction, finalizeTransaction, abortTransaction, ...props }, ref) => {
  const [localValue, setLocalValue] = React.useState(value);
  const [isEditing, setIsEditing] = React.useState(false);
  React.useEffect(() => {
    if (!isEditing) setLocalValue(value);
  }, [value, isEditing]);
  const handleFocus = () => {
    setIsEditing(true);
    startTransaction();
  };
  const handleBlur = () => {
    setIsEditing(false);
    onChange(localValue);
    finalizeTransaction();
  };
  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Escape") {
      setIsEditing(false);
      setLocalValue(value);
      abortTransaction();
      (e.target as HTMLTextAreaElement).blur();
    }
  };
  return <Textarea ref={ref} label={label} value={localValue} onChange={(e) => setLocalValue(e.target.value)} onFocus={handleFocus} onBlur={handleBlur} onKeyDown={handleKeyDown} {...props} />;
});

TransactionalTextarea.displayName = "TransactionalTextarea";
