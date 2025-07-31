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

import { FC, useEffect, useRef, useState } from 'react'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
import 'xterm/css/xterm.css'

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

class TerminalForm {
    protected terminal: Terminal
    protected onSubmit: (value: any) => void
    protected onCancel: () => void
    protected parameter: CommandParameter
    protected cleanupHandlers: (() => void)[] = []

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void) {
        this.terminal = terminal
        this.parameter = parameter
        this.onSubmit = onSubmit
        this.onCancel = onCancel
    }

    protected writeColored(text: string, color?: string): void {
        const colorCodes: Record<string, string> = {
            gray: '\x1b[90m',
            blue: '\x1b[94m',
            green: '\x1b[92m',
            red: '\x1b[91m',
            yellow: '\x1b[93m',
            white: '\x1b[97m',
            reset: '\x1b[0m'
        }

        if (color && colorCodes[color]) {
            this.terminal.write(colorCodes[color] + text + colorCodes.reset)
        } else {
            this.terminal.write(text)
        }
    }

    protected clearForm(): void {
        this.terminal.write('\x1b[2J\x1b[H') // Clear screen and move to top
    }

    start(): void {
        this.clearForm()
        this.render()
        this.setupHandlers()
    }

    protected render(): void {
        // Override in subclasses
    }

    protected setupHandlers(): void {
        // Override in subclasses
    }

    cleanup(): void {
        this.cleanupHandlers.forEach(cleanup => cleanup())
        this.cleanupHandlers = []
    }
}

class TypeIdForm extends TerminalForm {
    private selectedIndex = 0
    private mode: 'type' | 'variant' = 'type'
    private selectedType = ''
    private filteredItems: string[] = []
    private searchTerm = ''
    private types: any[] = []
    private typeNames: string[] = []
    private variants: string[] = []

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, designEditor: any) {
        super(terminal, parameter, onSubmit, onCancel)
        this.types = designEditor.kit?.types || []
        this.typeNames = [...new Set(this.types.map(t => t.name))]
        this.updateFilteredItems()
    }

    private updateFilteredItems(): void {
        const currentItems = this.mode === 'type' ? this.typeNames : this.variants
        this.filteredItems = currentItems.filter(item =>
            item.toLowerCase().includes(this.searchTerm.toLowerCase())
        )
        this.selectedIndex = 0
    }

    private updateVariants(): void {
        this.variants = this.selectedType
            ? [...new Set(this.types.filter(t => t.name === this.selectedType).map(t => t.variant).filter(v => Boolean(v)))]
            : []
    }

    protected render(): void {
        this.clearForm()

        this.writeColored('┌─────────────────────────────────────┐\r\n', 'gray')
        const title = this.mode === 'type' ? '🔧 Select Type:' : `🎨 Select Variant for ${this.selectedType}:`
        this.writeColored(`│ ${title.padEnd(35)} │\r\n`, 'gray')
        this.writeColored('├─────────────────────────────────────┤\r\n', 'gray')

        this.writeColored(`│ Search: ${this.searchTerm}▋${' '.repeat(35 - this.searchTerm.length - 9)} │\r\n`, 'white')
        this.writeColored('├─────────────────────────────────────┤\r\n', 'gray')

        this.filteredItems.slice(0, 5).forEach((item, index) => {
            const prefix = index === this.selectedIndex ? '> ' : '  '
            const color = index === this.selectedIndex ? 'blue' : 'white'
            this.writeColored(`│ ${prefix}${item.padEnd(33)} │\r\n`, color)
        })

        for (let i = this.filteredItems.length; i < 5; i++) {
            this.writeColored(`│${' '.repeat(37)}│\r\n`, 'gray')
        }

        this.writeColored('├─────────────────────────────────────┤\r\n', 'gray')
        const helpText = this.mode === 'type'
            ? 'Use ↑↓ to navigate, Tab autocomplete, Enter select'
            : 'Use ↑↓ navigate, Tab autocomplete, Enter confirm'
        this.writeColored(`│ ${helpText.padEnd(35)} │\r\n`, 'gray')
        this.writeColored('└─────────────────────────────────────┘\r\n', 'gray')
    }

    protected setupHandlers(): void {
        const handler = (e: string) => {
            if (e === '\x1b') { // ESC
                this.onCancel()
                return
            }

            if (e === '\t') { // Tab
                if (this.searchTerm && this.filteredItems.length > 0) {
                    this.searchTerm = this.filteredItems[0]
                    this.updateFilteredItems()
                    this.render()
                }
                return
            }

            if (e === '\r') { // Enter
                if (this.mode === 'type') {
                    const typeToSelect = this.filteredItems[this.selectedIndex] || this.searchTerm
                    if (this.typeNames.includes(typeToSelect)) {
                        this.selectedType = typeToSelect
                        this.mode = 'variant'
                        this.searchTerm = ''
                        this.updateVariants()
                        this.updateFilteredItems()
                        this.render()
                    }
                } else {
                    const variantToSelect = this.filteredItems[this.selectedIndex] || this.searchTerm
                    this.onSubmit({ name: this.selectedType, variant: variantToSelect || undefined })
                }
                return
            }

            if (e === '\x1b[A') { // Up arrow
                this.selectedIndex = Math.max(0, this.selectedIndex - 1)
                this.render()
                return
            }

            if (e === '\x1b[B') { // Down arrow
                this.selectedIndex = Math.min(this.filteredItems.length - 1, this.selectedIndex + 1)
                this.render()
                return
            }

            if (e === '\x7f') { // Backspace
                this.searchTerm = this.searchTerm.slice(0, -1)
                this.updateFilteredItems()
                this.render()
                return
            }

            if (e.length === 1 && e >= ' ') { // Printable character
                this.searchTerm += e
                this.updateFilteredItems()
                this.render()
            }
        }

        this.terminal.onData(handler)
        this.cleanupHandlers.push(() => this.terminal.onData(() => { }))
    }
}

