/** ⏱️ Where the running playback sits this frame; present only while the window plays. */
export interface Fem3dPlaybackClock {
  phase: number;
  reverse: boolean;
}

/** 🫧️ The running playback clock of one FEM 3D results window. */
export interface Fem3dResultsWindowTransient {
  clock?: Fem3dPlaybackClock;
}

export interface SetPlaybackClock {
  clock?: Fem3dPlaybackClock;
}
