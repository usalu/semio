// #region Header

// store.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, ReactNode, createContext, useContext, useMemo } from "react";
import { AbstractType, Doc } from "yjs";
import { EditorStore } from "../../store";

export interface DocsSectionState {
  isExpanded: boolean;
  progress?: number;
  completedPages?: string[];
}

export interface DocsSelection {
  section?: string;
  page?: string;
}

export interface DocsState {
  selection: DocsSelection;
  sectionStates: Record<string, DocsSectionState>;
  scrollPosition?: number;
}

export interface DocsSelectionDiff {
  section?: { prev?: string; next?: string };
  page?: { prev?: string; next?: string };
}

export interface DocsDiff {
  selectionDiff?: DocsSelectionDiff;
  sectionStatesDiff?: Record<string, Partial<DocsSectionState>>;
}

export interface DocsEdit {
  do: DocsEditorStep;
  undo: DocsEditorStep;
}

export interface DocsEditorStep {
  selectionDiff?: DocsSelectionDiff;
  sectionStatesDiff?: Record<string, Partial<DocsSectionState>>;
}

export interface DocsCommandContext {
  state: DocsState;
}

export interface DocsCommandResult {
  diff?: DocsDiff;
}

export class DocsEditorStore extends EditorStore<DocsState, DocsDiff, DocsSelectionDiff, DocsEdit, DocsCommandContext, DocsCommandResult> {
  constructor(yMap: AbstractType<any>, parentSketchpadDoc: Doc, guid: string) {
    super(yMap, parentSketchpadDoc, guid);
  }

  protected hash(state: DocsState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): DocsState {
    const ySelection = this.yMap.get("selection") || {};
    const ySectionStates = this.yMap.get("sectionStates") || {};
    return {
      selection: {
        section: ySelection.section,
        page: ySelection.page,
      },
      sectionStates: { ...ySectionStates },
      scrollPosition: this.yMap.get("scrollPosition"),
    };
  }

  protected applySelectionDiff(selectionDiff: DocsSelectionDiff): void {
    if (!selectionDiff) return;
    const ySelection = this.yMap.get("selection") || {};
    if (selectionDiff.section !== undefined) {
      ySelection.section = selectionDiff.section.next;
    }
    if (selectionDiff.page !== undefined) {
      ySelection.page = selectionDiff.page.next;
    }
    this.yMap.set("selection", ySelection);
  }

  protected inverseSelectionDiff(selection: DocsSelection, diff: DocsSelectionDiff): DocsSelectionDiff {
    const inverse: DocsSelectionDiff = {};
    if (diff.section !== undefined) {
      inverse.section = { prev: diff.section.next, next: diff.section.prev };
    }
    if (diff.page !== undefined) {
      inverse.page = { prev: diff.page.next, next: diff.page.prev };
    }
    return inverse;
  }

  protected getSelection(): DocsSelection {
    return this.state.selection;
  }

  applyDiff(diff: DocsDiff): void {
    if (diff.selectionDiff) {
      this.applySelectionDiff(diff.selectionDiff);
    }
    if (diff.sectionStatesDiff) {
      const ySectionStates = this.yMap.get("sectionStates") || {};
      for (const [section, stateDiff] of Object.entries(diff.sectionStatesDiff)) {
        ySectionStates[section] = { ...ySectionStates[section], ...stateDiff };
      }
      this.yMap.set("sectionStates", ySectionStates);
    }
  }
}

const DocsStoreContext = createContext<DocsEditorStore | null>(null);

export const DocsStoreProvider: FC<{ store: DocsEditorStore; children: ReactNode }> = ({ store, children }) => {
  return <DocsStoreContext.Provider value={store}>{children}</DocsStoreContext.Provider>;
};

export const useDocs = () => {
  const store = useContext(DocsStoreContext);
  if (!store) throw new Error("[ORIGIN] useDocs must be used within DocsStoreProvider");
  return store.state;
};

export const useDocsStore = () => {
  const store = useContext(DocsStoreContext);
  if (!store) throw new Error("[ORIGIN] useDocsStore must be used within DocsStoreProvider");
  return store;
};

export const useDocsCommands = () => {
  const store = useDocsStore();
  return useMemo(
    () => ({
      selectPage: async (section: string, page: string) => {
        return store.executeCommand("selectPage", { section, page });
      },
      toggleSection: async (section: string) => {
        return store.executeCommand("toggleSection", { section });
      },
      updateSectionProgress: async (section: string, progress: number) => {
        return store.executeCommand("updateSectionProgress", { section, progress });
      },
      markPageComplete: async (section: string, page: string) => {
        return store.executeCommand("markPageComplete", { section, page });
      },
    }),
    [store],
  );
};
