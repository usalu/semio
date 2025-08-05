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

import { FitAddon } from "@xterm/addon-fit";
import { Terminal } from "@xterm/xterm";
import "@xterm/xterm/css/xterm.css";
import React, { FC, useEffect, useRef, useState } from "react";

import { Design, DesignId, Kit, useDesignEditor } from "@semio/js";

export interface CommandParameter {
  name: string;
  type: "string" | "number" | "boolean" | "select" | "DiagramPoint" | "Plane" | "TypeId" | "PortId" | "PieceId";
  description: string;
  required?: boolean;
  options?: { value: string; label: string }[];
  defaultValue?: any | (() => any);
}

export interface CommandContext {
  kit: Kit;
  designId: DesignId;
  selection: any;
}

export interface CommandResult {
  design?: Design;
  selection?: any;
  fileUrls?: string[];
  fullscreenPanel?: any;
  content?: React.ReactNode;
}

export interface Command {
  id: string;
  name: string;
  icon?: string;
  description: string;
  parameters: CommandParameter[];
  hotkey?: string;
  editorOnly?: boolean;
  execute: (context: CommandContext, payload: Record<string, any>) => Promise<CommandResult>;
}

export interface ParameterFormProps {
  parameter: CommandParameter;
  value?: any;
  onSubmit: (value: any) => void;
  onCancel: () => void;
}

class EnhancedCommandRegistry {
  private commands = new Map<string, Command>();

  register(command: Command): () => void {
    this.commands.set(command.id, command);
    return () => this.unregister(command.id);
  }

  unregister(commandId: string): void {
    this.commands.delete(commandId);
  }

  get(commandId: string): Command | undefined {
    return this.commands.get(commandId);
  }

  getAll(): Command[] {
    return Array.from(this.commands.values());
  }

  search(query: string): Command[] {
    const allCommands = this.getAll();

    if (!query.trim()) {
      return allCommands; // Return all commands if query is empty
    }

    const lowercaseQuery = query.toLowerCase();
    const filtered = allCommands.filter((cmd) => cmd.name.toLowerCase().includes(lowercaseQuery) || cmd.description.toLowerCase().includes(lowercaseQuery) || cmd.id.toLowerCase().includes(lowercaseQuery));

    return filtered;
  }

  async execute(commandId: string, context: CommandContext, payload: Record<string, any> = {}): Promise<CommandResult> {
    const command = this.get(commandId);
    if (!command) throw new Error(`Command '${commandId}' not found`);
    return command.execute(context, payload);
  }
}

export interface ConsoleState {
  mode: "input" | "parameter-gathering" | "command-output";
  input: string;
  suggestions: string[];
  selectedSuggestion: number;
  currentCommand?: Command;
  parameterIndex: number;
  gatheredParameters: Record<string, any>;
  outputContent?: React.ReactNode;
  commandHistory: string[];
  historyIndex: number;
}

interface ConsolePanelProps {
  visible: boolean;
  leftPanelVisible: boolean;
  rightPanelVisible: boolean;
  leftPanelWidth?: number;
  rightPanelWidth?: number;
  height: number;
  setHeight: (height: number) => void;
}

//#region Forms

class TerminalForm {
  protected terminal: Terminal;
  protected onSubmit: (value: any) => void;
  protected onCancel: () => void;
  protected onBack: () => void;
  protected parameter: CommandParameter;
  protected cleanupHandlers: (() => void)[] = [];

  constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void) {
    this.terminal = terminal;
    this.parameter = parameter;
    this.onSubmit = onSubmit;
    this.onCancel = onCancel;
    this.onBack = onBack;
  }

  protected writeColored(text: string, color?: string): void {
    const colorCodes: Record<string, string> = {
      gray: "\x1b[90m",
      blue: "\x1b[94m",
      green: "\x1b[92m",
      red: "\x1b[91m",
      yellow: "\x1b[93m",
      white: "\x1b[97m",
      reset: "\x1b[0m",
    };

    if (color && colorCodes[color]) {
      this.terminal.write(colorCodes[color] + text + colorCodes.reset);
    } else {
      this.terminal.write(text);
    }
  }

  protected clearForm(): void {
    this.terminal.clear(); // Clear screen but don't move cursor to top
  }

  start(): void {
    this.clearForm();
    // Position form content above the input line
    const terminalRows = this.terminal.rows;
    const inputRow = terminalRows;
    this.terminal.write(`\x1b[${Math.max(1, inputRow - 5)};1H`); // Position form above input
    this.render();
    this.setupHandlers();
  }

  protected render(): void {
    // Override in subclasses - don't clear here as positioning is already set
  }

  protected setupHandlers(): void {
    // Override in subclasses
  }

  cleanup(): void {
    this.cleanupHandlers.forEach((cleanup) => cleanup());
    this.cleanupHandlers = [];
  }
}

class TypeIdForm extends TerminalForm {
  private selectedIndex = 0;
  private mode: "type" | "variant" = "type";
  private selectedType = "";
  private filteredItems: string[] = [];
  private searchTerm = "";
  private types: any[] = [];
  private typeNames: string[] = [];
  private variants: string[] = [];

  constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void, designEditor: any) {
    super(terminal, parameter, onSubmit, onCancel, onBack);
    this.types = designEditor.kit?.types || [];
    this.typeNames = [...new Set(this.types.map((t) => t.name))];
    this.updateFilteredItems();
  }

  private updateFilteredItems(): void {
    const currentItems = this.mode === "type" ? this.typeNames : this.variants;
    this.filteredItems = currentItems.filter((item) => item.toLowerCase().includes(this.searchTerm.toLowerCase()));
    this.selectedIndex = 0;
  }

  private updateVariants(): void {
    this.variants = this.selectedType
      ? [
          ...new Set(
            this.types
              .filter((t) => t.name === this.selectedType)
              .map((t) => t.variant)
              .filter((v) => Boolean(v)),
          ),
        ]
      : [];
  }

  protected render(): void {
    // Don't clear form here - position is already set in start()
    this.writeColored("┌─────────────────────────────────────┐\r\n", "gray");
    const title = this.mode === "type" ? "🔧 Select Type:" : `🎨 Select Variant for ${this.selectedType}:`;
    this.writeColored(`│ ${title.padEnd(35)} │\r\n`, "gray");
    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");

    this.writeColored(`│ Search: ${this.searchTerm}▋${" ".repeat(35 - this.searchTerm.length - 9)} │\r\n`, "white");
    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");

    this.filteredItems.slice(0, 5).forEach((item, index) => {
      const prefix = index === this.selectedIndex ? "> " : "  ";
      const color = index === this.selectedIndex ? "blue" : "white";
      this.writeColored(`│ ${prefix}${item.padEnd(33)} │\r\n`, color);
    });

    for (let i = this.filteredItems.length; i < 5; i++) {
      this.writeColored(`│${" ".repeat(37)}│\r\n`, "gray");
    }

    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");
    const helpText = this.mode === "type" ? "Use ↑↓ to navigate, Tab autocomplete, Enter select" : "Use ↑↓ navigate, Tab autocomplete, Enter confirm";
    this.writeColored(`│ ${helpText.padEnd(35)} │\r\n`, "gray");
    this.writeColored("└─────────────────────────────────────┘\r\n", "gray");
  }

  protected setupHandlers(): void {
    const handler = (e: string) => {
      if (e === "\x1b") {
        // ESC
        this.onCancel();
        return;
      }

      if (e === "\t") {
        // Tab
        if (this.searchTerm && this.filteredItems.length > 0) {
          this.searchTerm = this.filteredItems[0];
          this.updateFilteredItems();
          this.render();
        }
        return;
      }

      if (e === "\r") {
        // Enter
        if (this.mode === "type") {
          const typeToSelect = this.filteredItems[this.selectedIndex] || this.searchTerm;
          if (this.typeNames.includes(typeToSelect)) {
            this.selectedType = typeToSelect;
            this.mode = "variant";
            this.searchTerm = "";
            this.updateVariants();
            this.updateFilteredItems();
            this.render();
          }
        } else {
          const variantToSelect = this.filteredItems[this.selectedIndex] || this.searchTerm;
          this.onSubmit({
            name: this.selectedType,
            variant: variantToSelect || undefined,
          });
        }
        return;
      }

      if (e === "\x1b[A") {
        // Up arrow
        this.selectedIndex = Math.max(0, this.selectedIndex - 1);
        this.render();
        return;
      }

      if (e === "\x1b[B") {
        // Down arrow
        this.selectedIndex = Math.min(this.filteredItems.length - 1, this.selectedIndex + 1);
        this.render();
        return;
      }

      if (e === "\x7f") {
        // Backspace
        if (this.searchTerm.length > 0) {
          // If there's a search term, delete from it
          this.searchTerm = this.searchTerm.slice(0, -1);
          this.updateFilteredItems();
          this.render();
        } else {
          // If no search term, go back to previous parameter
          this.onBack();
        }
        return;
      }

      if (e.length === 1 && e >= " ") {
        // Printable character
        this.searchTerm += e;
        this.updateFilteredItems();
        this.render();
      }
    };

    // Remove any existing handlers first
    this.cleanupHandlers.forEach((cleanup) => cleanup());
    this.cleanupHandlers = [];

    this.terminal.onData(handler);
    this.cleanupHandlers.push(() => this.terminal.onData(() => {}));
  }
}

class StringForm extends TerminalForm {
  private inputValue = "";

  constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void, initialValue?: any) {
    super(terminal, parameter, onSubmit, onCancel, onBack);
    this.inputValue = initialValue || parameter.defaultValue || "";
  }

  protected render(): void {
    // Don't clear form here - position is already set in start()
    this.writeColored("┌─────────────────────────────────────┐\r\n", "gray");
    this.writeColored(`│ 📝 ${this.parameter.description.padEnd(31)} │\r\n`, "gray");
    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");
    this.writeColored(`│ ${this.inputValue}▋${" ".repeat(34 - this.inputValue.length)} │\r\n`, "white");
    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");
    this.writeColored(`│ Enter to confirm, Esc to cancel     │\r\n`, "gray");
    this.writeColored("└─────────────────────────────────────┘\r\n", "gray");
  }

  protected setupHandlers(): void {
    const handler = (e: string) => {
      if (e === "\x1b") {
        // ESC
        this.onCancel();
        return;
      }

      if (e === "\r") {
        // Enter
        this.onSubmit(this.inputValue);
        return;
      }

      if (e === "\x7f") {
        // Backspace
        if (this.inputValue.length > 0) {
          // If there's input, delete from it
          this.inputValue = this.inputValue.slice(0, -1);
          this.render();
        } else {
          // If no input, go back to previous parameter
          this.onBack();
        }
        return;
      }

      if (e.length === 1 && e >= " ") {
        // Printable character
        this.inputValue += e;
        this.render();
      }
    };

    // Remove any existing handlers first
    this.cleanupHandlers.forEach((cleanup) => cleanup());
    this.cleanupHandlers = [];

    this.terminal.onData(handler);
    this.cleanupHandlers.push(() => this.terminal.onData(() => {}));
  }
}

