// #region Header

// DesignEditor.tsx

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

const COMMAND_STACK_MAX = 50

import { FC, useCallback, useEffect, useMemo, useRef, useState } from 'react'

import {
    Design,
    DesignId,
    Kit,
    useDesignEditor
} from '@semio/js'

interface CommandParameter {
    name: string
    type: 'string' | 'number' | 'boolean' | 'select' | 'DiagramPoint' | 'Plane' | 'TypeId' | 'PortId' | 'PieceId'
    description: string
    required?: boolean
    options?: { value: string; label: string }[]
    defaultValue?: any | (() => any)
}

export interface CommandContext {
    kit: Kit
    designId: DesignId
    selection: any
}

export interface CommandResult {
    design?: Design
    selection?: any
    fileUrls?: string[]
    fullscreenPanel?: any
    content?: React.ReactNode
}

export interface Command {
    id: string
    name: string
    icon?: string
    description: string
    parameters: CommandParameter[]
    hotkey?: string
    editorOnly?: boolean
    execute: (context: CommandContext, payload: Record<string, any>) => Promise<CommandResult>
}

export interface ParameterForm {
    type: CommandParameter['type']
    component: FC<ParameterFormProps>
}

export interface ParameterFormProps {
    parameter: CommandParameter
    value?: any
    onSubmit: (value: any) => void
    onCancel: () => void
}


class EnhancedCommandRegistry {
    private commands = new Map<string, Command>()

    register(command: Command): () => void {
        this.commands.set(command.id, command)
        return () => this.unregister(command.id)
    }

    unregister(commandId: string): void {
        this.commands.delete(commandId)
    }

    get(commandId: string): Command | undefined {
        return this.commands.get(commandId)
    }

    getAll(): Command[] {
        return Array.from(this.commands.values())
    }

    search(query: string): Command[] {
        const lowercaseQuery = query.toLowerCase()
        return this.getAll().filter(cmd =>
            cmd.name.toLowerCase().includes(lowercaseQuery) ||
            cmd.description.toLowerCase().includes(lowercaseQuery) ||
            cmd.id.toLowerCase().includes(lowercaseQuery)
        )
    }

    async execute(commandId: string, context: CommandContext, payload: Record<string, any> = {}): Promise<CommandResult> {
        const command = this.get(commandId)
        if (!command) throw new Error(`Command '${commandId}' not found`)
        return command.execute(context, payload)
    }
}

export const commandRegistry = new EnhancedCommandRegistry()


export interface ConsoleState {
    mode: 'input' | 'parameter-gathering' | 'command-output'
    input: string
    suggestions: string[]
    selectedSuggestion: number
    currentCommand?: Command
    parameterIndex: number
    gatheredParameters: Record<string, any>
    outputContent?: React.ReactNode
    commandHistory: string[]
    historyIndex: number
}


export interface ParameterFormProps {
    parameter: CommandParameter
    value?: any
    onSubmit: (value: any) => void
    onCancel: () => void
}

export interface ConsoleState {
    mode: 'input' | 'parameter-gathering' | 'command-output'
    input: string
    suggestions: string[]
    selectedSuggestion: number
    currentCommand?: Command
    parameterIndex: number
    gatheredParameters: Record<string, any>
    outputContent?: React.ReactNode
    commandHistory: string[]
    historyIndex: number
}

//#region Forms

