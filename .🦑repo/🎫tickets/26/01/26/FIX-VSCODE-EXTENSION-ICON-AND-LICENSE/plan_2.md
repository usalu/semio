# Plan: Ticket Click Opens Markdown Preview

## Problem Analysis

Clicking on a ticket tree item in the VS Code extension opens the `ticket.md` file as plain text using `vscode.window.showTextDocument()`. The user wants it to open as a markdown preview instead.

## Solution

Change the `compose.openTicket` command to use `vscode.commands.executeCommand("markdown.showPreview", uri)` instead of `vscode.window.showTextDocument(uri)`.

## Implementation

1. Locate the `compose.openTicket` command registration in `js/vscode/extension.ts`
2. Replace `vscode.window.showTextDocument(uri)` with `vscode.commands.executeCommand("markdown.showPreview", uri)`

## Files Modified

- `js/vscode/extension.ts` - Updated `compose.openTicket` command to open markdown preview
