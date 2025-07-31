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

import React, { FC, useEffect, useRef, useState } from 'react'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
import 'xterm/css/xterm.css'

import {
    addConnectionToDesign,
    Design,
    DesignId,
    findDesignInKit,
    flattenDesign,
    Kit,
    removePiecesAndConnectionsFromDesign,
    setPieceInDesign,
    useDesignEditor
} from '@semio/js'
import { orientDesign } from '../../semio'

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
        const allCommands = this.getAll()

        if (!query.trim()) {
            return allCommands // Return all commands if query is empty
        }

        const lowercaseQuery = query.toLowerCase()
        const filtered = allCommands.filter(cmd =>
            cmd.name.toLowerCase().includes(lowercaseQuery) ||
            cmd.description.toLowerCase().includes(lowercaseQuery) ||
            cmd.id.toLowerCase().includes(lowercaseQuery)
        )

        return filtered
    }

    async execute(commandId: string, context: CommandContext, payload: Record<string, any> = {}): Promise<CommandResult> {
        const command = this.get(commandId)
        if (!command) throw new Error(`Command '${commandId}' not found`)
        return command.execute(context, payload)
    }
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

interface ConsolePanelProps {
    visible: boolean
    leftPanelVisible: boolean
    rightPanelVisible: boolean
    leftPanelWidth?: number
    rightPanelWidth?: number
    height: number
    setHeight: (height: number) => void
}

//#region Forms

class TerminalForm {
    protected terminal: Terminal
    protected onSubmit: (value: any) => void
    protected onCancel: () => void
    protected onBack: () => void
    protected parameter: CommandParameter
    protected cleanupHandlers: (() => void)[] = []

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void) {
        this.terminal = terminal
        this.parameter = parameter
        this.onSubmit = onSubmit
        this.onCancel = onCancel
        this.onBack = onBack
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
        this.terminal.clear() // Clear screen but don't move cursor to top
    }

    start(): void {
        this.clearForm()
        // Position form content above the input line
        const terminalRows = this.terminal.rows
        const inputRow = terminalRows
        this.terminal.write(`\x1b[${Math.max(1, inputRow - 5)};1H`) // Position form above input
        this.render()
        this.setupHandlers()
    }

    protected render(): void {
        // Override in subclasses - don't clear here as positioning is already set
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

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void, designEditor: any) {
        super(terminal, parameter, onSubmit, onCancel, onBack)
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
        // Don't clear form here - position is already set in start()
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
                if (this.searchTerm.length > 0) {
                    // If there's a search term, delete from it
                    this.searchTerm = this.searchTerm.slice(0, -1)
                    this.updateFilteredItems()
                    this.render()
                } else {
                    // If no search term, go back to previous parameter
                    this.onBack()
                }
                return
            }

            if (e.length === 1 && e >= ' ') { // Printable character
                this.searchTerm += e
                this.updateFilteredItems()
                this.render()
            }
        }

        // Remove any existing handlers first
        this.cleanupHandlers.forEach(cleanup => cleanup())
        this.cleanupHandlers = []

        this.terminal.onData(handler)
        this.cleanupHandlers.push(() => this.terminal.onData(() => { }))
    }
}

class StringForm extends TerminalForm {
    private inputValue = ''

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void, initialValue?: any) {
        super(terminal, parameter, onSubmit, onCancel, onBack)
        this.inputValue = initialValue || parameter.defaultValue || ''
    }

    protected render(): void {
        // Don't clear form here - position is already set in start()
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
                if (this.inputValue.length > 0) {
                    // If there's input, delete from it
                    this.inputValue = this.inputValue.slice(0, -1)
                    this.render()
                } else {
                    // If no input, go back to previous parameter
                    this.onBack()
                }
                return
            }

            if (e.length === 1 && e >= ' ') { // Printable character
                this.inputValue += e
                this.render()
            }
        }

        // Remove any existing handlers first
        this.cleanupHandlers.forEach(cleanup => cleanup())
        this.cleanupHandlers = []

        this.terminal.onData(handler)
        this.cleanupHandlers.push(() => this.terminal.onData(() => { }))
    }
}

class SelectForm extends TerminalForm {
    private selectedIndex = 0
    private options: { value: string; label: string }[] = []

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void) {
        super(terminal, parameter, onSubmit, onCancel, onBack)
        this.options = parameter.options || []
    }

    protected render(): void {
        // Don't clear form here - position is already set in start()
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

            if (e === '\x7f') { // Backspace
                this.onBack()
                return
            }
        }

        // Remove any existing handlers first
        this.cleanupHandlers.forEach(cleanup => cleanup())
        this.cleanupHandlers = []

        this.terminal.onData(handler)
        this.cleanupHandlers.push(() => this.terminal.onData(() => { }))
    }
}

class NumberForm extends TerminalForm {
    private inputValue = ''

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void, initialValue?: any) {
        super(terminal, parameter, onSubmit, onCancel, onBack)
        this.inputValue = String(initialValue || parameter.defaultValue || '0')
    }

    protected render(): void {
        // Don't clear form here - position is already set in start()
        this.writeColored('┌─────────────────────────────────────┐\r\n', 'gray')
        this.writeColored(`│ 🔢 ${this.parameter.description.padEnd(31)} │\r\n`, 'gray')
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
                const value = parseFloat(this.inputValue)
                if (!isNaN(value)) {
                    this.onSubmit(value)
                } else {
                    this.onSubmit(0)
                }
                return
            }

            if (e === '\x7f') { // Backspace
                if (this.inputValue.length > 0) {
                    // If there's input, delete from it
                    this.inputValue = this.inputValue.slice(0, -1)
                    this.render()
                } else {
                    // If no input, go back to previous parameter
                    this.onBack()
                }
                return
            }

            if (e.length === 1 && (e >= '0' && e <= '9' || e === '.' || e === '-' || e === '+')) {
                this.inputValue += e
                this.render()
            }
        }

        // Remove any existing handlers first
        this.cleanupHandlers.forEach(cleanup => cleanup())
        this.cleanupHandlers = []

        this.terminal.onData(handler)
        this.cleanupHandlers.push(() => this.terminal.onData(() => { }))
    }
}

class BooleanForm extends TerminalForm {
    private value = false

    constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void) {
        super(terminal, parameter, onSubmit, onCancel, onBack)
        this.value = parameter.defaultValue || false
    }

    protected render(): void {
        // Don't clear form here - position is already set in start()
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

            if (e === '\x7f') { // Backspace
                this.onBack()
                return
            }
        }

        // Remove any existing handlers first
        this.cleanupHandlers.forEach(cleanup => cleanup())
        this.cleanupHandlers = []

        this.terminal.onData(handler)
        this.cleanupHandlers.push(() => this.terminal.onData(() => { }))
    }
}