const TypeIdForm: FC<ParameterFormProps> = ({ parameter, onSubmit, onCancel }) => {
    const designEditor = useDesignEditor()
    const [selectedIndex, setSelectedIndex] = useState(0)
    const [mode, setMode] = useState<'type' | 'variant'>('type')
    const [selectedType, setSelectedType] = useState<string>('')
    const [filteredItems, setFilteredItems] = useState<string[]>([])
    const [searchTerm, setSearchTerm] = useState('')

    const types = designEditor.kit?.types || []
    const typeNames = useMemo(() => [...new Set(types.map(t => t.name))], [types])
    const variants = useMemo(() =>
        selectedType ? [...new Set(types.filter(t => t.name === selectedType).map(t => t.variant).filter((v): v is string => Boolean(v)))] : [],
        [types, selectedType]
    )

    const currentItems = mode === 'type' ? typeNames : variants

    useEffect(() => {
        const filtered = currentItems.filter(item =>
            item.toLowerCase().includes(searchTerm.toLowerCase())
        )
        setFilteredItems(filtered)
        setSelectedIndex(0)
    }, [currentItems, searchTerm])

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Escape') {
            onCancel()
            return
        }

        if (e.key === 'Tab' && searchTerm) {
            e.preventDefault()
            if (filteredItems.length > 0) {
                setSearchTerm(filteredItems[0])
            }
            return
        }

        if (e.key === 'Enter') {
            e.preventDefault()
            if (mode === 'type') {
                const typeToSelect = filteredItems[selectedIndex] || searchTerm
                if (typeNames.includes(typeToSelect)) {
                    setSelectedType(typeToSelect)
                    setMode('variant')
                    setSearchTerm('')
                    setSelectedIndex(0)
                }
            } else {
                const variantToSelect = filteredItems[selectedIndex] || searchTerm
                onSubmit({ name: selectedType, variant: variantToSelect || undefined })
            }
            return
        }

        if (e.key === 'ArrowUp') {
            e.preventDefault()
            setSelectedIndex(prev => Math.max(0, prev - 1))
            return
        }

        if (e.key === 'ArrowDown') {
            e.preventDefault()
            setSelectedIndex(prev => Math.min(filteredItems.length - 1, prev + 1))
            return
        }
    }

    return (
        <div className="p-2 border-b border-gray-400">
            <div className="text-secondary text-xs mb-1">
                {mode === 'type' ? '🔧 Select Type:' : `🎨 Select Variant for ${selectedType}:`}
            </div>

            <input
                type="text"
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                onKeyDown={handleKeyDown}
                className="w-full bg-dark text-primary border border-gray-400 px-1 py-px mb-1 font-mono text-xs"
                placeholder="Search..."
                autoFocus
            />

            <div className="max-h-24 overflow-y-auto text-xs">
                {filteredItems.map((item, index) => (
                    <div
                        key={item}
                        className={`px-1 py-px cursor-pointer font-mono ${index === selectedIndex ? 'bg-primary text-light' : 'text-primary hover:bg-gray-300'
                            }`}
                        onClick={() => {
                            if (mode === 'type') {
                                setSelectedType(item)
                                setMode('variant')
                                setSearchTerm('')
                                setSelectedIndex(0)
                            } else {
                                onSubmit({ name: selectedType, variant: item || undefined })
                            }
                        }}
                    >
                        {index === selectedIndex ? '> ' : '  '}{item}
                    </div>
                ))}
            </div>

            <div className="text-gray-400 text-xs mt-1">
                {mode === 'type'
                    ? 'Use ↑↓ to navigate, Tab for autocomplete, Enter to select'
                    : 'Use ↑↓ to navigate, Tab for autocomplete, Enter to confirm, or type custom variant'
                }
            </div>
        </div>
    )
}

const StringForm: FC<ParameterFormProps> = ({ parameter, value, onSubmit, onCancel }) => {
    const [inputValue, setInputValue] = useState(value || parameter.defaultValue || '')

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Escape') {
            onCancel()
            return
        }

        if (e.key === 'Enter') {
            e.preventDefault()
            onSubmit(inputValue)
            return
        }
    }

    return (
        <div className="p-2 border-b border-gray-400">
            <div className="text-secondary text-xs mb-1">📝 {parameter.description}:</div>
            <input
                type="text"
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
                onKeyDown={handleKeyDown}
                className="w-full bg-dark text-primary border border-gray-400 px-1 py-px font-mono text-xs"
                placeholder="Enter value..."
                autoFocus
            />
            <div className="text-gray-400 text-xs mt-1">
                Type value and press Enter to confirm, Esc to cancel
            </div>
        </div>
    )
}

const SelectForm: FC<ParameterFormProps> = ({ parameter, onSubmit, onCancel }) => {
    const [selectedIndex, setSelectedIndex] = useState(0)
    const options = parameter.options || []

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Escape') {
            onCancel()
            return
        }

        if (e.key === 'Enter') {
            e.preventDefault()
            onSubmit(options[selectedIndex]?.value)
            return
        }

        if (e.key === 'ArrowUp') {
            e.preventDefault()
            setSelectedIndex(prev => Math.max(0, prev - 1))
            return
        }

        if (e.key === 'ArrowDown') {
            e.preventDefault()
            setSelectedIndex(prev => Math.min(options.length - 1, prev + 1))
            return
        }
    }

    return (
        <div className="p-2 border-b border-gray-400" onKeyDown={handleKeyDown} tabIndex={0}>
            <div className="text-secondary text-xs mb-1">📋 {parameter.description}:</div>

            <div className="max-h-24 overflow-y-auto text-xs">
                {options.map((option, index) => (
                    <div
                        key={option.value}
                        className={`px-1 py-px cursor-pointer font-mono ${index === selectedIndex ? 'bg-primary text-light' : 'text-primary hover:bg-gray-300'
                            }`}
                        onClick={() => onSubmit(option.value)}
                    >
                        {index === selectedIndex ? '> ' : '  '}{option.label}
                    </div>
                ))}
            </div>

            <div className="text-gray-400 text-xs mt-1">
                Use ↑↓ to navigate, Enter to select, Esc to cancel
            </div>
        </div>
    )
}

