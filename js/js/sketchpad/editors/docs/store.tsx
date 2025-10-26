// #region Header

// store.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, ReactNode, createContext, useContext, useMemo } from "react";

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

// Simplified docs store (not using EditorStore yet)
export class DocsEditorStore {
  // Placeholder for future store implementation
}

const DocsStoreContext = createContext<DocsEditorStore | null>(null);

export const DocsStoreProvider: FC<{ store: DocsEditorStore; children: ReactNode }> = ({ store, children }) => {
  return <DocsStoreContext.Provider value={store}>{children}</DocsStoreContext.Provider>;
};

export const useDocs = () => {
  const store = useContext(DocsStoreContext);
  if (!store) throw new Error("[ORIGIN] useDocs must be used within DocsStoreProvider");
  // Return placeholder state for now
  return {
    selection: {},
    sectionStates: {},
  };
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
        // Placeholder
      },
      toggleSection: async (section: string) => {
        // Placeholder
      },
      updateSectionProgress: async (section: string, progress: number) => {
        // Placeholder
      },
      markPageComplete: async (section: string, page: string) => {
        // Placeholder
      },
    }),
    [store],
  );
};
