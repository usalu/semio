/** ⏱️ Where the running playback sits this frame; present only while the window plays. */
export interface FemPlaybackClock {
  phase: number;
  reverse: boolean;
}

/** 🫧️ The running playback clock of one FEM results window. */
export interface FemResultsWindowTransient {
  clock?: FemPlaybackClock;
}

export interface SetPlaybackClock {
  clock?: FemPlaybackClock;
}
