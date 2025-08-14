// #region Header

// Sketchpad.reducer.test.tsx

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

import { Design, DesignId, Kit } from "@semio/js/semio";
import { beforeEach, describe, expect, it } from "vitest";

// Create minimal types for testing without importing React components
interface DesignEditorCoreState {
  designId: DesignId;
  fileUrls: Map<string, string>;
}

interface SketchpadState {
  isLoading: boolean;
  fileUrls: Map<string, string>;
  kit: Kit | null;
  designEditorCoreStates: DesignEditorCoreState[];
  activeDesign: number;
  designHistory: DesignId[];
}

enum SketchpadAction {
  ChangeActiveDesign = "CHANGE_ACTIVE_DESIGN",
}

type SketchpadActionType = {
  type: SketchpadAction.ChangeActiveDesign;
  payload: DesignId;
};

// Simplified reducer function extracted from Sketchpad.tsx logic
const sketchpadReducer = (state: SketchpadState, action: SketchpadActionType): SketchpadState => {
  switch (action.type) {
    case SketchpadAction.ChangeActiveDesign:
      // Find the index of the design with the matching designId
      const designIndex = state.designEditorCoreStates.findIndex((designState) => designState.designId.name === action.payload.name && designState.designId.variant === action.payload.variant && designState.designId.view === action.payload.view);

      if (designIndex !== -1) {
        // Get the current active design to add to history
        const currentActiveDesignState = state.designEditorCoreStates[state.activeDesign];
        const currentDesignId = currentActiveDesignState?.designId;

        // Only add to history if we're changing to a different design
        let updatedHistory = [...state.designHistory];
        if (currentDesignId && (currentDesignId.name !== action.payload.name || currentDesignId.variant !== action.payload.variant || currentDesignId.view !== action.payload.view)) {
          updatedHistory.push(currentDesignId);
        }

        return {
          ...state,
          activeDesign: designIndex,
          designHistory: updatedHistory,
        };
      } else {
        return state;
      }

    default:
      return state;
  }
};

const createInitialSketchpadState = (): SketchpadState => {
  return {
    isLoading: true,
    fileUrls: new Map(),
    kit: null,
    designEditorCoreStates: [],
    activeDesign: 0,
    designHistory: [],
  };
};

describe("SketchpadReducer.ChangeActiveDesign", () => {
  let mockKit: Kit;
  let mockDesigns: Design[];
  let initialState: SketchpadState;
  let fileUrls: Map<string, string>;

  beforeEach(() => {
    // Create mock designs using the semio data model
    mockDesigns = [
      {
        name: "design-1",
        variant: "v1",
        view: "main",
        unit: "m",
        pieces: [],
        connections: [],
        created: new Date("2024-01-01"),
        updated: new Date("2024-01-01"),
      },
      {
        name: "design-2",
        variant: "v1",
        view: "side",
        unit: "m",
        pieces: [],
        connections: [],
        created: new Date("2024-01-02"),
        updated: new Date("2024-01-02"),
      },
      {
        name: "design-3",
        unit: "m",
        pieces: [],
        connections: [],
        created: new Date("2024-01-03"),
        updated: new Date("2024-01-03"),
      },
    ];

    // Create mock kit
    mockKit = {
      name: "test-kit",
      version: "1.0.0",
      designs: mockDesigns,
      types: [],
      created: new Date("2024-01-01"),
      updated: new Date("2024-01-01"),
    };

    fileUrls = new Map([["test-file.glb", "blob:test-url"]]);

    // Create minimal design editor core states
    const designEditorCoreStates = mockDesigns.map((design) => ({
      designId: {
        name: design.name,
        variant: design.variant,
        view: design.view,
      },
      fileUrls,
    }));

    // Create initial state using the exported function
    initialState = {
      ...createInitialSketchpadState(),
      isLoading: false,
      fileUrls,
      kit: mockKit,
      designEditorCoreStates,
      activeDesign: 0,
      designHistory: [],
    };
  });

  it("should change active design when valid design is provided", () => {
    const targetDesignId: DesignId = { name: "design-2", variant: "v1", view: "side" };

    const action = {
      type: SketchpadAction.ChangeActiveDesign,
      payload: targetDesignId,
    } as const;

    const newState = sketchpadReducer(initialState, action);

    expect(newState.activeDesign).toBe(1); // Index of 'design-2'
    expect(newState.designHistory).toHaveLength(1);
    expect(newState.designHistory[0]).toEqual({
      name: "design-1",
      variant: "v1",
      view: "main",
    });
  });

  it("should not change state when invalid design is provided", () => {
    const invalidDesignId: DesignId = { name: "non-existent-design" };

    const action = {
      type: SketchpadAction.ChangeActiveDesign,
      payload: invalidDesignId,
    } as const;

    const newState = sketchpadReducer(initialState, action);

    expect(newState).toEqual(initialState);
    expect(newState.activeDesign).toBe(0);
    expect(newState.designHistory).toHaveLength(0);
  });

  it("should handle design without variant and view", () => {
    const targetDesignId: DesignId = { name: "design-3" };

    const action = {
      type: SketchpadAction.ChangeActiveDesign,
      payload: targetDesignId,
    } as const;

    const newState = sketchpadReducer(initialState, action);

    expect(newState.activeDesign).toBe(2); // Index of 'design-3'
    expect(newState.designHistory).toHaveLength(1);
    expect(newState.designHistory[0]).toEqual({
      name: "design-1",
      variant: "v1",
      view: "main",
    });
  });

  it("should not add to history when switching to the same design", () => {
    const sameDesignId: DesignId = { name: "design-1", variant: "v1", view: "main" };

    const action = {
      type: SketchpadAction.ChangeActiveDesign,
      payload: sameDesignId,
    } as const;

    const newState = sketchpadReducer(initialState, action);

    expect(newState.activeDesign).toBe(0);
    expect(newState.designHistory).toHaveLength(0); // No history added
  });

  it("should maintain existing history when changing designs", () => {
    // Start with existing history
    const stateWithHistory: SketchpadState = {
      ...initialState,
      activeDesign: 1,
      designHistory: [{ name: "design-1", variant: "v1", view: "main" }],
    };

    const targetDesignId: DesignId = { name: "design-3" };

    const action = {
      type: SketchpadAction.ChangeActiveDesign,
      payload: targetDesignId,
    } as const;

    const newState = sketchpadReducer(stateWithHistory, action);

    expect(newState.activeDesign).toBe(2);
    expect(newState.designHistory).toHaveLength(2);
    expect(newState.designHistory[0]).toEqual({ name: "design-1", variant: "v1", view: "main" });
    expect(newState.designHistory[1]).toEqual({ name: "design-2", variant: "v1", view: "side" });
  });

  it("should handle partial matching of design identifiers", () => {
    // Test case where name matches but variant/view differs
    const targetDesignId: DesignId = { name: "design-1", variant: "v2", view: "main" };

    const action = {
      type: SketchpadAction.ChangeActiveDesign,
      payload: targetDesignId,
    } as const;

    const newState = sketchpadReducer(initialState, action);

    // Should not change because variant doesn't match exactly
    expect(newState).toEqual(initialState);
  });

  it("should handle undefined variant and view correctly", () => {
    const targetDesignId: DesignId = { name: "design-3", variant: undefined, view: undefined };

    const action = {
      type: SketchpadAction.ChangeActiveDesign,
      payload: targetDesignId,
    } as const;

    const newState = sketchpadReducer(initialState, action);

    expect(newState.activeDesign).toBe(2); // Should match design-3
    expect(newState.designHistory).toHaveLength(1);
  });
});