const createParameterForm = (
    terminal: Terminal,
    parameter: CommandParameter,
    onSubmit: (value: any) => void,
    onCancel: () => void,
    onBack: () => void,
    designEditor?: any,
    initialValue?: any
): TerminalForm | null => {
    switch (parameter.type) {
        case 'TypeId':
            return new TypeIdForm(terminal, parameter, onSubmit, onCancel, onBack, designEditor)
        case 'string':
            return new StringForm(terminal, parameter, onSubmit, onCancel, onBack, initialValue)
        case 'number':
            return new NumberForm(terminal, parameter, onSubmit, onCancel, onBack, initialValue)
        case 'select':
            return new SelectForm(terminal, parameter, onSubmit, onCancel, onBack)
        case 'boolean':
            return new BooleanForm(terminal, parameter, onSubmit, onCancel, onBack)
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
            fontSize: 14,
            fontFamily: '"Anta", "Noto Emoji"',
            cursorBlink: false, // Don't change this
            cursorStyle: 'block', // Don't change this
            theme: {
                background: '#d3d2c5', // Don't change this
                foreground: '#001117',
                cursor: '#001117',
                cursorAccent: '#ff344f',
                selection: 'rgba(255, 52, 79, 0.3)',
                black: '#001117',
                red: '#a60009',
                green: '#7eb77f',
                yellow: '#fa9500',
                blue: '#34d1bf',
                magenta: '#ff344f',
                cyan: '#dbbea1',
                white: '#f7f3e3',
                brightBlack: '#7b827d',
                brightRed: '#a60009',
                brightGreen: '#7eb77f',
                brightYellow: '#fa9500',
                brightBlue: '#34d1bf',
                brightMagenta: '#ff344f',
                brightCyan: '#dbbea1',
                brightWhite: '#001117'
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

        // Initialize suggestions for empty input
        this.updateState({
            ...this.state,
            mode: 'input'
        })

        // Ensure terminal has focus
        this.terminal.focus()
    }

    private showWelcome(): void {
        // Don't show welcome message as it will be cleared anyway
        this.updatePrompt()
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
            // Clear the entire terminal to prevent history buildup
            this.terminal.clear()

            // Calculate available space for content
            const terminalRows = this.terminal.rows
            const inputRow = terminalRows
            let usedRows = 1 // Reserve one row for input

            // Only show suggestions when typing and not navigating history
            const showSuggestions = this.state.input.length > 0 &&
                this.state.suggestions.length > 0 &&
                this.state.historyIndex === -1

            if (showSuggestions) {
                const maxSuggestions = Math.min(5, this.state.suggestions.length)
                const suggestionRows = maxSuggestions + 2 // +2 for header and spacing
                usedRows += suggestionRows

                // Position suggestions above the input
                const suggestionStartRow = Math.max(1, inputRow - usedRows + 1)
                this.terminal.write(`\x1b[${suggestionStartRow};1H`)

                this.writeColored('Available commands:\r\n', 'blue')
                this.state.suggestions.slice(0, maxSuggestions).forEach((suggestion, index) => {
                    const isSelected = index === this.state.selectedSuggestion
                    const prefix = isSelected ? '▶ ' : '  '
                    const color = isSelected ? 'magenta' : 'gray'
                    this.writeColored(`${prefix}${suggestion}\r\n`, color)
                })
                this.terminal.write('\r\n')
            }

            // Position cursor at the bottom row for input
            this.terminal.write(`\x1b[${inputRow};1H`)
            // Clear the line and write the input
            this.terminal.write('\x1b[K') // Clear to end of line
            this.terminal.write(this.state.input)

            // Ensure terminal maintains focus after prompt update
            setTimeout(() => this.terminal.focus(), 0)
        }
    }

    private setupTerminalHandlers(): void {
        this.terminal.onData((data: string) => {
            if (data === '\x03') { // Ctrl+C - handle this even during parameter gathering
                this.handleCancel()
                return
            }

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
                } else if (this.state.input.length > 0 &&
                    this.state.suggestions.length > 0 &&
                    this.state.selectedSuggestion >= 0 &&
                    this.state.historyIndex === -1) {
                    // Execute selected suggestion only when not in history mode
                    this.updateState({
                        ...this.state,
                        input: this.state.suggestions[this.state.selectedSuggestion],
                        historyIndex: -1
                    })
                    // Small delay to let the state update, then submit
                    setTimeout(() => this.handleSubmit(), 10)
                } else {
                    this.handleSubmit()
                }
                return
            }

            if (data === '\x1b[A') { // Up arrow
                // Check if we're in history navigation mode (empty input or already navigating history)
                if (this.state.historyIndex > -1) {
                    // Already in history mode - navigate further back
                    const newIndex = Math.min(this.state.historyIndex + 1, this.state.commandHistory.length - 1)
                    if (newIndex !== this.state.historyIndex) {
                        this.updateState({
                            ...this.state,
                            input: this.state.commandHistory[this.state.commandHistory.length - 1 - newIndex],
                            historyIndex: newIndex
                        })
                    }
                } else if (this.state.input.length === 0 && this.state.commandHistory.length > 0) {
                    // Start history navigation from empty input
                    this.updateState({
                        ...this.state,
                        input: this.state.commandHistory[this.state.commandHistory.length - 1],
                        historyIndex: 0
                    })
                } else if (this.state.input.length > 0 && this.state.suggestions.length > 0) {
                    // Navigate suggestions only when typing and not in history mode
                    const newIndex = Math.max(0, this.state.selectedSuggestion - 1)
                    this.updateState({
                        ...this.state,
                        selectedSuggestion: newIndex
                    })
                }
                return
            }

            if (data === '\x1b[B') { // Down arrow
                // Check if we're in history navigation mode
                if (this.state.historyIndex > -1) {
                    // Navigate command history
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
                } else if (this.state.input.length > 0 && this.state.suggestions.length > 0) {
                    // Navigate suggestions only when typing and not in history mode
                    const newIndex = Math.min(this.state.suggestions.length - 1, this.state.selectedSuggestion + 1)
                    this.updateState({
                        ...this.state,
                        selectedSuggestion: newIndex
                    })
                }
                return
            }

            if (data === '\t' && this.state.input.length > 0 && this.state.suggestions.length > 0 && this.state.historyIndex === -1) { // Tab
                this.updateState({
                    ...this.state,
                    input: this.state.suggestions[this.state.selectedSuggestion],
                    historyIndex: -1
                })
                return
            }

            if (this.state.mode === 'input') {
                if (data === '\x7f') { // Backspace
                    this.updateState({
                        ...this.state,
                        input: this.state.input.slice(0, -1),
                        historyIndex: -1,
                        selectedSuggestion: 0
                    })
                } else if (data.length === 1 && data >= ' ') { // Printable character
                    this.updateState({
                        ...this.state,
                        input: this.state.input + data,
                        historyIndex: -1,
                        selectedSuggestion: 0
                    })
                }
            }
        })
    }

    private updateState(newState: ConsoleState): void {
        // Update suggestions without causing infinite loop
        if (newState.mode === 'input') {
            const commands = commandRegistry.search(newState.input)
            const suggestions = commands.map(cmd => cmd.name)

            // Update state with new suggestions
            newState = {
                ...newState,
                suggestions,
                selectedSuggestion: Math.min(Math.max(0, newState.selectedSuggestion), Math.max(0, suggestions.length - 1))
            }
        }

        this.state = newState
        this.updatePrompt()
    }

    private notifyStateChange(): void {
        this.onStateChange(this.state)
    }

    public updateDesignEditor(designEditor: any): void {
        this.designEditor = designEditor
    }

    private updateSuggestions(): void {
        // This method is now handled in updateState to prevent infinite loops
        // Keep for backward compatibility if needed
    }

    private handleSubmit(): void {
        if (this.state.mode === 'input') {
            const command = this.state.input.trim()
            if (command) {
                this.updateState({
                    ...this.state,
                    input: '',
                    historyIndex: -1
                })
                this.notifyStateChange()
                this.executeCommandWithParameters(command)
            }
        }
    }

    private handleCancel(): void {
        if (this.currentForm) {
            this.currentForm.cleanup()
            this.currentForm = null
        }

        // Show cancellation message if there was something to cancel
        const hadActiveOperation = this.state.mode !== 'input' || this.state.input.length > 0
        const wasGatheringParameters = this.state.mode === 'parameter-gathering'

        // Clear the screen and start fresh
        this.terminal.clear()

        // Don't show cancellation message when returning from command output
        if (hadActiveOperation && this.state.mode !== 'command-output') {
            const message = wasGatheringParameters ? '❌ Command cancelled' : '❌ Operation cancelled'
            this.writeColored(`${message}\r\n\r\n`, 'red')
        }

        this.updateState({
            ...this.state,
            mode: 'input',
            input: '',
            currentCommand: undefined,
            parameterIndex: 0,
            gatheredParameters: {},
            outputContent: undefined,
            historyIndex: -1
        })
        this.notifyStateChange()

        // Force cursor to bottom after state update
        setTimeout(() => {
            this.updatePrompt()
            this.terminal.focus()
        }, 10)
    }

    private async executeCommandWithParameters(commandName: string): Promise<void> {
        const command = commandRegistry.getAll().find(cmd =>
            cmd.name.toLowerCase() === commandName.toLowerCase() ||
            cmd.id.toLowerCase() === commandName.toLowerCase()
        )

        if (!command) {
            this.terminal.clear()
            this.writeColored(`❌ Command not found: ${commandName}\r\n`, 'red')
            this.updateState({
                ...this.state,
                mode: 'command-output'
            })
            this.notifyStateChange()
            setTimeout(() => {
                this.terminal.write('\r\n\x1b[90mPress Enter or Escape to continue...\x1b[0m')
            }, 1000)
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
            this.notifyStateChange()
            this.showParameterForm()
        }
    }

    private showParameterForm(): void {
        if (!this.state.currentCommand) return

        // Clear the terminal first
        this.terminal.clear()

        const parameter = this.state.currentCommand.parameters[this.state.parameterIndex]

        const onSubmit = (value: any) => {
            this.handleParameterSubmit(value)
        }

        const onCancel = () => {
            this.handleCancel()
        }

        const onBack = () => {
            this.handleParameterBack()
        }

        this.currentForm = createParameterForm(
            this.terminal,
            parameter,
            onSubmit,
            onCancel,
            onBack,
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
            this.notifyStateChange()
        } else {
            this.updateState({
                ...this.state,
                parameterIndex: this.state.parameterIndex + 1,
                gatheredParameters: newParameters
            })
            this.notifyStateChange()
            this.showParameterForm()
        }
    }

    private handleParameterBack(): void {
        if (!this.state.currentCommand) return

        if (this.currentForm) {
            this.currentForm.cleanup()
            this.currentForm = null
        }

        if (this.state.parameterIndex > 0) {
            // Go back to previous parameter
            this.updateState({
                ...this.state,
                parameterIndex: this.state.parameterIndex - 1
            })
            this.notifyStateChange()
            this.showParameterForm()
        } else {
            // Go back to command input
            this.handleCancel()
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

        // Add command to history only when actually executing (not during parameter gathering)
        this.updateState({
            ...this.state,
            commandHistory: [commandName, ...this.state.commandHistory.slice(0, 99)]
        })

        try {
            const context = {
                kit: this.designEditor.kit || { types: [], designs: [] },
                designId: this.designEditor.designId || '',
                selection: this.designEditor.selection || { selectedPieceIds: [], selectedConnections: [], selectedPiecePortId: undefined }
            }

            if (command.editorOnly) {
                const result = await command.execute(context, payload)

                if (result.selection && this.designEditor.setSelection) {
                    this.designEditor.setSelection(result.selection)
                }

                if (result.content) {
                    this.renderReactContent(result.content)
                } else {
                    this.terminal.clear()
                    this.writeColored('✅ Command executed successfully\r\n', 'green')
                    this.updateState({
                        ...this.state,
                        mode: 'command-output'
                    })
                    this.notifyStateChange()
                    setTimeout(() => {
                        this.terminal.write('\r\n\x1b[90mPress Enter or Escape to continue...\x1b[0m')
                    }, 500)
                }
            } else {
                if (this.designEditor.startTransaction) {
                    this.designEditor.startTransaction()
                }

                try {
                    const result = await command.execute(context, payload)

                    if (result.design && this.designEditor.setDesign) {
                        this.designEditor.setDesign(result.design)
                    }
                    if (result.selection && this.designEditor.setSelection) {
                        this.designEditor.setSelection(result.selection)
                    }

                    if (this.designEditor.finalizeTransaction) {
                        this.designEditor.finalizeTransaction()
                    }

                    if (result.content) {
                        this.renderReactContent(result.content)
                    } else {
                        this.terminal.clear()
                        this.writeColored('✅ Command executed successfully\r\n', 'green')
                        this.updateState({
                            ...this.state,
                            mode: 'command-output'
                        })
                        this.notifyStateChange()
                        setTimeout(() => {
                            this.terminal.write('\r\n\x1b[90mPress Enter or Escape to continue...\x1b[0m')
                        }, 500)
                    }

                    // Don't automatically return to prompt - wait for user input
                } catch (error) {
                    if (this.designEditor.abortTransaction) {
                        this.designEditor.abortTransaction()
                    }
                    throw error
                }
            }
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred'
            this.terminal.clear()
            this.writeColored(`❌ Error: ${errorMessage}\r\n`, 'red')
            this.updateState({
                ...this.state,
                mode: 'command-output'
            })
            this.notifyStateChange()
            setTimeout(() => {
                this.terminal.write('\r\n\x1b[90mPress Enter or Escape to continue...\x1b[0m')
            }, 1000)
        }
    }

    private renderReactContent(content: React.ReactNode): void {
        // Handle clear command specially
        if (content === null) {
            this.terminal.clear()
            this.updatePrompt()
            return
        }

        // Clear screen to show only the output
        this.terminal.clear()

        // For now, just extract text content from React components
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

        // Update state to show command output mode with delay for user to read
        this.updateState({
            ...this.state,
            mode: 'command-output',
            outputContent: content
        })
        this.notifyStateChange()

        // Show prompt to continue after 1 second
        setTimeout(() => {
            this.terminal.write('\r\n\x1b[90mPress Enter or Escape to continue...\x1b[0m')
        }, 1000)
    }

    public resize(): void {
        this.fitAddon.fit()
        // Update prompt after resize to ensure proper positioning
        setTimeout(() => {
            if (this.state.mode === 'input') {
                this.updatePrompt()
            }
        }, 50)
    } public focus(): void {
        this.terminal.focus()
        // Change cursor to primary color when focused
        this.updateCursorColor('#ff344f')
    }

    public blur(): void {
        // Change cursor to foreground color when not focused  
        this.updateCursorColor('#001117')
    }

    private updateCursorColor(color: string): void {
        this.terminal.options.theme = {
            ...this.terminal.options.theme,
            cursor: color
        }
    }

    public dispose(): void {
        if (this.currentForm) this.currentForm.cleanup()
        this.terminal.dispose()
    }
}