class SelectForm extends TerminalForm {
  private selectedIndex = 0;
  private options: { value: string; label: string }[] = [];

  constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void) {
    super(terminal, parameter, onSubmit, onCancel, onBack);
    this.options = parameter.options || [];
  }

  protected render(): void {
    // Don't clear form here - position is already set in start()
    this.writeColored("┌─────────────────────────────────────┐\r\n", "gray");
    this.writeColored(`│ 📋 ${this.parameter.description.padEnd(31)} │\r\n`, "gray");
    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");

    this.options.forEach((option, index) => {
      const prefix = index === this.selectedIndex ? "> " : "  ";
      const color = index === this.selectedIndex ? "blue" : "white";
      this.writeColored(`│ ${prefix}${option.label.padEnd(33)} │\r\n`, color);
    });

    for (let i = this.options.length; i < 6; i++) {
      this.writeColored(`│${" ".repeat(37)}│\r\n`, "gray");
    }

    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");
    this.writeColored(`│ Use ↑↓ to navigate, Enter to select │\r\n`, "gray");
    this.writeColored("└─────────────────────────────────────┘\r\n", "gray");
  }

  protected setupHandlers(): void {
    const handler = (e: string) => {
      if (e === "\x1b") {
        // ESC
        this.onCancel();
        return;
      }

      if (e === "\r") {
        // Enter
        this.onSubmit(this.options[this.selectedIndex]?.value);
        return;
      }

      if (e === "\x1b[A") {
        // Up arrow
        this.selectedIndex = Math.max(0, this.selectedIndex - 1);
        this.render();
        return;
      }

      if (e === "\x1b[B") {
        // Down arrow
        this.selectedIndex = Math.min(this.options.length - 1, this.selectedIndex + 1);
        this.render();
        return;
      }

      if (e === "\x7f") {
        // Backspace
        this.onBack();
        return;
      }
    };

    // Remove any existing handlers first
    this.cleanupHandlers.forEach((cleanup) => cleanup());
    this.cleanupHandlers = [];

    this.terminal.onData(handler);
    this.cleanupHandlers.push(() => this.terminal.onData(() => {}));
  }
}

class NumberForm extends TerminalForm {
  private inputValue = "";

  constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void, initialValue?: any) {
    super(terminal, parameter, onSubmit, onCancel, onBack);
    this.inputValue = String(initialValue || parameter.defaultValue || "0");
  }

  protected render(): void {
    // Don't clear form here - position is already set in start()
    this.writeColored("┌─────────────────────────────────────┐\r\n", "gray");
    this.writeColored(`│ 🔢 ${this.parameter.description.padEnd(31)} │\r\n`, "gray");
    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");
    this.writeColored(`│ ${this.inputValue}▋${" ".repeat(34 - this.inputValue.length)} │\r\n`, "white");
    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");
    this.writeColored(`│ Enter to confirm, Esc to cancel     │\r\n`, "gray");
    this.writeColored("└─────────────────────────────────────┘\r\n", "gray");
  }

  protected setupHandlers(): void {
    const handler = (e: string) => {
      if (e === "\x1b") {
        // ESC
        this.onCancel();
        return;
      }

      if (e === "\r") {
        // Enter
        const value = parseFloat(this.inputValue);
        if (!isNaN(value)) {
          this.onSubmit(value);
        } else {
          this.onSubmit(0);
        }
        return;
      }

      if (e === "\x7f") {
        // Backspace
        if (this.inputValue.length > 0) {
          // If there's input, delete from it
          this.inputValue = this.inputValue.slice(0, -1);
          this.render();
        } else {
          // If no input, go back to previous parameter
          this.onBack();
        }
        return;
      }

      if (e.length === 1 && ((e >= "0" && e <= "9") || e === "." || e === "-" || e === "+")) {
        this.inputValue += e;
        this.render();
      }
    };

    // Remove any existing handlers first
    this.cleanupHandlers.forEach((cleanup) => cleanup());
    this.cleanupHandlers = [];

    this.terminal.onData(handler);
    this.cleanupHandlers.push(() => this.terminal.onData(() => {}));
  }
}

class BooleanForm extends TerminalForm {
  private value = false;

  constructor(terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void) {
    super(terminal, parameter, onSubmit, onCancel, onBack);
    this.value = parameter.defaultValue || false;
  }