class StringForm extends TerminalForm {
    private inputValue = ''

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, initialValue?: any) {
        super(terminal, parameter, onSubmit, onCancel)
        this.inputValue = initialValue || parameter.defaultValue || ''
    }

    protected render(): void {
        this.clearForm()

        this.writeColored('┌─────────────────────────────────────┐\r\n', 'gray')
        this.writeColored(`│ 📝 ${this.parameter.description.padEnd(31)} │\r\n`, 'gray')
        this.writeColored('├─────────────────────────────────────┤\r\n', 'gray')
        this.writeColored(`│ ${this.inputValue}▋${' '.repeat(34 - this.inputValue.length)} │\r\n`, 'white')
        this.writeColored('├─────────────────────────────────────┤\r\n', 'gray')
        this.writeColored(`│ Enter to confirm, Esc to cancel     │\r\n`, 'gray')
        this.writeColored('└─────────────────────────────────────┘\r\n', 'gray')
    }

    protected setupHandlers(): void {
        const handler = (e: string) => {
            if (e === '\x1b') { // ESC
                this.onCancel()
                return
            }

            if (e === '\r') { // Enter
                this.onSubmit(this.inputValue)
                return
            }

            if (e === '\x7f') { // Backspace
                this.inputValue = this.inputValue.slice(0, -1)
                this.render()
                return
            }

            if (e.length === 1 && e >= ' ') { // Printable character
                this.inputValue += e
                this.render()
            }
        }

        this.terminal.onData(handler)
        this.cleanupHandlers.push(() => this.terminal.onData(() => { }))
    }
}

