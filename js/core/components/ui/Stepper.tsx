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
}

const Stepper: FC<StepperProps> = ({
    value,
    defaultValue = 0,
    min,
    max,
    step = 1,
    onChange
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
        <div className="flex items-center">
            <button
                type="button"
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
                className="rounded-none border-l-0 border-r-0 text-center"
                step={step}
                min={min}
                max={max}
            />
            <button
                type="button"
                onClick={handleStepUp}
                disabled={!canStepUp}
                className="h-9 w-9 border border-l-0 rounded-r-md bg-background hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
            >
                <Plus className="h-4 w-4" />
            </button>
        </div>
    )
}

export default Stepper