  protected render(): void {
    // Don't clear form here - position is already set in start()
    this.writeColored("┌─────────────────────────────────────┐\r\n", "gray");
    this.writeColored(`│ ❓ ${this.parameter.description.padEnd(31)} │\r\n`, "gray");
    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");

    const valueText = this.value ? "✓ Yes" : "✗ No";
    const color = this.value ? "green" : "red";
    this.writeColored(`│ ${valueText.padEnd(35)} │\r\n`, color);

    this.writeColored("├─────────────────────────────────────┤\r\n", "gray");
    this.writeColored(`│ Y/N, Space to toggle, Enter confirm │\r\n`, "gray");
    this.writeColored("└─────────────────────────────────────┘\r\n", "gray");
  }

  protected setupHandlers(): void {
    const handler = (e: string) => {
      if (e === "\x1b") {
        // ESC
        this.onCancel();
        return;
      }

      if (e === "\r") {
        // Enter
        this.onSubmit(this.value);
        return;
      }

      if (e === "y" || e === "Y") {
        this.value = true;
        this.render();
        return;
      }

      if (e === "n" || e === "N") {
        this.value = false;
        this.render();
        return;
      }

      if (e === " ") {
        this.value = !this.value;
        this.render();
        return;
      }

      if (e === "\x7f") {
        // Backspace
        this.onBack();
        return;
      }
    };

    // Remove any existing handlers first
    this.cleanupHandlers.forEach((cleanup) => cleanup());
    this.cleanupHandlers = [];

    this.terminal.onData(handler);
    this.cleanupHandlers.push(() => this.terminal.onData(() => {}));
  }
}

const createParameterForm = (terminal: Terminal, parameter: CommandParameter, onSubmit: (value: any) => void, onCancel: () => void, onBack: () => void, designEditor?: any, initialValue?: any): TerminalForm | null => {
  switch (parameter.type) {
    case "TypeId":
      return new TypeIdForm(terminal, parameter, onSubmit, onCancel, onBack, designEditor);
    case "string":
      return new StringForm(terminal, parameter, onSubmit, onCancel, onBack, initialValue);
    case "number":
      return new NumberForm(terminal, parameter, onSubmit, onCancel, onBack, initialValue);
    case "select":
      return new SelectForm(terminal, parameter, onSubmit, onCancel, onBack);
    case "boolean":
      return new BooleanForm(terminal, parameter, onSubmit, onCancel, onBack);
    default:
      return null;
  }
};

//#endregion Forms

class TerminalConsole {
  private terminal: Terminal;
  private fitAddon: FitAddon;
  private state: ConsoleState;
  private currentForm: TerminalForm | null = null;
  private designEditor: any;
  private onStateChange: (state: ConsoleState) => void;

  constructor(element: HTMLElement, designEditor: any, initialState: ConsoleState, onStateChange: (state: ConsoleState) => void) {
    this.terminal = new Terminal({
      fontSize: 14,
      fontFamily: '"Anta", "Noto Emoji"',
      cursorBlink: false, // Don't change this
      cursorStyle: "block", // Don't change this
      theme: {
        background: "#d3d2c5", // Don't change this
        foreground: "#001117",
        cursor: "#001117",
        cursorAccent: "#ff344f",
        selection: "rgba(255, 52, 79, 0.3)",
        black: "#001117",
        red: "#a60009",
        green: "#7eb77f",
        yellow: "#fa9500",
        blue: "#34d1bf",
        magenta: "#ff344f",
        cyan: "#dbbea1",
        white: "#f7f3e3",
        brightBlack: "#7b827d",
        brightRed: "#a60009",
        brightGreen: "#7eb77f",
        brightYellow: "#fa9500",
        brightBlue: "#34d1bf",
        brightMagenta: "#ff344f",
        brightCyan: "#dbbea1",
        brightWhite: "#001117",
      },
    });

    this.fitAddon = new FitAddon();
    this.terminal.loadAddon(this.fitAddon);
    this.terminal.open(element);
    this.fitAddon.fit();

    this.designEditor = designEditor;
    this.state = initialState;
    this.onStateChange = onStateChange;

    this.setupTerminalHandlers();
    this.showWelcome();

    // Initialize suggestions for empty input
    this.updateState({
      ...this.state,
      mode: "input",
    });

    // Ensure terminal has focus
    this.terminal.focus();
  }

  private showWelcome(): void {
    // Don't show welcome message as it will be cleared anyway
    this.updatePrompt();
  }

  private writeColored(text: string, color?: string): void {
    const colorCodes: Record<string, string> = {
      gray: "\x1b[90m",
      blue: "\x1b[94m",
      green: "\x1b[92m",
      red: "\x1b[91m",
      yellow: "\x1b[93m",
      white: "\x1b[97m",
      reset: "\x1b[0m",
    };

    if (color && colorCodes[color]) {
      this.terminal.write(colorCodes[color] + text + colorCodes.reset);
    } else {
      this.terminal.write(text);
    }
  }

