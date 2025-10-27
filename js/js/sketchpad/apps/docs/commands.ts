// #region Header

// commands.ts

// 2025 Ueli Saluz

// #endregion

import { DocsAppStore, DocsCommandContext, DocsCommandResult } from "./store";

export const commands = {
  "semio.docsApp.selectPage": (context: DocsCommandContext, section: string, page: string): DocsCommandResult => {
    return {
      diff: {
        selectionDiff: {
          section: { prev: context.state.selection.section, next: section },
          page: { prev: context.state.selection.page, next: page },
        },
      },
    };
  },
  "semio.docsApp.toggleSection": (context: DocsCommandContext, section: string): DocsCommandResult => {
    const currentState = context.state.sectionStates[section] || { isExpanded: false };
    return {
      diff: {
        sectionStatesDiff: {
          [section]: { isExpanded: !currentState.isExpanded },
        },
      },
    };
  },
  "semio.docsApp.updateSectionProgress": (context: DocsCommandContext, section: string, progress: number): DocsCommandResult => {
    return {
      diff: {
        sectionStatesDiff: {
          [section]: { progress },
        },
      },
    };
  },
  "semio.docsApp.markPageComplete": (context: DocsCommandContext, section: string, page: string): DocsCommandResult => {
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

export function registerDocsCommands(store: DocsAppStore): void {
  store.registerCommand("selectPage", (args: { section: string; page: string }) => commands["semio.docsApp.selectPage"](store.buildContext(), args.section, args.page));
  store.registerCommand("toggleSection", (args: { section: string }) => commands["semio.docsApp.toggleSection"](store.buildContext(), args.section));
  store.registerCommand("updateSectionProgress", (args: { section: string; progress: number }) => commands["semio.docsApp.updateSectionProgress"](store.buildContext(), args.section, args.progress));
  store.registerCommand("markPageComplete", (args: { section: string; page: string }) => commands["semio.docsApp.markPageComplete"](store.buildContext(), args.section, args.page));
}