const Console: FC = () => {
    const designEditor = useDesignEditor()
    const terminalRef = useRef<HTMLDivElement>(null)
    const terminalConsoleRef = useRef<TerminalConsole | null>(null)
    const [state, setState] = useState<ConsoleState>(() => {
        // Initialize with empty suggestions first, they'll be populated when terminal is ready
        return {
            mode: 'input',
            input: '',
            suggestions: [],
            selectedSuggestion: 0,
            parameterIndex: 0,
            gatheredParameters: {},
            commandHistory: [],
            historyIndex: -1
        }
    })

    useEffect(() => {
        if (terminalRef.current && !terminalConsoleRef.current) {
            terminalConsoleRef.current = new TerminalConsole(
                terminalRef.current,
                designEditor,
                state,
                setState
            )

            // Ensure focus after initial setup
            setTimeout(() => {
                if (terminalConsoleRef.current) {
                    terminalConsoleRef.current.focus()
                }
            }, 100)
        }

        return () => {
            if (terminalConsoleRef.current) {
                terminalConsoleRef.current.dispose()
                terminalConsoleRef.current = null
            }
        }
    }, []) // Remove designEditor dependency to prevent recreating terminal

    // Update design editor reference when it changes
    useEffect(() => {
        if (terminalConsoleRef.current) {
            terminalConsoleRef.current.updateDesignEditor(designEditor)
        }
    }, [designEditor])

    // Remove the problematic useEffect that causes infinite loops
    // The terminal console manages its own state through onStateChange callback

    // Inject CSS for WebKit scrollbar styling
    useEffect(() => {
        const style = document.createElement('style')
        style.textContent = `
            .console-terminal::-webkit-scrollbar {
                width: 8px;
                height: 8px;
            }
            
            .console-terminal::-webkit-scrollbar-track {
                background: transparent;
            }
            
            .console-terminal::-webkit-scrollbar-thumb {
                background: transparent;
                border-radius: 4px;
            }
            
            .console-terminal:hover::-webkit-scrollbar-thumb {
                background: #7b827d;
            }
            
            .console-terminal::-webkit-scrollbar-thumb:hover {
                background: #001117 !important;
            }
        `
        document.head.appendChild(style)

        return () => {
            document.head.removeChild(style)
        }
    }, [])

    useEffect(() => {
        const handleResize = () => {
            if (terminalConsoleRef.current) {
                terminalConsoleRef.current.resize()
            }
        }

        const handleClick = () => {
            // Ensure terminal regains focus when clicked
            if (terminalConsoleRef.current) {
                terminalConsoleRef.current.focus()
            }
        }

        let resizeObserver: ResizeObserver | null = null

        if (terminalRef.current) {
            // Watch for container size changes
            resizeObserver = new ResizeObserver(() => {
                handleResize()
            })
            resizeObserver.observe(terminalRef.current)
            terminalRef.current.addEventListener('click', handleClick)
        }

        window.addEventListener('resize', handleResize)

        return () => {
            window.removeEventListener('resize', handleResize)
            if (resizeObserver) {
                resizeObserver.disconnect()
            }
            if (terminalRef.current) {
                terminalRef.current.removeEventListener('click', handleClick)
            }
        }
    }, [])

    return (
        <div className="h-full w-full flex flex-col bg-background-level-2">
            <div
                ref={terminalRef}
                className="flex-1 w-full overflow-hidden focus:outline-none console-terminal"
                style={{
                    fontFamily: 'var(--font-mono)',
                    scrollbarWidth: 'thin',
                    scrollbarColor: 'transparent transparent'
                }}
                tabIndex={0}
                onFocus={() => {
                    if (terminalConsoleRef.current) {
                        terminalConsoleRef.current.focus()
                    }
                }}
                onBlur={() => {
                    if (terminalConsoleRef.current) {
                        terminalConsoleRef.current.blur()
                    }
                }}
                onMouseEnter={(e) => {
                    e.currentTarget.style.scrollbarColor = '#7b827d transparent'
                }}
                onMouseLeave={(e) => {
                    e.currentTarget.style.scrollbarColor = 'transparent transparent'
                }}
            />
        </div>
    )
}

