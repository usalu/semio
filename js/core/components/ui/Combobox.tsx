import { cn } from "@semio/js/lib/utils"
import { Check, ChevronsUpDown } from "lucide-react"
import { FC, useState } from "react"
import { Button } from "./Button"
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from "./Command"
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "./Popover"

interface ComboboxOption {
    value: string
    label: string
}

interface ComboboxProps {
    options: ComboboxOption[]
    value?: string
    placeholder?: string
    emptyMessage?: string
    onValueChange?: (value: string) => void
    className?: string
    allowClear?: boolean
    label?: string
}

const Combobox: FC<ComboboxProps> = ({
    options,
    value = '',
    placeholder = 'Select option...',
    emptyMessage = 'No options found.',
    onValueChange,
    className,
    allowClear = false,
    label
}) => {
    const [open, setOpen] = useState(false)

    const selectedOption = options.find(option => option.value === value)

    const handleSelect = (optionValue: string) => {
        if (allowClear && optionValue === value) {
            onValueChange?.('')
        } else {
            onValueChange?.(optionValue)
        }
        setOpen(false)
    }

    return (
        <div className={cn("flex items-center gap-2 border-b border-border pb-1", className)}>
            {label && <span className="text-sm font-medium flex-shrink-0 min-w-[80px] text-left">{label}</span>}
            <Popover open={open} onOpenChange={setOpen}>
                <PopoverTrigger asChild>
                    <Button
                        variant="outline"
                        role="combobox"
                        aria-expanded={open}
                        className="w-full justify-between flex-1"
                    >
                        {selectedOption ? selectedOption.label : placeholder}
                        <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                    </Button>
                </PopoverTrigger>
                <PopoverContent className="w-full p-0" align="start">
                    <Command>
                        <CommandInput placeholder="Search..." />
                        <CommandList>
                            <CommandEmpty>{emptyMessage}</CommandEmpty>
                            <CommandGroup>
                                {allowClear && value && (
                                    <CommandItem
                                        value=""
                                        onSelect={() => handleSelect('')}
                                    >
                                        <div className="mr-2 h-4 w-4" />
                                        <span className="text-muted-foreground italic">Clear selection</span>
                                    </CommandItem>
                                )}
                                {options.map((option) => (
                                    <CommandItem
                                        key={option.value}
                                        value={option.value}
                                        onSelect={() => handleSelect(option.value)}
                                    >
                                        <Check className={cn("mr-2 h-4 w-4", value === option.value ? "opacity-100" : "opacity-0")} />
                                        {option.label}
                                    </CommandItem>
                                ))}
                            </CommandGroup>
                        </CommandList>
                    </Command>
                </PopoverContent>
            </Popover>
        </div>
    )
}

export default Combobox