class SelectForm extends TerminalForm {
    private selectedIndex = 0
    private options: { value: string; label: string }[] = []

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void) {
        super(terminal, parameter, onSubmit, onCancel)
        this.options = parameter.options || []
    }

    protected render(): void {
        this.clearForm()

        this.writeColored('┌─────────────────────────────────────┐\r\n', 'gray')
        this.writeColored(`│ 📋 ${this.parameter.description.padEnd(31)} │\r\n`, 'gray')
        this.writeColored('├─────────────────────────────────────┤\r\n', 'gray')

        this.options.forEach((option, index) => {
            const prefix = index === this.selectedIndex ? '> ' : '  '
            const color = index === this.selectedIndex ? 'blue' : 'white'
            this.writeColored(`│ ${prefix}${option.label.padEnd(33)} │\r\n`, color)
        })

        for (let i = this.options.length; i < 6; i++) {
            this.writeColored(`│${' '.repeat(37)}│\r\n`, 'gray')
        }

        this.writeColored('├─────────────────────────────────────┤\r\n', 'gray')
        this.writeColored(`│ Use ↑↓ to navigate, Enter to select │\r\n`, 'gray')
        this.writeColored('└─────────────────────────────────────┘\r\n', 'gray')
    }

    protected setupHandlers(): void {
        const handler = (e: string) => {
            if (e === '\x1b') { // ESC
                this.onCancel()
                return
            }

            if (e === '\r') { // Enter
                this.onSubmit(this.options[this.selectedIndex]?.value)
                return
            }

            if (e === '\x1b[A') { // Up arrow
                this.selectedIndex = Math.max(0, this.selectedIndex - 1)
                this.render()
                return
            }

            if (e === '\x1b[B') { // Down arrow
                this.selectedIndex = Math.min(this.options.length - 1, this.selectedIndex + 1)
                this.render()
                return
            }
        }

        this.terminal.onData(handler)
        this.cleanupHandlers.push(() => this.terminal.onData(() => { }))
    }
}

class BooleanForm extends TerminalForm {
    private value = false

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void) {
        super(terminal, parameter, onSubmit, onCancel)
        this.value = parameter.defaultValue || false
    }

    protected render(): void {
        this.clearForm()

        this.writeColored('┌─────────────────────────────────────┐\r\n', 'gray')
        this.writeColored(`│ ❓ ${this.parameter.description.padEnd(31)} │\r\n`, 'gray')
        this.writeColored('├─────────────────────────────────────┤\r\n', 'gray')

        const valueText = this.value ? '✓ Yes' : '✗ No'
        const color = this.value ? 'green' : 'red'
        this.writeColored(`│ ${valueText.padEnd(35)} │\r\n`, color)

        this.writeColored('├─────────────────────────────────────┤\r\n', 'gray')
        this.writeColored(`│ Y/N, Space to toggle, Enter confirm │\r\n`, 'gray')
        this.writeColored('└─────────────────────────────────────┘\r\n', 'gray')
    }

    protected setupHandlers(): void {
        const handler = (e: string) => {
            if (e === '\x1b') { // ESC
                this.onCancel()
                return
            }

            if (e === '\r') { // Enter
                this.onSubmit(this.value)
                return
            }

            if (e === 'y' || e === 'Y') {
                this.value = true
                this.render()
                return
            }

            if (e === 'n' || e === 'N') {
                this.value = false
                this.render()
                return
            }

            if (e === ' ') {
                this.value = !this.value
                this.render()
                return
            }
        }

        this.terminal.onData(handler)
        this.cleanupHandlers.push(() => this.terminal.onData(() => { }))
    }
}

const createParameterForm = (
    terminal: Terminal,
    parameter: CommandParameter,
    onSubmit: (value: any) => void,
    onCancel: () => void,
    designEditor?: any,
    initialValue?: any
): TerminalForm | null => {
    switch (parameter.type) {
        case 'TypeId':
            return new TypeIdForm(terminal, parameter, onSubmit, onCancel, designEditor)
        case 'string':
            return new StringForm(terminal, parameter, onSubmit, onCancel, initialValue)
        case 'select':
            return new SelectForm(terminal, parameter, onSubmit, onCancel)
        case 'boolean':
            return new BooleanForm(terminal, parameter, onSubmit, onCancel)
        default:
            return null
    }
}

//#endregion Forms

class TerminalConsole {
    private terminal: Terminal
    private fitAddon: FitAddon
    private state: ConsoleState
    private currentForm: TerminalForm | null = null
    private designEditor: any
    private onStateChange: (state: ConsoleState) => void

