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
  store.registerCommand("selectPage", (args: { section: string; page: string }) => commands["semio.docsEditor.selectPage"](store.buildContext(), args.section, args.page));
  store.registerCommand("toggleSection", (args: { section: string }) => commands["semio.docsEditor.toggleSection"](store.buildContext(), args.section));
  store.registerCommand("updateSectionProgress", (args: { section: string; progress: number }) => commands["semio.docsEditor.updateSectionProgress"](store.buildContext(), args.section, args.progress));
  store.registerCommand("markPageComplete", (args: { section: string; page: string }) => commands["semio.docsEditor.markPageComplete"](store.buildContext(), args.section, args.page));
}