export const ConsolePanel: FC<ConsolePanelProps> = ({
    visible,
    leftPanelVisible,
    rightPanelVisible,
    leftPanelWidth = 230,
    rightPanelWidth = 230,
    height,
    setHeight
}) => {
    if (!visible) return null

    const [isResizeHovered, setIsResizeHovered] = useState(false)
    const [isResizing, setIsResizing] = useState(false)

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault()
        setIsResizing(true)

        const startY = e.clientY
        const startHeight = height

        const handleMouseMove = (e: MouseEvent) => {
            const deltaY = startY - e.clientY
            const newHeight = Math.max(200, Math.min(800, startHeight + deltaY))
            setHeight(newHeight)
        }

        const handleMouseUp = () => {
            setIsResizing(false)
            document.removeEventListener('mousemove', handleMouseMove)
            document.removeEventListener('mouseup', handleMouseUp)
        }

        document.addEventListener('mousemove', handleMouseMove)
        document.addEventListener('mouseup', handleMouseUp)
    }

    return (
        <div
            className={`absolute z-20 bg-background-level-2 text-foreground border border-t
                      ${isResizing || isResizeHovered ? 'border-t-primary' : 'border-t'}`}
            style={{
                left: leftPanelVisible ? `${leftPanelWidth + 16}px` : '16px',
                right: rightPanelVisible ? `${rightPanelWidth + 16}px` : '16px',
                bottom: '16px',
                height: `${height}px`,
                overflow: 'hidden',
                display: 'flex',
                flexDirection: 'column'
            }}
        >
            <div
                className="absolute top-0 left-0 right-0 h-1 cursor-ns-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
            />
            <div className="flex-1 w-full" style={{ paddingTop: 4 }}>
                <Console />
            </div>
        </div>
    )
}

export const commandRegistry = new EnhancedCommandRegistry()

commandRegistry.register({
    id: 'help',
    name: 'Help',
    icon: '❓',
    description: 'Show available commands or help for specific command',
    parameters: [
        { name: 'command', type: 'string', description: 'Optional: specific command to get help for', required: false }
    ],
    execute: async (context, payload) => {
        const commands = commandRegistry.getAll()

        if (payload.command) {
            const command = commands.find(cmd =>
                cmd.name.toLowerCase() === payload.command.toLowerCase() ||
                cmd.id.toLowerCase() === payload.command.toLowerCase()
            )

            if (!command) {
                return { content: `❌ Command not found: ${payload.command}` }
            }

            let parameterInfo = ''
            if (command.parameters.length > 0) {
                const paramList = command.parameters.map(param => {
                    const required = param.required ? ' (required)' : param.defaultValue !== undefined ? ` (default: ${param.defaultValue})` : ' (optional)'
                    return `  • ${param.name} (${param.type}): ${param.description}${required}`
                }).join('\n')
                parameterInfo = `\n\nParameters:\n${paramList}`
            }

            const hotkey = command.hotkey ? `\nHotkey: ${command.hotkey}` : ''

            return {
                content: `${command.icon || '⚡'} ${command.name}\n${command.description}${hotkey}${parameterInfo}`
            }
        }

        const content = `📋 Available Commands:\n\n${commands.map(cmd =>
            `${cmd.icon || '•'} ${cmd.name} - ${cmd.description}${cmd.hotkey ? ` (${cmd.hotkey})` : ''}`
        ).join('\n')}\n\nTip: Use "help <command>" for detailed help on a specific command.`
        return { content }
    }
})