  private updatePrompt(): void {
    if (this.state.mode === "input") {
      // Clear the entire terminal to prevent history buildup
      this.terminal.clear();

      // Calculate available space for content
      const terminalRows = this.terminal.rows;
      const inputRow = terminalRows;
      let usedRows = 1; // Reserve one row for input

      // Only show suggestions when typing and not navigating history
      const showSuggestions = this.state.input.length > 0 && this.state.suggestions.length > 0 && this.state.historyIndex === -1;

      if (showSuggestions) {
        const maxSuggestions = Math.min(5, this.state.suggestions.length);
        const suggestionRows = maxSuggestions + 2; // +2 for header and spacing
        usedRows += suggestionRows;

        // Position suggestions above the input
        const suggestionStartRow = Math.max(1, inputRow - usedRows + 1);
        this.terminal.write(`\x1b[${suggestionStartRow};1H`);

        this.writeColored("Available commands:\r\n", "blue");
        this.state.suggestions.slice(0, maxSuggestions).forEach((suggestion, index) => {
          const isSelected = index === this.state.selectedSuggestion;
          const prefix = isSelected ? "▶ " : "  ";
          const color = isSelected ? "magenta" : "gray";
          this.writeColored(`${prefix}${suggestion}\r\n`, color);
        });
        this.terminal.write("\r\n");
      }

      // Position cursor at the bottom row for input
      this.terminal.write(`\x1b[${inputRow};1H`);
      // Clear the line and write the input
      this.terminal.write("\x1b[K"); // Clear to end of line
      this.terminal.write(this.state.input);

      // Ensure terminal maintains focus after prompt update
      setTimeout(() => this.terminal.focus(), 0);
    }
  }

  private setupTerminalHandlers(): void {
    this.terminal.onData((data: string) => {
      if (data === "\x03") {
        // Ctrl+C - check for selection first
        this.handleCtrlC();
        return;
      }

      if (this.state.mode === "parameter-gathering") return; // Let form handle input

      if (data === "\x1b") {
        // ESC
        if (this.state.mode === "command-output") this.returnToInputMode();
        return;
      }

      if (data === "\r") {
        // Enter
        if (this.state.mode === "command-output") {
          this.returnToInputMode();
        } else if (this.state.input.length > 0 && this.state.suggestions.length > 0 && this.state.selectedSuggestion >= 0 && this.state.historyIndex === -1) {
          // Execute selected suggestion only when not in history mode
          this.updateState({
            ...this.state,
            input: this.state.suggestions[this.state.selectedSuggestion],
            historyIndex: -1,
          });
          // Small delay to let the state update, then submit
          setTimeout(() => this.handleSubmit(), 10);
        } else {
          this.handleSubmit();
        }
        return;
      }

      if (data === "\x1b[A") {
        // Up arrow
        if (this.state.mode === "command-output") {
          this.returnToInputMode();
          return;
        }
        // Check if we're in history navigation mode (empty input or already navigating history)
        if (this.state.historyIndex > -1) {
          // Already in history mode - navigate further back
          const newIndex = Math.min(this.state.historyIndex + 1, this.state.commandHistory.length - 1);
          if (newIndex !== this.state.historyIndex) {
            this.updateState({
              ...this.state,
              input: this.state.commandHistory[this.state.commandHistory.length - 1 - newIndex],
              historyIndex: newIndex,
            });
          }
        } else if (this.state.input.length === 0 && this.state.commandHistory.length > 0) {
          // Start history navigation from empty input
          this.updateState({
            ...this.state,
            input: this.state.commandHistory[this.state.commandHistory.length - 1],
            historyIndex: 0,
          });
        } else if (this.state.input.length > 0 && this.state.suggestions.length > 0) {
          // Navigate suggestions only when typing and not in history mode
          const newIndex = Math.max(0, this.state.selectedSuggestion - 1);
          this.updateState({
            ...this.state,
            selectedSuggestion: newIndex,
          });
        }
        return;
      }

      if (data === "\x1b[B") {
        // Down arrow
        if (this.state.mode === "command-output") {
          this.returnToInputMode();
          return;
        }
        // Check if we're in history navigation mode
        if (this.state.historyIndex > -1) {
          // Navigate command history
          const newIndex = this.state.historyIndex - 1;
          if (newIndex === -1) {
            this.updateState({ ...this.state, input: "", historyIndex: -1 });
          } else {
            this.updateState({
              ...this.state,
              input: this.state.commandHistory[this.state.commandHistory.length - 1 - newIndex],
              historyIndex: newIndex,
            });
          }
        } else if (this.state.input.length > 0 && this.state.suggestions.length > 0) {
          // Navigate suggestions only when typing and not in history mode
          const newIndex = Math.min(this.state.suggestions.length - 1, this.state.selectedSuggestion + 1);
          this.updateState({
            ...this.state,
            selectedSuggestion: newIndex,
          });
        }
        return;
      }

      if (data === "\t" && this.state.input.length > 0 && this.state.suggestions.length > 0 && this.state.historyIndex === -1) {
        // Tab
        this.updateState({
          ...this.state,
          input: this.state.suggestions[this.state.selectedSuggestion],
          historyIndex: -1,
        });
        return;
      }

      if (data === "\t" && this.state.mode === "command-output") {
        // Tab in command-output mode
        this.returnToInputMode();
        return;
      }

      if (this.state.mode === "input") {
        if (data === "\x7f") {
          // Backspace
          this.updateState({
            ...this.state,
            input: this.state.input.slice(0, -1),
            historyIndex: -1,
            selectedSuggestion: 0,
          });
        } else if (data.length === 1 && data >= " ") {
          // Printable character
          this.updateState({
            ...this.state,
            input: this.state.input + data,
            historyIndex: -1,
            selectedSuggestion: 0,
          });
        }
      } else if (this.state.mode === "command-output") {
        // When in command-output mode, any input should transition back to input mode
        if (data === "\x7f") {
          // Backspace
          this.updateState({
            ...this.state,
            mode: "input",
            input: "",
            outputContent: undefined,
            historyIndex: -1,
            selectedSuggestion: 0,
          });
        } else if (data.length === 1 && data >= " ") {
          // Printable character
          this.updateState({
            ...this.state,
            mode: "input",
            input: data,
            outputContent: undefined,
            historyIndex: -1,
            selectedSuggestion: 0,
          });
        }
      }
    });
  }