const BooleanForm: FC<ParameterFormProps> = ({ parameter, onSubmit, onCancel }) => {
    const [value, setValue] = useState(false)

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Escape') {
            onCancel()
            return
        }

        if (e.key === 'Enter') {
            e.preventDefault()
            onSubmit(value)
            return
        }

        if (e.key === 'y' || e.key === 'Y') {
            setValue(true)
        }

        if (e.key === 'n' || e.key === 'N') {
            setValue(false)
        }

        if (e.key === ' ') {
            e.preventDefault()
            setValue(prev => !prev)
        }
    }

    return (
        <div className="p-2 border-b border-gray-400" onKeyDown={handleKeyDown} tabIndex={0}>
            <div className="text-secondary text-xs mb-1">❓ {parameter.description}:</div>
            <div className="mb-1">
                <span className={`font-mono text-xs ${value ? 'text-success' : 'text-danger'}`}>
                    {value ? '✓ Yes' : '✗ No'}
                </span>
            </div>
            <div className="flex gap-1">
                <button
                    onClick={() => setValue(true)}
                    className={`px-2 py-px font-mono text-xs ${value ? 'bg-success text-light' : 'bg-gray-300 text-success'}`}
                >
                    Yes
                </button>
                <button
                    onClick={() => setValue(false)}
                    className={`px-2 py-px font-mono text-xs ${!value ? 'bg-danger text-light' : 'bg-gray-300 text-danger'}`}
                >
                    No
                </button>
            </div>
            <div className="text-gray-400 text-xs mt-1">
                Press Y/N, Space to toggle, Enter to confirm, Esc to cancel
            </div>
        </div>
    )
}

const parameterForms: Map<CommandParameter['type'], FC<ParameterFormProps>> = new Map([
    ['TypeId', TypeIdForm],
    ['string', StringForm],
    ['select', SelectForm],
    ['boolean', BooleanForm],
    // Add more form types as needed
])

//#endregion Forms



const ConsoleCanvas: FC<{ content?: React.ReactNode }> = ({ content }) => {
    if (!content) {
        return (
            <div className="p-2 border border-secondary bg-dark text-xs">
                <div className="text-secondary">🎉 Welcome to Semio Console! Type "help" for available commands.</div>
            </div>
        )
    }

    return (
        <div className="flex-1 border border-secondary bg-dark overflow-auto text-xs">
            {content}
        </div>
    )
}

const ConsoleInput: FC<{
    state: ConsoleState
    onInputChange: (input: string) => void
    onSubmit: () => void
    onCancel: () => void
}> = ({ state, onInputChange, onSubmit, onCancel }) => {
    const inputRef = useRef<HTMLInputElement>(null)

    useEffect(() => {
        if (inputRef.current && state.mode === 'input') {
            inputRef.current.focus()
        }
    }, [state.mode])

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (state.mode === 'parameter-gathering') return

        if (e.key === 'Escape') {
            if (state.mode === 'command-output') {
                onCancel()
            }
            return
        }

        if (e.key === 'Enter') {
            e.preventDefault()
            if (state.mode === 'command-output') {
                onCancel()
            } else {
                onSubmit()
            }
            return
        }

        if (e.key === 'ArrowUp' && state.commandHistory.length > 0) {
            e.preventDefault()
            const newIndex = Math.min(state.historyIndex + 1, state.commandHistory.length - 1)
            if (newIndex !== state.historyIndex) {
                onInputChange(state.commandHistory[state.commandHistory.length - 1 - newIndex])
            }
            return
        }

        if (e.key === 'ArrowDown' && state.historyIndex > -1) {
            e.preventDefault()
            const newIndex = state.historyIndex - 1
            if (newIndex === -1) {
                onInputChange('')
            } else {
                onInputChange(state.commandHistory[state.commandHistory.length - 1 - newIndex])
            }
            return
        }

        if (e.key === 'Tab' && state.suggestions.length > 0) {
            e.preventDefault()
            onInputChange(state.suggestions[state.selectedSuggestion])
            return
        }
    }

    return (
        <div className="">
            {state.suggestions.length > 0 && (
                <div className=" text-xs">
                    <span className="text-warning">Suggestions: </span>
                    {state.suggestions.slice(0, 5).map((suggestion, index) => (
                        <span
                            key={suggestion}
                            className={`ml-2 cursor-pointer ${index === state.selectedSuggestion ? 'text-primary' : 'text-gray-400'}`}
                            onClick={() => onInputChange(suggestion)}
                        >
                            {suggestion}
                        </span>
                    ))}
                </div>
            )}

            <div className="p-1 flex items-center text-xs">
                <span className="text-primary mr-1 font-mono">$</span>
                <input
                    ref={inputRef}
                    type="text"
                    value={state.input}
                    onChange={(e) => onInputChange(e.target.value)}
                    onKeyDown={handleKeyDown}
                    className="flex-1 bg-transparent text-primary outline-none font-mono text-xs"
                    placeholder="Type command..."
                    disabled={state.mode !== 'input'}
                />
                <span className="text-primary ml-1 font-mono animate-pulse">▋</span>
            </div>

            <div className="px-1 pb-1">
                <div className="text-gray-400 text-xs">
                    {state.mode === 'input'
                        ? 'Type command, Tab for autocomplete, ↑↓ for history'
                        : state.mode === 'command-output'
                            ? 'Press Esc or Enter to return to input mode'
                            : 'Gathering parameters...'
                    }
                </div>
            </div>
        </div>
    )
}