    constructor(
        element: HTMLElement,
        designEditor: any,
        initialState: ConsoleState,
        onStateChange: (state: ConsoleState) => void
    ) {
        this.terminal = new Terminal({
            cursorBlink: true,
            fontSize: 14,
            fontFamily: 'JetBrains Mono, Monaco, Consolas, monospace',
            theme: {
                background: '#1a1a1a',
                foreground: '#ffffff',
                cursor: '#ffffff',
                selection: '#333333'
            }
        })

        this.fitAddon = new FitAddon()
        this.terminal.loadAddon(this.fitAddon)
        this.terminal.open(element)
        this.fitAddon.fit()

        this.designEditor = designEditor
        this.state = initialState
        this.onStateChange = onStateChange

        this.setupTerminalHandlers()
        this.showWelcome()
        this.updatePrompt()
    }

    private showWelcome(): void {
        this.terminal.writeln('\x1b[90m🎉 Welcome to Semio Console! Type "help" for available commands.\x1b[0m')
        this.terminal.writeln('')
    }

    private writeColored(text: string, color?: string): void {
        const colorCodes: Record<string, string> = {
            gray: '\x1b[90m',
            blue: '\x1b[94m',
            green: '\x1b[92m',
            red: '\x1b[91m',
            yellow: '\x1b[93m',
            white: '\x1b[97m',
            reset: '\x1b[0m'
        }

        if (color && colorCodes[color]) {
            this.terminal.write(colorCodes[color] + text + colorCodes.reset)
        } else {
            this.terminal.write(text)
        }
    }

    private updatePrompt(): void {
        if (this.state.mode === 'input') {
            this.terminal.write('\r\x1b[K') // Clear line

            if (this.state.suggestions.length > 0) {
                this.writeColored('Suggestions: ', 'yellow')
                this.state.suggestions.slice(0, 5).forEach((suggestion, index) => {
                    const color = index === this.state.selectedSuggestion ? 'blue' : 'gray'
                    this.writeColored(' ' + suggestion, color)
                })
                this.terminal.write('\r\n')
            }

            this.writeColored('$ ', 'blue')
            this.terminal.write(this.state.input)

            if (this.state.mode === 'input') {
                this.terminal.write('▋')
            }
        }
    }

    private setupTerminalHandlers(): void {
        this.terminal.onData((data: string) => {
            if (this.state.mode === 'parameter-gathering') {
                return // Let form handle input
            }

            if (data === '\x1b') { // ESC
                if (this.state.mode === 'command-output') {
                    this.handleCancel()
                }
                return
            }

            if (data === '\r') { // Enter
                if (this.state.mode === 'command-output') {
                    this.handleCancel()
                } else {
                    this.handleSubmit()
                }
                return
            }

            if (data === '\x1b[A' && this.state.commandHistory.length > 0) { // Up arrow
                const newIndex = Math.min(this.state.historyIndex + 1, this.state.commandHistory.length - 1)
                if (newIndex !== this.state.historyIndex) {
                    this.updateState({
                        ...this.state,
                        input: this.state.commandHistory[this.state.commandHistory.length - 1 - newIndex],
                        historyIndex: newIndex
                    })
                }
                return
            }

            if (data === '\x1b[B' && this.state.historyIndex > -1) { // Down arrow
                const newIndex = this.state.historyIndex - 1
                if (newIndex === -1) {
                    this.updateState({ ...this.state, input: '', historyIndex: -1 })
                } else {
                    this.updateState({
                        ...this.state,
                        input: this.state.commandHistory[this.state.commandHistory.length - 1 - newIndex],
                        historyIndex: newIndex
                    })
                }
                return
            }

            if (data === '\t' && this.state.suggestions.length > 0) { // Tab
                this.updateState({
                    ...this.state,
                    input: this.state.suggestions[this.state.selectedSuggestion]
                })
                return
            }

            if (this.state.mode === 'input') {
                if (data === '\x7f') { // Backspace
                    this.updateState({
                        ...this.state,
                        input: this.state.input.slice(0, -1),
                        historyIndex: -1
                    })
                } else if (data.length === 1 && data >= ' ') { // Printable character
                    this.updateState({
                        ...this.state,
                        input: this.state.input + data,
                        historyIndex: -1
                    })
                }
            }
        })
    }

