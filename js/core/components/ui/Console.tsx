// #region Header

// Console.tsx

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

import { Box, Text, useInput } from 'ink'
import { FC, useCallback, useEffect, useMemo, useState } from 'react'

import {
    Design,
    DesignId,
    Kit,
    useDesignEditor
} from '@semio/js'

const COMMAND_STACK_MAX = 50

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

    useInput((input, key) => {
        if (key.escape) {
            onCancel()
            return
        }

        if (key.tab && searchTerm) {
            if (filteredItems.length > 0) {
                setSearchTerm(filteredItems[0])
            }
            return
        }

        if (key.return) {
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

        if (key.upArrow) {
            setSelectedIndex(prev => Math.max(0, prev - 1))
            return
        }

        if (key.downArrow) {
            setSelectedIndex(prev => Math.min(filteredItems.length - 1, prev + 1))
            return
        }

        if (input && !key.ctrl && !key.meta) {
            setSearchTerm(prev => prev + input)
        }

        if (key.backspace) {
            setSearchTerm(prev => prev.slice(0, -1))
        }
    })

    return (
        <Box flexDirection="column" borderStyle="single" paddingX={1}>
            <Text color="gray">
                {mode === 'type' ? '🔧 Select Type:' : `🎨 Select Variant for ${selectedType}:`}
            </Text>

            <Box marginY={1}>
                <Text>Search: {searchTerm}▋</Text>
            </Box>

            <Box flexDirection="column" height={6}>
                {filteredItems.slice(0, 5).map((item, index) => (
                    <Text key={item} color={index === selectedIndex ? 'blue' : 'white'}>
                        {index === selectedIndex ? '> ' : '  '}{item}
                    </Text>
                ))}
            </Box>

            <Text color="gray" dimColor>
                {mode === 'type'
                    ? 'Use ↑↓ to navigate, Tab for autocomplete, Enter to select'
                    : 'Use ↑↓ to navigate, Tab for autocomplete, Enter to confirm, or type custom variant'
                }
            </Text>
        </Box>
    )
}

const StringForm: FC<ParameterFormProps> = ({ parameter, value, onSubmit, onCancel }) => {
    const [inputValue, setInputValue] = useState(value || parameter.defaultValue || '')

    useInput((input, key) => {
        if (key.escape) {
            onCancel()
            return
        }

        if (key.return) {
            onSubmit(inputValue)
            return
        }

        if (input && !key.ctrl && !key.meta) {
            setInputValue((prev: string) => prev + input)
        }

        if (key.backspace) {
            setInputValue((prev: string) => prev.slice(0, -1))
        }
    })

    return (
        <Box flexDirection="column" borderStyle="single" paddingX={1}>
            <Text color="gray">📝 {parameter.description}:</Text>
            <Box marginY={1}>
                <Text>{inputValue}▋</Text>
            </Box>
            <Text color="gray" dimColor>
                Type value and press Enter to confirm, Esc to cancel
            </Text>
        </Box>
    )
}

const SelectForm: FC<ParameterFormProps> = ({ parameter, onSubmit, onCancel }) => {
    const [selectedIndex, setSelectedIndex] = useState(0)
    const options = parameter.options || []

    useInput((input, key) => {
        if (key.escape) {
            onCancel()
            return
        }

        if (key.return) {
            onSubmit(options[selectedIndex]?.value)
            return
        }

        if (key.upArrow) {
            setSelectedIndex(prev => Math.max(0, prev - 1))
            return
        }

        if (key.downArrow) {
            setSelectedIndex(prev => Math.min(options.length - 1, prev + 1))
            return
        }
    })

    return (
        <Box flexDirection="column" borderStyle="single" paddingX={1}>
            <Text color="gray">📋 {parameter.description}:</Text>

            <Box flexDirection="column" marginY={1} height={6}>
                {options.map((option, index) => (
                    <Text key={option.value} color={index === selectedIndex ? 'blue' : 'white'}>
                        {index === selectedIndex ? '> ' : '  '}{option.label}
                    </Text>
                ))}
            </Box>

            <Text color="gray" dimColor>
                Use ↑↓ to navigate, Enter to select, Esc to cancel
            </Text>
        </Box>
    )
}