  private updateState(newState: ConsoleState): void {
    // Update suggestions without causing infinite loop
    if (newState.mode === "input") {
      const commands = commandRegistry.search(newState.input);
      const suggestions = commands.map((cmd) => cmd.name);

      // Update state with new suggestions
      newState = {
        ...newState,
        suggestions,
        selectedSuggestion: Math.min(Math.max(0, newState.selectedSuggestion), Math.max(0, suggestions.length - 1)),
      };
    }

    this.state = newState;
    this.updatePrompt();
  }

  private notifyStateChange(): void {
    this.onStateChange(this.state);
  }

  private returnToInputMode(): void {
    this.updateState({
      ...this.state,
      mode: "input",
      outputContent: undefined,
    });
    this.notifyStateChange();
  }

  public updateDesignEditor(designEditor: any): void {
    this.designEditor = designEditor;
  }

  private updateSuggestions(): void {
    // This method is now handled in updateState to prevent infinite loops
    // Keep for backward compatibility if needed
  }

  private handleSubmit(): void {
    if (this.state.mode === "input") {
      const command = this.state.input.trim();
      if (command) {
        this.updateState({
          ...this.state,
          input: "",
          historyIndex: -1,
        });
        this.notifyStateChange();
        this.executeCommandWithParameters(command);
      }
    }
  }

  private async handleCtrlC(): Promise<void> {
    // Check if there's selected text in the terminal
    const selectedText = this.terminal.getSelection();

    if (selectedText && selectedText.trim().length > 0) {
      // Copy selected text to clipboard
      try {
        await navigator.clipboard.writeText(selectedText);
        // Optionally show a brief feedback message
        // this.writeColored('📋 Text copied to clipboard\r\n', 'green')
      } catch (error) {
        // Fallback for browsers that don't support clipboard API
        console.warn("Could not copy to clipboard:", error);
      }
    } else {
      // No text selected, proceed with cancel operation
      this.handleCancel();
    }
  }

  private handleCancel(): void {
    if (this.currentForm) {
      this.currentForm.cleanup();
      this.currentForm = null;
    }

    // Show cancellation message if there was something to cancel
    const hadActiveOperation = this.state.mode !== "input" || this.state.input.length > 0;
    const wasGatheringParameters = this.state.mode === "parameter-gathering";

    // Clear the screen and start fresh
    this.terminal.clear();

    // Don't show cancellation message when returning from command output
    if (hadActiveOperation && this.state.mode !== "command-output") {
      const message = wasGatheringParameters ? "❌ Command cancelled" : "❌ Operation cancelled";
      this.writeColored(`${message}\r\n\r\n`, "red");
    }

    this.updateState({
      ...this.state,
      mode: "input",
      input: "",
      currentCommand: undefined,
      parameterIndex: 0,
      gatheredParameters: {},
      outputContent: undefined,
      historyIndex: -1,
    });
    this.notifyStateChange();

    // Force cursor to bottom after state update
    setTimeout(() => {
      this.updatePrompt();
      this.terminal.focus();
    }, 10);
  }

  private async executeCommandWithParameters(commandName: string): Promise<void> {
    const command = commandRegistry.getAll().find((cmd) => cmd.name.toLowerCase() === commandName.toLowerCase() || cmd.id.toLowerCase() === commandName.toLowerCase());

    if (!command) {
      this.terminal.clear();
      this.writeColored(`❌ Command not found: ${commandName}\r\n`, "red");
      this.updateState({
        ...this.state,
        mode: "command-output",
      });
      this.notifyStateChange();
      return;
    }

    if (command.parameters.length === 0) {
      await this.executeCommand(commandName, {});
    } else {
      this.updateState({
        ...this.state,
        mode: "parameter-gathering",
        currentCommand: command,
        parameterIndex: 0,
        gatheredParameters: {},
      });
      this.notifyStateChange();
      this.showParameterForm();
    }
  }

  private showParameterForm(): void {
    if (!this.state.currentCommand) return;

    // Clear the terminal first
    this.terminal.clear();

    const parameter = this.state.currentCommand.parameters[this.state.parameterIndex];

    const onSubmit = (value: any) => {
      this.handleParameterSubmit(value);
    };

    const onCancel = () => {
      this.handleCancel();
    };

    const onBack = () => {
      this.handleParameterBack();
    };

    this.currentForm = createParameterForm(this.terminal, parameter, onSubmit, onCancel, onBack, this.designEditor, this.state.gatheredParameters[parameter.name]);

    if (this.currentForm) {
      this.currentForm.start();
    } else {
      this.writeColored(`❌ Unsupported parameter type: ${parameter.type}\r\n`, "red");
      this.handleCancel();
    }
  }