const Console: FC = () => {
    const designEditor = useDesignEditor()
    const [state, setState] = useState<ConsoleState>({
        mode: 'input',
        input: '',
        suggestions: [],
        selectedSuggestion: 0,
        parameterIndex: 0,
        gatheredParameters: {},
        commandHistory: [],
        historyIndex: -1
    })

    const updateSuggestions = useCallback((input: string) => {
        if (!input.trim()) {
            setState(prev => ({ ...prev, suggestions: [], selectedSuggestion: 0 }))
            return
        }

        const commands = commandRegistry.search(input)
        const suggestions = commands.map(cmd => cmd.name)

        setState(prev => ({
            ...prev,
            suggestions,
            selectedSuggestion: Math.min(prev.selectedSuggestion, suggestions.length - 1)
        }))
    }, [commandRegistry])

    const handleInputChange = useCallback((input: string) => {
        setState(prev => ({ ...prev, input, historyIndex: -1 }))
        updateSuggestions(input)
    }, [updateSuggestions])

    const executeCommand = useCallback(async (commandName: string, payload: Record<string, any> = {}) => {
        const command = commandRegistry.getAll().find(cmd =>
            cmd.name.toLowerCase() === commandName.toLowerCase() ||
            cmd.id.toLowerCase() === commandName.toLowerCase()
        )

        if (!command) {
            setState(prev => ({
                ...prev,
                mode: 'command-output',
                outputContent: <div className="p-2 text-xs text-danger">❌ Command not found: {commandName}</div>
            }))

            // Auto-return to input mode after 2 seconds
            setTimeout(() => {
                setState(prev => prev.mode === 'command-output' ? {
                    ...prev,
                    mode: 'input',
                    outputContent: undefined
                } : prev)
            }, 2000)
            return
        }

        try {
            const context: CommandContext = {
                kit: designEditor.kit!,
                designId: designEditor.designId,
                selection: designEditor.selection
            }

            // If command has editorOnly flag, don't use transaction scope
            if (command.editorOnly) {
                const result = await command.execute(context, payload)

                // Apply editor-only changes
                if (result.selection) {
                    designEditor.setSelection(result.selection)
                }

                setState(prev => ({
                    ...prev,
                    mode: 'command-output',
                    outputContent: result.content || <div className="p-2 text-xs text-success">✅ Command executed successfully</div>
                }))
            } else {
                // Use transaction scope for design-modifying commands
                designEditor.startTransaction()

                try {
                    const result = await command.execute(context, payload)

                    // Apply changes through transaction
                    if (result.design) {
                        designEditor.setDesign(result.design)
                    }
                    if (result.selection) {
                        designEditor.setSelection(result.selection)
                    }
                    if (result.fullscreenPanel) {
                        // TODO: Handle fullscreen panel - check if designEditor has this property
                        // designEditor.setFullscreenPanel(result.fullscreenPanel)
                    }

                    designEditor.finalizeTransaction()

                    setState(prev => ({
                        ...prev,
                        mode: 'command-output',
                        outputContent: result.content || <div className="p-2 text-xs text-success">✅ Command executed successfully</div>
                    }))

                    // Auto-return to input mode after 2 seconds unless it's clear command
                    if (command.id !== 'clear') {
                        setTimeout(() => {
                            setState(prev => prev.mode === 'command-output' ? {
                                ...prev,
                                mode: 'input',
                                outputContent: undefined
                            } : prev)
                        }, 2000)
                    }

                    // Auto-return to input mode after 2 seconds unless it's clear command
                    if (command.id !== 'clear') {
                        setTimeout(() => {
                            setState(prev => prev.mode === 'command-output' ? {
                                ...prev,
                                mode: 'input',
                                outputContent: undefined
                            } : prev)
                        }, 2000)
                    }
                } catch (error) {
                    designEditor.abortTransaction()
                    throw error
                }
            }
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred'
            setState(prev => ({
                ...prev,
                mode: 'command-output',
                outputContent: <div className="p-2 text-xs text-danger">❌ Error: {errorMessage}</div>
            }))

            // Auto-return to input mode after 3 seconds for errors
            setTimeout(() => {
                setState(prev => prev.mode === 'command-output' ? {
                    ...prev,
                    mode: 'input',
                    outputContent: undefined
                } : prev)
            }, 3000)
        }
    }, [commandRegistry, designEditor])

    const executeCommandWithParameters = useCallback(async (commandName: string) => {
        const command = commandRegistry.getAll().find(cmd =>
            cmd.name.toLowerCase() === commandName.toLowerCase() ||
            cmd.id.toLowerCase() === commandName.toLowerCase()
        )

        if (!command) {
            setState(prev => ({
                ...prev,
                mode: 'command-output',
                outputContent: <div className="p-2 text-xs text-danger">❌ Command not found: {commandName}</div>
            }))
            return
        }

        if (command.parameters.length === 0) {
            await executeCommand(commandName, {})
        } else {
            // Start parameter gathering
            setState(prev => ({
                ...prev,
                mode: 'parameter-gathering',
                currentCommand: command,
                parameterIndex: 0,
                gatheredParameters: {}
            }))
        }
    }, [executeCommand, commandRegistry])

    const handleSubmit = useCallback(() => {
        if (state.mode === 'input') {
            const command = state.input.trim()
            if (command) {
                setState(prev => ({
                    ...prev,
                    commandHistory: [command, ...prev.commandHistory.slice(0, 99)],
                    input: ''
                }))
                executeCommandWithParameters(command)
            }
        }
    }, [state, executeCommandWithParameters])

    const handleCancel = useCallback(() => {
        setState(prev => ({
            ...prev,
            mode: 'input',
            input: '',
            currentCommand: undefined,
            parameterIndex: 0,
            gatheredParameters: {},
            outputContent: undefined
        }))
    }, [])

    const handleParameterSubmit = useCallback((value: any) => {
        if (!state.currentCommand) return

        const newParameters = {
            ...state.gatheredParameters,
            [state.currentCommand.parameters[state.parameterIndex].name]: value
        }

        if (state.parameterIndex >= state.currentCommand.parameters.length - 1) {
            // All parameters gathered, execute command
            executeCommand(state.currentCommand.name, newParameters)
            setState(prev => ({
                ...prev,
                mode: 'input',
                currentCommand: undefined,
                parameterIndex: 0,
                gatheredParameters: {}
            }))
        } else {
            // Move to next parameter
            setState(prev => ({
                ...prev,
                parameterIndex: prev.parameterIndex + 1,
                gatheredParameters: newParameters
            }))
        }
    }, [state, executeCommand])

    // Render parameter form if in parameter gathering mode
    const renderParameterForm = () => {
        if (state.mode !== 'parameter-gathering' || !state.currentCommand) return null

        const parameter = state.currentCommand.parameters[state.parameterIndex]
        const FormComponent = parameterForms.get(parameter.type)

        if (!FormComponent) {
            return <div className="p-2 text-xs text-danger">❌ Unsupported parameter type: {parameter.type}</div>
        }

        return (
            <FormComponent
                parameter={parameter}
                value={state.gatheredParameters[parameter.name]}
                onSubmit={handleParameterSubmit}
                onCancel={handleCancel}
            />
        )
    }

    const upperContent = state.mode === 'parameter-gathering'
        ? renderParameterForm()
        : state.outputContent

    return (
        <div className="h-full flex flex-col justify-end">
            <ConsoleCanvas content={upperContent} />
            <ConsoleInput
                state={state}
                onInputChange={handleInputChange}
                onSubmit={handleSubmit}
                onCancel={handleCancel}
            />
        </div>
    )
}

export default Console