    private updateState(newState: ConsoleState): void {
        this.state = newState
        this.onStateChange(newState)
        this.updateSuggestions()
        this.updatePrompt()
    }

    private updateSuggestions(): void {
        if (!this.state.input.trim()) {
            this.updateState({ ...this.state, suggestions: [], selectedSuggestion: 0 })
            return
        }

        const commands = commandRegistry.search(this.state.input)
        const suggestions = commands.map(cmd => cmd.name)

        this.updateState({
            ...this.state,
            suggestions,
            selectedSuggestion: Math.min(this.state.selectedSuggestion, suggestions.length - 1)
        })
    }

    private handleSubmit(): void {
        if (this.state.mode === 'input') {
            const command = this.state.input.trim()
            if (command) {
                this.terminal.write('\r\n')
                this.updateState({
                    ...this.state,
                    commandHistory: [command, ...this.state.commandHistory.slice(0, 99)],
                    input: ''
                })
                this.executeCommandWithParameters(command)
            }
        }
    }

    private handleCancel(): void {
        if (this.currentForm) {
            this.currentForm.cleanup()
            this.currentForm = null
        }

        this.updateState({
            ...this.state,
            mode: 'input',
            input: '',
            currentCommand: undefined,
            parameterIndex: 0,
            gatheredParameters: {},
            outputContent: undefined
        })

        this.terminal.write('\r\n')
        this.updatePrompt()
    }

    private async executeCommandWithParameters(commandName: string): Promise<void> {
        const command = commandRegistry.getAll().find(cmd =>
            cmd.name.toLowerCase() === commandName.toLowerCase() ||
            cmd.id.toLowerCase() === commandName.toLowerCase()
        )

        if (!command) {
            this.writeColored(`❌ Command not found: ${commandName}\r\n`, 'red')
            setTimeout(() => this.updatePrompt(), 2000)
            return
        }

        if (command.parameters.length === 0) {
            await this.executeCommand(commandName, {})
        } else {
            this.updateState({
                ...this.state,
                mode: 'parameter-gathering',
                currentCommand: command,
                parameterIndex: 0,
                gatheredParameters: {}
            })
            this.showParameterForm()
        }
    }

    private showParameterForm(): void {
        if (!this.state.currentCommand) return

        const parameter = this.state.currentCommand.parameters[this.state.parameterIndex]

        const onSubmit = (value: any) => {
            this.handleParameterSubmit(value)
        }

        const onCancel = () => {
            this.handleCancel()
        }

        this.currentForm = createParameterForm(
            this.terminal,
            parameter,
            onSubmit,
            onCancel,
            this.designEditor,
            this.state.gatheredParameters[parameter.name]
        )

        if (this.currentForm) {
            this.currentForm.start()
        } else {
            this.writeColored(`❌ Unsupported parameter type: ${parameter.type}\r\n`, 'red')
            this.handleCancel()
        }
    }

    private handleParameterSubmit(value: any): void {
        if (!this.state.currentCommand) return

        const newParameters = {
            ...this.state.gatheredParameters,
            [this.state.currentCommand.parameters[this.state.parameterIndex].name]: value
        }

        if (this.currentForm) {
            this.currentForm.cleanup()
            this.currentForm = null
        }

        if (this.state.parameterIndex >= this.state.currentCommand.parameters.length - 1) {
            this.executeCommand(this.state.currentCommand.name, newParameters)
            this.updateState({
                ...this.state,
                mode: 'input',
                currentCommand: undefined,
                parameterIndex: 0,
                gatheredParameters: {}
            })
        } else {
            this.updateState({
                ...this.state,
                parameterIndex: this.state.parameterIndex + 1,
                gatheredParameters: newParameters
            })
            this.showParameterForm()
        }
    }

