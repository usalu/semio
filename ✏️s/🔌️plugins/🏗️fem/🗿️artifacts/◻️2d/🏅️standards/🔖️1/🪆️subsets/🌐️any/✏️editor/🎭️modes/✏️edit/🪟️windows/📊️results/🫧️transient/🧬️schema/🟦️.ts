/** ⏱️ Where the running playback sits this frame; present only while the window plays. */
export interface Fem2dPlaybackClock {
  phase: number;
  reverse: boolean;
}

/** 🫧️ The running playback clock of one FEM 2D results window. */
export interface Fem2dResultsWindowTransient {
  clock?: Fem2dPlaybackClock;
}

export interface SetPlaybackClock {
  clock?: Fem2dPlaybackClock;
}