commandRegistry.register({
    id: 'clear',
    name: 'Clear',
    icon: '🧹',
    description: 'Clear the console',
    parameters: [],
    execute: async () => {
        return { content: null }
    }
})

export default Console


// Helper functions for commands
const addPieceToDesignHelper = (design: Design, piece: any): Design => ({
    ...design,
    pieces: [...(design.pieces || []), piece]
})

// Selection utility functions (taken from DesignEditor.tsx)
const selectAll = (design: Design): any => ({
    selectedPieceIds: design.pieces?.map((p: any) => p.id_) || [],
    selectedConnections: design.connections?.map((c: any) => ({ connectedPieceId: c.connected.piece.id_, connectingPieceId: c.connecting.piece.id_ })) || [],
    selectedPiecePortId: undefined
})

const deselectAll = (selection: any): any => ({
    selectedPieceIds: [],
    selectedConnections: [],
    selectedPiecePortId: undefined
})

const invertSelection = (selection: any, design: Design): any => {
    const allPieceIds = design.pieces?.map((p: any) => p.id_) || []
    const allConnections = design.connections?.map((c: any) => ({ connectedPieceId: c.connected.piece.id_, connectingPieceId: c.connecting.piece.id_ })) || []
    const newSelectedPieceIds = allPieceIds.filter(id => !selection.selectedPieceIds.includes(id))
    const newSelectedConnections = allConnections.filter(conn => {
        return !selection.selectedConnections.some((selected: any) =>
            selected.connectingPieceId === conn.connectingPieceId && selected.connectedPieceId === conn.connectedPieceId
        )
    })
    return ({ selectedPieceIds: newSelectedPieceIds, selectedConnections: newSelectedConnections, selectedPiecePortId: undefined })
}

const selectAllPieces = (design: Design): any => ({
    selectedPieceIds: design.pieces?.map((p: any) => p.id_) || [],
    selectedConnections: [],
    selectedPiecePortId: undefined
})

const selectAllConnections = (design: Design): any => ({
    selectedPieceIds: [],
    selectedConnections: design.connections?.map((c: any) => ({ connectedPieceId: c.connected.piece.id_, connectingPieceId: c.connecting.piece.id_ })) || [],
    selectedPiecePortId: undefined
})

const selectConnected = (design: Design, selection: any): any => {
    const connectedPieceIds = new Set<string>()
    selection.selectedPieceIds.forEach((pieceId: string) => {
        design.connections?.forEach((conn: any) => {
            if (conn.connecting.piece.id_ === pieceId) {
                connectedPieceIds.add(conn.connected.piece.id_)
            }
            if (conn.connected.piece.id_ === pieceId) {
                connectedPieceIds.add(conn.connecting.piece.id_)
            }
        })
    })
    return {
        ...selection,
        selectedPieceIds: [...new Set([...selection.selectedPieceIds, ...Array.from(connectedPieceIds)])]
    }
}

const selectDisconnected = (design: Design): any => {
    const connectedPieceIds = new Set<string>()
    design.connections?.forEach((conn: any) => {
        connectedPieceIds.add(conn.connecting.piece.id_)
        connectedPieceIds.add(conn.connected.piece.id_)
    })
    const disconnectedPieceIds = design.pieces?.filter((p: any) => !connectedPieceIds.has(p.id_)).map((p: any) => p.id_) || []
    return {
        selectedPieceIds: disconnectedPieceIds,
        selectedConnections: [],
        selectedPiecePortId: undefined
    }
}

const selectPiecesOfType = (design: Design, typeId: any): any => {
    const matchingPieceIds = design.pieces?.filter((p: any) =>
        p.type.name === typeId.name && (typeId.variant === undefined || p.type.variant === typeId.variant)
    ).map((p: any) => p.id_) || []
    return {
        selectedPieceIds: matchingPieceIds,
        selectedConnections: [],
        selectedPiecePortId: undefined
    }
}