  private handleParameterSubmit(value: any): void {
    if (!this.state.currentCommand) return;

    const newParameters = {
      ...this.state.gatheredParameters,
      [this.state.currentCommand.parameters[this.state.parameterIndex].name]: value,
    };

    if (this.currentForm) {
      this.currentForm.cleanup();
      this.currentForm = null;
    }

    if (this.state.parameterIndex >= this.state.currentCommand.parameters.length - 1) {
      this.executeCommand(this.state.currentCommand.name, newParameters);
      this.updateState({
        ...this.state,
        mode: "input",
        currentCommand: undefined,
        parameterIndex: 0,
        gatheredParameters: {},
      });
      this.notifyStateChange();
    } else {
      this.updateState({
        ...this.state,
        parameterIndex: this.state.parameterIndex + 1,
        gatheredParameters: newParameters,
      });
      this.notifyStateChange();
      this.showParameterForm();
    }
  }

  private handleParameterBack(): void {
    if (!this.state.currentCommand) return;

    if (this.currentForm) {
      this.currentForm.cleanup();
      this.currentForm = null;
    }

    if (this.state.parameterIndex > 0) {
      // Go back to previous parameter
      this.updateState({
        ...this.state,
        parameterIndex: this.state.parameterIndex - 1,
      });
      this.notifyStateChange();
      this.showParameterForm();
    } else {
      // Go back to command input
      this.handleCancel();
    }
  }

