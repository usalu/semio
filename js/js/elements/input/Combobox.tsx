import { Check, ChevronsUpDown } from "lucide-react";
import { FC, useState } from "react";
import { useTranslation } from "react-i18next";
import { cn } from "../../semio";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from "../Command";
import { Popover, PopoverContent, PopoverTrigger } from "../Popover";
import { IdTooltipContent, Tooltip, TooltipContent, TooltipTrigger, useTooltipMode } from "../display/Tooltip";
import { Button } from "./Button";

interface ComboboxOption {
  value: string;
  label: string;
}

interface ComboboxProps {
  options: ComboboxOption[];
  value?: string;
  placeholder?: string;
  placeholderId?: string;
  emptyMessage?: string;
  onValueChange?: (value: string) => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
  className?: string;
  allowClear?: boolean;
  showLabel?: boolean;
  id: string;
}

const Combobox: FC<ComboboxProps> = ({ options, value = "", placeholder = "Select option...", placeholderId, emptyMessage = "No options found.", onValueChange, startTransaction, finalizeTransaction, className, allowClear = false, showLabel, id }) => {
  const [open, setOpen] = useState(false);
  const mode = useTooltipMode();
  const { t } = useTranslation();
  const computedPlaceholder = placeholderId ? t(placeholderId) : placeholder;

  const selectedOption = options.find((option) => option.value === value);

  const handleOpenChange = (isOpen: boolean) => {
    setOpen(isOpen);
    if (isOpen) {
      startTransaction?.();
    } else {
      finalizeTransaction?.();
    }
  };

  const handleSelect = (optionValue: string) => {
    if (allowClear && optionValue === value) {
      onValueChange?.("");
    } else {
      onValueChange?.(optionValue);
    }
    setOpen(false);
    finalizeTransaction?.();
  };

  const popoverTrigger = (
    <PopoverTrigger asChild>
      <Button variant="default" role="combobox" aria-expanded={open} className="w-full justify-between flex-1 min-w-0">
        {selectedOption ? selectedOption.label : computedPlaceholder}
        <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
      </Button>
    </PopoverTrigger>
  );

  const wrappedTrigger = id ? (
    <Tooltip>
      <TooltipTrigger asChild>{popoverTrigger}</TooltipTrigger>
      <TooltipContent>
        <IdTooltipContent id={id} mode={mode} />
      </TooltipContent>
    </Tooltip>
  ) : (
    popoverTrigger
  );

  return (
    <div className={cn("group flex items-center gap-2 min-w-0 w-full", className)}>
      {showLabel && (
        <Tooltip>
          <TooltipTrigger asChild>
            <span className="inline-flex h-9 items-center px-3 text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate cursor-pointer transition-colors group-hover:bg-hover-panel">{t(`${id}.label`)}</span>
          </TooltipTrigger>
          <TooltipContent>
            <IdTooltipContent id={id} mode={mode} />
          </TooltipContent>
        </Tooltip>
      )}
      <Popover open={open} onOpenChange={handleOpenChange}>
        {wrappedTrigger}
        <PopoverContent className="w-full p-0" align="start">
          <Command>
            <CommandInput placeholder="Search..." />
            <CommandList>
              <CommandEmpty>{emptyMessage}</CommandEmpty>
              <CommandGroup>
                {allowClear && value && (
                  <CommandItem value="" onSelect={() => handleSelect("")}>
                    <div className="mr-2 h-4 w-4" />
                    <span className="text-muted-foreground italic">Clear selection</span>
                  </CommandItem>
                )}
                {options.map((option) => (
                  <CommandItem key={option.value} value={option.value} onSelect={() => handleSelect(option.value)}>
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
  );
};

export default Combobox;
