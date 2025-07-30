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
import { Minus, Plus } from "lucide-react"
import { FC, useEffect, useState } from "react"
import { Input } from "./Input"

interface StepperProps {
    value?: number
    defaultValue?: number
    min?: number
    max?: number
    step?: number
    onChange?: (value: number) => void
    onPointerDown?: () => void
    onPointerUp?: () => void
    onPointerCancel?: () => void
    label?: string
}

const Stepper: FC<StepperProps> = ({
    value,
    defaultValue = 0,
    min,
    max,
    step = 1,
    onChange,
    onPointerDown,
    onPointerUp,
    onPointerCancel,
    label
}) => {
    const [internalValue, setInternalValue] = useState(value ?? defaultValue)

    useEffect(() => {
        if (value !== undefined) {
            setInternalValue(value)
        }
    }, [value])

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = parseFloat(e.target.value)
        if (!isNaN(newValue)) {
            const clampedValue = clampValue(newValue)
            setInternalValue(clampedValue)
            onChange?.(clampedValue)
        }
    }

    const handleStepUp = () => {
        const newValue = clampValue(internalValue + step)
        setInternalValue(newValue)
        onChange?.(newValue)
    }

    const handleStepDown = () => {
        const newValue = clampValue(internalValue - step)
        setInternalValue(newValue)
        onChange?.(newValue)
    }

    const clampValue = (val: number): number => {
        let clampedValue = val
        if (min !== undefined) clampedValue = Math.max(clampedValue, min)
        if (max !== undefined) clampedValue = Math.min(clampedValue, max)
        return clampedValue
    }

    const canStepDown = min === undefined || internalValue > min
    const canStepUp = max === undefined || internalValue < max

    return (
        <div className="flex items-center gap-2 border-b border-border pb-1">
            {label && <span className="text-sm font-medium flex-shrink-0 min-w-[80px] text-left">{label}</span>}
            <div className="flex items-center flex-1">
                <button
                    type="button"
                    onPointerDown={onPointerDown}
                    onPointerUp={onPointerUp}
                    onPointerCancel={onPointerCancel}
                    onClick={handleStepDown}
                    disabled={!canStepDown}
                    className="h-9 w-9 border border-r-0 rounded-l-md bg-background hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
                >
                    <Minus className="h-4 w-4" />
                </button>
                <Input
                    type="number"
                    value={internalValue.toString()}
                    onChange={handleInputChange}
                    onFocus={onPointerDown}
                    onBlur={onPointerUp}
                    className="rounded-none border-l-0 border-r-0 text-center"
                    step={step}
                    min={min}
                    max={max}
                />
                <button
                    type="button"
                    onPointerDown={onPointerDown}
                    onPointerUp={onPointerUp}
                    onPointerCancel={onPointerCancel}
                    onClick={handleStepUp}
                    disabled={!canStepUp}
                    className="h-9 w-9 border border-l-0 rounded-r-md bg-background hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
                >
                    <Plus className="h-4 w-4" />
                </button>
            </div>
        </div>
    )
}

export default Stepper