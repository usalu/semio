@capability-lane-runtime
@oracle-rate-gate-30hz
@comparison-sends-per-second-at-most-30
Feature: Live lanes stay responsive and never freeze on connection loss
  Executed by `🔬️unit/🦀️.rs`.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Drag ticks coalesce to one preview per frame: Given fifty pointer moves within a frame When the frame is taken Then one preview is applied and forty-nine ticks were dropped
  - Presence is limited to 30 Hz: Given a presence that changes every millisecond for a second When it is polled every millisecond Then at most thirty sends happen
  - A short connection loss does not freeze editing: Given a lost link during a drag When the drag continues and is released Then the local preview and the commit work and presence resumes with the latest value after reconnect
  - Aborting leaves no trace: Given a drag in progress with a published presence drag When Escape is pressed Then the transient is clear and the next presence has no drag
  - Evaluation is debounced: Given a pointer tick When 60 ms and 250 ms pass Then tier one and then tier two become due, and a commit makes both due at once
  - Viewport actions fold into the ephemeral view lane: Given a default transient When the viewport actions select the field "stress" and the track "fire", seek 1800, play and set the speed 2 Then the view lane holds those values and re-selecting a track resets the value and stops playback
