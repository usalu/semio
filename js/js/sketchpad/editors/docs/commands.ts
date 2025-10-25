// #region Header

// commands.ts

// 2025 Ueli Saluz

// #endregion

import { DocsCommandContext, DocsCommandResult, DocsEditorStore } from "./store";

export const commands = {
  "semio.docsEditor.selectPage": (context: DocsCommandContext, section: string, page: string): DocsCommandResult => {
    return {
      diff: {
        selectionDiff: {
          section: { prev: context.state.selection.section, next: section },
          page: { prev: context.state.selection.page, next: page },
        },
      },
    };
  },
  "semio.docsEditor.toggleSection": (context: DocsCommandContext, section: string): DocsCommandResult => {
    const currentState = context.state.sectionStates[section] || { isExpanded: false };
    return {
      diff: {
        sectionStatesDiff: {
          [section]: { isExpanded: !currentState.isExpanded },
        },
      },
    };
  },
  "semio.docsEditor.updateSectionProgress": (context: DocsCommandContext, section: string, progress: number): DocsCommandResult => {
    return {
      diff: {
        sectionStatesDiff: {
          [section]: { progress },
        },
      },
    };
  },
  "semio.docsEditor.markPageComplete": (context: DocsCommandContext, section: string, page: string): DocsCommandResult => {
    const currentState = context.state.sectionStates[section] || { isExpanded: false, completedPages: [] };
    const completedPages = currentState.completedPages || [];
    if (!completedPages.includes(page)) {
      completedPages.push(page);
    }
    return {
      diff: {
        sectionStatesDiff: {
          [section]: { completedPages: [...completedPages] },
        },
      },
    };
  },
};

export function registerDocsCommands(store: DocsEditorStore): void {
  Object.entries(commands).forEach(([name, handler]) => {
    store.registerCommand(name, handler);
  });
}