    private async executeCommand(commandName: string, payload: Record<string, any> = {}): Promise<void> {
        const command = commandRegistry.getAll().find(cmd =>
            cmd.name.toLowerCase() === commandName.toLowerCase() ||
            cmd.id.toLowerCase() === commandName.toLowerCase()
        )

        if (!command) {
            this.writeColored(`❌ Command not found: ${commandName}\r\n`, 'red')
            setTimeout(() => this.updatePrompt(), 2000)
            return
        }

        try {
            const context = {
                kit: this.designEditor.kit!,
                designId: this.designEditor.designId,
                selection: this.designEditor.selection
            }

            if (command.editorOnly) {
                const result = await command.execute(context, payload)

                if (result.selection) {
                    this.designEditor.setSelection(result.selection)
                }

                if (result.content) {
                    this.renderReactContent(result.content)
                } else {
                    this.writeColored('✅ Command executed successfully\r\n', 'green')
                }
            } else {
                this.designEditor.startTransaction()

                try {
                    const result = await command.execute(context, payload)

                    if (result.design) {
                        this.designEditor.setDesign(result.design)
                    }
                    if (result.selection) {
                        this.designEditor.setSelection(result.selection)
                    }

                    this.designEditor.finalizeTransaction()

                    if (result.content) {
                        this.renderReactContent(result.content)
                    } else {
                        this.writeColored('✅ Command executed successfully\r\n', 'green')
                    }

                    if (command.id !== 'clear') {
                        setTimeout(() => this.updatePrompt(), 2000)
                    }
                } catch (error) {
                    this.designEditor.abortTransaction()
                    throw error
                }
            }
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred'
            this.writeColored(`❌ Error: ${errorMessage}\r\n`, 'red')
            setTimeout(() => this.updatePrompt(), 3000)
        }
    }

    private renderReactContent(content: React.ReactNode): void {
        // For now, just extract text content from React components
        // In a full implementation, you might want to render components to text
        if (typeof content === 'string') {
            this.terminal.write(content + '\r\n')
        } else if (content && typeof content === 'object' && 'props' in content) {
            // Extract text from React element
            const extractText = (node: any): string => {
                if (typeof node === 'string') return node
                if (typeof node === 'number') return String(node)
                if (Array.isArray(node)) return node.map(extractText).join('')
                if (node && typeof node === 'object' && node.props) {
                    if (node.props.children) {
                        return extractText(node.props.children)
                    }
                }
                return ''
            }

            this.terminal.write(extractText(content) + '\r\n')
        }

        setTimeout(() => this.updatePrompt(), 2000)
    }

    public setState(newState: ConsoleState): void {
        this.state = newState
        this.updatePrompt()
    }

    public resize(): void {
        this.fitAddon.fit()
    }

    public dispose(): void {
        if (this.currentForm) {
            this.currentForm.cleanup()
        }
        this.terminal.dispose()
    }
}

const Console: FC = () => {
    const designEditor = useDesignEditor()
    const terminalRef = useRef<HTMLDivElement>(null)
    const terminalConsoleRef = useRef<TerminalConsole | null>(null)
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

    useEffect(() => {
        if (terminalRef.current && !terminalConsoleRef.current) {
            terminalConsoleRef.current = new TerminalConsole(
                terminalRef.current,
                designEditor,
                state,
                setState
            )
        }

        return () => {
            if (terminalConsoleRef.current) {
                terminalConsoleRef.current.dispose()
                terminalConsoleRef.current = null
            }
        }
    }, [designEditor])

    useEffect(() => {
        if (terminalConsoleRef.current) {
            terminalConsoleRef.current.setState(state)
        }
    }, [state])

    useEffect(() => {
        const handleResize = () => {
            if (terminalConsoleRef.current) {
                terminalConsoleRef.current.resize()
            }
        }

        window.addEventListener('resize', handleResize)
        return () => window.removeEventListener('resize', handleResize)
    }, [])

    return (
        <div className="h-full w-full">
            <div
                ref={terminalRef}
                className="h-full w-full bg-background font-mono"
                style={{ minHeight: '400px' }}
            />
        </div>
    )
}

export default Console