  private async executeCommand(commandName: string, payload: Record<string, any> = {}): Promise<void> {
    const command = commandRegistry.getAll().find((cmd) => cmd.name.toLowerCase() === commandName.toLowerCase() || cmd.id.toLowerCase() === commandName.toLowerCase());

    if (!command) {
      this.writeColored(`❌ Command not found: ${commandName}\r\n`, "red");
      setTimeout(() => this.updatePrompt(), 2000);
      return;
    }

    // Add command to history only when actually executing (not during parameter gathering)
    this.updateState({
      ...this.state,
      commandHistory: [commandName, ...this.state.commandHistory.slice(0, 99)],
    });

    try {
      const context = {
        kit: this.designEditor.kit || { types: [], designs: [] },
        designId: this.designEditor.designId || "",
        selection: this.designEditor.selection || {
          selectedPieceIds: [],
          selectedConnections: [],
          selectedPiecePortId: undefined,
        },
      };

      if (command.editorOnly) {
        const result = await command.execute(context, payload);

        if (result.selection && this.designEditor.setSelection) {
          this.designEditor.setSelection(result.selection);
        }

        if (result.content) {
          this.renderReactContent(result.content);
        } else {
          this.terminal.clear();
          this.writeColored("✅ Command executed successfully\r\n", "green");
          this.updateState({
            ...this.state,
            mode: "command-output",
          });
          this.notifyStateChange();
        }
      } else {
        if (this.designEditor.startTransaction) {
          this.designEditor.startTransaction();
        }

        try {
          const result = await command.execute(context, payload);

          if (result.design && this.designEditor.setDesign) {
            this.designEditor.setDesign(result.design);
          }
          if (result.selection && this.designEditor.setSelection) {
            this.designEditor.setSelection(result.selection);
          }

          if (this.designEditor.finalizeTransaction) {
            this.designEditor.finalizeTransaction();
          }

          if (result.content) {
            this.renderReactContent(result.content);
          } else {
            this.terminal.clear();
            this.writeColored("✅ Command executed successfully\r\n", "green");
            this.updateState({
              ...this.state,
              mode: "command-output",
            });
            this.notifyStateChange();
          }

          // Don't automatically return to prompt - wait for user input
        } catch (error) {
          if (this.designEditor.abortTransaction) {
            this.designEditor.abortTransaction();
          }
          throw error;
        }
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Unknown error occurred";
      this.terminal.clear();
      this.writeColored(`❌ Error: ${errorMessage}\r\n`, "red");
      this.updateState({
        ...this.state,
        mode: "command-output",
      });
      this.notifyStateChange();
    }
  }

  private renderReactContent(content: React.ReactNode): void {
    // Handle clear command specially
    if (content === null) {
      this.terminal.clear();
      this.updatePrompt();
      return;
    }

    // Clear screen to show only the output
    this.terminal.clear();

    // For now, just extract text content from React components
    if (typeof content === "string") {
      // Convert \n to \r\n for proper terminal line breaks
      const terminalContent = content.replace(/\n/g, "\r\n");
      this.terminal.write(terminalContent + "\r\n");
    } else if (content && typeof content === "object" && "props" in content) {
      // Extract text from React element
      const extractText = (node: any): string => {
        if (typeof node === "string") return node;
        if (typeof node === "number") return String(node);
        if (Array.isArray(node)) return node.map(extractText).join("");
        if (node && typeof node === "object" && node.props) {
          if (node.props.children) {
            return extractText(node.props.children);
          }
        }
        return "";
      };

      this.terminal.write(extractText(content) + "\r\n");
    }

    // Update state to show command output mode with delay for user to read
    this.updateState({
      ...this.state,
      mode: "command-output",
      outputContent: content,
    });
    this.notifyStateChange();
  }

  public resize(): void {
    this.fitAddon.fit();
    // Update prompt after resize to ensure proper positioning
    setTimeout(() => {
      if (this.state.mode === "input") {
        this.updatePrompt();
      }
    }, 50);
  }
  public focus(): void {
    this.terminal.focus();
    // Change cursor to primary color when focused
    this.updateCursorColor("#ff344f");
  }

  public blur(): void {
    // Change cursor to foreground color when not focused
    this.updateCursorColor("#001117");
  }

  private updateCursorColor(color: string): void {
    this.terminal.options.theme = {
      ...this.terminal.options.theme,
      cursor: color,
    };
  }

  public dispose(): void {
    if (this.currentForm) this.currentForm.cleanup();
    this.terminal.dispose();
  }
}

const Console: FC = () => {
  const designEditor = useDesignEditor();
  const terminalRef = useRef<HTMLDivElement>(null);
  const terminalConsoleRef = useRef<TerminalConsole | null>(null);
  const [state, setState] = useState<ConsoleState>(() => {
    // Initialize with empty suggestions first, they'll be populated when terminal is ready
    return {
      mode: "input",
      input: "",
      suggestions: [],
      selectedSuggestion: 0,
      parameterIndex: 0,
      gatheredParameters: {},
      commandHistory: [],
      historyIndex: -1,
    };
  });

  useEffect(() => {
    if (terminalRef.current && !terminalConsoleRef.current) {
      terminalConsoleRef.current = new TerminalConsole(terminalRef.current, designEditor, state, setState);

      // Ensure focus after initial setup
      setTimeout(() => {
        if (terminalConsoleRef.current) {
          terminalConsoleRef.current.focus();
        }
      }, 100);
    }

    return () => {
      if (terminalConsoleRef.current) {
        terminalConsoleRef.current.dispose();
        terminalConsoleRef.current = null;
      }
    };
  }, []); // Remove designEditor dependency to prevent recreating terminal

  // Update design editor reference when it changes
  useEffect(() => {
    if (terminalConsoleRef.current) {
      terminalConsoleRef.current.updateDesignEditor(designEditor);
    }
  }, [designEditor]);

  // Remove the problematic useEffect that causes infinite loops
  // The terminal console manages its own state through onStateChange callback

  // Inject CSS for WebKit scrollbar styling
  useEffect(() => {
    const style = document.createElement("style");
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
        `;
    document.head.appendChild(style);

    return () => {
      document.head.removeChild(style);
    };
  }, []);

  useEffect(() => {
    const handleResize = () => {
      if (terminalConsoleRef.current) {
        terminalConsoleRef.current.resize();
      }
    };

    const handleClick = () => {
      // Ensure terminal regains focus when clicked
      if (terminalConsoleRef.current) {
        terminalConsoleRef.current.focus();
      }
    };

    let resizeObserver: ResizeObserver | null = null;

    if (terminalRef.current) {
      // Watch for container size changes
      resizeObserver = new ResizeObserver(() => {
        handleResize();
      });
      resizeObserver.observe(terminalRef.current);
      terminalRef.current.addEventListener("click", handleClick);
    }

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
      if (resizeObserver) {
        resizeObserver.disconnect();
      }
      if (terminalRef.current) {
        terminalRef.current.removeEventListener("click", handleClick);
      }
    };
  }, []);

  return (
    <div className="h-full w-full flex flex-col bg-background-level-2">
      <div
        ref={terminalRef}
        className="flex-1 w-full overflow-hidden focus:outline-none console-terminal"
        style={{
          fontFamily: "var(--font-mono)",
          scrollbarWidth: "thin",
          scrollbarColor: "transparent transparent",
        }}
        tabIndex={0}
        onFocus={() => {
          if (terminalConsoleRef.current) {
            terminalConsoleRef.current.focus();
          }
        }}
        onBlur={() => {
          if (terminalConsoleRef.current) {
            terminalConsoleRef.current.blur();
          }
        }}
        onMouseEnter={(e) => {
          e.currentTarget.style.scrollbarColor = "#7b827d transparent";
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.scrollbarColor = "transparent transparent";
        }}
      />
    </div>
  );
};

export const ConsolePanel: FC<ConsolePanelProps> = ({ visible, leftPanelVisible, rightPanelVisible, leftPanelWidth = 230, rightPanelWidth = 230, height, setHeight }) => {
  if (!visible) return null;

  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startY = e.clientY;
    const startHeight = height;

    const handleMouseMove = (e: MouseEvent) => {
      const deltaY = startY - e.clientY;
      const newHeight = Math.max(200, Math.min(800, startHeight + deltaY));
      setHeight(newHeight);
    };

    const handleMouseUp = () => {
      setIsResizing(false);
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  };

  return (
    <div
      className={`absolute bottom-4 z-20 bg-background-level-2 text-foreground border
                      ${isResizing || isResizeHovered ? "border-t-primary" : "border-t"}`}
      style={{
        left: leftPanelVisible ? `${leftPanelWidth + 32}px` : "16px",
        right: rightPanelVisible ? `${rightPanelWidth + 32}px` : "16px",
        height: `${height}px`,
        overflow: "hidden",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <div className="absolute top-0 left-0 right-0 h-1 cursor-ns-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
      <div className="flex-1 w-full p-6">
        <div className="h-full w-full border border-border/20 rounded-lg overflow-hidden">
          <Console />
        </div>
      </div>
    </div>
  );
};

export const commandRegistry = new EnhancedCommandRegistry();

export default Console;