const BooleanForm: FC<ParameterFormProps> = ({ parameter, onSubmit, onCancel }) => {
    const [value, setValue] = useState(false)

    useInput((input, key) => {
        if (key.escape) {
            onCancel()
            return
        }

        if (key.return) {
            onSubmit(value)
            return
        }

        if (input === 'y' || input === 'Y') {
            setValue(true)
        }

        if (input === 'n' || input === 'N') {
            setValue(false)
        }

        if (input === ' ') {
            setValue(prev => !prev)
        }
    })

    return (
        <Box flexDirection="column" borderStyle="single" paddingX={1}>
            <Text color="gray">❓ {parameter.description}:</Text>
            <Box marginY={1}>
                <Text color={value ? 'green' : 'red'}>
                    {value ? '✓ Yes' : '✗ No'}
                </Text>
            </Box>
            <Text color="gray" dimColor>
                Press Y/N, Space to toggle, Enter to confirm, Esc to cancel
            </Text>
        </Box>
    )
}

const parameterForms: Map<CommandParameter['type'], FC<ParameterFormProps>> = new Map([
    ['TypeId', TypeIdForm],
    ['string', StringForm],
    ['select', SelectForm],
    ['boolean', BooleanForm],
])

//#endregion Forms

const ConsoleCanvas: FC<{ content?: React.ReactNode }> = ({ content }) => {
    if (!content) {
        return (
            <Box borderStyle="single" padding={1}>
                <Text color="gray">🎉 Welcome to Semio Console! Type "help" for available commands.</Text>
            </Box>
        )
    }

    return (
        <Box flexDirection="column" borderStyle="single" padding={1} height={20}>
            {content}
        </Box>
    )
}