export const designEditorCommands: Command[] = [
    // === PIECE COMMANDS ===
    {
        id: 'add-piece',
        name: 'Add Piece',
        icon: '➕',
        description: 'Add a new piece to the design',
        parameters: [
            { name: 'type', type: 'TypeId', description: 'Type of the piece', required: true },
            { name: 'center', type: 'DiagramPoint', description: 'Center coordinates (x,y)' },
            { name: 'plane', type: 'Plane', description: 'Plane for the piece' }
        ],
        execute: async (context, payload) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)

            const piece = {
                id_: `piece-${Date.now()}`,
                type: payload.type,
                center: payload.center,
                plane: payload.plane
            }

            return {
                design: addPieceToDesignHelper(design, piece),
                content: `✅ Added piece: ${payload.type.name}`
            }
        }
    },

    {
        id: 'duplicate-selected',
        name: 'Duplicate Selected',
        icon: '📋',
        description: 'Duplicate the selected pieces',
        parameters: [
            { name: 'offset', type: 'string', description: 'Offset coordinates (x,y)', defaultValue: '1,0' }
        ],
        hotkey: 'mod+d',
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)
            const offset = payload.offset ? payload.offset.split(',').map((n: string) => parseFloat(n.trim())) : [1, 0]

            let newDesign = design
            const newPieceIds: string[] = []

            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece) {
                    const newPiece = {
                        ...piece,
                        id_: `piece-${Date.now()}-${Math.random()}`,
                        center: {
                            x: (piece.center?.x || 0) + offset[0],
                            y: (piece.center?.y || 0) + offset[1]
                        }
                    }
                    newDesign = addPieceToDesignHelper(newDesign, newPiece)
                    newPieceIds.push(newPiece.id_)
                }
            }

            return {
                design: newDesign,
                selection: { ...selection, selectedPieceIds: newPieceIds },
                content: `✅ Duplicated ${newPieceIds.length} pieces`
            }
        }
    },

    {
        id: 'fix-selected-pieces',
        name: 'Fix Selected Pieces',
        icon: '📌',
        description: 'Fix the selected pieces at their current positions',
        parameters: [],
        hotkey: 'f',
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)
            let newDesign = design

            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece && !piece.plane) {
                    const updatedPiece = {
                        ...piece,
                        plane: {
                            origin: { x: piece.center?.x || 0, y: piece.center?.y || 0, z: 0 },
                            xAxis: { x: 1, y: 0, z: 0 },
                            yAxis: { x: 0, y: 1, z: 0 }
                        }
                    }
                    newDesign = setPieceInDesign(newDesign, updatedPiece)
                }
            }

            return {
                design: newDesign,
                content: `✅ Fixed ${selection.selectedPieceIds.length} pieces`
            }
        }
    },

    {
        id: 'unfix-selected-pieces',
        name: 'Unfix Selected Pieces',
        icon: '📌',
        description: 'Unfix the selected pieces (remove their plane)',
        parameters: [],
        hotkey: 'shift+f',
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)
            let newDesign = design

            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece && piece.plane) {
                    const updatedPiece = { ...piece, plane: undefined }
                    newDesign = setPieceInDesign(newDesign, updatedPiece)
                }
            }

            return {
                design: newDesign,
                content: `✅ Unfixed ${selection.selectedPieceIds.length} pieces`
            }
        }
    },

    // === DELETION COMMANDS ===
    {
        id: 'delete-selected',
        name: 'Delete Selected',
        icon: '🗑️',
        description: 'Delete the selected pieces and connections',
        parameters: [],
        hotkey: 'delete',
        execute: async (context) => {
            const { kit, designId, selection } = context
            const selectedPieces = selection.selectedPieceIds.map((id: string) => ({ id_: id }))
            const selectedConnections = selection.selectedConnections.map((conn: any) => ({
                connecting: { piece: { id_: conn.connectingPieceId } },
                connected: { piece: { id_: conn.connectedPieceId } }
            }))
            return {
                design: removePiecesAndConnectionsFromDesign(kit, designId, selectedPieces, selectedConnections),
                selection: deselectAll(selection),
                content: `✅ Deleted ${selectedPieces.length} pieces and ${selectedConnections.length} connections`
            }
        }
    },

    {
        id: 'delete-all-pieces',
        name: 'Delete All Pieces',
        icon: '🗑️',
        description: 'Delete all pieces from the design',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)
            return {
                design: { ...design, pieces: [], connections: [] },
                selection: deselectAll(context.selection),
                content: `✅ Deleted all pieces and connections`
            }
        }
    },

    // === SELECTION COMMANDS ===
    {
        id: 'select-all',
        name: 'Select All',
        icon: '🔘',
        description: 'Select all pieces and connections',
        parameters: [],
        hotkey: 'mod+a',
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectAll(design),
                content: `✅ Selected all pieces and connections`
            }
        }
    },

    {
        id: 'deselect-all',
        name: 'Deselect All',
        icon: '⭕',
        description: 'Deselect all pieces and connections',
        parameters: [],
        hotkey: 'escape',
        editorOnly: true,
        execute: async (context) => {
            return {
                selection: deselectAll(context.selection),
                content: `✅ Deselected all`
            }
        }
    },

    {
        id: 'invert-selection',
        name: 'Invert Selection',
        icon: '🔄',
        description: 'Invert the current selection',
        parameters: [],
        hotkey: 'mod+i',
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: invertSelection(context.selection, design),
                content: `✅ Inverted selection`
            }
        }
    },

    {
        id: 'select-all-pieces',
        name: 'Select All Pieces',
        icon: '🎯',
        description: 'Select all pieces (not connections)',
        parameters: [],
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectAllPieces(design),
                content: `✅ Selected all pieces`
            }
        }
    },

    {
        id: 'select-all-connections',
        name: 'Select All Connections',
        icon: '🔗',
        description: 'Select all connections (not pieces)',
        parameters: [],
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectAllConnections(design),
                content: `✅ Selected all connections`
            }
        }
    },

    {
        id: 'select-connected',
        name: 'Select Connected',
        icon: '🔗',
        description: 'Select all pieces connected to the current selection',
        parameters: [],
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectConnected(design, context.selection),
                content: `✅ Selected connected pieces`
            }
        }
    },

    {
        id: 'select-disconnected',
        name: 'Select Disconnected',
        icon: '⚡',
        description: 'Select all pieces not connected to anything',
        parameters: [],
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectDisconnected(design),
                content: `✅ Selected disconnected pieces`
            }
        }
    },

    {
        id: 'select-by-type',
        name: 'Select by Type',
        icon: '🏷️',
        description: 'Select all pieces of a specific type',
        parameters: [
            { name: 'type', type: 'TypeId', description: 'Type to select', required: true }
        ],
        execute: async (context, payload) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectPiecesOfType(design, payload.type),
                content: `✅ Selected pieces of type: ${payload.type.name}`
            }
        }
    },

    // === DESIGN MANIPULATION COMMANDS ===
    {
        id: 'flatten-design',
        name: 'Flatten Design',
        icon: '🏗️',
        description: 'Flatten the design by fixing all pieces at calculated positions',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)
            return {
                design: flattenDesign(kit, designId),
                content: `✅ Design flattened`
            }
        }
    },

    {
        id: 'orient-design',
        name: 'Orient Design',
        icon: '🧭',
        description: 'Orient the design based on the connections',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)
            return {
                design: orientDesign(design),
                content: `✅ Design oriented`
            }
        }
    },

    {
        id: 'center-design',
        name: 'Center Design',
        icon: '📐',
        description: 'Center all pieces around the origin',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)

            if (!design.pieces?.length) {
                return { content: `⚠️ No pieces to center` }
            }

            // Calculate centroid
            const totalX = design.pieces.reduce((sum, p) => sum + (p.center?.x || 0), 0)
            const totalY = design.pieces.reduce((sum, p) => sum + (p.center?.y || 0), 0)
            const centroidX = totalX / design.pieces.length
            const centroidY = totalY / design.pieces.length

            // Offset all pieces
            const centeredPieces = design.pieces.map(piece => ({
                ...piece,
                center: {
                    x: (piece.center?.x || 0) - centroidX,
                    y: (piece.center?.y || 0) - centroidY
                }
            }))

            return {
                design: { ...design, pieces: centeredPieces },
                content: `✅ Design centered`
            }
        }
    },

    // === CONNECTION COMMANDS ===
    {
        id: 'add-connection',
        name: 'Add Connection',
        icon: '🔗',
        description: 'Add a connection between two pieces',
        parameters: [
            { name: 'connectingPiece', type: 'PieceId', description: 'Connecting piece ID', required: true },
            { name: 'connectedPiece', type: 'PieceId', description: 'Connected piece ID', required: true },
            { name: 'connectingPort', type: 'PortId', description: 'Connecting port ID', required: false },
            { name: 'connectedPort', type: 'PortId', description: 'Connected port ID', required: false }
        ],
        execute: async (context, payload) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)

            const connection = {
                id_: `connection-${Date.now()}`,
                connecting: {
                    piece: { id_: payload.connectingPiece },
                    port: payload.connectingPort ? { id_: payload.connectingPort } : { id_: undefined }
                },
                connected: {
                    piece: { id_: payload.connectedPiece },
                    port: payload.connectedPort ? { id_: payload.connectedPort } : { id_: undefined }
                }
            }

            return {
                design: addConnectionToDesign(design, connection),
                content: `✅ Added connection`
            }
        }
    },

    {
        id: 'delete-all-connections',
        name: 'Delete All Connections',
        icon: '⛓️‍💥',
        description: 'Delete all connections from the design',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)
            return {
                design: { ...design, connections: [] },
                selection: { ...context.selection, selectedConnections: [] },
                content: `✅ Deleted all connections`
            }
        }
    },

    // === GRID AND ALIGNMENT COMMANDS ===
    {
        id: 'align-pieces-horizontal',
        name: 'Align Pieces Horizontally',
        icon: '⚖️',
        description: 'Align selected pieces horizontally',
        parameters: [
            {
                name: 'position', type: 'select', description: 'Alignment position', options: [
                    { value: 'top', label: 'Top' },
                    { value: 'center', label: 'Center' },
                    { value: 'bottom', label: 'Bottom' }
                ], defaultValue: 'center'
            }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length < 2) {
                return { content: `⚠️ Select at least 2 pieces to align` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            const yPositions = selectedPieces.map(p => p.center?.y || 0)

            let targetY: number
            switch (payload.position) {
                case 'top': targetY = Math.max(...yPositions); break
                case 'bottom': targetY = Math.min(...yPositions); break
                default: targetY = yPositions.reduce((sum, y) => sum + y, 0) / yPositions.length
            }

            let newDesign = design
            for (const piece of selectedPieces) {
                const updatedPiece = {
                    ...piece,
                    center: { ...piece.center, x: piece.center?.x || 0, y: targetY }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Aligned ${selectedPieces.length} pieces horizontally`
            }
        }
    },

    {
        id: 'align-pieces-vertical',
        name: 'Align Pieces Vertically',
        icon: '⚖️',
        description: 'Align selected pieces vertically',
        parameters: [
            {
                name: 'position', type: 'select', description: 'Alignment position', options: [
                    { value: 'left', label: 'Left' },
                    { value: 'center', label: 'Center' },
                    { value: 'right', label: 'Right' }
                ], defaultValue: 'center'
            }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length < 2) {
                return { content: `⚠️ Select at least 2 pieces to align` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            const xPositions = selectedPieces.map(p => p.center?.x || 0)

            let targetX: number
            switch (payload.position) {
                case 'left': targetX = Math.min(...xPositions); break
                case 'right': targetX = Math.max(...xPositions); break
                default: targetX = xPositions.reduce((sum, x) => sum + x, 0) / xPositions.length
            }

            let newDesign = design
            for (const piece of selectedPieces) {
                const updatedPiece = {
                    ...piece,
                    center: { ...piece.center, x: targetX, y: piece.center?.y || 0 }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Aligned ${selectedPieces.length} pieces vertically`
            }
        }
    },

    {
        id: 'distribute-pieces-horizontal',
        name: 'Distribute Pieces Horizontally',
        icon: '📏',
        description: 'Distribute selected pieces evenly horizontally',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length < 3) {
                return { content: `⚠️ Select at least 3 pieces to distribute` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            selectedPieces.sort((a, b) => (a.center?.x || 0) - (b.center?.x || 0))

            const minX = selectedPieces[0].center?.x || 0
            const maxX = selectedPieces[selectedPieces.length - 1].center?.x || 0
            const spacing = (maxX - minX) / (selectedPieces.length - 1)

            let newDesign = design
            for (let i = 1; i < selectedPieces.length - 1; i++) {
                const piece = selectedPieces[i]
                const updatedPiece = {
                    ...piece,
                    center: { x: minX + i * spacing, y: piece.center?.y || 0 }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Distributed ${selectedPieces.length} pieces horizontally`
            }
        }
    },

    {
        id: 'distribute-pieces-vertical',
        name: 'Distribute Pieces Vertically',
        icon: '📏',
        description: 'Distribute selected pieces evenly vertically',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length < 3) {
                return { content: `⚠️ Select at least 3 pieces to distribute` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            selectedPieces.sort((a, b) => (a.center?.y || 0) - (b.center?.y || 0))

            const minY = selectedPieces[0].center?.y || 0
            const maxY = selectedPieces[selectedPieces.length - 1].center?.y || 0
            const spacing = (maxY - minY) / (selectedPieces.length - 1)

            let newDesign = design
            for (let i = 1; i < selectedPieces.length - 1; i++) {
                const piece = selectedPieces[i]
                const updatedPiece = {
                    ...piece,
                    center: { x: piece.center?.x || 0, y: minY + i * spacing }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Distributed ${selectedPieces.length} pieces vertically`
            }
        }
    },

    // === TRANSFORM COMMANDS ===
    {
        id: 'move-selected',
        name: 'Move Selected',
        icon: '🏃',
        description: 'Move selected pieces by offset',
        parameters: [
            { name: 'offset', type: 'string', description: 'Move offset (x,y)', defaultValue: '0,0', required: true }
        ],
        hotkey: 'mod+m',
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)
            const offset = payload.offset.split(',').map((n: string) => parseFloat(n.trim()))

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to move` }
            }

            let newDesign = design
            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece) {
                    const updatedPiece = {
                        ...piece,
                        center: {
                            x: (piece.center?.x || 0) + offset[0],
                            y: (piece.center?.y || 0) + offset[1]
                        }
                    }
                    newDesign = setPieceInDesign(newDesign, updatedPiece)
                }
            }

            return {
                design: newDesign,
                content: `✅ Moved ${selection.selectedPieceIds.length} pieces`
            }
        }
    },

    {
        id: 'rotate-selected',
        name: 'Rotate Selected',
        icon: '🔄',
        description: 'Rotate selected pieces around their centroid',
        parameters: [
            { name: 'angle', type: 'number', description: 'Rotation angle in degrees', defaultValue: 90, required: true }
        ],
        hotkey: 'mod+r',
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to rotate` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []

            // Calculate centroid
            const totalX = selectedPieces.reduce((sum, p) => sum + (p.center?.x || 0), 0)
            const totalY = selectedPieces.reduce((sum, p) => sum + (p.center?.y || 0), 0)
            const centroidX = totalX / selectedPieces.length
            const centroidY = totalY / selectedPieces.length

            const angle = (payload.angle * Math.PI) / 180
            const cos = Math.cos(angle)
            const sin = Math.sin(angle)

            let newDesign = design
            for (const piece of selectedPieces) {
                const relX = (piece.center?.x || 0) - centroidX
                const relY = (piece.center?.y || 0) - centroidY
                const newX = centroidX + relX * cos - relY * sin
                const newY = centroidY + relX * sin + relY * cos

                const updatedPiece = {
                    ...piece,
                    center: { x: newX, y: newY }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Rotated ${selectedPieces.length} pieces by ${payload.angle}°`
            }
        }
    },

    {
        id: 'scale-selected',
        name: 'Scale Selected',
        icon: '🔍',
        description: 'Scale selected pieces around their centroid',
        parameters: [
            { name: 'factor', type: 'number', description: 'Scale factor', defaultValue: 1.5, required: true }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to scale` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []

            // Calculate centroid
            const totalX = selectedPieces.reduce((sum, p) => sum + (p.center?.x || 0), 0)
            const totalY = selectedPieces.reduce((sum, p) => sum + (p.center?.y || 0), 0)
            const centroidX = totalX / selectedPieces.length
            const centroidY = totalY / selectedPieces.length

            let newDesign = design
            for (const piece of selectedPieces) {
                const relX = (piece.center?.x || 0) - centroidX
                const relY = (piece.center?.y || 0) - centroidY
                const newX = centroidX + relX * payload.factor
                const newY = centroidY + relY * payload.factor

                const updatedPiece = {
                    ...piece,
                    center: { x: newX, y: newY }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Scaled ${selectedPieces.length} pieces by ${payload.factor}x`
            }
        }
    },

    // === ANALYSIS COMMANDS ===
    {
        id: 'analyze-design',
        name: 'Analyze Design',
        icon: '🔍',
        description: 'Show design statistics and analysis',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)

            const pieceCount = design.pieces?.length || 0
            const connectionCount = design.connections?.length || 0

            const typeCount = new Map<string, number>()
            design.pieces?.forEach(piece => {
                const typeName = piece.type.name
                typeCount.set(typeName, (typeCount.get(typeName) || 0) + 1)
            })

            const fixedPieces = design.pieces?.filter(p => p.plane) || []
            const disconnectedPieces = design.pieces?.filter(piece => {
                return !design.connections?.some(conn =>
                    conn.connecting.piece.id_ === piece.id_ || conn.connected.piece.id_ === piece.id_
                )
            }) || []

            const typesList = Array.from(typeCount.entries())
                .map(([type, count]) => `  • ${type}: ${count}`)
                .join('\n')

            return {
                content: `📊 Design Analysis:
• Pieces: ${pieceCount}
• Connections: ${connectionCount}
• Fixed pieces: ${fixedPieces.length}
• Disconnected pieces: ${disconnectedPieces.length}

🏷️ Types used:
${typesList}`
            }
        }
    },

    {
        id: 'randomize-positions',
        name: 'Randomize Positions',
        icon: '🎲',
        description: 'Randomize positions of selected pieces',
        parameters: [
            { name: 'range', type: 'number', description: 'Randomization range', defaultValue: 5 }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to randomize` }
            }

            let newDesign = design
            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece) {
                    const updatedPiece = {
                        ...piece,
                        center: {
                            x: (piece.center?.x || 0) + (Math.random() - 0.5) * payload.range * 2,
                            y: (piece.center?.y || 0) + (Math.random() - 0.5) * payload.range * 2
                        }
                    }
                    newDesign = setPieceInDesign(newDesign, updatedPiece)
                }
            }

            return {
                design: newDesign,
                content: `✅ Randomized ${selection.selectedPieceIds.length} piece positions`
            }
        }
    },

    {
        id: 'create-grid-layout',
        name: 'Create Grid Layout',
        icon: '📋',
        description: 'Arrange selected pieces in a grid layout',
        parameters: [
            { name: 'columns', type: 'number', description: 'Number of columns', defaultValue: 3, required: true },
            { name: 'spacing', type: 'number', description: 'Spacing between pieces', defaultValue: 2, required: true }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to arrange` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            const { columns, spacing } = payload

            let newDesign = design
            selectedPieces.forEach((piece, index) => {
                const row = Math.floor(index / columns)
                const col = index % columns
                const updatedPiece = {
                    ...piece,
                    center: {
                        x: col * spacing,
                        y: row * spacing
                    }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            })

            return {
                design: newDesign,
                content: `✅ Arranged ${selectedPieces.length} pieces in ${columns}-column grid`
            }
        }
    },

    {
        id: 'mirror-selected-horizontal',
        name: 'Mirror Selected Horizontally',
        icon: '🪞',
        description: 'Mirror selected pieces horizontally around their centroid',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to mirror` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []

            // Calculate centroid
            const totalX = selectedPieces.reduce((sum, p) => sum + (p.center?.x || 0), 0)
            const centroidX = totalX / selectedPieces.length

            let newDesign = design
            for (const piece of selectedPieces) {
                const distanceFromCenter = (piece.center?.x || 0) - centroidX
                const updatedPiece = {
                    ...piece,
                    center: {
                        x: centroidX - distanceFromCenter,
                        y: piece.center?.y || 0
                    }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Mirrored ${selectedPieces.length} pieces horizontally`
            }
        }
    },

    {
        id: 'mirror-selected-vertical',
        name: 'Mirror Selected Vertically',
        icon: '🪞',
        description: 'Mirror selected pieces vertically around their centroid',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to mirror` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []

            // Calculate centroid
            const totalY = selectedPieces.reduce((sum, p) => sum + (p.center?.y || 0), 0)
            const centroidY = totalY / selectedPieces.length

            let newDesign = design
            for (const piece of selectedPieces) {
                const distanceFromCenter = (piece.center?.y || 0) - centroidY
                const updatedPiece = {
                    ...piece,
                    center: {
                        x: piece.center?.x || 0,
                        y: centroidY - distanceFromCenter
                    }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Mirrored ${selectedPieces.length} pieces vertically`
            }
        }
    },

    // === EXPORT/IMPORT COMMANDS ===
    {
        id: 'export-selected-to-clipboard',
        name: 'Export Selected to Clipboard',
        icon: '📋',
        description: 'Copy selected pieces to clipboard as JSON',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to export` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            const selectedConnections = design.connections?.filter(conn =>
                selection.selectedConnections.some((selConn: any) =>
                    selConn.connectingPieceId === conn.connecting.piece.id_ &&
                    selConn.connectedPieceId === conn.connected.piece.id_
                )
            ) || []

            const exportData = {
                pieces: selectedPieces,
                connections: selectedConnections
            }

            try {
                await navigator.clipboard.writeText(JSON.stringify(exportData, null, 2))
                return {
                    content: `✅ Exported ${selectedPieces.length} pieces and ${selectedConnections.length} connections to clipboard`
                }
            } catch (error) {
                return {
                    content: `❌ Failed to copy to clipboard`
                }
            }
        }
    },

    // === UTILITY COMMANDS ===
    {
        id: 'help',
        name: 'Help',
        icon: '❓',
        description: 'Show available commands or help for specific command',
        parameters: [
            { name: 'command', type: 'string', description: 'Optional: specific command to get help for', required: false }
        ],
        execute: async (context, payload) => {
            const commands = designEditorCommands

            if (payload.command) {
                const command = commands.find(cmd =>
                    cmd.name.toLowerCase() === payload.command.toLowerCase() ||
                    cmd.id.toLowerCase() === payload.command.toLowerCase()
                )

                if (!command) {
                    return { content: `❌ Command not found: ${payload.command}` }
                }

                let parameterInfo = ''
                if (command.parameters.length > 0) {
                    const paramList = command.parameters.map(param => {
                        const required = param.required ? ' (required)' : param.defaultValue !== undefined ? ` (default: ${param.defaultValue})` : ' (optional)'
                        return `  • ${param.name} (${param.type}): ${param.description}${required}`
                    }).join('\n')
                    parameterInfo = `\n\nParameters:\n${paramList}`
                }

                const hotkey = command.hotkey ? `\nHotkey: ${command.hotkey}` : ''

                return {
                    content: `${command.icon || '⚡'} ${command.name}\n${command.description}${hotkey}${parameterInfo}`
                }
            }

            const commandsList = commands.map(cmd => {
                const hotkey = cmd.hotkey ? ` (${cmd.hotkey})` : ''
                return `${cmd.icon || '⚡'} ${cmd.name}${hotkey}
  - ${cmd.description}`
            }).join('\n\n')

            return {
                content: `📚 Available Commands:

${commandsList}

Tip: Use "help <command>" for detailed help on a specific command.`
            }
        }
    },

    {
        id: 'clear',
        name: 'Clear',
        icon: '🧹',
        description: 'Clear the console',
        parameters: [],
        editorOnly: true,
        execute: async () => ({ content: null })
    }
]

// Register all design editor commands
designEditorCommands.forEach(command => {
    commandRegistry.register(command)
})