const ConsoleInput: FC<{
    state: ConsoleState
    onInputChange: (input: string) => void
    onSubmit: () => void
    onCancel: () => void
}> = ({ state, onInputChange, onSubmit, onCancel }) => {

    useInput((input, key) => {
        if (state.mode === 'parameter-gathering') return

        if (key.escape) {
            if (state.mode === 'command-output') {
                onCancel()
            }
            return
        }

        if (key.return) {
            if (state.mode === 'command-output') {
                onCancel()
            } else {
                onSubmit()
            }
            return
        }

        if (key.upArrow && state.commandHistory.length > 0) {
            const newIndex = Math.min(state.historyIndex + 1, state.commandHistory.length - 1)
            if (newIndex !== state.historyIndex) {
                onInputChange(state.commandHistory[state.commandHistory.length - 1 - newIndex])
            }
            return
        }

        if (key.downArrow && state.historyIndex > -1) {
            const newIndex = state.historyIndex - 1
            if (newIndex === -1) {
                onInputChange('')
            } else {
                onInputChange(state.commandHistory[state.commandHistory.length - 1 - newIndex])
            }
            return
        }

        if (key.tab && state.suggestions.length > 0) {
            onInputChange(state.suggestions[state.selectedSuggestion])
            return
        }

        if (state.mode === 'input') {
            if (input && !key.ctrl && !key.meta) {
                onInputChange(state.input + input)
            }

            if (key.backspace) {
                onInputChange(state.input.slice(0, -1))
            }
        }
    })

    return (
        <Box flexDirection="column">
            {state.suggestions.length > 0 && (
                <Box marginBottom={1}>
                    <Text color="yellow">Suggestions: </Text>
                    {state.suggestions.slice(0, 5).map((suggestion, index) => (
                        <Text key={suggestion} color={index === state.selectedSuggestion ? 'blue' : 'gray'}>
                            {' '}{suggestion}
                        </Text>
                    ))}
                </Box>
            )}

            <Box>
                <Text color="blue">$ </Text>
                <Text>{state.input}</Text>
                {state.mode === 'input' && <Text>▋</Text>}
            </Box>

            <Box marginTop={1}>
                <Text color="gray" dimColor>
                    {state.mode === 'input'
                        ? 'Type command, Tab for autocomplete, ↑↓ for history'
                        : state.mode === 'command-output'
                            ? 'Press Esc or Enter to return to input mode'
                            : 'Gathering parameters...'
                    }
                </Text>
            </Box>
        </Box>
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
    }, [])

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
                outputContent: <Text color="red">❌ Command not found: {commandName}</Text>
            }))

            setTimeout(() => {
                setState((prev: ConsoleState) => prev.mode === 'command-output' ? {
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

            if (command.editorOnly) {
                const result = await command.execute(context, payload)

                if (result.selection) {
                    designEditor.setSelection(result.selection)
                }

                setState(prev => ({
                    ...prev,
                    mode: 'command-output',
                    outputContent: result.content || <Text color="green">✅ Command executed successfully</Text>
                }))
            } else {
                designEditor.startTransaction()

                try {
                    const result = await command.execute(context, payload)

                    if (result.design) {
                        designEditor.setDesign(result.design)
                    }
                    if (result.selection) {
                        designEditor.setSelection(result.selection)
                    }

                    designEditor.finalizeTransaction()

                    setState(prev => ({
                        ...prev,
                        mode: 'command-output',
                        outputContent: result.content || <Text color="green">✅ Command executed successfully</Text>
                    }))

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
                outputContent: <Text color="red">❌ Error: {errorMessage}</Text>
            }))

            setTimeout(() => {
                setState((prev: ConsoleState) => prev.mode === 'command-output' ? {
                    ...prev,
                    mode: 'input',
                    outputContent: undefined
                } : prev)
            }, 3000)
        }
    }, [designEditor])

    const executeCommandWithParameters = useCallback(async (commandName: string) => {
        const command = commandRegistry.getAll().find(cmd =>
            cmd.name.toLowerCase() === commandName.toLowerCase() ||
            cmd.id.toLowerCase() === commandName.toLowerCase()
        )

        if (!command) {
            setState(prev => ({
                ...prev,
                mode: 'command-output',
                outputContent: <Text color="red">❌ Command not found: {commandName}</Text>
            }))
            return
        }

        if (command.parameters.length === 0) {
            await executeCommand(commandName, {})
        } else {
            setState(prev => ({
                ...prev,
                mode: 'parameter-gathering',
                currentCommand: command,
                parameterIndex: 0,
                gatheredParameters: {}
            }))
        }
    }, [executeCommand])

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
            executeCommand(state.currentCommand.name, newParameters)
            setState(prev => ({
                ...prev,
                mode: 'input',
                currentCommand: undefined,
                parameterIndex: 0,
                gatheredParameters: {}
            }))
        } else {
            setState(prev => ({
                ...prev,
                parameterIndex: prev.parameterIndex + 1,
                gatheredParameters: newParameters
            }))
        }
    }, [state, executeCommand])

    const renderParameterForm = () => {
        if (state.mode !== 'parameter-gathering' || !state.currentCommand) return null

        const parameter = state.currentCommand.parameters[state.parameterIndex]
        const FormComponent = parameterForms.get(parameter.type)

        if (!FormComponent) {
            return <Text color="red">❌ Unsupported parameter type: {parameter.type}</Text>
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
        <Box flexDirection="column" height="100%">
            <Box flexGrow={1}>
                <ConsoleCanvas content={upperContent} />
            </Box>
            <ConsoleInput
                state={state}
                onInputChange={handleInputChange}
                onSubmit={handleSubmit}
                onCancel={handleCancel}
            />
        </Box>
    )
}

export default Console